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
const guiHost = $("#blender-gui-host");
const glbHost = $("#glb-preview-host");
const modeHelp = $("#mode-help");
const startBlenderButton = $("#start-blender");
const snapshotGlbButton = $("#snapshot-glb");
const clearGeneratedButton = $("#clear-generated");
const closeBlenderButton = $("#close-blender");
const fullscreenBlenderButton = $("#fullscreen-blender");
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
let createBlenderGui;
let preview;
let blenderGui;

let examples = [];
let activeExample = null;
let activeRun = null;
let runSerial = 0;
let draftTimer = 0;
let ready = false;
let previewMode = "blender";
let guiAvailable = false;
let guiFailure = "";
let guiFatal = false;
let guiDocumentInvalidated = false;
let guiSnapshotPreviousDocument = false;
let guiFrameVisible = false;
let guiFrameWaiter = null;
let guiPendingCreateRequestId = null;
let guiCanceledCreateRequestId = null;
let guiLeaseToken = null;
let guiIdentity = null;
let guiScenePresent = false;
let guiSummary = null;
let guiSnapshotPresent = false;
let guiSnapshotFresh = false;
let glbIdentity = null;
let glbSummary = null;
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
  if (previewMode === "blender" && guiScenePresent) {
    previewBadge.textContent = "Blender scene · source changed";
    runtimeNote.textContent = "The editable Blender scene is unchanged. Run again to create another generated scene after clearing the current one.";
    return;
  }
  if (previewMode === "glb" && guiSnapshotPresent) {
    previewBadge.textContent = guiSnapshotFresh ? "GLB snapshot · current" : "GLB snapshot · stale";
    runtimeNote.textContent = "Source changed. The GLB shows the Blender scene at snapshot time; run again to preview the current source.";
    return;
  }
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
  glbSummary = summary;
  glbIdentity = identity;
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

function updateGuiActions() {
  const mounted = Boolean(blenderGui);
  const guiReady = mounted && blenderGui.ready === true && guiFrameVisible && !guiDocumentInvalidated;
  startBlenderButton.disabled = !ready || !guiAvailable || guiFatal || guiDocumentInvalidated || Boolean(activeRun) || guiReady || Boolean(blenderGui?.busy);
  startBlenderButton.textContent = guiReady ? "Blender initialized" : "Start Blender";
  snapshotGlbButton.disabled = !guiReady || !guiLeaseToken || Boolean(blenderGui?.busy) || Boolean(activeRun);
  clearGeneratedButton.disabled = !guiReady || !guiLeaseToken || Boolean(blenderGui?.busy) || Boolean(activeRun);
  closeBlenderButton.disabled = !mounted;
  fullscreenBlenderButton.disabled = !guiReady || Boolean(blenderGui?.busy) || Boolean(activeRun);
  fullscreenBlenderButton.textContent = document.fullscreenElement === guiHost ? "Exit fullscreen" : "Fullscreen editor";
  $("#mode-blender").disabled = (!guiAvailable && !mounted) || Boolean(activeRun);
  $("#mode-glb").disabled = Boolean(activeRun);
  $("#mode-blender").setAttribute("aria-pressed", String(previewMode === "blender"));
  $("#mode-glb").setAttribute("aria-pressed", String(previewMode === "glb"));
  if (previewMode === "blender") {
    guiHost.hidden = false;
    glbHost.hidden = true;
    modeHelp.textContent = guiDocumentInvalidated
      ? "Blender document changed; editor bridge is resuming. The previous document’s GLB snapshot is stale."
      : guiFatal
        ? `${guiFailure || "Blender runtime stopped safely."} Scene state is uncertain. The last validated GLB snapshot, if present, is stale.`
        : guiAvailable
        ? guiReady ? "Blender editor is ready. Run Rust to create generated objects; clear them before another run, or snapshot edits to those objects to GLB. Other Blender objects are not included." : "Start Blender to open the editable workspace; Run can also start it."
        : `Blender UI unavailable${guiFailure ? `: ${guiFailure}` : ". Choose GLB preview to use the preserved headless workflow."}`;
  } else {
    guiHost.hidden = true;
    glbHost.hidden = false;
    modeHelp.textContent = guiSnapshotPresent
      ? guiSnapshotFresh ? "GLB captured from the Rust-generated objects; other Blender objects are not included." : guiFatal ? "Last validated GLB snapshot; stale because Blender scene state is uncertain." : guiSnapshotPreviousDocument ? "This GLB snapshot belongs to the previous Blender document." : "This GLB snapshot is stale. Return to Blender and update it after editing the Rust-generated objects."
      : "Run Rust in GLB mode for the headless preview, or create a Rust-generated scene in the Blender editor and snapshot its objects.";
  }
}

