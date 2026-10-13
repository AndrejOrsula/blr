const DEFAULT_TIMEOUT_MS = 5_000;
const MAX_REASON_CHARS = 240;

function failure(code, reason) {
  return { supported: false, code, reason: String(reason).slice(0, MAX_REASON_CHARS) };
}

function messageOf(error, fallback) {
  const message = error instanceof Error ? error.message : String(error ?? '');
  return message ? `${fallback}: ${message}`.slice(0, MAX_REASON_CHARS) : fallback;
}

function pageCapabilityFailure() {
  if (globalThis.isSecureContext !== true) {
    return failure('insecure_context', 'Blender editor requires a secure browser context (HTTPS or localhost).');
  }
  if (globalThis.crossOriginIsolated !== true) {
    return failure('isolation_unavailable', 'Blender editor requires cross-origin isolation; reload after the isolation setup completes.');
  }
  if (typeof globalThis.Worker !== 'function') {
    return failure('worker_unavailable', 'Blender editor requires browser workers.');
  }
  if (typeof globalThis.OffscreenCanvas !== 'function'
    || typeof globalThis.HTMLCanvasElement !== 'function'
    || typeof globalThis.HTMLCanvasElement.prototype?.transferControlToOffscreen !== 'function') {
    return failure('offscreen_canvas_unavailable', 'Blender editor requires transferable OffscreenCanvas support.');
  }
  if (!globalThis.navigator?.gpu || typeof globalThis.navigator.gpu.requestAdapter !== 'function') {
    return failure('webgpu_unavailable', 'WebGPU is unavailable in this browser context. Use GLB preview instead.');
  }
  return null;
}

/**
 * Check whether this secure, isolated page can obtain a WebGPU device in a
 * disposable same-origin worker. This does not test Blender startup or render support.
 */
export function checkBlenderCapability({ signal, timeoutMs = DEFAULT_TIMEOUT_MS } = {}) {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    return Promise.reject(new RangeError('timeoutMs must be a finite positive number'));
  }
  const pageFailure = pageCapabilityFailure();
  if (pageFailure) return Promise.resolve(pageFailure);
  if (signal?.aborted) return Promise.resolve(failure('cancelled', 'WebGPU capability check was cancelled.'));

  return new Promise(resolve => {
    let worker;
    let timer = 0;
    let settled = false;

    const cleanup = ({ cancelWorker = false } = {}) => {
      clearTimeout(timer);
      signal?.removeEventListener('abort', onAbort);
      if (!worker) return;
      worker.removeEventListener('message', onMessage);
      worker.removeEventListener('error', onError);
      worker.removeEventListener('messageerror', onMessageError);
      if (cancelWorker) {
        try { worker.postMessage({ type: 'cancel' }); } catch { /* termination still releases the worker */ }
      }
      worker.terminate();
      worker = undefined;
    };

    const finish = (result, options) => {
      if (settled) return;
      settled = true;
      cleanup(options);
      resolve(result);
    };

    const onMessage = event => {
      const data = event.data;
      if (!data || data.type !== 'result' || typeof data.supported !== 'boolean'
        || typeof data.code !== 'string' || typeof data.reason !== 'string') {
        return finish(failure('worker_protocol_error', 'WebGPU capability worker returned an invalid result.'));
      }
      finish({ supported: data.supported, code: data.code, reason: data.reason.slice(0, MAX_REASON_CHARS) });
    };
    const onError = event => {
      event.preventDefault?.();
      finish(failure('worker_failed', messageOf(event.error ?? event.message, 'WebGPU capability worker failed.')));
    };
    const onMessageError = () => finish(failure('worker_message_error', 'WebGPU capability worker sent an unreadable result.'));
    const onAbort = () => finish(failure('cancelled', 'WebGPU capability check was cancelled.'), { cancelWorker: true });

    try {
      worker = new Worker(new URL("./gui-capability-worker.js?blr_app=fe2424e19037e1e8e7d38a3c9db9c0b23a65146b5a381da78ddc353447f3e9ba", import.meta.url), {
        type: 'module',
        name: 'blr-webgpu-capability',
      });
      worker.addEventListener('message', onMessage);
      worker.addEventListener('error', onError);
      worker.addEventListener('messageerror', onMessageError);
      signal?.addEventListener('abort', onAbort, { once: true });
      timer = setTimeout(() => finish(failure('timeout', 'WebGPU capability check timed out. Use GLB preview or retry.')), timeoutMs);
      if (signal?.aborted) return onAbort();
      worker.postMessage({ type: 'probe' });
    } catch (error) {
      finish(failure('worker_start_failed', messageOf(error, 'Could not start the WebGPU capability worker.')),
        { cancelWorker: true });
    }
  });
}
