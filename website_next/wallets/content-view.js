import { createElement } from "./dom.js";

/**
 * @typedef {Object} EmptyWalletViewOptions
 * @property {() => void} onAdd
 */

/**
 * @typedef {Object} LockedWalletViewOptions
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => void | Promise<void>} onUnlock
 * @property {() => void} onReset
 */

/**
 * @typedef {Object} SetupWalletViewOptions
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => void | Promise<void>} onCreate
 */

/**
 * @typedef {Object} UnlockedWalletView
 * @property {HTMLElement} settings
 * @property {HTMLElement} summary
 * @property {HTMLElement} status
 * @property {HTMLElement} results
 * @property {HTMLElement[]} nodes
 */

/**
 * @param {EmptyWalletViewOptions} options
 */
export function createEmptyWalletView(options) {
  const empty = createElement("section", "wallets__empty");
  const text = document.createElement("p");
  const button = document.createElement("button");

  text.append("No watch-only wallets yet");
  button.type = "button";
  button.append("Add watch-only wallet");
  button.addEventListener("click", options.onAdd);
  empty.append(text, button);

  return empty;
}

/**
 * @param {SetupWalletViewOptions} options
 */
export function createSetupWalletView(options) {
  const section = createElement("section", "wallets__setup");
  const title = document.createElement("h1");
  const description = createElement("div", "wallets__setup-description");
  const form = createElement("form", "wallets__setup-form");
  const password = document.createElement("input");
  const button = document.createElement("button");
  const status = createElement("p", "wallets__status");

  title.append("Wallets");
  description.append(
    createDescriptionText("Import an extended public key, often called an xpub, or a watch-only descriptor to view a Bitcoin wallet without giving this site spending access."),
    createDescriptionText("Your wallet sources stay in this browser and are encrypted before they are saved. Set a password for this local wallet vault."),
    createDescriptionText("If you forget the password, you can reset the vault and import the xpubs or descriptors again."),
  );
  password.name = "password";
  password.type = "password";
  password.autocomplete = "new-password";
  password.placeholder = "Set password";
  password.required = true;
  button.type = "submit";
  button.append("Continue");
  status.setAttribute("role", "status");
  form.append(password, button);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onCreate(password.value, button, status);
  });
  section.append(title, description, form, status);

  return section;
}

/**
 * @param {LockedWalletViewOptions} options
 */
export function createLockedWalletView(options) {
  const section = createElement("section", "wallets__unlock");
  const form = createElement("form", "wallets__unlock-form");
  const password = document.createElement("input");
  const button = document.createElement("button");
  const reset = document.createElement("button");
  const status = createElement("p", "wallets__status");

  password.name = "password";
  password.type = "password";
  password.autocomplete = "current-password";
  password.placeholder = "Password";
  password.required = true;
  button.type = "submit";
  button.append("Unlock");
  reset.type = "button";
  reset.className = "wallets__reset";
  reset.append("Reset vault");
  status.setAttribute("role", "status");
  form.append(password, button);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onUnlock(password.value, button, status);
  });
  reset.addEventListener("click", options.onReset);
  section.append(form, reset, status);

  return section;
}

/**
 * @param {string} text
 */
function createDescriptionText(text) {
  const paragraph = document.createElement("p");

  paragraph.append(text);

  return paragraph;
}

/**
 * @returns {UnlockedWalletView}
 */
export function createUnlockedWalletView() {
  const settings = createElement("section", "wallets__settings");
  const summary = createElement("section", "wallets__summary");
  const status = createElement("p", "wallets__status");
  const results = createElement("section", "wallets__results");

  settings.setAttribute("aria-label", "Wallet settings");
  status.setAttribute("role", "status");
  summary.setAttribute("aria-label", "Wallets summary");
  results.setAttribute("aria-label", "Wallets results");

  return {
    settings,
    summary,
    status,
    results,
    nodes: [settings, summary, status, results],
  };
}
