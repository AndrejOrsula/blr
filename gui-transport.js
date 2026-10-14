import { validateSceneCommands } from "./protocol/validate.js?blr_app=a04fd988c48d7984cfbb2a4470a1017f784b506d995b7dff1ad87a782f3fe926";

const CHANNEL = 'blr-gui-transport-v1';
const MAX_QUEUE = 8;
const MAX_COMMAND_BYTES = 256 * 1024;
const MAX_PYTHON_SOURCE_BYTES = 256 * 1024;
const MAX_SCENE_BYTES = 32 * 1024 * 1024;
const MAX_PYTHON_DIAGNOSTIC_BYTES = 64 * 1024;
const MAX_RESULT_BYTES = MAX_SCENE_BYTES + 3 * MAX_PYTHON_DIAGNOSTIC_BYTES + 8192;
const MAX_SMALL_BYTES = 4096;
const typedArrayTag = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), Symbol.toStringTag)?.get;
const MAX_OBJECTS = 64;
const MAX_VERTICES = 100_000;
const MAX_TRIANGLES = 200_000;
const REQUEST_TIMEOUT_MS = 180_000;
const START_TIMEOUT_MS = 180_000;
const POLL_MS = 50;
const EXPECTED_BUILD_HASH = 'fbe6228777e7';

function guiError(message) {
  return new Error(`Blender GUI transport: ${message}`);
}

function abortError() {
  return new DOMException('Blender GUI request canceled', 'AbortError');
}

function utf8Bytes(value) {
  return new TextEncoder().encode(value).byteLength;
}

function isUint8Array(value) {
  return ArrayBuffer.isView(value) && typedArrayTag?.call(value) === 'Uint8Array';
}

function decodeJson(bytes, label) {
  try {
    const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
    rejectDuplicateJsonKeys(text);
    return JSON.parse(text);
  } catch {
    throw guiError(`${label} is invalid JSON`);
  }
}

function rejectDuplicateJsonKeys(text) {
  let offset = 0;
  const whitespace = () => { while (/\s/.test(text[offset] || '') && offset < text.length) offset += 1; };
  function stringToken() {
    const start = offset;
    if (text[offset++] !== '"') throw new Error('expected JSON string');
    while (offset < text.length) {
      const code = text.charCodeAt(offset++);
      if (code === 0x22) return JSON.parse(text.slice(start, offset));
      if (code === 0x5c) {
        if (offset >= text.length) throw new Error('truncated JSON escape');
        offset += 1;
      } else if (code < 0x20) throw new Error('control character in JSON string');
    }
    throw new Error('unterminated JSON string');
  }
  function value(depth) {
    if (depth > 128) throw new Error('JSON nesting limit exceeded');
    whitespace();
    if (text[offset] === '{') {
      offset += 1;
      whitespace();
      const keys = new Set();
      if (text[offset] === '}') { offset += 1; return; }
      while (offset < text.length) {
        whitespace();
        const key = stringToken();
        if (keys.has(key)) throw new Error('duplicate JSON key');
        keys.add(key);
        whitespace();
        if (text[offset++] !== ':') throw new Error('expected JSON colon');
        value(depth + 1);
        whitespace();
        const separator = text[offset++];
        if (separator === '}') return;
        if (separator !== ',') throw new Error('expected JSON object separator');
      }
      throw new Error('unterminated JSON object');
    }
    if (text[offset] === '[') {
      offset += 1;
      whitespace();
      if (text[offset] === ']') { offset += 1; return; }
      while (offset < text.length) {
        value(depth + 1);
        whitespace();
        const separator = text[offset++];
        if (separator === ']') return;
        if (separator !== ',') throw new Error('expected JSON array separator');
      }
      throw new Error('unterminated JSON array');
    }
    if (text[offset] === '"') { stringToken(); return; }
    const start = offset;
    while (offset < text.length && !/[\s,:\]}]/.test(text[offset])) offset += 1;
    if (start === offset) throw new Error('expected JSON value');
  }
  value(0);
  whitespace();
  if (offset !== text.length) throw new Error('trailing JSON data');
}

