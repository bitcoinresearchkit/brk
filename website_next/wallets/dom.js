/**
 * @template {keyof HTMLElementTagNameMap} Tag
 * @param {Tag} tag
 * @param {string} className
 */
export function createElement(tag, className) {
  const element = document.createElement(tag);

  element.className = className;

  return element;
}

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

/**
 * @param {HTMLButtonElement} button
 * @param {boolean} busy
 * @param {string} idleLabel
 * @param {string} busyLabel
 */
export function setBusy(button, busy, idleLabel, busyLabel) {
  button.disabled = busy;
  button.ariaBusy = busy ? "true" : "false";
  button.textContent = busy ? busyLabel : idleLabel;
}

/**
 * @param {HTMLElement} status
 * @param {string} text
 */
export function setStatus(status, text) {
  status.textContent = text;
}
