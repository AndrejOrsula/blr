import initBrotli, { brotli_dec } from "./brotli-dec-wasm/pkg/brotli_dec_wasm.js";
import { readBoundedResponse } from "./asset-loading.js";

let ready = false;
let activePhase = "initialize";
let inputAsset = "rustc";

function report(phase, state, asset = "brotli-decoder", loadedBytes = null, totalBytes = null, error = null) {
  activePhase = phase;
  self.postMessage({ type: "progress", phase, asset, state, loadedBytes, totalBytes, source: "work", error });
}

function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

self.addEventListener("message", async (event) => {
  const message = event.data;
  try {
    if (message?.type === "init") {
      if (ready || !["rustc", "sysroot"].includes(message.asset)
        || typeof message.decoderUrl !== "string" || message.decoderBytes !== 206243
        || message.decoderSha256 !== "7cb4663fd0b11185b88177d0b3b3048136572faf49e81c4c78dccafe37bb68d7") {
        throw new Error("Invalid Brotli decoder initialization request.");
      }
      inputAsset = message.asset;
      activePhase = "fetch";
      const response = await fetch(message.decoderUrl, { credentials: "same-origin", cache: "force-cache" });
      const contentEncoding = response.headers?.get?.("content-encoding")?.trim().toLowerCase();
      const bytes = await readBoundedResponse(response, {
        limit: message.decoderBytes,
        label: "Decoder Wasm",
        onProgress: (progress) => {
          if (progress.state !== "error" && progress.state !== "cancelled") {
            self.postMessage({ type: "progress", ...progress });
          }
        },
        progress: {
          phase: "fetch", asset: "brotli-decoder",
          totalBytes: !contentEncoding || contentEncoding === "identity" ? message.decoderBytes : null,
          source: "network-or-cache",
        },
      });
      report("verify", "start", "brotli-decoder", bytes.byteLength, message.decoderBytes);
      if (bytes.byteLength !== message.decoderBytes
        || hex(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes))) !== message.decoderSha256) {
        throw new Error("Brotli decoder Wasm pin mismatch.");
      }
      report("verify", "complete", "brotli-decoder", bytes.byteLength, message.decoderBytes);
      report("initialize", "start");
      await initBrotli(await WebAssembly.compile(bytes));
      ready = true;
      report("initialize", "complete");
      self.postMessage({ type: "ready" });
      return;
    }
    if (message?.type !== "decode" || !ready || !(message.buffer instanceof ArrayBuffer)) {
      throw new Error("Invalid Brotli decode request.");
    }
    report("decode", "start", inputAsset);
    const decoded = brotli_dec(new Uint8Array(message.buffer));
    report("decode", "complete", inputAsset, decoded.byteLength);
    self.postMessage({ type: "result", bytes: decoded.buffer }, [decoded.buffer]);
  } catch (error) {
    const message = String(error?.message || error).slice(0, 1024);
    self.postMessage({ type: "failure", phase: activePhase, asset: activePhase === "decode" ? inputAsset : "brotli-decoder", error: message });
  }
});
