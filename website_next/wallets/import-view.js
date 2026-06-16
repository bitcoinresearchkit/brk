import {
  createElement,
  createField,
} from "./dom.js";
import { arePrivateValuesHidden } from "./privacy-view.js";

/**
 * @typedef {Object} AddWalletFormSubmit
 * @property {HTMLInputElement} name
 * @property {HTMLInputElement} xpub
 * @property {HTMLButtonElement} submit
 * @property {HTMLElement} status
 * @property {HTMLFormElement} form
 */

/**
 * @typedef {Object} AddWalletFormOptions
 * @property {() => void} onCancel
 * @property {(submit: AddWalletFormSubmit) => void | Promise<void>} onSubmit
 */

function createXpubInput() {
  const input = document.createElement("input");

  input.name = "xpub";
  input.type = arePrivateValuesHidden() ? "password" : "text";
  input.setAttribute("data-wallets-private-input", "");
  input.autocomplete = "off";
  input.placeholder = "xpub or descriptor...";
  input.required = true;
  input.spellcheck = false;

  return input;
}

/**
 * @param {AddWalletFormOptions} options
 */
export function createAddWalletForm(options) {
  const form = createElement("form", "wallets__dialog-form");
  const title = document.createElement("h2");
  const name = document.createElement("input");
  const xpub = createXpubInput();
  const actions = createElement("div", "wallets__dialog-actions");
  const cancel = document.createElement("button");
  const submit = document.createElement("button");
  const status = createElement("p", "wallets__status");
  const fields = [
    createField("name", name),
    createField("xpub or descriptor", xpub),
  ];

  title.append("Watch wallet");
  name.name = "name";
  name.autocomplete = "off";
  name.placeholder = "Wallet name";
  name.required = true;
  cancel.type = "button";
  cancel.append("Cancel");
  submit.type = "submit";
  submit.append("Add");
  status.setAttribute("role", "status");
  actions.append(cancel, submit);
  form.append(
    title,
    ...fields,
    actions,
    status,
  );
  cancel.addEventListener("click", options.onCancel);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onSubmit({
      name,
      xpub,
      submit,
      status,
      form,
    });
  });

  return form;
}
