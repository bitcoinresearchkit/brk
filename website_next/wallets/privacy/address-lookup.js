import { mapConcurrent } from "../concurrent.js";
import { rapidHashV3Prefix } from "./rapidhash.js";

const MIN_PREFIX_NIBBLES = 4;
const MAX_PREFIX_NIBBLES = 16;
const LOOKUP_CONCURRENCY = 8;

/**
 * @typedef {import("../xpub/index.js").AddressType} AddressType
 */

/**
 * @typedef {Object} GeneratedAddress
 * @property {number} index
 * @property {string} address
 * @property {Uint8Array} payload
 * @property {string} script
 * @property {string} network
 * @property {AddressType} addrType
 */

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
 * @typedef {Object} AddrHashPrefixMatches
 * @property {AddressType} addrType
 * @property {string} prefix
 * @property {boolean} truncated
 * @property {string[]} addresses
 */

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {string} script
 * @property {string} network
 * @property {AddressType} addrType
 * @property {number} balance
 * @property {number} received
 * @property {number} sent
 * @property {number} txCount
 * @property {number | undefined} typeIndex
 * @property {string[]} historyAddresses
 * @property {number} historyBucketSize
 */

/**
 * @typedef {Object} AddressClient
 * @property {(address: string, options?: { cache?: boolean }) => Promise<unknown>} getAddress
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 */

/**
 * @param {AddressStats} stats
 */
function getReceived(stats) {
  return stats.chainStats.fundedTxoSum + stats.mempoolStats.fundedTxoSum;
}

/**
 * @param {AddressStats} stats
 */
function getSent(stats) {
  return stats.chainStats.spentTxoSum + stats.mempoolStats.spentTxoSum;
}

/**
 * @param {AddressStats} stats
 */
function getTxCount(stats) {
  return stats.chainStats.txCount + stats.mempoolStats.txCount;
}

/**
 * @param {GeneratedAddress} generated
 * @param {number} historyBucketSize
 * @returns {WalletAddress}
 */
function createEmptyWalletAddress(generated, historyBucketSize = 0) {
  return {
    index: generated.index,
    address: generated.address,
    script: generated.script,
    network: generated.network,
    addrType: generated.addrType,
    balance: 0,
    received: 0,
    sent: 0,
    txCount: 0,
    typeIndex: undefined,
    historyAddresses: [],
    historyBucketSize,
  };
}

/**
 * @param {GeneratedAddress} generated
 * @param {AddressStats} stats
 * @param {readonly string[]} historyAddresses
 * @param {number} historyBucketSize
 * @returns {WalletAddress}
 */
function createWalletAddress(
  generated,
  stats,
  historyAddresses,
  historyBucketSize,
) {
  const received = getReceived(stats);
  const sent = getSent(stats);

  return {
    index: generated.index,
    address: generated.address,
    script: generated.script,
    network: generated.network,
    addrType: generated.addrType,
    balance: received - sent,
    received,
    sent,
    txCount: getTxCount(stats),
    typeIndex: stats.chainStats.typeIndex,
    historyAddresses: [...historyAddresses],
    historyBucketSize,
  };
}

/**
 * @param {AddressClient} client
 * @param {GeneratedAddress} generated
 * @param {number} nibbles
 * @returns {Promise<AddrHashPrefixMatches>}
 */
async function fetchPrefixMatches(client, generated, nibbles) {
  const prefix = rapidHashV3Prefix(generated.payload, nibbles);

  return /** @type {AddrHashPrefixMatches} */ (
    await client.getAddressHashPrefixMatches(generated.addrType, prefix, {
      cache: false,
    })
  );
}

/**
 * @param {AddressClient} client
 * @param {GeneratedAddress} generated
 * @returns {Promise<AddrHashPrefixMatches>}
 */
async function findUsableBucket(client, generated) {
  for (
    let nibbles = MIN_PREFIX_NIBBLES;
    nibbles <= MAX_PREFIX_NIBBLES;
    nibbles += 1
  ) {
    const matches = await fetchPrefixMatches(client, generated, nibbles);

    if (matches.truncated) continue;

    return matches;
  }

  throw new Error("Address prefix bucket is too large");
}

/**
 * @param {AddressClient} client
 * @param {readonly string[]} addresses
 * @param {Map<string, Promise<AddressStats>>} cache
 */
async function fetchBucketMetadata(client, addresses, cache) {
  for (const address of addresses) {
    if (!cache.has(address)) {
      cache.set(
        address,
        client.getAddress(address, { cache: false }).then(
          (stats) => /** @type {AddressStats} */ (stats),
        ),
      );
    }
  }

  await Promise.all(addresses.map((address) => cache.get(address)));
}

/**
 * @param {AddressClient} client
 * @param {GeneratedAddress} generated
 * @param {Map<string, Promise<AddressStats>>} metadataCache
 * @returns {Promise<WalletAddress>}
 */
async function fetchWalletAddress(client, generated, metadataCache) {
  const matches = await findUsableBucket(client, generated);

  if (!matches.addresses.includes(generated.address)) {
    return createEmptyWalletAddress(generated, matches.addresses.length);
  }

  await fetchBucketMetadata(client, matches.addresses, metadataCache);

  const stats = await metadataCache.get(generated.address);

  if (!stats) {
    return createEmptyWalletAddress(generated);
  }

  const historyAddresses = [];

  for (const address of matches.addresses) {
    const bucketStats = await metadataCache.get(address);

    if (bucketStats && getTxCount(bucketStats) > 0) {
      historyAddresses.push(address);
    }
  }

  return createWalletAddress(
    generated,
    stats,
    historyAddresses,
    matches.addresses.length,
  );
}

/**
 * @param {AddressClient} client
 * @param {readonly GeneratedAddress[]} generated
 * @returns {Promise<WalletAddress[]>}
 */
export async function fetchWalletAddresses(client, generated) {
  const metadataCache =
    /** @type {Map<string, Promise<AddressStats>>} */ (new Map());

  return mapConcurrent(generated, LOOKUP_CONCURRENCY, (address) => {
    return fetchWalletAddress(client, address, metadataCache);
  });
}
