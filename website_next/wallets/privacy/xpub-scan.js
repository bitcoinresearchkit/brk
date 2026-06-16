import { fetchWalletAddresses } from "./address-lookup.js";
import { generateAddressesFromWalletSource } from "../xpub/index.js";

export const XPUB_GAP_LIMIT = 10;

const SCAN_BATCH_SIZE = XPUB_GAP_LIMIT;
const MAX_SCANNED_ADDRESSES = 1_000;

/**
 * @typedef {import("../xpub/address.js").AddressScript} AddressScript
 * @typedef {import("../xpub/index.js").AddressType} AddressType
 */

/**
 * @typedef {Object} AddressClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 */

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {string} script
 * @property {string} network
 * @property {AddressType} addrType
 * @property {number} balance
 * @property {number} received
 * @property {number} sent
 * @property {number} txCount
 * @property {number | undefined} typeIndex
 * @property {string[]} historyAddresses
 * @property {number} historyBucketSize
 */

/**
 * @typedef {Object} ScanProgress
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @typedef {Object} ScanXpubOptions
 * @property {number} start
 * @property {AddressScript} script
 * @property {readonly number[]} path
 * @property {string} [branchId]
 * @property {(progress: ScanProgress) => void} [onProgress]
 */

/**
 * @typedef {Object} ScanXpubResult
 * @property {WalletAddress[]} addresses
 * @property {number} scannedCount
 * @property {number} gapLimit
 * @property {boolean} maxed
 */

/**
 * @param {WalletAddress} address
 */
function isUsedAddress(address) {
  return address.received > 0 || address.sent > 0 || address.txCount > 0;
}

/**
 * @param {AddressClient} client
 * @param {string} xpub
 * @param {ScanXpubOptions} options
 * @returns {Promise<ScanXpubResult>}
 */
export async function scanXpubWallet(client, xpub, options) {
  const addresses = /** @type {WalletAddress[]} */ ([]);
  let unusedInRow = 0;
  let nextStart = options.start;

  while (
    unusedInRow < XPUB_GAP_LIMIT &&
    addresses.length < MAX_SCANNED_ADDRESSES
  ) {
    const count = Math.min(
      SCAN_BATCH_SIZE,
      XPUB_GAP_LIMIT - unusedInRow,
      MAX_SCANNED_ADDRESSES - addresses.length,
    );
    const generated = await generateAddressesFromWalletSource(xpub, {
      start: nextStart,
      count,
      script: options.script,
      path: options.path,
      branchId: options.branchId,
    });
    const batch = /** @type {WalletAddress[]} */ (
      await fetchWalletAddresses(client, generated)
    );

    for (const address of batch) {
      addresses.push(address);
      unusedInRow = isUsedAddress(address) ? 0 : unusedInRow + 1;

      if (unusedInRow >= XPUB_GAP_LIMIT) {
        break;
      }
    }

    nextStart += count;
    options.onProgress?.({
      scannedCount: addresses.length,
      unusedInRow,
    });
  }

  return {
    addresses,
    scannedCount: addresses.length,
    gapLimit: XPUB_GAP_LIMIT,
    maxed: addresses.length >= MAX_SCANNED_ADDRESSES,
  };
}
