import { createField } from "../form/index.js";
import { redaction } from "../redaction/index.js";

/**
 * @typedef {Object} AddWalletFormSubmit
 * @property {HTMLInputElement} name
 * @property {HTMLInputElement} source
 * @property {HTMLButtonElement} submit
 * @property {HTMLElement} status
 * @property {HTMLFormElement} form
 */

/**
 * @typedef {Object} AddWalletFormOptions
 * @property {() => void} onCancel
 * @property {(submit: AddWalletFormSubmit) => void | Promise<void>} onSubmit
 */

function createSourceInput() {
  const input = document.createElement("input");

  input.name = "source";
  input.type = redaction.isHidden() ? "password" : "text";
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
export function createAddForm(options) {
  const form = document.createElement("form");
  const title = document.createElement("h2");
  const name = document.createElement("input");
  const source = createSourceInput();
  const actions = document.createElement("div");
  const cancel = document.createElement("button");
  const submit = document.createElement("button");
  const status = document.createElement("p");
  const fields = [
    createField("name", name),
    createField("xpub or descriptor", source),
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
      source,
      submit,
      status,
      form,
    });
  });

  return form;
}
