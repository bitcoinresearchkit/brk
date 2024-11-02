// @ts-check

const version = "v1";

self.addEventListener("install", (_event) => {
  console.log("service-worker: install");

  const event = /** @type {any} */ (_event);

  event.waitUntil(
    caches.open(version).then((cache) => {
      return cache.addAll([
        "/",
        "/index.html",
        "/assets/fonts/satoshi/2024-09/font.var.woff2",
        "/scripts/main.js",
        "/scripts/options.js",
        "/scripts/chart.js",
        "/styles/chart.css",
        "/packages/lean-qr/v2.3.4/script.js",
        "/packages/lightweight-charts/v4.2.0/script.js",
        "/packages/solid-signals/2024-11-01/script.js",
        "/packages/ufuzzy/v1.0.14/script.js",
      ]);
    }),
  );

  // @ts-ignore
  self.skipWaiting();
});

self.addEventListener("fetch", (_event) => {
  const event = /** @type {any} */ (_event);

  /** @type {Request} */
  let request = event.request;
  const method = request.method;
  let url = request.url;

  const { pathname, origin } = new URL(url);

  const slashMatches = url.match(/\//g);
  const dotMatches = pathname.split("/").at(-1)?.match(/./g);
  const endsWithDotHtml = pathname.endsWith(".html");
  const slashApiSlashMatches = url.match(/\/api\//g);

  if (
    slashMatches &&
    slashMatches.length <= 3 &&
    !slashApiSlashMatches &&
    (!dotMatches || endsWithDotHtml)
  ) {
    url = `${origin}/`;
  }
  request = new Request(url, request.mode !== "navigate" ? request : undefined);

  console.log(`service-worker: fetching: ${url}`);

  event.respondWith(
    caches.match(request).then(async (cachedResponse) => {
      return fetch(request)
        .then((response) => {
          const { status } = response;

          if (method !== "GET" || slashApiSlashMatches) {
            // API calls are cached in script.js
            return response;
          } else if (status === 200 || status === 304) {
            if (status === 200) {
              const clonedResponse = response.clone();
              caches.open(version).then((cache) => {
                cache.put(request, clonedResponse);
              });
            }
            return response;
          } else {
            return cachedResponse || response;
          }
        })
        .catch(() => {
          console.log("service-worker: offline");

          return cachedResponse;
        });
    }),
  );
});
