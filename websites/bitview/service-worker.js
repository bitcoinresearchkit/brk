// DO NOT CHANGE, Exact format is expected in `brk_bundler`
const CACHE_VERSION = "__VERSION__";

const SHELL_FILES = ["/", "/index.html"];

/** @type {ServiceWorkerGlobalScope} */
const sw = /** @type {any} */ (self);

sw.addEventListener("install", (event) => {
  console.log("sw: install");
  event.waitUntil(
    caches
      .open(CACHE_VERSION)
      .then((c) => c.addAll(SHELL_FILES))
      .then(() => sw.skipWaiting()),
  );
});

sw.addEventListener("activate", (event) => {
  console.log("sw: active");
  event.waitUntil(
    Promise.all([
      sw.clients.claim(),
      caches
        .keys()
        .then((keys) =>
          Promise.all(
            keys
              .filter((key) => key !== CACHE_VERSION)
              .map((key) => caches.delete(key)),
          ),
        ),
    ]),
  );
});

async function indexHTMLOrOffline() {
  return caches.match("/index.html").then((cached) => {
    if (cached) return cached;
    return new Response("Offline and no cached version", {
      status: 503,
      statusText: "Service Unavailable",
      headers: { "Content-Type": "text/plain" },
    });
  });
}

sw.addEventListener("fetch", (event) => {
  const req = event.request;
  const url = new URL(req.url);

  // 1) Bypass API calls & non-GETs
  if (
    req.method !== "GET" ||
    url.pathname.startsWith("/api") ||
    url.pathname === "/changelog" ||
    url.pathname === "/crate" ||
    url.pathname === "/discord" ||
    url.pathname === "/github" ||
    url.pathname === "/health" ||
    url.pathname === "/install" ||
    url.pathname === "/mcp" ||
    url.pathname === "/nostr" ||
    url.pathname === "/service" ||
    url.pathname === "/status" ||
    url.pathname === "/version"
  ) {
    return; // let the browser handle it
  }

  const cache = caches.open(CACHE_VERSION);

  // 2) NAVIGATION: network‐first on your shell
  if (req.mode === "navigate") {
    event.respondWith(
      // Always fetch index.html
      fetch("/index.html")
        .then((response) => {
          // If we got a valid 2xx back, cache it (optional) and return it
          if (response.ok || response.status === 304) {
            if (response.ok) {
              const clone = response.clone();
              cache.then((cache) => cache.put("/index.html", clone));
            }
            return response;
          }
          throw new Error("Non-2xx on shell");
        })
        // On any failure, fall back to the cached shell
        .catch(indexHTMLOrOffline),
    );
    return;
  }

  // 3) For all other GETs: network-first, fallback to cache
  event.respondWith(
    fetch(req)
      .then((response) => {
        if (response.ok) {
          const clone = response.clone();
          cache.then((cache) => cache.put(req, clone));
        }
        return response;
      })
      .catch(async () =>
        caches
          .match(req)
          .then((cached) => {
            return cached || indexHTMLOrOffline();
          })
          .catch(indexHTMLOrOffline),
      ),
  );
});
