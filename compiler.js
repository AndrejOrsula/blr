import { runWasiTool } from "./compiler/direct-host/index.js?blr_app=7e291aa70f255adf2d922cb97762e2d3457ad6d22b5d1d8ee2132da4497db313";
import { runGuest } from "./compiler/guest/runner.js?blr_app=7e291aa70f255adf2d922cb97762e2d3457ad6d22b5d1d8ee2132da4497db313";
import { notifyProgress } from "./compiler/asset-loading.js?blr_app=7e291aa70f255adf2d922cb97762e2d3457ad6d22b5d1d8ee2132da4497db313";
import { loadExampleInputs, loadPinnedInputs, PINNED_TOOLCHAIN } from "./compiler-inputs.js?blr_app=7e291aa70f255adf2d922cb97762e2d3457ad6d22b5d1d8ee2132da4497db313";

const MAX_SOURCE_BYTES = 256 * 1024;
const MAX_OUTPUT_CHARS = 512 * 1024;
const COMPILER_TIMEOUT_MS = 180_000;
const GUEST_TIMEOUT_MS = 10_000;
const MAX_GUEST_MODULE_BYTES = 16 * 1024 * 1024;
let cachedHelperLibrary;

function notifyStatus(onStatus, text) {
  try { onStatus?.(text); } catch {}
}

function emitOutput(onOutput, capture, record) {
  if (!record || (record.stream !== "stdout" && record.stream !== "stderr") || typeof record.text !== "string") return;
  const remaining = MAX_OUTPUT_CHARS - capture[record.stream].length;
  if (remaining > 0) capture[record.stream] += record.text.slice(0, remaining);
  try { onOutput?.(record); } catch {}
}

function abortError() {
  return new DOMException("Rust compilation or guest run aborted", "AbortError");
}

function assertSource(source, signal) {
  if (signal?.aborted) throw abortError();
  if (typeof source !== "string") throw new TypeError("Rust source must be a string.");
  const size = new TextEncoder().encode(source).byteLength;
  if (size > MAX_SOURCE_BYTES) throw new RangeError(`Rust source exceeds ${MAX_SOURCE_BYTES} bytes.`);
}

function isRustDiagnostic(output) {
  return /(?:^|\n)\s*(?:error(?:\[[A-Z0-9]+\])?:|aborting due to \d+ previous error)/m.test(output);
}

function compilerFailure(stdout, stderr, exitCode = 1) {
  return {
    ok: false,
    stage: "compile",
    exitCode,
    stdout,
    stderr,
    compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
  };
}

function helperLibraryKey(inputs) {
  const compilerIdentity = inputs.rustcSha256 || inputs.compilerSha256;
  if (!compilerIdentity || !inputs.helperSha256) return undefined;
  return [PINNED_TOOLCHAIN.compilerVersion, PINNED_TOOLCHAIN.target, compilerIdentity, inputs.helperSha256].join(":");
}

