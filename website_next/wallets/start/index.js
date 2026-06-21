import { createElement } from "../dom.js";
import { createResetButton } from "./reset/index.js";

/**
 * @typedef {"create" | "unlock"} StartMode
 */

/**
 * @typedef {Object} StartOptions
 * @property {StartMode} mode
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => boolean | void | Promise<boolean | void>} onPassword
 * @property {() => void} onEphemeral
 * @property {() => void} [onReset]
 */


/**
 * @param {StartOptions} options
 */
export function createStart(options) {
  const section = createElement("section", "start");
  const story = document.createElement("article");
  const title = document.createElement("h1");
  const titleBreak = document.createElement("br");
  const titleAccent = document.createElement("span");
  const lead = document.createElement("p");
  const details = document.createElement("ul");
  const warningRule = document.createElement("hr");
  const warning = document.createElement("p");
  const modes = document.createElement("div");
  const persistent = document.createElement("section");
  const persistentTitle = document.createElement("h2");
  const persistentText = document.createElement("p");
  const form = document.createElement("form");
  const password = document.createElement("input");
  const submit = document.createElement("button");
  const divider = document.createElement("p");
  const temporary = document.createElement("section");
  const temporaryTitle = document.createElement("h2");
  const temporaryText = document.createElement("p");
  const temporaryButton = document.createElement("button");
  const status = document.createElement("output");
  const unlock = options.mode === "unlock";

  titleAccent.append("wallets");
  title.append("Watch-only", titleBreak, titleAccent);
  lead.append("View a Bitcoin wallet privately, without spending access.");
  details.append(
    createDetail("Open xpubs and watch-only descriptors."),
    createDetail("Addresses are derived on your device."),
    createDetail("Anonymity sets increase lookup privacy."),
    createDetail("Save encrypted wallets, or use a temporary session."),
  );
  warning.append(
    "Use a VPN for extra network privacy.",
    document.createElement("br"),
    "On-chain address links will reduce anonymity.",
  );
  story.append(title, lead, details, warningRule, warning);
  persistentTitle.append("Persistent vault");
  persistentText.append(
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
  persistent.append(persistentTitle, persistentText, form);

  if (options.onReset) {
    persistent.append(createResetButton(options.onReset));
  }

  divider.append("OR");
  temporaryTitle.append("Temporary vault");
  temporaryText.append("Wallets are never saved to this browser.");
  temporaryButton.type = "button";
  temporaryButton.append("Start temporary");
  temporaryButton.addEventListener("click", () => {
    options.onEphemeral();
  });
  temporary.append(temporaryTitle, temporaryText, temporaryButton);
  persistent.append(status);
  modes.append(persistent, divider, temporary);
  section.append(story, modes);
  queueMicrotask(() => {
    password.focus({ preventScroll: true });
  });

  return section;
}

/**
 * @param {string} text
 */
function createDetail(text) {
  const item = document.createElement("li");

  item.append(text);

  return item;
}
