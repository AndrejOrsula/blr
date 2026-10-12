import {
  inspectWasm,
  MAX_MODULE_BYTES,
} from "./wasm-policy.js";

const DEFAULT_TIMEOUT_MS = 10_000;
const MAX_TIMEOUT_MS = 60_000;

function namedError(name, message, boundary) {
  const error = new Error(message);
  error.name = name;
  if (boundary) error.guestBoundary = boundary;
  return error;
}

/** Run a precompiled WASI preview1 wasm32 module in a fresh, disposable Worker. */
export function runGuest(wasmBytes, { signal, onOutput, timeoutMs = DEFAULT_TIMEOUT_MS } = {}) {
  if (!(wasmBytes instanceof Uint8Array)) {
    return Promise.reject(new TypeError("wasmBytes must be a Uint8Array"));
  }
  if (!Number.isInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > MAX_TIMEOUT_MS) {
    return Promise.reject(new RangeError(`timeoutMs must be an integer from 1 to ${MAX_TIMEOUT_MS}`));
  }

  let policy;
  try {
    policy = inspectWasm(wasmBytes);
  } catch (error) {
    return Promise.reject(error);
  }
  if (wasmBytes.byteLength > MAX_MODULE_BYTES) {
    return Promise.reject(new RangeError(`guest module exceeds ${MAX_MODULE_BYTES} bytes`));
  }
  if (signal?.aborted) {
    return Promise.reject(namedError("AbortError", "guest run aborted before start"));
  }
  if (typeof Worker !== "function") {
    return Promise.reject(new Error("guest execution requires browser Worker support"));
  }

  let worker;
  try {
    worker = new Worker(new URL("./worker.js", import.meta.url), {
      type: "module",
      name: "blr-untrusted-rust-guest",
    });
  } catch (error) {
    return Promise.reject(error);
  }

  return new Promise((resolve, reject) => {
    const output = { stdout: "", stderr: "" };
    let settled = false;
    let timeoutId;
    const boundary = termination => ({
      wasmImports: policy.imports,
      memoryMaxBytes: policy.memory.maximumBytes,
      preopens: 0,
      workerTerminated: true,
      termination,
    });

    const finish = (kind, value, termination) => {
      if (settled) return;
      settled = true;
      clearTimeout(timeoutId);
      signal?.removeEventListener("abort", onAbort);
      worker.removeEventListener("message", onMessage);
      worker.removeEventListener("error", onError);
      worker.removeEventListener("messageerror", onMessageError);
      worker.terminate();
      if (kind === "resolve") {
        resolve({
          ...value,
          stdout: output.stdout,
          stderr: output.stderr,
          guestBoundary: boundary(termination),
        });
      } else {
        value.guestBoundary = boundary(termination);
        reject(value);
      }
    };

    const onAbort = () => finish(
      "reject",
      namedError("AbortError", "guest run aborted"),
      "aborted",
    );
    const onError = event => {
      const detail = event.error ? String(event.error) : "";
      finish(
        "reject",
        new Error(`guest worker failed: ${event.message || detail || `${event.filename || "unknown worker error"}:${event.lineno || 0}:${event.colno || 0}`}`),
        "error",
      );
    };
    const onMessageError = () => finish(
      "reject",
      new Error("guest worker sent an unreadable message"),
      "error",
    );
    const onMessage = event => {
      const message = event.data;
      if (message?.type === "ready") return;
      if (message?.type === "output") {
        if (message.stream !== "stdout" && message.stream !== "stderr" || typeof message.text !== "string") {
          finish("reject", new Error("guest worker sent invalid output"), "error");
          return;
        }
        output[message.stream] += message.text;
        try {
          onOutput?.({ stream: message.stream, text: message.text });
        } catch (error) {
          finish("reject", error instanceof Error ? error : new Error(String(error)), "error");
        }
        return;
      }
      if (message?.type === "failure") {
        const detail = String(message.error).slice(0, 2048);
        const termination = /output limit exceeded|line exceeds/.test(detail) ? "output-limit" : "error";
        finish("reject", new Error(`guest failed: ${detail}`), termination);
        return;
      }
      if (message?.type === "result") {
        const evidence = message.memoryEvidence;
        const importsMatch = Array.isArray(message.imports)
          && message.imports.length === policy.imports.length
          && message.imports.every((value, index) => value === policy.imports[index]);
        const memoryMatches = evidence
          && evidence.initialPages === policy.memory.initialPages
          && evidence.maximumPages === policy.memory.maximumPages
          && evidence.maximumBytes === policy.memory.maximumBytes
          && Number.isInteger(evidence.actualBytes)
          && evidence.actualBytes >= evidence.initialPages * 65536
          && evidence.actualBytes <= evidence.maximumBytes
          && evidence.actualBytes % 65536 === 0;
        if (!Number.isInteger(message.exitCode) || !importsMatch || !memoryMatches) {
          finish("reject", new Error("guest worker returned invalid evidence"), "error");
          return;
        }
        resolveResult(message);
        return;
      }
      finish("reject", new Error("guest worker sent an unknown message"), "error");
    };
    const resolveResult = message => finish(
      "resolve",
      {
        exitCode: message.exitCode,
        imports: message.imports,
        memoryEvidence: message.memoryEvidence,
      },
      "completed",
    );

    worker.addEventListener("message", onMessage);
    worker.addEventListener("error", onError);
    worker.addEventListener("messageerror", onMessageError);
    signal?.addEventListener("abort", onAbort, { once: true });
    if (signal?.aborted) {
      onAbort();
      return;
    }
    timeoutId = setTimeout(() => finish(
      "reject",
      namedError("TimeoutError", `guest exceeded ${timeoutMs} ms`),
      "timeout",
    ), timeoutMs);
    const isolatedBytes = wasmBytes.slice();
    try {
      worker.postMessage({ type: "run", wasmBytes: isolatedBytes.buffer }, [isolatedBytes.buffer]);
    } catch (error) {
      finish("reject", error instanceof Error ? error : new Error(String(error)), "error");
    }
  });
}

export { MAX_MODULE_BYTES, MAX_OUTPUT_BYTES } from "./wasm-policy.js";
