import { isOutputDescriptor } from "./xpub/index.js";

const EXTENDED_PUBLIC_KEY_PATTERN =
  /\b(?:xpub|ypub|zpub|tpub|upub|vpub)[1-9A-HJ-NP-Za-km-z]{20,}\b/;

/**
 * @param {string} text
 */
export function readWalletSourceText(text) {
  const value = text.trim();

  if (isOutputDescriptor(value)) {
    return value;
  }

  const match = value.match(EXTENDED_PUBLIC_KEY_PATTERN);

  if (match) {
    return match[0];
  }

  throw new Error("Expected an xpub or descriptor");
}