function setPreviewMode(mode) {
  if (mode !== "blender" && mode !== "glb") return;
  if (mode === "blender" && !guiAvailable && !blenderGui) {
    modeHelp.textContent = `Blender UI unavailable${guiFailure ? `: ${guiFailure}` : ". Choose GLB preview to continue."}`;
    return;
  }
  if (mode === "blender" && previewMode === "glb" && guiScenePresent) {
    guiSnapshotFresh = false;
    if (guiSnapshotPresent) previewBadge.textContent = "GLB snapshot · stale after editing";
  }
  previewMode = mode;
  if (mode === "glb" && guiSnapshotPresent) {
    previewBadge.textContent = guiSnapshotFresh ? "GLB snapshot · current" : "GLB snapshot · stale";
  }
  if (mode === "blender" && guiScenePresent && guiSummary) {
    displayGeometrySummary(guiSummary, guiIdentity);
    previewBadge.textContent = "Blender editor · scene ready";
    viewportEmpty.hidden = true;
    currentPreview = false;
    enablePreviewControls(false, Boolean(preview?.artifact()));
  } else if (mode === "glb" && (guiSnapshotPresent || glbSummary)) {
    displayGeometrySummary(glbSummary, glbIdentity);
    viewportEmpty.hidden = true;
  } else if (mode === "glb" && guiScenePresent) {
    viewportEmpty.hidden = false;
    viewportMessage.textContent = "No GLB snapshot yet. Choose Snapshot / Update GLB to capture edits to the Rust-generated objects. Other Blender objects are not included.";
    previewBadge.textContent = "No GLB snapshot";
    currentPreview = false;
    enablePreviewControls(false, false);
  } else if (mode === "glb" && !glbSummary) {
    viewportEmpty.hidden = false;
    viewportMessage.textContent = "No GLB preview yet. Run Rust in GLB mode or start Blender and take a scene snapshot.";
    currentPreview = false;
    enablePreviewControls(false, false);
  } else if (mode === "blender") {
    currentPreview = false;
    enablePreviewControls(false, Boolean(preview?.artifact()));
    viewportEmpty.hidden = guiFrameVisible;
    if (guiFrameVisible) previewBadge.textContent = guiFatal ? "Blender scene state uncertain · editor retained" : "Blender editor ready · no generated scene";
    for (const cell of Object.values(stats)) cell.textContent = "—";
  }
  updateGuiActions();
}