async function compileHelperLibrary(inputs, { signal, onProgress, onOutput }) {
  const key = helperLibraryKey(inputs);
  if (key && cachedHelperLibrary?.key === key) {
    for (const stream of ["stdout", "stderr"]) {
      if (cachedHelperLibrary[stream]) emitOutput(onOutput, { stdout: "", stderr: "" }, { stream, text: cachedHelperLibrary[stream] });
    }
    return cachedHelperLibrary;
  }
  const capture = { stdout: "", stderr: "" };
  let built;
  try {
    built = await runWasiTool({
      moduleBytes: inputs.rustcBytes,
      args: [
        "rustc", "/src/blr_demo.rs", "--sysroot", "/sysroot", "--target", PINNED_TOOLCHAIN.target,
        "--edition=2021", "--crate-name", "blr", "--crate-type=rlib", "-o", "/target/libblr.rlib",
      ],
      files: {
        ...inputs.sysrootFiles,
        "src/blr_demo.rs": inputs.helper,
      },
      signal,
      timeoutMs: COMPILER_TIMEOUT_MS,
      memory: { initialPages: 399, maximumPages: 16384 },
      onProgress,
      onOutput: (record) => emitOutput(onOutput, capture, record),
    });
  } catch (error) {
    if (signal?.aborted || error?.name === "AbortError") throw abortError();
    const diagnostic = capture.stderr + capture.stdout;
    if (isRustDiagnostic(diagnostic)) {
      return { exitCode: 1, stdout: capture.stdout, stderr: capture.stderr };
    }
    throw error;
  }
  if (built.exitCode !== 0) {
    return { exitCode: built.exitCode, stdout: built.stdout, stderr: built.stderr };
  }
  const library = built.files["/target/libblr.rlib"];
  if (!(library instanceof Uint8Array) || library.byteLength === 0) {
    return {
      exitCode: 1,
      stdout: built.stdout,
      stderr: `${built.stderr}\nRust compiler did not produce the expected browser helper library.`.trim(),
    };
  }
  if (signal?.aborted) throw abortError();
  const result = {
    exitCode: 0,
    key,
    bytes: library.slice(),
    stdout: built.stdout,
    stderr: built.stderr,
    rustcAssetSha256: inputs.rustcAssetSha256,
    helperSha256: inputs.helperSha256,
  };
  if (key) cachedHelperLibrary = result;
  return result;
}

