import { notifyProgress, readBoundedJson, readBoundedResponse, runWithAssetLoadBounds } from "./compiler/asset-loading.js?blr_app=543af08c9c9d6446694684fd0182efa61b8269029917ff4e95fd0f76dfb5ab59";

const MAX_HELPER_BYTES = 128 * 1024;
const MAX_AUTHORING_LIBRARY_BYTES = 16 * 1024 * 1024;
const MAX_COMPILER_MANIFEST_BYTES = 8 * 1024;
const MAX_EXAMPLES_MANIFEST_BYTES = 32 * 1024;
const MAX_COMPRESSED_BYTES = 24 * 1024 * 1024;
const MAX_SYSROOT_TAR_BYTES = 80 * 1024 * 1024;
const DECODER_WASM_BYTES = 206243;
const DECODER_WASM_SHA256 = "7cb4663fd0b11185b88177d0b3b3048136572faf49e81c4c78dccafe37bb68d7";

const pins = new Map([
  ["compiler/assets/rustc_opt.wasm.br", {
    bytes: 19774341,
    sha256: "10f06919521c61f177b432ae7fd08cc3d4547efc3b13c2f547560fb53cf05ca6",
    rawBytes: 95343874,
    rawSha256: "bfa0423877e4f9821f728524aaa8c0eec252cf6aec7ce1fafaf281fc063a0f67",
  }],
  ["compiler/assets/wasm32-wasip1.tar.br", {
    bytes: 18933366,
    sha256: "bf29ba33cbf6267c10a1c1838edfff76d19750287159adfa08ab308db922bfdb",
    rawBytes: 74096640,
    rawSha256: "c177ce2fed463d9bee1c50808f210f5a255e604064d0950e6dfd21e2f5018881",
  }],
  ["compiler/brotli-dec-wasm/pkg/brotli_dec_wasm_bg.wasm", {
    bytes: DECODER_WASM_BYTES,
    sha256: DECODER_WASM_SHA256,
  }],
]);

function abortError(message = "Compiler asset loading aborted") {
  return new DOMException(message, "AbortError");
}

function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function digest(bytes) {
  return hex(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)));
}

function checkAbort(signal) {
  if (signal?.aborted) throw abortError();
}

async function fetchBytes(url, { signal, limit, label, onProgress, asset, totalBytes, cache = "force-cache" }) {
  const progress = { phase: "fetch", asset, totalBytes: null, source: "network-or-cache" };
  let response;
  try {
    response = await fetch(url, { cache, credentials: "same-origin", signal });
  } catch (error) {
    notifyProgress(onProgress, {
      ...progress,
      state: isAbort(error, signal) ? "cancelled" : "error",
      loadedBytes: 0,
      error: isAbort(error, signal) ? null : errorMessage(error),
    });
    throw error;
  }
  const contentEncoding = response.headers?.get?.("content-encoding")?.trim().toLowerCase();
  // Fetch streams expose decoded response-body bytes. A pinned length is a
  // useful denominator only when the server did not apply HTTP content coding.
  progress.totalBytes = !contentEncoding || contentEncoding === "identity" ? totalBytes ?? null : null;
  return readBoundedResponse(response, { signal, limit, label, onProgress, progress });
}

function errorMessage(error) {
  return error instanceof Error ? error.message : String(error);
}

function isAbort(error, signal) {
  return signal?.aborted || error?.name === "AbortError";
}

function verifiedStage(onProgress, asset, state, loadedBytes, totalBytes, error) {
  notifyProgress(onProgress, {
    phase: "verify", asset, state, loadedBytes, totalBytes,
    source: "work", error: error ?? null,
  });
}

function workStage(onProgress, phase, asset, state, loadedBytes = null, totalBytes = null, error) {
  notifyProgress(onProgress, {
    phase, asset, state, loadedBytes, totalBytes,
    source: "work", error: error ?? null,
  });
}

