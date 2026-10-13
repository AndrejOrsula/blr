function abortError(message = "Compiler asset loading aborted") {
  return new DOMException(message, "AbortError");
}

export function notifyProgress(onProgress, event) {
  if (typeof onProgress !== "function") return;
  try {
    onProgress({
      phase: event.phase,
      asset: event.asset ?? null,
      state: event.state,
      loadedBytes: event.loadedBytes ?? null,
      totalBytes: event.totalBytes ?? null,
      source: event.source ?? null,
      error: event.error ?? null,
    });
  } catch {
    // Progress is advisory and must not interrupt loading.
  }
}

function progressEvent(onProgress, progress, state, loadedBytes, error) {
  notifyProgress(onProgress, {
    phase: progress?.phase ?? "fetch",
    asset: progress?.asset,
    state,
    loadedBytes,
    totalBytes: progress?.totalBytes,
    source: progress?.source ?? "network-or-cache",
    error,
  });
}

function isAbort(error, signal) {
  return signal?.aborted || error?.name === "AbortError";
}

/** Cap response-body bytes; joining the retained chunks also allocates the returned buffer. */
export async function readBoundedResponse(response, { limit, label, signal, onProgress, progress } = {}) {
  progressEvent(onProgress, progress, "start", 0);
  if (!response?.ok || !response.body) {
    const error = new Error(`${label || "Asset"} request failed (HTTP ${response?.status ?? "unknown"}).`);
    progressEvent(onProgress, progress, "error", 0, error.message);
    throw error;
  }
  if (!Number.isSafeInteger(limit) || limit < 0) {
    const error = new RangeError("Asset byte limit must be a non-negative safe integer.");
    progressEvent(onProgress, progress, "error", 0, error.message);
    throw error;
  }
  const reader = response.body.getReader();
  const chunks = [];
  let size = 0;
  const cancelReader = () => { reader.cancel().catch(() => {}); };
  signal?.addEventListener("abort", cancelReader, { once: true });
  if (signal?.aborted) cancelReader();
  try {
    while (true) {
      if (signal?.aborted) throw abortError();
      const { done, value } = await reader.read();
      if (signal?.aborted) throw abortError();
      if (done) break;
      size += value.byteLength;
      if (size > limit) throw new RangeError(`${label || "Asset"} exceeds ${limit} bytes.`);
      chunks.push(value);
      progressEvent(onProgress, progress, "update", size);
    }
  } catch (error) {
    await reader.cancel(error).catch(() => {});
    progressEvent(onProgress, progress, isAbort(error, signal) ? "cancelled" : "error", size,
      isAbort(error, signal) ? null : error instanceof Error ? error.message : String(error));
    throw error;
  } finally {
    signal?.removeEventListener("abort", cancelReader);
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
export async function readBoundedJson(response, { limit, label, signal, onProgress, progress } = {}) {
  const bytes = await readBoundedResponse(response, { limit, label, signal, onProgress, progress });
  try {
    const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    return JSON.parse(text);
  } catch (error) {
    const failure = error instanceof SyntaxError
      ? new Error(`${label || "Asset"} contains invalid JSON.`, { cause: error })
      : error;
    if (progress) {
      notifyProgress(onProgress, {
        phase: "verify", asset: progress.asset, state: isAbort(error, signal) ? "cancelled" : "error",
        loadedBytes: bytes.byteLength, totalBytes: null, source: "work",
        error: isAbort(error, signal) ? null : failure instanceof Error ? failure.message : String(failure),
      });
    }
    throw failure;
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
