import { createElement } from "../dom.js";
import { renderReceiveButton } from "./receive/index.js";
import { renderWalletSummary } from "./summary/index.js";
import { createAddressTable } from "./table/index.js";

/**
 * @typedef {import("../scan/index.js").WalletScan} WalletScan
 * @typedef {Parameters<typeof createAddressTable>[1]} WalletRenderOptions
 *
 * @typedef {Object} WalletPanel
 * @property {HTMLElement} settings
 * @property {HTMLElement} summary
 * @property {HTMLElement} status
 * @property {HTMLElement} results
 * @property {HTMLElement[]} nodes
 */

/**
 * @returns {WalletPanel}
 */
export function createWalletPanel() {
  const settings = createElement("section", "wallets__settings");
  const summary = createElement("section", "wallets__summary");
  const status = createElement("p", "wallets__status");
  const results = createElement("section", "wallets__results");

  settings.setAttribute("aria-label", "Wallet settings");
  status.setAttribute("role", "status");
  summary.setAttribute("aria-label", "Wallets summary");
  results.setAttribute("aria-label", "Wallets results");

  return {
    settings,
    summary,
    status,
    results,
    nodes: [settings, summary, status, results],
  };
}

/**
 * @param {WalletScan} scan
 * @param {WalletPanel} panel
 * @param {WalletRenderOptions} options
 */
export function renderWalletPanel(scan, panel, options) {
  renderWalletSummary(panel.summary, scan.addresses, scan.btcUsdPrice);
  renderReceiveButton(panel.settings, scan.receiveAddress);
  panel.results.replaceChildren(createAddressTable(scan.addresses, options));
}
