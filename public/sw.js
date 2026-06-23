const TILE_CACHE = "river-map-tiles-v1";
const TILE_CACHE_PREFIX = "river-map-tiles-";
const MAX_TILE_CACHE_ENTRIES = 600;

self.addEventListener("install", (event) => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key.startsWith(TILE_CACHE_PREFIX) && key !== TILE_CACHE)
            .map((key) => caches.delete(key)),
        ),
      )
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || !isTileRequest(event.request)) {
    return;
  }

  event.respondWith(cacheTile(event.request));
});

function isTileRequest(request) {
  const url = new URL(request.url);

  if (
    url.hostname === "cyberjapandata.gsi.go.jp" &&
    url.pathname.startsWith("/xyz/")
  ) {
    return true;
  }

  return (
    (url.hostname === "tile.openstreetmap.org" ||
      url.hostname.endsWith(".tile.openstreetmap.org")) &&
    /^\/\d+\/\d+\/\d+\.png$/.test(url.pathname)
  );
}

async function cacheTile(request) {
  const cache = await caches.open(TILE_CACHE);
  const cached = await cache.match(request);
  if (cached) {
    return cached;
  }

  const response = await fetch(request);
  if (response.status === 200 || response.type === "opaque") {
    await cache.put(request, response.clone());
    await trimCache(cache);
  }

  return response;
}

async function trimCache(cache) {
  const keys = await cache.keys();
  if (keys.length <= MAX_TILE_CACHE_ENTRIES) {
    return;
  }

  await Promise.all(
    keys
      .slice(0, keys.length - MAX_TILE_CACHE_ENTRIES)
      .map((request) => cache.delete(request)),
  );
}
