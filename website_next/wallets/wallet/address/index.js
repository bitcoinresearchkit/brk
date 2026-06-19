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
      if (Number.isNaN(Number(character))) {
        group.append(character);
      } else {
        const number = document.createElement("var");

        number.append(character);
        group.append(number);
      }
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
  const element = createGroupedAddress(address);

  redaction.setAddress(element, address, (text) => {
    element.replaceChildren(...createGroupedAddress(text).childNodes);
  });

  return element;
}

/**
 * @param {WalletAddress} row
 */
function createAddressBadge(row) {
  const badge = document.createElement("b");
  const label = row.branchLabel?.toLowerCase() ?? "address";

  badge.append(label, ` #${formatNumber(row.index)}`);

  return badge;
}

/**
 * @param {WalletAddress} row
 */
export function createAddressCellContent(row) {
  const element = createElement("div", "wallets__address-cell");
  const anonSet = document.createElement("small");

  anonSet.append(`anon set: ${formatNumber(row.historyBucketSize)}`);
  element.append(
    createAddressBadge(row),
    createPrivateAddress(row.address),
    anonSet,
  );

  return element;
}
