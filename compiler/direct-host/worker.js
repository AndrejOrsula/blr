const EXIT = Symbol("WASI process exit");
const THREAD_EXIT = Symbol("WASI thread exit");
const ALLOWED_WASI = new Set([
  "args_get", "args_sizes_get", "environ_get", "environ_sizes_get", "clock_time_get", "random_get",
  "fd_close", "fd_fdstat_get", "fd_fdstat_set_flags", "fd_filestat_get", "fd_filestat_set_size",
  "fd_filestat_set_times", "fd_pread", "fd_prestat_get", "fd_prestat_dir_name", "fd_read", "fd_readdir",
  "fd_seek", "fd_write", "path_create_directory", "path_filestat_get", "path_link", "path_open",
  "path_readlink", "path_remove_directory", "path_rename", "path_symlink", "path_unlink_file",
  "poll_oneoff", "proc_exit", "sched_yield",
]);
let rpc;
let rpcState;
let tid;
let isThread = false;

function callHost(module, name, args) {
  Atomics.store(rpcState, 0, 0);
  postMessage({ type: "rpc", module, name, args });
  Atomics.wait(rpcState, 0, 0);
  const status = Atomics.load(rpcState, 0);
  const value = Atomics.load(rpcState, 1);
  if (status === 2) {
    if (isThread) throw THREAD_EXIT;
    const error = new Error("WASI process exit");
    error.code = value;
    throw Object.assign(error, { [EXIT]: true });
  }
  if (status < 0 && (name === "wasi.thread-spawn" || name === "thread-spawn")) return -1;
  return value;
}

function importsFor(module, imports) {
  const result = {};
  for (const item of imports) {
    result[item.module] ??= {};
    result[item.module][item.name] = (...args) => callHost(item.module, item.name, args);
  }
  return result;
}

async function instantiate(module, memory, imports) {
  const namespaces = importsFor("", imports);
  namespaces.env ??= {};
  if (memory) namespaces.env.memory = memory;
  const instance = await WebAssembly.instantiate(module, namespaces);
  return instance;
}

self.onmessage = async ({ data }) => {
  try {
    if (data?.type === "compile") {
      const module = await WebAssembly.compile(data.bytes);
      const imports = WebAssembly.Module.imports(module);
      const memoryImports = imports.filter((item) => item.kind === "memory");
      for (const item of imports) {
        if (item.kind === "memory" && item.module === "env" && item.name === "memory") continue;
        if (item.kind === "function" && item.module === "wasi_snapshot_preview1" && ALLOWED_WASI.has(item.name)) continue;
        if (item.kind === "function" && item.module === "wasi" && item.name === "thread-spawn") continue;
        throw new Error(`unsupported compiler import ${item.module}.${item.name} (${item.kind})`);
      }
      const initial = data.memory?.initialPages ?? 399;
      const maximum = data.memory?.maximumPages ?? 16384;
      const memory = memoryImports.length ? new WebAssembly.Memory({ initial, maximum, shared: true }) : undefined;
      const moduleImports = imports.filter((item) => item.kind === "function").map((item) => ({ module: item.module, name: item.name, kind: item.kind }));
      const instance = await instantiate(module, memory, moduleImports);
      const moduleMemory = instance.exports.memory;
      if (!(moduleMemory instanceof WebAssembly.Memory) || !(moduleMemory.buffer instanceof SharedArrayBuffer)) {
        throw new Error("tool must expose shared WebAssembly memory");
      }
      if (typeof instance.exports._start !== "function") throw new Error("tool module lacks _start export");
      self.__pending = { module, instance, memory: moduleMemory, moduleImports };
      postMessage({ type: "ready", module, memory: moduleMemory, imports: moduleImports });
    } else if (data?.type === "thread") {
      isThread = true;
      tid = data.tid;
      rpc = data.rpc;
      rpcState = new Int32Array(rpc);
      const instance = await instantiate(data.module, data.memory, data.imports);
      if (typeof instance.exports.wasi_thread_start !== "function") throw new Error("thread worker lacks wasi_thread_start export");
      try { instance.exports.wasi_thread_start(data.tid, data.arg); } catch (error) { if (error !== THREAD_EXIT) throw error; }
      postMessage({ type: "thread-done", tid });
      close();
    } else if (data?.type === "start") {
      rpc = data.rpc;
      rpcState = new Int32Array(rpc);
      const { instance } = self.__pending;
      let exitCode = 0;
      try { instance.exports._start(); }
      catch (error) {
        if (error?.[EXIT]) exitCode = error.code;
        else if (error === THREAD_EXIT) exitCode = 0;
        else throw error;
      }
      postMessage({ type: "result", exitCode });
    }
  } catch (error) {
    postMessage({ type: "failure", error: error instanceof Error ? error.message : String(error) });
  }
};
