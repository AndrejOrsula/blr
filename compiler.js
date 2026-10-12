import { runWasiTool } from "./compiler/direct-host/index.js";
import { runGuest } from "./compiler/guest/runner.js";
import { loadPinnedInputs, PINNED_TOOLCHAIN } from "./compiler-inputs.js";

const MAX_SOURCE_BYTES = 256 * 1024;
const MAX_OUTPUT_CHARS = 512 * 1024;
const COMPILER_TIMEOUT_MS = 180_000;
const GUEST_TIMEOUT_MS = 10_000;
const MAX_GUEST_MODULE_BYTES = 16 * 1024 * 1024;

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

/** Compile editable Rust locally to wasm32-wasip1, then run it in the separate bounded guest worker. */
export async function compileRust(source, { signal, onStatus, onOutput } = {}) {
  assertSource(source, signal);
  const compileCapture = { stdout: "", stderr: "" };
  notifyStatus(onStatus, "Loading pinned Rust compiler and wasm32-wasip1 standard library");
  const inputs = await loadPinnedInputs({ signal });
  if (signal?.aborted) throw abortError();
  const files = {
    ...inputs.sysrootFiles,
    "src/main.rs": new TextEncoder().encode(source),
    "src/blr_demo.rs": inputs.helper,
  };
  notifyStatus(onStatus, "Compiling Rust for wasm32-wasip1");
  let built;
  try {
    built = await runWasiTool({
      moduleBytes: inputs.rustcBytes,
      args: [
        "rustc", "/src/main.rs", "--sysroot", "/sysroot", "--target", PINNED_TOOLCHAIN.target,
        "-Clinker=wasm-ld", "-C", "link-arg=--max-memory=268435456", "-o", "/target/main.wasm",
      ],
      files,
      signal,
      timeoutMs: COMPILER_TIMEOUT_MS,
      memory: { initialPages: 399, maximumPages: 16384 },
      onOutput: (record) => emitOutput(onOutput, compileCapture, record),
    });
  } catch (error) {
    if (signal?.aborted || error?.name === "AbortError") throw abortError();
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
    return {
      ok: false,
      stage: "compile",
      exitCode: built.exitCode,
      stdout: built.stdout,
      stderr: built.stderr,
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
    };
  }
  const wasmBytes = built.files["/target/main.wasm"];
  if (!(wasmBytes instanceof Uint8Array) || wasmBytes.byteLength === 0) {
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
    return {
      ok: false,
      stage: "compile",
      exitCode: 1,
      stdout: built.stdout,
      stderr: `Compiled guest exceeds ${MAX_GUEST_MODULE_BYTES} bytes.`,
      compilerVersion: PINNED_TOOLCHAIN.compilerVersion,
    };
  }
  if (signal?.aborted) throw abortError();

  const guestCapture = { stdout: "", stderr: "" };
  notifyStatus(onStatus, "Running compiled Rust guest");
  try {
    const guest = await runGuest(wasmBytes, {
      signal,
      timeoutMs: GUEST_TIMEOUT_MS,
      onOutput: (record) => emitOutput(onOutput, guestCapture, record),
    });
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
    if (signal?.aborted || error?.name === "AbortError") throw abortError();
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

export { PINNED_TOOLCHAIN };
