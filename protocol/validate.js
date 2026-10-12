const LIMITS = Object.freeze({
  commandBytes: 256 * 1024,
  objects: 64,
  vertices: 100_000,
  triangles: 200_000,
  transformAbs: 1000,
  worldAbs: 1_000_000,
  minimumEffectiveFeature: 0.0001,
  nameUtf8Bytes: 128,
});
const COMMON_FIELDS = new Set(['kind', 'name', 'location', 'rotation', 'scale', 'color']);
const KIND_FIELDS = Object.freeze({
  cube: new Set(['size']),
  plane: new Set(['size']),
  uv_sphere: new Set(['radius', 'segments', 'ring_count']),
  cylinder: new Set(['radius', 'depth', 'segments']),
  cone: new Set(['radius1', 'radius2', 'depth', 'segments']),
  torus: new Set(['major_radius', 'minor_radius', 'major_segments', 'minor_segments']),
});
const UTF8 = new TextEncoder();

function utf8ByteLength(value, limit = Infinity) {
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code <= 0x7f) bytes += 1;
    else if (code <= 0x7ff) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff
      && value.charCodeAt(index + 1) >= 0xdc00 && value.charCodeAt(index + 1) <= 0xdfff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
    if (bytes > limit) return bytes;
  }
  return bytes;
}

function invalidJson() {
  throw new Error('scene commands are not valid JSON');
}

function hasUnpairedSurrogate(value) {
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return true;
      index += 1;
    } else if (code >= 0xdc00 && code <= 0xdfff) {
      return true;
    }
  }
  return false;
}

// JSON.parse discards duplicate keys. Scan the original text first, decode each key so escaped
// spellings collide, and retain numeric lexemes where serde expects integer fields.
function scanJson(rawText) {
  let offset = 0;
  const integerTokens = new Map();
  const whitespace = () => {
    while (offset < rawText.length && /[\u0009\u000a\u000d\u0020]/.test(rawText[offset])) offset += 1;
  };
  const stringToken = () => {
    const start = offset;
    if (rawText[offset] !== '"') invalidJson();
    offset += 1;
    while (offset < rawText.length) {
      const code = rawText.charCodeAt(offset);
      if (code === 0x22) {
        offset += 1;
        let value;
        try { value = JSON.parse(rawText.slice(start, offset)); } catch { invalidJson(); }
        if (hasUnpairedSurrogate(value)) invalidJson();
        return value;
      }
      if (code < 0x20) invalidJson();
      if (code === 0x5c) {
        offset += 1;
        if (offset >= rawText.length) invalidJson();
        const escaped = rawText[offset];
        if (escaped === 'u') {
          if (!/^[0-9a-fA-F]{4}$/.test(rawText.slice(offset + 1, offset + 5))) invalidJson();
          offset += 5;
          continue;
        }
        if (!'"\\/bfnrt'.includes(escaped)) invalidJson();
      }
      offset += 1;
    }
    invalidJson();
  };
  const parseValue = (path, depth) => {
    whitespace();
    if (depth > 128 || offset >= rawText.length) invalidJson();
    const first = rawText[offset];
    if (first === '"') {
      stringToken();
      return;
    }
    if (first === '{') {
      offset += 1;
      whitespace();
      if (rawText[offset] === '}') { offset += 1; return; }
      const keys = new Set();
      while (offset < rawText.length) {
        whitespace();
        const key = stringToken();
        if (keys.has(key)) throw new Error('scene commands contain a duplicate JSON object field');
        keys.add(key);
        whitespace();
        if (rawText[offset] !== ':') invalidJson();
        offset += 1;
        parseValue([...path, key], depth + 1);
        whitespace();
        if (rawText[offset] === '}') { offset += 1; return; }
        if (rawText[offset] !== ',') invalidJson();
        offset += 1;
      }
      invalidJson();
    }
    if (first === '[') {
      offset += 1;
      whitespace();
      if (rawText[offset] === ']') { offset += 1; return; }
      let index = 0;
      while (offset < rawText.length) {
        parseValue([...path, index], depth + 1);
        index += 1;
        whitespace();
        if (rawText[offset] === ']') { offset += 1; return; }
        if (rawText[offset] !== ',') invalidJson();
        offset += 1;
      }
      invalidJson();
    }
    if (first === '-' || (first >= '0' && first <= '9')) {
      const match = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(rawText.slice(offset));
      if (!match) invalidJson();
      integerTokens.set(JSON.stringify(path), match[0]);
      offset += match[0].length;
      return;
    }
    for (const literal of ['true', 'false', 'null']) {
      if (rawText.startsWith(literal, offset)) {
        offset += literal.length;
        return;
      }
    }
    invalidJson();
  };
  parseValue([], 0);
  whitespace();
  if (offset !== rawText.length) invalidJson();
  return integerTokens;
}

function own(object, field) {
  return Object.hasOwn(object, field);
}

function finite(value) {
  return typeof value === 'number' && Number.isFinite(value);
}

