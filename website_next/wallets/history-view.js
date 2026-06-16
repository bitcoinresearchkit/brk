import { createElement } from "./dom.js";
import { formatBtc } from "./format.js";
import {
  createPrivateValue,
  setPrivateTitle,
  setPrivateValue,
} from "./privacy-view.js";

/**
 * @typedef {Object} AddressHistory
 * @property {unknown[]} transactions
 */

/**
 * @param {unknown} transaction
 */
function getTransactionId(transaction) {
  if (
    transaction &&
    typeof transaction === "object" &&
    "txid" in transaction &&
    typeof transaction.txid === "string"
  ) {
    return transaction.txid;
  }

  return "";
}

/**
 * @param {string} txid
 */
function formatTxid(txid) {
  return txid.length > 16 ? `${txid.slice(0, 8)}...${txid.slice(-8)}` : txid;
}

/**
 * @param {unknown} value
 */
function readSats(value) {
  return typeof value === "number" && Number.isFinite(value) ? value : 0;
}

/**
 * @param {unknown} output
 * @param {string} address
 */
function isAddressOutput(output, address) {
  return (
    output &&
    typeof output === "object" &&
    "scriptpubkeyAddress" in output &&
    output.scriptpubkeyAddress === address
  );
}

/**
 * @param {unknown} output
 */
function getOutputValue(output) {
  if (
    output &&
    typeof output === "object" &&
    "value" in output
  ) {
    return readSats(output.value);
  }

  return 0;
}

/**
 * @param {unknown} transaction
 * @param {string} address
 */
function getTransactionReceived(transaction, address) {
  if (
    !transaction ||
    typeof transaction !== "object" ||
    !("vout" in transaction) ||
    !Array.isArray(transaction.vout)
  ) {
    return 0;
  }

  return transaction.vout.reduce((total, output) => {
    return (
      total + (isAddressOutput(output, address) ? getOutputValue(output) : 0)
    );
  }, 0);
}

/**
 * @param {unknown} transaction
 * @param {string} address
 */
function getTransactionSent(transaction, address) {
  if (
    !transaction ||
    typeof transaction !== "object" ||
    !("vin" in transaction) ||
    !Array.isArray(transaction.vin)
  ) {
    return 0;
  }

  return transaction.vin.reduce((total, input) => {
    if (
      !input ||
      typeof input !== "object" ||
      !("prevout" in input)
    ) {
      return total;
    }

    const prevout = input.prevout;

    return (
      total + (isAddressOutput(prevout, address) ? getOutputValue(prevout) : 0)
    );
  }, 0);
}

/**
 * @param {unknown} transaction
 */
function getTransactionFee(transaction) {
  if (
    transaction &&
    typeof transaction === "object" &&
    "fee" in transaction
  ) {
    return readSats(transaction.fee);
  }

  return 0;
}

/**
 * @param {number} net
 */
function getTransactionDirection(net) {
  if (net > 0) return "received";
  if (net < 0) return "sent";

  return "moved";
}

/**
 * @param {unknown} transaction
 */
function getTransactionTime(transaction) {
  if (
    transaction &&
    typeof transaction === "object" &&
    "status" in transaction &&
    transaction.status &&
    typeof transaction.status === "object" &&
    "blockTime" in transaction.status &&
    typeof transaction.status.blockTime === "number"
  ) {
    return new Date(transaction.status.blockTime * 1_000).toLocaleDateString(
      "en-US",
    );
  }

  return "mempool";
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
    const received = getTransactionReceived(transaction, address);
    const sent = getTransactionSent(transaction, address);
    const net = received - sent;
    const id = getTransactionId(transaction);

    setPrivateTitle(txid, id);
    setPrivateValue(txid, formatTxid(id));
    date.append(getTransactionTime(transaction));
    direction.append(getTransactionDirection(net));
    setPrivateValue(amount, formatBtc(Math.abs(net)), "fixed");
    fee.append(
      "fee ",
      createPrivateValue("span", formatBtc(getTransactionFee(transaction)), "fixed"),
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