function updateGuiState(state) {
  if (state && typeof state === "object") {
    if (Object.hasOwn(state, "leaseToken")) guiLeaseToken = typeof state.leaseToken === "string" ? state.leaseToken : null;
    else if (typeof blenderGui?.lease === "string") guiLeaseToken = blenderGui.lease;
    if (state.visibleFrame === true || state.state === "frame-available") {
      guiFrameVisible = true;
      guiFrameWaiter?.resolve();
      guiFrameWaiter = null;
    }
    if (state.state === "busy" && Number.isSafeInteger(state.requestId) && activeRun && previewMode === "blender") {
      guiPendingCreateRequestId = state.requestId;
    }
    if (state.state === "invalidated" && state.fatal !== true) {
      guiDocumentInvalidated = true;
      guiSnapshotPreviousDocument = guiSnapshotPresent || Boolean(state.lastSnapshot);
      guiAvailable = true;
      guiFailure = "";
      guiLeaseToken = null;
      guiPendingCreateRequestId = null;
      guiCanceledCreateRequestId = null;
      guiScenePresent = false;
      guiSummary = null;
      if (state.lastSnapshotStale || guiSnapshotPresent) guiSnapshotFresh = false;
      if (guiSnapshotPresent) previewBadge.textContent = "GLB snapshot · previous document";
      runtimeNote.textContent = "Blender document changed. Waiting for the editor bridge to resume; the previous document’s GLB snapshot remains stale.";
      appendDiagnostic("[Blender UI] Document changed; previous generated-scene lease cleared. Waiting for bridge recovery.");
    } else if ((state.state === "document-ready" || state.state === "ready") && state.ready === true && guiDocumentInvalidated && state.fatal !== true) {
      guiDocumentInvalidated = false;
      guiAvailable = true;
      guiFailure = "";
      runtimeNote.textContent = "Blender editor bridge resumed after the document change. Run Rust to create a new generated scene.";
      appendDiagnostic("[Blender UI] Editor bridge resumed after document load.");
      if (state.lastSnapshot?.source === "create" && !state.lastSnapshotStale) syncActiveGuiScene(state.lastSnapshot);
    } else if (state.fatal === true || state.state === "fatal" || state.state === "poisoned" || state.state === "failed") {
      guiDocumentInvalidated = false;
      guiFrameWaiter?.reject(new Error(state.message || "The Blender GUI runtime became unavailable."));
      guiFrameWaiter = null;
      guiAvailable = false;
      guiFailure = state.message || "The Blender editor runtime became unavailable.";
      guiFatal = state.fatal === true || state.state === "fatal" || state.state === "poisoned";
      if (state.visibleFrame === true || blenderGui?.visibleFrame === true) guiFrameVisible = true;
      guiLeaseToken = null;
      if (guiFatal) {
        guiScenePresent = false;
        guiSummary = null;
      }
      if (guiSnapshotPresent || state.lastSnapshotStale) guiSnapshotFresh = false;
      if (state.lastSnapshot?.scene) void showLastValidatedSnapshot(state.lastSnapshot);
      runtimeNote.textContent = guiFatal
        ? `${guiFailure} Blender scene state is uncertain. The editor remains open; close it explicitly to discard its contents.${guiSnapshotPresent || state.lastSnapshot?.scene ? " The last validated GLB snapshot remains available and is stale." : " No GLB snapshot is available."}`
        : `${guiFailure} The Blender editor remains available for inspection; switch to GLB preview for the preserved headless path.`;
      appendDiagnostic(`[Blender UI] ${guiFailure}`);
    }
    if (state.state === "ready" && state.ready === true && state.leaseToken && state.lastSnapshot?.source === "create" && !state.lastSnapshotStale) {
      const afterCancel = state.requestId === guiCanceledCreateRequestId;
      syncActiveGuiScene(state.lastSnapshot, { afterCancel });
      if (state.requestId === guiPendingCreateRequestId) guiPendingCreateRequestId = null;
      if (afterCancel) guiCanceledCreateRequestId = null;
    }
    if (state.state === "disposed") {
      guiFrameWaiter?.reject(new Error("The Blender editor was closed before its canvas became available."));
      guiFrameWaiter = null;
      guiAvailable = false;
      guiLeaseToken = null;
      guiScenePresent = false;
      guiSummary = null;
      guiSnapshotFresh = false;
      guiFrameVisible = false;
    }
  }
  updateGuiActions();
}

