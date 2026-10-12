(() => {
  'use strict';

  const CHANNEL = 'blr-gui-transport-v1';
  const MAX_MANIFEST_BYTES = 16 * 1024;
  const MAX_LOADER_BYTES = 2 * 1024 * 1024;
  const MAX_COMPRESSED_WASM_BYTES = 150 * 1024 * 1024;
  const MAX_WASM_BYTES = 320 * 1024 * 1024;
  const ALLOWED_ENV = Object.freeze({
    BLENDER_SYSTEM_PYTHON: '/assets',
    BLENDER_SYSTEM_RESOURCES: '/assets/5.2',
    PYTHONHOME: '/assets',
    BLENDER_SYSTEM_SCRIPTS: '/assets/5.2/scripts',
    BLENDER_SYSTEM_DATAFILES: '/assets/5.2/datafiles',
    PYTHONDONTWRITEBYTECODE: '1',
  });
  const params = new URLSearchParams(location.search);
  const runId = params.get('run') || '';
  const status = document.querySelector('#status');
  const canvas = document.querySelector('#canvas');
  let failed = false;
  let notifyingCanvasResize = false;
  let canvasTransferred = false;
  let backingWidth = 0;
  let backingHeight = 0;
  let cssWidth = 0;
  let cssHeight = 0;
  let backingRatio = 0;

  const transferCanvas = canvas.transferControlToOffscreen;
  if (typeof transferCanvas === 'function') {
    Object.defineProperty(canvas, 'transferControlToOffscreen', {
      configurable: true,
      value(...args) {
        const offscreen = Reflect.apply(transferCanvas, this, args);
        if (this === canvas) canvasTransferred = true;
        return offscreen;
      },
    });
  }

  function resizeCanvas() {
    const bounds = canvas.getBoundingClientRect();
    const ratio = Math.min(Math.max(Number(window.devicePixelRatio) || 1, 1), 4);
    const width = Math.max(1, Math.round(bounds.width * ratio));
    const height = Math.max(1, Math.round(bounds.height * ratio));
    const backingChanged = backingWidth !== width || backingHeight !== height;
    const sizeChanged = backingChanged || cssWidth !== bounds.width || cssHeight !== bounds.height
      || backingRatio !== ratio;
    if (!sizeChanged) return;
    if (backingChanged && !canvasTransferred) {
      canvas.width = width;
      canvas.height = height;
    }
    backingWidth = width;
    backingHeight = height;
    cssWidth = bounds.width;
    cssHeight = bounds.height;
    backingRatio = ratio;
    if (!notifyingCanvasResize) {
      notifyingCanvasResize = true;
      window.dispatchEvent(new Event('resize'));
      notifyingCanvasResize = false;
    }
  }

  resizeCanvas();
  window.addEventListener('resize', resizeCanvas);
  if (typeof ResizeObserver === 'function') new ResizeObserver(resizeCanvas).observe(canvas);
  let dprMedia = null;
  function watchDevicePixelRatio() {
    dprMedia?.removeEventListener('change', handleDevicePixelRatioChange);
    dprMedia = window.matchMedia(`(resolution: ${window.devicePixelRatio || 1}dppx)`);
    dprMedia.addEventListener('change', handleDevicePixelRatioChange, { once: true });
  }
  function handleDevicePixelRatioChange() {
    resizeCanvas();
    watchDevicePixelRatio();
  }
  watchDevicePixelRatio();

  function post(type, detail = {}) {
    window.parent.postMessage({ channel: CHANNEL, runId, type, ...detail }, location.origin);
  }

  function fail(message) {
    if (failed) return;
    failed = true;
    status.textContent = `Blender GUI failed: ${String(message).slice(0, 512)}`;
    status.hidden = false;
    post('error', { message: String(message).slice(0, 512) });
  }

  function sha256(bytes) {
    return crypto.subtle.digest('SHA-256', bytes).then(hash =>
      Array.from(new Uint8Array(hash), byte => byte.toString(16).padStart(2, '0')).join(''));
  }

  function sha256Integrity(hexDigest) {
    const digest = new Uint8Array(hexDigest.match(/../g).map(value => Number.parseInt(value, 16)));
    return `sha256-${btoa(String.fromCharCode(...digest))}`;
  }

  async function readBounded(stream, maximum, label) {
    if (!stream) throw new Error(`${label} response has no body`);
    const reader = stream.getReader();
    const chunks = [];
    let length = 0;
    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        length += value.byteLength;
        if (length > maximum) {
          await reader.cancel();
          throw new Error(`${label} exceeds the ${maximum}-byte limit`);
        }
        chunks.push(value);
      }
    } finally {
      reader.releaseLock();
    }
    const bytes = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) {
      bytes.set(chunk, offset);
      offset += chunk.byteLength;
    }
    return bytes;
  }

  function validateManifest(manifest) {
    if (!manifest || typeof manifest !== 'object' || Array.isArray(manifest)
      || !manifest.files || typeof manifest.files !== 'object' || Array.isArray(manifest.files)) {
      throw new Error('GUI runtime manifest has an unsupported format');
    }
    const files = {};
    for (const name of ['blender.js', 'blender.wasm.gz', 'blender.wasm']) {
      const item = manifest.files[name];
      if (!item || !Number.isSafeInteger(item.size) || item.size <= 0
        || typeof item.sha256 !== 'string' || !/^[a-f0-9]{64}$/.test(item.sha256)) {
        throw new Error(`GUI runtime manifest is missing valid metadata for ${name}`);
      }
      const maximum = name === 'blender.js' ? MAX_LOADER_BYTES
        : name === 'blender.wasm.gz' ? MAX_COMPRESSED_WASM_BYTES : MAX_WASM_BYTES;
      if (item.size > maximum) throw new Error(`${name} exceeds the ${maximum}-byte GUI runtime limit`);
      files[name] = { size: item.size, sha256: item.sha256 };
    }
    return files;
  }

  async function fetchVerified(url, metadata, label, maximum) {
    const response = await fetch(url, { credentials: 'same-origin', cache: 'force-cache' });
    if (!response.ok) throw new Error(`${label} request failed (${response.status})`);
    const bytes = await readBounded(response.body, maximum, label);
    if (bytes.byteLength !== metadata.size || await sha256(bytes) !== metadata.sha256) {
      throw new Error(`${label} does not match the GUI runtime manifest`);
    }
    return bytes;
  }

  async function start() {
    if (window.parent === window || params.getAll('run').length !== 1
      || !/^[A-Za-z0-9._:-]{1,128}$/.test(runId)
      || [...params.keys()].some(key => key !== 'run')) throw new Error('invalid GUI frame URL');
    if (!window.isSecureContext || !window.crossOriginIsolated || typeof SharedArrayBuffer === 'undefined') {
      throw new Error('isolated secure browser context is required for the GUI runtime');
    }
    if (!navigator.gpu || typeof navigator.gpu.requestAdapter !== 'function') {
      throw new Error('WebGPU is unavailable in this browser');
    }
    if (!crypto.subtle || !('DecompressionStream' in window)) throw new Error('runtime integrity APIs are unavailable');
    const manifestResponse = await fetch(new URL('runtime/gui/manifest.json', location.href), {
      credentials: 'same-origin', cache: 'no-store',
    });
    if (!manifestResponse.ok) throw new Error(`GUI runtime manifest request failed (${manifestResponse.status})`);
    const manifestBytes = await readBounded(manifestResponse.body, MAX_MANIFEST_BYTES, 'GUI runtime manifest');
    const files = validateManifest(JSON.parse(new TextDecoder().decode(manifestBytes)));
    const runtimeRoot = new URL('runtime/gui/', location.href);
    const loaderUrl = new URL('blender.js', runtimeRoot);
    const compressedUrl = new URL('blender.wasm.gz', runtimeRoot);
    loaderUrl.searchParams.set('v', files['blender.js'].sha256);
    compressedUrl.searchParams.set('v', files['blender.wasm.gz'].sha256);
    if (loaderUrl.origin !== location.origin || compressedUrl.origin !== location.origin
      || !loaderUrl.pathname.startsWith(runtimeRoot.pathname)
      || !compressedUrl.pathname.startsWith(runtimeRoot.pathname)) throw new Error('GUI runtime path escaped its directory');
    const [loaderBytes, compressedBytes] = await Promise.all([
      fetchVerified(loaderUrl, files['blender.js'], 'Blender GUI loader', MAX_LOADER_BYTES),
      fetchVerified(compressedUrl, files['blender.wasm.gz'], 'compressed Blender GUI runtime', MAX_COMPRESSED_WASM_BYTES),
    ]);
    const wasmBinary = await readBounded(
      new Blob([compressedBytes]).stream().pipeThrough(new DecompressionStream('gzip')),
      MAX_WASM_BYTES,
      'decompressed Blender GUI Wasm',
    );
    if (wasmBinary.byteLength !== files['blender.wasm'].size
      || await sha256(wasmBinary) !== files['blender.wasm'].sha256) throw new Error('decompressed Blender GUI Wasm does not match its manifest');

    resizeCanvas();
    window.Module = {
      canvas,
      wasmBinary,
      arguments: [
        '--factory-startup',
        '--gpu-backend', 'webgpu',
        '--env-system-datafiles', '/assets/5.2/datafiles',
        '--env-system-scripts', '/assets/5.2/scripts',
        '--python', '/assets/gui/session_driver.py',
      ],
      preRun: [() => {
        if (!window.Module.ENV || typeof window.Module.ENV !== 'object') throw new Error('GUI runtime did not export its environment interface');
        Object.assign(window.Module.ENV, ALLOWED_ENV);
      }],
      locateFile(path, prefix) {
        if (path !== 'blender.wasm') throw new Error(`unexpected GUI runtime file: ${path}`);
        const resolved = new URL(path, prefix || runtimeRoot.href);
        if (resolved.origin !== location.origin || resolved.href !== new URL('blender.wasm', runtimeRoot).href) {
          throw new Error('GUI runtime requested a file outside its directory');
        }
        return resolved.href;
      },
      onRuntimeInitialized() {
        status.hidden = true;
        post('initialized');
        const firstFrame = () => {
          const bounds = canvas.getBoundingClientRect();
          if (bounds.width > 0 && bounds.height > 0) post('frame-available', { width: bounds.width, height: bounds.height });
          else requestAnimationFrame(firstFrame);
        };
        requestAnimationFrame(firstFrame);
      },
      onAbort(reason) { fail(`runtime aborted: ${String(reason).slice(0, 384)}`); },
      onExit(code) { fail(`GUI runtime exited with status ${String(code)}`); },
      printErr(line) { post('output', { stream: 'stderr', text: String(line).slice(0, 4096) }); },
    };
    const script = document.createElement('script');
    script.src = URL.createObjectURL(new Blob([loaderBytes], { type: 'text/javascript' }));
    script.integrity = sha256Integrity(files['blender.js'].sha256);
    script.crossOrigin = 'anonymous';
    script.onerror = () => fail('could not load verified Blender GUI loader');
    document.head.append(script);
  }

  window.addEventListener('error', event => fail(event.error?.message || event.message || 'unexpected frame error'));
  window.addEventListener('unhandledrejection', event => fail(event.reason?.message || String(event.reason || 'unexpected frame rejection')));
  start().catch(error => fail(error instanceof Error ? error.message : 'GUI frame startup failed'));
})();