function validateIdentity(identity) {
  if (!identity || typeof identity !== 'object' || Array.isArray(identity)
    || identity.module !== '_blr_browser_probe' || identity.origin !== 'built-in'
    || identity.has_file !== false || identity.function_module !== '_blr_browser_probe'
    || JSON.stringify(identity.blender_version) !== '[5,2,0]'
    || identity.blender_build_hash !== EXPECTED_BUILD_HASH || identity.gpu_backend !== 'WEBGPU') {
    throw guiError('Blender 5.2, built-in Rust, or WebGPU identity was not proven');
  }
  return identity;
}

function validateScene(scene) {
  if (!scene || typeof scene !== 'object' || Array.isArray(scene)
    || scene.schema !== 'blr-scene-v1' || !Array.isArray(scene.objects)
    || Object.keys(scene).some(key => key !== 'schema' && key !== 'objects')
    || scene.objects.length > MAX_OBJECTS) throw guiError('Blender returned an unsupported scene schema');
  const names = new Set();
  let vertices = 0;
  let triangles = 0;
  for (const object of scene.objects) {
    if (!object || typeof object !== 'object' || Array.isArray(object)
      || typeof object.name !== 'string' || object.name.length === 0 || utf8Bytes(object.name) > 256
      || names.has(object.name) || !Array.isArray(object.positions) || !Array.isArray(object.triangles)
      || !Array.isArray(object.color) || object.color.length !== 4
      || Object.keys(object).some(key => !['name', 'positions', 'triangles', 'color'].includes(key))) {
      throw guiError('Blender returned malformed or duplicate object geometry');
    }
    names.add(object.name);
    vertices += object.positions.length;
    triangles += object.triangles.length;
    if (vertices > MAX_VERTICES || triangles > MAX_TRIANGLES) throw guiError('Blender geometry exceeds scene limits');
    if (!object.color.every(value => Number.isFinite(value) && value >= 0 && value <= 1)) {
      throw guiError('Blender returned invalid linear RGBA color');
    }
    for (const position of object.positions) {
      if (!Array.isArray(position) || position.length !== 3
        || !position.every(value => Number.isFinite(value) && Math.abs(value) <= 1_000_000)) {
        throw guiError('Blender returned invalid world-space positions');
      }
    }
    for (const triangle of object.triangles) {
      if (!Array.isArray(triangle) || triangle.length !== 3
        || !triangle.every(index => Number.isSafeInteger(index) && index >= 0 && index < object.positions.length)) {
        throw guiError('Blender returned invalid triangle indices');
      }
    }
  }
  if (utf8Bytes(JSON.stringify(scene)) > MAX_SCENE_BYTES) throw guiError('Blender scene exceeds 32 MiB');
  return scene;
}

function validateProgress(record) {
  const phases = ['fetch', 'verify', 'decode', 'unpack', 'initialize', 'compile', 'execute'];
  const states = ['start', 'update', 'complete', 'error', 'cancelled'];
  const sources = ['network-or-cache', 'memory-cache', 'work'];
  if (!record || typeof record !== 'object' || Array.isArray(record)
    || Object.keys(record).sort().join(',') !== 'asset,error,loadedBytes,phase,source,state,totalBytes'
    || !phases.includes(record.phase) || !states.includes(record.state)
    || (record.asset !== null && (typeof record.asset !== 'string' || record.asset.length > 128))
    || (record.source !== null && !sources.includes(record.source))
    || (record.loadedBytes !== null && (!Number.isSafeInteger(record.loadedBytes) || record.loadedBytes < 0))
    || (record.totalBytes !== null && (!Number.isSafeInteger(record.totalBytes) || record.totalBytes < 0))
    || (record.loadedBytes !== null && record.totalBytes !== null && record.loadedBytes > record.totalBytes)
    || (record.error !== null && (typeof record.error !== 'string' || utf8Bytes(record.error) > 512))) {
    throw guiError('runtime progress record is malformed');
  }
  return record;
}

