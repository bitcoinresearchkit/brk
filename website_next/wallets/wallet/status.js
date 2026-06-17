import { setStatus } from "../dom.js";
import { formatNumber } from "../format.js";
import { GAP_LIMIT } from "../scan/branch.js";

/**
 * @typedef {import("../scan/index.js").WalletScanProgress} WalletScanProgress
 */

function createScanPendingMessage() {
  return `Scanning until ${GAP_LIMIT} unused addresses`;
}

/**
 * @param {HTMLElement} status
 */
function setPending(status) {
  setStatus(status, createScanPendingMessage());
}

/**
 * @param {HTMLElement} status
 * @param {WalletScanProgress} progress
 */
function setProgress(status, progress) {
  setStatus(
    status,
    `${progress.branchLabel}: scanned ${formatNumber(progress.scannedCount)} addresses, ${progress.unusedInRow}/${GAP_LIMIT} unused`,
  );
}

export const scanStatus = /** @type {const} */ ({
  setPending,
  setProgress,
});