function withAbort(promise, signal, worker, onProgress, asset) {
  if (signal?.aborted) {
    worker.terminate();
    return Promise.reject(abortError());
  }
  return new Promise((resolve, reject) => {
    let settled = false;
    let initialized = false;
    let activePhase = null;
    let activeAsset = "brotli-decoder";
    let compressedBuffer;
    const beginDecode = () => {
      if (!initialized || !compressedBuffer || settled) return;
      const buffer = compressedBuffer;
      compressedBuffer = undefined;
      worker.postMessage({ type: "decode", buffer }, [buffer]);
    };
    const finish = (callback, value) => {
      if (settled) return;
      settled = true;
      signal?.removeEventListener("abort", onAbort);
      worker.removeEventListener("message", onMessage);
      worker.removeEventListener("error", onError);
      worker.terminate();
      callback(value);
    };
    const onAbort = () => {
      if (activePhase) {
        notifyProgress(onProgress, {
          phase: activePhase, asset: activeAsset, state: "cancelled", loadedBytes: null,
          totalBytes: null, source: "work", error: null,
        });
      }
      finish(reject, abortError());
    };
    const onError = (event) => {
      const error = new Error(`Brotli decoder worker failed: ${event.message || "unknown worker error"}`);
      notifyProgress(onProgress, {
        phase: "decode", asset, state: "error", loadedBytes: null,
        totalBytes: null, source: "work", error: error.message,
      });
      finish(reject, error);
    };
    const onMessage = (event) => {
      const message = event.data;
      if (message?.type === "progress") {
        if (settled || signal?.aborted) return;
        if (message.state === "start" || message.state === "update") {
          activePhase = message.phase;
          activeAsset = message.asset || "brotli-decoder";
        } else if (message.state === "complete" || message.state === "error" || message.state === "cancelled") {
          activePhase = null;
        }
        notifyProgress(onProgress, {
          phase: message.phase,
          asset: message.asset,
          state: message.state,
          loadedBytes: message.loadedBytes,
          totalBytes: message.totalBytes,
          source: message.source,
          error: message.error,
        });
        return;
      }
      if (message?.type === "ready") {
        initialized = true;
        beginDecode();
        return;
      }
      if (message?.type === "result" && message.bytes instanceof ArrayBuffer) finish(resolve, new Uint8Array(message.bytes));
      else if (message?.type === "failure") {
        activePhase = null;
        const error = new Error(`Pinned asset decode failed: ${String(message.error).slice(0, 1024)}`);
        const failurePhase = ["fetch", "verify", "decode", "initialize"].includes(message.phase) ? message.phase : "decode";
        notifyProgress(onProgress, {
          phase: failurePhase, asset: message.asset || asset, state: "error",
          loadedBytes: failurePhase === "fetch" ? 0 : null,
          totalBytes: null, source: failurePhase === "fetch" ? "network-or-cache" : "work", error: error.message,
        });
        finish(reject, error);
      }
      else finish(reject, new Error("Brotli decoder worker returned an invalid response."));
    };
    worker.addEventListener("message", onMessage);
    worker.addEventListener("error", onError);
    signal?.addEventListener("abort", onAbort, { once: true });
    if (signal?.aborted) return onAbort();
    promise.then((compressed) => {
      if (settled) return;
      compressedBuffer = compressed.buffer.slice(compressed.byteOffset, compressed.byteOffset + compressed.byteLength);
      beginDecode();
    }, (error) => finish(reject, error));
  });
}

