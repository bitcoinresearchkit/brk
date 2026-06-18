/**
 * @param {string} label
 * @param {HTMLInputElement | HTMLSelectElement} control
 */
export function createField(label, control) {
  const element = document.createElement("label");
  const text = document.createElement("span");

  text.append(label);
  element.append(text, control);

  return element;
}
