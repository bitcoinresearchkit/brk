import { createElement } from "../../dom.js";
import { createBtcAmount } from "../../amount/index.js";
import { redaction } from "../../redaction/index.js";
import { createAddressCellContent } from "../address/index.js";

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
function createTransactionDetails(transaction) {
  const content = document.createElement("div");
  const txid = document.createElement("code");
  const meta = document.createElement("p");
  const list = document.createElement("div");

  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, transaction.txid);
  meta.append(
    transaction.status,
    " · ",
    createBtcAmount("span", transaction.amount, { signed: true }),
    " · fee ",
    createBtcAmount("span", transaction.fee),
  );
  for (const address of transaction.addresses) {
    list.append(createAddressCellContent(address.walletAddress));
  }
  content.append(txid, meta, list);

  return content;
}

/**
 * @param {WalletTransaction} transaction
 */
function createTransactionRow(transaction) {
  const row = document.createElement("li");
  const main = document.createElement("div");
  const label = document.createElement("strong");
  const amount = createBtcAmount(
    "span",
    transaction.amount,
    { signed: true },
  );
  const detail = document.createElement("p");
  const txid = document.createElement("code");
  const details = document.createElement("details");
  const summary = document.createElement("summary");

  label.append(typeLabels[transaction.type]);
  amount.dataset.walletsTxAmount =
    transaction.amount >= 0 ? "positive" : "negative";
  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, formatTxid(transaction.txid));
  summary.append("Details");
  appendTransactionDetail(detail, transaction);
  detail.append(" · ", txid);
  details.append(summary, createTransactionDetails(transaction));
  main.append(label, amount);
  row.append(main, detail, details);

  return row;
}

/**
 * @param {readonly WalletTransaction[]} transactions
 */
function groupTransactionsByDate(transactions) {
  const groups = /** @type {Map<string, WalletTransaction[]>} */ (new Map());

  for (const transaction of transactions) {
    const group = groups.get(transaction.date) ?? [];

    group.push(transaction);
    groups.set(transaction.date, group);
  }

  return groups;
}

/**
 * @param {HTMLElement} element
 * @param {readonly WalletTransaction[]} transactions
 */
export function renderTransactions(element, transactions) {
  const activity = createElement("section", "wallets__activity");
  const title = document.createElement("h2");
  const groups = groupTransactionsByDate(transactions);

  title.append("Activity");
  activity.append(title);

  if (transactions.length === 0) {
    const empty = document.createElement("p");

    empty.append("No activity yet");
    activity.append(empty);
    element.replaceChildren(activity);
    return;
  }

  for (const [date, group] of groups) {
    const section = document.createElement("section");
    const heading = document.createElement("h3");
    const list = document.createElement("ol");

    heading.append(redaction.createValue("span", date, "fixed"));
    for (const transaction of group) {
      list.append(createTransactionRow(transaction));
    }
    section.append(heading, list);
    activity.append(section);
  }

  element.replaceChildren(activity);
}
