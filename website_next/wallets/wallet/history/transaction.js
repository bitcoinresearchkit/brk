/**
 * @typedef {Object} HistoryTransaction
 * @property {string} txid
 * @property {string} date
 * @property {string} direction
 * @property {number} amount
 * @property {number} fee
 */

/**
 * @param {unknown} transaction
 */
function getTransactionId(transaction) {
  return readString(readObject(transaction)?.txid) ?? "";
}

/**
 * @param {unknown} value
 */
function readObject(value) {
  return value && typeof value === "object"
    ? /** @type {Record<string, unknown>} */ (value)
    : undefined;
}

/**
 * @param {unknown} value
 */
function readNumber(value) {
  return typeof value === "number" && Number.isFinite(value)
    ? value
    : undefined;
}

/**
 * @param {unknown} value
 */
function readString(value) {
  return typeof value === "string" ? value : undefined;
}

/**
 * @param {unknown} value
 * @param {string} key
 */
function readArray(value, key) {
  const array = readObject(value)?.[key];

  return Array.isArray(array) ? array : [];
}

/**
 * @param {unknown} output
 * @param {string} address
 */
function isAddressOutput(output, address) {
  return readString(readObject(output)?.scriptpubkeyAddress) === address;
}

/**
 * @param {unknown} output
 */
function getOutputValue(output) {
  return readNumber(readObject(output)?.value) ?? 0;
}

/**
 * @param {unknown} transaction
 * @param {string} address
 */
function getTransactionReceived(transaction, address) {
  return readArray(transaction, "vout").reduce((total, output) => {
    return (
      total + (isAddressOutput(output, address) ? getOutputValue(output) : 0)
    );
  }, 0);
}

/**
 * @param {unknown} transaction
 * @param {string} address
 */
function getTransactionSent(transaction, address) {
  return readArray(transaction, "vin").reduce((total, input) => {
    const prevout = readObject(input)?.prevout;

    return (
      total + (isAddressOutput(prevout, address) ? getOutputValue(prevout) : 0)
    );
  }, 0);
}

/**
 * @param {unknown} transaction
 */
function getTransactionFee(transaction) {
  return readNumber(readObject(transaction)?.fee) ?? 0;
}

/**
 * @param {number} net
 */
function getTransactionDirection(net) {
  if (net > 0) return "received";
  if (net < 0) return "sent";

  return "moved";
}

/**
 * @param {unknown} transaction
 */
function getTransactionDate(transaction) {
  const blockTime = readNumber(
    readObject(readObject(transaction)?.status)?.blockTime,
  );

  if (blockTime !== undefined) {
    return new Date(blockTime * 1_000).toLocaleDateString("en-US");
  }

  return "mempool";
}

/**
 * @param {unknown} transaction
 * @param {string} address
 * @returns {HistoryTransaction}
 */
export function readHistoryTransaction(transaction, address) {
  const received = getTransactionReceived(transaction, address);
  const sent = getTransactionSent(transaction, address);
  const net = received - sent;

  return {
    txid: getTransactionId(transaction),
    date: getTransactionDate(transaction),
    direction: getTransactionDirection(net),
    amount: Math.abs(net),
    fee: getTransactionFee(transaction),
  };
}
