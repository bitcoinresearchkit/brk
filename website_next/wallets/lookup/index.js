import { mapConcurrent } from "../concurrent.js";
import {
  getAddressReceived,
  getAddressSent,
  getAddressTxCount,
} from "./stats.js";
import { findUsablePrefixBucket } from "./bucket.js";
import { isLocalClient } from "./local.js";

const LOOKUP_CONCURRENCY = 8;

/**
 * @typedef {import("../derive/index.js").AddressType} AddressType
 * @typedef {import("../derive/index.js").GeneratedAddress} GeneratedAddress
 */

/**
 * @typedef {import("./stats.js").AddressStats} AddressStats
 * @typedef {import("./bucket.js").AddrHashPrefixMatches} AddrHashPrefixMatches
 */

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {GeneratedAddress["script"]} script
 * @property {GeneratedAddress["network"]} network
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
 * @property {string} domain
 * @property {(address: string, options?: { cache?: boolean }) => Promise<AddressStats>} getAddress
 * @property {(addrType: AddressType, payload: Uint8Array, nibbles: number, options?: { cache?: boolean }) => Promise<AddrHashPrefixMatches>} getAddressPayloadHashPrefixMatches
 */

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
  const received = getAddressReceived(stats);
  const sent = getAddressSent(stats);

  return {
    index: generated.index,
    address: generated.address,
    script: generated.script,
    network: generated.network,
    addrType: generated.addrType,
    balance: received - sent,
    received,
    sent,
    txCount: getAddressTxCount(stats),
    typeIndex: stats.chainStats.typeIndex,
    historyAddresses: [...historyAddresses],
    historyBucketSize,
  };
}

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
 * @param {AddressClient} client
 * @param {readonly string[]} addresses
 * @param {Map<string, Promise<AddressStats>>} cache
 */
async function fetchBucketMetadata(client, addresses, cache) {
  for (const address of addresses) {
    if (!cache.has(address)) {
      cache.set(
        address,
        client.getAddress(address, { cache: false }),
      );
    }
  }

  await Promise.all(addresses.map((address) => cache.get(address)));
}

/**
 * @param {AddressClient} client
 * @param {GeneratedAddress} generated
 * @returns {Promise<WalletAddress>}
 */
async function fetchDirectWalletAddress(client, generated) {
  try {
    const stats = await client.getAddress(generated.address, { cache: false });
    const historyAddresses =
      getAddressTxCount(stats) > 0 ? [generated.address] : [];

    return createWalletAddress(generated, stats, historyAddresses, 1);
  } catch (error) {
    if (isNotFound(error)) {
      return createEmptyWalletAddress(generated, 1);
    }

    throw error;
  }
}

/**
 * @param {AddressClient} client
 * @param {GeneratedAddress} generated
 * @param {Map<string, Promise<AddressStats>>} metadataCache
 * @returns {Promise<WalletAddress>}
 */
async function fetchWalletAddress(client, generated, metadataCache) {
  const matches = await findUsablePrefixBucket(client, generated);

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

    if (bucketStats && getAddressTxCount(bucketStats) > 0) {
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
  if (isLocalClient(client)) {
    return mapConcurrent(generated, LOOKUP_CONCURRENCY, (address) => {
      return fetchDirectWalletAddress(client, address);
    });
  }

  const metadataCache =
    /** @type {Map<string, Promise<AddressStats>>} */ (new Map());

  return mapConcurrent(generated, LOOKUP_CONCURRENCY, (address) => {
    return fetchWalletAddress(client, address, metadataCache);
  });
}
