const MASK_64 = 0xffffffffffffffffn;
const DEFAULT_SECRETS = /** @type {const} */ ([
  0x2d358dccaa6c78a5n,
  0x8bb84b93962eacc9n,
  0x4b33a62ed433d4a3n,
  0x4d5a2da51de1aa47n,
  0xa0761d6478bd642fn,
  0xe7037ed1a0b428dbn,
  0x90ed1765281c388cn,
]);
const DEFAULT_SEED = rapidHashSeed(0n);

/**
 * @param {bigint} value
 */
function u64(value) {
  return value & MASK_64;
}

/**
 * @param {bigint} left
 * @param {bigint} right
 */
function rapidMix(left, right) {
  const result = u64(left) * u64(right);

  return u64(result) ^ u64(result >> 64n);
}

/**
 * @param {bigint} left
 * @param {bigint} right
 * @returns {[bigint, bigint]}
 */
function rapidMum(left, right) {
  const result = u64(left) * u64(right);

  return [u64(result), u64(result >> 64n)];
}

/**
 * @param {bigint} seed
 */
function rapidHashSeed(seed) {
  return u64(seed ^ rapidMix(seed ^ DEFAULT_SECRETS[2], DEFAULT_SECRETS[1]));
}

/**
 * @param {Uint8Array} bytes
 * @param {number} offset
 */
function readU64(bytes, offset) {
  return (
    BigInt(bytes[offset]) |
    (BigInt(bytes[offset + 1]) << 8n) |
    (BigInt(bytes[offset + 2]) << 16n) |
    (BigInt(bytes[offset + 3]) << 24n) |
    (BigInt(bytes[offset + 4]) << 32n) |
    (BigInt(bytes[offset + 5]) << 40n) |
    (BigInt(bytes[offset + 6]) << 48n) |
    (BigInt(bytes[offset + 7]) << 56n)
  );
}

/**
 * @param {Uint8Array} bytes
 */
function rapidHashV3(bytes) {
  const length = bytes.length;

  if (length <= 16) {
    throw new Error("Expected more than 16 bytes");
  }

  if (length > 32) {
    throw new Error("Expected at most 32 bytes");
  }

  let seed = rapidMix(
    readU64(bytes, 0) ^ DEFAULT_SECRETS[2],
    readU64(bytes, 8) ^ DEFAULT_SEED,
  );
  let a = readU64(bytes, length - 16) ^ BigInt(length);
  let b = readU64(bytes, length - 8);

  if (length > 32) {
    seed = rapidMix(
      readU64(bytes, 16) ^ DEFAULT_SECRETS[2],
      readU64(bytes, 24) ^ seed,
    );
  }

  a ^= DEFAULT_SECRETS[1];
  b ^= seed;

  [a, b] = rapidMum(a, b);

  return rapidMix(a ^ 0xaaaaaaaaaaaaaaaan, b ^ DEFAULT_SECRETS[1] ^ BigInt(length));
}

/**
 * @param {Uint8Array} bytes
 */
function rapidHashV3Hex(bytes) {
  return rapidHashV3(bytes).toString(16).padStart(16, "0");
}

/**
 * @param {Uint8Array} bytes
 * @param {number} nibbles
 */
export function rapidHashV3Prefix(bytes, nibbles) {
  return rapidHashV3Hex(bytes).slice(0, nibbles);
}
