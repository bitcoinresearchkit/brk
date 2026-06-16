/**
 * @param {number} length
 */
export function createBytes(length) {
  return new Uint8Array(length);
}

/**
 * @param {Uint8Array[]} parts
 */
export function concatBytes(parts) {
  const length = parts.reduce((total, part) => total + part.length, 0);
  const bytes = createBytes(length);
  let offset = 0;

  for (const part of parts) {
    bytes.set(part, offset);
    offset += part.length;
  }

  return bytes;
}

/**
 * @param {Uint8Array} bytes
 * @param {number} start
 */
export function readUint32(bytes, start) {
  return (
    bytes[start] * 0x1000000 +
    bytes[start + 1] * 0x10000 +
    bytes[start + 2] * 0x100 +
    bytes[start + 3]
  );
}

/**
 * @param {number} value
 */
export function writeUint32(value) {
  const bytes = createBytes(4);

  bytes[0] = value >>> 24;
  bytes[1] = value >>> 16;
  bytes[2] = value >>> 8;
  bytes[3] = value;

  return bytes;
}

/**
 * @param {Uint8Array} bytes
 */
export function bytesToBigInt(bytes) {
  let value = 0n;

  for (const byte of bytes) {
    value = (value << 8n) + BigInt(byte);
  }

  return value;
}

/**
 * @param {bigint} value
 * @param {number} length
 */
export function bigIntToBytes(value, length) {
  const bytes = createBytes(length);
  let remaining = value;

  for (let i = length - 1; i >= 0; i -= 1) {
    bytes[i] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }

  return bytes;
}

/**
 * @param {Uint8Array} left
 * @param {Uint8Array} right
 */
export function bytesEqual(left, right) {
  if (left.length !== right.length) return false;

  for (let i = 0; i < left.length; i += 1) {
    if (left[i] !== right[i]) return false;
  }

  return true;
}
