import { createBtcAmount } from "../../amount/index.js";
import { createElement } from "../../dom.js";
import { redaction } from "../../redaction/index.js";
import { createAddressCellContent } from "../address/index.js";
import { utxoCache } from "./cache.js";
import { readWalletUtxo } from "./utxo.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 * @typedef {import("../../scan/index.js").WalletScan} WalletScan
 * @typedef {import("./utxo.js").WalletUtxo} WalletUtxo
 * @typedef {Parameters<typeof utxoCache.load>[0]} UtxoClient
 */

/**
 * @param {string} txid
 */
function formatTxid(txid) {
  return txid.length > 16 ? `${txid.slice(0, 8)}...${txid.slice(-8)}` : txid;
}

/**
 * @param {WalletUtxo} utxo
 */
function createUtxoRow(utxo) {
  const row = document.createElement("li");
  const main = document.createElement("header");
  const label = document.createElement("strong");
  const amount = createBtcAmount("span", utxo.value);
  const detail = document.createElement("p");
  const outpoint = document.createElement("code");

  redaction.setTitle(outpoint, `${utxo.txid}:${utxo.vout}`);
  redaction.setValue(outpoint, `${formatTxid(utxo.txid)}:${utxo.vout}`);
  label.append(utxo.confirmed ? "Confirmed" : "Mempool");
  detail.append(outpoint);
  row.append(main, detail, createAddressCellContent(utxo.address));
  main.append(label, amount);

  return row;
}

/**
 * @param {WalletUtxo} a
 * @param {WalletUtxo} b
 */
function compareUtxos(a, b) {
  return b.value - a.value || a.txid.localeCompare(b.txid) || a.vout - b.vout;
}

/**
 * @param {UtxoClient} client
 * @param {readonly WalletAddress[]} addresses
 */
async function loadWalletUtxos(client, addresses) {
  const utxos = /** @type {WalletUtxo[]} */ ([]);

  for (const address of addresses) {
    const loaded = await utxoCache.load(client, address);

    for (const utxo of loaded.utxos) {
      const walletUtxo = readWalletUtxo(address, utxo);

      if (walletUtxo) {
        utxos.push(walletUtxo);
      }
    }
  }

  return utxos.sort(compareUtxos);
}

/**
 * @param {HTMLElement} panel
 * @param {readonly WalletUtxo[]} utxos
 */
function renderHoldings(panel, utxos) {
  const section = createElement("section", "holdings");
  const title = document.createElement("h2");
  const list = document.createElement("ol");

  title.append("Holdings");
  section.append(title);

  if (utxos.length === 0) {
    const empty = document.createElement("p");

    empty.append("No UTXOs");
    section.append(empty);
    panel.replaceChildren(section);
    return;
  }

  for (const utxo of utxos) {
    list.append(createUtxoRow(utxo));
  }

  section.append(list);
  panel.replaceChildren(section);
}

/**
 * @param {WalletScan} scan
 * @param {UtxoClient} client
 */
export function createHoldingsTab(scan, client) {
  const panel = document.createElement("section");
  let loaded = false;

  return {
    id: "holdings",
    label: "Holdings",
    panel,
    mount() {
      if (loaded) return;

      loaded = true;
      panel.replaceChildren("Loading holdings");
      void loadWalletUtxos(client, scan.addresses).then(
        (utxos) => {
          renderHoldings(panel, utxos);
        },
        () => {
          panel.replaceChildren("Holdings unavailable");
        },
      );
    },
  };
}
