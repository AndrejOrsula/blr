const FIXED_ORBIT = "35deg 65deg 105%";
const FIXED_FIELD_OF_VIEW = "auto";
const EXPECTED_ASSETS = ["prism", "grid-low", "grid-high", "turntable"];
const EXPECTED_RECIPE = { prism: "procedural_asset", "grid-low": "geometry_nodes", "grid-high": "geometry_nodes", turntable: "camera_turntable" };
const assetURL = (path) => new URL(path, import.meta.url).href;
const status = document.getElementById("asset-status");
let buildIdentity;

function sendProtocol(type, message, phase) {
  if (window.parent === window) return;
  const body = { type, protocol: 1, id: "blr" };
  if (buildIdentity) body.build = buildIdentity;
  if (message) body.message = message;
  if (phase) body.phase = phase;
  window.parent.postMessage(body, location.origin);
}

function shellFailure(error) {
  console.error("BLR showcase initialization failed", error);
  const message = "Gallery controls are unavailable. Blender previews and downloads remain available.";
  if (status) status.textContent = `${message} Blender renders and downloads remain available.`;
  sendProtocol("wasm-demo-error", message, buildIdentity ? undefined : "startup");
}

function validateCatalog(catalog) {
  if (!catalog || typeof catalog !== "object" || Array.isArray(catalog)) throw new Error("Gallery catalog has an unsupported format.");
  if (typeof catalog.build === "string" && /^[0-9a-f]{64}$/.test(catalog.build)) buildIdentity = catalog.build;
  if (catalog.schema_version !== 1 || !Array.isArray(catalog.assets)) throw new Error("Gallery catalog has an unsupported format.");
  if (!buildIdentity || catalog.provenance !== "build-provenance.json") throw new Error("Gallery build provenance is missing or invalid.");
  if (catalog.assets.length !== EXPECTED_ASSETS.length || catalog.assets.some((asset, index) => asset?.id !== EXPECTED_ASSETS[index])) {
    throw new Error("Gallery catalog does not contain the four expected recipe variants.");
  }
  for (const asset of catalog.assets) {
    const manifest = asset.manifest;
    if (typeof asset.title !== "string" || !asset.title || typeof asset.description !== "string" || typeof asset.recipe !== "string" ||
        asset.recipe !== EXPECTED_RECIPE[asset.id] || manifest?.schema_version !== 1 || manifest.recipe !== asset.recipe ||
        manifest.runtime?.engine !== "CYCLES" || manifest.runtime?.device !== "CPU" ||
        !Number.isInteger(manifest.geometry?.triangles) || manifest.geometry.triangles <= 0 ||
        !Number.isInteger(manifest.seed) || !Number.isFinite(manifest.parameters?.height) ||
        !manifest.files || typeof manifest.files !== "object") {
      throw new Error(`Gallery metadata is incomplete for ${asset.id}.`);
    }
    for (const [file, mediaType] of [["asset.glb", "model/gltf-binary"], ["asset.blend", "application/x-blender"], ["preview.png", "image/png"]]) {
      const record = manifest.files[file];
      if (record?.media_type !== mediaType || !/^[0-9a-f]{64}$/.test(record.sha256 ?? "") || !Number.isInteger(record.bytes) || record.bytes <= 0) {
        throw new Error(`Gallery download metadata is incomplete for ${asset.id}/${file}.`);
      }
    }
    if (asset.id === "grid-low" && (manifest.variant !== "low" || !Number.isFinite(manifest.parameters.height))) {
      throw new Error("The low Geometry Nodes variant is invalid.");
    }
    if (asset.id === "grid-high" && (manifest.variant !== "high" || !Number.isFinite(manifest.parameters.height))) {
      throw new Error("The high Geometry Nodes variant is invalid.");
    }
    if (asset.id === "turntable") {
      const frames = manifest.frames;
      if (manifest.frame_count !== 8 || !Number.isFinite(manifest.fps) || manifest.fps !== 8 || !Array.isArray(frames) || frames.length !== 8 ||
          frames.some((frame, index) => frame?.frame !== index + 1 || frame.file !== `frames/frame-${String(index + 1).padStart(4, "0")}.png` ||
            !manifest.files[frame.file] || manifest.files[frame.file].media_type !== "image/png" ||
            !/^[0-9a-f]{64}$/.test(manifest.files[frame.file].sha256 ?? "") || !Number.isInteger(manifest.files[frame.file].bytes) || manifest.files[frame.file].bytes <= 0)) {
        throw new Error("Camera turntable frame metadata is incomplete.");
      }
    }
  }
  if (catalog.assets[1].manifest.parameters.height * 2 !== catalog.assets[2].manifest.parameters.height) {
    throw new Error("Geometry Nodes variants do not match their half/full height contract.");
  }
  return catalog.assets;
}

