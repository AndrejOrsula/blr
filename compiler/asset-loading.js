function abortError(message = "Compiler asset loading aborted") {
  return new DOMException(message, "AbortError");
}

/** Read a response body without ever retaining more than its declared limit. */
export async function readBoundedResponse(response, { limit, label, signal } = {}) {
  if (!response?.ok || !response.body) {
    throw new Error(`${label || "Asset"} request failed (HTTP ${response?.status ?? "unknown"}).`);
  }
  if (!Number.isSafeInteger(limit) || limit < 0) throw new RangeError("Asset byte limit must be a non-negative safe integer.");
  const reader = response.body.getReader();
  const chunks = [];
  let size = 0;
  try {
    while (true) {
      if (signal?.aborted) throw abortError();
      const { done, value } = await reader.read();
      if (done) break;
      size += value.byteLength;
      if (size > limit) throw new RangeError(`${label || "Asset"} exceeds ${limit} bytes.`);
      chunks.push(value);
    }
  } catch (error) {
    await reader.cancel(error).catch(() => {});
    throw error;
  } finally {
    reader.releaseLock();
  }
  const bytes = new Uint8Array(size);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return bytes;
}

/** Parse JSON only after its complete response body has passed the streaming byte cap. */
export async function readBoundedJson(response, { limit, label, signal } = {}) {
  const bytes = await readBoundedResponse(response, { limit, label, signal });
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  try {
    return JSON.parse(text);
  } catch (error) {
    if (error instanceof SyntaxError) throw new Error(`${label || "Asset"} contains invalid JSON.`, { cause: error });
    throw error;
  }
}

/** Bound a group of concurrent asset loads and abort siblings on every failure. */
export function runWithAssetLoadBounds(load, { signal, timeoutMs = 60_000 } = {}) {
  if (typeof load !== "function") return Promise.reject(new TypeError("Asset loader must be a function."));
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1) return Promise.reject(new RangeError("Asset loading timeout must be a positive safe integer."));
  if (signal?.aborted) return Promise.reject(abortError());

  const controller = new AbortController();
  let timeoutId;
  let rejectAbort;
  const aborted = new Promise((_, reject) => { rejectAbort = reject; });
  const onAbort = () => {
    controller.abort();
    rejectAbort(abortError());
  };
  signal?.addEventListener("abort", onAbort, { once: true });
  if (signal?.aborted) onAbort();

  const work = Promise.resolve().then(() => load(controller.signal)).catch((error) => {
    controller.abort();
    throw error;
  });
  const deadline = new Promise((_, reject) => {
    timeoutId = setTimeout(() => {
      controller.abort();
      const error = new Error(`Compiler asset loading exceeded ${timeoutMs} ms.`);
      error.name = "TimeoutError";
      reject(error);
    }, timeoutMs);
  });
  return Promise.race([work, deadline, aborted]).finally(() => {
    clearTimeout(timeoutId);
    signal?.removeEventListener("abort", onAbort);
  });
}
