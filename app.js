const MAX_COMMAND_BYTES = 256 * 1024;
const MAX_SOURCE_BYTES = 256 * 1024;
const MAX_GUTTER_LINES = 5000;
const MAX_DIAGNOSTIC_CHARS = 48 * 1024;
const MAX_GUEST_MEMORY_BYTES = 256 * 1024 * 1024;
const SUPPORTED_WASI_IMPORTS = new Set([
  "args_get", "args_sizes_get", "clock_res_get", "clock_time_get",
  "environ_get", "environ_sizes_get", "fd_close", "fd_fdstat_get",
  "fd_fdstat_set_flags", "fd_fdstat_set_rights", "fd_filestat_get",
  "fd_read", "fd_seek", "fd_sync", "fd_tell", "fd_write",
  "poll_oneoff", "proc_exit", "proc_raise", "random_get", "sched_yield",
]);
const DRAFT_PREFIX = "blr-playground-draft-v1:";
const $ = (selector) => document.querySelector(selector);

const select = $("#example-select");
const editor = $("#source-editor");
const gutter = $("#line-numbers");
const runButton = $("#run-button");
const cancelButton = $("#cancel-button");
const resetButton = $("#reset-button");
const diagnostics = $("#diagnostics");
const phase = $("#phase");
const runPanel = $(".run-panel");
const runtimeNote = $("#runtime-note");
const viewportEmpty = $("#viewport-empty");
const viewportMessage = $("#viewport-message");
const previewBadge = $("#preview-badge");
const stats = {
  objects: $("#stat-objects"),
  vertices: $("#stat-vertices"),
  triangles: $("#stat-triangles"),
  version: $("#stat-version"),
};
const stages = { compile: $("#stage-compile"), blender: $("#stage-blender"), preview: $("#stage-preview") };
let compileRust;
let runBlenderScene;
let createScenePreview;
let preview;

let examples = [];
let activeExample = null;
let activeRun = null;
let runSerial = 0;
let draftTimer = 0;
let ready = false;
let currentPreview = false;
let hasRunResult = false;
let lastGuestBoundaryProof = null;

function setPhase(label, state = "idle") {
  phase.textContent = label;
  phase.dataset.state = state;
}

function appendDiagnostic(text) {
  if (typeof text !== "string" || text.length === 0) return;
  const next = `${diagnostics.textContent}${diagnostics.textContent ? "\n" : ""}${text}`;
  diagnostics.textContent = next.length > MAX_DIAGNOSTIC_CHARS ? next.slice(-MAX_DIAGNOSTIC_CHARS) : next;
  diagnostics.scrollTop = diagnostics.scrollHeight;
}

function setStage(name, state, detail) {
  const element = stages[name];
  if (!element) return;
  element.dataset.state = state;
  if (detail) element.setAttribute("aria-label", `${element.textContent.trim()}: ${detail}`);
  else element.removeAttribute("aria-label");
}

function resetStages() {
  for (const name of Object.keys(stages)) setStage(name, "idle");
}

function setRunButton(label, disabled = false) {
  runButton.disabled = disabled;
  runButton.querySelector("span:last-child").textContent = label;
}

function sourceWithinLimit(source) {
  return new TextEncoder().encode(source).byteLength <= MAX_SOURCE_BYTES;
}

function updateGutter() {
  if (!sourceWithinLimit(editor.value)) {
    gutter.textContent = "…";
    gutter.scrollTop = editor.scrollTop;
    return;
  }
  let count = 1;
  for (const character of editor.value) if (character === "\n") count += 1;
  const shown = Math.min(count, MAX_GUTTER_LINES);
  gutter.textContent = Array.from({ length: shown }, (_, index) => String(index + 1)).join("\n");
  if (count > MAX_GUTTER_LINES) gutter.textContent += `\n… (${count} lines)`;
  gutter.scrollTop = editor.scrollTop;
}

function draftKey(id) {
  return `${DRAFT_PREFIX}${id}`;
}

