/**
 * @typedef {Object} AddressStatsPart
 * @property {number} fundedTxoSum
 * @property {number} spentTxoSum
 * @property {number} txCount
 */

/**
 * @typedef {AddressStatsPart & {
 *   typeIndex: number,
 * }} AddressChainStats
 */

/**
 * @typedef {Object} AddressStats
 * @property {string} address
 * @property {AddressChainStats} chainStats
 * @property {AddressStatsPart} mempoolStats
 */

/**
 * @param {AddressStats} stats
 */
export function getAddressReceived(stats) {
  return stats.chainStats.fundedTxoSum + stats.mempoolStats.fundedTxoSum;
}

/**
 * @param {AddressStats} stats
 */
export function getAddressSent(stats) {
  return stats.chainStats.spentTxoSum + stats.mempoolStats.spentTxoSum;
}

/**
 * @param {AddressStats} stats
 */
export function getAddressTxCount(stats) {
  return stats.chainStats.txCount + stats.mempoolStats.txCount;
}
