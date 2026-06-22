/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 *
 * @typedef {Object} WalletTransactionAddress
 * @property {WalletAddress} walletAddress
 * @property {number} received
 * @property {number} sent
 * @property {number} net
 *
 * @typedef {Object} WalletTransaction
 * @property {string} txid
 * @property {string} date
 * @property {number | undefined} time
 * @property {string} status
 * @property {"receive" | "send" | "consolidation"} type
 * @property {number} amount
 * @property {number} fee
 * @property {WalletTransactionAddress[]} addresses
 * @property {unknown} raw
 */

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
function getOutputAddress(output) {
  return readString(readObject(output)?.scriptpubkeyAddress);
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
function getTransactionId(transaction) {
  return readString(readObject(transaction)?.txid) ?? "";
}

/**
 * @param {unknown} transaction
 */
function getTransactionFee(transaction) {
  return readNumber(readObject(transaction)?.fee) ?? 0;
}

/**
 * @param {unknown} transaction
 */
function getTransactionTime(transaction) {
  return readNumber(readObject(readObject(transaction)?.status)?.blockTime);
}

/**
 * @param {number | undefined} blockTime
 */
function getTransactionDate(blockTime) {
  if (blockTime !== undefined) {
    return new Date(blockTime * 1_000).toLocaleDateString("en-US", {
      month: "long",
      day: "numeric",
      year: "numeric",
    });
  }

  return "Mempool";
}

/**
 * @param {unknown} transaction
 */
function getTransactionStatus(transaction) {
  return readObject(readObject(transaction)?.status)?.confirmed === true
    ? "confirmed"
    : "mempool";
}

/**
 * @param {number} sent
 * @param {number} externalOutputValue
 */
function getTransactionType(sent, externalOutputValue) {
  if (sent === 0) return "receive";
  if (externalOutputValue === 0) return "consolidation";

  return "send";
}

/**
 * @param {string | undefined} address
 * @param {number} value
 * @param {ReadonlySet<string>} walletAddressSet
 */
function isExternalOutput(address, value, walletAddressSet) {
  if (address === undefined) return value > 0;

  return !walletAddressSet.has(address);
}

/**
 * @param {unknown} transaction
 * @param {ReadonlySet<string>} walletAddressSet
 */
function getExternalOutputValue(transaction, walletAddressSet) {
  return readArray(transaction, "vout").reduce((total, output) => {
    const address = getOutputAddress(output);
    const value = getOutputValue(output);

    return (
      total +
      (isExternalOutput(address, value, walletAddressSet) ? value : 0)
    );
  }, 0);
}

/**
 * @param {unknown} transaction
 * @param {readonly WalletAddress[]} walletAddresses
 * @returns {WalletTransaction}
 */
export function readWalletTransaction(transaction, walletAddresses) {
  const walletAddressSet = new Set(
    walletAddresses.map((walletAddress) => walletAddress.address),
  );
  const addresses = walletAddresses.map((walletAddress) => {
    const received = getTransactionReceived(transaction, walletAddress.address);
    const sent = getTransactionSent(transaction, walletAddress.address);

    return {
      walletAddress,
      received,
      sent,
      net: received - sent,
    };
  }).filter((address) => {
    return address.received > 0 || address.sent > 0;
  });
  const received = addresses.reduce((total, address) => {
    return total + address.received;
  }, 0);
  const sent = addresses.reduce((total, address) => {
    return total + address.sent;
  }, 0);
  const externalOutputValue = getExternalOutputValue(
    transaction,
    walletAddressSet,
  );
  const net = received - sent;
  const time = getTransactionTime(transaction);

  return {
    txid: getTransactionId(transaction),
    date: getTransactionDate(time),
    time,
    status: getTransactionStatus(transaction),
    type: getTransactionType(sent, externalOutputValue),
    amount: net,
    fee: getTransactionFee(transaction),
    addresses,
    raw: transaction,
  };
}
