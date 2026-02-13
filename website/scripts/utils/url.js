/**
 * @param {string | string[]} [pathname]
 */
function processPathname(pathname) {
  pathname ||= window.location.pathname;
  const result = Array.isArray(pathname) ? pathname.join("/") : pathname;
  // Strip leading slash to avoid double slashes when prepending /
  return result.startsWith("/") ? result.slice(1) : result;
}

const chartParamsWhitelist = ["range"];

/**
 * @param {string | string[]} pathname
 */
export function pushHistory(pathname) {
  const urlParams = new URLSearchParams(window.location.search);
  pathname = processPathname(pathname);
  try {
    const url = `/${pathname}?${urlParams.toString()}`;
    window.history.pushState(null, "", url);
  } catch (_) {}
}

/**
 * @param {Object} args
 * @param {URLSearchParams} [args.urlParams]
 * @param {string | string[]} [args.pathname]
 */
export function replaceHistory({ urlParams, pathname }) {
  urlParams ||= new URLSearchParams(window.location.search);
  pathname = processPathname(pathname);
  try {
    const url = `/${pathname}?${urlParams.toString()}`;
    window.history.replaceState(null, "", url);
  } catch (_) {}
}

/**
 * @param {Option} option
 */
export function resetParams(option) {
  const urlParams = new URLSearchParams();
  if (option.kind === "chart") {
    [...new URLSearchParams(window.location.search).entries()]
      .filter(([key, _]) => chartParamsWhitelist.includes(key))
      .forEach(([key, value]) => {
        urlParams.set(key, value);
      });
  }
  replaceHistory({ urlParams, pathname: option.path.join("/") });
}

/**
 * @param {string} key
 * @param {string | boolean | null | undefined} value
 */
export function writeParam(key, value) {
  const urlParams = new URLSearchParams(window.location.search);

  if (value !== null && value !== undefined) {
    urlParams.set(key, String(value));
  } else {
    urlParams.delete(key);
  }

  replaceHistory({ urlParams });
}

/**
 *
 * @param {string} key
 * @returns {string | null}
 */
export function readParam(key) {
  const params = new URLSearchParams(window.location.search);
  return params.get(key);
}
