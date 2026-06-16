import { decryptSecret, encryptSecret } from "./encryption.js";

const STORAGE_KEY = "bitview.wallets.v2";

/**
 * @typedef {import("./encryption.js").EncryptedSecret} EncryptedSecret
 * @typedef {import("../xpub/address.js").AddressScript} AddressScript
 */

/**
 * @typedef {Object} StoredWallet
 * @property {string} id
 * @property {string} name
 * @property {AddressScript} script
 * @property {string} xpub
 * @property {number} createdAt
 * @property {number} updatedAt
 */

/**
 * @typedef {Object} AddWalletInput
 * @property {string} name
 * @property {AddressScript} script
 * @property {string} xpub
 */

/**
 * @typedef {Object} UpdateWalletScriptInput
 * @property {string} walletId
 * @property {AddressScript} script
 */

/**
 * @typedef {Object} WalletVault
 * @property {StoredWallet[]} wallets
 */

/**
 * @param {unknown} value
 * @returns {EncryptedSecret | undefined}
 */
function readEncryptedVault(value) {
  return value && typeof value === "object"
    ? /** @type {EncryptedSecret} */ (value)
    : undefined;
}

/**
 * @param {unknown} value
 * @returns {WalletVault}
 */
function readVault(value) {
  if (!value || typeof value !== "object" || !("wallets" in value)) {
    return { wallets: [] };
  }

  return Array.isArray(value.wallets)
    ? /** @type {WalletVault} */ (value)
    : { wallets: [] };
}

function createWalletId() {
  return crypto.randomUUID();
}

function now() {
  return Date.now();
}

export function hasStoredWallets() {
  return Boolean(localStorage.getItem(STORAGE_KEY));
}

export function resetWalletVault() {
  localStorage.removeItem(STORAGE_KEY);
}

/**
 * @param {string} pagePassword
 */
export async function createWalletVault(pagePassword) {
  await writeWallets([], pagePassword);
}

/**
 * @param {string} pagePassword
 */
export async function loadWallets(pagePassword) {
  const value = localStorage.getItem(STORAGE_KEY);
  const encrypted = value ? readEncryptedVault(JSON.parse(value)) : undefined;

  if (!encrypted) return [];

  return readVault(JSON.parse(await decryptSecret(encrypted, pagePassword))).wallets;
}

/**
 * @param {StoredWallet[]} wallets
 * @param {string} pagePassword
 */
async function writeWallets(wallets, pagePassword) {
  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify(
      await encryptSecret(JSON.stringify({ wallets }), pagePassword),
    ),
  );
}

/**
 * @param {StoredWallet[]} wallets
 * @param {AddWalletInput} input
 * @param {string} pagePassword
 */
export async function addWallet(wallets, input, pagePassword) {
  const time = now();
  const wallet = {
    id: createWalletId(),
    name: input.name.trim(),
    script: input.script,
    xpub: input.xpub.trim(),
    createdAt: time,
    updatedAt: time,
  };
  const nextWallets = [...wallets, wallet];

  await writeWallets(nextWallets, pagePassword);

  return {
    wallet,
    wallets: nextWallets,
  };
}

/**
 * @param {StoredWallet[]} wallets
 * @param {UpdateWalletScriptInput} input
 * @param {string} pagePassword
 */
export async function updateWalletScript(wallets, input, pagePassword) {
  const time = now();
  const nextWallets = wallets.map((wallet) => {
    return wallet.id === input.walletId
      ? {
        ...wallet,
        script: input.script,
        updatedAt: time,
      }
      : wallet;
  });

  await writeWallets(nextWallets, pagePassword);

  return nextWallets;
}
