import { createElement } from "../dom.js";

/**
 * @typedef {Object} SetupOptions
 * @property {(password: string, button: HTMLButtonElement, status: HTMLElement) => void | Promise<void>} onCreate
 */

/**
 * @param {string} text
 */
function createDescriptionText(text) {
  const paragraph = document.createElement("p");

  paragraph.append(text);

  return paragraph;
}

/**
 * @param {SetupOptions} options
 */
export function createSetup(options) {
  const section = createElement("section", "wallets__setup");
  const title = document.createElement("h1");
  const description = document.createElement("article");
  const form = document.createElement("form");
  const password = document.createElement("input");
  const button = document.createElement("button");
  const status = document.createElement("output");

  title.append("Wallets");
  description.append(
    createDescriptionText(
      "A privacy-preserving xpub viewer that runs in your browser and never uploads your xpub.",
    ),
    createDescriptionText(
      "Import an xpub or watch-only descriptor to view a Bitcoin wallet without spending access.",
    ),
    createDescriptionText(
      "Addresses are derived locally, checked through prefix buckets, and saved encrypted in this browser.",
    ),
    createDescriptionText(
      "Privacy benefits can be drastically reduced if those addresses are already linked together on-chain.",
    ),
  );
  password.name = "password";
  password.type = "password";
  password.autocomplete = "new-password";
  password.placeholder = "Set password";
  password.required = true;
  button.type = "submit";
  button.classList.add("primary");
  button.append("Continue");
  form.append(password, button);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onCreate(password.value, button, status);
  });
  section.append(title, description, form, status);

  return section;
}
