import { brk } from "../utils/client.js";
import { createGroupedAddress } from "./address-view.js";
import { renderWalletAddresses } from "./addresses-view.js";
import {
  createEmptyWalletView,
  createLockedWalletView,
  createSetupWalletView,
  createUnlockedWalletView,
} from "./content-view.js";
import {
  createElement,
  setBusy,
  setStatus,
} from "./dom.js";
import { getErrorMessage } from "./errors.js";
import {
  createAddWalletForm,
} from "./import-view.js";
import {
  syncPrivacyButton,
  togglePrivateValues,
} from "./privacy-view.js";
import { fetchAddressHistory } from "./privacy/address-history.js";
import { renderReceiveButton } from "./receive-view.js";
import { inferAddressScript } from "./script-inference.js";
import { readWalletSourceText } from "./wallet-source.js";
import {
  addWallet,
  createWalletVault,
  hasStoredWallets,
  loadWallets,
  resetWalletVault,
  updateWalletScript,
} from "./storage/wallets.js";
import {
  createScanPendingMessage,
  scanWalletAddresses,
} from "./scan.js";
import {
  initWalletSelector,
  renderWalletSelector,
} from "./selector-view.js";
import { renderWalletSettings } from "./settings-view.js";
import { renderWalletSummary } from "./summary-view.js";

/**
 * @typedef {import("./xpub/address.js").AddressScript} AddressScript
 * @typedef {import("./scan.js").WalletAddress} WalletAddress
 * @typedef {import("./scan.js").WalletScan} WalletScan
 * @typedef {import("./storage/wallets.js").StoredWallet} StoredWallet
 */

/**
 * @param {WalletScan} scan
 * @param {HTMLElement} summary
 * @param {HTMLElement} settings
 * @param {HTMLElement} results
 */
function renderWalletScan(scan, summary, settings, results) {
  renderWalletSummary(summary, scan.addresses, scan.btcUsdPrice);
  renderReceiveButton(settings, scan.receiveAddress);
  renderWalletAddresses(results, scan.addresses, {
    fetchHistory(address) {
      return fetchAddressHistory(brk, address);
    },
    getErrorMessage,
  });
}

