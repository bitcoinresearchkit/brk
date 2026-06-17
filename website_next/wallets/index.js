import { brk } from "../utils/client.js";
import { createGroupedAddress } from "./wallet/address/index.js";
import {
  setStatus,
  withBusy,
} from "./dom.js";
import { createEmpty } from "./empty/index.js";
import { getErrorMessage } from "./errors.js";
import { createAddForm } from "./add/index.js";
import { createLayout } from "./layout/index.js";
import { createLock } from "./lock/index.js";
import { redaction } from "./redaction/index.js";
import { inferAddressScript } from "./add/inference.js";
import { readWalletSourceText } from "./add/source.js";
import { scanStatus } from "./wallet/status.js";
import { createSelector } from "./selector/index.js";
import { renderSettings } from "./wallet/settings/index.js";
import { createSetup } from "./setup/index.js";
import {
  createWalletPanel,
  renderWalletPanel,
} from "./wallet/index.js";
import { createVault } from "./vault/index.js";

/**
 * @typedef {import("./derive/address.js").AddressScript} AddressScript
 * @typedef {import("./scan/index.js").WalletScan} WalletScan
 * @typedef {import("./vault/index.js").StoredWallet} StoredWallet
 * @typedef {import("./vault/index.js").WalletRuntime} WalletRuntime
 */

export function createWalletsPage() {
  const {
    main,
    header,
    addButton,
    privacyButton,
    lockButton,
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
    redaction.toggle(main, privacyButton, createGroupedAddress);
  });

  lockButton.addEventListener("click", () => {
    lock();
  });

  addButton.addEventListener("click", () => {
    openAdd();
  });

  function renderLocked() {
    content.replaceChildren(createLock({
      onUnlock(password, button, status) {
        return unlock(password, button, status);
      },
      onReset() {
        reset();
      },
    }));
  }

  function renderSetup() {
    content.replaceChildren(createSetup({
      onCreate(password, button, status) {
        return setup(password, button, status);
      },
    }));
  }

  /**
   * @param {StoredWallet} wallet
   * @param {WalletRuntime} runtime
   */
  function renderUnlocked(wallet, runtime) {
    const panel = createWalletPanel();

    content.replaceChildren(...panel.nodes);
    renderSettings(panel.settings, wallet, {
      onScriptChange(script, select, status) {
        return updateScript(wallet, script, select, status);
      },
    });

    if (runtime.scan) {
      renderWalletData(runtime.scan, panel);
      setStatus(panel.status, "Ready");
      return;
    }

    scanStatus.setPending(panel.status);
    void runtime.load({
      client: brk,
      script: wallet.script,
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
   */
  async function unlock(password, button, status) {
    await withBusy(button, "Unlock", "Unlocking", async () => {
      setStatus(status, "");

      try {
        await vault.unlock(password);
        render();
      } catch {
        setStatus(status, "Invalid password");
      }
    });
  }

  /**
   * @param {string} password
   * @param {HTMLButtonElement} button
   * @param {HTMLElement} status
   */
  async function setup(password, button, status) {
    await withBusy(button, "Continue", "Creating", async () => {
      setStatus(status, "");

      try {
        await vault.setup(password);
        render();
      } catch (error) {
        setStatus(status, getErrorMessage(error));
      }
    });
  }

  /**
   * @param {StoredWallet} wallet
   * @param {AddressScript} script
   * @param {HTMLSelectElement} select
   * @param {HTMLElement} status
   */
  async function updateScript(
    wallet,
    script,
    select,
    status,
  ) {
    if (script === wallet.script) return;

    select.disabled = true;
    setStatus(status, "Saving");

    try {
      await vault.updateWalletScript(wallet, script);
      render();
    } catch (error) {
      select.value = wallet.script;
      setStatus(status, getErrorMessage(error));
    } finally {
      select.disabled = false;
    }
  }

  function renderContent() {
    const needsSetup = vault.needsSetup();
    const locked = vault.isLocked();
    const current = vault.current();
    const empty = !needsSetup && !locked && !current;

    main.toggleAttribute("data-wallets-page-locked", locked || needsSetup);
    main.toggleAttribute("data-wallets-page-empty", empty);
    header.hidden = locked || needsSetup || empty;
    selectorElement.hidden = locked || needsSetup || empty;
    lockButton.hidden = locked || needsSetup || !vault.hasPassword;

    if (needsSetup) {
      renderSetup();
      return;
    }

    if (locked) {
      renderLocked();
      return;
    }

    if (!current) {
      content.replaceChildren(createEmpty({
        onAdd() {
          openAdd();
        },
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
   * @param {HTMLInputElement} options.source
   * @param {HTMLButtonElement} options.submit
   * @param {HTMLElement} options.status
   * @param {HTMLFormElement} options.form
   */
  async function submitAdd({
    name,
    source,
    submit,
    status,
    form,
  }) {
    await withBusy(submit, "Add", "Adding", async () => {
      setStatus(status, "Checking address type");

      try {
        const value = readWalletSourceText(source.value);
        const script = await inferAddressScript(brk, value);

        setStatus(status, "Saving");

        await vault.addWallet({
          name: name.value,
          source: value,
          script,
        });

        form.reset();
        addDialog.close();
        render();
      } catch (error) {
        setStatus(status, getErrorMessage(error));
      }
    });
  }

  render();

  return main;
}