function saveDraft() {
  if (!activeExample) return;
  try {
    localStorage.setItem(draftKey(activeExample.id), editor.value);
    $("#draft-status").textContent = "Draft saved in this browser";
  } catch {
    $("#draft-status").textContent = "Draft stays in this tab (browser storage unavailable)";
  }
}

function loadDraft(example) {
  try {
    return localStorage.getItem(draftKey(example.id)) ?? example.source;
  } catch {
    return example.source;
  }
}

function selectExample(id) {
  if (activeRun) return;
  const example = examples.find((item) => item.id === id);
  if (!example) return;
  if (activeExample) saveDraft();
  activeExample = example;
  editor.value = loadDraft(example);
  editor.disabled = false;
  resetButton.disabled = false;
  $("#draft-status").textContent = `Drafts saved per example · ${example.title}`;
  $("#example-description").textContent = example.description;
  updateGutter();
  if (!sourceWithinLimit(editor.value)) runtimeNote.textContent = "Saved draft exceeds the 256 KiB source limit. Reduce it before running; the draft was preserved.";
  markResultStale();
}

function markResultStale() {
  if (!hasRunResult) return;
  previewBadge.textContent = "Previous run";
  runtimeNote.textContent = "Source changed. Geometry and stats show the previous completed run; run again to update them.";
}

function enablePreviewControls(interactive, artifact = interactive) {
  for (const id of ["orbit-left", "orbit-right", "zoom-in", "zoom-out", "pan-left", "pan-right", "preview-reset"]) {
    $(`#${id}`).disabled = !interactive;
  }
  $("#download-scene").disabled = !artifact;
}

function summarizeIdentity(identity) {
  if (!identity || typeof identity !== "object") return "Unknown";
  const value = identity.blenderVersion ?? identity.blender_version ?? identity.version;
  return Array.isArray(value) ? value.join(".") : typeof value === "string" || typeof value === "number" ? String(value) : "Unknown";
}

function setSceneSummary(summary, identity) {
  stats.objects.textContent = String(summary.objects);
  stats.vertices.textContent = String(summary.vertices);
  stats.triangles.textContent = String(summary.triangles);
  stats.version.textContent = summarizeIdentity(identity);
  currentPreview = $("#preview-host model-viewer") !== null;
  hasRunResult = true;
  enablePreviewControls(currentPreview, Boolean(preview.artifact()));
  previewBadge.textContent = currentPreview ? "Preview ready" : "Mesh ready · viewer unavailable";
  viewportEmpty.hidden = true;
}

function clearScene(message = "Your Blender-created mesh will appear here.") {
  preview.clear();
  currentPreview = false;
  hasRunResult = false;
  enablePreviewControls(false);
  viewportEmpty.hidden = false;
  viewportMessage.textContent = message;
  previewBadge.textContent = "Waiting for a run";
  for (const cell of Object.values(stats)) cell.textContent = "—";
}

function errorText(error, fallback) {
  const message = error instanceof Error && error.message ? error.message : fallback;
  return message.length > 1600 ? `${message.slice(0, 1600)} … [error text truncated]` : message;
}

function formatGuestBoundary(value) {
  if (value === undefined || value === null) return null;
  if (typeof value === "string") return value.slice(0, 2048);
  try {
    const serialized = JSON.stringify(value);
    return typeof serialized === "string" ? serialized.slice(0, 2048) : "[guest boundary metadata unavailable]";
  } catch {
    return "[guest boundary metadata could not be serialized]";
  }
}

function validateGuestBoundary(boundary) {
  if (!boundary || typeof boundary !== "object" || Array.isArray(boundary)) return false;
  const imports = boundary.wasmImports;
  return Array.isArray(imports)
    && imports.length > 0
    && imports.length <= 256
    && imports.every((value) => {
      if (typeof value !== "string") return false;
      const separator = value.indexOf(".");
      return separator > 0
        && value.slice(0, separator) === "wasi_snapshot_preview1"
        && SUPPORTED_WASI_IMPORTS.has(value.slice(separator + 1));
    })
    && Number.isSafeInteger(boundary.memoryMaxBytes)
    && boundary.memoryMaxBytes > 0
    && boundary.memoryMaxBytes <= MAX_GUEST_MEMORY_BYTES
    && boundary.preopens === 0
    && boundary.workerTerminated === true
    && boundary.termination === "completed";
}

