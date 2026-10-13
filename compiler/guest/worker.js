import { WASI, ConsoleStdout, File, OpenFile, wasi as wasiDefs } from "../browser-wasi-shim/dist/index.js?blr_app=9282b9d231ff7b61276521b7d9f2c2caf25e4dd34eb49dac28a63b89d0ea837e";
import { inspectWasm, MAX_LINE_BYTES, MAX_OUTPUT_BYTES } from "./wasm-policy.js?blr_app=9282b9d231ff7b61276521b7d9f2c2caf25e4dd34eb49dac28a63b89d0ea837e";

self.postMessage({ type: "ready" });

function errorMessage(error) {
  if (error instanceof Error) return error.message.slice(0, 2048);
  return String(error).slice(0, 2048);
}

self.onmessage = async event => {
  if (event.data?.type !== "run") return;
  try {
    const wasmBytes = new Uint8Array(event.data.wasmBytes);
    const policy = inspectWasm(wasmBytes);
    const outputBytes = { stdout: 0, stderr: 0 };
    const lineBytes = { stdout: 0, stderr: 0 };
    const decoder = {
      stdout: new TextDecoder("utf-8", { fatal: false }),
      stderr: new TextDecoder("utf-8", { fatal: false }),
    };
    const outputFd = stream => new ConsoleStdout(bytes => {
      outputBytes[stream] += bytes.byteLength;
      if (outputBytes.stdout + outputBytes.stderr > MAX_OUTPUT_BYTES) {
        throw new Error(`guest rejected: output limit exceeded (${MAX_OUTPUT_BYTES} bytes)`);
      }
      for (const byte of bytes) {
        lineBytes[stream] = byte === 10 ? 0 : lineBytes[stream] + 1;
        if (lineBytes[stream] > MAX_LINE_BYTES) {
          throw new Error(`guest rejected: ${stream} line exceeds ${MAX_LINE_BYTES} bytes`);
        }
      }
      const text = decoder[stream].decode(bytes, { stream: true });
      if (text) self.postMessage({ type: "output", stream, text });
    });

    // No preopened directory: guest file operations can see only these three
    // descriptors, with empty stdin and write-only output descriptors.
    const wasi = new WASI([], [], [
      new OpenFile(new File(new Uint8Array(0), { readonly: true })),
      outputFd("stdout"),
      outputFd("stderr"),
    ], { debug: false });
    let guestMemory;
    const originalFdWrite = wasi.wasiImport.fd_write;
    wasi.wasiImport.fd_write = (fd, iovsPointer, iovsCount, writtenPointer) => {
      if (fd !== 1 && fd !== 2) return wasiDefs.ERRNO_BADF;
      if (iovsCount > 1024) throw new Error("guest rejected: too many output buffers");
      if (!guestMemory) throw new Error("guest rejected: output before memory setup");
      const memoryView = new DataView(guestMemory.buffer);
      const memoryBytes = new Uint8Array(guestMemory.buffer);
      let pendingBytes = 0;
      let pendingLineBytes = lineBytes[fd === 1 ? "stdout" : "stderr"];
      for (let index = 0; index < iovsCount; index++) {
        const descriptor = iovsPointer + index * 8;
        const start = memoryView.getUint32(descriptor, true);
        const length = memoryView.getUint32(descriptor + 4, true);
        if (start + length > memoryBytes.byteLength) throw new Error("guest rejected: invalid output buffer");
        pendingBytes += length;
        if (outputBytes.stdout + outputBytes.stderr + pendingBytes > MAX_OUTPUT_BYTES) {
          throw new Error(`guest rejected: output limit exceeded (${MAX_OUTPUT_BYTES} bytes)`);
        }
        for (let position = start; position < start + length; position++) {
          pendingLineBytes = memoryBytes[position] === 10 ? 0 : pendingLineBytes + 1;
          if (pendingLineBytes > MAX_LINE_BYTES) {
            throw new Error(`guest rejected: ${fd === 1 ? "stdout" : "stderr"} line exceeds ${MAX_LINE_BYTES} bytes`);
          }
        }
      }
      return originalFdWrite(fd, iovsPointer, iovsCount, writtenPointer);
    };
    const originalFdRead = wasi.wasiImport.fd_read;
    wasi.wasiImport.fd_read = (fd, iovsPointer, iovsCount, readPointer) => {
      if (fd !== 0) return wasiDefs.ERRNO_BADF;
      if (iovsCount > 1024) throw new Error("guest rejected: too many input buffers");
      return originalFdRead(fd, iovsPointer, iovsCount, readPointer);
    };
    const module = await WebAssembly.compile(wasmBytes);
    const instance = await WebAssembly.instantiate(module, {
      wasi_snapshot_preview1: wasi.wasiImport,
    });
    if (!(instance.exports.memory instanceof WebAssembly.Memory)) {
      throw new Error("guest rejected: exported linear memory required");
    }
    guestMemory = instance.exports.memory;
    const actualBytes = instance.exports.memory.buffer.byteLength;
    const declaredBytes = policy.memory.initialPages * 65536;
    if (actualBytes !== declaredBytes || actualBytes > policy.memory.maximumBytes) {
      throw new Error("guest rejected: instantiated memory does not match validated limits");
    }
    if (typeof instance.exports._start !== "function") {
      throw new Error("guest rejected: WASI _start export required");
    }
    const exitCode = wasi.start(instance);
    for (const stream of ["stdout", "stderr"]) {
      const tail = decoder[stream].decode();
      if (tail) self.postMessage({ type: "output", stream, text: tail });
    }
    self.postMessage({
      type: "result",
      exitCode,
      imports: policy.imports,
      memoryEvidence: {
        initialPages: policy.memory.initialPages,
        maximumPages: policy.memory.maximumPages,
        maximumBytes: policy.memory.maximumBytes,
        actualBytes: instance.exports.memory.buffer.byteLength,
      },
    });
  } catch (error) {
    self.postMessage({ type: "failure", error: errorMessage(error) });
  }
};
