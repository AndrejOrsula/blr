const SCHEMA = "blr-scene-v1";
const MAX_OBJECTS = 64;
const MAX_VERTICES = 100_000;
const MAX_TRIANGLES = 200_000;
const MAX_COORDINATE = 1_000_000;
const VIEWER_PATH = "./vendor/model-viewer.min.js";
const INITIAL_ORBIT = "35deg 65deg 105%";

function invalid(message) {
  throw new TypeError(`Invalid ${SCHEMA} scene: ${message}`);
}

function tuple(value, size, label) {
  if (!Array.isArray(value) || value.length !== size) {
    invalid(`${label} must contain exactly ${size} finite numbers`);
  }
  for (let index = 0; index < size; index += 1) {
    if (typeof value[index] !== "number" || !Number.isFinite(value[index])) invalid(`${label} must contain exactly ${size} finite numbers`);
  }
  return value;
}

function validateScene(scene) {
  if (!scene || typeof scene !== "object" || Array.isArray(scene) || scene.schema !== SCHEMA || !Array.isArray(scene.objects)) {
    invalid(`expected {schema: "${SCHEMA}", objects: [...]}`);
  }
  if (scene.objects.length > MAX_OBJECTS) invalid(`objects must contain 0 to ${MAX_OBJECTS} entries`);

  let vertexTotal = 0;
  let triangleTotal = 0;
  const validated = scene.objects.map((object, objectIndex) => {
    if (!object || typeof object !== "object" || Array.isArray(object)) invalid(`objects[${objectIndex}] must be an object`);
    if (typeof object.name !== "string" || object.name.length === 0 || object.name.length > 128) invalid(`objects[${objectIndex}].name must be 1 to 128 characters`);
    if (!Array.isArray(object.positions) || object.positions.length < 3) invalid(`objects[${objectIndex}].positions must contain at least 3 vertices`);
    if (!Array.isArray(object.triangles) || object.triangles.length === 0) invalid(`objects[${objectIndex}].triangles must contain at least 1 triangle`);
    vertexTotal += object.positions.length;
    triangleTotal += object.triangles.length;
    if (vertexTotal > MAX_VERTICES) invalid(`total vertices exceed ${MAX_VERTICES}`);
    if (triangleTotal > MAX_TRIANGLES) invalid(`total triangles exceed ${MAX_TRIANGLES}`);

    const positions = Array.from(object.positions, (position, index) => {
      const p = tuple(position, 3, `objects[${objectIndex}].positions[${index}]`);
      if (p.some((component) => Math.abs(component) > MAX_COORDINATE)) invalid(`objects[${objectIndex}].positions[${index}] exceeds coordinate limit ${MAX_COORDINATE}`);
      // Blender is Z-up. Bake a proper -90-degree X rotation into glTF's Y-up coordinates.
      return [p[0], p[2], p[1] === 0 ? 0 : -p[1]];
    });
    const triangles = Array.from(object.triangles, (triangle, index) => {
      const t = tuple(triangle, 3, `objects[${objectIndex}].triangles[${index}]`);
      if (t.some((vertex) => !Number.isInteger(vertex) || vertex < 0 || vertex >= positions.length)) {
        invalid(`objects[${objectIndex}].triangles[${index}] has an out-of-range vertex index`);
      }
      if (new Set(t).size !== 3) invalid(`objects[${objectIndex}].triangles[${index}] repeats a vertex index`);
      return t;
    });
    const color = tuple(object.color, 4, `objects[${objectIndex}].color`);
    if (color.some((component) => component < 0 || component > 1)) invalid(`objects[${objectIndex}].color values must be in [0, 1]`);
    return { name: object.name, positions, triangles, color };
  });
  return { objects: validated, vertexTotal, triangleTotal };
}

function vectorBytes(values) {
  return new Uint8Array(new Float32Array(values).buffer);
}

function indexBytes(values) {
  return new Uint8Array(new Uint32Array(values).buffer);
}

