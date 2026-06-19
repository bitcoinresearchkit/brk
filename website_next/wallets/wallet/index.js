import { createElement } from "../dom.js";
import { renderReceiveButton } from "./receive/index.js";
import { renderWalletSummary } from "./summary/index.js";
import { transactionCache } from "./transactions/cache.js";
import { renderTransactions } from "./transactions/index.js";

/**
 * @typedef {import("../scan/index.js").WalletScan} WalletScan
 * @typedef {Parameters<typeof transactionCache.load>[0]} TransactionClient
 *
 * @typedef {Object} WalletPanel
 * @property {HTMLElement} actions
 * @property {HTMLElement} summary
 * @property {HTMLElement} status
 * @property {HTMLElement} results
 * @property {HTMLElement[]} nodes
 */

/**
 * @returns {WalletPanel}
 */
export function createWalletPanel() {
  const actions = createElement("section", "wallets__wallet-actions");
  const summary = createElement("section", "wallets__summary");
  const status = document.createElement("output");
  const results = createElement("section", "wallets__results");

  actions.setAttribute("aria-label", "Wallet actions");
  summary.setAttribute("aria-label", "Wallets summary");
  results.setAttribute("aria-label", "Wallets results");

  return {
    actions,
    summary,
    status,
    results,
    nodes: [actions, summary, status, results],
  };
}

/**
 * @param {WalletScan} scan
 * @param {WalletPanel} panel
 * @param {TransactionClient} client
 */
export function renderWalletPanel(scan, panel, client) {
  renderWalletSummary(panel.summary, scan.addresses, scan.btcUsdPrice);
  renderReceiveButton(panel.actions, scan.receiveAddress);
  panel.results.replaceChildren("Loading activity");
  void transactionCache.load(client, scan.addresses).then((transactions) => {
    if (panel.results.isConnected) {
      renderTransactions(panel.results, transactions);
    }
  }, () => {
    if (panel.results.isConnected) {
      panel.results.replaceChildren("Activity unavailable");
    }
  });
}
