import { encodeBase58Check } from "./base58.js";
import { concatBytes } from "./bytes.js";
import { hash160, sha256 } from "./hash.js";
import {
  addXOnlyPublicKeyTweak,
  getXOnlyPublicKey,
} from "./secp256k1.js";

const bech32Alphabet = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const bech32Generator = /** @type {const} */ ([
  0x3b6a57b2,
  0x26508e6d,
  0x1ea119fa,
  0x3d4233dd,
  0x2a1462b3,
]);
const BECH32_CHECKSUM = 1;
const BECH32M_CHECKSUM = 0x2bc830a3;

const p2pkhVersions = /** @type {const} */ ({
  mainnet: 0x00,
  testnet: 0x6f,
});

const p2shVersions = /** @type {const} */ ({
  mainnet: 0x05,
  testnet: 0xc4,
});

const bech32Prefixes = /** @type {const} */ ({
  mainnet: "bc",
  testnet: "tb",
});

/**
 * @typedef {"mainnet" | "testnet"} BitcoinNetwork
 * @typedef {"p2pkh" | "p2sh_p2wpkh" | "v0_p2wpkh" | "v1_p2tr" | "v0_p2wsh_sortedmulti"} AddressScript
 * @typedef {Object} EncodedAddress
 * @property {string} address
 * @property {Uint8Array} payload
 */

/**
 * @param {number} version
 * @param {Uint8Array} payload
 */
function encodeVersionedBase58(version, payload) {
  return encodeBase58Check(concatBytes([Uint8Array.of(version), payload]));
}

/**
 * @param {string} prefix
 */
function expandBech32Prefix(prefix) {
  const values = /** @type {number[]} */ ([]);

  for (const character of prefix) {
    values.push(character.charCodeAt(0) >> 5);
  }

  values.push(0);

  for (const character of prefix) {
    values.push(character.charCodeAt(0) & 31);
  }

  return values;
}

/**
 * @param {number[]} values
 */
function bech32Polymod(values) {
  let checksum = 1;

  for (const value of values) {
    const top = checksum >>> 25;

    checksum = ((checksum & 0x1ffffff) << 5) ^ value;

    for (let i = 0; i < bech32Generator.length; i += 1) {
      if ((top >>> i) & 1) {
        checksum ^= bech32Generator[i];
      }
    }
  }

  return checksum;
}

/**
 * @param {string} prefix
 * @param {number[]} values
 * @param {number} checksumConstant
 */
function createBech32Checksum(prefix, values, checksumConstant) {
  const polymod = bech32Polymod([
    ...expandBech32Prefix(prefix),
    ...values,
    0,
    0,
    0,
    0,
    0,
    0,
  ]);
  const checksum = /** @type {number[]} */ ([]);
  const combined = polymod ^ checksumConstant;

  for (let i = 0; i < 6; i += 1) {
    checksum.push((combined >>> (5 * (5 - i))) & 31);
  }

  return checksum;
}

/**
 * @param {Uint8Array} bytes
 * @param {number} fromBits
 * @param {number} toBits
 */
function convertBits(bytes, fromBits, toBits) {
  const maxValue = (1 << toBits) - 1;
  const values = /** @type {number[]} */ ([]);
  let accumulator = 0;
  let bits = 0;

  for (const byte of bytes) {
    accumulator = (accumulator << fromBits) | byte;
    bits += fromBits;

    while (bits >= toBits) {
      bits -= toBits;
      values.push((accumulator >>> bits) & maxValue);
    }
  }

  if (bits > 0) {
    values.push((accumulator << (toBits - bits)) & maxValue);
  }

  return values;
}

/**
 * @param {string} prefix
 * @param {number[]} values
 * @param {number} checksumConstant
 */
function encodeBech32(prefix, values, checksumConstant) {
  const checksum = createBech32Checksum(prefix, values, checksumConstant);
  const characters = [...values, ...checksum].map((value) => {
    return bech32Alphabet[value];
  });

  return `${prefix}1${characters.join("")}`;
}

/**
 * @param {string} tag
 * @param {Uint8Array} bytes
 */
async function taggedHash(tag, bytes) {
  const tagHash = await sha256(new TextEncoder().encode(tag));

  return sha256(concatBytes([tagHash, tagHash, bytes]));
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodeP2pkhAddressData(publicKey, network) {
  const payload = await hash160(publicKey);

  return {
    address: await encodeVersionedBase58(p2pkhVersions[network], payload),
    payload,
  };
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 */
export async function encodeP2pkhAddress(publicKey, network) {
  return (await encodeP2pkhAddressData(publicKey, network)).address;
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodeP2shP2wpkhAddressData(publicKey, network) {
  const publicKeyHash = await hash160(publicKey);
  const redeemScript = concatBytes([Uint8Array.of(0x00, 0x14), publicKeyHash]);
  const payload = await hash160(redeemScript);

  return {
    address: await encodeVersionedBase58(p2shVersions[network], payload),
    payload,
  };
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 */
export async function encodeP2shP2wpkhAddress(publicKey, network) {
  return (await encodeP2shP2wpkhAddressData(publicKey, network)).address;
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodeP2wpkhAddressData(publicKey, network) {
  const payload = await hash160(publicKey);
  const values = [0, ...convertBits(payload, 8, 5)];

  return {
    address: encodeBech32(bech32Prefixes[network], values, BECH32_CHECKSUM),
    payload,
  };
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 */
export async function encodeP2wpkhAddress(publicKey, network) {
  return (await encodeP2wpkhAddressData(publicKey, network)).address;
}

/**
 * @param {Uint8Array} witnessScript
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodeP2wshAddressData(witnessScript, network) {
  const payload = await sha256(witnessScript);
  const values = [0, ...convertBits(payload, 8, 5)];

  return {
    address: encodeBech32(bech32Prefixes[network], values, BECH32_CHECKSUM),
    payload,
  };
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodeP2trAddressData(publicKey, network) {
  const internalKey = getXOnlyPublicKey(publicKey);
  const tweak = await taggedHash("TapTweak", internalKey);
  const payload = addXOnlyPublicKeyTweak(publicKey, tweak);
  const values = [1, ...convertBits(payload, 8, 5)];

  return {
    address: encodeBech32(bech32Prefixes[network], values, BECH32M_CHECKSUM),
    payload,
  };
}

/**
 * @param {Uint8Array} publicKey
 * @param {BitcoinNetwork} network
 */
export async function encodeP2trAddress(publicKey, network) {
  return (await encodeP2trAddressData(publicKey, network)).address;
}

/**
 * @param {Uint8Array} publicKey
 * @param {AddressScript} script
 * @param {BitcoinNetwork} network
 * @returns {Promise<EncodedAddress>}
 */
export async function encodePublicKeyAddressData(publicKey, script, network) {
  if (script === "p2pkh") {
    return encodeP2pkhAddressData(publicKey, network);
  }

  if (script === "p2sh_p2wpkh") {
    return encodeP2shP2wpkhAddressData(publicKey, network);
  }

  if (script === "v0_p2wpkh") {
    return encodeP2wpkhAddressData(publicKey, network);
  }

  if (script === "v1_p2tr") {
    return encodeP2trAddressData(publicKey, network);
  }

  throw new Error("Expected a single-key address script");
}

/**
 * @param {Uint8Array} publicKey
 * @param {AddressScript} script
 * @param {BitcoinNetwork} network
 */
export async function encodePublicKeyAddress(publicKey, script, network) {
  return (await encodePublicKeyAddressData(publicKey, script, network)).address;
}
