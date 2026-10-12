const MAX_COMMAND_BYTES = 256 * 1024;
const MAX_OUTPUT_BYTES = 32 * 1024 * 1024;
const MAX_ORDINARY_OUTPUT = 64 * 1024;
const MAX_OBJECTS = 64;
const MAX_VERTICES = 100_000;
const MAX_TRIANGLES = 200_000;
const KINDS = new Set(['cube', 'uv_sphere', 'cylinder', 'cone', 'torus', 'plane']);
const EXPECTED_BUILD_HASH = 'fbe6228777e7';
const RUN_TIMEOUT_MS = 180_000;
const READY_TIMEOUT_MS = 30_000;
const CHANNEL = 'blr-scene-v1';

function protocolError(message) {
  return new Error(`Blender scene runtime: ${message}`);
}

function bytesOf(value) {
  return new TextEncoder().encode(value).byteLength;
}

function validateCommands(input) {
  if (typeof input !== 'string') throw protocolError('commands must be supplied as their original JSON text');
  const json = input;
  if (typeof json !== 'string' || bytesOf(json) > MAX_COMMAND_BYTES) throw protocolError('commands exceed the 256 KiB limit');
  let commands;
  try {
    commands = JSON.parse(json);
  } catch {
    throw protocolError('commands are not valid JSON');
  }
  if (!commands || typeof commands !== 'object' || Array.isArray(commands)
    || commands.schema !== 'blr-scene-commands-v1' || !Array.isArray(commands.objects)
    || commands.objects.length > MAX_OBJECTS) {
    throw protocolError('commands have an unsupported schema or object count');
  }
  if (Object.keys(commands).some(key => !['schema', 'objects'].includes(key))) {
    throw protocolError('commands contain an unsupported field');
  }
  const names = new Set();
  const allowed = new Set([
    'kind', 'name', 'location', 'rotation', 'scale', 'color', 'size', 'radius', 'radius1', 'radius2',
    'depth', 'segments', 'ring_count', 'major_segments', 'minor_segments', 'major_radius', 'minor_radius',
  ]);
  for (const object of commands.objects) {
    if (!object || typeof object !== 'object' || Array.isArray(object)
      || typeof object.name !== 'string' || object.name.length < 1 || bytesOf(object.name) > 128
      || names.has(object.name) || Object.keys(object).some(key => !allowed.has(key))) {
      throw protocolError('each command needs a unique, bounded object name');
    }
    if (typeof object.kind !== 'string' || !KINDS.has(object.kind)) throw protocolError('command primitive kind is unsupported');
    names.add(object.name);
    for (const field of ['location', 'rotation', 'scale']) {
      if (object[field] === undefined) continue;
      const vector = object[field];
      if (!Array.isArray(vector) || vector.length !== 3
        || !vector.every(value => finiteNumber(value) && Math.abs(value) <= 1000)
        || (field === 'scale' && vector.some(value => value === 0))) {
        throw protocolError(`command ${field} coordinates must be finite and within the 1000 bound${field === 'scale' ? ' with no zero component' : ''}`);
      }
    }
    if (object.color !== undefined && (!Array.isArray(object.color) || object.color.length !== 4
      || !object.color.every(value => finiteNumber(value) && value >= 0 && value <= 1))) {
      throw protocolError('command color must be linear RGBA in the 0 through 1 range');
    }
    for (const field of ['size', 'radius', 'radius1', 'radius2', 'depth', 'major_radius', 'minor_radius']) {
      if (object[field] !== undefined && (!finiteNumber(object[field]) || object[field] < 0 || object[field] > 1000)) {
        throw protocolError(`command ${field} must be a finite value from 0 through 1000`);
      }
    }
    for (const field of ['segments', 'ring_count', 'major_segments', 'minor_segments']) {
      if (object[field] !== undefined && (!Number.isSafeInteger(object[field]) || object[field] < 3 || object[field] > 128)) {
        throw protocolError(`command ${field} must be an integer from 3 through 128`);
      }
    }
    if (object.kind === 'torus' && (object.minor_radius ?? 0.25) >= (object.major_radius ?? 1)) {
      throw protocolError('torus minor radius must be smaller than major radius');
    }
  }
  return json;
}

function finiteNumber(value) {
  return typeof value === 'number' && Number.isFinite(value);
}

