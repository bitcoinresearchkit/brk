/**
 * @param {number} value
 */
export function formatNumber(value) {
  return new Intl.NumberFormat("en-US").format(value);
}

/**
 * @param {number} dollars
 */
export function formatUsd(dollars) {
  return new Intl.NumberFormat("en-US", {
    currency: "USD",
    maximumFractionDigits: 0,
    style: "currency",
  }).format(dollars);
}
