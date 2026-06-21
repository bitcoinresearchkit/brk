import { createElement } from "../dom.js";

/**
 * @typedef {Object} EmptyOptions
 * @property {() => void} onAdd
 * @property {() => void} [onClear]
 */

/**
 * @param {EmptyOptions} options
 */
export function createEmpty(options) {
  const empty = createElement("section", "empty");
  const title = document.createElement("h1");
  const text = document.createElement("p");
  const actions = document.createElement("menu");
  const button = document.createElement("button");

  title.append("No wallet yet");
  text.append("Import an xpub or watch-only descriptor to start watching activity.");
  button.type = "button";
  button.append("Add wallet");
  button.addEventListener("click", options.onAdd);
  actions.append(button);

  if (options.onClear) {
    const clear = document.createElement("button");

    clear.type = "button";
    clear.append("Clear");
    clear.addEventListener("click", options.onClear);
    actions.append(clear);
  }

  empty.append(title, text, actions);

  return empty;
}