export function createWalletsPage() {
  const main = createElement("main", "wallets");
  const header = createElement("header", "wallets__header");
  const actions = createElement("div", "wallets__actions");
  const privacyButton = document.createElement("button");
  const lockButton = document.createElement("button");
  const selector = createElement("section", "wallets__selector");
  const walletList = createElement("div", "wallets__wallet-list");
  const addButton = document.createElement("button");
  const content = createElement("section", "wallets__content");
  const addDialog = createElement("dialog", "wallets__dialog");
  /** @type {StoredWallet[]} */
  let wallets = [];
  let selectedWalletId = "";
  let pageLocked = hasStoredWallets();
  let pagePassword = "";
  /** @type {Map<string, { xpub: string, scan?: WalletScan, scanPromise?: Promise<WalletScan | undefined> }>} */
  const walletStates = new Map();

  privacyButton.type = "button";
  syncPrivacyButton(privacyButton);
  lockButton.type = "button";
  lockButton.append("Lock");
  addButton.type = "button";
  addButton.append("Add watch-only wallet");
  content.setAttribute("aria-live", "polite");
  walletList.setAttribute("tabindex", "0");
  walletList.setAttribute("aria-label", "Wallets");
  header.append(selector, actions);
  actions.append(addButton, privacyButton, lockButton);
  selector.append(walletList);
  initWalletSelector(walletList, {
    getSelectedWalletId() {
      return selectedWalletId;
    },
    onSelect: selectWallet,
  });

  /**
   * @returns {StoredWallet | undefined}
   */
  function getSelectedWallet() {
    return wallets.find((wallet) => wallet.id === selectedWalletId);
  }

  /**
   * @param {string} walletId
   */
  function selectWallet(walletId) {
    selectedWalletId = walletId;
    render();
  }

  function lockPage() {
    wallets = [];
    selectedWalletId = "";
    walletStates.clear();
    pagePassword = "";
    pageLocked = hasStoredWallets();
    render();
  }

  function resetWallets() {
    resetWalletVault();
    wallets = [];
    selectedWalletId = "";
    walletStates.clear();
    pagePassword = "";
    pageLocked = false;
    render();
  }

  function openAddDialog() {
    addDialog.replaceChildren(createAddWalletForm({
      onCancel() {
        addDialog.close();
      },
      onSubmit(submit) {
        return addStoredWallet(submit);
      },
    }));
    addDialog.showModal();
  }

  privacyButton.addEventListener("click", () => {
    togglePrivateValues(main, privacyButton, createGroupedAddress);
  });

  lockButton.addEventListener("click", () => {
    lockPage();
  });

  addButton.addEventListener("click", () => {
    openAddDialog();
  });

  function syncSelectedWallet() {
    selectedWalletId = wallets.some((wallet) => wallet.id === selectedWalletId)
      ? selectedWalletId
      : wallets[0]?.id ?? "";
  }

  function renderLockedWallet() {
    content.replaceChildren(createLockedWalletView({
      onUnlock(password, button, status) {
        return unlockWallet(password, button, status);
      },
      onReset() {
        resetWallets();
      },
    }));
  }

  function renderSetupWallet() {
    content.replaceChildren(createSetupWalletView({
      onCreate(password, button, status) {
        return setupWallet(password, button, status);
      },
    }));
  }

  /**
   * @param {StoredWallet} wallet
   * @param {{ xpub: string, scan?: WalletScan, scanPromise?: Promise<WalletScan | undefined> }} state
   */
  function renderUnlockedWallet(wallet, state) {
    const view = createUnlockedWalletView();

    content.replaceChildren(...view.nodes);
    renderWalletSettings(view.settings, wallet, {
      onScriptChange(script, select, status) {
        return updateSelectedWalletScript(wallet, state, script, select, status);
      },
    });

    if (state.scan) {
      renderWalletScan(state.scan, view.summary, view.settings, view.results);
      setStatus(view.status, "Ready");
      return;
    }

    if (!state.scanPromise) {
      state.scanPromise = scanWalletAddresses({
        client: brk,
        xpub: state.xpub,
        start: 0,
        script: wallet.script,
        status: view.status,
      });
    } else {
      setStatus(view.status, createScanPendingMessage());
    }

    void state.scanPromise.then((scan) => {
      if (!scan || walletStates.get(wallet.id) !== state) return;

      state.scan = scan;

      if (pageLocked || selectedWalletId !== wallet.id || !view.results.isConnected) {
        return;
      }

      renderWalletScan(scan, view.summary, view.settings, view.results);
      setStatus(view.status, "Ready");
    });
  }

  /**
   * @param {string} password
   */
  async function unlockPageWallets(password) {
    wallets = await loadWallets(password);
    selectedWalletId = wallets.some((wallet) => wallet.id === selectedWalletId)
      ? selectedWalletId
      : wallets[0]?.id ?? "";

    walletStates.clear();
    pagePassword = password;

    for (const wallet of wallets) {
      walletStates.set(wallet.id, { xpub: wallet.xpub });
    }
  }

  /**
   * @param {string} password
   * @param {HTMLButtonElement} button
   * @param {HTMLElement} status
   */
  async function unlockWallet(password, button, status) {
    setBusy(button, true, "Unlock", "Unlocking");
    setStatus(status, "");

    try {
      await unlockPageWallets(password);
      pageLocked = false;
      render();
    } catch {
      setStatus(status, "Invalid password");
    } finally {
      setBusy(button, false, "Unlock", "Unlocking");
    }
  }

  /**
   * @param {string} password
   * @param {HTMLButtonElement} button
   * @param {HTMLElement} status
   */
  async function setupWallet(password, button, status) {
    setBusy(button, true, "Continue", "Creating");
    setStatus(status, "");

    try {
      await createWalletVault(password);
      wallets = [];
      selectedWalletId = "";
      walletStates.clear();
      pagePassword = password;
      pageLocked = false;
      render();
    } catch (error) {
      setStatus(status, getErrorMessage(error));
    } finally {
      setBusy(button, false, "Continue", "Creating");
    }
  }

  /**
   * @param {StoredWallet} wallet
   * @param {{ xpub: string, scan?: WalletScan, scanPromise?: Promise<WalletScan | undefined> }} state
   * @param {AddressScript} script
   * @param {HTMLSelectElement} select
   * @param {HTMLElement} status
   */
  async function updateSelectedWalletScript(wallet, state, script, select, status) {
    if (script === wallet.script) return;

    select.disabled = true;
    setStatus(status, "Saving");

    try {
      wallets = await updateWalletScript(wallets, {
        walletId: wallet.id,
        script,
      }, pagePassword);
      walletStates.set(wallet.id, { xpub: state.xpub });
      render();
    } catch (error) {
      select.value = wallet.script;
      setStatus(status, getErrorMessage(error));
    } finally {
      select.disabled = false;
    }
  }

  function renderSelectedWallet() {
    const hasVault = hasStoredWallets();
    const setup = !hasVault && !pagePassword;
    const locked = pageLocked && hasVault;
    const wallet = getSelectedWallet();
    const state = wallet ? walletStates.get(wallet.id) : undefined;

    main.toggleAttribute("data-wallets-page-locked", locked || setup);
    header.hidden = locked || setup;
    selector.hidden = locked || setup;
    lockButton.hidden = locked || setup || !pagePassword;

    if (setup) {
      renderSetupWallet();
      return;
    }

    if (locked) {
      renderLockedWallet();
      return;
    }

    if (!wallet) {
      content.replaceChildren(createEmptyWalletView({
        onAdd() {
          openAddDialog();
        },
      }));
      return;
    }

    if (state) {
      renderUnlockedWallet(wallet, state);
      return;
    }

    renderLockedWallet();
  }

  function render() {
    syncSelectedWallet();
    if (pageLocked && hasStoredWallets()) {
      walletList.replaceChildren();
    } else {
      renderWalletSelector(walletList, {
        wallets,
        selectedWalletId,
        onSelect: selectWallet,
      });
    }
    renderSelectedWallet();
  }

  /**
   * @param {Object} options
   * @param {HTMLInputElement} options.name
   * @param {HTMLInputElement} options.xpub
   * @param {HTMLButtonElement} options.submit
   * @param {HTMLElement} options.status
   * @param {HTMLFormElement} options.form
   */
  async function addStoredWallet({
    name,
    xpub,
    submit,
    status,
    form,
  }) {
    setBusy(submit, true, "Add", "Adding");
    setStatus(status, "Checking address type");

    try {
      const value = readWalletSourceText(xpub.value);
      const script = await inferAddressScript(brk, value);

      setStatus(status, "Saving");

      const added = await addWallet(wallets, {
        name: name.value,
        xpub: value,
        script,
      }, pagePassword);

      form.reset();
      addDialog.close();
      wallets = added.wallets;
      selectedWalletId = added.wallet.id;
      pageLocked = false;
      walletStates.set(added.wallet.id, { xpub: added.wallet.xpub });
      render();
    } catch (error) {
      setStatus(status, getErrorMessage(error));
    } finally {
      setBusy(submit, false, "Add", "Adding");
    }
  }

  main.append(header, selector, content, addDialog);
  render();

  return main;
}
