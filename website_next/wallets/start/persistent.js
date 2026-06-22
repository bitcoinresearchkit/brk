import { createResetButton } from "./reset/index.js";

/**
 * @typedef {"create" | "unlock"} StartMode
 *
 * @typedef {Object} PersistentOptions
 * @property {StartMode} mode
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => boolean | void | Promise<boolean | void>} onPassword
 * @property {() => void} [onReset]
 */

/**
 * @param {PersistentOptions} options
 */
export function createPersistentVault(options) {
  const persistent = document.createElement("section");
  const title = document.createElement("h2");
  const text = document.createElement("p");
  const form = document.createElement("form");
  const password = document.createElement("input");
  const submit = document.createElement("button");
  const status = document.createElement("output");
  const unlock = options.mode === "unlock";

  title.append("Persistent vault");
  text.append(
    unlock
      ? "Unlock the encrypted vault saved in this browser."
      : "Create an encrypted vault saved in this browser.",
  );
  password.name = "password";
  password.type = "password";
  password.autocomplete = unlock ? "current-password" : "new-password";
  password.autofocus = true;
  password.placeholder = unlock ? "Password" : "Set password";
  password.required = true;
  submit.type = "submit";
  submit.append(unlock ? "Unlock" : "Create");
  form.append(password, submit);

  function clearInvalid() {
    password.removeAttribute("aria-invalid");
  }

  password.addEventListener("input", clearInvalid);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    clearInvalid();
    void (async () => {
      const valid = await options.onPassword(password.value, submit, status);

      if (valid === false) {
        password.setAttribute("aria-invalid", "true");
        password.focus({ preventScroll: true });
      }
    })();
  });

  persistent.append(title, text, form);

  if (options.onReset) {
    persistent.append(createResetButton(options.onReset));
  }

  persistent.append(status);

  return {
    element: persistent,
    password,
  };
}
