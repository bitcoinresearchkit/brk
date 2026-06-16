import { concatBytes, writeUint32 } from "./bytes.js";
import { hmacSha512 } from "./hash.js";
import { parseExtendedPublicKey } from "./key.js";
import { addPublicKeyTweak } from "./secp256k1.js";

const HARDENED_INDEX = 0x80000000;

/**
 * @typedef {import("./key.js").ExtendedPublicKey} ExtendedPublicKey
 */

/**
 * @typedef {Object} DerivedPublicKey
 * @property {number} index
 * @property {Uint8Array} publicKey
 */

/**
 * @param {ExtendedPublicKey} key
 * @param {number} index
 * @returns {Promise<ExtendedPublicKey>}
 */
export async function derivePublicChild(key, index) {
  if (!Number.isSafeInteger(index) || index < 0 || index >= HARDENED_INDEX) {
    throw new Error("Expected a non-hardened child index");
  }

  const data = concatBytes([key.publicKey, writeUint32(index)]);
  const digest = await hmacSha512(key.chainCode, data);
  const tweak = digest.slice(0, 32);
  const chainCode = digest.slice(32);

  return {
    text: key.text,
    depth: key.depth + 1,
    childNumber: index,
    parentFingerprint: key.parentFingerprint,
    chainCode,
    publicKey: addPublicKeyTweak(key.publicKey, tweak),
    version: key.version,
  };
}

/**
 * @param {ExtendedPublicKey} key
 * @param {readonly number[]} path
 */
export async function derivePublicPath(key, path) {
  let child = key;

  for (const index of path) {
    child = await derivePublicChild(child, index);
  }

  return child;
}

/**
 * @param {ExtendedPublicKey} key
 * @param {number} start
 * @param {number} count
 * @param {readonly number[]} [path]
 * @returns {Promise<DerivedPublicKey[]>}
 */
export async function derivePublicKeys(key, start, count, path = []) {
  const parent = await derivePublicPath(key, path);
  const children = /** @type {DerivedPublicKey[]} */ ([]);

  for (let offset = 0; offset < count; offset += 1) {
    const index = start + offset;
    const child = await derivePublicChild(parent, index);

    children.push({ index, publicKey: child.publicKey });
  }

  return children;
}

/**
 * @param {string} text
 */
export async function parseXpub(text) {
  return parseExtendedPublicKey(text);
}
