import { createElement } from "../../dom.js";
import { formatBtc } from "../../format.js";
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
 * @param {number} sats
 */
function formatSignedBtc(sats) {
  if (sats > 0) return `+${formatBtc(sats)}`;
  if (sats < 0) return `-${formatBtc(Math.abs(sats))}`;

  return formatBtc(sats);
}

/**
 * @param {WalletTransaction} transaction
 */
function getTransactionDetail(transaction) {
  if (transaction.type === "consolidation") {
    return `${transaction.addresses.length} wallet addresses · fee only`;
  }

  if (transaction.type === "send") {
    return `to external wallet · fee ${formatBtc(transaction.fee)}`;
  }

  return transaction.status;
}

/**
 * @param {WalletTransaction} transaction
 */
function createTransactionDetails(transaction) {
  const dialog = createElement("dialog", "wallets__dialog wallets__tx-dialog");
  const content = createElement("div", "wallets__tx-details");
  const title = document.createElement("h2");
  const txid = document.createElement("code");
  const meta = document.createElement("p");
  const list = createElement("div", "wallets__tx-addresses");
  const close = document.createElement("button");

  title.append(typeLabels[transaction.type]);
  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, transaction.txid);
  meta.append(
    transaction.status,
    " · ",
    redaction.createValue("span", formatSignedBtc(transaction.amount), "fixed"),
    " · fee ",
    redaction.createValue("span", formatBtc(transaction.fee), "fixed"),
  );
  for (const address of transaction.addresses) {
    list.append(createAddressCellContent(address.walletAddress));
  }
  close.type = "button";
  close.append("Close");
  content.append(title, txid, meta, list, close);
  dialog.append(content);
  close.addEventListener("click", () => {
    dialog.close();
  });
  dialog.addEventListener("close", () => {
    dialog.remove();
  });
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) {
      dialog.close();
    }
  });

  return dialog;
}

/**
 * @param {WalletTransaction} transaction
 */
function createTransactionRow(transaction) {
  const row = createElement("li", "wallets__tx");
  const main = createElement("div", "wallets__tx-main");
  const label = document.createElement("strong");
  const amount = redaction.createValue(
    "span",
    formatSignedBtc(transaction.amount),
    "fixed",
  );
  const detail = createElement("p", "wallets__tx-detail");
  const txid = document.createElement("code");
  const more = document.createElement("button");

  label.append(typeLabels[transaction.type]);
  amount.dataset.walletsTxAmount =
    transaction.amount >= 0 ? "positive" : "negative";
  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, formatTxid(transaction.txid));
  more.type = "button";
  more.append("View more");
  detail.append(getTransactionDetail(transaction), " · ", txid);
  main.append(label, amount);
  row.append(main, detail, more);
  more.addEventListener("click", () => {
    const dialog = createTransactionDetails(transaction);
    const mainElement = document.querySelector("main.wallets") ?? document.body;

    mainElement.append(dialog);
    dialog.showModal();
  });

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
    const section = createElement("section", "wallets__tx-group");
    const heading = document.createElement("h3");
    const list = createElement("ol", "wallets__tx-list");

    heading.append(date);
    for (const transaction of group) {
      list.append(createTransactionRow(transaction));
    }
    section.append(heading, list);
    activity.append(section);
  }

  element.replaceChildren(activity);
}
