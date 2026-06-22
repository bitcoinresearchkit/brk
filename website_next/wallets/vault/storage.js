import { encryption } from "./encryption.js";

const STORAGE_KEY = "bitview.wallets.v3";

/**
 * @typedef {import("./encryption.js").EncryptedSecret} EncryptedSecret
 */

/**
 * @typedef {Object} StoredWallet
 * @property {string} id
 * @property {string} name
 * @property {string} source
 * @property {number} createdAt
 * @property {number} updatedAt
 */

/**
 * @typedef {Object} AddWalletInput
 * @property {string} name
 * @property {string} source
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

function has() {
  return Boolean(localStorage.getItem(STORAGE_KEY));
}

function reset() {
  localStorage.removeItem(STORAGE_KEY);
}

/**
 * @param {AddWalletInput} input
 */
function createWallet(input) {
  const time = now();

  return {
    id: createWalletId(),
    name: input.name.trim(),
    source: input.source.trim(),
    createdAt: time,
    updatedAt: time,
  };
}

/**
 * @param {string} pagePassword
 */
async function setup(pagePassword) {
  await writeWallets([], pagePassword);
}

/**
 * @param {string} pagePassword
 */
async function load(pagePassword) {
  const value = localStorage.getItem(STORAGE_KEY);
  const encrypted = value ? readEncryptedVault(JSON.parse(value)) : undefined;

  if (!encrypted) return [];

  const decrypted = await encryption.decrypt(encrypted, pagePassword);
  const vault = readVault(JSON.parse(decrypted));

  return vault.wallets;
}

/**
 * @param {StoredWallet[]} wallets
 * @param {string} pagePassword
 */
async function writeWallets(wallets, pagePassword) {
  localStorage.setItem(
    STORAGE_KEY,
    JSON.stringify(
      await encryption.encrypt(JSON.stringify({ wallets }), pagePassword),
    ),
  );
}

/**
 * @param {StoredWallet[]} wallets
 * @param {AddWalletInput} input
 * @param {string} pagePassword
 */
async function addWallet(wallets, input, pagePassword) {
  const wallet = createWallet(input);
  const nextWallets = [...wallets, wallet];

  await writeWallets(nextWallets, pagePassword);

  return {
    wallet,
    wallets: nextWallets,
  };
}

/**
 * @param {StoredWallet[]} wallets
 * @param {string} walletId
 * @param {string} pagePassword
 */
async function deleteWallet(wallets, walletId, pagePassword) {
  const nextWallets = wallets.filter((wallet) => wallet.id !== walletId);

  await writeWallets(nextWallets, pagePassword);

  return nextWallets;
}

export const vaultStorage = /** @type {const} */ ({
  has,
  reset,
  createWallet,
  setup,
  load,
  addWallet,
  deleteWallet,
});
