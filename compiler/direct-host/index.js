import { WASI, WASIProcExit, File, Directory, OpenFile, PreopenDirectory, wasi as wasiDefs } from "../browser-wasi-shim/dist/index.js?blr_app=a04fd988c48d7984cfbb2a4470a1017f784b506d995b7dff1ad87a782f3fe926";
import { notifyProgress } from "../asset-loading.js?blr_app=a04fd988c48d7984cfbb2a4470a1017f784b506d995b7dff1ad87a782f3fe926";

const DEFAULT_TIMEOUT_MS = 180_000;
const MAX_TIMEOUT_MS = 600_000;
const MAX_THREADS = 16;
const MAX_THREAD_ID = (1 << 29) - 1;
const MAX_MODULE_BYTES = 128 * 1024 * 1024;
const MAX_MEMORY_PAGES = 16_384;
const MAX_FILE_BYTES = 256 * 1024 * 1024;
const MAX_TOTAL_BYTES = 512 * 1024 * 1024;
const MAX_PATH_BYTES = 4096;
const MAX_ARGS = 128;
const MAX_OUTPUT_BYTES = 2 * 1024 * 1024;
const encoder = new TextEncoder();

function fail(message, ErrorType = Error) {
  throw new ErrorType(`WASI compiler host: ${message}`);
}

function safePath(path) {
  if (typeof path !== "string" || !path || encoder.encode(path).byteLength > MAX_PATH_BYTES) fail("invalid virtual file path", TypeError);
  const normalized = path.replaceAll("\\", "/").replace(/^\/+/, "");
  const parts = normalized.split("/");
  if (parts.some((part) => !part || part === "." || part === "..")) fail(`unsafe virtual file path: ${path}`, TypeError);
  return parts;
}

function toFileMap(files) {
  const entries = files instanceof Map ? [...files] : Object.entries(files ?? {});
  const root = new Directory([]);
  const result = new Map();
  let total = 0;
  for (const [path, bytes] of entries) {
    const parts = safePath(path);
    if (!(bytes instanceof Uint8Array)) fail(`file ${path} must be Uint8Array`, TypeError);
    if (bytes.byteLength > MAX_FILE_BYTES) fail(`file ${path} exceeds ${MAX_FILE_BYTES} bytes`, RangeError);
    total += bytes.byteLength;
    if (total > MAX_TOTAL_BYTES) fail(`input files exceed ${MAX_TOTAL_BYTES} bytes`, RangeError);
    let directory = root;
    for (const part of parts.slice(0, -1)) {
      let child = directory.contents.get(part);
      if (!child) {
        child = new Directory([]);
        directory.contents.set(part, child);
      }
      if (!(child instanceof Directory)) fail(`file/directory path collision at ${path}`);
      directory = child;
    }
    const name = parts.at(-1);
    if (directory.contents.has(name)) fail(`duplicate virtual file path: ${path}`);
    const file = new File(bytes.slice(), { readonly: parts[0] === "sysroot" });
    directory.contents.set(name, file);
    result.set(`/${parts.join("/")}`, file);
  }
  return { root, result };
}

function collectFiles(root) {
  const output = Object.create(null);
  let total = 0;
  const visit = (directory, prefix) => {
    for (const [name, inode] of directory.contents) {
      const path = `${prefix}/${name}`;
      if (inode instanceof Directory) visit(inode, path);
      else if (inode instanceof File) {
        total += inode.data.byteLength;
        if (total > MAX_TOTAL_BYTES) fail(`output files exceed ${MAX_TOTAL_BYTES} bytes`, RangeError);
        output[path] = inode.data;
      }
    }
  };
  visit(root, "");
  return output;
}

function filesystemBytes(root) {
  const seen = new Set();
  let total = 0;
  const visit = (directory) => {
    for (const inode of directory.contents.values()) {
      if (inode instanceof Directory) visit(inode);
      else if (inode instanceof File && !seen.has(inode)) {
        seen.add(inode);
        total += inode.data.byteLength;
      }
    }
  };
  visit(root);
  return total;
}

