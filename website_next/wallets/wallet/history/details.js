import { createBtcAmount } from "../../amount/index.js";
import { redaction } from "../../redaction/index.js";
import { createAddressCellContent } from "../address/index.js";

/**
 * @typedef {import("./transaction.js").WalletTransaction} WalletTransaction
 */

/**
 * @param {WalletTransaction} transaction
 */
export function createTransactionDetails(transaction) {
  const content = document.createElement("section");
  const txid = document.createElement("code");
  const meta = document.createElement("p");
  const list = document.createElement("ul");

  redaction.setTitle(txid, transaction.txid);
  redaction.setValue(txid, transaction.txid);
  meta.append(
    transaction.status,
    " · ",
    createBtcAmount("span", transaction.amount, { signed: true }),
    " · fee ",
    createBtcAmount("span", transaction.fee),
  );

  for (const address of transaction.addresses) {
    const item = document.createElement("li");

    item.append(createAddressCellContent(address.walletAddress));
    list.append(item);
  }

  content.append(txid, meta, list);

  return content;
}