function parseSceneCommands(stdout) {
  if (typeof stdout !== "string") throw new Error("Rust compiler returned no scene-command output.");
  const lines = stdout.split(/\r?\n/);
  const prefix = "BLR_SCENE_COMMANDS ";
  const payloads = lines.filter((line) => line.startsWith(prefix));
  if (payloads.length !== 1) throw new Error("Expected exactly one BLR_SCENE_COMMANDS JSON line on Rust stdout.");
  const outputLines = lines.filter((line) => line.trim() && !line.startsWith(prefix));
  const json = payloads[0].slice(prefix.length);
  if (new TextEncoder().encode(json).byteLength > MAX_COMMAND_BYTES) throw new Error("Scene commands exceed the 256 KiB protocol limit.");
  let commands;
  try {
    commands = JSON.parse(json);
  } catch {
    throw new Error("Rust emitted invalid scene-command JSON.");
  }
  if (!commands || typeof commands !== "object" || Array.isArray(commands) || commands.schema !== "blr-scene-commands-v1" || !Array.isArray(commands.objects)) {
    throw new Error('Rust output must use the "blr-scene-commands-v1" schema.');
  }
  return { json, outputLines };
}

function onOutput(run, record) {
  if (activeRun !== run || run.controller.signal.aborted || !record || typeof record.text !== "string") return;
  const stream = record.stream === "stderr" ? "stderr" : "stdout";
  run.streamedStreams.add(stream);
  if (run.guestStarted) run.guestStreamedStreams.add(stream);
  const clipped = record.text.length > 4000 ? `${record.text.slice(0, 4000)} … [output chunk truncated]` : record.text;
  appendDiagnostic(`[${stream}] ${clipped.replace(/[\r\n]+$/, "")}`);
}

function onStatus(run, message) {
  if (activeRun !== run || run.controller.signal.aborted) return;
  const text = typeof message === "string" ? message : "Working…";
  if (/Running compiled Rust guest/i.test(text)) run.guestStarted = true;
  runtimeNote.textContent = text.slice(0, 220);
  if (stages.blender.dataset.state === "running") {
    setPhase(/isolation/i.test(text) ? "Preparing browser isolation" : "Running Blender", "running");
    setStage("blender", "running", text);
  } else if (stages.compile.dataset.state === "running") {
    setPhase("Compiling Rust", "running");
    setStage("compile", "running", text);
  }
}

