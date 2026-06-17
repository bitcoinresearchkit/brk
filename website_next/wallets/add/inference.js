import { fetchWalletAddresses } from "../lookup/index.js";
import {
  generateAddressesFromKey,
  isOutputDescriptor,
} from "../derive/index.js";
import { parseOutputDescriptor } from "../derive/descriptor.js";
import { addressScripts } from "../derive/script.js";

const RECEIVE_PATH = /** @type {const} */ ([0]);

/**
 * @typedef {import("../derive/address.js").AddressScript} AddressScript
 * @typedef {import("../scan/branch.js").AddressClient} AddressClient
 * @typedef {import("../lookup/index.js").WalletAddress} WalletAddress
 */

/**
 * @param {WalletAddress} address
 */
function hasHistory(address) {
  return address.received > 0 || address.sent > 0 || address.txCount > 0;
}

/**
 * @param {AddressClient} client
 * @param {string} source
 * @returns {Promise<AddressScript>}
 */
export async function inferAddressScript(client, source) {
  if (isOutputDescriptor(source)) {
    return parseOutputDescriptor(source).script;
  }

  for (const { id } of addressScripts) {
    const generated = await generateAddressesFromKey(source, {
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
