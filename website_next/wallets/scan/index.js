import { scanBranches } from "./branches.js";

/**
 * @typedef {import("../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../derive/index.js").AddressType} AddressType
 * @typedef {Awaited<ReturnType<typeof scanBranches>>["addresses"][number]} WalletAddress
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

/**
 * @typedef {Object} WalletScanProgress
 * @property {string} branchLabel
 * @property {number} scannedCount
 * @property {number} unusedInRow
 */

/**
 * @param {Object} options
 * @param {WalletScanClient} options.client
 * @param {string} options.source
 * @param {AddressScript} options.script
 * @param {(progress: WalletScanProgress) => void} [options.onProgress]
 * @returns {Promise<WalletScan>}
 */
export async function scanWalletAddresses({
  client,
  source,
  script,
  onProgress,
}) {
  const scan = await scanBranches(client, source, {
    script,
    onProgress,
  });
  const addresses = /** @type {WalletAddress[]} */ (scan.addresses);
  const btcUsdPrice = /** @type {number} */ (
    await client.getLivePrice({ cache: false })
  );

  return {
    addresses,
    receiveAddress: scan.receiveAddress,
    btcUsdPrice,
  };
}
