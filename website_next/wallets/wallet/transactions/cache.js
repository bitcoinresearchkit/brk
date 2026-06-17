import { addressHistory } from "./history.js";
import { readWalletTransaction } from "./transaction.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 * @typedef {import("./transaction.js").WalletTransaction} WalletTransaction
 */

/**
 * @typedef {Object} TransactionClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressTxs
 */

/**
 * @param {WalletAddress} address
 */
function isUsedAddress(address) {
  return address.txCount > 0;
}

/**
 * @param {WalletTransaction} a
 * @param {WalletTransaction} b
 */
function compareTransactions(a, b) {
  if (a.time === undefined && b.time === undefined) {
    return a.txid.localeCompare(b.txid);
  }

  if (a.time === undefined) return -1;
  if (b.time === undefined) return 1;

  return b.time - a.time;
}

/**
 * @param {TransactionClient} client
 * @param {readonly WalletAddress[]} addresses
 * @returns {Promise<WalletTransaction[]>}
 */
async function load(client, addresses) {
  const transactionsById = /** @type {Map<string, WalletTransaction>} */ (
    new Map()
  );
  const usedAddresses = addresses.filter(isUsedAddress);

  for (const address of usedAddresses) {
    const history = await addressHistory.load(client, address);

    for (const transaction of history.transactions) {
      const walletTransaction = readWalletTransaction(transaction, usedAddresses);

      if (walletTransaction.txid) {
        transactionsById.set(walletTransaction.txid, walletTransaction);
      }
    }
  }

  return [...transactionsById.values()].sort(compareTransactions);
}

export const transactionCache = /** @type {const} */ ({
  load,
});
