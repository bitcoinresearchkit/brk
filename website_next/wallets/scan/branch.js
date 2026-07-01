import { fetchWalletAddresses } from "../lookup/index.js";
import { generateAddressesFromWalletSource } from "../derive/index.js";

export const GAP_LIMIT = 10;

const SCAN_BATCH_SIZE = GAP_LIMIT;
const MAX_SCANNED_ADDRESSES = 1_000;

/**
 * @typedef {import("../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../lookup/index.js").AddressClient} AddressClient
 * @typedef {import("../lookup/index.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} ScanProgress
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @typedef {Object} ScanOptions
 * @property {AddressScript} script
 * @property {readonly number[]} path
 * @property {string} [branchId]
 * @property {(progress: ScanProgress) => void} [onProgress]
 */

/**
 * @typedef {Object} ScanResult
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
 * @param {string} source
 * @param {ScanOptions} options
 * @returns {Promise<ScanResult>}
 */
export async function scanBranch(client, source, options) {
  const addresses = /** @type {WalletAddress[]} */ ([]);
  let unusedInRow = 0;
  let nextStart = 0;

  while (
    unusedInRow < GAP_LIMIT &&
    addresses.length < MAX_SCANNED_ADDRESSES
  ) {
    const count = Math.min(
      SCAN_BATCH_SIZE,
      GAP_LIMIT - unusedInRow,
      MAX_SCANNED_ADDRESSES - addresses.length,
    );
    const generated = await generateAddressesFromWalletSource(source, {
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

      if (unusedInRow >= GAP_LIMIT) {
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
    gapLimit: GAP_LIMIT,
    maxed: addresses.length >= MAX_SCANNED_ADDRESSES,
  };
}
