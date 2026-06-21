import { createElement } from "../dom.js";

/**
 * @typedef {Object} WalletsLayout
 * @property {HTMLElement} main
 * @property {HTMLElement} header
 * @property {HTMLButtonElement} addButton
 * @property {HTMLButtonElement} privacyButton
 * @property {HTMLButtonElement} sessionButton
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
  const sessionButton = document.createElement("button");
  const selector = createElement("section", "wallets__selector");
  const walletList = document.createElement("nav");
  const content = document.createElement("article");
  const addDialog = document.createElement("dialog");

  addButton.type = "button";
  addButton.append("Add watch-only wallet");
  privacyButton.type = "button";
  sessionButton.type = "button";
  sessionButton.append("Lock");
  content.setAttribute("aria-live", "polite");
  walletList.setAttribute("tabindex", "0");
  walletList.setAttribute("aria-label", "Wallets");
  actions.append(addButton, privacyButton, sessionButton);
  header.append(actions);
  selector.append(walletList);
  main.append(header, selector, content, addDialog);

  return {
    main,
    header,
    addButton,
    privacyButton,
    sessionButton,
    selector,
    walletList,
    content,
    addDialog,
  };
}
