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
  let ephemeral = false;
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
    ephemeral = false;
    locked = hasVault();
  }

  function reset() {
    vaultStorage.reset();
    clear();
    password = "";
    ephemeral = false;
    locked = false;
  }

  function startEphemeral() {
    clear();
    password = "";
    ephemeral = true;
    locked = false;
  }

  function clearEphemeral() {
    clear();
    password = "";
    ephemeral = false;
    locked = hasVault();
  }

  /**
   * @param {string} pagePassword
   */
  async function setup(pagePassword) {
    await vaultStorage.setup(pagePassword);
    clear();
    password = pagePassword;
    ephemeral = false;
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
    ephemeral = false;
    locked = false;

    for (const wallet of wallets) {
      runtimes.set(wallet.id, createRuntime(wallet.source));
    }
  }

  /**
   * @param {AddWalletInput} input
   */
  async function addWallet(input) {
    if (ephemeral) {
      const wallet = vaultStorage.createWallet(input);

      wallets = [...wallets, wallet];
      selectedId = wallet.id;
      locked = false;
      runtimes.set(wallet.id, createRuntime(wallet.source));
      return;
    }

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
      return !hasVault() && !password && !ephemeral;
    },
    isLocked() {
      return !ephemeral && locked && hasVault();
    },
    isEphemeral() {
      return ephemeral;
    },
    current,
    isCurrent,
    select,
    lock,
    reset,
    startEphemeral,
    clearEphemeral,
    setup,
    unlock,
    addWallet,
  };
}
