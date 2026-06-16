const FIXED_PRIVATE_TEXT = "*****";

let privateValuesHidden = false;

export function arePrivateValuesHidden() {
  return privateValuesHidden;
}

/**
 * @param {string} value
 */
export function createPrivateText(value) {
  return [...value].map((character) => {
    return character === " " ? " " : "*";
  }).join("");
}

/**
 * @param {string} value
 * @param {string | null} mode
 */
function maskPrivateText(value, mode) {
  return mode === "fixed" ? FIXED_PRIVATE_TEXT : createPrivateText(value);
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 * @param {"exact" | "fixed"} [mode]
 */
export function setPrivateValue(element, value, mode = "exact") {
  element.setAttribute("data-wallets-private-value", value);
  element.setAttribute("data-wallets-private-mode", mode);
  element.textContent = privateValuesHidden ? maskPrivateText(value, mode) : value;
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 */
export function setPrivateTitle(element, value) {
  element.setAttribute("data-wallets-private-title", value);
  element.title = privateValuesHidden ? createPrivateText(value) : value;
}

/**
 * @template {keyof HTMLElementTagNameMap} Tag
 * @param {Tag} tag
 * @param {string} value
 * @param {"exact" | "fixed"} [mode]
 */
export function createPrivateValue(tag, value, mode = "exact") {
  const element = document.createElement(tag);

  setPrivateValue(element, value, mode);

  return element;
}

/**
 * @param {HTMLElement} root
 * @param {(text: string) => HTMLElement} createAddress
 */
export function syncPrivateValues(root, createAddress) {
  const values = root.querySelectorAll("[data-wallets-private-value]");
  const titles = root.querySelectorAll("[data-wallets-private-title]");
  const addresses = root.querySelectorAll("[data-wallets-private-address]");
  const inputs = root.querySelectorAll("[data-wallets-private-input]");

  for (const value of values) {
    const text = value.getAttribute("data-wallets-private-value") ?? "";
    const mode = value.getAttribute("data-wallets-private-mode");

    value.textContent = privateValuesHidden
      ? maskPrivateText(text, mode)
      : text;
  }

  for (const element of titles) {
    const title = /** @type {HTMLElement} */ (element);
    const text = title.getAttribute("data-wallets-private-title") ?? "";

    title.title = privateValuesHidden
      ? createPrivateText(text)
      : text;
  }

  for (const address of addresses) {
    const text = address.getAttribute("data-wallets-private-address") ?? "";
    const next = privateValuesHidden ? createPrivateText(text) : text;

    address.replaceChildren(...createAddress(next).childNodes);
  }

  for (const input of inputs) {
    if (input instanceof HTMLInputElement) {
      input.type = privateValuesHidden ? "password" : "text";
    }
  }
}

/**
 * @param {HTMLButtonElement} button
 */
export function syncPrivacyButton(button) {
  button.textContent = privateValuesHidden ? "Reveal" : "Privacy";
  button.setAttribute("aria-pressed", privateValuesHidden ? "true" : "false");
}

/**
 * @param {HTMLElement} root
 * @param {HTMLButtonElement} button
 * @param {(text: string) => HTMLElement} createAddress
 */
export function togglePrivateValues(root, button, createAddress) {
  privateValuesHidden = !privateValuesHidden;
  syncPrivateValues(root, createAddress);
  syncPrivacyButton(button);
}