function validateSceneCommands(rawText) {
  if (typeof rawText !== 'string') throw new Error('scene command text is missing');
  if (utf8ByteLength(rawText, LIMITS.commandBytes) > LIMITS.commandBytes) throw new Error('scene commands exceed the 256 KiB limit');
  const integerTokens = scanJson(rawText);
  let commands;
  try { commands = JSON.parse(rawText); } catch { invalidJson(); }
  if (!commands || typeof commands !== 'object' || Array.isArray(commands)
    || commands.schema !== 'blr-scene-commands-v1' || !Array.isArray(commands.objects)
    || commands.objects.length > LIMITS.objects) {
    throw new Error('unsupported scene command schema or object count');
  }
  if (Object.keys(commands).some(key => !['schema', 'objects'].includes(key))) {
    throw new Error('unsupported scene command field');
  }

  const names = new Set();
  let vertices = 0;
  let triangles = 0;
  for (const [index, object] of commands.objects.entries()) {
    if (!object || typeof object !== 'object' || Array.isArray(object)
      || typeof object.kind !== 'string' || !Object.hasOwn(KIND_FIELDS, object.kind)
      || Object.keys(object).some(key => !COMMON_FIELDS.has(key) && !KIND_FIELDS[object.kind].has(key))
      || typeof object.name !== 'string' || object.name.length === 0 || object.name.includes('\0')
      || UTF8.encode(object.name).byteLength > LIMITS.nameUtf8Bytes || names.has(object.name)) {
      throw new Error('invalid primitive command');
    }
    names.add(object.name);
    const valueOr = (field, fallback) => own(object, field) && object[field] !== null ? object[field] : fallback;
    const vector = (field, fallback, zeroAllowed) => {
      const value = own(object, field) ? object[field] : fallback;
      if (!Array.isArray(value) || value.length !== 3
        || !value.every(component => finite(component) && Math.abs(component) <= LIMITS.transformAbs)
        || (!zeroAllowed && value.some(component => component === 0))) {
        throw new Error(`${field} coordinates must be finite and within the 1000 bound`);
      }
      return value;
    };
    const location = vector('location', [0, 0, 0], true);
    vector('rotation', [0, 0, 0], true);
    const scale = vector('scale', [1, 1, 1], false);
    const color = own(object, 'color') ? object.color : [1, 1, 1, 1];
    if (!Array.isArray(color) || color.length !== 4
      || !color.every(value => finite(Math.fround(value)) && Math.fround(value) >= 0 && Math.fround(value) <= 1)) {
      throw new Error('invalid RGBA color');
    }
    const number = (field, fallback, zeroAllowed = false) => {
      const value = valueOr(field, fallback);
      if (!finite(value) || (zeroAllowed ? value < 0 : value <= 0) || value > 1000) {
        throw new Error(`invalid ${field} dimension`);
      }
      return value;
    };
    const count = (field, fallback, minimum, maximum) => {
      const value = valueOr(field, fallback);
      const token = integerTokens.get(JSON.stringify(['objects', index, field]));
      const needsLexicalCheck = own(object, field) && object[field] !== null;
      if (!Number.isSafeInteger(value) || (needsLexicalCheck && (!Number.isSafeInteger(Number(token))
        || !/^(?:0|[1-9][0-9]*)$/.test(token || '')))
        || value < minimum || value > maximum) throw new Error(`invalid ${field} count`);
      return value;
    };

    let estimatedVertices;
    let estimatedTriangles;
    let radius;
    let minimumFeature;
    switch (object.kind) {
      case 'cube': {
        const size = number('size', 1);
        [estimatedVertices, estimatedTriangles] = [8, 12];
        radius = size * Math.sqrt(3) / 2;
        minimumFeature = size;
        break;
      }
      case 'plane': {
        const size = number('size', 2);
        [estimatedVertices, estimatedTriangles] = [4, 2];
        radius = size / Math.sqrt(2);
        minimumFeature = size;
        break;
      }
      case 'uv_sphere': {
        const segments = count('segments', 16, 3, 64);
        const rings = count('ring_count', 8, 3, 64);
        const size = number('radius', 1);
        [estimatedVertices, estimatedTriangles] = [segments * rings + 2, 2 * segments * rings];
        radius = size;
        minimumFeature = size;
        break;
      }
      case 'cylinder': {
        const segments = count('segments', 32, 3, 128);
        const r = number('radius', 1);
        const depth = number('depth', 2);
        [estimatedVertices, estimatedTriangles] = [2 * segments, 4 * segments - 4];
        radius = Math.hypot(r, depth / 2);
        minimumFeature = Math.min(r, depth);
        break;
      }
      case 'cone': {
        const segments = count('segments', 32, 3, 128);
        const r1 = number('radius1', 1);
        const r2 = number('radius2', 0, true);
        const depth = number('depth', 2);
        [estimatedVertices, estimatedTriangles] = r2 === 0
          ? [segments + 1, 2 * segments - 2]
          : [2 * segments, 4 * segments - 4];
        radius = Math.hypot(Math.max(r1, r2), depth / 2);
        minimumFeature = Math.min(r1, depth);
        break;
      }
      case 'torus': {
        const majorSegments = count('major_segments', 32, 3, 64);
        const minorSegments = count('minor_segments', 12, 3, 64);
        const majorRadius = number('major_radius', 1);
        const minorRadius = number('minor_radius', 0.25);
        if (minorRadius >= majorRadius) throw new Error('torus minor radius must be smaller than major radius');
        [estimatedVertices, estimatedTriangles] = [majorSegments * minorSegments, 2 * majorSegments * minorSegments];
        radius = majorRadius + minorRadius;
        minimumFeature = minorRadius;
        break;
      }
      default:
        throw new Error('invalid primitive command');
    }
    const minScale = Math.min(...scale.map(Math.abs));
    const maxScale = Math.max(...scale.map(Math.abs));
    if (minimumFeature * minScale < LIMITS.minimumEffectiveFeature) {
      throw new Error('effective minimum primitive feature must be at least 0.0001 world units');
    }
    if (Math.max(...location.map(Math.abs)) + radius * maxScale > LIMITS.worldAbs) {
      throw new Error('transformed primitive may exceed world coordinate bounds');
    }
    vertices += estimatedVertices;
    triangles += estimatedTriangles;
    if (vertices > LIMITS.vertices || triangles > LIMITS.triangles) {
      throw new Error('scene exceeds geometry limits');
    }
  }
  return rawText;
}

export { validateSceneCommands, LIMITS as sceneProtocolLimits };
