import { addressScripts } from "../../derive/script.js";

/**
 * @typedef {import("../../derive/address.js").AddressScript} AddressScript
 */

/**
 * @param {AddressScript} [value]
 */
export function createAddressScriptSelect(value) {
  const select = document.createElement("select");

  select.name = "script";

  for (const { id, label } of addressScripts) {
    const option = document.createElement("option");

    option.value = id;
    option.selected = id === value;
    option.append(label);
    select.append(option);
  }

  return select;
}

/**
 * @param {HTMLSelectElement} select
 * @returns {AddressScript}
 */
export function readAddressScript(select) {
  return /** @type {AddressScript} */ (select.value);
}
