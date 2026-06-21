const FIXED_PRIVATE_TEXT = "*****";

let hidden = false;
const effects = /** @type {RedactionEffect[]} */ ([]);

/**
 * @typedef {"exact" | "fixed"} RedactionMode
 *
 * @typedef {Object} RedactionEffect
 * @property {HTMLElement} element
 * @property {() => void} sync
 */

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
 * @param {RedactionMode} mode
 */
function mask(value, mode) {
  return mode === "fixed" ? FIXED_PRIVATE_TEXT : createText(value);
}

/**
 * @param {HTMLElement} element
 * @param {() => void} sync
 */
function addEffect(element, sync) {
  effects.push({ element, sync });
  sync();
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 * @param {RedactionMode} [mode]
 */
function setValue(element, value, mode = "exact") {
  addEffect(element, () => {
    element.textContent = hidden ? mask(value, mode) : value;
  });
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 */
function setTitle(element, value) {
  addEffect(element, () => {
    element.title = hidden ? createText(value) : value;
  });
}

/**
 * @param {HTMLElement} element
 * @param {string} value
 * @param {(text: string) => void} render
 */
function setAddress(element, value, render) {
  addEffect(element, () => {
    render(hidden ? createText(value) : value);
  });
}

/**
 * @param {HTMLInputElement | HTMLTextAreaElement} input
 */
function setInput(input) {
  addEffect(input, () => {
    if (input instanceof HTMLTextAreaElement) {
      input.style.setProperty("-webkit-text-security", hidden ? "disc" : "");
    } else {
      input.type = hidden ? "password" : "text";
    }
  });
}

/**
 * @template {keyof HTMLElementTagNameMap} Tag
 * @param {Tag} tag
 * @param {string} value
 * @param {RedactionMode} [mode]
 */
function createValue(tag, value, mode = "exact") {
  const element = document.createElement(tag);

  setValue(element, value, mode);

  return element;
}

function sync() {
  for (let index = effects.length - 1; index >= 0; index -= 1) {
    const effect = effects[index];

    if (!effect.element.isConnected) {
      effects.splice(index, 1);
    } else {
      effect.sync();
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
 * @param {HTMLButtonElement} button
 */
function toggle(button) {
  hidden = !hidden;
  sync();
  syncButton(button);
}

export const redaction = /** @type {const} */ ({
  isHidden,
  createText,
  setValue,
  setTitle,
  setAddress,
  setInput,
  createValue,
  syncButton,
  toggle,
});
