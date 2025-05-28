const version = "v1";

/** @type {ServiceWorkerGlobalScope} */
const sw = /** @type {any} */ (self);

sw.addEventListener("install", (_event) => {
  console.log("sw: install");
  sw.skipWaiting();
});

sw.addEventListener("activate", (event) => {
  console.log("sw: active");
  event.waitUntil(sw.clients.claim());
});

sw.addEventListener("fetch", (event) => {
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

  console.log(request);

  console.log(`service-worker: fetch ${url}`);

  event.respondWith(
    caches.match(request).then(async (cachedResponse) => {
      return fetch(request)
        .then((response) => {
          const { status, type } = response;

          if (method !== "GET" || slashApiSlashMatches) {
            // API calls are cached in script.js
            return response;
          } else if ((status === 200 || status === 304) && type === "basic") {
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

          return (
            cachedResponse ||
            new Response("Offline", {
              status: 503,
              statusText: "Service Unavailable",
            })
          );
        });
    }),
  );
});
