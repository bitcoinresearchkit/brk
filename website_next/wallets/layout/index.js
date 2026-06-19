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
  const header = document.createElement("header");
  const actions = document.createElement("menu");
  const addButton = document.createElement("button");
  const privacyButton = document.createElement("button");
  const lockButton = document.createElement("button");
  const selector = createElement("section", "wallets__selector");
  const walletList = document.createElement("nav");
  const content = document.createElement("article");
  const addDialog = createElement("dialog", "wallets__dialog");

  addButton.type = "button";
  addButton.classList.add("primary");
  addButton.append("Add watch-only wallet");
  privacyButton.type = "button";
  privacyButton.classList.add("primary");
  lockButton.type = "button";
  lockButton.classList.add("primary");
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
