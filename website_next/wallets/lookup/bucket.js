import { rapidHashV3Prefix } from "./hash.js";

const MIN_PREFIX_NIBBLES = 4;
const MAX_PREFIX_NIBBLES = 16;

/**
 * @typedef {import("../derive/index.js").AddressType} AddressType
 * @typedef {import("../derive/index.js").GeneratedAddress} GeneratedAddress
 */

/**
 * @typedef {Object} AddrHashPrefixMatches
 * @property {AddressType} addrType
 * @property {string} prefix
 * @property {boolean} truncated
 * @property {string[]} addresses
 */

/**
 * @typedef {Object} AddressClient
 * @property {(addrType: AddressType, prefix: string, options?: { cache?: boolean }) => Promise<unknown>} getAddressHashPrefixMatches
 */

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
export async function findUsablePrefixBucket(client, generated) {
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