function requiredElement(id) {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Showcase control is missing: ${id}.`);
  return element;
}

async function initialize() {
  if (window.parent !== window) document.documentElement.dataset.embedded = "true";
  // Empty docs builds intentionally retain only the static fallback; do not probe absent assets.
  if (!document.getElementById("asset-gallery")) return;
  try {
    const response = await fetch(assetURL("./gallery.json"), { cache: "no-cache" });
    if (!response.ok) throw new Error(`Gallery catalog request failed (${response.status}).`);
    const catalog = await response.json();
    const assets = validateCatalog(catalog);

    const gallery = requiredElement("asset-gallery");
    const stage = gallery.querySelector(".stage");
    const preview = requiredElement("asset-preview");
    const renderButton = requiredElement("render-view");
    const modelButton = requiredElement("model-view");
    const resetButton = requiredElement("reset-view");
    const orbitHelp = gallery.querySelector(".orbit-help");
    const recipeChoices = gallery.querySelector(".recipe-choices");
    const nodeVariants = requiredElement("node-variants");
    const turntableControls = requiredElement("turntable-controls");
    const playButton = requiredElement("turntable-play");
    const previousButton = requiredElement("turntable-prev");
    const nextButton = requiredElement("turntable-next");
    const frameRange = requiredElement("turntable-frame");
    const frameLabel = requiredElement("turntable-frame-label");
    const downloadFrame = requiredElement("download-frame");
    const title = requiredElement("asset-title");
    const description = requiredElement("asset-description");
    const triangles = requiredElement("triangle-count");
    const seed = requiredElement("asset-seed");
    const height = requiredElement("asset-height");
    const stageLabel = requiredElement("stage-label");
    const downloadGlb = requiredElement("download-glb");
    const downloadBlend = requiredElement("download-blend");
    const downloadManifest = requiredElement("download-manifest");
    const recipeSource = document.getElementById("recipe-source");
    const statusElement = requiredElement("asset-status");
    if (!(preview instanceof HTMLImageElement) || !(stage instanceof HTMLElement) || !(frameRange instanceof HTMLInputElement) ||
        frameRange.type !== "range" || Number(frameRange.min) !== 1 || Number(frameRange.max) !== 8) {
      throw new Error("Showcase markup has unsupported controls.");
    }
    if (!status || status !== statusElement) throw new Error("Showcase status element is missing.");
    frameLabel.setAttribute("aria-live", "off");

    const recipeButtons = Array.from(recipeChoices?.querySelectorAll("button[data-recipe]") ?? []);
    const nodeButtons = Array.from(nodeVariants.querySelectorAll("button[data-variant]"));
    if (recipeButtons.length !== 3 || new Set(recipeButtons.map((button) => button.dataset.recipe)).size !== 3 ||
        !["procedural_asset", "geometry_nodes", "camera_turntable"].every((recipe) => recipeButtons.some((button) => button.dataset.recipe === recipe)) ||
        nodeButtons.length !== 2 || !nodeButtons.some((button) => button.dataset.variant === "grid-low") ||
        !nodeButtons.some((button) => button.dataset.variant === "grid-high") ||
        recipeButtons.some((button) => !button.disabled) || nodeButtons.some((button) => !button.disabled)) {
      throw new Error("Showcase recipe controls do not match the gallery catalog.");
    }
    const assetById = new Map(assets.map((asset) => [asset.id, asset]));
    let selected = assets[0];
    let generation = 0;
    let model;
    let cancelModelLoad;
    let viewerModule;
    let viewerImportFailed = false;
    let frameIndex = 0;
    let playbackTimer;
    let isPlaying = false;
    let playbackGeneration = 0;
    let frameRequest = 0;
    const decodedFrames = new Map();
    const modelFailures = new Map();

    const stopPlayback = () => {
      playbackGeneration += 1;
      isPlaying = false;
      clearTimeout(playbackTimer);
      playbackTimer = undefined;
      playButton.textContent = "Play";
      playButton.setAttribute("aria-label", "Play rendered orbit");
      playButton.setAttribute("aria-pressed", "false");
    };
    const pauseForPageState = () => { if (document.hidden) stopPlayback(); };
    document.addEventListener("visibilitychange", pauseForPageState);
    window.addEventListener("pagehide", () => {
      stopPlayback();
      frameRequest += 1;
      cancelModelLoad?.();
      if (model) model.src = "";
    }, { once: true });

    const orbitControls = document.createElement("span");
    orbitControls.className = "orbit-controls";
    orbitControls.setAttribute("role", "group");
    orbitControls.setAttribute("aria-label", "Keyboard camera controls");
    const adjustCamera = (thetaDelta = 0, phiDelta = 0, radiusScale = 1, action = "Camera view adjusted.") => {
      if (!model) return;
      const current = model.getCameraOrbit();
      const theta = ((current.theta + thetaDelta) % (Math.PI * 2) + Math.PI * 2) % (Math.PI * 2);
      const phi = Math.min(Math.PI - 0.05, Math.max(0.05, current.phi + phiDelta));
      const radius = current.radius * radiusScale;
      model.cameraOrbit = `${theta}rad ${phi}rad ${radius}m`;
      model.jumpCameraToGoal();
      statusElement.textContent = action;
    };
    const cameraButtons = [
      ["orbit-left", "Rotate left", "←", -0.35, 0, 1, "Camera rotated left."],
      ["orbit-right", "Rotate right", "→", 0.35, 0, 1, "Camera rotated right."],
      ["orbit-up", "Tilt up", "↑", 0, -0.25, 1, "Camera tilted up."],
      ["orbit-down", "Tilt down", "↓", 0, 0.25, 1, "Camera tilted down."],
      ["zoom-in", "Zoom in", "+", 0, 0, 0.85, "Camera zoomed in."],
      ["zoom-out", "Zoom out", "−", 0, 0, 1.15, "Camera zoomed out."],
    ];
    for (const [id, label, symbol, thetaDelta, phiDelta, radiusScale, message] of cameraButtons) {
      const button = document.createElement("button");
      button.id = id;
      button.type = "button";
      button.textContent = symbol;
      button.setAttribute("aria-label", label);
      button.title = label;
      button.addEventListener("click", () => adjustCamera(thetaDelta, phiDelta, radiusScale, message));
      orbitControls.append(button);
    }
    resetButton.before(orbitControls);

    const disposeModel = () => {
      cancelModelLoad?.();
      cancelModelLoad = undefined;
      if (!model) return;
      model.src = "";
      model.remove();
      model = undefined;
    };
    const showRender = (message = "Blender Cycles CPU render. Browser real-time lighting differs from Blender.") => {
      generation += 1;
      stopPlayback();
      frameRequest += 1;
      disposeModel();
      modelButton.setAttribute("aria-disabled", "false");
      modelButton.removeAttribute("aria-busy");
      preview.hidden = false;
      renderButton.setAttribute("aria-pressed", "true");
      modelButton.setAttribute("aria-pressed", "false");
      resetButton.disabled = true;
      orbitHelp?.setAttribute("hidden", "");
      stageLabel.textContent = selected.id === "turntable" ? "Blender · Cycles CPU · rendered frame" : "Blender · Cycles CPU";
      statusElement.textContent = message;
      turntableControls.hidden = selected.id !== "turntable";
      nodeVariants.hidden = !selected.id.startsWith("grid-");
    };
    const chooseAsset = (asset) => {
      stopPlayback();
      frameRequest += 1;
      selected = asset;
      gallery.dataset.selectedAsset = asset.id;
      showRender();
      const base = `./gallery/${asset.id}/`;
      preview.src = assetURL(`${base}preview.png`);
      preview.alt = `${asset.title}, rendered in Blender on CPU`;
      title.textContent = asset.title;
      description.textContent = asset.description;
      triangles.textContent = String(asset.manifest.geometry.triangles);
      seed.textContent = String(asset.manifest.seed);
      height.textContent = `${asset.manifest.parameters.height} m`;
      downloadGlb.href = assetURL(`${base}asset.glb`);
      downloadBlend.href = assetURL(`${base}asset.blend`);
      downloadManifest.href = assetURL(`${base}manifest.json`);
      if (recipeSource) recipeSource.href = assetURL(`${recipeSource.dataset.sourcePrefix}${asset.recipe}.rs`);
      for (const button of recipeButtons) button.setAttribute("aria-pressed", String(button.dataset.recipe === asset.recipe));
      for (const button of nodeButtons) button.setAttribute("aria-pressed", String(button.dataset.variant === asset.id));
      if (asset.id.startsWith("grid-")) {
        const lowHeight = assetById.get("grid-low").manifest.parameters.height;
        const highHeight = assetById.get("grid-high").manifest.parameters.height;
        statusElement.textContent = `Showing ${asset.id === "grid-low" ? "half" : "full"} height (${height.textContent}); choose ${asset.id === "grid-low" ? `full height (${highHeight} m)` : `half height (${lowHeight} m)`} to compare the same editable Geometry Nodes result.`;
      }
      if (asset.id === "turntable") {
        frameIndex = 0;
        frameRange.value = "1";
        setFrameLabel(0);
      } else {
        frameLabel.textContent = "";
        downloadFrame.removeAttribute("href");
      }
    };
    function setFrameLabel(index) {
      const entry = selected.manifest.frames[index];
      frameIndex = index;
      frameRange.value = String(entry.frame);
      frameLabel.textContent = `Frame ${entry.frame} of ${selected.manifest.frame_count} · ${selected.manifest.fps} fps`;
      downloadFrame.href = assetURL(`./gallery/${selected.id}/${entry.file}`);
      downloadFrame.download = entry.file.split("/").pop();
      downloadFrame.textContent = `Download frame ${entry.frame} PNG`;
    }
    const decodeFrame = (asset, index) => {
      const entry = asset.manifest.frames[index];
      const key = `${asset.id}:${entry.frame}`;
      if (decodedFrames.has(key)) return decodedFrames.get(key);
      const image = new Image();
      const promise = new Promise((resolve, reject) => {
        image.onload = async () => {
          try { if (typeof image.decode === "function") await image.decode(); resolve(image); }
          catch (error) { reject(error); }
        };
        image.onerror = () => reject(new Error(`Rendered frame ${entry.frame} could not be loaded.`));
        image.src = assetURL(`./gallery/${asset.id}/${entry.file}`);
      });
      decodedFrames.set(key, promise);
      promise.catch(() => decodedFrames.delete(key));
      while (decodedFrames.size > 8) decodedFrames.delete(decodedFrames.keys().next().value);
      return promise;
    };
    const selectFrame = async (index, { playback = false, token = 0 } = {}) => {
      if (selected.id !== "turntable") return false;
      const request = ++frameRequest;
      const asset = selected;
      try {
        const image = await decodeFrame(asset, index);
        if (request !== frameRequest || selected !== asset || (playback && token !== playbackGeneration)) return false;
        preview.src = image.src;
        preview.alt = `${asset.title}, rendered Blender frame ${asset.manifest.frames[index].frame} of ${asset.manifest.frame_count}`;
        setFrameLabel(index);
        stageLabel.textContent = "Blender · Cycles CPU · rendered frame";
        return true;
      } catch (error) {
        if (request === frameRequest && selected === asset) {
          stopPlayback();
          setFrameLabel(frameIndex);
          console.error("BLR turntable frame failed", error);
          statusElement.textContent = "This rendered frame is unavailable. The Blender preview, scene, and downloads remain available.";
        }
        return false;
      }
    };
    const showModelFailure = (token, error, requestedModel = false) => {
      if (token !== generation) return;
      console.error("BLR interactive preview failed", error);
      if (requestedModel) modelFailures.set(selected.id, (modelFailures.get(selected.id) ?? 0) + 1);
      showRender("Interactive model unavailable. The Blender render and canonical downloads remain available. Select Inspect 3D to retry.");
    };
    const loadViewerModule = async () => {
      try {
        if (!viewerModule) {
          const loading = import(assetURL("./vendor/model-viewer.min.js"));
          let timeout;
          try {
            viewerModule = await Promise.race([
              loading,
              new Promise((_, reject) => { timeout = setTimeout(() => reject(new Error("Interactive viewer import timed out.")), 15000); }),
            ]);
          } finally { clearTimeout(timeout); }
        }
        if (!customElements.get("model-viewer")) throw new Error("Interactive viewer did not register its element.");
        viewerImportFailed = false;
      } catch (error) {
        viewerImportFailed = true;
        throw error;
      }
    };
    const waitForModel = (element, token) => new Promise((resolve, reject) => {
      let settled = false;
      const finish = (error) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        element.removeEventListener("load", onLoad);
        element.removeEventListener("error", onError);
        cancelModelLoad = undefined;
        if (error) reject(error); else resolve();
      };
      const onLoad = () => finish();
      const onError = () => finish(new Error("The GLB could not be loaded by the browser viewer."));
      const timer = setTimeout(() => finish(new Error("Interactive model loading timed out.")), 45000);
      element.addEventListener("load", onLoad, { once: true });
      element.addEventListener("error", onError, { once: true });
      cancelModelLoad = () => finish(new DOMException("Model request superseded.", "AbortError"));
      if (token !== generation) cancelModelLoad();
    });
    const inspectModel = async () => {
      if (modelButton.getAttribute("aria-disabled") === "true") return;
      const token = ++generation;
      stopPlayback();
      frameRequest += 1;
      disposeModel();
      modelButton.setAttribute("aria-disabled", "true");
      modelButton.setAttribute("aria-busy", "true");
      statusElement.textContent = "Loading the interactive model. The Blender render and downloads remain available.";
      let requestedModel = false;
      try {
        await loadViewerModule();
        if (token !== generation) return;
        const viewer = document.createElement("model-viewer");
        viewer.setAttribute("camera-controls", "");
        viewer.setAttribute("interaction-prompt", "none");
        viewer.setAttribute("camera-orbit", FIXED_ORBIT);
        viewer.setAttribute("field-of-view", FIXED_FIELD_OF_VIEW);
        viewer.setAttribute("touch-action", "pan-y");
        viewer.setAttribute("alt", `${selected.title}. Drag or touch to rotate. Use the visible camera buttons to orbit and zoom.`);
        model = viewer;
        stage.append(viewer);
        const ready = waitForModel(viewer, token);
        const failures = modelFailures.get(selected.id) ?? 0;
        const modelPath = `./gallery/${selected.id}/asset.glb`;
        requestedModel = true;
        viewer.src = assetURL(failures ? `${modelPath}?blr_retry=${failures}` : modelPath);
        await ready;
        if (token !== generation || model !== viewer) return;
        preview.hidden = true;
        renderButton.setAttribute("aria-pressed", "false");
        modelButton.setAttribute("aria-pressed", "true");
        resetButton.disabled = false;
        orbitHelp?.removeAttribute("hidden");
        turntableControls.hidden = true;
        stageLabel.textContent = "Interactive glTF · real-time browser lighting";
        statusElement.textContent = "Drag or touch the model to orbit. Use the camera buttons for keyboard orbit and zoom. Reset view restores the starting view.";
        viewer.addEventListener("error", () => showModelFailure(token, new Error("The interactive viewer reported a rendering error."), true));
      } catch (error) {
        if (token === generation && error?.name !== "AbortError") {
          showModelFailure(token, error, requestedModel);
          if (viewerImportFailed) modelButton.textContent = "Reload to retry 3D";
        }
      } finally {
        if (token === generation) {
          modelButton.setAttribute("aria-disabled", "false");
          modelButton.removeAttribute("aria-busy");
        }
      }
    };

    for (const button of recipeButtons) button.addEventListener("click", () => {
      switch (button.dataset.recipe) {
        case "procedural_asset": chooseAsset(assetById.get("prism")); break;
        case "geometry_nodes": chooseAsset(assetById.get(selected.id.startsWith("grid-") ? selected.id : "grid-low")); break;
        case "camera_turntable": chooseAsset(assetById.get("turntable")); break;
      }
    });
    for (const button of nodeButtons) button.addEventListener("click", () => {
      const asset = assetById.get(button.dataset.variant);
      if (asset) chooseAsset(asset);
    });
    renderButton.addEventListener("click", () => showRender());
    modelButton.addEventListener("click", () => {
      if (modelButton.getAttribute("aria-disabled") === "true") return;
      if (viewerImportFailed) window.location.reload();
      else inspectModel();
    });
    resetButton.addEventListener("click", () => {
      if (!model || modelButton.getAttribute("aria-pressed") !== "true") return;
      model.setAttribute("camera-orbit", FIXED_ORBIT);
      model.setAttribute("camera-target", "auto auto auto");
      model.setAttribute("field-of-view", FIXED_FIELD_OF_VIEW);
      model.jumpCameraToGoal();
      statusElement.textContent = "View reset to the full starting frame.";
    });
    previousButton.addEventListener("click", () => { stopPlayback(); void selectFrame((frameIndex + 7) % 8); });
    nextButton.addEventListener("click", () => { stopPlayback(); void selectFrame((frameIndex + 1) % 8); });
    frameRange.addEventListener("input", () => { stopPlayback(); void selectFrame(Number(frameRange.value) - 1); });
    playButton.addEventListener("click", async () => {
      if (selected.id !== "turntable") return;
      if (isPlaying) { stopPlayback(); return; }
      isPlaying = true;
      const token = ++playbackGeneration;
      const period = 1000 / selected.manifest.fps;
      let deadline = performance.now() + period;
      playButton.textContent = "Pause";
      playButton.setAttribute("aria-label", "Pause rendered orbit");
      playButton.setAttribute("aria-pressed", "true");
      statusElement.textContent = `Playing eight prerecorded Blender Cycles frames at ${selected.manifest.fps} fps.`;
      const tick = async () => {
        if (token !== playbackGeneration || document.hidden || selected.id !== "turntable") return;
        const next = (frameIndex + 1) % 8;
        if (!await selectFrame(next, { playback: true, token })) return;
        if (token === playbackGeneration) {
          deadline += period;
          playbackTimer = setTimeout(tick, Math.max(0, deadline - performance.now()));
        }
      };
      if (!await selectFrame(frameIndex, { playback: true, token })) return;
      if (token === playbackGeneration) playbackTimer = setTimeout(tick, 1000 / selected.manifest.fps);
    });

    gallery.querySelector(".view-controls")?.removeAttribute("hidden");
    modelButton.textContent = "Inspect 3D";
    modelButton.disabled = false;
    modelButton.setAttribute("aria-disabled", "false");
    resetButton.disabled = true;
    chooseAsset(assets[0]);
    for (const button of recipeButtons) button.disabled = false;
    for (const button of nodeButtons) button.disabled = false;
    sendProtocol("wasm-demo-ready");
  } catch (error) {
    shellFailure(error);
  }
}

initialize();