function computeNormals(positions, triangles, name) {
  const normals = Array.from({ length: positions.length }, () => [0, 0, 0]);
  for (const [a, b, c] of triangles) {
    const p0 = positions[a];
    const p1 = positions[b];
    const p2 = positions[c];
    const ab = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    const ac = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    const normal = [ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]];
    const length = Math.hypot(...normal);
    if (!Number.isFinite(length) || length === 0) invalid(`object ${JSON.stringify(name)} contains a degenerate triangle`);
    for (const vertex of [a, b, c]) {
      normals[vertex][0] += normal[0];
      normals[vertex][1] += normal[1];
      normals[vertex][2] += normal[2];
    }
  }
  return normals.map((normal, index) => {
    const length = Math.hypot(...normal);
    if (!Number.isFinite(length) || length === 0) invalid(`object ${JSON.stringify(name)} has an unused or cancelling vertex normal at index ${index}`);
    return normal.map((component) => component / length);
  });
}

function buildGlb(validated) {
  const binaryParts = [];
  let binaryLength = 0;
  const bufferViews = [];
  const accessors = [];
  const materials = [];
  const meshes = [];
  const nodes = [];
  const append = (bytes, target) => {
    const padding = (4 - (binaryLength % 4)) % 4;
    if (padding) {
      binaryParts.push(new Uint8Array(padding));
      binaryLength += padding;
    }
    const bufferView = { buffer: 0, byteOffset: binaryLength, byteLength: bytes.byteLength };
    if (target) bufferView.target = target;
    const index = bufferViews.push(bufferView) - 1;
    binaryParts.push(bytes);
    binaryLength += bytes.byteLength;
    return index;
  };

  for (const [objectIndex, object] of validated.objects.entries()) {
    const flatPositions = object.positions.flat();
    const normals = computeNormals(object.positions, object.triangles, object.name).flat();
    const flatIndices = object.triangles.flat();
    const positionsView = append(vectorBytes(flatPositions), 34962);
    const normalsView = append(vectorBytes(normals), 34962);
    const indicesView = append(indexBytes(flatIndices), 34963);
    const min = [Infinity, Infinity, Infinity];
    const max = [-Infinity, -Infinity, -Infinity];
    for (const position of object.positions) {
      for (let axis = 0; axis < 3; axis += 1) {
        min[axis] = Math.min(min[axis], position[axis]);
        max[axis] = Math.max(max[axis], position[axis]);
      }
    }
    const positionAccessor = accessors.push({ bufferView: positionsView, componentType: 5126, count: object.positions.length, type: "VEC3", min, max }) - 1;
    const normalAccessor = accessors.push({ bufferView: normalsView, componentType: 5126, count: object.positions.length, type: "VEC3" }) - 1;
    let minimumIndex = Infinity;
    let maximumIndex = -Infinity;
    for (const index of flatIndices) {
      minimumIndex = Math.min(minimumIndex, index);
      maximumIndex = Math.max(maximumIndex, index);
    }
    const indexAccessor = accessors.push({ bufferView: indicesView, componentType: 5125, count: flatIndices.length, type: "SCALAR", min: [minimumIndex], max: [maximumIndex] }) - 1;
    const material = { name: `${object.name} material`, pbrMetallicRoughness: { baseColorFactor: object.color, metallicFactor: 0, roughnessFactor: 0.8 } };
    if (object.color[3] < 1) material.alphaMode = "BLEND";
    materials.push(material);
    meshes.push({ name: object.name, primitives: [{ attributes: { POSITION: positionAccessor, NORMAL: normalAccessor }, indices: indexAccessor, material: objectIndex, mode: 4 }] });
    nodes.push({ name: object.name, mesh: objectIndex });
  }

  const document = validated.objects.length === 0
    ? { asset: { version: "2.0", generator: "blr browser scene preview" } }
    : {
      asset: { version: "2.0", generator: "blr browser scene preview" },
      scene: 0,
      scenes: [{ nodes: nodes.map((_, index) => index) }],
      nodes,
      meshes,
      materials,
      bufferViews,
      accessors,
    };
  if (binaryLength > 0) document.buffers = [{ byteLength: binaryLength }];
  const encoder = new TextEncoder();
  const jsonRaw = encoder.encode(JSON.stringify(document));
  const jsonLength = Math.ceil(jsonRaw.byteLength / 4) * 4;
  const jsonBytes = new Uint8Array(jsonLength);
  jsonBytes.fill(0x20);
  jsonBytes.set(jsonRaw);
  const binLength = Math.ceil(binaryLength / 4) * 4;
  const binBytes = new Uint8Array(binLength);
  let binOffset = 0;
  for (const part of binaryParts) {
    binBytes.set(part, binOffset);
    binOffset += part.byteLength;
  }
  const totalLength = 12 + 8 + jsonLength + (binLength > 0 ? 8 + binLength : 0);
  if (totalLength > 16 * 1024 * 1024) invalid("generated GLB exceeds 16 MiB");
  const glb = new ArrayBuffer(totalLength);
  const view = new DataView(glb);
  view.setUint32(0, 0x46546c67, true);
  view.setUint32(4, 2, true);
  view.setUint32(8, totalLength, true);
  view.setUint32(12, jsonLength, true);
  view.setUint32(16, 0x4e4f534a, true);
  new Uint8Array(glb, 20, jsonLength).set(jsonBytes);
  if (binLength > 0) {
    const binHeader = 20 + jsonLength;
    view.setUint32(binHeader, binLength, true);
    view.setUint32(binHeader + 4, 0x004e4942, true);
    new Uint8Array(glb, binHeader + 8, binLength).set(binBytes);
  }
  return new Blob([glb], { type: "model/gltf-binary" });
}

