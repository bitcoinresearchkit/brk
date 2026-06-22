/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 *
 * @typedef {Object} WalletUtxo
 * @property {WalletAddress} address
 * @property {string} txid
 * @property {number} vout
 * @property {number} value
 * @property {boolean} confirmed
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
 * @param {unknown} utxo
 */
function isConfirmed(utxo) {
  return readObject(readObject(utxo)?.status)?.confirmed === true;
}

/**
 * @param {WalletAddress} address
 * @param {unknown} utxo
 * @returns {WalletUtxo | undefined}
 */
export function readWalletUtxo(address, utxo) {
  const txid = readString(readObject(utxo)?.txid);
  const vout = readNumber(readObject(utxo)?.vout);
  const value = readNumber(readObject(utxo)?.value);

  if (txid === undefined || vout === undefined || value === undefined) {
    return undefined;
  }

  return {
    address,
    txid,
    vout,
    value,
    confirmed: isConfirmed(utxo),
  };
}
