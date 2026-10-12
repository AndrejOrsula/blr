import { readBoundedJson, readBoundedResponse, runWithAssetLoadBounds } from "./compiler/asset-loading.js";

const MAX_HELPER_BYTES = 128 * 1024;
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

async function fetchBytes(url, { signal, limit, label }) {
  const response = await fetch(url, { cache: "force-cache", credentials: "same-origin", signal });
  return readBoundedResponse(response, { signal, limit, label });
}

function withAbort(promise, signal, worker) {
  if (signal?.aborted) {
    worker.terminate();
    return Promise.reject(abortError());
  }
  return new Promise((resolve, reject) => {
    let settled = false;
    let initialized = false;
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
    const onAbort = () => finish(reject, abortError());
    const onError = (event) => finish(reject, new Error(`Brotli decoder worker failed: ${event.message || "unknown worker error"}`));
    const onMessage = (event) => {
      const message = event.data;
      if (message?.type === "ready") {
        initialized = true;
        beginDecode();
        return;
      }
      if (message?.type === "result" && message.bytes instanceof ArrayBuffer) finish(resolve, new Uint8Array(message.bytes));
      else if (message?.type === "failure") finish(reject, new Error(`Pinned asset decode failed: ${String(message.error).slice(0, 1024)}`));
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

async function decodeAsset(file, pin, signal) {
  const compressed = await fetchBytes(new URL(`./${file}`, import.meta.url), {
    signal,
    limit: MAX_COMPRESSED_BYTES,
    label: file,
  });
  if (compressed.byteLength !== pin.bytes || await digest(compressed) !== pin.sha256) {
    throw new Error(`Pinned compiler asset integrity check failed: ${file}.`);
  }
  checkAbort(signal);
  const worker = new Worker(new URL("./compiler/decode-worker.js", import.meta.url), {
    type: "module",
    name: "blr-pinned-toolchain-decoder",
  });
  const decode = withAbort(Promise.resolve(compressed), signal, worker);
  worker.postMessage({
    type: "init",
    decoderUrl: new URL("./compiler/brotli-dec-wasm/pkg/brotli_dec_wasm_bg.wasm", import.meta.url).href,
    decoderBytes: DECODER_WASM_BYTES,
    decoderSha256: DECODER_WASM_SHA256,
  });
  const raw = await decode;
  if (raw.byteLength !== pin.rawBytes || await digest(raw) !== pin.rawSha256) {
    throw new Error(`Decoded compiler asset integrity check failed: ${file}.`);
  }
  return raw;
}

async function loadManifest(signal) {
  const response = await fetch(new URL("./compiler/cache-manifest.json", import.meta.url), {
    cache: "no-store",
    credentials: "same-origin",
    signal,
  });
  const manifest = await readBoundedJson(response, {
    signal,
    limit: MAX_COMPILER_MANIFEST_BYTES,
    label: "Compiler asset manifest",
  });
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
  return manifest;
}

async function loadHelper(signal) {
  const bytes = await fetchBytes(new URL("./blr_demo.rs", import.meta.url), {
    signal,
    limit: MAX_HELPER_BYTES,
    label: "Rust scene helper",
  });
  const examplesResponse = await fetch(new URL("./examples.json", import.meta.url), {
    cache: "no-store",
    credentials: "same-origin",
    signal,
  });
  const examples = await readBoundedJson(examplesResponse, {
    signal,
    limit: MAX_EXAMPLES_MANIFEST_BYTES,
    label: "Rust example manifest",
  });
  if (examples?.schema !== "blr-rust-demo-examples-v1"
    || examples?.compilerTarget !== "wasm32-wasip1"
    || examples?.helperModule !== "blr_demo.rs"
    || !/^[a-f0-9]{64}$/.test(examples.helperSha256 || "")
    || await digest(bytes) !== examples.helperSha256) {
    throw new Error("Rust scene helper does not match the pinned example manifest.");
  }
  return bytes;
}

function makeSysrootFiles(tarBytes) {
  const files = Object.create(null);
  const decoder = new TextDecoder();
  const field = (view, start, length) => decoder.decode(view.subarray(start, start + length)).replace(/\0.*$/s, "");
  const base = "sysroot/lib/rustlib/wasm32-wasip1/lib/";
  let entries = 0;
  for (let offset = 0; offset + 512 <= tarBytes.byteLength;) {
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
  }
  if (entries !== 30) throw new Error(`Expected 30 pinned Rust sysroot files; found ${entries}.`);
  files["tmp/.keep"] = new Uint8Array(0);
  files["target/.keep"] = new Uint8Array(0);
  return files;
}

let cachedInputs;
export async function loadPinnedInputs({ signal } = {}) {
  checkAbort(signal);
  if (cachedInputs) return cachedInputs;
  return runWithAssetLoadBounds(async (loadSignal) => {
    await loadManifest(loadSignal);
    const helperPromise = loadHelper(loadSignal);
    const rustcPromise = decodeAsset("compiler/assets/rustc_opt.wasm.br", pins.get("compiler/assets/rustc_opt.wasm.br"), loadSignal);
    const sysrootPromise = decodeAsset("compiler/assets/wasm32-wasip1.tar.br", pins.get("compiler/assets/wasm32-wasip1.tar.br"), loadSignal);
    const [helper, rustcBytes, sysrootTar] = await Promise.all([helperPromise, rustcPromise, sysrootPromise]);
    checkAbort(loadSignal);
    const inputs = { helper, rustcBytes, sysrootFiles: makeSysrootFiles(sysrootTar) };
    cachedInputs = inputs;
    return inputs;
  }, { signal });
}

export function clearPinnedInputCacheForTest() {
  cachedInputs = undefined;
}

export const PINNED_TOOLCHAIN = Object.freeze({
  compilerVersion: "rustc 1.83.0-dev",
  target: "wasm32-wasip1",
  rawCompilerBytes: 95343874,
  rawSysrootTarBytes: 74096640,
  compressedBytes: 38707707,
  helperMaxBytes: MAX_HELPER_BYTES,
  sysrootTarBytes: 74096640,
  sysrootTarMaxBytes: MAX_SYSROOT_TAR_BYTES,
});
