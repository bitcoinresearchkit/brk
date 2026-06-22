import { createBtcAmount } from "../../amount/index.js";
import { redaction } from "../../redaction/index.js";
import { createTransactionDetails } from "./details.js";

/**
 * @typedef {import("./transaction.js").WalletTransaction} WalletTransaction
 */

const typeLabels = /** @type {const} */ ({
  receive: "Received",
  send: "Sent",
  consolidation: "Consolidated",
});

/**
 * @param {string} txid
 */
function formatTxid(txid) {
  return txid.length > 16 ? `${txid.slice(0, 8)}...${txid.slice(-8)}` : txid;
}

/**
 * @param {HTMLElement} element
 * @param {WalletTransaction} transaction
 */
function appendTransactionDetail(element, transaction) {
  if (transaction.type === "consolidation") {
    element.append(
      `${transaction.addresses.length} wallet addresses · fee only`,
    );
    return;
  }

  if (transaction.type === "send") {
    element.append(
      "to external wallet · fee ",
      createBtcAmount("span", transaction.fee),
    );
    return;
  }

  element.append(transaction.status);
}

/**
 * @param {WalletTransaction} transaction
 */
export function createTransactionRow(transaction) {
  const row = document.createElement("li");
  const details = document.createElement("details");
  const summary = document.createElement("summary");
  const main = document.createElement("header");
  const label = document.createElement("strong");
  const amount = createBtcAmount(
    "span",
    transaction.amount,
    { signed: true },
  );
  const detail = document.createElement("p");
  const txid = document.createElement("code");

  label.append(typeLabels[transaction.type]);
  if (transaction.amount > 0) {
    amount.classList.add("positive");
  }
  if (transaction.amount < 0) {
    amount.classList.add("negative");
  }
  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, formatTxid(transaction.txid));
  appendTransactionDetail(detail, transaction);
  detail.append(" · ", txid);
  main.append(label, amount);
  summary.append(main, detail);
  details.append(summary, createTransactionDetails(transaction));
  row.append(details);

  return row;
}