async function runCurrentSource() {
  if (!ready || activeRun || !activeExample || editor.disabled) return;
  if (!sourceWithinLimit(editor.value)) {
    setPhase("Source too large", "error");
    runtimeNote.textContent = "Source exceeds the 256 KiB limit. Reduce it before running; your text remains in the editor.";
    appendDiagnostic("[input error] Source exceeds the 256 KiB limit; no compiler or runtime started.");
    return;
  }
  saveDraft();
  const run = {
    id: ++runSerial,
    controller: new AbortController(),
    streamedStreams: new Set(),
    guestStreamedStreams: new Set(),
    guestStarted: false,
  };
  activeRun = run;
  clearScene("Running new source. Previous preview cleared.");
  resetStages();
  diagnostics.textContent = "";
  runPanel.setAttribute("aria-busy", "true");
  cancelButton.hidden = false;
  setRunButton("Running…", true);
  resetButton.disabled = true;
  select.disabled = true;
  editor.disabled = true;
  setPhase("Starting Rust compile", "running");
  setStage("compile", "running", "Starting compiler");
  runtimeNote.textContent = "Starting the Rust compiler. Runtime assets load only for an explicit run.";
  const options = {
    signal: run.controller.signal,
    onStatus: (text) => onStatus(run, text),
    onOutput: (record) => onOutput(run, record),
  };

  try {
    appendDiagnostic(`[source] ${activeExample.title} · ${editor.value.length} characters`);
    const compiled = await compileRust(editor.value, options);
    if (activeRun !== run || run.controller.signal.aborted) return;
    lastGuestBoundaryProof = formatGuestBoundary(compiled?.guestBoundary);
    if (lastGuestBoundaryProof) {
      const boundary = compiled?.guestBoundary;
      appendDiagnostic(`[guest boundary] ${lastGuestBoundaryProof}`);
      if (validateGuestBoundary(boundary)) {
        const memoryMiB = boundary.memoryMaxBytes / (1024 * 1024);
        appendDiagnostic(`[guest boundary summary] Guest Wasm imports: ${boundary.wasmImports.length} allowlisted wasi_snapshot_preview1 functions only; memory max: ${memoryMiB} MiB; preopens: 0; worker terminated: true.`);
      }
    }
    if (!compiled || compiled.ok !== true || compiled.exitCode !== 0) {
      const failedDuringGuestRun = compiled?.stage === "run";
      setStage("compile", failedDuringGuestRun ? "complete" : "error", failedDuringGuestRun ? "Rust compiled; guest execution failed" : "Compilation failed");
      const streamed = failedDuringGuestRun ? run.guestStreamedStreams : run.streamedStreams;
      for (const stream of ["stderr", "stdout"]) {
        if (typeof compiled?.[stream] === "string" && compiled[stream] && !streamed.has(stream)) {
          const label = failedDuringGuestRun ? `guest ${stream}` : `compiler ${stream}`;
          const clipped = compiled[stream].length > 12000 ? `${compiled[stream].slice(0, 12000)} … [fallback output truncated]` : compiled[stream];
          appendDiagnostic(`[${label}] ${clipped}`);
        }
      }
      throw new Error(`Rust ${failedDuringGuestRun ? "guest run" : "compilation"} failed with exit code ${compiled?.exitCode ?? "unknown"}. See diagnostics above.`);
    }
    setStage("compile", "complete", `Rust ${compiled.compilerVersion ?? "compiler"} succeeded`);
    appendDiagnostic(`[compile] Success · ${compiled.compilerVersion ?? "Rust"}`);
    if (compiled.stage !== "run") throw new Error("The compiler returned without running the compiled Rust guest.");
    if (!validateGuestBoundary(compiled.guestBoundary)) {
      throw new Error("The compiled Rust guest did not provide a valid bounded WASI isolation proof. Blender was not started.");
    }
    appendDiagnostic(`[guest] Completed · exit ${compiled.exitCode ?? 0}.`);
    let parsedCommands;
    try {
      parsedCommands = parseSceneCommands(compiled.stdout);
    } catch (error) {
      setStage("compile", "error", "Scene-command output was invalid");
      throw error;
    }
    if (!run.guestStreamedStreams.has("stdout")) {
      for (const line of parsedCommands.outputLines) appendDiagnostic(`[guest stdout] ${line.slice(0, 2000)}`);
    }
    if (compiled.stderr && !run.guestStreamedStreams.has("stderr")) {
      const clipped = compiled.stderr.length > 12000 ? `${compiled.stderr.slice(0, 12000)} … [fallback output truncated]` : compiled.stderr;
      appendDiagnostic(`[guest stderr] ${clipped}`);
    }

    if (run.controller.signal.aborted || activeRun !== run) return;
    setPhase("Starting Blender", "running");
    setStage("blender", "running", "Starting a fresh Blender runtime frame");
    runtimeNote.textContent = "Rust compiled. Starting a fresh Blender runtime and loading its local assets…";
    appendDiagnostic("[runtime] Starting a fresh Blender runtime and blr scene execution.");
    const result = await runBlenderScene(parsedCommands.json, options);
    if (activeRun !== run || run.controller.signal.aborted) return;
    if (!result || !result.scene) throw new Error("Blender runtime returned no scene geometry.");
    setStage("blender", "complete", "Blender returned scene geometry");
    appendDiagnostic("[runtime] Blender scene received. Preparing browser mesh preview.");
    setPhase("Preparing mesh preview", "running");
    setStage("preview", "running", "Building interactive GLB preview");
    if (!Array.isArray(result.scene.objects) || result.scene.objects.length === 0) {
      clearScene("Blender completed successfully and returned an empty scene. There is no mesh to preview.");
      hasRunResult = true;
      stats.objects.textContent = "0";
      stats.vertices.textContent = "0";
      stats.triangles.textContent = "0";
      stats.version.textContent = summarizeIdentity(result.identity);
      previewBadge.textContent = "Empty scene · run complete";
      setStage("preview", "complete", "Empty scene; no mesh preview to build");
      setPhase("Complete · empty scene", "ready");
      runtimeNote.textContent = "Run complete. Blender returned no mesh objects.";
      appendDiagnostic("[preview] Empty scene · 0 objects · no GLB artifact.");
      setRunButton("Run again");
      return;
    }
    const summary = await preview.show(result.scene);
    if (activeRun !== run || run.controller.signal.aborted) return;
    setSceneSummary(summary, result.identity);
    setStage("preview", "complete", "Interactive browser preview ready");
    setPhase("Complete", "ready");
    runtimeNote.textContent = currentPreview
      ? "Run complete. Rust compiled and Blender returned the displayed mesh."
      : "Blender returned the mesh, but this browser could not show the interactive preview. The run-specific GLB is available to download.";
    appendDiagnostic(`[preview] ${currentPreview ? "Interactive preview ready" : "Mesh ready; interactive preview unavailable"} · ${summary.objects} objects · ${summary.vertices} vertices · ${summary.triangles} triangles · Blender ${summarizeIdentity(result.identity)}.`);
    setRunButton("Run again");
  } catch (error) {
    if (activeRun !== run || run.controller.signal.aborted) return;
    const reason = errorText(error, "The run failed for an unknown reason.");
    if (stages.blender.dataset.state === "running") setStage("blender", "error", "Blender run failed");
    else if (stages.compile.dataset.state === "running") setStage("compile", "error", "Rust compile failed");
    else if (stages.preview.dataset.state === "running") setStage("preview", "error", "Mesh preview failed");
    clearScene(`No preview: ${reason}`);
    setPhase("Run failed", "error");
    runtimeNote.textContent = reason;
    appendDiagnostic(`[error] ${reason}`);
    setRunButton("Retry run");
  } finally {
    if (activeRun === run) {
      activeRun = null;
      cancelButton.hidden = true;
      editor.disabled = false;
      select.disabled = false;
      resetButton.disabled = false;
      runPanel.setAttribute("aria-busy", "false");
      setRunButton(phase.dataset.state === "error" ? "Retry run" : "Run again");
    }
  }
}

