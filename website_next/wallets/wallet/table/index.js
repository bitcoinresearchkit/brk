import { createAddressCellContent } from "../address/index.js";
import { createElement } from "../../dom.js";
import { formatBtc } from "../../format.js";
import { createAddressHistoryButton } from "../history/button.js";
import { redaction } from "../../redaction/index.js";

const ADDRESS_TABLE_COLUMN_COUNT = 3;

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} AddressHistory
 * @property {unknown[]} transactions
 */

/**
 * @typedef {Object} AddressTableOptions
 * @property {(address: WalletAddress) => Promise<AddressHistory>} fetchHistory
 * @property {(error: unknown) => string} getErrorMessage
 */

/**
 * @param {HTMLTableRowElement} row
 * @param {Node | string} value
 */
function appendCell(row, value) {
  const cell = document.createElement("td");

  cell.append(value);
  row.append(cell);
}

/**
 * @param {WalletAddress[]} addresses
 * @param {AddressTableOptions} options
 * @returns {HTMLTableElement}
 */
export function createAddressTable(addresses, options) {
  const table = createElement("table", "wallets__table");
  const head = document.createElement("thead");
  const body = document.createElement("tbody");
  const header = document.createElement("tr");

  for (const value of [
    "address",
    "balance",
    "history",
  ]) {
    const cell = document.createElement("th");

    cell.scope = "col";
    cell.append(value);
    header.append(cell);
  }

  head.append(header);

  for (const row of addresses) {
    const item = document.createElement("tr");

    appendCell(item, createAddressCellContent(row));
    appendCell(
      item,
      redaction.createValue("span", formatBtc(row.balance), "fixed"),
    );
    appendCell(
      item,
      createAddressHistoryButton(row, item, options, ADDRESS_TABLE_COLUMN_COUNT),
    );
    body.append(item);
  }

  table.append(head, body);

  return table;
}
