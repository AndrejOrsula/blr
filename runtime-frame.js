(() => {
  'use strict';

  const CHANNEL = 'blr-demo';
  const MAX_CAPTURED_OUTPUT = 64 * 1024;
  const MAX_MANIFEST_BYTES = 16 * 1024;
  const MAX_LOADER_BYTES = 2 * 1024 * 1024;
  const MAX_COMPRESSED_WASM_BYTES = 99 * 1024 * 1024;
  const MAX_WASM_BYTES = 150 * 1024 * 1024;
  const ALLOWED_ENV = Object.freeze({
    BLENDER_SYSTEM_PYTHON: '/assets',
    BLENDER_SYSTEM_RESOURCES: '/assets/5.2',
    PYTHONHOME: '/assets',
    BLENDER_SYSTEM_SCRIPTS: '/assets/5.2/scripts',
    BLENDER_SYSTEM_DATAFILES: '/assets/5.2/datafiles',
    PYTHONDONTWRITEBYTECODE: '1',
  });
  const params = new URLSearchParams(location.search);
  const statusNode = document.querySelector('#runtime-status');
  let capturedOutput = 0;
  let resultRecord = null;
  let failed = false;

  function post(type, values = {}) {
    window.parent.postMessage({ channel: CHANNEL, runId, type, ...values }, location.origin);
  }

  function setStatus(state) {
    statusNode.textContent = state;
    post('status', { status: state });
  }

  const runId = params.get('run') || '';
  function fail(message) {
    if (failed) return;
    failed = true;
    const detail = String(message).slice(0, 512);
    statusNode.textContent = `Blender runtime failed: ${detail}`;
    post('error', { message: detail });
  }

  window.addEventListener('error', event => {
    const detail = typeof event.error?.message === 'string' ? event.error.message : event.message;
    fail(detail || 'unexpected runtime error');
  });
  window.addEventListener('unhandledrejection', event => {
    const reason = event.reason;
    fail(typeof reason?.message === 'string' ? reason.message : String(reason || 'unexpected runtime rejection'));
  });

  function sha256(bytes) {
    return crypto.subtle.digest('SHA-256', bytes).then(hash =>
      Array.from(new Uint8Array(hash), byte => byte.toString(16).padStart(2, '0')).join(''));
  }

  function sha256Integrity(hexDigest) {
    const bytes = new Uint8Array(hexDigest.match(/../g).map(value => Number.parseInt(value, 16)));
    return `sha256-${btoa(String.fromCharCode(...bytes))}`;
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
    const result = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) {
      result.set(chunk, offset);
      offset += chunk.byteLength;
    }
    return result.buffer;
  }

  function validateManifest(manifest) {
    if (!manifest || typeof manifest !== 'object' || Array.isArray(manifest)
      || !manifest.files || typeof manifest.files !== 'object' || Array.isArray(manifest.files)) {
      throw new Error('runtime manifest has an unsupported format');
    }
    const entries = {};
    for (const name of ['blender.js', 'blender.wasm.gz', 'blender.wasm']) {
      const entry = manifest.files[name];
      if (!entry || !Number.isSafeInteger(entry.size) || entry.size <= 0
        || typeof entry.sha256 !== 'string' || !/^[a-f0-9]{64}$/.test(entry.sha256)) {
        throw new Error(`runtime manifest is missing valid metadata for ${name}`);
      }
      const maximum = name === 'blender.js' ? MAX_LOADER_BYTES
        : name === 'blender.wasm.gz' ? MAX_COMPRESSED_WASM_BYTES : MAX_WASM_BYTES;
      if (entry.size > maximum) throw new Error(`${name} exceeds the ${maximum}-byte runtime limit`);
      entries[name] = { size: entry.size, sha256: entry.sha256 };
    }
    return entries;
  }

  async function fetchVerified(url, metadata, label) {
    const response = await fetch(url, { credentials: 'same-origin', cache: 'force-cache' });
    if (!response.ok) throw new Error(`${label} request failed (${response.status})`);
    const maximum = label === 'Blender loader' ? MAX_LOADER_BYTES : MAX_COMPRESSED_WASM_BYTES;
    const bytes = await readBounded(response.body, maximum, label);
    if (bytes.byteLength !== metadata.size) throw new Error(`${label} size does not match the runtime manifest`);
    if (await sha256(bytes) !== metadata.sha256) throw new Error(`${label} SHA-256 does not match the runtime manifest`);
    return bytes;
  }

  function makePythonExpression(size) {
    const code = [
      'import bpy, json, sys',
      'probe = __import__("_blr_browser_probe")',
      'version = tuple(bpy.app.version)',
      'if version != (5, 2, 0): raise RuntimeError("expected Blender 5.2.0; observed %r" % (version,))',
      'if hasattr(probe, "__file__"): raise AssertionError("Rust module unexpectedly has __file__")',
      'spec = getattr(probe, "__spec__", None)',
      'origin = getattr(spec, "origin", None)',
      'if origin != "built-in": raise AssertionError("Rust module is not built-in: %r" % (origin,))',
      'identity = {"module": probe.__name__, "origin": origin, "has_file": False, "function_module": probe.run_cube_probe.__module__}',
      'def inventory():',
      '    data = bpy.data',
      '    return (sorted(item.session_uid for item in data.objects), sorted(item.session_uid for item in data.meshes))',
      'before = inventory()',
      `requested = ${size.toString()}`,
      'result = probe.run_cube_probe(requested)',
      'after = inventory()',
      'if result != (requested, 8, 12, 12): raise AssertionError("Rust cube result mismatch: %r" % (result,))',
      'if before != after: raise AssertionError("Rust cube call changed object/mesh session_uid inventories")',
      'record = {"case": "browser-cube", "blender_version": list(version), "python_version": sys.version, "blender_build_hash": str(bpy.app.build_hash), "rust_module": identity, "requested_size": requested, "result": list(result), "inventory_unchanged": True}',
      'print("BLR_BROWSER_PROBE_RESULT " + json.dumps(record, separators=(",", ":")))',
    ].join('\n');
    return `exec(${JSON.stringify(code)})`;
  }

  function parseRuntimeOutput(line) {
    capturedOutput += line.length;
    if (capturedOutput > MAX_CAPTURED_OUTPUT) {
      fail('Blender output exceeded the 64 KiB limit');
      return;
    }
    const prefix = 'BLR_BROWSER_PROBE_RESULT ';
    if (!line.startsWith(prefix)) return;
    if (resultRecord) {
      fail('Blender emitted more than one result record');
      return;
    }
    try {
      const record = JSON.parse(line.slice(prefix.length));
      const expectedSize = Number(params.get('size'));
      const rust = record.rust_module;
      if (record.case !== 'browser-cube'
        || JSON.stringify(record.blender_version) !== '[5,2,0]'
        || typeof record.python_version !== 'string' || !/^3\.13\./.test(record.python_version)
        || typeof record.blender_build_hash !== 'string' || !record.blender_build_hash
        || !rust || rust.module !== '_blr_browser_probe' || rust.origin !== 'built-in'
        || rust.has_file !== false || rust.function_module !== '_blr_browser_probe'
        || record.requested_size !== expectedSize
        || !Array.isArray(record.result) || record.result.length !== 4
        || record.result[0] !== expectedSize || record.result[1] !== 8
        || record.result[2] !== 12 || record.result[3] !== 12
        || record.inventory_unchanged !== true) {
        throw new Error('Blender result did not satisfy the built-in cube contract');
      }
      resultRecord = {
        blender_version: record.blender_version,
        python_version: record.python_version,
        blender_build_hash: record.blender_build_hash,
        rust_module: rust,
        requested_size: expectedSize,
        result: record.result,
        inventory_unchanged: true,
      };
    } catch (error) {
      fail(error instanceof Error ? error.message : 'invalid Blender result record');
    }
  }

  async function start() {
    if (params.getAll('run').length !== 1 || !/^[A-Za-z0-9._:-]{1,128}$/.test(runId)) {
      throw new Error('missing or invalid run identifier');
    }
    if (params.getAll('size').length !== 1 || !/^(?:\d+(?:\.\d*)?|\.\d+)$/.test(params.get('size') || '')) {
      throw new Error('size must be a number from 0.1 through 4');
    }
    const size = Number(params.get('size'));
    if (!Number.isFinite(size) || size < 0.1 || size > 4) throw new Error('size must be a number from 0.1 through 4');
    if ([...params.keys()].some(key => key !== 'run' && key !== 'size')) throw new Error('unsupported runtime URL parameter');
    if (!window.blrIsolationReady || typeof window.blrIsolationReady.then !== 'function') {
      throw new Error('cross-origin isolation setup is unavailable');
    }
    await window.blrIsolationReady;
    if (!('DecompressionStream' in window) || !crypto.subtle) throw new Error('this browser cannot verify the runtime files');

    setStatus('loading');
    const manifestResponse = await fetch(new URL('runtime/manifest.json', location.href), {
      credentials: 'same-origin', cache: 'no-store',
    });
    if (!manifestResponse.ok) throw new Error(`runtime manifest request failed (${manifestResponse.status})`);
    const manifestBytes = await readBounded(manifestResponse.body, MAX_MANIFEST_BYTES, 'runtime manifest');
    const manifest = JSON.parse(new TextDecoder().decode(manifestBytes));
    const files = validateManifest(manifest);
    setStatus('verifying runtime files');

    const runtimeRoot = new URL('runtime/', location.href);
    const loaderUrl = new URL('blender.js', runtimeRoot);
    const compressedUrl = new URL('blender.wasm.gz', runtimeRoot);
    loaderUrl.searchParams.set('v', files['blender.js'].sha256);
    compressedUrl.searchParams.set('v', files['blender.wasm.gz'].sha256);
    if (loaderUrl.origin !== location.origin || compressedUrl.origin !== location.origin
      || !loaderUrl.pathname.startsWith(runtimeRoot.pathname)
      || !compressedUrl.pathname.startsWith(runtimeRoot.pathname)) {
      throw new Error('runtime path escaped its same-origin directory');
    }

    const [loaderBytes, compressedBytes] = await Promise.all([
      fetchVerified(loaderUrl, files['blender.js'], 'Blender loader'),
      fetchVerified(compressedUrl, files['blender.wasm.gz'], 'compressed Blender runtime'),
    ]);
    const decompressed = await readBounded(
      new Blob([compressedBytes]).stream().pipeThrough(new DecompressionStream('gzip')),
      MAX_WASM_BYTES,
      'decompressed Wasm',
    );
    if (decompressed.byteLength !== files['blender.wasm'].size) throw new Error('decompressed Wasm size does not match the runtime manifest');
    if (await sha256(decompressed) !== files['blender.wasm'].sha256) throw new Error('decompressed Wasm SHA-256 does not match the runtime manifest');

    setStatus('starting Blender 5.2');
    window.Module = {
      wasmBinary: decompressed,
      arguments: [
        '--background', '--factory-startup', '--threads', '1',
        '--python-exit-code', '1', '--python-expr', makePythonExpression(size),
      ],
      preRun: [() => {
        if (!window.Module.ENV || typeof window.Module.ENV !== 'object') {
          throw new Error('Blender runtime did not export its environment interface');
        }
        Object.assign(window.Module.ENV, ALLOWED_ENV);
      }],
      print: parseRuntimeOutput,
      printErr: line => {
        capturedOutput += String(line).length;
        if (capturedOutput > MAX_CAPTURED_OUTPUT) fail('Blender output exceeded the 64 KiB limit');
      },
      onRuntimeInitialized: () => setStatus('running Blender 5.2'),
      onAbort: reason => fail(`Blender runtime aborted: ${String(reason).slice(0, 384)}`),
      onExit: status => {
        const code = Number(status);
        post('exit', { status: Number.isInteger(code) ? code : -1 });
        if (code !== 0) {
          fail(`Blender exited with status ${String(status)}`);
        } else if (!resultRecord) {
          fail('Blender exited successfully without a valid Rust result');
        } else if (!failed) {
          statusNode.textContent = 'Blender demo complete';
          post('result', resultRecord);
        }
      },
      locateFile: (path, prefix) => {
        if (path !== 'blender.wasm') throw new Error(`unexpected Blender runtime file: ${path}`);
        const resolved = new URL(path, prefix || runtimeRoot.href);
        if (resolved.origin !== location.origin || resolved.href !== new URL('blender.wasm', runtimeRoot).href) {
          throw new Error('Blender runtime requested a file outside its runtime directory');
        }
        return resolved.href;
      },
    };
    const script = document.createElement('script');
    script.src = loaderUrl.href;
    script.integrity = sha256Integrity(files['blender.js'].sha256);
    script.crossOrigin = 'anonymous';
    script.onerror = () => fail('could not load Blender loader');
    document.head.append(script);
  }

  if (window.parent === window) {
    statusNode.textContent = 'This runtime must be opened by the demo page.';
  } else {
    start().catch(error => fail(error instanceof Error ? error.message : 'runtime startup failed'));
  }
})();
