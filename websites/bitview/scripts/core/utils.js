/**
 * @param {number} value
 * @param {number} [digits]
 * @param {Intl.NumberFormatOptions} [options]
 */
function numberToUSFormat(value, digits, options) {
  return value.toLocaleString("en-us", {
    ...options,
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

export const locale = {
  numberToUSFormat,
};

export const formatters = {
  dollars: new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }),
  percentage: new Intl.NumberFormat("en-US", {
    style: "percent",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }),
};

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
