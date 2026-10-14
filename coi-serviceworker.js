/*
 * Same-origin COOP/COEP response headers for the /blr/ Pages project path.
 * This is a small project implementation; it does not copy third-party code.
 */
"use strict";

const scopePath = new URL(self.registration.scope).pathname;
const runtimeCacheName = `blr-runtime-assets:${scopePath}`;
const compilerCacheName = `blr-compiler-assets:${scopePath}`;
/* BLR_ASSET_PINS_BEGIN */
const runtimeAssets = new Map([
  [`${scopePath}runtime/blender.js`, { size: 224813, sha256: "562cce8059f290b3c34459b9ca5c1688a9c9664a317b7bac0efdd2f77f81f217" }],
  [`${scopePath}runtime/blender.wasm.gz`, { size: 33041621, sha256: "8c7c8ee8720e06c6415ca2c6456c77778e2c564e1345cd40a4cff8e5d57ee901" }],
]);
const maxRuntimeCacheBytes = 34603008;
const compilerAssets = new Map([
  [`${scopePath}compiler/cache-manifest.json`, { size: 670, sha256: "e5ff1d6050b7181acef21c773a35ed85049354769a2c928548a1022423327494" }],
  [`${scopePath}compiler/assets/rustc_opt.wasm.br`, { size: 19774341, sha256: "10f06919521c61f177b432ae7fd08cc3d4547efc3b13c2f547560fb53cf05ca6" }],
  [`${scopePath}compiler/assets/wasm32-wasip1.tar.br`, { size: 18933366, sha256: "bf29ba33cbf6267c10a1c1838edfff76d19750287159adfa08ab308db922bfdb" }],
  [`${scopePath}compiler/brotli-dec-wasm/pkg/brotli_dec_wasm_bg.wasm`, { size: 206243, sha256: "7cb4663fd0b11185b88177d0b3b3048136572faf49e81c4c78dccafe37bb68d7" }],
]);
const maxCompilerCacheBytes = 83886080;
/* BLR_ASSET_PINS_END */
const runtimeCacheBytes = [...runtimeAssets.values()].reduce((total, asset) => total + asset.size, 0);
const compilerCacheBytes = [...compilerAssets.values()].reduce((total, asset) => total + asset.size, 0);

self.addEventListener("install", (event) => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

function withIsolationHeaders(response) {
  if (response.type === "opaque" || response.type === "opaqueredirect" || response.status === 0) {
    return response;
  }

  const headers = new Headers(response.headers);
  headers.set("Cross-Origin-Opener-Policy", "same-origin");
  headers.set("Cross-Origin-Embedder-Policy", "require-corp");
  headers.set("Cross-Origin-Resource-Policy", "same-origin");
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

function exactRuntimeAsset(request, requestUrl) {
  const asset = runtimeAssets.get(requestUrl.pathname);
  if (
    !asset ||
    request.method !== "GET" ||
    request.mode === "navigate" ||
    requestUrl.search !== `?v=${asset.sha256}` ||
    request.headers.has("range")
  ) {
    return null;
  }
  return asset;
}

function runtimeCacheKey(pathname, asset) {
  return `${self.location.origin}${pathname}?v=${asset.sha256}`;
}

function compilerCacheKey(pathname, asset) {
  return `${self.location.origin}${pathname}?v=${asset.sha256}`;
}

async function trimRuntimeCache(cache) {
  if (runtimeCacheBytes > maxRuntimeCacheBytes) {
    throw new Error("declared runtime cache exceeds its 33 MiB bound");
  }
  const allowed = new Set([...runtimeAssets].map(([pathname, asset]) => runtimeCacheKey(pathname, asset)));
  for (const request of await cache.keys()) {
    if (!allowed.has(request.url)) await cache.delete(request);
  }
}

async function trimCompilerCache(cache) {
  if (compilerCacheBytes > maxCompilerCacheBytes) {
    throw new Error("declared compiler cache exceeds its 80 MiB bound");
  }
  const allowed = new Set([...compilerAssets].map(([pathname, asset]) => compilerCacheKey(pathname, asset)));
  for (const request of await cache.keys()) {
    if (!allowed.has(request.url)) await cache.delete(request);
  }
}

async function readBoundedBody(body, expectedSize) {
  if (!body) return null;
  const reader = body.getReader();
  const bytes = new Uint8Array(expectedSize);
  let offset = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      if (value.byteLength > expectedSize - offset) {
        reader.cancel().catch(() => {});
        return null;
      }
      bytes.set(value, offset);
      offset += value.byteLength;
    }
  } finally {
    reader.releaseLock();
  }
  return offset === expectedSize ? bytes : null;
}

