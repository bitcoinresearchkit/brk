/**
 * @param {number} value
 * @param {number} [digits]
 * @param {Intl.NumberFormatOptions} [options]
 */
export function numberToUSNumber(value, digits, options) {
  return value.toLocaleString("en-us", {
    ...options,
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

export const numberToDollars = new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: "USD",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

export const numberToPercentage = new Intl.NumberFormat("en-US", {
  style: "percent",
  minimumFractionDigits: 2,
  maximumFractionDigits: 2,
});

/**
 * @param {string} s
 */
export function stringToId(s) {
  return (
    s
      // .replace(/\W/g, " ")
      .trim()
      .replace(/ +/g, "-")
      .toLowerCase()
  );
}