function syncActiveGuiScene(snapshot, { afterCancel = false } = {}) {
  if (!snapshot || typeof snapshot.leaseToken !== "string" || snapshot.leaseToken !== guiLeaseToken) return;
  try {
    const summary = summarizeScene(snapshot.scene);
    guiScenePresent = true;
    guiSummary = summary;
    guiIdentity = snapshot.identity ?? guiIdentity;
    if (guiSnapshotPresent) guiSnapshotFresh = false;
    if (previewMode === "blender") {
      displayGeometrySummary(summary, guiIdentity);
      previewBadge.textContent = "Blender editor · scene ready";
      viewportEmpty.hidden = true;
      runtimeNote.textContent = afterCancel
        ? "A Rust scene completed in Blender after its run was canceled. It is the active generated scene; clear generated objects before replacing it."
        : "The active Rust scene is ready in Blender. Clear generated objects before replacing it.";
    }
    appendDiagnostic(`[Blender UI] Active generated scene confirmed from transport state · ${summary.objects} objects · ${summary.vertices} vertices · ${summary.triangles} triangles.`);
  } catch (error) {
    appendDiagnostic(`[Blender UI state error] ${errorText(error, "Could not summarize the active Blender scene.")}`);
  }
}

async function showLastValidatedSnapshot(snapshot) {
  try {
    const summary = await preview.show(snapshot.scene);
    guiSnapshotPresent = true;
    guiSnapshotFresh = false;
    guiIdentity = snapshot.identity ?? guiIdentity;
    glbIdentity = snapshot.identity ?? glbIdentity;
    glbSummary = summary;
    displayGeometrySummary(summary, glbIdentity);
    if (previewMode === "glb") {
      viewportEmpty.hidden = true;
      previewBadge.textContent = "Last validated GLB snapshot · stale";
    }
    updateGuiActions();
  } catch (error) {
    appendDiagnostic(`[Blender UI snapshot fallback error] ${errorText(error, "Could not display the last validated snapshot.")}`);
  }
}

function summarizeScene(scene) {
  if (!scene || scene.schema !== "blr-scene-v1" || !Array.isArray(scene.objects)) {
    throw new Error("Blender returned no valid blr-scene-v1 geometry.");
  }
  return {
    objects: scene.objects.length,
    vertices: scene.objects.reduce((total, object) => total + object.positions.length, 0),
    triangles: scene.objects.reduce((total, object) => total + object.triangles.length, 0),
  };
}

function displayGeometrySummary(summary, identity) {
  if (!summary) return;
  stats.objects.textContent = String(summary.objects);
  stats.vertices.textContent = String(summary.vertices);
  stats.triangles.textContent = String(summary.triangles);
  stats.version.textContent = summarizeIdentity(identity);
  hasRunResult = true;
}

async function ensureBlenderGui({ signal } = {}) {
  if (signal?.aborted) throw new DOMException("Blender GUI startup canceled", "AbortError");
  if (!guiAvailable || typeof createBlenderGui !== "function") {
    throw new Error(guiFailure || "The actual Blender browser editor is unavailable. Choose GLB preview for the preserved headless workflow.");
  }
  if (!blenderGui) {
    blenderGui = createBlenderGui({
      host: guiHost,
      onStatus: (message) => {
        if (typeof message === "string") runtimeNote.textContent = message.slice(0, 220);
      },
      onOutput: (record) => {
        if (record && typeof record.text === "string") {
          const stream = record.stream === "stderr" ? "stderr" : "stdout";
          appendDiagnostic(`[Blender UI ${stream}] ${record.text.slice(0, 4000).replace(/[\r\n]+$/, "")}`);
        }
      },
      onState: updateGuiState,
    });
  }
  if (!blenderGui.ready) await blenderGui.start();
  if (signal?.aborted) throw new DOMException("Blender GUI startup canceled", "AbortError");
  if (!blenderGui.ready) throw new Error("Blender did not report an initialized editor runtime.");
  guiLeaseToken = blenderGui.lease ?? guiLeaseToken;
  if (blenderGui.visibleFrame === true) guiFrameVisible = true;
  if (!guiFrameVisible) await waitForGuiFrame(signal);
  updateGuiActions();
  return blenderGui;
}