function cancelRun() {
  const run = activeRun;
  if (!run) return;
  activeRun = null;
  run.controller.abort();
  clearScene("Run canceled. You can edit the source and try again.");
  setPhase("Canceled", "idle");
  setStage("compile", "idle");
  setStage("blender", "idle");
  setStage("preview", "idle");
  appendDiagnostic("[cancel] Run canceled. Any late output from this run is ignored.");
  runtimeNote.textContent = "Run canceled. The runtime worker was asked to stop; retry is available.";
  runPanel.setAttribute("aria-busy", "false");
  cancelButton.hidden = true;
  editor.disabled = false;
  select.disabled = false;
  resetButton.disabled = false;
  setRunButton("Run again");
}

async function loadExamples() {
  const response = await fetch("./examples.json", { cache: "no-store", credentials: "same-origin" });
  if (!response.ok) throw new Error(`Examples could not load (HTTP ${response.status}).`);
  const data = await response.json();
  if (!data || data.schema !== "blr-rust-demo-examples-v1" || data.helperModule !== "blr_demo.rs"
    || !/^[a-f0-9]{64}$/.test(data.helperSha256 || "") || data.compilerTarget !== "wasm32-wasip1"
    || !Array.isArray(data.examples) || data.examples.length === 0 || data.examples.length > 12) {
    throw new Error("The examples manifest is invalid or targets an unsupported compiler.");
  }
  await verifyDemoHelper(data.helperSha256);
  const ids = new Set();
  examples = data.examples.map((example, index) => {
    if (!example || typeof example.id !== "string" || typeof example.title !== "string" || typeof example.source !== "string"
      || !example.id || example.id.length > 80 || !example.title || example.title.length > 120 || ids.has(example.id)) {
      throw new Error(`Example ${index + 1} has an invalid or duplicate id, title, or source.`);
    }
    if (!sourceWithinLimit(example.source)) throw new Error(`Example ${index + 1} exceeds the 256 KiB source limit.`);
    ids.add(example.id);
    return { id: example.id, title: example.title, description: typeof example.description === "string" ? example.description : "", source: example.source };
  });
  select.replaceChildren();
  for (const example of examples) {
    const option = document.createElement("option");
    option.value = example.id;
    option.textContent = example.title;
    select.append(option);
  }
  select.disabled = false;
  selectExample(examples[0].id);
}

