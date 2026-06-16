const ENCRYPTION_VERSION = 1;
const PBKDF2_ITERATIONS = 250_000;
const KEY_BITS = 256;
const SALT_BYTES = 16;
const IV_BYTES = 12;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

/**
 * @typedef {Object} EncryptedSecret
 * @property {1} version
 * @property {"PBKDF2-SHA256"} kdf
 * @property {number} iterations
 * @property {"AES-GCM"} cipher
 * @property {string} salt
 * @property {string} iv
 * @property {string} ciphertext
 */

/**
 * @param {Uint8Array} bytes
 */
function toArrayBuffer(bytes) {
  const buffer = new ArrayBuffer(bytes.byteLength);

  new Uint8Array(buffer).set(bytes);

  return buffer;
}

/**
 * @param {Uint8Array} bytes
 */
function bytesToBase64(bytes) {
  let binary = "";

  for (const byte of bytes) {
    binary += String.fromCharCode(byte);
  }

  return btoa(binary);
}

/**
 * @param {string} base64
 */
function base64ToBytes(base64) {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);

  for (let i = 0; i < binary.length; i += 1) {
    bytes[i] = binary.charCodeAt(i);
  }

  return bytes;
}

/**
 * @param {number} length
 */
function randomBytes(length) {
  const bytes = new Uint8Array(length);

  crypto.getRandomValues(bytes);

  return bytes;
}

/**
 * @param {string} password
 */
async function importPassword(password) {
  return crypto.subtle.importKey(
    "raw",
    encoder.encode(password),
    "PBKDF2",
    false,
    ["deriveKey"],
  );
}

/**
 * @param {string} password
 * @param {Uint8Array} salt
 * @param {number} iterations
 */
async function deriveKey(password, salt, iterations) {
  const key = await importPassword(password);

  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      hash: "SHA-256",
      salt: toArrayBuffer(salt),
      iterations,
    },
    key,
    {
      name: "AES-GCM",
      length: KEY_BITS,
    },
    false,
    ["encrypt", "decrypt"],
  );
}

/**
 * @param {string} secret
 * @param {string} password
 * @returns {Promise<EncryptedSecret>}
 */
export async function encryptSecret(secret, password) {
  const salt = randomBytes(SALT_BYTES);
  const iv = randomBytes(IV_BYTES);
  const key = await deriveKey(password, salt, PBKDF2_ITERATIONS);
  const encrypted = await crypto.subtle.encrypt(
    {
      name: "AES-GCM",
      iv: toArrayBuffer(iv),
    },
    key,
    encoder.encode(secret),
  );

  return {
    version: ENCRYPTION_VERSION,
    kdf: "PBKDF2-SHA256",
    iterations: PBKDF2_ITERATIONS,
    cipher: "AES-GCM",
    salt: bytesToBase64(salt),
    iv: bytesToBase64(iv),
    ciphertext: bytesToBase64(new Uint8Array(encrypted)),
  };
}

/**
 * @param {EncryptedSecret} encrypted
 * @param {string} password
 */
export async function decryptSecret(encrypted, password) {
  if (encrypted.version !== ENCRYPTION_VERSION) {
    throw new Error("Unsupported wallet encryption version");
  }

  const salt = base64ToBytes(encrypted.salt);
  const iv = base64ToBytes(encrypted.iv);
  const ciphertext = base64ToBytes(encrypted.ciphertext);
  const key = await deriveKey(password, salt, encrypted.iterations);
  const decrypted = await crypto.subtle.decrypt(
    {
      name: "AES-GCM",
      iv: toArrayBuffer(iv),
    },
    key,
    toArrayBuffer(ciphertext),
  );

  return decoder.decode(decrypted);
}
