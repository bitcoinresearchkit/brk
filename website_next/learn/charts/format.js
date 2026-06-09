const suffixes = ["M", "B", "T", "P", "E", "Z", "Y"];
const numberFormats = [0, 1, 2, 3].map(
  (digits) =>
    new Intl.NumberFormat("en-US", {
      maximumFractionDigits: digits,
      minimumFractionDigits: digits,
    }),
);
const percentFormat = new Intl.NumberFormat("en-US", {
  maximumFractionDigits: 2,
  minimumFractionDigits: 2,
});

/**
 * @param {number} value
 * @param {number} digits
 */
function formatNumber(value, digits) {
  return numberFormats[digits].format(value);
}

/** @param {number} value */
export function formatNumberValue(value) {
  if (value === 0) return "0";

  const absolute = Math.abs(value);

  if (absolute < 10) return formatNumber(value, 3);
  if (absolute < 1_000) return formatNumber(value, 2);
  if (absolute < 10_000) return formatNumber(value, 1);
  if (absolute < 1_000_000) return formatNumber(value, 0);
  if (absolute >= 1e27) return "Inf.";

  const log = Math.floor(Math.log10(absolute) - 6);
  const suffixIndex = Math.floor(log / 3);
  const digits = 3 - (log % 3);
  const scaled = value / (1_000_000 * 1_000 ** suffixIndex);

  return `${formatNumber(scaled, digits)}${suffixes[suffixIndex]}`;
}

/** @param {number} value */
export function formatPercentValue(value) {
  return value === 0 ? "0%" : `${percentFormat.format(value)}%`;
}
