import { createAddressCellContent } from "./address-view.js";
import { createElement } from "./dom.js";
import { formatBtc } from "./format.js";
import {
  createHistoryContent,
  createHistoryMessage,
  createHistoryRow,
  replaceHistoryRowContent,
} from "./history-view.js";
import { createPrivateValue } from "./privacy-view.js";

const ADDRESS_TABLE_COLUMN_COUNT = 3;

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {number} balance
 * @property {number} txCount
 * @property {string[]} historyAddresses
 * @property {number} historyBucketSize
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
 * @param {WalletAddress} address
 * @param {HTMLTableRowElement} parent
 * @param {AddressTableOptions} options
 */
function createHistoryButton(address, parent, options) {
  if (address.txCount === 0) {
    return "";
  }

  const button = document.createElement("button");
  /** @type {HTMLTableRowElement | undefined} */
  let historyRow;

  button.type = "button";
  button.className = "wallets__history-button";
  button.append("History");

  button.addEventListener("click", async () => {
    if (historyRow?.isConnected) {
      historyRow.remove();
      button.textContent = "History";
      return;
    }

    button.disabled = true;
    button.textContent = "Loading";
    historyRow = createHistoryRow(
      createHistoryMessage("Loading history"),
      ADDRESS_TABLE_COLUMN_COUNT,
    );
    parent.after(historyRow);

    try {
      const history = await options.fetchHistory(address);

      replaceHistoryRowContent(
        historyRow,
        createHistoryContent(history, address.address),
        ADDRESS_TABLE_COLUMN_COUNT,
      );
      button.textContent = "Hide";
    } catch (error) {
      replaceHistoryRowContent(
        historyRow,
        createHistoryMessage(options.getErrorMessage(error)),
        ADDRESS_TABLE_COLUMN_COUNT,
      );
      button.textContent = "History";
    } finally {
      button.disabled = false;
    }
  });

  return button;
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
    appendCell(item, createPrivateValue("span", formatBtc(row.balance), "fixed"));
    appendCell(item, createHistoryButton(row, item, options));
    body.append(item);
  }

  table.append(head, body);

  return table;
}
