export const addressScripts = /** @type {const} */ ([
  { id: "v0_p2wpkh", label: "P2WPKH" },
  { id: "v1_p2tr", label: "P2TR" },
  { id: "p2sh_p2wpkh", label: "Nested P2WPKH" },
  { id: "p2pkh", label: "P2PKH" },
]);

/**
 * @typedef {typeof addressScripts[number]["id"]} AddressScript
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
