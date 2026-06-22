import { redaction } from "../../redaction/index.js";
import { createTransactionRow } from "./row.js";

/**
 * @typedef {import("./transaction.js").WalletTransaction} WalletTransaction
 */

/**
 * @param {string} date
 * @param {readonly WalletTransaction[]} transactions
 */
export function createTransactionSection(date, transactions) {
  const section = document.createElement("section");
  const heading = document.createElement("h3");
  const list = document.createElement("ol");

  heading.append(redaction.createValue("span", date, "fixed"));
  for (const transaction of transactions) {
    list.append(createTransactionRow(transaction));
  }
  section.append(heading, list);

  return section;
}
