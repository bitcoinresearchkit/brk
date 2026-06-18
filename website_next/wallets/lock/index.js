import { createElement } from "../dom.js";

/**
 * @typedef {Object} LockOptions
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => void | Promise<void>} onUnlock
 * @property {() => void} onReset
 */

const RESET_HOLD_MS = 2_000;

/**
 * @param {HTMLButtonElement} button
 * @param {() => void} onReset
 */
function bindResetHold(button, onReset) {
  /** @type {number | undefined} */
  let timer;

  function cancel() {
    if (timer === undefined) return;

    clearTimeout(timer);
    timer = undefined;
    button.removeAttribute("data-wallets-holding");
  }

  function start() {
    if (timer !== undefined) return;

    button.setAttribute("data-wallets-holding", "");
    timer = window.setTimeout(() => {
      timer = undefined;
      button.removeAttribute("data-wallets-holding");
      onReset();
    }, RESET_HOLD_MS);
  }

  button.addEventListener("pointerdown", (event) => {
    if (event.button !== 0) return;

    button.setPointerCapture(event.pointerId);
    start();
  });
  button.addEventListener("pointerup", cancel);
  button.addEventListener("pointercancel", cancel);
  button.addEventListener("lostpointercapture", cancel);
  button.addEventListener("keydown", (event) => {
    if (event.repeat || (event.key !== " " && event.key !== "Enter")) return;

    event.preventDefault();
    start();
  });
  button.addEventListener("keyup", (event) => {
    if (event.key === " " || event.key === "Enter") {
      cancel();
    }
  });
  button.addEventListener("blur", cancel);
}

/**
 * @param {LockOptions} options
 */
export function createLock(options) {
  const section = createElement("section", "wallets__unlock");
  const title = document.createElement("h1");
  const form = document.createElement("form");
  const password = document.createElement("input");
  const button = document.createElement("button");
  const reset = document.createElement("button");
  const status = document.createElement("p");

  title.append("Unlock vault");
  password.name = "password";
  password.type = "password";
  password.autocomplete = "current-password";
  password.autofocus = true;
  password.placeholder = "Password";
  password.required = true;
  button.type = "submit";
  button.append("Unlock");
  reset.type = "button";
  reset.append("Reset vault");
  status.setAttribute("role", "status");
  form.append(password, button);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onUnlock(password.value, button, status);
  });
  bindResetHold(reset, options.onReset);
  section.append(title, form, reset, status);
  queueMicrotask(() => {
    password.focus({ preventScroll: true });
  });

  return section;
}
