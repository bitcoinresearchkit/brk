import { redaction } from "../redaction/index.js";

const SATS_PER_BTC = 100_000_000;
const FRACTION_DIGITS = 8;
const FIXED_PRIVATE_TEXT = "*****";
const amounts = /** @type {BtcAmountRecord[]} */ ([]);

/**
 * @typedef {Object} BtcAmountOptions
 * @property {boolean} [signed]
 *
 * @typedef {Object} BtcAmount
 * @property {number} sats
 * @property {boolean} signed
 *
 * @typedef {Object} BtcAmountRecord
 * @property {HTMLElement} element
 * @property {BtcAmount} amount
 */

/**
 * @typedef {Object} BtcPart
 * @property {string} text
 * @property {boolean} muted
 */

/**
 * @param {BtcPart[]} parts
 * @param {string} text
 * @param {boolean} muted
 */
function pushPart(parts, text, muted) {
  const last = parts[parts.length - 1];

  if (last && last.muted === muted) {
    last.text += text;
    return;
  }

  parts.push({ text, muted });
}

/**
 * @param {number} value
 */
function formatInteger(value) {
  return value.toString().replace(/\B(?=(\d{3})+(?!\d))/g, " ");
}

/**
 * @param {number} sats
 */
function splitBtc(sats) {
  const absolute = Math.abs(sats);

  return {
    whole: Math.floor(absolute / SATS_PER_BTC),
    fraction: String(absolute % SATS_PER_BTC).padStart(FRACTION_DIGITS, "0"),
  };
}

/**
 * @param {string} fraction
 * @param {(index: number) => boolean} isMuted
 * @param {(index: number) => boolean} isSpaceMuted
 */
function getFractionParts(fraction, isMuted, isSpaceMuted) {
  const parts = /** @type {BtcPart[]} */ ([]);

  for (let index = 0; index < fraction.length; index += 1) {
    pushPart(parts, fraction[index], isMuted(index));

    if (index === 1 || index === 4) {
      pushPart(parts, " ", isSpaceMuted(index));
    }
  }

  return parts;
}

/**
 * @param {number} sats
 * @param {BtcAmountOptions} [options]
 */
function getBtcParts(sats, options = {}) {
  const parts = /** @type {BtcPart[]} */ ([]);
  const { whole, fraction } = splitBtc(sats);
  const firstFractionDigit = fraction.search(/[1-9]/);
  const lastFractionDigit = Math.max(...[...fraction].map((digit, index) => {
    return digit === "0" ? -1 : index;
  }));

  if (options.signed && sats > 0) pushPart(parts, "+", false);
  if (sats < 0) pushPart(parts, "-", false);

  pushPart(parts, "₿", true);

  if (whole === 0) {
    const mutedUntil = firstFractionDigit === -1
      ? FRACTION_DIGITS
      : firstFractionDigit;

    pushPart(parts, "0.", true);
    for (const part of getFractionParts(
      fraction,
      (index) => index < mutedUntil,
      (index) => index < mutedUntil,
    )) {
      pushPart(parts, part.text, part.muted);
    }

    return parts;
  }

  pushPart(parts, formatInteger(whole), false);

  if (lastFractionDigit === -1) {
    pushPart(parts, ".", true);
    for (const part of getFractionParts(fraction, () => true, () => true)) {
      pushPart(parts, part.text, part.muted);
    }

    return parts;
  }

  pushPart(parts, ".", false);
  for (const part of getFractionParts(
    fraction,
    (index) => index > lastFractionDigit,
    (index) => index >= lastFractionDigit,
  )) {
    pushPart(parts, part.text, part.muted);
  }

  return parts;
}

/**
 * @param {HTMLElement} element
 * @param {BtcAmount} amount
 */
function renderBtcAmount(element, amount) {
  if (redaction.isHidden()) {
    element.textContent = FIXED_PRIVATE_TEXT;
    return;
  }

  element.replaceChildren(...getBtcParts(amount.sats, amount).map((part) => {
    const span = document.createElement("span");

    if (part.muted) {
      span.classList.add("muted");
    }
    span.append(part.text);

    return span;
  }));
}

/**
 * @template {keyof HTMLElementTagNameMap} Tag
 * @param {Tag} tag
 * @param {number} sats
 * @param {BtcAmountOptions} [options]
 */
export function createBtcAmount(tag, sats, options = {}) {
  const element = document.createElement(tag);
  const amount = {
    sats,
    signed: options.signed === true,
  };

  element.classList.add("amount");
  amounts.push({ element, amount });
  renderBtcAmount(element, amount);

  return element;
}

export function syncBtcAmounts() {
  for (let index = amounts.length - 1; index >= 0; index -= 1) {
    const { element, amount } = amounts[index];

    if (!element.isConnected) {
      amounts.splice(index, 1);
    } else {
      renderBtcAmount(element, amount);
    }
  }
}
