import { createBytes } from "./bytes.js";

const ripemdLeftIndexes = /** @type {const} */ ([
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
  7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8,
  3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12,
  1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
  4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
]);

const ripemdRightIndexes = /** @type {const} */ ([
  5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12,
  6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2,
  15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13,
  8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10, 14,
  12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11,
]);

const ripemdLeftShifts = /** @type {const} */ ([
  11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8,
  7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12,
  11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5,
  11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12,
  9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
]);

const ripemdRightShifts = /** @type {const} */ ([
  8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6,
  9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11,
  9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5,
  15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8,
  8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11,
]);

const ripemdLeftConstants = /** @type {const} */ ([
  0x00000000,
  0x5a827999,
  0x6ed9eba1,
  0x8f1bbcdc,
  0xa953fd4e,
]);

const ripemdRightConstants = /** @type {const} */ ([
  0x50a28be6,
  0x5c4dd124,
  0x6d703ef3,
  0x7a6d76e9,
  0x00000000,
]);

/**
 * @param {Uint8Array} bytes
 */
function toArrayBuffer(bytes) {
  const buffer = new ArrayBuffer(bytes.length);

  new Uint8Array(buffer).set(bytes);

  return buffer;
}

/**
 * @param {Uint8Array} bytes
 */
export async function sha256(bytes) {
  return new Uint8Array(
    await crypto.subtle.digest("SHA-256", toArrayBuffer(bytes)),
  );
}

/**
 * @param {Uint8Array} bytes
 */
async function doubleSha256(bytes) {
  return sha256(await sha256(bytes));
}

/**
 * @param {Uint8Array} key
 * @param {Uint8Array} bytes
 */
export async function hmacSha512(key, bytes) {
  const cryptoKey = await crypto.subtle.importKey(
    "raw",
    toArrayBuffer(key),
    { name: "HMAC", hash: "SHA-512" },
    false,
    ["sign"],
  );

  return new Uint8Array(
    await crypto.subtle.sign("HMAC", cryptoKey, toArrayBuffer(bytes)),
  );
}

/**
 * @param {number} value
 * @param {number} bits
 */
function rotateLeft(value, bits) {
  return (value << bits) | (value >>> (32 - bits));
}

/**
 * @param {number} round
 * @param {number} x
 * @param {number} y
 * @param {number} z
 */
function ripemdFunction(round, x, y, z) {
  if (round < 16) return x ^ y ^ z;
  if (round < 32) return (x & y) | (~x & z);
  if (round < 48) return (x | ~y) ^ z;
  if (round < 64) return (x & z) | (y & ~z);

  return x ^ (y | ~z);
}

/**
 * @param {Uint8Array} bytes
 */
function createRipemdBlocks(bytes) {
  const bitLength = BigInt(bytes.length) * 8n;
  const length = bytes.length + 1 + 8;
  const paddedLength = Math.ceil(length / 64) * 64;
  const padded = createBytes(paddedLength);

  padded.set(bytes);
  padded[bytes.length] = 0x80;

  for (let i = 0; i < 8; i += 1) {
    padded[paddedLength - 8 + i] = Number(
      (bitLength >> (BigInt(i) * 8n)) & 0xffn,
    );
  }

  return padded;
}

/**
 * @param {Uint8Array} block
 * @param {number} offset
 */
function readRipemdWords(block, offset) {
  const words = /** @type {number[]} */ ([]);

  for (let i = 0; i < 16; i += 1) {
    const start = offset + i * 4;
    words.push(
      block[start] |
        (block[start + 1] << 8) |
        (block[start + 2] << 16) |
        (block[start + 3] << 24),
    );
  }

  return words;
}

/**
 * @param {Uint8Array} target
 * @param {number} offset
 * @param {number} value
 */
function writeRipemdWord(target, offset, value) {
  target[offset] = value;
  target[offset + 1] = value >>> 8;
  target[offset + 2] = value >>> 16;
  target[offset + 3] = value >>> 24;
}

/**
 * @param {Uint8Array} bytes
 */
function ripemd160(bytes) {
  const blocks = createRipemdBlocks(bytes);
  const digest = createBytes(20);
  let h0 = 0x67452301;
  let h1 = 0xefcdab89;
  let h2 = 0x98badcfe;
  let h3 = 0x10325476;
  let h4 = 0xc3d2e1f0;

  for (let offset = 0; offset < blocks.length; offset += 64) {
    const words = readRipemdWords(blocks, offset);
    let al = h0;
    let bl = h1;
    let cl = h2;
    let dl = h3;
    let el = h4;
    let ar = h0;
    let br = h1;
    let cr = h2;
    let dr = h3;
    let er = h4;

    for (let round = 0; round < 80; round += 1) {
      const leftGroup = Math.floor(round / 16);
      const rightGroup = Math.floor(round / 16);
      const nextLeft =
        (rotateLeft(
          (al +
            ripemdFunction(round, bl, cl, dl) +
            words[ripemdLeftIndexes[round]] +
            ripemdLeftConstants[leftGroup]) |
            0,
          ripemdLeftShifts[round],
        ) +
          el) |
        0;
      const nextRight =
        (rotateLeft(
          (ar +
            ripemdFunction(79 - round, br, cr, dr) +
            words[ripemdRightIndexes[round]] +
            ripemdRightConstants[rightGroup]) |
            0,
          ripemdRightShifts[round],
        ) +
          er) |
        0;

      al = el;
      el = dl;
      dl = rotateLeft(cl, 10);
      cl = bl;
      bl = nextLeft;

      ar = er;
      er = dr;
      dr = rotateLeft(cr, 10);
      cr = br;
      br = nextRight;
    }

    const nextH0 = (h1 + cl + dr) | 0;
    h1 = (h2 + dl + er) | 0;
    h2 = (h3 + el + ar) | 0;
    h3 = (h4 + al + br) | 0;
    h4 = (h0 + bl + cr) | 0;
    h0 = nextH0;
  }

  writeRipemdWord(digest, 0, h0);
  writeRipemdWord(digest, 4, h1);
  writeRipemdWord(digest, 8, h2);
  writeRipemdWord(digest, 12, h3);
  writeRipemdWord(digest, 16, h4);

  return digest;
}

/**
 * @param {Uint8Array} bytes
 */
export async function hash160(bytes) {
  return ripemd160(await sha256(bytes));
}

/**
 * @param {Uint8Array} bytes
 */
export async function checksum(bytes) {
  return (await doubleSha256(bytes)).slice(0, 4);
}
