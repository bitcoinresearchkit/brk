import {
  createAddressScriptSelect,
  readAddressScript,
} from "./script.js";
import { createElement } from "../../dom.js";
import { createField } from "../../form/index.js";
import { isOutputDescriptor } from "../../derive/index.js";

/**
 * @typedef {import("../../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../../vault/index.js").StoredWallet} StoredWallet
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
export function renderSettings(element, wallet, options) {
  if (isOutputDescriptor(wallet.source)) {
    element.replaceChildren();
    return;
  }

  const script = createAddressScriptSelect(
    /** @type {AddressScript} */ (wallet.script),
  );
  const status = createElement("p", "wallets__status");

  status.setAttribute("role", "status");
  script.addEventListener("change", () => {
    void options.onScriptChange(readAddressScript(script), script, status);
  });
  element.replaceChildren(createField("Address type", script), status);
}
