import { createBtcAmount } from "../../amount/index.js";
import { formatUsd } from "../../format.js";
import { redaction } from "../../redaction/index.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @param {number} balance
 * @param {number} btcUsdPrice
 */
function createBalanceSummary(balance, btcUsdPrice) {
  const element = document.createElement("p");
  const btc = createBtcAmount("strong", balance);
  const usd = redaction.createValue(
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
