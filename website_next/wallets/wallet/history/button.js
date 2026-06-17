import {
  createHistoryContent,
  createHistoryMessage,
  createHistoryRow,
  replaceHistoryRowContent,
} from "./index.js";

/**
 * @typedef {import("../../scan/index.js").WalletAddress} WalletAddress
 */

/**
 * @typedef {Object} AddressHistory
 * @property {unknown[]} transactions
 */

/**
 * @typedef {Object} AddressHistoryButtonOptions
 * @property {(address: WalletAddress) => Promise<AddressHistory>} fetchHistory
 * @property {(error: unknown) => string} getErrorMessage
 */

/**
 * @param {WalletAddress} address
 * @param {HTMLTableRowElement} parent
 * @param {AddressHistoryButtonOptions} options
 * @param {number} columnCount
 */
export function createAddressHistoryButton(
  address,
  parent,
  options,
  columnCount,
) {
  if (address.txCount === 0) {
    return "";
  }

  const button = document.createElement("button");
  /** @type {HTMLTableRowElement | undefined} */
  let historyRow;

  button.type = "button";
  button.className = "wallets__history-button";
  button.append("History");

  button.addEventListener("click", async () => {
    if (historyRow?.isConnected) {
      historyRow.remove();
      button.textContent = "History";
      return;
    }

    button.disabled = true;
    button.textContent = "Loading";
    historyRow = createHistoryRow(
      createHistoryMessage("Loading history"),
      columnCount,
    );
    parent.after(historyRow);

    try {
      const history = await options.fetchHistory(address);

      replaceHistoryRowContent(
        historyRow,
        createHistoryContent(history, address.address),
        columnCount,
      );
      button.textContent = "Hide";
    } catch (error) {
      replaceHistoryRowContent(
        historyRow,
        createHistoryMessage(options.getErrorMessage(error)),
        columnCount,
      );
      button.textContent = "History";
    } finally {
      button.disabled = false;
    }
  });

  return button;
}
