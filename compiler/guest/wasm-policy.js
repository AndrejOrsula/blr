export const MAX_MODULE_BYTES = 16 * 1024 * 1024;
export const MAX_MEMORY_BYTES = 256 * 1024 * 1024;
export const MAX_MEMORY_PAGES = MAX_MEMORY_BYTES / 65536;
export const MAX_TABLE_ELEMENTS = 262_144;
export const MAX_WASM_SECTIONS = 256;
export const MAX_OUTPUT_BYTES = 512 * 1024;
export const MAX_LINE_BYTES = 256 * 1024;

// Rust's WASI libc uses this subset. Filesystem path calls, sockets, poll on
// descriptors, and every non-WASI module import stay unavailable.
export const ALLOWED_WASI_IMPORTS = new Set([
  "args_get", "args_sizes_get", "clock_res_get", "clock_time_get",
  "environ_get", "environ_sizes_get", "fd_close", "fd_fdstat_get",
  "fd_fdstat_set_flags", "fd_fdstat_set_rights", "fd_filestat_get",
  "fd_read", "fd_seek", "fd_sync", "fd_tell", "fd_write",
  "poll_oneoff", "proc_exit", "proc_raise", "random_get", "sched_yield",
]);

function fail(message) {
  throw new Error(`guest rejected: ${message}`);
}

function readU32(bytes, state) {
  let value = 0;
  let shift = 0;
  for (let count = 0; count < 5; count++) {
    if (state.offset >= state.end) fail("truncated unsigned LEB128");
    const byte = bytes[state.offset++];
    value |= (byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) return value >>> 0;
    shift += 7;
  }
  fail("oversized unsigned LEB128");
}

function readName(bytes, state) {
  const length = readU32(bytes, state);
  if (length > state.end - state.offset) fail("truncated import name");
  const value = new TextDecoder("utf-8", { fatal: true }).decode(
    bytes.subarray(state.offset, state.offset + length),
  );
  state.offset += length;
  return value;
}

function readLimits(bytes, state) {
  const flags = readU32(bytes, state);
  // Only ordinary wasm32 memories with a declared maximum are accepted.
  if ((flags & ~1) !== 0 || (flags & 1) === 0) {
    fail("memory must be unshared wasm32 with a declared maximum");
  }
  const initialPages = readU32(bytes, state);
  const maximumPages = readU32(bytes, state);
  if (initialPages > maximumPages || maximumPages > MAX_MEMORY_PAGES) {
    fail(`memory limit exceeds ${MAX_MEMORY_BYTES} bytes`);
  }
  return { initialPages, maximumPages };
}

function readTableLimits(bytes, state) {
  const elementType = bytes[state.offset++];
  if (elementType === undefined) fail("truncated table import");
  const flags = readU32(bytes, state);
  if ((flags & ~1) !== 0 || (flags & 1) === 0) fail("unbounded table import");
  const initialElements = readU32(bytes, state);
  const maximumElements = readU32(bytes, state);
  if (initialElements > maximumElements || maximumElements > MAX_TABLE_ELEMENTS) {
    fail(`table limit exceeds ${MAX_TABLE_ELEMENTS} elements`);
  }
  return { initialElements, maximumElements };
}

export function inspectWasm(bytes) {
  if (!(bytes instanceof Uint8Array)) fail("input must be Uint8Array");
  if (bytes.byteLength === 0 || bytes.byteLength > MAX_MODULE_BYTES) {
    fail(`module size must be 1..${MAX_MODULE_BYTES} bytes`);
  }
  if (bytes.length < 8 || bytes[0] !== 0 || bytes[1] !== 0x61 || bytes[2] !== 0x73 || bytes[3] !== 0x6d || bytes[4] !== 1 || bytes[5] !== 0 || bytes[6] !== 0 || bytes[7] !== 0) {
    fail("invalid wasm header");
  }

  const imports = [];
  let memory;
  let table;
  let sawImports = false;
  let sawMemory = false;
  let sawTable = false;
  let sectionCount = 0;
  let offset = 8;
  while (offset < bytes.length) {
    if (++sectionCount > MAX_WASM_SECTIONS) fail(`too many wasm sections (maximum ${MAX_WASM_SECTIONS})`);
    const sectionId = bytes[offset++];
    const sectionState = { offset, end: bytes.length };
    const sectionSize = readU32(bytes, sectionState);
    const sectionStart = sectionState.offset;
    const sectionEnd = sectionStart + sectionSize;
    if (sectionEnd > bytes.length) fail("truncated section");
    offset = sectionEnd;
    if (sectionId === 0) continue;
    if (sectionId === 2) {
      if (sawImports) fail("duplicate import section");
      sawImports = true;
      const state = { offset: sectionStart, end: sectionEnd };
      const count = readU32(bytes, state);
      if (count > 256) fail("too many imports");
      for (let i = 0; i < count; i++) {
        const module = readName(bytes, state);
        const name = readName(bytes, state);
        if (state.offset >= state.end) fail("truncated import descriptor");
        const kind = bytes[state.offset++];
        if (module !== "wasi_snapshot_preview1" || kind !== 0 || !ALLOWED_WASI_IMPORTS.has(name)) {
          fail(`unsupported import ${module}.${name}`);
        }
        readU32(bytes, state); // function type index
        imports.push(`${module}.${name}`);
      }
      if (state.offset !== state.end) fail("malformed import section");
    } else if (sectionId === 4) {
      if (sawTable) fail("multiple table sections");
      sawTable = true;
      const state = { offset: sectionStart, end: sectionEnd };
      const count = readU32(bytes, state);
      if (count > 1) fail("at most one table is allowed");
      if (count === 1) table = readTableLimits(bytes, state);
      if (state.offset !== state.end) fail("malformed table section");
    } else if (sectionId === 5) {
      if (sawMemory) fail("multiple memory sections");
      sawMemory = true;
      const state = { offset: sectionStart, end: sectionEnd };
      const count = readU32(bytes, state);
      if (count !== 1) fail("exactly one memory is required");
      memory = readLimits(bytes, state);
      if (state.offset !== state.end) fail("malformed memory section");
    }
  }
  if (!memory) fail("a bounded memory section is required");
  return {
    imports: imports.sort(),
    table,
    memory: {
      initialPages: memory.initialPages,
      maximumPages: memory.maximumPages,
      maximumBytes: memory.maximumPages * 65536,
    },
  };
}