function waitForGuiFrame(signal) {
  if (guiFrameVisible || blenderGui?.visibleFrame === true) return Promise.resolve();
  return new Promise((resolve, reject) => {
    let timer;
    const finish = (error) => {
      clearTimeout(timer);
      signal?.removeEventListener("abort", onAbort);
      guiFrameWaiter = null;
      if (error) reject(error);
      else resolve();
    };
    const onAbort = () => finish(new DOMException("Blender GUI startup canceled", "AbortError"));
    guiFrameWaiter = { resolve: () => finish(), reject: (error) => finish(error) };
    timer = window.setTimeout(() => finish(new Error("Blender initialized, but its browser canvas did not become available within 30 seconds.")), 30_000);
    if (signal?.aborted) return onAbort();
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}

async function startBlender() {
  if (!ready || activeRun || !guiAvailable) return;
  startBlenderButton.disabled = true;
  setPreviewMode("blender");
  runtimeNote.textContent = "Starting the actual Blender 5.2 workspace…";
  setPhase("Starting Blender editor", "running");
  try {
    await ensureBlenderGui();
    viewportEmpty.hidden = true;
    setPhase("Blender editor ready", "ready");
    runtimeNote.textContent = "Blender editor is ready. Run Rust to create a generated scene. Clear generated objects before another run, or snapshot edits to those objects to GLB. Other Blender objects are not included.";
    appendDiagnostic("[Blender UI] Workspace initialized. No scene objects were changed.");
  } catch (error) {
    guiFailure = errorText(error, "Blender editor initialization failed.");
    guiAvailable = false;
    setPreviewMode("glb");
    setPhase("Blender editor unavailable", "error");
    runtimeNote.textContent = `${guiFailure} Choose GLB preview to run through the preserved headless path.`;
    appendDiagnostic(`[Blender UI error] ${guiFailure}`);
  } finally {
    updateGuiActions();
  }
}

async function snapshotBlenderScene() {
  if (!blenderGui?.ready || !guiLeaseToken || blenderGui.busy || activeRun) return;
  const controller = new AbortController();
  snapshotGlbButton.disabled = true;
  runtimeNote.textContent = "Capturing the current editable Blender scene for GLB preview…";
  try {
    const result = await blenderGui.snapshot({ signal: controller.signal });
    if (!result || result.leaseToken !== guiLeaseToken) throw new Error("Blender snapshot did not match the active generated-scene lease.");
    setPreviewMode("glb");
    const summary = await preview.show(result.scene);
    guiIdentity = result.identity;
    glbIdentity = result.identity;
    glbSummary = summary;
    guiSnapshotPresent = true;
    guiSnapshotFresh = true;
    guiSnapshotPreviousDocument = false;
    hasRunResult = true;
    displayGeometrySummary(summary, guiIdentity);
    currentPreview = $("#preview-host model-viewer") !== null;
    enablePreviewControls(currentPreview, Boolean(preview.artifact()));
    previewBadge.textContent = previewMode === "glb" ? "GLB snapshot · current" : "Rust scene · GLB snapshot updated";
    viewportEmpty.hidden = true;
    runtimeNote.textContent = `GLB snapshot updated from Rust-generated Blender objects. Other Blender objects are not included. ${summary.objects} objects, ${summary.vertices} vertices, ${summary.triangles} triangles.`;
    appendDiagnostic(`[snapshot] Rust-generated Blender objects copied to GLB preview · ${summary.objects} objects · ${summary.vertices} vertices · ${summary.triangles} triangles.`);
    updateGuiActions();
  } catch (error) {
    const reason = errorText(error, "Blender snapshot failed.");
    runtimeNote.textContent = guiFatal
      ? `${reason} Blender scene state is uncertain; close the editor explicitly to discard it.`
      : `${reason} The Blender scene remains open and editable.`;
    appendDiagnostic(`[snapshot error] ${reason}`);
  } finally {
    updateGuiActions();
  }
}

async function clearGeneratedObjects() {
  if (!blenderGui?.ready || !guiLeaseToken || blenderGui.busy || activeRun) return;
  clearGeneratedButton.disabled = true;
  runtimeNote.textContent = "Removing only objects from the active Rust scene lease…";
  try {
    const clearingLease = guiLeaseToken;
    const result = await blenderGui.reset({});
    if (!result?.reset || result.leaseToken !== clearingLease) throw new Error("Blender did not confirm reset of the active generated-scene lease.");
    guiLeaseToken = null;
    guiScenePresent = false;
    guiSummary = null;
    guiIdentity = result.identity ?? guiIdentity;
    if (guiSnapshotPresent) guiSnapshotFresh = false;
    if (previewMode === "blender") {
      for (const cell of Object.values(stats)) cell.textContent = "—";
      hasRunResult = false;
    }
    previewBadge.textContent = guiSnapshotPresent ? "GLB snapshot · stale after reset" : "Generated objects cleared";
    runtimeNote.textContent = `Cleared ${result.removed ?? 0} generated object${result.removed === 1 ? "" : "s"}. User-created Blender objects remain.`;
    appendDiagnostic(`[Blender UI] Cleared generated objects · removed ${result.removed ?? 0} · missing ${result.missing ?? 0}.`);
    updateGuiActions();
  } catch (error) {
    const reason = errorText(error, "Could not clear generated Blender objects.");
    runtimeNote.textContent = guiFatal
      ? `${reason} Blender scene state is uncertain; the editor remains open until explicitly closed.`
      : `${reason} Existing Blender contents remain available for inspection.`;
    appendDiagnostic(`[Blender UI reset error] ${reason}`);
  } finally {
    updateGuiActions();
  }
}

function closeBlenderEditor() {
  if (!blenderGui) return;
  if (!window.confirm("Close the Blender editor? Unsaved Blender edits will be lost. This cannot be undone.")) return;
  if (activeRun) cancelRun();
  blenderGui.dispose();
  blenderGui = null;
  guiAvailable = typeof createBlenderGui === "function";
  guiLeaseToken = null;
  guiScenePresent = false;
  guiSummary = null;
  guiSnapshotFresh = false;
  guiFatal = false;
  guiDocumentInvalidated = false;
  previewBadge.textContent = guiSnapshotPresent ? "GLB snapshot · stale after editor closed" : "Blender editor closed";
  runtimeNote.textContent = "Blender editor closed. Its unsaved edits are gone; the GLB snapshot remains available if one exists.";
  appendDiagnostic("[Blender UI] Editor closed after explicit confirmation.");
  updateGuiActions();
}

async function toggleBlenderFullscreen() {
  if (!blenderGui?.ready || !guiFrameVisible || activeRun || blenderGui.busy) return;
  setPreviewMode("blender");
  try {
    if (document.fullscreenElement === guiHost) {
      await document.exitFullscreen();
    } else if (typeof guiHost.requestFullscreen === "function") {
      await guiHost.requestFullscreen();
    } else {
      throw new Error("Fullscreen is unavailable in this browser.");
    }
  } catch (error) {
    runtimeNote.textContent = errorText(error, "Could not change Blender fullscreen mode.");
  } finally {
    updateGuiActions();
  }
}

function clearScene(message = "Your Blender-created mesh will appear here.") {
  preview.clear();
  glbSummary = null;
  glbIdentity = null;
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
  if (previewMode === "blender" && guiDocumentInvalidated) {
    setPhase("Blender document changed", "running");
    runtimeNote.textContent = "Waiting for the Blender editor bridge to resume after document load before creating a new scene.";
    return;
  }
  if (previewMode === "blender" && !guiAvailable) {
    setPhase("Blender editor unavailable", "error");
    runtimeNote.textContent = `${guiFailure || "The actual Blender editor is unavailable."} Choose GLB preview for the preserved headless Blender-to-GLB workflow.`;
    appendDiagnostic(`[Blender UI unavailable] ${runtimeNote.textContent}`);
    return;
  }
  if (previewMode === "blender" && guiLeaseToken) {
    setPhase("Generated scene already exists", "ready");
    runtimeNote.textContent = "The active Rust-created scene remains editable. Use ‘Clear generated objects’ explicitly before creating a replacement; user-created Blender objects are preserved.";
    clearGeneratedButton.focus();
    return;
  }
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
  updateGuiActions();
  if (previewMode === "glb") clearScene("Running new source. Previous GLB preview cleared.");
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
    setPhase(previewMode === "blender" ? "Starting Blender editor" : "Starting headless Blender", "running");
    setStage("blender", "running", previewMode === "blender" ? "Creating scene in persistent Blender editor" : "Starting isolated headless Blender frame");
    runtimeNote.textContent = previewMode === "blender"
      ? "Rust compiled. Sending bounded scene commands to the persistent Blender editor…"
      : "Rust compiled. Starting the preserved isolated headless Blender scene runtime…";
    appendDiagnostic(previewMode === "blender"
      ? "[runtime] Creating the Rust scene in the persistent Blender GUI."
      : "[runtime] Starting isolated headless Blender scene execution.");
    let result;
    if (previewMode === "blender") {
      const gui = await ensureBlenderGui({ signal: run.controller.signal });
      result = await gui.createScene(parsedCommands.json, { signal: run.controller.signal });
    } else {
      result = await runBlenderScene(parsedCommands.json, options);
    }
    if (activeRun !== run || run.controller.signal.aborted) return;
    if (!result || !result.scene) throw new Error("Blender runtime returned no scene geometry.");
    setStage("blender", "complete", "Blender returned scene geometry");
    if (previewMode === "blender") {
      if (typeof result.leaseToken !== "string" || !result.leaseToken || !result.identity) {
        throw new Error("Blender GUI did not return its validated scene lease and runtime identity.");
      }
      const summary = summarizeScene(result.scene);
      guiLeaseToken = result.leaseToken;
      guiIdentity = result.identity;
      guiScenePresent = true;
      guiSummary = summary;
      if (guiSnapshotPresent) guiSnapshotFresh = false;
      viewportEmpty.hidden = true;
      stats.objects.textContent = String(summary.objects);
      stats.vertices.textContent = String(summary.vertices);
      stats.triangles.textContent = String(summary.triangles);
      stats.version.textContent = summarizeIdentity(guiIdentity);
      hasRunResult = true;
      currentPreview = false;
      previewBadge.textContent = "Blender editor · scene ready";
      setStage("preview", "complete", "Editable scene in the actual Blender workspace");
      setPhase("Complete · editable Blender scene", "ready");
      runtimeNote.textContent = `Scene is live in Blender 5.2 and remains editable. ${summary.objects} objects, ${summary.vertices} vertices, ${summary.triangles} triangles. Snapshot explicitly to update GLB.`;
      appendDiagnostic(`[Blender UI] Editable scene ready · ${summary.objects} objects · ${summary.vertices} vertices · ${summary.triangles} triangles · Blender ${summarizeIdentity(result.identity)}.`);
      updateGuiActions();
      setRunButton("Run in Blender");
      return;
    }
    appendDiagnostic("[runtime] Blender scene received. Preparing browser mesh preview.");
    setPhase("Preparing mesh preview", "running");
    setStage("preview", "running", "Building interactive GLB preview");
    if (!Array.isArray(result.scene.objects) || result.scene.objects.length === 0) {
      clearScene("Blender completed successfully and returned an empty scene. There is no mesh to preview.");
      hasRunResult = true;
      glbSummary = { objects: 0, vertices: 0, triangles: 0 };
      glbIdentity = result.identity;
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
    setStage("preview", "complete", currentPreview ? "Interactive browser preview ready" : "Viewer unavailable; GLB download ready");
    setPhase(currentPreview ? "Complete" : "Complete · download available", "ready");
    runtimeNote.textContent = currentPreview
      ? "Run complete. Rust compiled and Blender returned the displayed mesh."
      : "Blender returned the mesh, but this browser could not show the interactive preview. The run-specific GLB is available to download.";
    appendDiagnostic(`[preview] ${currentPreview ? "Interactive preview ready" : "Mesh ready; interactive preview unavailable"} · ${summary.objects} objects · ${summary.vertices} vertices · ${summary.triangles} triangles · Blender ${summarizeIdentity(result.identity)}.`);
    setRunButton("Run again");
  } catch (error) {
    if (activeRun !== run || run.controller.signal.aborted) return;
    const reason = errorText(error, "The run failed for an unknown reason.");
    const guiOperationFailed = previewMode === "blender" && stages.blender.dataset.state === "running";
    if (stages.blender.dataset.state === "running") setStage("blender", "error", "Blender run failed");
    else if (stages.compile.dataset.state === "running") setStage("compile", "error", "Rust compile failed");
    else if (stages.preview.dataset.state === "running") setStage("preview", "error", "Mesh preview failed");
    if (previewMode === "glb") clearScene(`No preview: ${reason}`);
    setPhase("Run failed", "error");
    runtimeNote.textContent = guiOperationFailed
      ? `${reason} The Blender workspace remains open. Choose GLB preview to use the preserved headless path.`
      : reason;
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
      setRunButton(phase.dataset.state === "error" ? "Retry run" : previewMode === "blender" ? "Run in Blender" : "Run Rust + Blender");
      updateGuiActions();
    }
  }
}

function cancelRun() {
  const run = activeRun;
  if (!run) return;
  if (guiPendingCreateRequestId !== null) guiCanceledCreateRequestId = guiPendingCreateRequestId;
  activeRun = null;
  run.controller.abort();
  if (previewMode === "glb") clearScene("Run canceled. You can edit the source and try again.");
  setPhase("Canceled", "idle");
  setStage("compile", "idle");
  setStage("blender", "idle");
  setStage("preview", "idle");
  appendDiagnostic("[cancel] Run canceled. Any late output from this run is ignored.");
  runtimeNote.textContent = previewMode === "blender"
    ? "Run canceled. Existing Blender edits remain in the open workspace; retry is available."
    : "Run canceled. The headless runtime worker was asked to stop; retry is available.";
  runPanel.setAttribute("aria-busy", "false");
  cancelButton.hidden = true;
  editor.disabled = false;
  select.disabled = false;
  resetButton.disabled = false;
  setRunButton(previewMode === "blender" ? "Run in Blender" : "Run Rust + Blender");
  updateGuiActions();
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
    runtimeNote.textContent = "Loading the Rust compiler, Blender scene helpers, and browser isolation…";
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
    const previewHost = $("#preview-host");
    if (glbHost.parentNode !== previewHost) previewHost.append(glbHost);
    preview = createScenePreview(glbHost);
    try {
      const guiModule = await import("./gui-transport.js");
      if (typeof guiModule.createBlenderGui !== "function") throw new Error("GUI transport is missing createBlenderGui.");
      createBlenderGui = guiModule.createBlenderGui;
      guiAvailable = true;
      guiFailure = "";
    } catch (error) {
      guiAvailable = false;
      guiFailure = errorText(error, "The Blender GUI transport could not be loaded.");
      previewMode = "glb";
    }
    const isolation = checkIsolation();
    await Promise.all([isolation, loadExamples()]);
    if (!window.crossOriginIsolated || typeof SharedArrayBuffer === "undefined") {
      throw new Error("This browser did not enable cross-origin isolation required by the local compiler and Blender runtime.");
    }
    ready = true;
    updateGuiActions();
    setPhase("Ready", "ready");
    runtimeNote.textContent = guiAvailable
      ? "Ready. Choose Blender editor or GLB preview. Runtime assets load only when you start Blender or run Rust."
      : `Blender editor unavailable: ${guiFailure} Choose GLB preview for the preserved headless workflow.`;
    if (!guiAvailable) setPreviewMode("glb");
    setRunButton(previewMode === "blender" ? "Run in Blender" : "Run Rust + Blender");
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
$("#mode-blender").addEventListener("click", () => setPreviewMode("blender"));
$("#mode-glb").addEventListener("click", () => setPreviewMode("glb"));
startBlenderButton.addEventListener("click", () => { void startBlender(); });
snapshotGlbButton.addEventListener("click", () => { void snapshotBlenderScene(); });
clearGeneratedButton.addEventListener("click", () => { void clearGeneratedObjects(); });
closeBlenderButton.addEventListener("click", closeBlenderEditor);
fullscreenBlenderButton.addEventListener("click", () => { void toggleBlenderFullscreen(); });
document.addEventListener?.("fullscreenchange", updateGuiActions);
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
