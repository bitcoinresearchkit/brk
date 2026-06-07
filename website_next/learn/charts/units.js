export const units = /** @type {const} */ ({
  btc: { id: "btc", name: "Bitcoin" },
  usd: { id: "usd", name: "US Dollars" },
});

/** @typedef {keyof typeof units} ChartUnitKey */
/** @typedef {typeof units[ChartUnitKey]} ChartUnit */
