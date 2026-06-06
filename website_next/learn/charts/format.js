/**
 * @param {number} value
 * @param {number} digits
 */
function formatNumber(value, digits) {
  return value.toLocaleString("en-us", {
    maximumFractionDigits: digits,
    minimumFractionDigits: digits,
  });
}

/** @param {number} value */
export function formatValue(value) {
  const absolute = Math.abs(value);

  if (absolute < 10) return formatNumber(value, 3);
  if (absolute < 1_000) return formatNumber(value, 2);
  if (absolute < 10_000) return formatNumber(value, 1);
  if (absolute < 1_000_000) return formatNumber(value, 0);
  if (absolute >= 1e27) return "Inf.";

  const log = Math.floor(Math.log10(absolute) - 6);
  const suffixes = ["M", "B", "T", "P", "E", "Z", "Y"];
  const suffixIndex = Math.floor(log / 3);
  const digits = 3 - (log % 3);
  const scaled = value / (1_000_000 * 1_000 ** suffixIndex);

  return `${formatNumber(scaled, digits)}${suffixes[suffixIndex]}`;
}
