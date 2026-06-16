import * as leanQr from "../modules/lean-qr/2.7.1/index.mjs";
import { createGroupedAddress } from "./address-view.js";
import { createElement } from "./dom.js";
import { formatNumber } from "./format.js";

/**
 * @typedef {Object} ReceiveAddress
 * @property {number} index
 * @property {string} address
 * @property {string} branchLabel
 */

/**
 * @param {ReceiveAddress} receiveAddress
 */
function createReceiveTitle(receiveAddress) {
  const title = document.createElement("h2");

  title.append(
    `${receiveAddress.branchLabel.toLowerCase()} #${formatNumber(receiveAddress.index)}`,
  );

  return title;
}

/**
 * @param {ReceiveAddress} receiveAddress
 */
function createReceiveQr(receiveAddress) {
  const image = document.createElement("img");
  const uri = `bitcoin:${receiveAddress.address}`;

  image.className = "wallets__receive-qr";
  image.alt = `QR code for ${receiveAddress.address}`;
  // @ts-ignore - lean-qr types do not resolve for file path imports.
  image.src = leanQr.generate(uri)?.toDataURL({ scale: 8 }) ?? "";

  return image;
}

/**
 * @param {ReceiveAddress} receiveAddress
 */
function createReceiveAddress(receiveAddress) {
  const element = createElement("div", "wallets__receive-address");

  element.append(createGroupedAddress(receiveAddress.address));

  return element;
}

/**
 * @param {ReceiveAddress} receiveAddress
 * @param {HTMLButtonElement} copy
 */
async function copyReceiveAddress(receiveAddress, copy) {
  await navigator.clipboard.writeText(receiveAddress.address);
  copy.textContent = "Copied";
}

/**
 * @param {ReceiveAddress} receiveAddress
 */
function openReceiveDialog(receiveAddress) {
  const main = document.querySelector("main.wallets") ?? document.body;
  const dialog = createElement("dialog", "wallets__dialog wallets__receive-dialog");
  const content = createElement("div", "wallets__receive-card");
  const actions = createElement("div", "wallets__receive-actions");
  const copy = document.createElement("button");
  const close = document.createElement("button");

  copy.type = "button";
  copy.append("Copy");
  close.type = "button";
  close.append("Close");
  actions.append(copy, close);
  content.append(
    createReceiveTitle(receiveAddress),
    createReceiveQr(receiveAddress),
    createReceiveAddress(receiveAddress),
    actions,
  );
  dialog.append(content);
  main.append(dialog);

  copy.addEventListener("click", () => {
    void copyReceiveAddress(receiveAddress, copy).catch(() => {
      copy.textContent = "Copy failed";
    });
  });
  close.addEventListener("click", () => {
    dialog.close();
  });
  dialog.addEventListener("close", () => {
    dialog.remove();
  });
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) {
      dialog.close();
    }
  });
  dialog.showModal();
}

/**
 * @param {HTMLElement} element
 * @param {ReceiveAddress | undefined} receiveAddress
 */
export function renderReceiveButton(element, receiveAddress) {
  const button = document.createElement("button");

  button.type = "button";
  button.className = "wallets__receive-button";
  button.disabled = !receiveAddress;
  button.append("Receive");
  button.addEventListener("click", () => {
    if (receiveAddress) {
      openReceiveDialog(receiveAddress);
    }
  });
  element.append(button);
}
