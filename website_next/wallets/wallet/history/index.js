import { createElement } from "../../dom.js";
import { historyCache } from "./cache.js";
import { createTransactionSection } from "./section.js";

/**
 * @typedef {import("./transaction.js").WalletTransaction} WalletTransaction
 * @typedef {import("../../scan/index.js").WalletScan} WalletScan
 * @typedef {Parameters<typeof historyCache.load>[0]} HistoryClient
 */

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
function renderHistory(element, transactions) {
  const activity = createElement("section", "activity");
  const title = document.createElement("h2");
  const groups = groupTransactionsByDate(transactions);

  title.append("History");
  activity.append(title);

  if (transactions.length === 0) {
    const empty = document.createElement("p");

    empty.append("No activity yet");
    activity.append(empty);
    element.replaceChildren(activity);
    return;
  }

  for (const [date, group] of groups) {
    activity.append(createTransactionSection(date, group));
  }

  element.replaceChildren(activity);
}

/**
 * @param {WalletScan} scan
 * @param {HistoryClient} client
 */
export function createHistoryTab(scan, client) {
  const panel = document.createElement("section");
  let loaded = false;

  return {
    id: "history",
    label: "History",
    panel,
    mount() {
      if (loaded) return;

      loaded = true;
      panel.replaceChildren("Loading history");
      void historyCache.load(client, scan.addresses).then(
        (transactions) => {
          renderHistory(panel, transactions);
        },
        () => {
          panel.replaceChildren("History unavailable");
        },
      );
    },
  };
}
