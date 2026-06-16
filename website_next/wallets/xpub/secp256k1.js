import { bigIntToBytes, bytesToBigInt, createBytes } from "./bytes.js";

const FIELD_PRIME =
  0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2fn;
const GROUP_ORDER =
  0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141n;
const GENERATOR = /** @type {const} */ ({
  x: 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798n,
  y: 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8n,
});

/**
 * @typedef {Object} Secp256k1Point
 * @property {bigint} x
 * @property {bigint} y
 */

/**
 * @param {bigint} value
 * @param {bigint} modulo
 */
function mod(value, modulo) {
  const result = value % modulo;

  return result >= 0n ? result : result + modulo;
}

/**
 * @param {bigint} value
 * @param {bigint} exponent
 * @param {bigint} modulo
 */
function modPow(value, exponent, modulo) {
  let result = 1n;
  let base = mod(value, modulo);
  let power = exponent;

  while (power > 0n) {
    if (power & 1n) result = mod(result * base, modulo);

    base = mod(base * base, modulo);
    power >>= 1n;
  }

  return result;
}

/**
 * @param {bigint} value
 */
function invertField(value) {
  let low = mod(value, FIELD_PRIME);
  let high = FIELD_PRIME;
  let lowCoefficient = 1n;
  let highCoefficient = 0n;

  while (low > 1n) {
    const ratio = high / low;

    [low, high] = [high - low * ratio, low];
    [lowCoefficient, highCoefficient] = [
      highCoefficient - lowCoefficient * ratio,
      lowCoefficient,
    ];
  }

  return mod(lowCoefficient, FIELD_PRIME);
}

/**
 * @param {Secp256k1Point} point
 */
function isOnCurve(point) {
  const left = mod(point.y * point.y, FIELD_PRIME);
  const right = mod(point.x * point.x * point.x + 7n, FIELD_PRIME);

  return left === right;
}

/**
 * @param {Secp256k1Point} point
 */
function doublePoint(point) {
  if (point.y === 0n) return null;

  const slope = mod(
    3n * point.x * point.x * invertField(2n * point.y),
    FIELD_PRIME,
  );
  const x = mod(slope * slope - 2n * point.x, FIELD_PRIME);
  const y = mod(slope * (point.x - x) - point.y, FIELD_PRIME);

  return { x, y };
}

/**
 * @param {Secp256k1Point} point
 */
function negatePoint(point) {
  return { x: point.x, y: mod(-point.y, FIELD_PRIME) };
}

/**
 * @param {Secp256k1Point} point
 */
function forceEvenY(point) {
  return point.y & 1n ? negatePoint(point) : point;
}

/**
 * @param {Secp256k1Point | null} left
 * @param {Secp256k1Point | null} right
 * @returns {Secp256k1Point | null}
 */
function addPoints(left, right) {
  if (!left) return right;
  if (!right) return left;

  if (left.x === right.x) {
    if (mod(left.y + right.y, FIELD_PRIME) === 0n) return null;

    return doublePoint(left);
  }

  const slope = mod(
    (right.y - left.y) * invertField(right.x - left.x),
    FIELD_PRIME,
  );
  const x = mod(slope * slope - left.x - right.x, FIELD_PRIME);
  const y = mod(slope * (left.x - x) - left.y, FIELD_PRIME);

  return { x, y };
}

/**
 * @param {bigint} scalar
 * @param {Secp256k1Point} point
 * @returns {Secp256k1Point | null}
 */
function multiplyPoint(scalar, point) {
  let result = /** @type {Secp256k1Point | null} */ (null);
  let addend = /** @type {Secp256k1Point | null} */ (point);
  let remaining = scalar;

  while (remaining > 0n) {
    if (remaining & 1n) result = addPoints(result, addend);

    addend = addPoints(addend, addend);
    remaining >>= 1n;
  }

  return result;
}

/**
 * @param {bigint} x
 * @param {boolean} odd
 */
function liftX(x, odd) {
  if (x >= FIELD_PRIME) {
    throw new Error("Invalid secp256k1 x coordinate");
  }

  let y = modPow(x * x * x + 7n, (FIELD_PRIME + 1n) / 4n, FIELD_PRIME);

  if (Boolean(y & 1n) !== odd) {
    y = FIELD_PRIME - y;
  }

  const point = { x, y };

  if (!isOnCurve(point)) {
    throw new Error("Invalid secp256k1 point");
  }

  return point;
}

/**
 * @param {Uint8Array} publicKey
 */
export function parseCompressedPublicKey(publicKey) {
  if (
    publicKey.length !== 33 ||
    (publicKey[0] !== 0x02 && publicKey[0] !== 0x03)
  ) {
    throw new Error("Expected a compressed public key");
  }

  return liftX(bytesToBigInt(publicKey.slice(1)), publicKey[0] === 0x03);
}

/**
 * @param {Secp256k1Point} point
 */
export function compressPublicKey(point) {
  const publicKey = createBytes(33);

  publicKey[0] = point.y & 1n ? 0x03 : 0x02;
  publicKey.set(bigIntToBytes(point.x, 32), 1);

  return publicKey;
}

/**
 * @param {Uint8Array} publicKey
 */
export function getXOnlyPublicKey(publicKey) {
  const point = forceEvenY(parseCompressedPublicKey(publicKey));

  return bigIntToBytes(point.x, 32);
}

/**
 * @param {Uint8Array} publicKey
 * @param {Uint8Array} tweak
 */
export function addPublicKeyTweak(publicKey, tweak) {
  const scalar = bytesToBigInt(tweak);

  if (scalar === 0n || scalar >= GROUP_ORDER) {
    throw new Error("Invalid secp256k1 public key tweak");
  }

  const tweakPoint = multiplyPoint(scalar, GENERATOR);
  const childPoint = addPoints(parseCompressedPublicKey(publicKey), tweakPoint);

  if (!childPoint) {
    throw new Error("Invalid secp256k1 child public key");
  }

  return compressPublicKey(childPoint);
}

/**
 * @param {Uint8Array} publicKey
 * @param {Uint8Array} tweak
 */
export function addXOnlyPublicKeyTweak(publicKey, tweak) {
  const scalar = bytesToBigInt(tweak);

  if (scalar >= GROUP_ORDER) {
    throw new Error("Invalid secp256k1 x-only public key tweak");
  }

  const internalPoint = forceEvenY(parseCompressedPublicKey(publicKey));
  const tweakPoint = scalar === 0n ? null : multiplyPoint(scalar, GENERATOR);
  const outputPoint = addPoints(internalPoint, tweakPoint);

  if (!outputPoint) {
    throw new Error("Invalid secp256k1 x-only child public key");
  }

  return bigIntToBytes(outputPoint.x, 32);
}
