import { createElement } from "./dom.js";
import { formatNumber } from "./format.js";
import {
  arePrivateValuesHidden,
  createPrivateText,
} from "./privacy-view.js";

/**
 * @typedef {Object} WalletAddress
 * @property {number} index
 * @property {string} address
 * @property {string} [branchLabel]
 * @property {number} historyBucketSize
 */

/**
 * @param {string} text
 */
export function createGroupedAddress(text) {
  const element = createElement("code", "wallets__address");
  const groups = text.match(/.{1,4}/g) ?? [];

  for (let groupIndex = 0; groupIndex < groups.length; groupIndex += 1) {
    const group = createElement("span", "wallets__address-group");

    for (const character of groups[groupIndex]) {
      const span = createElement(
        "span",
        Number.isNaN(Number(character))
          ? "wallets__address-letter"
          : "wallets__address-number",
      );

      span.append(character);
      group.append(span);
    }

    element.append(group);
    if (groupIndex < groups.length - 1) {
      element.append(" ");
    }
  }

  return element;
}

/**
 * @param {string} address
 */
export function createPrivateAddress(address) {
  const hidden = createPrivateText(address);
  const element = arePrivateValuesHidden()
    ? createGroupedAddress(hidden)
    : createGroupedAddress(address);

  element.setAttribute("data-wallets-private-address", address);

  return element;
}

/**
 * @param {WalletAddress} row
 */
function createAddressBadge(row) {
  const badge = createElement("span", "wallets__address-badge");
  const label = row.branchLabel?.toLowerCase() ?? "address";

  badge.setAttribute("data-wallets-address-branch", label);
  badge.append(label, ` #${formatNumber(row.index)}`);

  return badge;
}

/**
 * @param {WalletAddress} row
 */
export function createAddressCellContent(row) {
  const element = createElement("div", "wallets__address-cell");
  const anonSet = createElement("span", "wallets__address-meta");

  anonSet.append(`anon set: ${formatNumber(row.historyBucketSize)}`);
  element.append(createAddressBadge(row), createPrivateAddress(row.address), anonSet);

  return element;
}