function validatePythonResult(record, currentLease) {
  const keys = ['schema', 'ok', 'scene', 'lease_token', 'missing', 'stdout', 'stderr', 'traceback'];
  if (!record || typeof record !== 'object' || Array.isArray(record)
    || Object.keys(record).sort().join(',') !== [...keys].sort().join(',')
    || record.schema !== 'blr-python-scene-v1' || typeof record.ok !== 'boolean'
    || (record.lease_token !== null && (typeof record.lease_token !== 'string'
      || !/^[\x21-\x7e]{1,128}$/.test(record.lease_token)))
    || !Array.isArray(record.missing) || record.missing.length > MAX_OBJECTS
    || record.missing.some(name => typeof name !== 'string' || !name || utf8Bytes(name) > 256)) {
    throw guiError('built-in Python scene result has an unsupported shape');
  }
  for (const field of ['stdout', 'stderr', 'traceback']) {
    const value = record[field];
    if (field === 'traceback' && value === null) continue;
    if (typeof value !== 'string' || utf8Bytes(value) > MAX_PYTHON_DIAGNOSTIC_BYTES) {
      throw guiError(`built-in Python ${field} exceeds its byte limit`);
    }
  }
  if (record.ok) {
    if (record.lease_token === null || record.missing.length !== 0) throw guiError('successful Python result has no lease or reports missing objects');
    validateScene(record.scene);
  } else if (record.scene !== null || record.lease_token !== currentLease) {
    throw guiError('failed Python result does not preserve the current lease');
  }
  return {
    python: true,
    ok: record.ok,
    scene: record.scene === null ? null : validateScene(record.scene),
    leaseToken: record.lease_token,
    missing: record.missing,
    stdout: record.stdout,
    stderr: record.stderr,
    traceback: record.traceback,
  };
}

function validateBridgeResult(operation, record, currentLease) {
  if (!record || typeof record !== 'object' || Array.isArray(record)) throw guiError('built-in result is malformed');
  const identity = validateIdentity(record.identity);
  if (operation === 'run_python') return { ...validatePythonResult(record.result, currentLease), identity };
  if (operation === 'create' || operation === 'snapshot') {
    const result = record.result;
    if (!result || typeof result !== 'object' || Array.isArray(result)
      || result.schema !== 'blr-gui-scene-v1'
      || Object.keys(result).some(key => !['schema', 'lease_token', 'scene', 'missing'].includes(key))
      || Object.keys(result).length !== 4
      || typeof result.lease_token !== 'string' || !result.lease_token || result.lease_token.length > 128
      || !/^[\x21-\x7e]{1,128}$/.test(result.lease_token)
      || !Array.isArray(result.missing) || result.missing.length > MAX_OBJECTS
      || result.missing.some(name => typeof name !== 'string' || name.length === 0)) {
      throw guiError('built-in Rust scene result has an unsupported shape');
    }
    if (operation === 'snapshot' && result.lease_token !== currentLease) throw guiError('snapshot returned a stale lease');
    if (operation === 'create' && result.missing.length !== 0) throw guiError('create reported missing objects');
    const scene = validateScene(result.scene);
    return { scene, leaseToken: result.lease_token, missing: result.missing, identity };
  }
  const result = record.result;
  if (!result || typeof result !== 'object' || Array.isArray(result)
    || Object.keys(result).length !== 4 || result.schema !== 'blr-gui-reset-v1'
    || Object.keys(result).some(key => !['schema', 'lease_token', 'removed', 'missing'].includes(key))
    || result.lease_token !== currentLease
    || !Number.isSafeInteger(result.removed) || result.removed < 0 || result.removed > MAX_OBJECTS
    || !Number.isSafeInteger(result.missing) || result.missing < 0 || result.missing > MAX_OBJECTS) {
    throw guiError('built-in Rust reset result has an unsupported shape');
  }
  return { reset: true, leaseToken: result.lease_token, removed: result.removed, missing: result.missing, identity };
}