function validateScene(scene) {
  if (!scene || typeof scene !== 'object' || Array.isArray(scene)
    || scene.schema !== 'blr-scene-v1' || !Array.isArray(scene.objects)
    || Object.keys(scene).some(key => !['schema', 'objects'].includes(key))
    || scene.objects.length > MAX_OBJECTS) {
    throw protocolError('Blender returned an unsupported scene schema');
  }
  const names = new Set();
  let vertices = 0;
  let triangles = 0;
  for (const object of scene.objects) {
    if (!object || typeof object !== 'object' || Array.isArray(object)
      || typeof object.name !== 'string' || object.name.length < 1 || bytesOf(object.name) > 256
      || names.has(object.name) || !Array.isArray(object.positions) || !Array.isArray(object.triangles)
      || !Array.isArray(object.color) || object.color.length !== 4
      || Object.keys(object).some(key => !['name', 'positions', 'triangles', 'color'].includes(key))) {
      throw protocolError('Blender returned malformed or duplicate object geometry');
    }
    names.add(object.name);
    vertices += object.positions.length;
    triangles += object.triangles.length;
    if (vertices > MAX_VERTICES || triangles > MAX_TRIANGLES) {
      throw protocolError('Blender geometry exceeds the vertex or triangle limit');
    }
    if (!object.color.every(value => finiteNumber(value) && value >= 0 && value <= 1)) {
      throw protocolError('Blender returned an invalid linear RGBA color');
    }
    for (const position of object.positions) {
      if (!Array.isArray(position) || position.length !== 3
        || !position.every(value => finiteNumber(value) && Math.abs(value) <= 1_000_000)) {
        throw protocolError('Blender returned an invalid world-space position');
      }
    }
    for (const triangle of object.triangles) {
      if (!Array.isArray(triangle) || triangle.length !== 3
        || !triangle.every(index => Number.isSafeInteger(index) && index >= 0 && index < object.positions.length)) {
        throw protocolError('Blender returned an invalid triangle index');
      }
    }
  }
  if (bytesOf(JSON.stringify(scene)) > MAX_OUTPUT_BYTES) {
    throw protocolError('Blender geometry exceeds the 32 MiB output limit');
  }
  return { scene, vertices, triangles };
}

function randomRunId() {
  const bytes = crypto.getRandomValues(new Uint8Array(32));
  return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
}

function abortError() {
  return new DOMException('Blender scene run canceled', 'AbortError');
}

async function waitForIsolation(promise, signal) {
  let timeoutId = 0;
  let onAbort;
  const waiting = [Promise.resolve(promise)];
  waiting.push(new Promise((_, reject) => {
    timeoutId = setTimeout(() => reject(protocolError('browser isolation setup timed out')), 15_000);
    if (signal) {
      onAbort = () => reject(abortError());
      signal.addEventListener('abort', onAbort, { once: true });
    }
  }));
  try {
    await Promise.race(waiting);
  } finally {
    clearTimeout(timeoutId);
    if (onAbort) signal.removeEventListener('abort', onAbort);
  }
  if (signal?.aborted) throw abortError();
}

