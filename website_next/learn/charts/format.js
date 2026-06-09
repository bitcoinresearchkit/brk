const suffixes = ["M", "B", "T", "P", "E", "Z", "Y"];
const compactBase = 1_000_000;
const compactStep = 1_000;
const compactMax = 1e27;
const maxLength = 7;
const tinyDigits = [3, 2, 1, 0];
const smallDigits = [2, 1, 0];
const mediumDigits = [1, 0];
const integerDigits = [0];
const numberFormats = createNumberFormats(true);
const ungroupedNumberFormats = createNumberFormats(false);

/** @param {boolean} useGrouping */
function createNumberFormats(useGrouping) {
  return [0, 1, 2, 3].map(
    (digits) =>
      new Intl.NumberFormat("en-US", {
        maximumFractionDigits: digits,
        minimumFractionDigits: digits,
        useGrouping,
      }),
  );
}

/**
 * @param {number} value
 * @param {number} digits
 * @param {boolean} [useGrouping]
 */
function formatNumber(value, digits, useGrouping = true) {
  return (useGrouping ? numberFormats : ungroupedNumberFormats)[digits].format(
    value,
  );
}

/** @param {string} value */
function parseFormattedNumber(value) {
  return Number(value.replaceAll(",", ""));
}

/** @param {number} index */
function getCompactFactor(index) {
  return compactBase * compactStep ** index;
}

/** @param {number} absolute */
function getCompactIndex(absolute) {
  return Math.max(
    0,
    Math.min(
      suffixes.length - 1,
      Math.floor(Math.log10(absolute / compactBase) / 3),
    ),
  );
}

/** @param {number} absolute */
function getPlainDigits(absolute) {
  if (absolute < 10) return tinyDigits;
  if (absolute < 1_000) return smallDigits;
  if (absolute < 10_000) return mediumDigits;
  return integerDigits;
}

/**
 * @param {number} value
 * @param {number} index
 * @param {number} length
 * @param {boolean} useGrouping
 */
function formatCompact(value, index, length, useGrouping) {
  for (let suffixIndex = index; suffixIndex < suffixes.length; suffixIndex += 1) {
    const suffix = suffixes[suffixIndex];
    const scaled = value / getCompactFactor(suffixIndex);

    for (let digits = 3; digits >= 0; digits -= 1) {
      const formatted = formatNumber(scaled, digits, useGrouping);

      if (
        Math.abs(parseFormattedNumber(formatted)) >= compactStep &&
        suffixIndex < suffixes.length - 1
      ) break;

      if (`${formatted}${suffix}`.length <= length) {
        return `${formatted}${suffix}`;
      }
    }
  }

  return "Inf.";
}

/**
 * @param {number} value
 * @param {number} length
 * @param {boolean} useGrouping
 */
function formatPlain(value, length, useGrouping) {
  const absolute = Math.abs(value);

  for (const digit of getPlainDigits(absolute)) {
    const formatted = formatNumber(value, digit, useGrouping);

    if (formatted.length <= length) return formatted;
  }

  return formatCompact(value, 0, length, useGrouping);
}

/**
 * @param {number} value
 * @param {number} length
 * @param {boolean} [useGrouping]
 */
function formatValue(value, length, useGrouping = true) {
  if (value === 0) return "0";

  const absolute = Math.abs(value);

  if (absolute >= compactMax) return "Inf.";
  if (absolute < compactBase) return formatPlain(value, length, useGrouping);

  return formatCompact(value, getCompactIndex(absolute), length, useGrouping);
}

/** @param {number} value */
export function formatNumberValue(value) {
  return formatValue(value, maxLength);
}

/** @param {number} value */
export function formatPercentValue(value) {
  return `${formatValue(value, maxLength - 1, false)}%`;
}