async function verifyDemoHelper(expectedHash) {
  if (!crypto?.subtle) throw new Error("SHA-256 verification is unavailable in this browser.");
  const response = await fetch("./blr_demo.rs", { cache: "no-store", credentials: "same-origin" });
  if (!response.ok || !response.body) throw new Error(`The browser demo helper could not load (HTTP ${response.status}).`);
  const reader = response.body.getReader();
  const chunks = [];
  let size = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      size += value.byteLength;
      if (size > 128 * 1024) throw new Error("The browser demo helper exceeds the 128 KiB input limit.");
      chunks.push(value);
    }
  } catch (error) {
    await reader.cancel().catch(() => {});
    throw error;
  } finally {
    reader.releaseLock();
  }
  const bytes = new Uint8Array(size);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  const actualHash = Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
  if (actualHash !== expectedHash) throw new Error("The browser demo helper does not match the examples manifest.");
}

function checkIsolation() {
  if (typeof Worker === "undefined") throw new Error("This browser does not support workers required by the local compiler and Blender runtime.");
  if (!window.blrIsolationReady || typeof window.blrIsolationReady.then !== "function") {
    throw new Error("Browser isolation did not initialize. Reload the page and retry.");
  }
  return window.blrIsolationReady;
}

async function prepare() {
  try {
    runtimeNote.textContent = "Loading local compiler, Blender bridge, viewer modules, examples, and browser isolation…";
    const [compilerModule, blenderModule, viewerModule] = await Promise.all([
      import("./compiler.js"),
      import("./blender-runner.js"),
      import("./scene-viewer.js"),
    ]);
    if (typeof compilerModule.compileRust !== "function" || typeof blenderModule.runBlenderScene !== "function" || typeof viewerModule.createScenePreview !== "function") {
      throw new Error("A local playground module is missing its required entry point.");
    }
    compileRust = compilerModule.compileRust;
    runBlenderScene = blenderModule.runBlenderScene;
    createScenePreview = viewerModule.createScenePreview;
    preview = createScenePreview($("#preview-host"));
    const isolation = checkIsolation();
    await Promise.all([isolation, loadExamples()]);
    if (!window.crossOriginIsolated || typeof SharedArrayBuffer === "undefined") {
      throw new Error("This browser did not enable cross-origin isolation required by the local compiler and Blender runtime.");
    }
    ready = true;
    setPhase("Ready", "ready");
    runtimeNote.textContent = "Ready. Rust and Blender runtime assets download only after you press Run.";
    setRunButton("Run Rust + Blender");
  } catch (error) {
    ready = false;
    select.disabled = examples.length === 0;
    setPhase("Unavailable", "error");
    runtimeNote.textContent = errorText(error, "The playground could not finish preparing.");
    setRunButton("Playground unavailable", true);
    appendDiagnostic(`[setup error] ${runtimeNote.textContent}`);
  }
}

