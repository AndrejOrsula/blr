/*
 * Same-origin COOP/COEP response headers for the /blr/ Pages project path.
 * This is a small project implementation; it does not copy third-party code.
 */
"use strict";

const scopePath = new URL(self.registration.scope).pathname;
const runtimeCacheName = `blr-runtime-assets:${scopePath}`;
const runtimeAssets = new Map([
  [
    `${scopePath}runtime/blender.js`,
    { size: 224813, sha256: "562cce8059f290b3c34459b9ca5c1688a9c9664a317b7bac0efdd2f77f81f217" },
  ],
  [
    `${scopePath}runtime/blender.wasm.gz`,
    { size: 32746982, sha256: "3f53fd8c5c81036d7b62147493ca315db61baaafeaf776164c4a498341455d17" },
  ],
]);
const runtimeCacheBytes = [...runtimeAssets.values()].reduce((total, asset) => total + asset.size, 0);

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

async function trimRuntimeCache(cache) {
  if (runtimeCacheBytes > 33 * 1024 * 1024) {
    throw new Error("declared runtime cache exceeds its 33 MiB bound");
  }
  const allowed = new Set([...runtimeAssets].map(([pathname, asset]) => runtimeCacheKey(pathname, asset)));
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
        await reader.cancel();
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

async function fetchRuntimeAsset(event, asset) {
  let cache;
  try {
    cache = await caches.open(runtimeCacheName);
    await trimRuntimeCache(cache);
    const cached = await cache.match(event.request);
    if (cached) return withIsolationHeaders(cached);
  } catch {
    // CacheStorage may be unavailable or over quota; the online path still works.
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
    return withIsolationHeaders(await fetch(event.request));
  })());
});
