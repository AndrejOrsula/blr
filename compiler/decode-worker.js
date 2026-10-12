import initBrotli, { brotli_dec } from "./brotli-dec-wasm/pkg/brotli_dec_wasm.js";
import { readBoundedResponse } from "./asset-loading.js";

let ready = false;

function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

self.addEventListener("message", async (event) => {
  const message = event.data;
  try {
    if (message?.type === "init") {
      if (ready || typeof message.decoderUrl !== "string" || message.decoderBytes !== 206243
        || message.decoderSha256 !== "7cb4663fd0b11185b88177d0b3b3048136572faf49e81c4c78dccafe37bb68d7") {
        throw new Error("Invalid Brotli decoder initialization request.");
      }
      const response = await fetch(message.decoderUrl, { credentials: "same-origin", cache: "force-cache" });
      const bytes = await readBoundedResponse(response, { limit: message.decoderBytes, label: "Decoder Wasm" });
      if (bytes.byteLength !== message.decoderBytes
        || hex(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes))) !== message.decoderSha256) {
        throw new Error("Brotli decoder Wasm pin mismatch.");
      }
      await initBrotli(await WebAssembly.compile(bytes));
      ready = true;
      self.postMessage({ type: "ready" });
      return;
    }
    if (message?.type !== "decode" || !ready || !(message.buffer instanceof ArrayBuffer)) {
      throw new Error("Invalid Brotli decode request.");
    }
    const decoded = brotli_dec(new Uint8Array(message.buffer));
    self.postMessage({ type: "result", bytes: decoded.buffer }, [decoded.buffer]);
  } catch (error) {
    self.postMessage({ type: "failure", error: String(error?.message || error).slice(0, 1024) });
  }
});
