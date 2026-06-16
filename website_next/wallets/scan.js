import {
  setBusy,
  setStatus,
} from "./dom.js";
import { getErrorMessage } from "./errors.js";
import { formatNumber } from "./format.js";
import {
  scanXpubBranches,
} from "./privacy/xpub-wallet.js";
import { XPUB_GAP_LIMIT } from "./privacy/xpub-scan.js";

/**
 * @typedef {import("./xpub/address.js").AddressScript} AddressScript
 * @typedef {import("./xpub/index.js").AddressType} AddressType
 * @typedef {Awaited<ReturnType<typeof scanXpubBranches>>["addresses"][number]} WalletAddress
 */

/**
 * @typedef {Object} WalletScan
 * @property {WalletAddress[]} addresses
 * @property {WalletAddress | undefined} receiveAddress
 * @property {number} btcUsdPrice
 */

/**
 * @typedef {Object} WalletScanClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 * @property {(options?: { cache?: boolean }) => Promise<unknown>} getLivePrice
 */

export function createScanPendingMessage() {
  return `Scanning until ${XPUB_GAP_LIMIT} unused addresses`;
}

/**
 * @param {Object} options
 * @param {WalletScanClient} options.client
 * @param {string} options.xpub
 * @param {number} options.start
 * @param {AddressScript} options.script
 * @param {HTMLButtonElement} [options.button]
 * @param {HTMLElement} options.status
 * @returns {Promise<WalletScan | undefined>}
 */
export async function scanWalletAddresses({
  client,
  xpub,
  start,
  script,
  button,
  status,
}) {
  if (button) {
    setBusy(button, true, "Scan", "Scanning");
  }
  setStatus(status, createScanPendingMessage());

  try {
    const scan = await scanXpubBranches(client, xpub, {
      start,
      script,
      onProgress(progress) {
        setStatus(
          status,
          `${progress.branchLabel}: scanned ${formatNumber(progress.scannedCount)} addresses, ${progress.unusedInRow}/${XPUB_GAP_LIMIT} unused`,
        );
      },
    });
    const addresses = /** @type {WalletAddress[]} */ (scan.addresses);
    const btcUsdPrice = /** @type {number} */ (
      await client.getLivePrice({ cache: false })
    );

    setStatus(status, "Ready");

    return {
      addresses,
      receiveAddress: scan.receiveAddress,
      btcUsdPrice,
    };
  } catch (error) {
    setStatus(status, getErrorMessage(error));
  } finally {
    if (button) {
      setBusy(button, false, "Scan", "Scanning");
    }
  }
}
