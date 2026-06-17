import { createElement } from "../dom.js";

/**
 * @param {string} label
 * @param {HTMLInputElement | HTMLSelectElement} control
 */
export function createField(label, control) {
  const element = createElement("label", "wallets__field");
  const text = createElement("span", "wallets__label");

  text.append(label);
  element.append(text, control);

  return element;
}
