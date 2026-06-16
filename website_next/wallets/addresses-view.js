import { createAddressTable } from "./table-view.js";

/**
 * @typedef {Parameters<typeof createAddressTable>[0][number] & {
 *   branchLabel?: string,
 * }} WalletAddress
 */

/**
 * @typedef {Parameters<typeof createAddressTable>[1]} AddressTableOptions
 */

/**
 * @param {HTMLElement} results
 * @param {WalletAddress[]} addresses
 * @param {AddressTableOptions} tableOptions
 */
export function renderWalletAddresses(results, addresses, tableOptions) {
  results.replaceChildren(createAddressTable(addresses, tableOptions));
}
