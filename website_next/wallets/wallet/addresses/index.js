import { createBtcAmount } from "../../amount/index.js";
import { createElement } from "../../dom.js";
import { formatNumber } from "../../format.js";
import { createAddressCellContent } from "../address/index.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 * @typedef {import("../../scan/index.js").WalletScan} WalletScan
 */

/**
 * @param {WalletAddress} address
 */
function createAddressRow(address) {
  const row = document.createElement("li");
  const meta = document.createElement("p");

  meta.append(
    createBtcAmount("span", address.balance),
    " · ",
    formatNumber(address.txCount),
    address.txCount === 1 ? " tx" : " txs",
  );
  row.append(createAddressCellContent(address), meta);

  return row;
}

/**
 * @param {HTMLElement} panel
 * @param {readonly WalletAddress[]} addresses
 */
function renderAddresses(panel, addresses) {
  const section = createElement("section", "addresses");
  const title = document.createElement("h2");
  const list = document.createElement("ol");

  title.append("Addresses");
  for (const address of addresses) {
    list.append(createAddressRow(address));
  }
  section.append(title, list);
  panel.replaceChildren(section);
}

/**
 * @param {WalletScan} scan
 */
export function createAddressesTab(scan) {
  const panel = document.createElement("section");
  let mounted = false;

  return {
    id: "addresses",
    label: "Addresses",
    panel,
    mount() {
      if (mounted) return;

      mounted = true;
      renderAddresses(panel, scan.addresses);
    },
  };
}
