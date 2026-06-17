const FIXED_PRIVATE_TEXT = "*****";

let hidden = false;

function isHidden() {
  return hidden;
}

/**
 * @param {string} value
 */
function createText(value) {
  return [...value].map((character) => {
    return character === " " ? " " : "*";
  }).join("");
}

/**
 * @param {string} value
 * @param {string | null} mode
 */
function mask(value, mode) {
  return mode === "fixed" ? FIXED_PRIVATE_TEXT : createText(value);
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 * @param {"exact" | "fixed"} [mode]
 */
function setValue(element, value, mode = "exact") {
  element.setAttribute("data-wallets-private-value", value);
  element.setAttribute("data-wallets-private-mode", mode);
  element.textContent = hidden
    ? mask(value, mode)
    : value;
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 */
function setTitle(element, value) {
  element.setAttribute("data-wallets-private-title", value);
  element.title = hidden ? createText(value) : value;
}

/**
 * @template {keyof HTMLElementTagNameMap} Tag
 * @param {Tag} tag
 * @param {string} value
 * @param {"exact" | "fixed"} [mode]
 */
function createValue(tag, value, mode = "exact") {
  const element = document.createElement(tag);

  setValue(element, value, mode);

  return element;
}

/**
 * @param {HTMLElement} root
 * @param {(text: string) => HTMLElement} createAddress
 */
function sync(root, createAddress) {
  const values = root.querySelectorAll("[data-wallets-private-value]");
  const titles = root.querySelectorAll("[data-wallets-private-title]");
  const addresses = root.querySelectorAll("[data-wallets-private-address]");
  const inputs = root.querySelectorAll("[data-wallets-private-input]");

  for (const value of values) {
    const text = value.getAttribute("data-wallets-private-value") ?? "";
    const mode = value.getAttribute("data-wallets-private-mode");

    value.textContent = hidden
      ? mask(text, mode)
      : text;
  }

  for (const element of titles) {
    const title = /** @type {HTMLElement} */ (element);
    const text = title.getAttribute("data-wallets-private-title") ?? "";

    title.title = hidden
      ? createText(text)
      : text;
  }

  for (const address of addresses) {
    const text = address.getAttribute("data-wallets-private-address") ?? "";
    const next = hidden ? createText(text) : text;

    address.replaceChildren(...createAddress(next).childNodes);
  }

  for (const input of inputs) {
    if (input instanceof HTMLInputElement) {
      input.type = hidden ? "password" : "text";
    }
  }
}

/**
 * @param {HTMLButtonElement} button
 */
function syncButton(button) {
  button.textContent = hidden ? "Reveal" : "Privacy";
  button.setAttribute("aria-pressed", hidden ? "true" : "false");
}

/**
 * @param {HTMLElement} root
 * @param {HTMLButtonElement} button
 * @param {(text: string) => HTMLElement} createAddress
 */
function toggle(root, button, createAddress) {
  hidden = !hidden;
  sync(root, createAddress);
  syncButton(button);
}

export const redaction = /** @type {const} */ ({
  isHidden,
  createText,
  setValue,
  setTitle,
  createValue,
  syncButton,
  toggle,
});
