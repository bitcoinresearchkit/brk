/** Unit definitions for chart series */

/**
 * Unit enum with id (for serialization) and name (for display)
 */
export const Unit = /** @type {const} */ ({
  // Value units
  sats: { id: "sats", name: "Satoshis" },
  btc: { id: "btc", name: "Bitcoin" },
  usd: { id: "usd", name: "US Dollars" },

  // Ratios & percentages
  percentage: { id: "percentage", name: "Percentage" },
  ratio: { id: "ratio", name: "Ratio" },
  index: { id: "index", name: "Index" },
  sd: { id: "sd", name: "Std Dev" },

  // Relative percentages
  pctSupply: { id: "pct-supply", name: "% of Supply" },
  pctOwn: { id: "pct-own", name: "% of Own Supply" },
  pctMcap: { id: "pct-mcap", name: "% of Market Cap" },
  pctRcap: { id: "pct-rcap", name: "% of Realized Cap" },
  pctOwnMcap: { id: "pct-own-mcap", name: "% of Own Market Cap" },
  pctOwnPnl: { id: "pct-own-pnl", name: "% of Own P&L" },

  // Time
  days: { id: "days", name: "Days" },
  years: { id: "years", name: "Years" },
  secs: { id: "secs", name: "Seconds" },

  // Counts
  count: { id: "count", name: "Count" },
  blocks: { id: "blocks", name: "Blocks" },

  // Size
  bytes: { id: "bytes", name: "Bytes" },
  vb: { id: "vb", name: "Virtual Bytes" },
  wu: { id: "wu", name: "Weight Units" },

  // Mining
  hashRate: { id: "hashrate", name: "Hash Rate" },
  difficulty: { id: "difficulty", name: "Difficulty" },
  epoch: { id: "epoch", name: "Epoch" },

  // Fees
  feeRate: { id: "feerate", name: "Sats/vByte" },

  // Rates
  perSec: { id: "per-sec", name: "Per Second" },

  // Cointime
  coinblocks: { id: "coinblocks", name: "Coinblocks" },
  coindays: { id: "coindays", name: "Coindays" },

  // Hash price/value
  usdPerThsPerDay: { id: "usd-ths-day", name: "USD/TH/s/Day" },
  usdPerPhsPerDay: { id: "usd-phs-day", name: "USD/PH/s/Day" },
  satsPerThsPerDay: { id: "sats-ths-day", name: "Sats/TH/s/Day" },
  satsPerPhsPerDay: { id: "sats-phs-day", name: "Sats/PH/s/Day" },
});

/** @typedef {keyof typeof Unit} UnitKey */
/** @typedef {typeof Unit[UnitKey]} UnitObject */
