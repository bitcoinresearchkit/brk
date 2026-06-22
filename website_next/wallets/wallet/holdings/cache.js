import { mapConcurrent } from "../../concurrent.js";

const UTXO_CONCURRENCY = 4;

const utxosByBucketKey =
  /** @type {Map<string, Promise<Map<string, unknown[]>>>} */ (new Map());

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} UtxoClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressUtxos
 */

/**
 * @typedef {Object} AddressUtxos
 * @property {unknown[]} utxos
 */

/**
 * @param {unknown} error
 */
function isNotFound(error) {
  return (
    error instanceof Error &&
    /** @type {{ status?: unknown }} */ (error).status === 404
  );
}

/**
 * @param {readonly string[]} addresses
 */
function createBucketKey(addresses) {
  return [...addresses].sort().join("\n");
}

/**
 * @param {UtxoClient} client
 * @param {string} address
 */
async function fetchAddressUtxos(client, address) {
  try {
    return /** @type {unknown[]} */ (
      await client.getAddressUtxos(address, { cache: false })
    );
  } catch (error) {
    if (isNotFound(error)) return [];

    throw error;
  }
}

/**
 * @param {UtxoClient} client
 * @param {readonly string[]} addresses
 * @returns {Promise<Map<string, unknown[]>>}
 */
async function fetchBucketUtxos(client, addresses) {
  const entries = await mapConcurrent(
    addresses,
    UTXO_CONCURRENCY,
    async (address) => {
      return /** @type {const} */ ([
        address,
        await fetchAddressUtxos(client, address),
      ]);
    },
  );

  return new Map(entries);
}

/**
 * @param {UtxoClient} client
 * @param {WalletAddress} address
 * @returns {Promise<AddressUtxos>}
 */
async function load(client, address) {
  if (address.balance <= 0 || address.historyAddresses.length === 0) {
    return {
      utxos: [],
    };
  }

  const key = createBucketKey(address.historyAddresses);
  let utxos = utxosByBucketKey.get(key);

  if (!utxos) {
    utxos = fetchBucketUtxos(client, address.historyAddresses).catch(
      (error) => {
        utxosByBucketKey.delete(key);
        throw error;
      },
    );
    utxosByBucketKey.set(key, utxos);
  }

  const bucketUtxos = await utxos;

  return {
    utxos: bucketUtxos.get(address.address) ?? [],
  };
}

export const utxoCache = /** @type {const} */ ({
  load,
});
