import { scanWalletAddresses } from "../scan/index.js";

/**
 * @typedef {import("../scan/index.js").WalletScan} WalletScan
 * @typedef {import("../scan/index.js").WalletScanClient} WalletScanClient
 * @typedef {import("../scan/index.js").WalletScanProgress} WalletScanProgress
 *
 * @typedef {Object} LoadOptions
 * @property {WalletScanClient} client
 * @property {(progress: WalletScanProgress) => void} [onProgress]
 */

/**
 * @param {string} source
 */
export function createRuntime(source) {
  /** @type {WalletScan | undefined} */
  let scan;
  /** @type {Promise<WalletScan> | undefined} */
  let pending;

  /**
   * @param {LoadOptions} options
   */
  function load(options) {
    if (scan) return Promise.resolve(scan);

    if (!pending) {
      pending = scanWalletAddresses({
        client: options.client,
        source,
        onProgress: options.onProgress,
      }).then((nextScan) => {
        scan = nextScan;
        pending = undefined;

        return nextScan;
      }, (error) => {
        pending = undefined;

        throw error;
      });
    }

    return pending;
  }

  return {
    get scan() {
      return scan;
    },
    load,
  };
}
