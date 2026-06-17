import { mapConcurrent } from "../../concurrent.js";

const HISTORY_CONCURRENCY = 4;
const MAX_SELECTED_ADDRESS_TXS = 100;

const historyByBucketKey =
  /** @type {Map<string, Promise<Map<string, unknown[]>>>} */ (new Map());

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} AddressHistoryClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressTxs
 */

/**
 * @typedef {Object} AddressHistory
 * @property {unknown[]} transactions
 * @property {number} fetchedAddressCount
 * @property {number} bucketSize
 */

/**
 * @param {readonly string[]} addresses
 */
function createBucketKey(addresses) {
  return [...addresses].sort().join("\n");
}

/**
 * @param {WalletAddress} address
 */
function assertHistoryIsReasonable(address) {
  if (address.txCount > MAX_SELECTED_ADDRESS_TXS) {
    throw new Error(
      `History disabled for addresses over ${MAX_SELECTED_ADDRESS_TXS} transactions`,
    );
  }
}

/**
 * @param {AddressHistoryClient} client
 * @param {readonly string[]} addresses
 * @returns {Promise<Map<string, unknown[]>>}
 */
async function fetchBucketHistory(client, addresses) {
  const entries = await mapConcurrent(
    addresses,
    HISTORY_CONCURRENCY,
    async (address) => {
      const transactions = /** @type {unknown[]} */ (
        await client.getAddressTxs(address, { cache: false })
      );

      return /** @type {const} */ ([address, transactions]);
    },
  );

  return new Map(entries);
}

/**
 * @param {AddressHistoryClient} client
 * @param {WalletAddress} address
 * @returns {Promise<AddressHistory>}
 */
async function load(client, address) {
  assertHistoryIsReasonable(address);

  if (address.historyAddresses.length === 0) {
    return {
      transactions: [],
      fetchedAddressCount: 0,
      bucketSize: address.historyBucketSize,
    };
  }

  const key = createBucketKey(address.historyAddresses);
  let history = historyByBucketKey.get(key);

  if (!history) {
    history = fetchBucketHistory(client, address.historyAddresses).catch(
      (error) => {
        historyByBucketKey.delete(key);
        throw error;
      },
    );
    historyByBucketKey.set(key, history);
  }

  const bucketHistory = await history;

  return {
    transactions: bucketHistory.get(address.address) ?? [],
    fetchedAddressCount: address.historyAddresses.length,
    bucketSize: address.historyBucketSize,
  };
}

export const addressHistory = /** @type {const} */ ({
  load,
});
