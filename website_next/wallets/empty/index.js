import { createElement } from "../dom.js";

/**
 * @typedef {Object} EmptyOptions
 * @property {() => void} onAdd
 */

/**
 * @param {EmptyOptions} options
 */
export function createEmpty(options) {
  const empty = createElement("section", "wallets__empty");
  const text = document.createElement("p");
  const button = document.createElement("button");

  text.append("No wallet imported yet");
  button.type = "button";
  button.append("Add wallet");
  button.addEventListener("click", options.onAdd);
  empty.append(text, button);

  return empty;
}
