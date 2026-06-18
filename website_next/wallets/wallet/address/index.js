import { createElement } from "../../dom.js";
import { formatNumber } from "../../format.js";
import { redaction } from "../../redaction/index.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @param {string} text
 */
export function createGroupedAddress(text) {
  const element = createElement("code", "wallets__address");
  const groups = text.match(/.{1,4}/g) ?? [];

  for (let groupIndex = 0; groupIndex < groups.length; groupIndex += 1) {
    const group = document.createElement("span");

    for (const character of groups[groupIndex]) {
      const span = document.createElement("span");

      span.setAttribute(
        "data-wallets-address-character",
        Number.isNaN(Number(character)) ? "letter" : "number",
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
function createPrivateAddress(address) {
  const hidden = redaction.createText(address);
  const element = redaction.isHidden()
    ? createGroupedAddress(hidden)
    : createGroupedAddress(address);

  element.setAttribute("data-wallets-private-address", address);

  return element;
}

/**
 * @param {WalletAddress} row
 */
function createAddressBadge(row) {
  const badge = document.createElement("span");
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
  const anonSet = document.createElement("span");

  anonSet.append(`anon set: ${formatNumber(row.historyBucketSize)}`);
  element.append(
    createAddressBadge(row),
    createPrivateAddress(row.address),
    anonSet,
  );

  return element;
}
