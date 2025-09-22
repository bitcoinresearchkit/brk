const imports = {
  async signals() {
    return import("./packages/solidjs-signals/wrapper.js").then(
      (d) => d.default,
    );
  },
  async chart() {
    return window.document.fonts.ready.then(() =>
      import("./core/chart.js").then((d) => d.default),
    );
  },
  async leanQr() {
    return import("./packages/lean-qr/2.5.0/index.mjs").then((d) => d);
  },
  async ufuzzy() {
    return import("./packages/leeoniya-ufuzzy/1.0.19/dist/uFuzzy.mjs").then(
      ({ default: d }) => d,
    );
  },
  async modernScreenshot() {
    return import("./packages/modern-screenshot/wrapper.js").then((d) => d);
  },

  async options() {
    return import("./core/options/full.js").then((d) => d);
  },
  async vecs() {
    return import("./bridge/vecs.js").then((d) => d);
  },
  async pools() {
    return import("./bridge/pools.js").then((d) => d);
  },
};

/**
 * @template {keyof typeof imports} K
 * @param {K} key
 */
function lazyImport(key) {
  /** @type {any | null} */
  let packagePromise = null;

  return function () {
    if (!packagePromise) {
      packagePromise = imports[key]();
    }
    return /** @type {ReturnType<typeof imports[K]>} */ (packagePromise);
  };
}

export default /** @type {{ [K in keyof typeof imports]: () => ReturnType<typeof imports[K]> }} */ (
  Object.fromEntries(
    Object.keys(imports).map((key) => [
      key,
      lazyImport(/** @type {keyof typeof imports} */ (key)),
    ]),
  )
);
