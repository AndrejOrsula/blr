const MAX_REASON_CHARS = 240;
let cancelled = false;
let completed = false;
let activeDevice = null;

function reasonOf(error, fallback) {
  const message = error instanceof Error ? error.message : String(error ?? '');
  return message ? `${fallback}: ${message}`.slice(0, MAX_REASON_CHARS) : fallback;
}

function finish(supported, code, reason) {
  if (completed || cancelled) return;
  completed = true;
  self.postMessage({ supported, code, reason: String(reason).slice(0, MAX_REASON_CHARS), type: 'result' });
}

function destroyDevice() {
  const device = activeDevice;
  activeDevice = null;
  try { device?.destroy(); } catch { /* device loss or worker teardown already released it */ }
}

async function probe() {
  if (!self.navigator?.gpu || typeof self.navigator.gpu.requestAdapter !== 'function') {
    finish(false, 'worker_webgpu_unavailable', 'WebGPU is unavailable in the disposable worker context. Use GLB preview instead.');
    return;
  }

  let adapter;
  try {
    adapter = await self.navigator.gpu.requestAdapter();
  } catch (error) {
    finish(false, 'adapter_rejected', reasonOf(error, 'The browser rejected its WebGPU adapter request.'));
    return;
  }
  if (cancelled) return;
  if (!adapter) {
    finish(false, 'adapter_unavailable', 'The browser did not provide a usable WebGPU adapter. Use GLB preview instead.');
    return;
  }

  let device;
  try {
    device = await adapter.requestDevice();
  } catch (error) {
    finish(false, 'device_rejected', reasonOf(error, 'The browser rejected its WebGPU device request.'));
    return;
  }
  activeDevice = device;
  if (cancelled) {
    destroyDevice();
    return;
  }
  if (!device || !device.lost || typeof device.destroy !== 'function') {
    destroyDevice();
    finish(false, 'device_invalid', 'The browser returned an unusable WebGPU device. Use GLB preview instead.');
    return;
  }

  // WebGPU may resolve requestDevice() with an already-lost device. Give that
  // loss promise one task to settle before reporting a successful acquisition.
  let lossTimer;
  const immediateLoss = await Promise.race([
    Promise.resolve(device.lost).then(info => ({ info })),
    new Promise(resolve => { lossTimer = setTimeout(() => resolve(null), 0); }),
  ]);
  clearTimeout(lossTimer);
  if (cancelled) {
    destroyDevice();
    return;
  }
  if (immediateLoss) {
    destroyDevice();
    finish(false, 'device_lost', reasonOf(immediateLoss.info?.message, 'The WebGPU device was lost during capability checking.'));
    return;
  }

  destroyDevice();
  finish(true, 'available', 'WebGPU adapter and device are available. Blender rendering support has not been tested.');
}

self.onmessage = event => {
  if (event.data?.type === 'cancel') {
    cancelled = true;
    destroyDevice();
    return;
  }
  if (event.data?.type !== 'probe' || completed || cancelled) return;
  void probe().catch(error => {
    destroyDevice();
    finish(false, 'probe_failed', reasonOf(error, 'WebGPU capability checking failed.'));
  });
};
