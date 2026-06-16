import { createElement } from "./dom.js";
import { formatBtc, formatUsd } from "./format.js";
import { createPrivateValue } from "./privacy-view.js";

/**
 * @typedef {Object} WalletAddress
 * @property {number} balance
 */

/**
 * @param {number} balance
 * @param {number} btcUsdPrice
 */
function createBalanceSummary(balance, btcUsdPrice) {
  const element = createElement("p", "wallets__metric wallets__balance");
  const btc = createPrivateValue("strong", formatBtc(balance), "fixed");
  const usd = createPrivateValue(
    "span",
    formatUsd((balance / 100_000_000) * btcUsdPrice),
    "fixed",
  );

  element.append(btc, usd);

  return element;
}

/**
 * @param {HTMLElement} summary
 * @param {WalletAddress[]} addresses
 * @param {number} btcUsdPrice
 */
export function renderWalletSummary(summary, addresses, btcUsdPrice) {
  const balance = addresses.reduce((total, row) => total + row.balance, 0);

  summary.replaceChildren(createBalanceSummary(balance, btcUsdPrice));
}
