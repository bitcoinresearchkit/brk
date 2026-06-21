import { createField } from "../form/index.js";
import { createElement } from "../dom.js";
import { redaction } from "../redaction/index.js";

/**
 * @typedef {Object} AddWalletFormSubmit
 * @property {HTMLInputElement} name
 * @property {HTMLTextAreaElement} source
 * @property {HTMLButtonElement} submit
 * @property {HTMLFormElement} form
 */

/**
 * @typedef {Object} AddWalletFormOptions
 * @property {() => void} onCancel
 * @property {(submit: AddWalletFormSubmit) => void | Promise<void>} onSubmit
 */

function createSourceInput() {
  const input = document.createElement("textarea");

  input.name = "source";
  redaction.setInput(input);
  input.autocomplete = "off";
  input.placeholder = "xpub... or wsh(sortedmulti(...))";
  input.required = true;
  input.spellcheck = false;
  input.rows = 4;

  return input;
}

/**
 * @param {AddWalletFormOptions} options
 */
export function createAddForm(options) {
  const form = createElement("form", "add");
  const title = document.createElement("h2");
  const description = document.createElement("p");
  const name = document.createElement("input");
  const source = createSourceInput();
  const actions = document.createElement("footer");
  const cancel = document.createElement("button");
  const submit = document.createElement("button");
  const fields = [
    createField("name", name),
    createField("xpub or descriptor", source),
  ];

  title.append("Add wallet");
  description.append(
    "Import an xpub or watch-only descriptor. Spending keys are never needed.",
  );
  name.name = "name";
  name.autocomplete = "off";
  name.placeholder = "Wallet 1";
  name.required = true;
  cancel.type = "button";
  cancel.append("Cancel");
  submit.type = "submit";
  submit.append("Add");
  actions.append(cancel, submit);
  form.append(
    title,
    description,
    ...fields,
    actions,
  );
  cancel.addEventListener("click", options.onCancel);
  source.addEventListener("input", () => {
    source.removeAttribute("aria-invalid");
  });
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    void options.onSubmit({
      name,
      source,
      submit,
      form,
    });
  });

  return form;
}
