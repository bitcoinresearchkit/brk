import {
  createAddressScriptSelect,
  readAddressScript,
} from "./address-scripts.js";
import {
  createElement,
  createField,
} from "./dom.js";
import { isOutputDescriptor } from "./xpub/index.js";

/**
 * @typedef {import("./address-scripts.js").AddressScript} AddressScript
 * @typedef {import("./storage/wallets.js").StoredWallet} StoredWallet
 */

/**
 * @typedef {Object} WalletSettingsOptions
 * @property {(script: AddressScript, select: HTMLSelectElement, status: HTMLElement) => void | Promise<void>} onScriptChange
 */

/**
 * @param {HTMLElement} element
 * @param {StoredWallet} wallet
 * @param {WalletSettingsOptions} options
 */
export function renderWalletSettings(element, wallet, options) {
  if (isOutputDescriptor(wallet.xpub)) {
    element.replaceChildren();
    return;
  }

  const script = createAddressScriptSelect(
    /** @type {AddressScript} */ (wallet.script),
  );
  const status = createElement("p", "wallets__status wallets__settings-status");

  status.setAttribute("role", "status");
  script.addEventListener("change", () => {
    void options.onScriptChange(readAddressScript(script), script, status);
  });
  element.replaceChildren(createField("Address type", script), status);
}
