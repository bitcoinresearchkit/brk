import { createElement } from "../dom.js";

/**
 * @typedef {Object} WalletsLayout
 * @property {HTMLElement} main
 * @property {HTMLElement} header
 * @property {HTMLButtonElement} addButton
 * @property {HTMLButtonElement} privacyButton
 * @property {HTMLButtonElement} lockButton
 * @property {HTMLElement} selector
 * @property {HTMLElement} walletList
 * @property {HTMLElement} content
 * @property {HTMLDialogElement} addDialog
 */

/**
 * @returns {WalletsLayout}
 */
export function createLayout() {
  const main = createElement("main", "wallets");
  const header = createElement("header", "wallets__header");
  const actions = createElement("div", "wallets__actions");
  const addButton = document.createElement("button");
  const privacyButton = document.createElement("button");
  const lockButton = document.createElement("button");
  const selector = createElement("section", "wallets__selector");
  const walletList = createElement("div", "wallets__wallet-list");
  const content = createElement("section", "wallets__content");
  const addDialog = createElement("dialog", "wallets__dialog");

  addButton.type = "button";
  addButton.append("Add watch-only wallet");
  privacyButton.type = "button";
  lockButton.type = "button";
  lockButton.append("Lock");
  content.setAttribute("aria-live", "polite");
  walletList.setAttribute("tabindex", "0");
  walletList.setAttribute("aria-label", "Wallets");
  actions.append(addButton, privacyButton, lockButton);
  header.append(actions);
  selector.append(walletList);
  main.append(header, selector, content, addDialog);

  return {
    main,
    header,
    addButton,
    privacyButton,
    lockButton,
    selector,
    walletList,
    content,
    addDialog,
  };
}