async function cachedResponseMatchesPin(response, asset) {
  if (!response || response.status !== 200 || response.headers.has("content-range")) return false;
  const declared = response.headers.get("content-length");
  if (declared !== null && declared !== String(asset.size)) return false;
  const bytes = await readBoundedBody(response.clone().body, asset.size);
  if (!bytes) return false;
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  const hexDigest = [...digest].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  return hexDigest === asset.sha256;
}

async function cacheVerifiedRuntimeResponse(cache, request, response, asset) {
  if (
    request.signal.aborted ||
    response.status !== 200 ||
    response.headers.get("content-length") !== String(asset.size) ||
    response.headers.has("content-range")
  ) {
    return;
  }

  const candidate = response.clone();
  const bytes = await readBoundedBody(candidate.body, asset.size);
  if (!bytes || request.signal.aborted) return;
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  const hexDigest = [...digest].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  if (hexDigest !== asset.sha256 || request.signal.aborted) return;

  const cachedResponse = withIsolationHeaders(new Response(bytes, {
    status: response.status,
    statusText: response.statusText,
    headers: response.headers,
  }));
  await cache.put(request, cachedResponse);
}

async function cacheVerifiedCompilerResponse(cache, pathname, request, response, asset) {
  if (
    request.signal.aborted ||
    response.status !== 200 ||
    response.headers.has("content-range")
  ) {
    return;
  }

  const candidate = response.clone();
  const bytes = await readBoundedBody(candidate.body, asset.size);
  if (!bytes || request.signal.aborted) return;
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  const hexDigest = [...digest].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  if (hexDigest !== asset.sha256 || request.signal.aborted) return;

  const cachedResponse = withIsolationHeaders(new Response(bytes, {
    status: response.status,
    statusText: response.statusText,
    headers: response.headers,
  }));
  await cache.put(compilerCacheKey(pathname, asset), cachedResponse);
}

async function fetchRuntimeAsset(event, asset) {
  let cache;
  try {
    cache = await caches.open(runtimeCacheName);
    await trimRuntimeCache(cache);
    const cached = await cache.match(event.request);
    if (cached && await cachedResponseMatchesPin(cached, asset)) return withIsolationHeaders(cached);
    if (cached) await cache.delete(event.request);
  } catch {
    // CacheStorage may be unavailable or over quota; the online path still works.
    cache = undefined;
  }

  const response = await fetch(event.request);
  if (
    cache &&
    response.type !== "opaque" &&
    response.type !== "opaqueredirect" &&
    response.status !== 0
  ) {
    const write = cacheVerifiedRuntimeResponse(cache, event.request, response, asset).catch(() => {});
    try {
      event.waitUntil(write);
    } catch {
      // A cache write is best-effort and must not delay or fail the runtime fetch.
    }
  }
  return withIsolationHeaders(response);
}

async function fetchCompilerAsset(event, asset) {
  const pathname = new URL(event.request.url).pathname;
  let cache;
  try {
    cache = await caches.open(compilerCacheName);
    await trimCompilerCache(cache);
    const key = compilerCacheKey(pathname, asset);
    const cached = await cache.match(key);
    if (cached && await cachedResponseMatchesPin(cached, asset)) return withIsolationHeaders(cached);
    if (cached) await cache.delete(key);
  } catch {
    // CacheStorage may be unavailable or over quota; compilation can still fetch online.
    cache = undefined;
  }

  const response = await fetch(event.request);
  if (
    cache &&
    response.type !== "opaque" &&
    response.type !== "opaqueredirect" &&
    response.status !== 0
  ) {
    const write = cacheVerifiedCompilerResponse(cache, pathname, event.request, response, asset).catch(() => {});
    try {
      event.waitUntil(write);
    } catch {
      // Cache write is best-effort; do not delay or fail compiler asset loading.
    }
  }
  return withIsolationHeaders(response);
}

self.addEventListener("fetch", (event) => {
  const requestUrl = new URL(event.request.url);
  if (
    requestUrl.origin !== self.location.origin ||
    !requestUrl.pathname.startsWith(scopePath) ||
    (event.request.cache === "only-if-cached" && event.request.mode !== "same-origin")
  ) {
    return;
  }

  event.respondWith((async () => {
    const asset = exactRuntimeAsset(event.request, requestUrl);
    if (asset) return fetchRuntimeAsset(event, asset);
    const compilerAsset = exactCompilerAsset(event.request, requestUrl);
    if (compilerAsset) return fetchCompilerAsset(event, compilerAsset);
    return withIsolationHeaders(await fetch(event.request));
  })());
});

function exactCompilerAsset(request, requestUrl) {
  const asset = compilerAssets.get(requestUrl.pathname);
  if (
    !asset ||
    request.method !== "GET" ||
    request.mode === "navigate" ||
    requestUrl.search !== "" ||
    request.headers.has("range")
  ) {
    return null;
  }
  return asset;
}