async function decodeAsset(file, pin, signal, onProgress, asset) {
  const compressed = await fetchBytes(new URL(`./${file}`, import.meta.url), {
    signal,
    limit: MAX_COMPRESSED_BYTES,
    label: file,
    onProgress,
    asset,
    totalBytes: pin.bytes,
  });
  workStage(onProgress, "verify", asset, "start", compressed.byteLength, pin.bytes);
  try {
    if (compressed.byteLength !== pin.bytes || await digest(compressed) !== pin.sha256) {
      throw new Error(`Pinned compiler asset integrity check failed: ${file}.`);
    }
    checkAbort(signal);
    verifiedStage(onProgress, asset, "complete", compressed.byteLength, pin.bytes);
  } catch (error) {
    verifiedStage(onProgress, asset, isAbort(error, signal) ? "cancelled" : "error", compressed.byteLength, pin.bytes,
      isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  checkAbort(signal);
  notifyProgress(onProgress, {
      phase: "fetch", asset, state: "complete", loadedBytes: compressed.byteLength,
      totalBytes: pin.bytes, source: "network-or-cache", error: null,
    });
  checkAbort(signal);
  const worker = new Worker(new URL("./compiler/decode-worker.js?blr_app=543af08c9c9d6446694684fd0182efa61b8269029917ff4e95fd0f76dfb5ab59", import.meta.url), {
    type: "module",
    name: "blr-pinned-toolchain-decoder",
  });
  const decode = withAbort(Promise.resolve(compressed), signal, worker, onProgress, asset);
  worker.postMessage({
    type: "init",
    asset,
    decoderUrl: new URL("./compiler/brotli-dec-wasm/pkg/brotli_dec_wasm_bg.wasm", import.meta.url).href,
    decoderBytes: DECODER_WASM_BYTES,
    decoderSha256: DECODER_WASM_SHA256,
  });
  const raw = await decode;
  workStage(onProgress, "verify", asset, "start", raw.byteLength, pin.rawBytes);
  try {
    if (raw.byteLength !== pin.rawBytes || await digest(raw) !== pin.rawSha256) {
      throw new Error(`Decoded compiler asset integrity check failed: ${file}.`);
    }
    checkAbort(signal);
    verifiedStage(onProgress, asset, "complete", raw.byteLength, pin.rawBytes);
  } catch (error) {
    verifiedStage(onProgress, asset, isAbort(error, signal) ? "cancelled" : "error", raw.byteLength, pin.rawBytes,
      isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  checkAbort(signal);
  return raw;
}

async function loadManifest(signal, onProgress) {
  const asset = "compiler-manifest";
  let response;
  try {
    response = await fetch(new URL("./compiler/cache-manifest.json", import.meta.url), {
      cache: "no-store",
      credentials: "same-origin",
      signal,
    });
  } catch (error) {
    notifyProgress(onProgress, {
      phase: "fetch", asset, state: isAbort(error, signal) ? "cancelled" : "error",
      loadedBytes: 0, totalBytes: null, source: "network-or-cache",
      error: isAbort(error, signal) ? null : errorMessage(error),
    });
    throw error;
  }
  const manifest = await readBoundedJson(response, {
    signal,
    limit: MAX_COMPILER_MANIFEST_BYTES,
    label: "Compiler asset manifest",
    onProgress,
    progress: { phase: "fetch", asset, totalBytes: null, source: "network-or-cache" },
  });
  workStage(onProgress, "verify", asset, "start");
  try {
    checkAbort(signal);
    if (manifest?.schema !== "blr-rust-compiler-assets-v1"
      || manifest.compilerVersion !== "rustc 1.83.0-dev"
      || manifest.target !== "wasm32-wasip1"
      || !Array.isArray(manifest.assets) || manifest.assets.length !== pins.size) {
      throw new Error("Compiler asset manifest has an unsupported schema.");
    }
    const seen = new Set();
    for (const entry of manifest.assets) {
      const pin = pins.get(entry?.file);
      if (!pin || seen.has(entry.file) || entry.bytes !== pin.bytes || entry.sha256 !== pin.sha256) {
        throw new Error("Compiler asset manifest does not match the pinned toolchain.");
      }
      seen.add(entry.file);
    }
    if (seen.size !== pins.size) throw new Error("Compiler asset manifest is incomplete.");
    workStage(onProgress, "verify", asset, "complete");
  } catch (error) {
    workStage(onProgress, "verify", asset, isAbort(error, signal) ? "cancelled" : "error",
      null, null, isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  checkAbort(signal);
  notifyProgress(onProgress, {
      phase: "fetch", asset, state: "complete", loadedBytes: null,
      totalBytes: null, source: "network-or-cache", error: null,
    });
  checkAbort(signal);
  return manifest;
}

let cachedExampleInputs;

function validateExampleManifest(examples) {
  if (!examples || typeof examples !== "object" || Array.isArray(examples)
    || examples.schema !== "blr-rust-demo-examples-v2"
    || examples.compilerTarget !== "wasm32-wasip1"
    || examples.helperModule !== "blr_demo.rs"
    || !/^[a-f0-9]{64}$/.test(examples.helperSha256 || "")
    || !examples.authoringLibrary || typeof examples.authoringLibrary !== "object"
    || Array.isArray(examples.authoringLibrary)
    || examples.authoringLibrary.file !== "libblr.rlib"
    || !Number.isSafeInteger(examples.authoringLibrary.bytes)
    || examples.authoringLibrary.bytes <= 0
    || examples.authoringLibrary.bytes > MAX_AUTHORING_LIBRARY_BYTES
    || !/^[a-f0-9]{64}$/.test(examples.authoringLibrary.sha256 || "")
    || examples.authoringLibrary.compilerVersion !== "rustc 1.83.0-dev"
    || examples.authoringLibrary.target !== "wasm32-wasip1"
    || examples.authoringLibrary.rustcAssetSha256 !== pins.get("compiler/assets/rustc_opt.wasm.br").sha256
    || examples.authoringLibrary.sysrootAssetSha256 !== pins.get("compiler/assets/wasm32-wasip1.tar.br").sha256
    || !Array.isArray(examples.examples) || examples.examples.length === 0 || examples.examples.length > 12) {
    throw new Error("Rust example manifest has an unsupported schema.");
  }
  const ids = new Set();
  for (const example of examples.examples) {
    if (!example || typeof example !== "object" || Array.isArray(example)
      || typeof example.id !== "string" || !example.id || ids.has(example.id)
      || typeof example.source !== "string"
      || example.rustEntryPoint !== "scene"
      || (example.pythonSource !== undefined && typeof example.pythonSource !== "string")
      || (example.languages !== undefined && (!Array.isArray(example.languages)
        || example.languages.some((language) => language !== "rust" && language !== "python")
        || new Set(example.languages).size !== example.languages.length))) {
      throw new Error("Rust example manifest contains an invalid example.");
    }
    ids.add(example.id);
  }
}

/** Load and verify example metadata and pinned authoring inputs once for both the UI and compiler. */
export async function loadExampleInputs({ signal, onProgress } = {}) {
  checkAbort(signal);
  if (cachedExampleInputs) {
    notifyProgress(onProgress, {
      phase: "fetch", asset: "examples-manifest", state: "complete",
      loadedBytes: null, totalBytes: null, source: "memory-cache", error: null,
    });
    checkAbort(signal);
    notifyProgress(onProgress, {
      phase: "fetch", asset: "blr-helper", state: "complete",
      loadedBytes: null, totalBytes: null, source: "memory-cache", error: null,
    });
    checkAbort(signal);
    notifyProgress(onProgress, {
      phase: "fetch", asset: "blr-library", state: "complete",
      loadedBytes: null, totalBytes: null, source: "memory-cache", error: null,
    });
    checkAbort(signal);
    return cachedExampleInputs;
  }
  return runWithAssetLoadBounds(async (loadSignal) => {
    const examplesPromise = (async () => {
      const asset = "examples-manifest";
      let response;
      try {
        response = await fetch(new URL("./examples.json", import.meta.url), {
          cache: "no-store", credentials: "same-origin", signal: loadSignal,
        });
      } catch (error) {
        notifyProgress(onProgress, {
          phase: "fetch", asset, state: isAbort(error, loadSignal) ? "cancelled" : "error",
          loadedBytes: 0, totalBytes: null, source: "network-or-cache",
          error: isAbort(error, loadSignal) ? null : errorMessage(error),
        });
        throw error;
      }
      const manifest = await readBoundedJson(response, {
        signal: loadSignal, limit: MAX_EXAMPLES_MANIFEST_BYTES, label: "Rust example manifest",
        onProgress, progress: { phase: "fetch", asset, totalBytes: null, source: "network-or-cache" },
      });
      workStage(onProgress, "verify", asset, "start");
      try {
        validateExampleManifest(manifest);
        checkAbort(loadSignal);
        workStage(onProgress, "verify", asset, "complete");
      } catch (error) {
        workStage(onProgress, "verify", asset, isAbort(error, loadSignal) ? "cancelled" : "error",
          null, null, isAbort(error, loadSignal) ? null : errorMessage(error));
        throw error;
      }
      checkAbort(loadSignal);
      notifyProgress(onProgress, {
          phase: "fetch", asset, state: "complete", loadedBytes: null,
          totalBytes: null, source: "network-or-cache", error: null,
        });
      checkAbort(loadSignal);
      return manifest;
    })();
    const manifest = await examplesPromise;
    // Keep each helper version in a distinct browser HTTP-cache entry.
    const helperUrl = new URL(`./blr_demo.rs?v=${encodeURIComponent(manifest.helperSha256)}`, import.meta.url);
    const helperOptions = {
      signal: loadSignal, limit: MAX_HELPER_BYTES, label: "Rust scene helper",
      onProgress, asset: "blr-helper", totalBytes: null,
    };
    let helper = await fetchBytes(helperUrl, helperOptions);
    checkAbort(loadSignal);
    workStage(onProgress, "verify", "blr-helper", "start", helper.byteLength, null);
    let helperText;
    try {
      let helperDigest = await digest(helper);
      checkAbort(loadSignal);
      if (helperDigest !== manifest.helperSha256) {
        // Refresh this one pinned cache entry once; never accept bytes without the digest.
        helper = await fetchBytes(helperUrl, { ...helperOptions, cache: "reload" });
        checkAbort(loadSignal);
        helperDigest = await digest(helper);
        checkAbort(loadSignal);
        if (helperDigest !== manifest.helperSha256) {
          throw new Error("Rust scene helper does not match the examples manifest.");
        }
      }
      helperText = new TextDecoder("utf-8", { fatal: true }).decode(helper);
      checkAbort(loadSignal);
      verifiedStage(onProgress, "blr-helper", "complete", helper.byteLength, null);
    } catch (error) {
      verifiedStage(onProgress, "blr-helper", isAbort(error, loadSignal) ? "cancelled" : "error",
        helper.byteLength, null, isAbort(error, loadSignal) ? null : errorMessage(error));
      throw error;
    }
    checkAbort(loadSignal);
    notifyProgress(onProgress, {
        phase: "fetch", asset: "blr-helper", state: "complete", loadedBytes: helper.byteLength,
        totalBytes: null, source: "network-or-cache", error: null,
      });
    checkAbort(loadSignal);
    const libraryPin = manifest.authoringLibrary;
    const libraryUrl = new URL(`./${libraryPin.file}?v=${encodeURIComponent(libraryPin.sha256)}`, import.meta.url);
    const libraryOptions = {
      signal: loadSignal, limit: MAX_AUTHORING_LIBRARY_BYTES, label: "Rust authoring library",
      onProgress, asset: "blr-library", totalBytes: libraryPin.bytes,
    };
    let library = await fetchBytes(libraryUrl, libraryOptions);
    checkAbort(loadSignal);
    workStage(onProgress, "verify", "blr-library", "start", library.byteLength, libraryPin.bytes);
    try {
      if (library.byteLength !== libraryPin.bytes) {
        throw new Error(`Rust authoring library size ${library.byteLength} does not match the manifest size ${libraryPin.bytes}.`);
      }
      let libraryDigest = await digest(library);
      checkAbort(loadSignal);
      if (libraryDigest !== libraryPin.sha256) {
        library = await fetchBytes(libraryUrl, { ...libraryOptions, cache: "reload" });
        checkAbort(loadSignal);
        if (library.byteLength !== libraryPin.bytes) {
          throw new Error(`Rust authoring library size ${library.byteLength} does not match the manifest size ${libraryPin.bytes}.`);
        }
        libraryDigest = await digest(library);
        checkAbort(loadSignal);
      }
      if (libraryDigest !== libraryPin.sha256) {
        throw new Error("Rust authoring library does not match the examples manifest.");
      }
      verifiedStage(onProgress, "blr-library", "complete", library.byteLength, libraryPin.bytes);
    } catch (error) {
      verifiedStage(onProgress, "blr-library", isAbort(error, loadSignal) ? "cancelled" : "error",
        library?.byteLength ?? null, libraryPin.bytes, isAbort(error, loadSignal) ? null : errorMessage(error));
      throw error;
    }
    checkAbort(loadSignal);
    notifyProgress(onProgress, {
      phase: "fetch", asset: "blr-library", state: "complete", loadedBytes: library.byteLength,
      totalBytes: libraryPin.bytes, source: "network-or-cache", error: null,
    });
    checkAbort(loadSignal);
    const inputs = {
      manifest,
      examples: manifest.examples,
      helper,
      helperText,
      helperSha256: manifest.helperSha256,
      library,
      librarySha256: libraryPin.sha256,
    };
    cachedExampleInputs = inputs;
    return inputs;
  }, { signal });
}

async function useExampleInputs(inputs, { signal, onProgress } = {}) {
  const candidate = inputs ?? await loadExampleInputs({ signal, onProgress });
  checkAbort(signal);
  workStage(onProgress, "verify", "examples-manifest", "start");
  let manifest;
  try {
    manifest = structuredClone(candidate?.manifest);
    validateExampleManifest(manifest);
    checkAbort(signal);
    workStage(onProgress, "verify", "examples-manifest", "complete");
  } catch (error) {
    workStage(onProgress, "verify", "examples-manifest", isAbort(error, signal) ? "cancelled" : "error",
      null, null, isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  checkAbort(signal);
  workStage(onProgress, "verify", "blr-helper", "start");
  let helper;
  let helperText;
  try {
    if (!(candidate.helper instanceof Uint8Array)) {
      throw new Error("Rust scene helper does not match the examples manifest.");
    }
    helper = candidate.helper.slice();
    if (await digest(helper) !== manifest.helperSha256) {
      throw new Error("Rust scene helper does not match the examples manifest.");
    }
    helperText = new TextDecoder("utf-8", { fatal: true }).decode(helper);
    checkAbort(signal);
    verifiedStage(onProgress, "blr-helper", "complete", helper.byteLength, null);
  } catch (error) {
    verifiedStage(onProgress, "blr-helper", isAbort(error, signal) ? "cancelled" : "error",
      helper?.byteLength ?? null, null, isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  checkAbort(signal);
  const verified = {
    manifest,
    examples: manifest.examples,
    helper,
    helperText,
    helperSha256: manifest.helperSha256,
    librarySha256: manifest.authoringLibrary.sha256,
  };
  workStage(onProgress, "verify", "blr-library", "start", candidate.library?.byteLength ?? null,
    manifest.authoringLibrary.bytes);
  try {
    if (!(candidate.library instanceof Uint8Array)) {
      throw new Error("Rust authoring library does not match the examples manifest.");
    }
    verified.library = candidate.library.slice();
    const libraryPin = manifest.authoringLibrary;
    if (verified.library.byteLength !== libraryPin.bytes || await digest(verified.library) !== libraryPin.sha256) {
      throw new Error("Rust authoring library does not match the examples manifest.");
    }
    if (libraryPin.compilerVersion !== PINNED_TOOLCHAIN.compilerVersion
      || libraryPin.target !== PINNED_TOOLCHAIN.target
      || libraryPin.rustcAssetSha256 !== pins.get("compiler/assets/rustc_opt.wasm.br").sha256
      || libraryPin.sysrootAssetSha256 !== pins.get("compiler/assets/wasm32-wasip1.tar.br").sha256) {
      throw new Error("Rust authoring library does not match the pinned compiler and sysroot.");
    }
    checkAbort(signal);
    verifiedStage(onProgress, "blr-library", "complete", verified.library.byteLength, libraryPin.bytes);
  } catch (error) {
    verifiedStage(onProgress, "blr-library", isAbort(error, signal) ? "cancelled" : "error",
      verified.library?.byteLength ?? candidate.library?.byteLength ?? null,
      manifest.authoringLibrary.bytes, isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
  if (inputs) {
    for (const asset of ["examples-manifest", "blr-helper", "blr-library"]) {
      checkAbort(signal);
      notifyProgress(onProgress, {
        phase: "fetch", asset, state: "complete", loadedBytes: null,
        totalBytes: null, source: "memory-cache", error: null,
      });
    }
    checkAbort(signal);
  }
  return verified;
}

function makeSysrootFiles(tarBytes, { signal, onProgress } = {}) {
  const asset = "sysroot";
  const totalBytes = tarBytes.byteLength;
  workStage(onProgress, "unpack", asset, "start", 0, totalBytes);
  try {
  const files = Object.create(null);
  const decoder = new TextDecoder();
  const field = (view, start, length) => decoder.decode(view.subarray(start, start + length)).replace(/\0.*$/s, "");
  const base = "sysroot/lib/rustlib/wasm32-wasip1/lib/";
  let entries = 0;
  for (let offset = 0; offset + 512 <= tarBytes.byteLength;) {
    checkAbort(signal);
    const header = tarBytes.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) break;
    const name = field(header, 0, 100);
    const prefix = field(header, 345, 155);
    const path = (prefix ? `${prefix}/${name}` : name).replace(/\/$/, "");
    if (!path || path.split("/").some((part) => !part || part === "." || part === "..")) {
      throw new Error(`Unsafe Rust sysroot path: ${path}.`);
    }
    const size = Number.parseInt(field(header, 124, 12).trim(), 8);
    const type = header[156];
    if (!Number.isSafeInteger(size) || size < 0 || offset + 512 + size > tarBytes.byteLength) {
      throw new Error(`Invalid Rust sysroot entry: ${path}.`);
    }
    if (type === 0 || type === 48) {
      if (size === 0 || files[base + path]) throw new Error(`Invalid duplicate or empty Rust sysroot file: ${path}.`);
      files[base + path] = tarBytes.slice(offset + 512, offset + 512 + size);
      entries += 1;
    } else if (type === 53 && size === 0) {
      // Directories are reconstructed from validated file paths in the VFS.
    } else {
      throw new Error(`Unsupported Rust sysroot entry ${type} at ${path}.`);
    }
    offset += 512 + Math.ceil(size / 512) * 512;
    workStage(onProgress, "unpack", asset, "update", Math.min(offset, totalBytes), totalBytes);
  }
  checkAbort(signal);
  if (entries !== 30) throw new Error(`Expected 30 pinned Rust sysroot files; found ${entries}.`);
  files["tmp/.keep"] = new Uint8Array(0);
  files["target/.keep"] = new Uint8Array(0);
  workStage(onProgress, "unpack", asset, "complete", totalBytes, totalBytes);
  return files;
  } catch (error) {
    workStage(onProgress, "unpack", asset, isAbort(error, signal) ? "cancelled" : "error",
      null, totalBytes, isAbort(error, signal) ? null : errorMessage(error));
    throw error;
  }
}

let cachedInputs;
export async function loadPinnedInputs({ signal, onProgress, exampleInputs } = {}) {
  checkAbort(signal);
  if (cachedInputs) {
    const examples = await useExampleInputs(exampleInputs ?? cachedInputs.exampleInputs ?? cachedInputs, { signal, onProgress });
    for (const asset of ["rustc", "sysroot"]) {
      checkAbort(signal);
      notifyProgress(onProgress, {
        phase: "fetch", asset, state: "complete", loadedBytes: null,
        totalBytes: null, source: "memory-cache", error: null,
      });
    }
    checkAbort(signal);
    if (examples === cachedInputs.exampleInputs) return cachedInputs;
    return { ...cachedInputs, ...examples, exampleInputs: examples };
  }
  return runWithAssetLoadBounds(async (loadSignal) => {
    await loadManifest(loadSignal, onProgress);
    const examplesPromise = useExampleInputs(exampleInputs, { signal: loadSignal, onProgress });
    const rustcPin = pins.get("compiler/assets/rustc_opt.wasm.br");
    const sysrootPin = pins.get("compiler/assets/wasm32-wasip1.tar.br");
    const rustcPromise = decodeAsset("compiler/assets/rustc_opt.wasm.br", rustcPin, loadSignal, onProgress, "rustc");
    const sysrootPromise = decodeAsset("compiler/assets/wasm32-wasip1.tar.br", sysrootPin, loadSignal, onProgress, "sysroot");
    const [examples, rustcBytes, sysrootTar] = await Promise.all([examplesPromise, rustcPromise, sysrootPromise]);
    checkAbort(loadSignal);
    const sysrootFiles = makeSysrootFiles(sysrootTar, { signal: loadSignal, onProgress });
    checkAbort(loadSignal);
    const inputs = {
      ...examples,
      exampleInputs: examples,
      rustcBytes,
      rustcSha256: rustcPin.rawSha256,
      rustcAssetSha256: rustcPin.sha256,
      sysrootAssetSha256: sysrootPin.sha256,
      sysrootFiles,
    };
    cachedInputs = inputs;
    return inputs;
  }, { signal });
}

export function clearPinnedInputCacheForTest() {
  cachedInputs = undefined;
}

export function clearExampleInputCacheForTest() {
  cachedExampleInputs = undefined;
}

export const PINNED_TOOLCHAIN = Object.freeze({
  compilerVersion: "rustc 1.83.0-dev",
  target: "wasm32-wasip1",
  rustcAssetSha256: pins.get("compiler/assets/rustc_opt.wasm.br").sha256,
  sysrootAssetSha256: pins.get("compiler/assets/wasm32-wasip1.tar.br").sha256,
  rawCompilerBytes: 95343874,
  rawSysrootTarBytes: 74096640,
  compressedBytes: 38707707,
  helperMaxBytes: MAX_HELPER_BYTES,
  sysrootTarBytes: 74096640,
  sysrootTarMaxBytes: MAX_SYSROOT_TAR_BYTES,
});
