import { vaultStorage } from "./storage.js";
import { createRuntime } from "./runtime.js";

/**
 * @typedef {import("./storage.js").StoredWallet} StoredWallet
 * @typedef {import("./storage.js").AddWalletInput} AddWalletInput
 * @typedef {ReturnType<typeof createRuntime>} WalletRuntime
 */

export function createVault() {
  /** @type {StoredWallet[]} */
  let wallets = [];
  let selectedId = "";
  let locked = hasVault();
  let password = "";
  /** @type {Map<string, WalletRuntime>} */
  const runtimes = new Map();

  function hasVault() {
    return vaultStorage.has();
  }

  function syncSelected() {
    selectedId = wallets.some((wallet) => wallet.id === selectedId)
      ? selectedId
      : wallets[0]?.id ?? "";
  }

  function clear() {
    wallets = [];
    selectedId = "";
    runtimes.clear();
  }

  /**
   * @returns {StoredWallet | undefined}
   */
  function selectedWallet() {
    return wallets.find((wallet) => wallet.id === selectedId);
  }

  /**
   * @returns {{ wallet: StoredWallet, runtime: WalletRuntime } | undefined}
   */
  function current() {
    const wallet = selectedWallet();
    const runtime = wallet ? runtimes.get(wallet.id) : undefined;

    return wallet && runtime ? { wallet, runtime } : undefined;
  }

  /**
   * @param {StoredWallet} wallet
   * @param {WalletRuntime} runtime
   */
  function isCurrent(wallet, runtime) {
    return runtimes.get(wallet.id) === runtime;
  }

  /**
   * @param {string} walletId
   */
  function select(walletId) {
    selectedId = walletId;
    syncSelected();
  }

  function lock() {
    clear();
    password = "";
    locked = hasVault();
  }

  function reset() {
    vaultStorage.reset();
    clear();
    password = "";
    locked = false;
  }

  /**
   * @param {string} pagePassword
   */
  async function setup(pagePassword) {
    await vaultStorage.setup(pagePassword);
    clear();
    password = pagePassword;
    locked = false;
  }

  /**
   * @param {string} pagePassword
   */
  async function unlock(pagePassword) {
    wallets = await vaultStorage.load(pagePassword);
    syncSelected();
    runtimes.clear();
    password = pagePassword;
    locked = false;

    for (const wallet of wallets) {
      runtimes.set(wallet.id, createRuntime(wallet.source));
    }
  }

  /**
   * @param {AddWalletInput} input
   */
  async function addWallet(input) {
    const added = await vaultStorage.addWallet(wallets, input, password);

    wallets = added.wallets;
    selectedId = added.wallet.id;
    locked = false;
    runtimes.set(added.wallet.id, createRuntime(added.wallet.source));
  }

  return {
    get wallets() {
      return wallets;
    },
    get selectedId() {
      return selectedId;
    },
    get hasPassword() {
      return password !== "";
    },
    needsSetup() {
      return !hasVault() && !password;
    },
    isLocked() {
      return locked && hasVault();
    },
    current,
    isCurrent,
    select,
    lock,
    reset,
    setup,
    unlock,
    addWallet,
  };
}