editor.addEventListener("input", () => {
  const hadResult = hasRunResult;
  markResultStale();
  const withinLimit = sourceWithinLimit(editor.value);
  if (withinLimit) updateGutter();
  else {
    gutter.textContent = "…";
    runtimeNote.textContent = "Source exceeds the 256 KiB editor limit. Reduce it before running.";
  }
  if (withinLimit && ready && !activeRun) {
    runtimeNote.textContent = hadResult
      ? "Source changed. The viewport still shows the previous run; run again to update it."
      : "Draft updated. Press Run Rust + Blender when ready.";
  }
  clearTimeout(draftTimer);
  draftTimer = window.setTimeout(saveDraft, 250);
});
window.addEventListener("pagehide", () => {
  clearTimeout(draftTimer);
  saveDraft();
});
editor.addEventListener("scroll", () => { gutter.scrollTop = editor.scrollTop; });
editor.addEventListener("keydown", (event) => {
  if (event.key === "Tab" && event.shiftKey && !event.ctrlKey && !event.metaKey && !event.altKey) {
    event.preventDefault();
    select.focus();
  } else if (event.key === "Tab" && !event.ctrlKey && !event.metaKey && !event.altKey) {
    event.preventDefault();
    const start = editor.selectionStart;
    const end = editor.selectionEnd;
    editor.setRangeText("    ", start, end, "end");
    editor.dispatchEvent(new Event("input", { bubbles: true }));
  } else if (event.key === "Escape") {
    event.preventDefault();
    runButton.focus();
  } else if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
    event.preventDefault();
    void runCurrentSource();
  }
});
editor.addEventListener("beforeinput", (event) => {
  if (!event.inputType.startsWith("insert") || typeof event.data !== "string") return;
  const next = `${editor.value.slice(0, editor.selectionStart)}${event.data}${editor.value.slice(editor.selectionEnd)}`;
  if (new TextEncoder().encode(next).byteLength > MAX_SOURCE_BYTES) {
    event.preventDefault();
    runtimeNote.textContent = "Source limit is 256 KiB. This edit was not inserted; remove other text first.";
  }
});
editor.addEventListener("paste", (event) => {
  const pasted = event.clipboardData?.getData("text/plain");
  if (typeof pasted !== "string") return;
  const next = `${editor.value.slice(0, editor.selectionStart)}${pasted}${editor.value.slice(editor.selectionEnd)}`;
  if (!sourceWithinLimit(next)) {
    event.preventDefault();
    runtimeNote.textContent = "Source limit is 256 KiB. This paste was not inserted; remove other text first.";
  }
});
select.addEventListener("change", () => selectExample(select.value));
runButton.addEventListener("click", () => { void runCurrentSource(); });
cancelButton.addEventListener("click", cancelRun);
resetButton.addEventListener("click", () => {
  if (!activeExample || activeRun) return;
  try { localStorage.removeItem(draftKey(activeExample.id)); } catch {}
  editor.value = activeExample.source;
  updateGutter();
  $("#draft-status").textContent = `Example reset · ${activeExample.title}`;
  markResultStale();
  if (!hasRunResult) runtimeNote.textContent = "Example source restored. Your other example drafts are unchanged.";
  editor.focus();
});

$("#orbit-left").addEventListener("click", () => preview.orbit(-0.3));
$("#orbit-right").addEventListener("click", () => preview.orbit(0.3));
$("#zoom-in").addEventListener("click", () => preview.zoom(1.2));
$("#zoom-out").addEventListener("click", () => preview.zoom(1 / 1.2));
$("#pan-left").addEventListener("click", () => preview.pan(-0.15, 0));
$("#pan-right").addEventListener("click", () => preview.pan(0.15, 0));
$("#preview-reset").addEventListener("click", () => preview.reset());
$("#download-scene").addEventListener("click", () => preview.download());

window.__blrPlayground = {
  get activeRun() { return activeRun?.id ?? null; },
  get ready() { return ready; },
  get guestBoundaryProof() { return lastGuestBoundaryProof; },
};
void prepare();
