import { createElement } from "../../dom.js";
import { formatBtc } from "../../format.js";
import { redaction } from "../../redaction/index.js";
import { readHistoryTransaction } from "./transaction.js";

/**
 * @typedef {Object} AddressHistory
 * @property {unknown[]} transactions
 */

/**
 * @param {string} txid
 */
function formatTxid(txid) {
  return txid.length > 16 ? `${txid.slice(0, 8)}...${txid.slice(-8)}` : txid;
}

/**
 * @param {AddressHistory} history
 * @param {string} address
 */
export function createHistoryContent(history, address) {
  const element = createElement("div", "wallets__history");
  const list = createElement("ol", "wallets__history-list");

  for (const transaction of history.transactions) {
    const item = document.createElement("li");
    const txid = document.createElement("code");
    const date = document.createElement("span");
    const direction = document.createElement("span");
    const amount = document.createElement("strong");
    const fee = document.createElement("span");
    const itemData = readHistoryTransaction(transaction, address);

    redaction.setTitle(txid, itemData.txid);
    redaction.setValue(txid, formatTxid(itemData.txid));
    date.append(itemData.date);
    direction.append(itemData.direction);
    redaction.setValue(amount, formatBtc(itemData.amount), "fixed");
    fee.append(
      "fee ",
      redaction.createValue("span", formatBtc(itemData.fee), "fixed"),
    );
    item.append(date, direction, amount, fee, txid);
    list.append(item);
  }

  element.append(list);

  return element;
}

/**
 * @param {string} text
 */
export function createHistoryMessage(text) {
  const element = createElement("p", "wallets__history-message");

  element.append(text);

  return element;
}

/**
 * @param {Node} content
 * @param {number} columnCount
 */
function createHistoryCell(content, columnCount) {
  const cell = document.createElement("td");

  cell.colSpan = columnCount;
  cell.append(content);

  return cell;
}

/**
 * @param {Node} content
 * @param {number} columnCount
 */
export function createHistoryRow(content, columnCount) {
  const row = document.createElement("tr");

  row.append(createHistoryCell(content, columnCount));

  return row;
}

/**
 * @param {HTMLTableRowElement} row
 * @param {Node} content
 * @param {number} columnCount
 */
export function replaceHistoryRowContent(row, content, columnCount) {
  row.replaceChildren(createHistoryCell(content, columnCount));
}
