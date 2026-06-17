import { createElement } from "../dom.js";

/**
 * @typedef {Object} LockOptions
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => void | Promise<void>} onUnlock
 * @property {() => void} onReset
 */

/**
 * @param {LockOptions} options
 */
export function createLock(options) {
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