function randomRunId() {
  return Array.from(crypto.getRandomValues(new Uint8Array(24)), byte => byte.toString(16).padStart(2, '0')).join('');
}

export function createBlenderGui({ host, onStatus, onOutput, onProgress, onState } = {}) {
  if (!host || typeof host.append !== 'function') throw guiError('a GUI host element is required');
  if (typeof window === 'undefined' || typeof document === 'undefined') throw guiError('a browser document is required');
  const runId = randomRunId();
  const frame = document.createElement('iframe');
  frame.title = 'Interactive Blender 5.2 GUI';
  frame.setAttribute('allow', 'cross-origin-isolated');
  frame.setAttribute('allowfullscreen', '');
  const frameUrl = new URL("./gui-frame.html?blr_app=a04fd988c48d7984cfbb2a4470a1017f784b506d995b7dff1ad87a782f3fe926", import.meta.url);
  frameUrl.searchParams.set('run', runId);
  frame.src = frameUrl.href;
  frame.style.width = '100%';
  frame.style.height = '100%';
  frame.style.border = '0';
  frame.style.display = 'block';

  let ready = false;
  let initialized = false;
  let fatal = false;
  let visibleFrame = false;
  let lease = null;
  let lastSnapshot = null;
  let lastSnapshotStale = false;
  let failureMessage = null;
  let disposed = false;
  let startedPromise = null;
  let startupTimer = 0;
  let pollTimer = 0;
  let currentRequestId = 0;
  let nextEventId = 1;
  let leaseGeneration = 0;
  let documentInvalidated = false;
  let frameInitialized = false;
  let active = null;
  const pendingProgress = new Map();
  let startIdentity = null;
  let startResolve = null;
  let startReject = null;
  const queue = [];
  const observers = new Set();

  function updateState(state, requestId = null) {
    const detail = {
      state, ready, fatal, visibleFrame, leaseToken: lease, busy: Boolean(active), requestId,
      message: failureMessage, lastSnapshot, lastSnapshotStale,
    };
    try { if (typeof onState === 'function') onState(detail); } catch { /* observers cannot interrupt runtime ownership */ }
    for (const observer of observers) {
      try { observer(detail); } catch { /* observers cannot interrupt runtime ownership */ }
    }
  }

  function status(text) {
    try { if (typeof onStatus === 'function') onStatus(String(text).slice(0, 160)); } catch { /* status is advisory */ }
  }

  function output(record) {
    try { if (typeof onOutput === 'function') onOutput(record); } catch { /* output is advisory */ }
  }

  function progress(record) {
    try { if (typeof onProgress === 'function') onProgress(record); } catch { /* progress is advisory */ }
    try { if (typeof active?.onProgress === 'function') active.onProgress(record); } catch { /* progress is advisory */ }
  }

  function frameWindow() {
    try { return frame.contentWindow; } catch { return null; }
  }

  function moduleFS() {
    const module = frameWindow()?.Module;
    const fs = module?.FS;
    if (!fs || typeof fs.writeFile !== 'function' || typeof fs.rename !== 'function'
      || typeof fs.readFile !== 'function' || typeof fs.analyzePath !== 'function'
      || typeof fs.stat !== 'function' || typeof fs.unlink !== 'function') {
      throw guiError('GUI runtime did not export the required MEMFS operations');
    }
    return fs;
  }

  function settle(item, error, value) {
    if (item.settled) return;
    item.settled = true;
    clearTimeout(item.deadlineTimer);
    item.signal?.removeEventListener('abort', item.onAbort);
    if (error) item.reject(error);
    else item.resolve(value);
  }

  function rejectAll(error) {
    for (const item of queue.splice(0)) settle(item, error);
    if (active) {
      const item = active;
      active = null;
      clearTimeout(item.drainTimer);
      settle(item, error);
    }
  }

  function beginDrainDeadline(item) {
    if (item.drainTimer) return;
    item.drainTimer = setTimeout(() => {
      if (active === item) fail(guiError('dispatched request exceeded its 180-second drain deadline'));
    }, REQUEST_TIMEOUT_MS);
  }

  function fail(error) {
    if (disposed) return;
    const requestId = active?.id ?? null;
    fatal = true;
    if (lease !== null) leaseGeneration += 1;
    lease = null;
    lastSnapshotStale = lastSnapshot !== null;
    failureMessage = error instanceof Error ? error.message : String(error);
    clearTimeout(pollTimer);
    ready = false;
    status(failureMessage);
    startReject?.(error instanceof Error ? error : guiError('runtime failed'));
    startResolve = null;
    startReject = null;
    rejectAll(error instanceof Error ? error : guiError('runtime failed'));
    updateState('fatal', requestId);
  }

  function removeQueued(item, error) {
    const index = queue.indexOf(item);
    if (index !== -1) {
      queue.splice(index, 1);
      settle(item, error);
      updateState('ready');
    }
  }

  function publish(item) {
    if (disposed || active || queue.length === 0 || !ready) return;
    const next = queue.shift();
    if (next.settled) return publish(item);
    if (next.deadline <= Date.now()) {
      settle(next, guiError('request expired before dispatch'));
      return publish();
    }
    if ((next.operation === 'snapshot' || next.operation === 'reset')
      && (lease === null || next.leaseToken !== lease)) {
      settle(next, guiError('scene lease changed before dispatch'));
      return publish();
    }
    if (!Number.isSafeInteger(currentRequestId + 1)) {
      settle(next, guiError('request identifier limit reached'));
      fail(guiError('request identifier limit reached'));
      return;
    }
    next.id = ++currentRequestId;
    active = next;
    next.generation = leaseGeneration;
    updateState('busy', next.id);
    let request;
    if (next.operation === 'create' || next.operation === 'run_python') request = { id: next.id, operation: next.operation };
    else request = { id: next.id, operation: next.operation, lease_token: next.leaseToken };
    const text = JSON.stringify(request);
    if (utf8Bytes(text) > 4096) {
      active = null;
      settle(next, guiError('request exceeds its byte limit'));
      publish();
      return;
    }
    try {
      const fs = moduleFS();
      if (next.operation === 'create' || next.operation === 'run_python') {
        const commandPath = `/blr_gui/commands/${next.id}.json`;
        const commandTemporary = `${commandPath}.tmp`;
        fs.writeFile(commandTemporary, new TextEncoder().encode(next.operation === 'create' ? next.commandsJson : next.source));
        fs.rename(commandTemporary, commandPath);
      }
      const path = `/blr_gui/requests/${next.id}.json`;
      const temporary = `${path}.tmp`;
      fs.writeFile(temporary, new TextEncoder().encode(text));
      fs.rename(temporary, path);
      next.dispatched = true;
    } catch (error) {
      try {
        const fs = moduleFS();
        for (const path of [`/blr_gui/commands/${next.id}.json`, `/blr_gui/commands/${next.id}.json.tmp`, `/blr_gui/requests/${next.id}.json.tmp`]) {
          if (fs.analyzePath(path).exists) fs.unlink(path);
        }
      } catch { /* publication failure remains primary */ }
      active = null;
      settle(next, error instanceof Error ? error : guiError('could not publish request'));
      publish();
    }
  }

  function readFile(path, maximum, label) {
    const fs = moduleFS();
    if (!fs.analyzePath(path).exists) return null;
    const size = fs.stat(path).size;
    if (!Number.isSafeInteger(size) || size < 0 || size > maximum) throw guiError(`${label} exceeds its byte limit`);
    const bytes = fs.readFile(path);
    if (!isUint8Array(bytes)) throw guiError(`${label} is not a byte array`);
    if (bytes.byteLength > maximum) throw guiError(`${label} exceeds its byte limit`);
    fs.unlink(path);
    return bytes;
  }

  function processResponse() {
    if (!active) return;
    const max = active.operation === 'reset' ? MAX_SMALL_BYTES : MAX_RESULT_BYTES + MAX_SMALL_BYTES;
    const bytes = readFile(`/blr_gui/results/${active.id}.json`, max, 'GUI response');
    if (!bytes) return;
    const item = active;
    active = null;
    clearTimeout(item.drainTimer);
    try {
      const response = decodeJson(bytes, 'GUI response');
      if (!response || typeof response !== 'object' || Object.keys(response).some(key => !['id', 'ok', 'fatal', 'result', 'error', 'identity'].includes(key))
        || response.id !== item.id || typeof response.ok !== 'boolean' || typeof response.fatal !== 'boolean') throw guiError('response correlation or shape is invalid');
      validateIdentity(response.identity);
      if (item.generation !== leaseGeneration) {
        settle(item, guiError('stale GUI response discarded after document change'));
        publish();
        return;
      }
      if (!response.ok) {
        if (Object.hasOwn(response, 'result') || typeof response.error !== 'string' || response.error.length > 512) {
          throw guiError('runtime returned a malformed error');
        }
        if (response.fatal) {
          const error = guiError(response.error);
          settle(item, error);
          fail(error);
          return;
        }
        settle(item, guiError(response.error));
        updateState('ready', item.id);
      } else {
        if (Object.hasOwn(response, 'error') || response.fatal) throw guiError('successful response contains an error or fatal marker');
        const value = validateBridgeResult(item.operation, response, lease);
        if (item.operation === 'create') lease = value.leaseToken;
        if (item.operation === 'create' || item.operation === 'snapshot') {
          lastSnapshot = { ...value, capturedAt: Date.now(), source: item.operation };
          lastSnapshotStale = false;
        }
        if (item.operation === 'reset') {
          lease = null;
          leaseGeneration += 1;
          lastSnapshotStale = lastSnapshot !== null;
        }
        if (item.operation === 'run_python' && value.ok) {
          lease = value.leaseToken;
          lastSnapshot = { ...value, capturedAt: Date.now(), source: item.operation };
          lastSnapshotStale = false;
        }
        settle(item, null, value);
        updateState('ready', item.id);
      }
    } catch (error) {
      settle(item, error instanceof Error ? error : guiError('invalid response'));
      fail(error instanceof Error ? error : guiError('invalid response'));
      return;
    }
    publish();
  }

  function processEvents() {
    const fs = moduleFS();
    for (let count = 0; count < 16; count += 1) {
      const path = `/blr_gui/events/${nextEventId}.json`;
      const bytes = readFile(path, MAX_SMALL_BYTES, 'GUI event');
      if (!bytes) return;
      const event = decodeJson(bytes, 'GUI event');
      if (!event || event.id !== nextEventId || typeof event.type !== 'string'
        || Object.keys(event).some(key => !['id', 'type', 'message', 'identity', 'lease_active'].includes(key))) throw guiError('GUI lifecycle event is malformed');
      nextEventId += 1;
      if (event.type === 'document-invalidated') {
        if (Object.keys(event).some(key => !['id', 'type'].includes(key))) throw guiError('document invalidation event is malformed');
        documentInvalidated = true;
        ready = false;
        lease = null;
        leaseGeneration += 1;
        lastSnapshotStale = lastSnapshot !== null;
        const staleError = guiError('document changed; request belongs to the previous document');
        for (const item of queue.splice(0)) settle(item, staleError);
        if (active) {
          active.abandoned = true;
          settle(active, staleError);
          beginDrainDeadline(active);
        }
        updateState('invalidated');
      } else if (event.type === 'document-ready') {
        if (!documentInvalidated || event.lease_active !== false || !Object.hasOwn(event, 'identity')
          || Object.keys(event).length !== 4) {
          throw guiError('document recovery event is not proven');
        }
        startIdentity = validateIdentity(event.identity);
        documentInvalidated = false;
        ready = true;
        status('Blender document ready');
        updateState('ready');
        publish();
      } else if (event.type === 'fatal') {
        throw guiError(typeof event.message === 'string' ? event.message : 'runtime reported corruption');
      } else throw guiError('runtime sent an unknown lifecycle event');
    }
    if (fs.analyzePath(`/blr_gui/events/${nextEventId}.json`).exists) setTimeout(poll, 0);
  }

  function poll() {
    if (disposed || fatal || !frameInitialized) return;
    try {
      const fs = moduleFS();
      if (!initialized) {
        const bytes = readFile('/blr_gui/ready.json', MAX_SMALL_BYTES, 'GUI startup record');
        if (bytes) {
          const record = decodeJson(bytes, 'GUI startup record');
          if (!record || record.schema !== 'blr-gui-transport-ready-v1' || Object.keys(record).some(key => !['schema', 'identity', 'error'].includes(key))) {
            throw guiError('startup record has an unsupported shape');
          }
          if (record.error) throw guiError(String(record.error).slice(0, 512));
          startIdentity = validateIdentity(record.identity);
          initialized = true;
          ready = true;
          status('Blender GUI transport ready');
          updateState('ready');
          startResolve?.({ ready: true, visibleFrame, identity: startIdentity });
          startResolve = null;
          startReject = null;
          publish();
        }
      }
      if (initialized) {
        processEvents();
        processResponse();
      }
      if (!fatal) pollTimer = setTimeout(poll, POLL_MS);
    } catch (error) {
      fail(error instanceof Error ? error : guiError('GUI transport polling failed'));
    }
  }

  function onMessage(event) {
    if (event.source !== frameWindow() || event.origin !== location.origin) return;
    const message = event.data;
    if (!message || message.channel !== CHANNEL || message.runId !== runId || typeof message.type !== 'string') return;
    if (message.type === 'initialized') {
      if (frameInitialized) return fail(guiError('GUI frame initialized twice'));
      frameInitialized = true;
      status('Blender GUI initialized; waiting for owner-thread transport');
      poll();
    } else if (message.type === 'frame-available') {
      if (!Number.isFinite(message.width) || !Number.isFinite(message.height) || message.width <= 0 || message.height <= 0) return fail(guiError('GUI frame dimensions are invalid'));
      visibleFrame = true;
      updateState('frame-available');
    } else if (message.type === 'output') {
      if ((message.stream !== 'stdout' && message.stream !== 'stderr') || typeof message.text !== 'string') return fail(guiError('GUI runtime output is malformed'));
      output({ stream: message.stream, text: message.text.slice(0, 4096) });
    } else if (message.type === 'progress') {
      if (Object.keys(message).some(key => !['channel', 'runId', 'type', 'progress'].includes(key))) return fail(guiError('GUI progress message has unsupported fields'));
      try {
        const record = validateProgress(message.progress);
        const key = `${record.phase}\0${record.asset || ''}`;
        if (record.state === 'start' || record.state === 'update') pendingProgress.set(key, record);
        else pendingProgress.delete(key);
        progress(record);
      } catch (error) { return fail(error instanceof Error ? error : guiError('GUI progress message is malformed')); }
    } else if (message.type === 'error') fail(guiError(typeof message.message === 'string' ? message.message.slice(0, 512) : 'GUI frame failed'));
  }

  function onAbort(item) {
    const error = abortError();
    if (!item.dispatched) removeQueued(item, error);
    else {
      item.abandoned = true;
      settle(item, error);
      beginDrainDeadline(item);
      updateState('busy', item.id);
    }
  }

  function enqueue(operation, details = {}, { signal, onProgress } = {}) {
    if (!ready || disposed) return Promise.reject(guiError(disposed ? 'runtime was disposed' : 'runtime is not ready'));
    if (signal?.aborted) return Promise.reject(abortError());
    if (queue.length + (active ? 1 : 0) >= MAX_QUEUE) return Promise.reject(guiError('request queue is full'));
    if ((operation === 'snapshot' || operation === 'reset') && lease === null) {
      return Promise.reject(guiError('no active scene lease'));
    }
    let commandsJson;
    let source;
    if (operation === 'create') {
      try {
        commandsJson = validateSceneCommands(details.commands);
        if (utf8Bytes(commandsJson) > MAX_COMMAND_BYTES) throw guiError('scene commands exceed 256 KiB');
      } catch (error) {
        return Promise.reject(error instanceof Error ? error : guiError('scene commands are invalid'));
      }
    }
    if (operation === 'run_python') {
      if (typeof details.source !== 'string') return Promise.reject(guiError('Python source must be text'));
      source = details.source;
      if (utf8Bytes(source) > MAX_PYTHON_SOURCE_BYTES) return Promise.reject(guiError('Python source exceeds 256 KiB'));
    }
    return new Promise((resolve, reject) => {
      const item = {
        operation,
        commandsJson,
        source,
        onProgress,
        leaseToken: lease,
        resolve,
        reject,
        signal,
        id: null,
        dispatched: false,
        settled: false,
        deadline: Date.now() + REQUEST_TIMEOUT_MS,
        generation: leaseGeneration,
      };
      item.onAbort = () => onAbort(item);
      item.deadlineTimer = setTimeout(() => {
        const error = guiError('request exceeded 180 seconds');
        if (!item.dispatched) removeQueued(item, error);
        else {
          item.abandoned = true;
          settle(item, error);
          beginDrainDeadline(item);
          updateState('busy', item.id);
        }
      }, REQUEST_TIMEOUT_MS);
      signal?.addEventListener('abort', item.onAbort, { once: true });
      queue.push(item);
      updateState('busy');
      publish();
    });
  }

  function start() {
    if (startedPromise) return startedPromise;
    if (disposed) return Promise.reject(guiError('runtime was disposed'));
    if (!window.isSecureContext || !window.crossOriginIsolated || typeof SharedArrayBuffer === 'undefined') {
      return Promise.reject(guiError('secure cross-origin isolation is required'));
    }
    startedPromise = new Promise((resolve, reject) => {
      startResolve = resolve;
      startReject = reject;
      const timer = setTimeout(() => {
        if (ready) return;
        const error = guiError('GUI runtime startup exceeded 180 seconds');
        fail(error);
      }, START_TIMEOUT_MS);
      const clearStartupTimer = detail => {
        if (detail.state === 'ready' || detail.state === 'fatal') {
          clearTimeout(timer);
          observers.delete(clearStartupTimer);
        }
      };
      observers.add(clearStartupTimer);
      window.addEventListener('message', onMessage);
      frame.addEventListener('error', () => fail(guiError('GUI iframe failed to load')), { once: true });
      frame.title = 'Loading interactive Blender 5.2 GUI';
      startupTimer = timer;
      pollTimer = setTimeout(() => {
        if (!disposed && !frameInitialized) status('Loading interactive Blender 5.2 GUI');
      }, 250);
      host.append(frame);
      status('Starting interactive Blender 5.2 GUI');
      updateState('starting');
    });
    return startedPromise;
  }

  function dispose() {
    if (disposed) return;
    disposed = true;
    for (const record of pendingProgress.values()) {
      progress({ ...record, state: 'cancelled', error: null });
    }
    pendingProgress.clear();
    clearTimeout(startupTimer);
    clearTimeout(pollTimer);
    globalThis.window?.removeEventListener('message', onMessage);
    rejectAll(abortError());
    frame.remove();
    ready = false;
    visibleFrame = false;
    updateState('disposed');
  }

  return Object.freeze({
    start,
    createScene: (commands, options) => enqueue('create', { commands }, options),
    runPython: (source, options) => enqueue('run_python', { source }, options),
    snapshot: options => enqueue('snapshot', {}, options),
    reset: options => enqueue('reset', {}, options),
    dispose,
    close: dispose,
    get ready() { return ready; },
    get fatal() { return fatal; },
    get visibleFrame() { return visibleFrame; },
    get lease() { return lease; },
    get lastSnapshot() { return lastSnapshot; },
    get lastSnapshotStale() { return lastSnapshotStale; },
    get busy() { return Boolean(active || queue.length); },
  });
}
