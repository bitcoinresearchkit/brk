import { bytesEqual, bytesToBigInt, concatBytes, createBytes } from "./bytes.js";
import { checksum as createChecksum } from "./hash.js";

const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const base = 58n;

/**
 * @param {string} character
 */
function readBase58Character(character) {
  const index = alphabet.indexOf(character);

  if (index === -1) {
    throw new Error(`Invalid Base58 character: ${character}`);
  }

  return index;
}

/**
 * @param {string} text
 */
function countLeadingZeros(text) {
  let count = 0;

  while (text[count] === "1") {
    count += 1;
  }

  return count;
}

/**
 * @param {Uint8Array} bytes
 */
function countLeadingZeroBytes(bytes) {
  let count = 0;

  while (bytes[count] === 0) {
    count += 1;
  }

  return count;
}

/**
 * @param {string} text
 */
function decodeBase58(text) {
  let value = 0n;

  for (const character of text) {
    value = value * base + BigInt(readBase58Character(character));
  }

  const leadingZeros = countLeadingZeros(text);
  const decoded = /** @type {number[]} */ ([]);

  while (value > 0n) {
    decoded.push(Number(value & 0xffn));
    value >>= 8n;
  }

  decoded.reverse();

  const bytes = createBytes(leadingZeros + decoded.length);
  bytes.set(decoded, leadingZeros);

  return bytes;
}

/**
 * @param {Uint8Array} bytes
 */
function encodeBase58(bytes) {
  let value = bytesToBigInt(bytes);
  let text = "";

  while (value > 0n) {
    const index = Number(value % base);
    text = alphabet[index] + text;
    value /= base;
  }

  return "1".repeat(countLeadingZeroBytes(bytes)) + text;
}

/**
 * @param {string} text
 */
export async function decodeBase58Check(text) {
  const bytes = decodeBase58(text);

  if (bytes.length < 4) {
    throw new Error("Invalid Base58Check payload");
  }

  const payload = bytes.slice(0, -4);
  const expected = await createChecksum(payload);
  const actual = bytes.slice(-4);

  if (!bytesEqual(actual, expected)) {
    throw new Error("Invalid Base58Check checksum");
  }

  return payload;
}

/**
 * @param {Uint8Array} payload
 */
export async function encodeBase58Check(payload) {
  return encodeBase58(concatBytes([payload, await createChecksum(payload)]));
}
