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
 */

/**
 * @param {readonly string[]} addresses
 */
function createBucketKey(addresses) {
  return [...addresses].sort().join("\n");
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
  if (
    address.txCount > MAX_SELECTED_ADDRESS_TXS ||
    address.historyAddresses.length === 0
  ) {
    return {
      transactions: [],
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
  };
}

export const addressHistory = /** @type {const} */ ({
  load,
});
