/**
 * @param {string | string[]} [pathname]
 */
function processPathname(pathname) {
  pathname ||= window.location.pathname;
  return Array.isArray(pathname) ? pathname.join("/") : pathname;
}

export default {
  chartParamsWhitelist: ["from", "to"],
  /**
   * @param {string | string[]} pathname
   */
  pushHistory(pathname) {
    const urlParams = new URLSearchParams(window.location.search);
    pathname = processPathname(pathname);
    try {
      const url = `/${pathname}?${urlParams.toString()}`;
      console.log(`push history: ${url}`);
      window.history.pushState(null, "", url);
    } catch (_) {}
  },
  /**
   * @param {Object} args
   * @param {URLSearchParams} [args.urlParams]
   * @param {string | string[]} [args.pathname]
   */
  replaceHistory({ urlParams, pathname }) {
    urlParams ||= new URLSearchParams(window.location.search);
    pathname = processPathname(pathname);
    try {
      const url = `/${pathname}?${urlParams.toString()}`;
      console.log(`replace history: ${url}`);
      window.history.replaceState(null, "", url);
    } catch (_) {}
  },
  /**
   * @param {Option} option
   */
  resetParams(option) {
    const urlParams = new URLSearchParams();
    if (option.kind === "chart") {
      [...new URLSearchParams(window.location.search).entries()]
        .filter(([key, _]) => this.chartParamsWhitelist.includes(key))
        .forEach(([key, value]) => {
          urlParams.set(key, value);
        });
    }
    this.replaceHistory({ urlParams, pathname: option.path.join("/") });
  },
  /**
   * @param {string} key
   * @param {string | boolean | null | undefined} value
   */
  writeParam(key, value) {
    const urlParams = new URLSearchParams(window.location.search);

    if (value !== null && value !== undefined) {
      urlParams.set(key, String(value));
    } else {
      urlParams.delete(key);
    }

    this.replaceHistory({ urlParams });
  },
  /**
   * @param {string} key
   */
  removeParam(key) {
    this.writeParam(key, undefined);
  },
  /**
   * @param {string} key
   */
  readBoolParam(key) {
    const param = this.readParam(key);
    if (param) {
      return param === "true" || param === "1";
    }
    return null;
  },
  /**
   * @param {string} key
   */
  readNumberParam(key) {
    const param = this.readParam(key);
    if (param) {
      return Number(param);
    }
    return null;
  },
  /**
   *
   * @param {string} key
   * @returns {string | null}
   */
  readParam(key) {
    const params = new URLSearchParams(window.location.search);
    return params.get(key);
  },
};
