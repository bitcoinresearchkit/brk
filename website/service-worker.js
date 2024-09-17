// @ts-check

const version = "v1";

self.addEventListener("install", (_event) => {
  const event = /** @type {any} */ (_event);

  event.waitUntil(
    caches.open(version).then((cache) => {
      return cache.addAll([
        "/",
        "/index.html",
        "/script.js",
        "/packages/solid-signals/2024-04-17/script.js",
        "/packages/ufuzzy/2024-02-21/script.js",
        "/packages/lean-qr/v2.3.4/script.js",
        "/packages/lightweight-charts/v4.2.0/script.js",
        "/fonts/satoshi/2024-09/font.var.woff2",
      ]);
    })
  );

  // @ts-ignore
  self.skipWaiting();
});

/**
 * @param {Response | undefined} cachedResponse
 * @param {Response | undefined} badResponse
 */
function pickCorrectResponse(cachedResponse, badResponse) {
  if (cachedResponse) {
    return cachedResponse;
  } else {
    return caches
      .match("/")
      .then((response) => {
        return response ?? badResponse;
      })
      .catch(() => {
        return badResponse;
      });
  }
}

self.addEventListener("fetch", (_event) => {
  const event = /** @type {any} */ (_event);

  /** @type {Request} */
  const request = event.request;
  const { url, method } = request;

  event.respondWith(
    caches.match(request).then((cachedResponse) => {
      return fetch(request)
        .then((response) => {
          // @ts-ignore
          if (url.includes("/api/")) {
            return response;
          }

          return caches.open(version).then((cache) => {
            if (response.status === 200) {
              if (method === "GET") {
                cache.put(request, response.clone());
              }
              return response;
            }
            return pickCorrectResponse(cachedResponse, response);
          });
        })
        .catch(() => {
          return pickCorrectResponse(cachedResponse, undefined);
        });
    })
  );
});
