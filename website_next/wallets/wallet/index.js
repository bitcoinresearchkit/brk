import { createElement } from "../dom.js";
import { createAddressesTab } from "./addresses/index.js";
import { createHistoryTab } from "./history/index.js";
import { createHoldingsTab } from "./holdings/index.js";
import { renderReceiveButton } from "./receive/index.js";
import { renderWalletSummary } from "./summary/index.js";
import { createWalletTabs } from "./tabs/index.js";

/**
 * @typedef {import("../scan/index.js").WalletScan} WalletScan
 * @typedef {Parameters<typeof createHistoryTab>[1] & Parameters<typeof createHoldingsTab>[1]} WalletClient
 *
 * @typedef {Object} WalletPanel
 * @property {HTMLElement} summary
 * @property {HTMLElement} status
 * @property {HTMLElement} results
 * @property {HTMLElement[]} nodes
 */

/**
 * @returns {WalletPanel}
 */
export function createWalletPanel() {
  const summary = createElement("section", "summary");
  const status = document.createElement("output");
  const results = createElement("section", "results");

  summary.setAttribute("aria-label", "Wallets summary");
  results.setAttribute("aria-label", "Wallets results");

  return {
    summary,
    status,
    results,
    nodes: [summary, status, results],
  };
}

/**
 * @param {WalletScan} scan
 * @param {WalletPanel} panel
 * @param {WalletClient} client
 */
export function renderWalletPanel(scan, panel, client) {
  renderWalletSummary(panel.summary, scan.addresses, scan.btcUsdPrice);
  renderReceiveButton(panel.summary, scan.receiveAddress);
  panel.results.replaceChildren(createWalletTabs([
    createHistoryTab(scan, client),
    createHoldingsTab(scan, client),
    createAddressesTab(scan),
  ]));
}
