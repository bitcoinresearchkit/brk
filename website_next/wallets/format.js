/**
 * @param {number} value
 */
export function formatNumber(value) {
  return new Intl.NumberFormat("en-US").format(value);
}

/**
 * @param {number} sats
 */
export function formatBtc(sats) {
  return `${(sats / 100_000_000).toLocaleString("en-US", {
    maximumFractionDigits: 8,
  })} BTC`;
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