/** Compile editable Rust locally to wasm32-wasip1, then run it in the separate bounded guest worker. */
export async function compileRust(source, { signal, onStatus, onProgress, onOutput, exampleInputs } = {}) {
  assertSource(source, signal);
  const compileCapture = { stdout: "", stderr: "" };
  notifyStatus(onStatus, "Loading pinned Rust compiler and wasm32-wasip1 standard library");
  const inputs = await loadPinnedInputs({ signal, onProgress, exampleInputs });
  if (signal?.aborted) throw abortError();
  notifyStatus(onStatus, "Preparing the browser blr helper library");
  notifyProgress(onProgress, { phase: "compile", asset: "blr-helper", state: "start", source: "work" });
  let helperLibrary;
  try {
    helperLibrary = await compileHelperLibrary(inputs, { signal, onProgress, onOutput });
  } catch (error) {
    const aborted = signal?.aborted || error?.name === "AbortError";
    notifyProgress(onProgress, {
      phase: "compile", asset: "blr-helper", state: aborted ? "cancelled" : "error", source: "work",
      error: aborted ? null : String(error?.message || error).slice(0, 1024),
    });
    if (aborted) throw abortError();
    const diagnostic = compileCapture.stderr + compileCapture.stdout;
    if (isRustDiagnostic(diagnostic)) {
      return compilerFailure(compileCapture.stdout, compileCapture.stderr);
    }
    throw error;
  }
  if (helperLibrary.exitCode !== 0) {
    notifyProgress(onProgress, { phase: "compile", asset: "blr-helper", state: "error", source: "work", error: null });
    return compilerFailure(helperLibrary.stdout, helperLibrary.stderr, helperLibrary.exitCode);
  }
  if (signal?.aborted) throw abortError();
  notifyProgress(onProgress, { phase: "compile", asset: "blr-helper", state: "complete", source: "work" });
  const files = {
    ...inputs.sysrootFiles,
    "src/main.rs": new TextEncoder().encode(source),
    "src/blr_demo.rs": inputs.helper,
    "target/libblr.rlib": helperLibrary.bytes,
  };
  notifyStatus(onStatus, "Compiling Rust for wasm32-wasip1");
  notifyProgress(onProgress, { phase: "compile", asset: "user-source", state: "start", source: "work" });
  let built;
  try {
    built = await runWasiTool({
      moduleBytes: inputs.rustcBytes,
      args: [
        "rustc", "/src/main.rs", "--sysroot", "/sysroot", "--target", PINNED_TOOLCHAIN.target,
        "--edition=2021",
        "--extern", "blr=/target/libblr.rlib",
        "-Clinker=wasm-ld", "-C", "link-arg=--max-memory=268435456", "-o", "/target/main.wasm",
      ],
      files,
      signal,
      timeoutMs: COMPILER_TIMEOUT_MS,
      memory: { initialPages: 399, maximumPages: 16384 },
      onProgress,
      onOutput: (record) => emitOutput(onOutput, compileCapture, record),
    });
  } catch (error) {
    const aborted = signal?.aborted || error?.name === "AbortError";
    notifyProgress(onProgress, {
      phase: "compile", asset: "user-source", state: aborted ? "cancelled" : "error", source: "work",
      error: aborted ? null : String(error?.message || error).slice(0, 1024),
    });
    if (aborted) throw abortError();
    const diagnostic = compileCapture.stderr + compileCapture.stdout;
    if (isRustDiagnostic(diagnostic)) {
      return {
        ok: false,
        stage: "compile",
        exitCode: 1,
        stdout: compileCapture.stdout,
        stderr: compileCapture.stderr,
        compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
      };
    }
    throw error;
  }
  if (built.exitCode !== 0) {
    notifyProgress(onProgress, { phase: "compile", asset: "user-source", state: "error", source: "work", error: null });
    return compilerFailure(built.stdout, built.stderr, built.exitCode);
  }
  const wasmBytes = built.files["/target/main.wasm"];
  if (!(wasmBytes instanceof Uint8Array) || wasmBytes.byteLength === 0) {
    notifyProgress(onProgress, { phase: "compile", asset: "user-source", state: "error", source: "work", error: "compiler produced no module" });
    return {
      ok: false,
      stage: "compile",
      exitCode: 1,
      stdout: built.stdout,
      stderr: `${built.stderr}\nRust compiler did not produce the expected Wasm module.`.trim(),
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
    };
  }
  if (wasmBytes.byteLength > MAX_GUEST_MODULE_BYTES) {
    notifyProgress(onProgress, { phase: "compile", asset: "user-source", state: "error", source: "work", error: "compiled guest exceeds module limit" });
    return {
      ok: false,
      stage: "compile",
      exitCode: 1,
      stdout: built.stdout,
      stderr: `Compiled guest exceeds ${MAX_GUEST_MODULE_BYTES} bytes.`,
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
    };
  }
  notifyProgress(onProgress, { phase: "compile", asset: "user-source", state: "complete", source: "work" });
  if (signal?.aborted) throw abortError();

  const guestCapture = { stdout: "", stderr: "" };
  notifyStatus(onStatus, "Running compiled Rust guest");
  notifyProgress(onProgress, { phase: "execute", asset: "rust-guest", state: "start", source: "work" });
  try {
    const guest = await runGuest(wasmBytes, {
      signal,
      timeoutMs: GUEST_TIMEOUT_MS,
      onOutput: (record) => emitOutput(onOutput, guestCapture, record),
    });
    notifyProgress(onProgress, { phase: "execute", asset: "rust-guest", state: "complete", source: "work" });
    return {
      ok: guest.exitCode === 0,
      stage: "run",
      exitCode: guest.exitCode,
      stdout: guest.stdout,
      stderr: guest.stderr,
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
      guestBoundary: guest.guestBoundary,
    };
  } catch (error) {
    const aborted = signal?.aborted || error?.name === "AbortError";
    notifyProgress(onProgress, {
      phase: "execute", asset: "rust-guest", state: aborted ? "cancelled" : "error", source: "work",
      error: aborted ? null : String(error?.message || error).slice(0, 1024),
    });
    if (aborted) throw abortError();
    return {
      ok: false,
      stage: "run",
      exitCode: 1,
      stdout: guestCapture.stdout,
      stderr: `${guestCapture.stderr}${guestCapture.stderr ? "\n" : ""}${String(error?.message || error).slice(0, 2048)}`,
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
      ...(error?.guestBoundary ? { guestBoundary: error.guestBoundary } : {}),
    };
  }
}

export { loadExampleInputs, PINNED_TOOLCHAIN };
