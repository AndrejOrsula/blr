(() => {
  "use strict";

  const form = document.querySelector("#demo-form");
  const sizeInput = document.querySelector("#cube-size");
  const sizeRange = document.querySelector("#cube-size-range");
  const runButton = document.querySelector("#run-button");
  const cancelButton = document.querySelector("#cancel-button");
  const message = document.querySelector("#status-message");
  const phase = document.querySelector("#phase");
  const output = document.querySelector(".output-panel");
  const emptyState = document.querySelector("#empty-state");
  const resultPanel = document.querySelector("#result");
  const details = document.querySelector("#run-details");
  const log = document.querySelector("#run-log");
  const statusCells = {
    version: document.querySelector("#result-version"),
    vertices: document.querySelector("#result-vertices"),
    edges: document.querySelector("#result-edges"),
    triangles: document.querySelector("#result-triangles"),
    size: document.querySelector("#result-size"),
  };

  let active = null;
  let serial = 0;
  const maxLogEntries = 12;
  window.__blrDemo = { phase: "Checking browser", lastResult: null, exitStatus: null };

  function setPhase(label, state = "") {
    phase.textContent = label;
    window.__blrDemo.phase = label;
    if (state) phase.dataset.state = state;
    else delete phase.dataset.state;
  }

  function setMessage(text) {
    message.textContent = text;
  }

  function setReady() {
    runButton.disabled = false;
    runButton.querySelector("span:last-child").textContent = "Run Blender";
    setPhase("Ready", "ready");
    setMessage("Ready when you are. The Blender runtime starts after you press Run Blender.");
  }

  function setUnavailable(reason) {
    runButton.disabled = true;
    runButton.querySelector("span:last-child").textContent = "Unavailable in this browser";
    setPhase("Unavailable", "error");
    setMessage(reason);
  }

  function checkCapabilities() {
    if (typeof Blob === "undefined" || typeof Worker === "undefined" || typeof DecompressionStream === "undefined" || typeof crypto === "undefined" || !crypto.subtle) {
      throw new Error("This browser is missing a required feature (workers, streaming decompression, or SHA-256 verification). Try a recent desktop browser.");
    }
    if (!window.isSecureContext) {
      throw new Error("The isolated runtime needs a secure page. Open this demo over HTTPS.");
    }
    if (!window.blrIsolationReady || typeof window.blrIsolationReady.then !== "function") {
      throw new Error("Browser isolation could not be initialized. Reload the page and try again.");
    }
    return window.blrIsolationReady;
  }

  function syncInputs(value, fromRange = false) {
    sizeInput.setCustomValidity("");
    if (String(value).trim() === "") return;
    const parsed = Number(value);
    if (!Number.isFinite(parsed) || parsed < 0.1 || parsed > 4) return;
    if (fromRange) sizeInput.value = String(parsed);
    sizeRange.value = String(parsed);
  }

  function appendLog(text) {
    const lines = log.textContent ? log.textContent.split("\n") : [];
    lines.push(text);
    log.textContent = lines.slice(-maxLogEntries).join("\n");
    details.hidden = false;
  }

  function normalizeCount(value) {
    const number = Number(value);
    return Number.isFinite(number) && number >= 0 ? String(Math.trunc(number)) : null;
  }

  function pick(record, ...keys) {
    for (const key of keys) if (record && record[key] !== undefined && record[key] !== null) return record[key];
    return undefined;
  }

  function unpackResult(payload) {
    const data = payload?.result && typeof payload.result === "object" && !Array.isArray(payload.result) ? payload.result : payload;
    const version = pick(data, "blender_version", "blenderVersion", "version") ?? pick(payload, "blender_version", "blenderVersion", "version");
    const metrics = Array.isArray(data?.result) ? data.result : Array.isArray(payload?.result) ? payload.result : null;
    const geometry = data?.geometry && typeof data.geometry === "object" ? data.geometry : data;
    const vertices = normalizeCount(metrics?.[1] ?? pick(geometry, "vertices", "vertexCount", "vertex_count", "verts"));
    const edges = normalizeCount(metrics?.[2] ?? pick(geometry, "edges", "edgeCount", "edge_count"));
    const triangles = normalizeCount(metrics?.[3] ?? pick(geometry, "triangles", "triangleCount", "triangle_count"));
    const size = pick(data, "requested_size", "requestedSize", "size", "cubeSize", "cube_size") ?? pick(payload, "requested_size", "requestedSize", "size", "cubeSize", "cube_size") ?? metrics?.[0];
    const pythonVersion = pick(data, "python_version", "pythonVersion");
    const rustModule = pick(data, "rust_module", "rustModule");
    const inventoryUnchanged = pick(data, "inventory_unchanged", "inventoryUnchanged");
    if (version === undefined || vertices === null || edges === null || triangles === null || !Number.isFinite(Number(size))
      || typeof pythonVersion !== "string" || !/^3\.13\./.test(pythonVersion)
      || rustModule?.module !== "_blr_browser_probe" || rustModule?.function_module !== "_blr_browser_probe"
      || rustModule?.origin !== "built-in" || rustModule?.has_file !== false
      || inventoryUnchanged !== true) {
      throw new Error("Blender returned an incomplete geometry result. The run was not marked successful.");
    }
    const versionLabel = Array.isArray(version) ? version.join(".") : String(version);
    return { version: versionLabel, pythonVersion, vertices, edges, triangles, size: Number(size), builtInRust: true, inventoryUnchanged: true };
  }

  function cleanFrame(run, remove = true) {
    clearTimeout(run.timeout);
    window.removeEventListener("message", run.onMessage);
    run.frame.removeEventListener("error", run.onFrameError);
    if (remove) run.frame.remove();
    if (active === run) active = null;
  }

  function showResult(result) {
    window.__blrDemo.lastResult = { ...result };
    statusCells.version.textContent = result.version;
    statusCells.vertices.textContent = result.vertices;
    statusCells.edges.textContent = result.edges;
    statusCells.triangles.textContent = result.triangles;
    statusCells.size.textContent = `${result.size} m`;
    emptyState.hidden = true;
    resultPanel.hidden = false;
    output.setAttribute("aria-busy", "false");
    setPhase("Complete", "ready");
    setMessage(`Blender completed the ${result.size} meter cube scene successfully.`);
  }

  function finishIfProven(run) {
    if (active !== run || !run.result || !run.exitReceived) return;
    if (run.exitCode !== 0) {
      failRun(run, `Blender stopped with exit code ${String(run.exitCode)}. Adjust the size or retry.`);
      return;
    }
    cleanFrame(run);
    runButton.disabled = false;
    runButton.querySelector("span:last-child").textContent = "Run again";
    cancelButton.hidden = true;
    showResult(run.result);
    appendLog(`Completed · exit ${run.exitCode} · Blender ${run.result.version} · Python ${run.result.pythonVersion} · built-in Rust callback · inventories unchanged.`);
  }

  function failRun(run, reason) {
    if (active !== run) return;
    cleanFrame(run);
    runButton.disabled = false;
    runButton.querySelector("span:last-child").textContent = "Retry Blender";
    cancelButton.hidden = true;
    resultPanel.hidden = true;
    emptyState.hidden = false;
    output.setAttribute("aria-busy", "false");
    setPhase("Run failed", "error");
    setMessage(reason);
    appendLog(reason);
  }

  function handleMessage(run, event) {
    if (active !== run || event.source !== run.frame.contentWindow || event.origin !== location.origin) return;
    const data = event.data;
    if (!data || data.channel !== "blr-demo" || data.runId !== run.id || typeof data.type !== "string") return;

    if (data.type === "status") {
      const label = typeof data.phase === "string" ? data.phase : typeof data.status === "string" ? data.status : "Running Blender";
      setPhase(label, "running");
      if (typeof data.message === "string" && data.message.length < 240) setMessage(data.message);
      return;
    }
    if (data.type === "result") {
      try {
        run.result = unpackResult(data);
        if (run.result.version !== "5.2.0" || Math.abs(run.result.size - run.size) > 1e-9
          || run.result.vertices !== "8" || run.result.edges !== "12" || run.result.triangles !== "12") {
          throw new Error("Blender returned geometry outside the expected cube result. The run was not marked successful.");
        }
        appendLog(`Scene result received · ${run.result.vertices} vertices, ${run.result.edges} edges, ${run.result.triangles} triangles.`);
        finishIfProven(run);
      } catch (error) {
        failRun(run, error instanceof Error ? error.message : "Blender returned an invalid result.");
      }
      return;
    }
    if (data.type === "exit") {
      run.exitReceived = true;
      run.exitCode = Number(data.code ?? data.status ?? data.exitCode ?? data.exit_code);
      window.__blrDemo.exitStatus = Number.isInteger(run.exitCode) ? run.exitCode : null;
      if (!Number.isInteger(run.exitCode)) {
        failRun(run, "The runtime ended without a valid exit code. Retry the run.");
        return;
      }
      appendLog(`Runtime exit code ${run.exitCode}.`);
      finishIfProven(run);
      return;
    }
    if (data.type === "error") {
      const reason = typeof data.message === "string" ? data.message : "The Blender runtime reported an error.";
      failRun(run, reason.slice(0, 260));
    }
  }

  async function startRun(size) {
    window.__blrDemo.lastResult = null;
    window.__blrDemo.exitStatus = null;
    const run = {
      id: `${Date.now().toString(36)}-${(++serial).toString(36)}-${crypto.getRandomValues(new Uint32Array(1))[0].toString(36)}`,
      size,
      frame: document.createElement("iframe"),
      result: null,
      exitReceived: false,
      exitCode: null,
      timeout: 0,
    };
    active = run;
    run.frame.hidden = true;
    run.frame.title = "Isolated headless Blender runtime";
    run.frame.setAttribute("aria-hidden", "true");
    run.onMessage = (event) => handleMessage(run, event);
    run.onFrameError = () => failRun(run, "The Blender runtime frame could not load. Check your connection and retry.");
    run.frame.addEventListener("error", run.onFrameError, { once: true });
    window.addEventListener("message", run.onMessage);

    emptyState.hidden = false;
    resultPanel.hidden = true;
    details.hidden = true;
    log.textContent = "";
    output.setAttribute("aria-busy", "true");
    runButton.disabled = true;
    runButton.querySelector("span:last-child").textContent = "Running…";
    cancelButton.hidden = false;
    setPhase("Starting runtime", "running");
    setMessage("Starting a fresh Blender runtime. Runtime files total about 33 MB.");

    run.timeout = window.setTimeout(() => failRun(run, "This run exceeded 180 seconds and was stopped. You can retry with a smaller scene or try again later."), 180000);
    const url = new URL("./runtime-frame.html", location.href);
    url.searchParams.set("size", String(size));
    url.searchParams.set("run", run.id);
    run.frame.src = url.href;
    document.body.append(run.frame);
  }

  function cancelRun() {
    if (!active) return;
    const run = active;
    cleanFrame(run);
    runButton.disabled = false;
    runButton.querySelector("span:last-child").textContent = "Run again";
    cancelButton.hidden = true;
    resultPanel.hidden = true;
    emptyState.hidden = false;
    output.setAttribute("aria-busy", "false");
    setPhase("Canceled");
    setMessage("Run canceled. Its runtime frame was discarded; you can safely try again.");
    appendLog("Canceled by user; runtime frame discarded.");
  }

  sizeInput.addEventListener("input", () => syncInputs(sizeInput.value));
  sizeRange.addEventListener("input", () => syncInputs(sizeRange.value, true));
  cancelButton.addEventListener("click", cancelRun);
  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (active) return;
    sizeInput.setCustomValidity("");
    if (!form.reportValidity()) return;
    const size = Number(sizeInput.value);
    if (!Number.isFinite(size) || size < 0.1 || size > 4) {
      sizeInput.setCustomValidity("Enter a cube size from 0.1 to 4 meters.");
      form.reportValidity();
      return;
    }
    runButton.disabled = true;
    runButton.querySelector("span:last-child").textContent = "Preparing browser…";
    setPhase("Preparing", "running");
    setMessage("Preparing browser isolation. Blender has not started yet.");
    try {
      await window.blrIsolationReady;
      if (active) return;
      await startRun(size);
    } catch (error) {
      runButton.disabled = false;
      runButton.querySelector("span:last-child").textContent = "Retry Blender";
      setPhase("Unavailable", "error");
      setMessage(error instanceof Error ? error.message : "Browser isolation failed. Reload and retry.");
      appendLog("Isolation setup failed; no runtime frame was started.");
    }
  });

  try {
    checkCapabilities().then(setReady).catch((error) => setUnavailable(error instanceof Error ? error.message : "Browser isolation failed. Reload the page and try again."));
  } catch (error) {
    setUnavailable(error instanceof Error ? error.message : "This browser cannot run the isolated demo.");
  }
})();
