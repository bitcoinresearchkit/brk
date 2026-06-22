import { brk } from "../utils/client.js";
import {
  setStatus,
  withBusy,
} from "./dom.js";
import { createEmpty } from "./empty/index.js";
import { getErrorMessage } from "./errors.js";
import { createAddForm } from "./add/index.js";
import { createLayout } from "./layout/index.js";
import { redaction } from "./redaction/index.js";
import { readWalletSourceText } from "./add/source.js";
import { scanStatus } from "./wallet/status.js";
import { createSelector } from "./selector/index.js";
import { createStart } from "./start/index.js";
import {
  createWalletPanel,
  renderWalletPanel,
} from "./wallet/index.js";
import { createVault } from "./vault/index.js";
import { generateAddressesFromWalletSource } from "./derive/index.js";
import { syncBtcAmounts } from "./amount/index.js";

/**
 * @typedef {import("./scan/index.js").WalletScan} WalletScan
 * @typedef {import("./vault/index.js").StoredWallet} StoredWallet
 * @typedef {import("./vault/index.js").WalletRuntime} WalletRuntime
 */

export function createWalletsPage() {
  const {
    main,
    utilities,
    privacyButton,
    sessionButton,
    selector: selectorElement,
    walletList,
    content,
    addDialog,
  } = createLayout();
  const vault = createVault();
  const selector = createSelector(walletList, {
    getSelectedId() {
      return vault.selectedId;
    },
    onSelect: select,
    onAdd() {
      openAdd();
    },
    onDelete() {
      deleteWallet(vault.selectedId);
    },
  });

  redaction.syncButton(privacyButton);

  /**
   * @param {string} walletId
   */
  function select(walletId) {
    vault.select(walletId);
    render();
  }

  function lock() {
    vault.lock();
    render();
  }

  function reset() {
    vault.reset();
    render();
  }

  function startEphemeral() {
    vault.startEphemeral();
    render();
  }

  function clearEphemeral() {
    vault.clearEphemeral();
    render();
  }

  /**
   * @param {string} walletId
   */
  function deleteWallet(walletId) {
    void vault.deleteWallet(walletId).then(() => {
      render();
    }, (error) => {
      console.error(error);
    });
  }

  function openAdd() {
    addDialog.replaceChildren(createAddForm({
      onCancel() {
        addDialog.close();
      },
      onSubmit(submit) {
        return submitAdd(submit);
      },
    }));
    addDialog.showModal();
  }

  privacyButton.addEventListener("click", () => {
    redaction.toggle(privacyButton);
    syncBtcAmounts();
  });

  sessionButton.addEventListener("click", () => {
    if (vault.isEphemeral()) {
      clearEphemeral();
      return;
    }

    lock();
  });

  /**
   * @param {"create" | "unlock"} mode
   */
  function renderStart(mode) {
    content.replaceChildren(createStart({
      mode,
      onPassword(password, button, status) {
        return mode === "unlock"
          ? unlock(password, button, status)
          : setup(password, button, status);
      },
      onEphemeral() {
        startEphemeral();
      },
      onReset: mode === "unlock" ? reset : undefined,
    }));
  }

  /**
   * @param {StoredWallet} wallet
   * @param {WalletRuntime} runtime
   */
  function renderUnlocked(wallet, runtime) {
    const panel = createWalletPanel();

    content.replaceChildren(...panel.nodes);

    if (runtime.scan) {
      renderWalletData(runtime.scan, panel);
      setStatus(panel.status, "Ready");
      return;
    }

    scanStatus.setPending(panel.status);
    void runtime.load({
      client: brk,
      onProgress(progress) {
        scanStatus.setProgress(panel.status, progress);
      },
    }).then((scan) => {
      if (!isCurrentPanel(wallet, runtime, panel)) return;

      renderWalletData(scan, panel);
      setStatus(panel.status, "Ready");
    }, (error) => {
      if (isCurrentPanel(wallet, runtime, panel)) {
        setStatus(panel.status, getErrorMessage(error));
      }
    });
  }

  /**
   * @param {StoredWallet} wallet
   * @param {WalletRuntime} runtime
   * @param {ReturnType<typeof createWalletPanel>} panel
   */
  function isCurrentPanel(wallet, runtime, panel) {
    return (
      vault.isCurrent(wallet, runtime) &&
      !vault.isLocked() &&
      vault.selectedId === wallet.id &&
      panel.results.isConnected
    );
  }

  /**
   * @param {WalletScan} scan
   * @param {ReturnType<typeof createWalletPanel>} panel
   */
  function renderWalletData(scan, panel) {
    renderWalletPanel(scan, panel, brk);
  }

  /**
   * @param {string} password
   * @param {HTMLButtonElement} button
   * @param {HTMLElement} status
   * @returns {Promise<boolean>}
   */
  async function unlock(password, button, status) {
    let unlocked = false;

    await withBusy(button, "Unlock", "Unlocking", async () => {
      setStatus(status, "");

      try {
        await vault.unlock(password);
        unlocked = true;
        render();
      } catch {
        unlocked = false;
      }
    });

    return unlocked;
  }

  /**
   * @param {string} password
   * @param {HTMLButtonElement} button
   * @param {HTMLElement} status
   */
  async function setup(password, button, status) {
    await withBusy(button, "Create", "Creating", async () => {
      setStatus(status, "");

      try {
        await vault.setup(password);
        render();
      } catch (error) {
        setStatus(status, getErrorMessage(error));
      }
    });
  }

  function renderContent() {
    const needsSetup = vault.needsSetup();
    const locked = vault.isLocked();
    const ephemeral = vault.isEphemeral();
    const current = vault.current();
    const empty = !needsSetup && !locked && !current;

    utilities.hidden = locked || needsSetup || empty;
    selectorElement.hidden = locked || needsSetup || empty;
    sessionButton.hidden = locked || needsSetup || (!vault.hasPassword && !ephemeral);
    sessionButton.textContent = ephemeral ? "Clear" : "Lock";

    if (needsSetup) {
      renderStart("create");
      return;
    }

    if (locked) {
      renderStart("unlock");
      return;
    }

    if (!current) {
      content.replaceChildren(createEmpty({
        onAdd() {
          openAdd();
        },
        onClear: ephemeral ? clearEphemeral : undefined,
      }));
      return;
    }

    renderUnlocked(current.wallet, current.runtime);
  }

  function render() {
    if (vault.isLocked()) {
      selector.clear();
    } else {
      selector.render(vault.wallets);
    }
    renderContent();
  }

  /**
   * @param {Object} options
   * @param {HTMLInputElement} options.name
   * @param {HTMLTextAreaElement} options.source
   * @param {HTMLButtonElement} options.submit
   * @param {HTMLFormElement} options.form
   */
  async function submitAdd({
    name,
    source,
    submit,
    form,
  }) {
    await withBusy(submit, "Add", "Adding", async () => {
      source.removeAttribute("aria-invalid");

      try {
        const value = readWalletSourceText(source.value);

        await generateAddressesFromWalletSource(value, { count: 1 });

        await vault.addWallet({
          name: name.value,
          source: value,
        });

        form.reset();
        addDialog.close();
        render();
      } catch {
        source.setAttribute("aria-invalid", "true");
        source.focus();
      }
    });
  }

  render();

  return main;
}