export async function runBlenderScene(commands, { signal, onStatus, onOutput } = {}) {
  const commandJson = validateCommands(commands);
  if (signal?.aborted) throw abortError();
  if (typeof window === 'undefined' || typeof document === 'undefined') {
    throw protocolError('a browser document is required');
  }
  if (!window.isSecureContext) throw protocolError('a secure browser context is required');
  if (!window.blrIsolationReady || typeof window.blrIsolationReady.then !== 'function') {
    throw protocolError('cross-origin isolation setup is unavailable');
  }
  try {
    if (typeof onStatus === 'function') onStatus('Preparing browser isolation');
  } catch {
    // Status rendering cannot interrupt the runtime lifecycle.
  }
  await waitForIsolation(window.blrIsolationReady, signal);
  if (!window.crossOriginIsolated || typeof SharedArrayBuffer === 'undefined') {
    throw protocolError('secure cross-origin isolation is required');
  }
  if (signal?.aborted) throw abortError();

  const runId = randomRunId();
  const frame = document.createElement('iframe');
  frame.hidden = true;
  frame.title = 'Isolated headless Blender scene runtime';
  frame.setAttribute('aria-hidden', 'true');
  const frameUrl = new URL('./runtime-frame.html', import.meta.url);
  frameUrl.searchParams.set('run', runId);
  frame.src = frameUrl.href;

  return new Promise((resolve, reject) => {
    let timer = 0;
    let outputBytes = 0;
    let ready = false;
    let result = null;
    let identityRecord = null;
    let exitSeen = false;
    let exitCode = null;
    let settled = false;
    const cleanup = () => {
      clearTimeout(timer);
      window.removeEventListener('message', onMessage);
      frame.removeEventListener('error', onFrameError);
      signal?.removeEventListener('abort', onAbort);
      frame.remove();
    };
    const finish = (error, value) => {
      if (settled) return;
      settled = true;
      cleanup();
      if (error) reject(error);
      else resolve(value);
    };
    const status = text => {
      if (typeof onStatus === 'function') {
        try { onStatus(String(text).slice(0, 160)); } catch { /* status rendering cannot interrupt the run */ }
      }
    };
    const onAbort = () => finish(abortError());
    const onFrameError = () => finish(protocolError('runtime frame failed to load'));
    const onMessage = event => {
      if (event.source !== frame.contentWindow || event.origin !== location.origin) return;
      const data = event.data;
      if (!data || data.channel !== CHANNEL || data.runId !== runId || typeof data.type !== 'string') return;
      if (data.type === 'ready') {
        if (ready) return finish(protocolError('runtime sent duplicate ready messages'));
        ready = true;
        clearTimeout(timer);
        timer = setTimeout(() => finish(protocolError('Blender run exceeded 180 seconds')), RUN_TIMEOUT_MS);
        status('Blender runtime ready');
        frame.contentWindow.postMessage({ channel: CHANNEL, runId, type: 'run', commandText: commandJson }, location.origin);
        return;
      }
      if (!ready) return finish(protocolError('runtime sent data before the ready handshake'));
      if (data.type === 'status') {
        if (typeof data.status !== 'string' || data.status.length > 160) return finish(protocolError('runtime sent an invalid status'));
        status(data.status);
        return;
      }
      if (data.type === 'output') {
        if ((data.stream !== 'stdout' && data.stream !== 'stderr') || typeof data.text !== 'string'
          || data.text.length > 4096) return finish(protocolError('runtime sent malformed output'));
        outputBytes += bytesOf(data.text);
        if (outputBytes > MAX_ORDINARY_OUTPUT) return finish(protocolError('runtime diagnostic output exceeded 64 KiB'));
        if (typeof onOutput === 'function') {
          try { onOutput({ stream: data.stream, text: data.text }); } catch (error) {
            return finish(error instanceof Error ? error : protocolError('output callback failed'));
          }
        }
        return;
      }
      if (data.type === 'result') {
        if (result) return finish(protocolError('runtime sent more than one scene result'));
        try {
          if (data.identity?.inventory_unchanged !== true) throw protocolError('Blender did not prove scene inventory preservation');
          if (data.identity?.module !== '_blr_browser_probe' || data.identity?.origin !== 'built-in'
            || data.identity?.has_file !== false || data.identity?.function_module !== '_blr_browser_probe'
            || JSON.stringify(data.identity.blender_version) !== '[5,2,0]'
            || typeof data.identity.python_version !== 'string' || !/^3\.13\./.test(data.identity.python_version)
            || data.identity.blender_build_hash !== EXPECTED_BUILD_HASH) {
            throw protocolError('Blender or built-in Rust identity was not proven');
          }
          result = validateScene(data.scene).scene;
          identityRecord = data.identity;
        } catch (error) {
          return finish(error instanceof Error ? error : protocolError('invalid scene result'));
        }
        if (exitSeen) finish(exitCode === 0 ? null : protocolError(`Blender exited with status ${exitCode}`),
          exitCode === 0 ? { scene: result, identity: data.identity } : undefined);
        return;
      }
      if (data.type === 'exit') {
        if (exitSeen || !Number.isInteger(data.status)) return finish(protocolError('runtime sent an invalid exit status'));
        exitSeen = true;
        exitCode = data.status;
        if (exitCode !== 0) return finish(protocolError(`Blender exited with status ${exitCode}`));
        if (result) finish(null, { scene: result, identity: identityRecord });
        return;
      }
      if (data.type === 'error') {
        const detail = typeof data.message === 'string' ? data.message.slice(0, 512) : 'runtime failed';
        finish(protocolError(detail));
      }
    };
    window.addEventListener('message', onMessage);
    frame.addEventListener('error', onFrameError, { once: true });
    signal?.addEventListener('abort', onAbort, { once: true });
    timer = setTimeout(() => finish(protocolError('runtime ready handshake timed out')), READY_TIMEOUT_MS);
    document.body.append(frame);
    status('Starting isolated Blender runtime');
  });
}
