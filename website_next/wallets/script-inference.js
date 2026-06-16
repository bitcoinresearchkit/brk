import { addressScripts } from "./address-scripts.js";
import { fetchWalletAddresses } from "./privacy/address-lookup.js";
import {
  generateAddressesFromXpub,
  isOutputDescriptor,
} from "./xpub/index.js";
import { parseOutputDescriptor } from "./xpub/descriptor.js";

const RECEIVE_PATH = /** @type {const} */ ([0]);

/**
 * @typedef {import("./xpub/address.js").AddressScript} AddressScript
 * @typedef {import("./privacy/xpub-scan.js").AddressClient} AddressClient
 * @typedef {import("./privacy/address-lookup.js").WalletAddress} WalletAddress
 */

/**
 * @param {WalletAddress} address
 */
function hasHistory(address) {
  return address.received > 0 || address.sent > 0 || address.txCount > 0;
}

/**
 * @param {AddressClient} client
 * @param {string} xpub
 * @returns {Promise<AddressScript>}
 */
export async function inferAddressScript(client, xpub) {
  if (isOutputDescriptor(xpub)) {
    return parseOutputDescriptor(xpub).script;
  }

  for (const { id } of addressScripts) {
    const generated = await generateAddressesFromXpub(xpub, {
      start: 0,
      count: 1,
      script: id,
      path: RECEIVE_PATH,
    });
    const [address] = await fetchWalletAddresses(client, generated);

    if (address && hasHistory(address)) {
      return id;
    }
  }

  return addressScripts[0].id;
}
