import { formatNumberValue, formatPercentValue } from "./format.js";

export const units = /** @type {const} */ ({
  addresses: { id: "addresses", name: "Addresses", format: formatNumberValue },
  blocks: { id: "blocks", name: "Blocks", format: formatNumberValue },
  btc: { id: "btc", name: "Bitcoin", format: formatNumberValue },
  percent: { id: "%", name: "Percent", format: formatPercentValue },
  utxos: { id: "utxos", name: "UTXOs", format: formatNumberValue },
  usd: { id: "usd", name: "US Dollars", format: formatNumberValue },
});