/** Build a standards-compliant, self-contained GLB from a blr-scene-v1 mesh scene. */
export function createSceneGlb(scene) {
  const validated = validateScene(scene);
  return { blob: buildGlb(validated), summary: { objects: validated.objects.length, vertices: validated.vertexTotal, triangles: validated.triangleTotal, names: validated.objects.map(({ name }) => name) } };
}

function supportsWebGl(win) {
  try {
    const canvas = win.document.createElement("canvas");
    return Boolean(canvas.getContext("webgl2") || canvas.getContext("webgl"));
  } catch {
    return false;
  }
}

/**
 * Mount an interactive preview in `host`. Input positions use Blender's world-space Z-up axes.
 * Colors are linear sRGB RGBA factors in [0, 1], as required by glTF PBR materials.
 */
export function createScenePreview(host) {
  if (!host || typeof host.append !== "function" || !host.ownerDocument) throw new TypeError("createScenePreview requires a DOM host element");
  const doc = host.ownerDocument;
  const win = doc.defaultView;
  if (!win) throw new TypeError("createScenePreview requires a browser document");
  let viewer;
  let objectUrl;
  let currentBlob;
  let generation = 0;
  let disposed = false;
  let currentSummary;

  const shell = doc.createElement("div");
  shell.className = "blr-scene-preview";
  shell.style.display = "flex";
  shell.style.flexDirection = "column";
  shell.style.width = "100%";
  shell.style.height = "100%";
  shell.style.minHeight = "320px";
  const status = doc.createElement("p");
  status.className = "blr-scene-preview-status";
  status.setAttribute("role", "status");
  const downloadLink = doc.createElement("a");
  downloadLink.textContent = "Download scene GLB";
  downloadLink.download = "blr-scene.glb";
  downloadLink.hidden = true;
  shell.append(status, downloadLink);
  host.append(shell);

  const removeViewer = () => {
    if (!viewer) return;
    viewer.remove();
    viewer = undefined;
  };
  const revokeArtifact = () => {
    if (objectUrl) win.URL.revokeObjectURL(objectUrl);
    objectUrl = undefined;
    downloadLink.removeAttribute("href");
    downloadLink.hidden = true;
    currentSummary = undefined;
    currentBlob = undefined;
  };
  const failPreview = (message) => {
    removeViewer();
    const summary = currentSummary;
    const counts = summary ? `Scene summary: ${summary.objects} objects, ${summary.vertices} vertices, ${summary.triangles} triangles.` : "";
    status.textContent = `${message} ${counts} Download remains available.`;
  };
  const dispose = () => {
    if (disposed) return;
    disposed = true;
    generation += 1;
    removeViewer();
    revokeArtifact();
    shell.remove();
    win.removeEventListener("pagehide", dispose);
  };
  win.addEventListener("pagehide", dispose, { once: true });

  return {
    async show(scene) {
      if (disposed) throw new Error("Scene preview has been disposed");
      const { blob, summary } = createSceneGlb(scene);
      const token = ++generation;
      removeViewer();
      revokeArtifact();
      objectUrl = win.URL.createObjectURL(blob);
      currentBlob = blob;
      downloadLink.href = objectUrl;
      downloadLink.hidden = false;
      currentSummary = summary;
      status.textContent = `Scene ready: ${summary.objects} object${summary.objects === 1 ? "" : "s"}, ${summary.vertices} vertices, ${summary.triangles} triangles.`;
      if (summary.objects === 0) {
        status.textContent = "Scene contains no geometry: 0 objects, 0 vertices, 0 triangles. Empty GLB download available.";
        return summary;
      }
      if (!supportsWebGl(win)) {
        failPreview("Interactive preview unavailable: WebGL is not available in this browser.");
        return summary;
      }
      try {
        await import(new URL(VIEWER_PATH, import.meta.url).href);
        if (disposed || token !== generation) return summary;
        if (!win.customElements.get("model-viewer")) throw new Error("The local model-viewer component did not register.");
        const element = doc.createElement("model-viewer");
        element.setAttribute("camera-controls", "");
        element.setAttribute("interaction-prompt", "none");
        element.setAttribute("camera-orbit", INITIAL_ORBIT);
        element.setAttribute("camera-target", "auto auto auto");
        element.setAttribute("alt", `Blender scene preview: ${summary.names.join(", ")}`);
        element.style.display = "block";
        element.style.width = "100%";
        element.style.height = "100%";
        element.style.minHeight = "240px";
        element.style.flex = "1 1 auto";
        element.addEventListener("load", () => {
          if (!disposed && token === generation) status.textContent = `Interactive Blender mesh preview ready. ${summary.objects} objects, ${summary.triangles} triangles. Browser lighting.`;
        }, { once: true });
        element.addEventListener("error", () => {
          if (!disposed && token === generation) failPreview("Interactive preview failed to load.");
        }, { once: true });
        viewer = element;
        shell.prepend(element);
        element.src = objectUrl;
      } catch (error) {
        if (!disposed && token === generation) {
          const detail = error instanceof Error ? error.message : "unknown viewer error";
          failPreview(`Interactive preview unavailable: ${detail}`);
        }
      }
      return summary;
    },
    clear() {
      if (disposed) return;
      generation += 1;
      removeViewer();
      revokeArtifact();
      status.textContent = "No scene loaded.";
    },
    reset() {
      if (!viewer) return;
      viewer.setAttribute("camera-orbit", INITIAL_ORBIT);
      viewer.setAttribute("camera-target", "auto auto auto");
      viewer.jumpCameraToGoal();
    },
    orbit(deltaTheta, deltaPhi = 0) {
      if (!viewer) return;
      if (!Number.isFinite(deltaTheta) || !Number.isFinite(deltaPhi)) throw new TypeError("orbit deltas must be finite radians");
      const orbit = viewer.getCameraOrbit();
      const theta = ((orbit.theta + deltaTheta) % (Math.PI * 2) + Math.PI * 2) % (Math.PI * 2);
      const phi = Math.min(Math.PI - 0.02, Math.max(0.02, orbit.phi + deltaPhi));
      viewer.cameraOrbit = `${theta}rad ${phi}rad ${orbit.radius}m`;
      viewer.jumpCameraToGoal();
    },
    zoom(factor) {
      if (!Number.isFinite(factor) || factor <= 0) throw new TypeError("zoom factor must be finite and positive");
      if (!viewer) return;
      const orbit = viewer.getCameraOrbit();
      const radius = Math.max(0.001, Math.min(1_000_000, orbit.radius / factor));
      viewer.cameraOrbit = `${orbit.theta}rad ${orbit.phi}rad ${radius}m`;
      viewer.jumpCameraToGoal();
    },
    pan(x, y) {
      if (!Number.isFinite(x) || !Number.isFinite(y)) throw new TypeError("pan offsets must be finite");
      if (!viewer) return;
      const target = viewer.getCameraTarget();
      viewer.cameraTarget = `${target.x + x}m ${target.y + y}m ${target.z}m`;
      viewer.jumpCameraToGoal();
    },
    download() {
      if (!objectUrl) return undefined;
      downloadLink.click();
      return currentBlob;
    },
    artifact() {
      return currentSummary && objectUrl ? currentBlob : undefined;
    },
    dispose,
  };
}