function makeWorker() {
  if (typeof Worker !== "function") fail("browser Worker support is required");
  return new Worker(new URL("./worker.js?blr_app=a04fd988c48d7984cfbb2a4470a1017f784b506d995b7dff1ad87a782f3fe926", import.meta.url), { type: "module", name: "blr-rust-compiler" });
}

/** Run a standalone WASI command module in a disposable worker and in-memory filesystem. */
export function runWasiTool({ moduleBytes, args, files, signal, onOutput, onTrace, onProgress, timeoutMs = DEFAULT_TIMEOUT_MS, memory } = {}) {
  if (!(moduleBytes instanceof Uint8Array) || moduleBytes.byteLength === 0) return Promise.reject(new TypeError("moduleBytes must be a non-empty Uint8Array"));
  if (moduleBytes.byteLength > MAX_MODULE_BYTES) return Promise.reject(new RangeError(`moduleBytes exceeds ${MAX_MODULE_BYTES} bytes`));
  if (!Array.isArray(args) || args.length > MAX_ARGS || args.some((arg) => typeof arg !== "string" || encoder.encode(arg).byteLength > 16 * 1024)) return Promise.reject(new TypeError(`args must contain at most ${MAX_ARGS} bounded strings`));
  if (!Number.isInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > MAX_TIMEOUT_MS) return Promise.reject(new RangeError(`timeoutMs must be from 1 to ${MAX_TIMEOUT_MS}`));
  if (memory !== undefined && (memory === null || typeof memory !== "object" || Array.isArray(memory))) return Promise.reject(new TypeError("memory must be an object with initialPages and maximumPages"));
  const initialPages = memory?.initialPages ?? 399;
  const maximumPages = memory?.maximumPages ?? MAX_MEMORY_PAGES;
  if (!Number.isSafeInteger(initialPages) || !Number.isSafeInteger(maximumPages) || initialPages < 1 || maximumPages < initialPages || maximumPages > MAX_MEMORY_PAGES) {
    return Promise.reject(new RangeError(`memory pages must satisfy 1 <= initialPages <= maximumPages <= ${MAX_MEMORY_PAGES}`));
  }
  if (signal?.aborted) return Promise.reject(new DOMException("Compiler run aborted", "AbortError"));
  if (typeof SharedArrayBuffer !== "function" || !globalThis.crossOriginIsolated) return Promise.reject(new Error("WASI compiler requires cross-origin isolation and SharedArrayBuffer"));
  let activePhase = null;
  const initializeProgress = (state, error = null) => {
    if (state === "start" || state === "update") activePhase = "initialize";
    else if (activePhase === "initialize") activePhase = null;
    notifyProgress(onProgress, {
      phase: "initialize", asset: "rustc-compiler", state,
      loadedBytes: null, totalBytes: null, source: "work", error,
    });
  };
  initializeProgress("start");
  if (signal?.aborted) {
    initializeProgress("cancelled");
    return Promise.reject(new DOMException("Compiler run aborted", "AbortError"));
  }
  let fs;
  try { fs = toFileMap(files); } catch (error) {
    initializeProgress("error", error instanceof Error ? error.message : String(error));
    return Promise.reject(error);
  }
  if (signal?.aborted) {
    initializeProgress("cancelled");
    return Promise.reject(new DOMException("Compiler run aborted", "AbortError"));
  }
  let worker;
  try { worker = makeWorker(); } catch (error) {
    initializeProgress("error", error instanceof Error ? error.message : String(error));
    return Promise.reject(error);
  }

  return new Promise((resolve, reject) => {
    const output = { stdout: "", stderr: "" };
    const threads = new Map();
    let wasi;
    let settled = false;
    let timer;
    let nextTid = 1;
    let outputBytes = 0;
    const fdPaths = new Map([[3, "/"]]);

    const dispose = () => {
      clearTimeout(timer);
      signal?.removeEventListener("abort", abort);
      worker.removeEventListener("message", receive);
      worker.removeEventListener("error", onError);
      worker.terminate();
      for (const child of threads.values()) child.worker.terminate();
      threads.clear();
    };
    const finish = (error, exitCode) => {
      if (settled) return;
      settled = true;
      if (activePhase) {
        const state = error ? "error" : "complete";
        const phase = activePhase;
        activePhase = null;
        if (phase === "initialize") initializeProgress(state, error ? error.message : null);
        else notifyProgress(onProgress, {
          phase, asset: "rustc-compiler", state,
          loadedBytes: null, totalBytes: null, source: "work",
          error: error ? error.message : null,
        });
      }
      dispose();
      if (error) reject(error);
      else {
        try { resolve({ exitCode, stdout: output.stdout, stderr: output.stderr, files: collectFiles(fs.root) }); }
        catch (collectError) { reject(collectError); }
      }
    };
    const abort = () => {
      if (activePhase === "initialize") initializeProgress("cancelled");
      else if (activePhase) {
        const phase = activePhase;
        activePhase = null;
        notifyProgress(onProgress, {
          phase, asset: "rustc-compiler", state: "cancelled",
          loadedBytes: null, totalBytes: null, source: "work", error: null,
        });
      }
      finish(new DOMException("Compiler run aborted", "AbortError"));
    };
    const onError = (event) => {
      const error = new Error(`WASI compiler worker failed: ${event.message || "unknown worker error"}`);
      finish(error);
    };

    const createThread = (tid, arg, memoryObject, moduleObject, imports) => {
      if (threads.size >= MAX_THREADS) return false;
      const rpc = new SharedArrayBuffer(8);
      const state = new Int32Array(rpc);
      const child = makeWorker();
      threads.set(tid, { worker: child, state });
      child.addEventListener("message", (event) => receiveRpc(event, child, rpc, true));
      child.addEventListener("error", onError);
      child.postMessage({ type: "thread", module: moduleObject, memory: memoryObject, imports, tid, arg, rpc });
      return true;
    };

    const handleRpc = (event, sourceWorker, rpc, isThread) => {
      const message = event.data;
      if (message?.type !== "rpc") return;
      const state = new Int32Array(rpc);
      let status = 1;
      let result = 0;
      try {
        if (message.name === "wasi.thread-spawn" || message.name === "thread-spawn") {
          if (nextTid > MAX_THREAD_ID) fail(`WASI thread id limit ${MAX_THREAD_ID} exceeded`);
          const tid = nextTid++;
          if (!createThread(tid, message.args[0] | 0, wasi.inst.exports.memory, moduleObject, imports)) {
            status = -1;
            result = -1;
          } else result = tid;
        } else {
          refreshMemory();
          const fn = wasi.wasiImport[message.name];
          if (typeof fn !== "function") fail(`unsupported import ${message.module}.${message.name}`);
          const preflight = checkWasiMutation(message.name, message.args);
          if (preflight !== undefined && !(preflight && typeof preflight === "object")) result = preflight;
          else result = fn(...message.args, preflight);
          if (message.name === "path_open" && result === wasiDefs.ERRNO_SUCCESS) {
            const [fd, , pathPointer, pathLength, , , , , openedFdPointer] = message.args;
            fdPaths.set(memoryView().getUint32(openedFdPointer, true), resolvePath(fd, pathPointer, pathLength));
          }
          if (message.name === "fd_close" && result === wasiDefs.ERRNO_SUCCESS) fdPaths.delete(message.args[0]);
          if (message.name === "path_create_directory" || message.name === "path_link" || message.name === "path_rename" || message.name === "path_unlink_file" || message.name === "path_remove_directory") {
            if (filesystemBytes(fs.root) > MAX_TOTAL_BYTES) finish(new RangeError(`virtual filesystem exceeds ${MAX_TOTAL_BYTES} bytes`));
          }
        }
      } catch (error) {
        if (error instanceof WASIProcExit) {
          status = 2;
          result = error.code;
        } else {
          status = -1;
          result = 29; // WASI ERRNO_IO; retain a bounded host-side diagnostic.
          const text = error instanceof Error ? error.message : String(error);
          append("stderr", `\n[host] ${text.slice(0, 2048)}`);
        }
      }
      Atomics.store(state, 1, result | 0);
      Atomics.store(state, 0, status);
      Atomics.notify(state, 0);
      try { onTrace?.({ module: message.module, name: message.name, args: message.args, status, result: result | 0 }); }
      catch (error) { finish(error instanceof Error ? error : new Error(String(error))); }
      if (status === 2) finish(undefined, result | 0);
    };

    let moduleObject;
    let imports;
    const receiveRpc = (event, sourceWorker, rpc, isThread = false) => {
      if (event.data?.type === "rpc") handleRpc(event, sourceWorker, rpc, isThread);
      else if (event.data?.type === "thread-done") {
        threads.get(event.data.tid)?.worker.terminate();
        threads.delete(event.data.tid);
      } else if (event.data?.type === "failure") {
        const error = new Error(`WASI thread failed: ${String(event.data.error).slice(0, 2048)}`);
        finish(error);
      }
    };
    const receive = (event) => {
      const message = event.data;
      if (message?.type === "progress") {
        if (!settled && !signal?.aborted) {
          if (message.state === "start" || message.state === "update") activePhase = message.phase;
          else if (message.state === "complete" || message.state === "error" || message.state === "cancelled") activePhase = null;
          notifyProgress(onProgress, message);
        }
      } else if (message?.type === "ready") {
        try {
          moduleObject = message.module;
          imports = message.imports;
          const memoryObject = message.memory;
          wasi = new WASI(args, [], [new OpenFile(new File(new Uint8Array(0), { readonly: true })),
            new (class extends OpenFile { constructor() { super(new File(new Uint8Array(0))); this.fd_write = (bytes) => { const text = new TextDecoder().decode(bytes); append("stdout", text); return { ret: 0, nwritten: bytes.byteLength }; }; } })(),
            new (class extends OpenFile { constructor() { super(new File(new Uint8Array(0))); this.fd_write = (bytes) => { const text = new TextDecoder().decode(bytes); append("stderr", text); return { ret: 0, nwritten: bytes.byteLength }; }; } })(),
            new PreopenDirectory("/", fs.root.contents),
          ]);
          wasi.inst = { exports: { memory: memoryObject } };
          // The shim's synchronous poll implementation busy-waits on the UI thread.
          // Fail boundedly instead of freezing the page for arbitrary guest timeouts.
          wasi.wasiImport.poll_oneoff = () => 58;
          wasi.wasiImport.fd_write = boundedFdWrite(wasi.wasiImport.fd_write.bind(wasi.wasiImport));
          const rpc = new SharedArrayBuffer(8);
          const state = new Int32Array(rpc);
          worker.postMessage({ type: "start", rpc });
          workerRpc = { rpc, state };
          initializeProgress("complete");
        } catch (error) {
          const failure = error instanceof Error ? error : new Error(String(error));
          initializeProgress("error", failure.message);
          finish(failure);
        }
      } else if (message?.type === "result") finish(undefined, message.exitCode);
      else if (message?.type === "failure") {
        const error = new Error(`WASI compiler failed: ${String(message.error).slice(0, 2048)}`);
        finish(error);
      }
      else if (message?.type === "output") append(message.stream, message.text);
      else if (message?.type === "rpc") handleRpc(event, worker, workerRpc.rpc, false);
      else if (message?.type === "thread-done") {
        threads.get(message.tid)?.worker.terminate();
        threads.delete(message.tid);
      }
    };
    let workerRpc;
    const append = (stream, text) => {
      if (stream !== "stdout" && stream !== "stderr" || typeof text !== "string") return finish(new Error("WASI compiler returned invalid output"));
      outputBytes += encoder.encode(text).byteLength;
      if (outputBytes > MAX_OUTPUT_BYTES) return finish(new Error(`WASI compiler output exceeds ${MAX_OUTPUT_BYTES} bytes`));
      output[stream] += text;
      try { onOutput?.({ stream, text }); } catch (error) { finish(error instanceof Error ? error : new Error(String(error))); }
    };

    function refreshMemory() {
      // A cloned shared-memory wrapper can retain its earlier buffer view after
      // another worker grows the memory. grow(0) refreshes that view.
      wasi.inst.exports.memory.grow(0);
    }
    function memoryView() { refreshMemory(); return new DataView(wasi.inst.exports.memory.buffer); }
    function pathArgument(pointer, length) {
      const bytes = new Uint8Array(wasi.inst.exports.memory.buffer);
      if (length > MAX_PATH_BYTES) throw new RangeError(`WASI path exceeds ${MAX_PATH_BYTES} bytes`);
      if (pointer < 0 || length < 0 || pointer + length > bytes.byteLength) throw new RangeError("invalid WASI path buffer");
      return new TextDecoder("utf-8", { fatal: true }).decode(bytes.slice(pointer, pointer + length));
    }
    function resolvePath(fd, pointer, length) {
      const base = fdPaths.get(fd);
      if (base === undefined) throw new Error(`unknown virtual directory descriptor ${fd}`);
      const stack = base.split("/").filter(Boolean);
      for (const part of pathArgument(pointer, length).replaceAll("\\", "/").split("/")) {
        if (!part || part === ".") continue;
        if (part === "..") {
          if (stack.length === 0) throw new Error("virtual path escapes root");
          stack.pop();
        } else stack.push(part);
      }
      return `/${stack.join("/")}`;
    }
    function pathReadonly(path) { return path === "/sysroot" || path.startsWith("/sysroot/"); }
    function iovecs(pointer, count) {
      if (!Number.isInteger(count) || count < 0 || count > 1024) throw new RangeError("WASI iovec count exceeds 1024");
      const view = memoryView();
      if (pointer < 0 || pointer + count * 8 > view.byteLength) throw new RangeError("invalid WASI iovec table");
      const buffers = [];
      let bytes = 0;
      for (let index = 0; index < count; index++) {
        const start = view.getUint32(pointer + index * 8, true);
        const length = view.getUint32(pointer + index * 8 + 4, true);
        if (start + length > view.byteLength) throw new RangeError("invalid WASI iovec buffer");
        bytes += length;
        buffers.push({ start, length });
      }
      return { buffers, bytes };
    }
    function checkWasiMutation(name, args) {
      try {
        let writeSnapshot;
        let writeVector;
        if (["path_create_directory", "path_unlink_file", "path_remove_directory"].includes(name)) {
          if (pathReadonly(resolvePath(args[0], args[1], args[2]))) return wasiDefs.ERRNO_ROFS;
        } else if (name === "path_open") {
          const path = resolvePath(args[0], args[2], args[3]);
          const oflags = args[4];
          if (pathReadonly(path) && (oflags & (wasiDefs.OFLAGS_CREAT | wasiDefs.OFLAGS_EXCL | wasiDefs.OFLAGS_TRUNC))) return wasiDefs.ERRNO_ROFS;
        } else if (name === "path_link") {
          if (pathReadonly(resolvePath(args[0], args[2], args[3])) || pathReadonly(resolvePath(args[4], args[5], args[6]))) return wasiDefs.ERRNO_ROFS;
        } else if (name === "path_rename") {
          if (pathReadonly(resolvePath(args[0], args[1], args[2])) || pathReadonly(resolvePath(args[3], args[4], args[5]))) return wasiDefs.ERRNO_ROFS;
        } else if (["fd_write", "fd_pwrite", "fd_filestat_set_size", "fd_allocate"].includes(name)) {
          const descriptor = args[0];
          if (pathReadonly(fdPaths.get(descriptor) ?? "")) return wasiDefs.ERRNO_ROFS;
          const file = wasi.fds[descriptor]?.file;
          if (!(file instanceof File)) return undefined;
          const current = file.data.byteLength;
          let end = current;
          if (name === "fd_write" || name === "fd_pwrite") {
            const vector = iovecs(args[1], args[2]);
            if (name === "fd_write" && (descriptor === 1 || descriptor === 2) && outputBytes + vector.bytes > MAX_OUTPUT_BYTES) return wasiDefs.ERRNO_FBIG;
            writeVector = name === "fd_write" ? vector : undefined;
            const start = name === "fd_write" ? wasi.fds[descriptor].file_pos : BigInt(args[3]);
            const length = BigInt(vector.bytes);
            end = Number(start + length > BigInt(current) ? start + length : BigInt(current));
          } else if (name === "fd_filestat_set_size") end = Number(args[1]);
          else {
            const size = BigInt(args[1]) + BigInt(args[2]);
            end = Number(size > BigInt(current) ? size : BigInt(current));
          }
          if (!Number.isSafeInteger(end) || end > MAX_FILE_BYTES) return wasiDefs.ERRNO_FBIG;
          if (descriptor !== 1 && descriptor !== 2 && filesystemBytes(fs.root) - current + end > MAX_TOTAL_BYTES) return wasiDefs.ERRNO_NOSPC;
        }
        if (writeVector) writeSnapshot = {
          kind: "fd-write",
          chunks: writeVector.buffers.map(({ start: pointer, length: byteLength }) => new Uint8Array(wasi.inst.exports.memory.buffer, pointer, byteLength).slice()),
        };
        return writeSnapshot;
      } catch (error) {
        append("stderr", `[host] ${name} preflight failed: ${error instanceof Error ? error.message : String(error)}\n`);
        return wasiDefs.ERRNO_FAULT;
      }
    }
    function boundedFdWrite(original) {
      return (fd, iovsPointer, iovsCount, writtenPointer, snapshot) => {
        if (!snapshot || snapshot.kind !== "fd-write") return original(fd, iovsPointer, iovsCount, writtenPointer);
        const descriptor = wasi.fds[fd];
        if (!descriptor) return wasiDefs.ERRNO_BADF;
        let written = 0;
        for (const chunk of snapshot.chunks) {
          const result = descriptor.fd_write(chunk);
          if (result.ret !== wasiDefs.ERRNO_SUCCESS) {
            memoryView().setUint32(writtenPointer, written, true);
            return result.ret;
          }
          written += result.nwritten;
          if (result.nwritten !== chunk.byteLength) break;
        }
        memoryView().setUint32(writtenPointer, written, true);
        return wasiDefs.ERRNO_SUCCESS;
      };
    }

    worker.addEventListener("message", receive);
    worker.addEventListener("error", onError);
    signal?.addEventListener("abort", abort, { once: true });
    if (signal?.aborted) { abort(); return; }
    timer = setTimeout(() => finish(new DOMException(`WASI compiler timed out after ${timeoutMs} ms`, "TimeoutError")), timeoutMs);
    const isolatedModule = moduleBytes.slice();
    try {
      worker.postMessage({ type: "compile", bytes: isolatedModule.buffer, memory: { initialPages, maximumPages } }, [isolatedModule.buffer]);
    } catch (error) {
      const failure = error instanceof Error ? error : new Error(String(error));
      if (activePhase === "initialize") initializeProgress("error", failure.message);
      finish(failure);
    }
  });
}
