import * as leanQr from "../../../modules/lean-qr/2.7.1/index.mjs";
import { createGroupedAddress } from "../address/index.js";
import { createElement } from "../../dom.js";
import { formatNumber } from "../../format.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} ReceiveAddress
 */

/**
 * @typedef {Object} QrCode
 * @property {(options?: { scale?: number }) => string} toDataURL
 */

const generateQr =
  /** @type {(value: string) => QrCode | undefined} */ (
    /** @type {unknown} */ (leanQr.generate)
  );

/**
 * @param {string} value
 */
function createQrDataUrl(value) {
  const qr = generateQr(value);

  return qr?.toDataURL({ scale: 8 }) ?? "";
}

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

  image.alt = `QR code for ${receiveAddress.address}`;
  image.src = createQrDataUrl(uri);

  return image;
}

/**
 * @param {ReceiveAddress} receiveAddress
 */
function createReceiveAddress(receiveAddress) {
  const element = document.createElement("div");

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
  const dialog = createElement(
    "dialog",
    "wallets__dialog wallets__receive-dialog",
  );
  const content = document.createElement("div");
  const actions = document.createElement("div");
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
  button.disabled = !receiveAddress;
  button.append("Receive");
  button.addEventListener("click", () => {
    if (receiveAddress) {
      openReceiveDialog(receiveAddress);
    }
  });
  element.append(button);
}
