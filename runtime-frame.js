import { sceneProtocolLimits, validateSceneCommands } from "./protocol/validate.js?blr_app=fe2424e19037e1e8e7d38a3c9db9c0b23a65146b5a381da78ddc353447f3e9ba";

(() => {
  'use strict';

  const LEGACY_CHANNEL = 'blr-demo';
  const MAX_CAPTURED_OUTPUT = 64 * 1024;
  const MAX_SCENE_OUTPUT = 32 * 1024 * 1024;
  const MAX_PYTHON_SOURCE_BYTES = 256 * 1024;
  const MAX_PYTHON_DIAGNOSTIC_BYTES = 64 * 1024;
  const MAX_OBJECTS = sceneProtocolLimits.objects;
  const MAX_VERTICES = sceneProtocolLimits.vertices;
  const MAX_TRIANGLES = sceneProtocolLimits.triangles;
  const SCENE_CHANNEL = 'blr-scene-v1';
  const EXPECTED_BUILD_HASH = 'fbe6228777e7';
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
  let inputReceived = false;
  let legacySize = null;
  let runtimeMode = null;
  const pendingProgress = new Map();
  const startupAbort = new AbortController();

  function post(type, values = {}) {
    window.parent.postMessage({ channel: runChannel, runId, type, ...values }, location.origin);
  }

  function setStatus(state) {
    statusNode.textContent = state;
    post('status', { status: state });
  }

  function boundedUtf8(value, maximum = 512) {
    let text = '';
    let bytes = 0;
    for (const character of String(value)) {
      const codepoint = character.codePointAt(0);
      const size = codepoint <= 0x7f ? 1 : codepoint <= 0x7ff ? 2 : codepoint <= 0xffff ? 3 : 4;
      if (bytes + size > maximum) break;
      text += character;
      bytes += size;
    }
    return text;
  }

  function progress(phase, asset, state, loadedBytes = null, totalBytes = null, source = null, error = null) {
    if (failed) return;
    const record = { phase, asset, state, loadedBytes, totalBytes, source, error };
    const key = `${phase}\0${asset || ''}`;
    if (state === 'start' || state === 'update') pendingProgress.set(key, record);
    else pendingProgress.delete(key);
    try { post('progress', { progress: record }); }
    catch { /* progress reporting is advisory */ }
  }

  const runId = params.get('run') || '';
  const runChannel = params.has('size') ? LEGACY_CHANNEL : SCENE_CHANNEL;
  function fail(message) {
    if (failed) return;
    const detail = boundedUtf8(message);
    for (const record of pendingProgress.values()) {
      progress(record.phase, record.asset, 'error', record.loadedBytes, record.totalBytes, record.source, detail);
    }
    pendingProgress.clear();
    failed = true;
    startupAbort.abort();
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

  async function readBounded(stream, maximum, label, onChunk = null) {
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
        try { onChunk?.(length); } catch { /* progress reporting is advisory */ }
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

  async function fetchVerified(url, metadata, label, asset) {
    const maximum = label === 'Blender loader' ? MAX_LOADER_BYTES : MAX_COMPRESSED_WASM_BYTES;
    let loaded = 0;
    let phase = 'fetch';
    progress('fetch', asset, 'start', 0, metadata.size, 'network-or-cache');
    try {
      const response = await fetch(url, { credentials: 'same-origin', cache: 'force-cache', signal: startupAbort.signal });
      if (!response.ok) throw new Error(`${label} request failed (${response.status})`);
      const bytes = await readBounded(response.body, maximum, label, count => {
        loaded = count;
        progress('fetch', asset, 'update', count, metadata.size, 'network-or-cache');
      });
      progress('fetch', asset, 'complete', loaded, metadata.size, 'network-or-cache');
      phase = 'verify';
      progress('verify', asset, 'start', null, null, 'work');
      if (bytes.byteLength !== metadata.size) throw new Error(`${label} size does not match the runtime manifest`);
      if (await sha256(bytes) !== metadata.sha256) throw new Error(`${label} SHA-256 does not match the runtime manifest`);
      progress('verify', asset, 'complete', null, null, 'work');
      return bytes;
    } catch (error) {
      progress(phase, asset, 'error', phase === 'fetch' ? loaded : null, phase === 'fetch' ? metadata.size : null,
        phase === 'fetch' ? 'network-or-cache' : 'work', boundedUtf8(error?.message || error));
      throw error;
    }
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
      'raw_build_hash = bpy.app.build_hash',
      'build_hash = raw_build_hash.decode("ascii", "strict") if isinstance(raw_build_hash, bytes) else raw_build_hash',
      `if not isinstance(build_hash, str) or not build_hash.isascii() or build_hash != "${EXPECTED_BUILD_HASH}": raise AssertionError("unexpected Blender build hash: %r" % (build_hash,))`,
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
      'record = {"case": "browser-cube", "blender_version": list(version), "python_version": sys.version, "blender_build_hash": build_hash, "rust_module": identity, "requested_size": requested, "result": list(result), "inventory_unchanged": True}',
      'print("BLR_BROWSER_PROBE_RESULT " + json.dumps(record, separators=(",", ":")))',
    ].join('\n');
    return `exec(${JSON.stringify(code)})`;
  }

  function makeScenePythonExpression(commandJson) {
    const code = [
      'import bpy, json, sys',
      'probe = __import__("_blr_browser_probe")',
      'version = tuple(bpy.app.version)',
      'if version != (5, 2, 0): raise RuntimeError("expected Blender 5.2.0; observed %r" % (version,))',
      'if hasattr(probe, "__file__"): raise AssertionError("Rust module unexpectedly has __file__")',
      'spec = getattr(probe, "__spec__", None)',
      'origin = getattr(spec, "origin", None)',
      'if origin != "built-in": raise AssertionError("Rust module is not built-in: %r" % (origin,))',
      'raw_build_hash = bpy.app.build_hash',
      'build_hash = raw_build_hash.decode("ascii", "strict") if isinstance(raw_build_hash, bytes) else raw_build_hash',
      `if not isinstance(build_hash, str) or not build_hash.isascii() or build_hash != "${EXPECTED_BUILD_HASH}": raise AssertionError("unexpected Blender build hash: %r" % (build_hash,))`,
      'def inventory():',
      '    data = bpy.data',
      '    return tuple(sorted(item.session_uid for item in collection) for collection in (data.objects, data.meshes, data.materials))',
      'before = inventory()',
      `command_json = ${JSON.stringify(commandJson)}`,
      'scene_json = probe.run_scene(command_json)',
      'if not isinstance(scene_json, str): raise AssertionError("Rust scene callback did not return JSON text")',
      'after = inventory()',
      'if before != after: raise AssertionError("Rust scene call changed object/mesh/material session_uid inventories")',
      'scene = json.loads(scene_json)',
      'identity = {"module": probe.__name__, "origin": origin, "has_file": False, "function_module": probe.run_scene.__module__, "blender_version": list(version), "python_version": sys.version, "blender_build_hash": build_hash, "inventory_unchanged": True}',
      'record = {"scene": scene, "identity": identity}',
      'print("BLR_BROWSER_SCENE_RESULT " + json.dumps(record, separators=(",", ":"), ensure_ascii=False))',
    ].join('\n');
    return `exec(${JSON.stringify(code)})`;
  }

  function makePythonSceneExpression(source) {
    const code = [
      'import bpy, json, sys',
      'probe = __import__("_blr_browser_probe")',
      'version = tuple(bpy.app.version)',
      'if version != (5, 2, 0): raise RuntimeError("expected Blender 5.2.0; observed %r" % (version,))',
      'if hasattr(probe, "__file__"): raise AssertionError("Rust module unexpectedly has __file__")',
      'spec = getattr(probe, "__spec__", None)',
      'origin = getattr(spec, "origin", None)',
      'if origin != "built-in": raise AssertionError("Rust module is not built-in: %r" % (origin,))',
      'raw_build_hash = bpy.app.build_hash',
      'build_hash = raw_build_hash.decode("ascii", "strict") if isinstance(raw_build_hash, bytes) else raw_build_hash',
      `if not isinstance(build_hash, str) or not build_hash.isascii() or build_hash != "${EXPECTED_BUILD_HASH}": raise AssertionError("unexpected Blender build hash: %r" % (build_hash,))`,
      `source = ${JSON.stringify(source)}`,
      'result_text = probe.run_python_scene(source)',
      'if not isinstance(result_text, str): raise AssertionError("Python scene callback did not return JSON text")',
      'record = json.loads(result_text)',
      'record["identity"] = {"module": probe.__name__, "origin": origin, "has_file": False, "function_module": probe.run_python_scene.__module__, "blender_version": list(version), "python_version": sys.version, "blender_build_hash": build_hash, "inventory_unchanged": True}',
      'print("BLR_BROWSER_PYTHON_RESULT " + json.dumps(record, separators=(",", ":"), ensure_ascii=False))',
    ].join('\n');
    return `exec(${JSON.stringify(code)})`;
  }

  function validateScene(scene) {
    if (!scene || typeof scene !== 'object' || Array.isArray(scene)
      || scene.schema !== 'blr-scene-v1' || !Array.isArray(scene.objects)
      || scene.objects.length > MAX_OBJECTS) throw new Error('Blender returned an unsupported scene schema');
    const names = new Set();
    let vertices = 0;
    let triangles = 0;
    for (const object of scene.objects) {
      if (!object || typeof object !== 'object' || Array.isArray(object)
        || typeof object.name !== 'string' || !object.name || new TextEncoder().encode(object.name).byteLength > 256
        || names.has(object.name) || !Array.isArray(object.positions) || !Array.isArray(object.triangles)
        || !Array.isArray(object.color) || object.color.length !== 4
        || Object.keys(object).some(key => !['name', 'positions', 'triangles', 'color'].includes(key))) throw new Error('Blender returned malformed scene geometry');
      names.add(object.name);
      vertices += object.positions.length;
      triangles += object.triangles.length;
      if (vertices > MAX_VERTICES || triangles > MAX_TRIANGLES) throw new Error('Blender geometry exceeds the vertex or triangle limit');
      if (!object.color.every(value => Number.isFinite(value) && value >= 0 && value <= 1)) throw new Error('Blender returned invalid linear RGBA color');
      for (const position of object.positions) {
        if (!Array.isArray(position) || position.length !== 3
          || !position.every(value => Number.isFinite(value) && Math.abs(value) <= 1000000)) throw new Error('Blender returned invalid world-space positions');
      }
      for (const triangle of object.triangles) {
        if (!Array.isArray(triangle) || triangle.length !== 3
          || !triangle.every(index => Number.isSafeInteger(index) && index >= 0 && index < object.positions.length)) throw new Error('Blender returned invalid triangle indices');
      }
    }
    if (new TextEncoder().encode(JSON.stringify(scene)).byteLength > MAX_SCENE_OUTPUT) throw new Error('Blender geometry exceeds the 32 MiB output limit');
    return scene;
  }

  function parseRuntimeOutput(line) {
    const scenePrefix = 'BLR_BROWSER_SCENE_RESULT ';
    const pythonPrefix = 'BLR_BROWSER_PYTHON_RESULT ';
    const cubePrefix = 'BLR_BROWSER_PROBE_RESULT ';
    if (line.startsWith(scenePrefix)) {
      if (line.length > MAX_SCENE_OUTPUT + MAX_CAPTURED_OUTPUT) return fail('Blender scene result exceeded the output limit');
      if (resultRecord) return fail('Blender emitted more than one result record');
      try {
        const record = JSON.parse(line.slice(scenePrefix.length));
        const rust = record.identity;
        const scene = validateScene(record.scene);
        if (!rust || rust.module !== '_blr_browser_probe' || rust.origin !== 'built-in'
          || rust.has_file !== false || rust.function_module !== '_blr_browser_probe'
          || JSON.stringify(rust.blender_version) !== '[5,2,0]'
          || typeof rust.python_version !== 'string' || !/^3\.13\./.test(rust.python_version)
          || rust.blender_build_hash !== EXPECTED_BUILD_HASH
          || rust.inventory_unchanged !== true) throw new Error('Blender identity or inventory preservation was not proven');
        resultRecord = { scene, identity: rust };
      } catch (error) {
        return fail(error instanceof Error ? error.message : 'invalid Blender scene result');
      }
      return;
    }
    if (line.startsWith(pythonPrefix)) {
      if (line.length > MAX_SCENE_OUTPUT + 3 * MAX_PYTHON_DIAGNOSTIC_BYTES + 8192) return fail('Python scene result exceeded the output limit');
      if (resultRecord) return fail('Blender emitted more than one result record');
      try {
        const record = JSON.parse(line.slice(pythonPrefix.length));
        const expected = ['schema', 'ok', 'scene', 'lease_token', 'missing', 'stdout', 'stderr', 'traceback', 'identity'].sort().join(',');
        if (!record || typeof record !== 'object' || Array.isArray(record)
          || Object.keys(record).sort().join(',') !== expected || record.schema !== 'blr-python-scene-v1'
          || typeof record.ok !== 'boolean' || !Array.isArray(record.missing) || record.missing.length > MAX_OBJECTS
          || (record.lease_token !== null && typeof record.lease_token !== 'string')) throw new Error('Python result envelope is malformed');
        for (const field of ['stdout', 'stderr', 'traceback']) {
          const value = record[field];
          if (field === 'traceback' && value === null) continue;
          if (typeof value !== 'string' || new TextEncoder().encode(value).byteLength > MAX_PYTHON_DIAGNOSTIC_BYTES) throw new Error(`Python ${field} exceeds its byte limit`);
        }
        const rust = record.identity;
        if (!rust || rust.module !== '_blr_browser_probe' || rust.origin !== 'built-in'
          || rust.has_file !== false || rust.function_module !== '_blr_browser_probe'
          || JSON.stringify(rust.blender_version) !== '[5,2,0]'
          || typeof rust.python_version !== 'string' || !/^3\.13\./.test(rust.python_version)
          || rust.blender_build_hash !== EXPECTED_BUILD_HASH || rust.inventory_unchanged !== true) {
          throw new Error('Python Blender identity or cleanup was not proven');
        }
        if (record.ok) {
          record.scene = validateScene(record.scene);
          if (record.lease_token !== null || record.missing.length !== 0) throw new Error('successful headless Python result must not carry a GUI lease');
        } else if (record.scene !== null || record.lease_token !== null || record.missing.length !== 0) {
          throw new Error('failed headless Python result contains a scene lease');
        }
        resultRecord = record;
      } catch (error) {
        return fail(error instanceof Error ? error.message : 'invalid Blender Python result');
      }
      return;
    }
    if (line.startsWith(cubePrefix)) {
      capturedOutput += line.length;
      if (capturedOutput > MAX_CAPTURED_OUTPUT) return fail('Blender output exceeded the 64 KiB limit');
      if (resultRecord) return fail('Blender emitted more than one result record');
      try {
        const record = JSON.parse(line.slice(cubePrefix.length));
        const expectedSize = Number(params.get('size'));
        const rust = record.rust_module;
        if (record.case !== 'browser-cube' || JSON.stringify(record.blender_version) !== '[5,2,0]'
          || typeof record.python_version !== 'string' || !/^3\.13\./.test(record.python_version)
          || record.blender_build_hash !== EXPECTED_BUILD_HASH
          || !rust || rust.module !== '_blr_browser_probe' || rust.origin !== 'built-in'
          || rust.has_file !== false || rust.function_module !== '_blr_browser_probe'
          || record.requested_size !== expectedSize || !Array.isArray(record.result) || record.result.length !== 4
          || record.result[0] !== expectedSize || record.result[1] !== 8 || record.result[2] !== 12
          || record.result[3] !== 12 || record.inventory_unchanged !== true) throw new Error('Blender result did not satisfy the built-in cube contract');
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
      return;
    }
    capturedOutput += String(line).length;
    if (capturedOutput > MAX_CAPTURED_OUTPUT) return fail('Blender output exceeded the 64 KiB limit');
    post('output', { stream: 'stdout', text: String(line).slice(0, 4096) });
  }

  async function startRuntime(mode) {
    if (failed) return;
    runtimeMode = mode;
    if (!('DecompressionStream' in window) || !crypto.subtle) throw new Error('this browser cannot verify the runtime files');
    setStatus('loading');
    progress('fetch', 'runtime/manifest.json', 'start', 0, null, 'network-or-cache');
    const manifestResponse = await fetch(new URL('runtime/manifest.json', location.href), {
      credentials: 'same-origin', cache: 'no-store', signal: startupAbort.signal,
    });
    if (!manifestResponse.ok) throw new Error(`runtime manifest request failed (${manifestResponse.status})`);
    let manifestLoaded = 0;
    const manifestBytes = await readBounded(manifestResponse.body, MAX_MANIFEST_BYTES, 'runtime manifest', count => {
      manifestLoaded = count;
      progress('fetch', 'runtime/manifest.json', 'update', count, null, 'network-or-cache');
    });
    progress('fetch', 'runtime/manifest.json', 'complete', manifestLoaded, null, 'network-or-cache');
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
      || !compressedUrl.pathname.startsWith(runtimeRoot.pathname)) throw new Error('runtime path escaped its same-origin directory');
    const [loaderBytes, compressedBytes] = await Promise.all([
      fetchVerified(loaderUrl, files['blender.js'], 'Blender loader', 'blender.js'),
      fetchVerified(compressedUrl, files['blender.wasm.gz'], 'compressed Blender runtime', 'blender.wasm.gz'),
    ]);
    progress('unpack', 'blender.wasm', 'start', 0, files['blender.wasm'].size, 'work');
    let unpacked = 0;
    const decompressed = await readBounded(
      new Blob([compressedBytes]).stream().pipeThrough(new DecompressionStream('gzip')),
      MAX_WASM_BYTES,
      'decompressed Wasm',
      count => {
        unpacked = count;
        progress('unpack', 'blender.wasm', 'update', count, files['blender.wasm'].size, 'work');
      },
    );
    progress('unpack', 'blender.wasm', 'complete', unpacked, files['blender.wasm'].size, 'work');
    progress('verify', 'blender.wasm', 'start', null, null, 'work');
    if (decompressed.byteLength !== files['blender.wasm'].size) throw new Error('decompressed Wasm size does not match the runtime manifest');
    if (await sha256(decompressed) !== files['blender.wasm'].sha256) throw new Error('decompressed Wasm SHA-256 does not match the runtime manifest');
    progress('verify', 'blender.wasm', 'complete', null, null, 'work');
    setStatus('starting Blender 5.2');
    window.Module = {
      wasmBinary: decompressed,
      arguments: [
        '--background', '--factory-startup', '--threads', '1',
        '--python-exit-code', '1', '--python-expr', mode.kind === 'scene'
          ? makeScenePythonExpression(mode.commandJson) : mode.kind === 'python'
            ? makePythonSceneExpression(mode.source) : makePythonExpression(mode.size),
      ],
      preRun: [() => {
        if (!window.Module.ENV || typeof window.Module.ENV !== 'object') throw new Error('Blender runtime did not export its environment interface');
        Object.assign(window.Module.ENV, ALLOWED_ENV);
      }],
      print: parseRuntimeOutput,
      printErr: line => {
        capturedOutput += String(line).length;
        if (capturedOutput > MAX_CAPTURED_OUTPUT) return fail('Blender output exceeded the 64 KiB limit');
        post('output', { stream: 'stderr', text: String(line).slice(0, 4096) });
      },
      onRuntimeInitialized: () => {
        progress('initialize', null, 'complete', null, null, 'work');
        progress('execute', runtimeMode?.kind || null, 'start', null, null, 'work');
        setStatus('running Blender 5.2');
      },
      onAbort: reason => fail(`Blender runtime aborted: ${String(reason).slice(0, 384)}`),
      onExit: status => {
        const code = Number(status);
        post('exit', { status: Number.isInteger(code) ? code : -1 });
        if (code !== 0) {
          progress('execute', runtimeMode?.kind || null, 'error', null, null, 'work', `Blender exited with status ${String(status)}`);
          fail(`Blender exited with status ${String(status)}`);
        }
        else if (!resultRecord) fail('Blender exited successfully without a valid result');
        else if (!failed) {
          const pythonFailed = resultRecord.ok === false;
          statusNode.textContent = pythonFailed ? 'Python execution failed' : 'Blender demo complete';
          progress('execute', runtimeMode?.kind || null, pythonFailed ? 'error' : 'complete', null, null, 'work',
            pythonFailed ? boundedUtf8(resultRecord.traceback || 'Python execution failed') : null);
          post('result', resultRecord);
        }
      },
      locateFile: (path, prefix) => {
        if (path !== 'blender.wasm') throw new Error(`unexpected Blender runtime file: ${path}`);
        const resolved = new URL(path, prefix || runtimeRoot.href);
        if (resolved.origin !== location.origin || resolved.href !== new URL('blender.wasm', runtimeRoot).href) throw new Error('Blender runtime requested a file outside its runtime directory');
        return resolved.href;
      },
    };
    progress('initialize', null, 'start', null, null, 'work');
    const script = document.createElement('script');
    script.src = loaderUrl.href;
    script.integrity = sha256Integrity(files['blender.js'].sha256);
    script.crossOrigin = 'anonymous';
    script.onerror = () => fail('could not load Blender loader');
    document.head.append(script);
  }

  function receiveInput(event) {
    if (event.source !== window.parent || event.origin !== location.origin) return;
    const data = event.data;
    if (!data || data.channel !== SCENE_CHANNEL || data.runId !== runId || data.type !== 'run') return;
    if (inputReceived || legacySize !== null) return fail('runtime received duplicate scene input');
    inputReceived = true;
    try {
      if (data.language === 'rust' && Object.keys(data).sort().join(',') === 'channel,commandText,language,runId,type') {
        validateSceneCommands(data.commandText);
        startRuntime({ kind: 'scene', commandJson: data.commandText }).catch(error => fail(error instanceof Error ? error.message : 'runtime startup failed'));
      } else if (data.language === 'python' && Object.keys(data).sort().join(',') === 'channel,language,runId,source,type') {
        if (typeof data.source !== 'string' || new TextEncoder().encode(data.source).byteLength > MAX_PYTHON_SOURCE_BYTES) {
          throw new Error('Python source exceeds the 256 KiB runtime limit');
        }
        startRuntime({ kind: 'python', source: data.source }).catch(error => fail(error instanceof Error ? error.message : 'runtime startup failed'));
      } else throw new Error('runtime received an unsupported language request');
    } catch (error) {
      fail(error instanceof Error ? error.message : 'invalid scene commands');
    }
  }

  async function start() {
    if (params.getAll('run').length !== 1 || !/^[A-Za-z0-9._:-]{1,128}$/.test(runId)) throw new Error('missing or invalid run identifier');
    if (params.getAll('size').length > 1) throw new Error('duplicate runtime size parameter');
    if (params.has('size')) {
      if (!/^(?:\d+(?:\.\d*)?|\.\d+)$/.test(params.get('size') || '')) throw new Error('size must be a number from 0.1 through 4');
      legacySize = Number(params.get('size'));
      if (!Number.isFinite(legacySize) || legacySize < 0.1 || legacySize > 4) throw new Error('size must be a number from 0.1 through 4');
    }
    if (params.getAll('blr_app').length > 1
      || (params.has('blr_app') && !/^[a-f0-9]{64}$/.test(params.get('blr_app') || ''))) {
      throw new Error('invalid application revision');
    }
    if ([...params.keys()].some(key => key !== 'run' && key !== 'size' && key !== 'blr_app')) throw new Error('unsupported runtime URL parameter');
    if (!window.blrIsolationReady || typeof window.blrIsolationReady.then !== 'function') throw new Error('cross-origin isolation setup is unavailable');
    await window.blrIsolationReady;
    if (legacySize !== null) startRuntime({ kind: 'cube', size: legacySize }).catch(error => fail(error instanceof Error ? error.message : 'runtime startup failed'));
    else {
      window.addEventListener('message', receiveInput);
      post('ready');
    }
  }

  if (window.parent === window) {
    statusNode.textContent = 'This runtime must be opened by the demo page.';
  } else {
    start().catch(error => fail(error instanceof Error ? error.message : 'runtime startup failed'));
  }
})();
