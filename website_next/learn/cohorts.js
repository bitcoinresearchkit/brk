import { createSeries } from "./charts/config.js";
import { colors } from "../utils/colors.js";

/** @typedef {import("./charts/index.js").ChartSeries["color"]} ChartColor */
/** @typedef {import("./charts/index.js").ChartSeries["metric"]} Metric */

/** @type {ChartColor[]} */
const palette = [
  colors.red,
  colors.orange,
  colors.amber,
  colors.yellow,
  colors.avocado,
  colors.lime,
  colors.green,
  colors.emerald,
  colors.teal,
  colors.cyan,
  colors.sky,
  colors.blue,
  colors.indigo,
  colors.violet,
  colors.purple,
  colors.fuchsia,
  colors.pink,
  colors.rose,
];

/** @param {number} index */
function colorAt(index) {
  return palette[index % palette.length];
}

/**
 * @param {readonly { label: string, color?: ChartColor, metric: Metric }[]} items
 */
function createCohortSeries(items) {
  return createSeries(
    items.map(({ label, color, metric }, index) => ({
      label,
      color: color ?? colorAt(index),
      metric,
    })),
  );
}

/**
 * @template {string} Key
 * @param {readonly (readonly [string, Key])[]} items
 * @param {(key: Key) => Metric} createMetric
 */
function createCohortSeriesFromKeys(items, createMetric) {
  return createCohortSeries(
    items.map(([label, key]) => ({
      label,
      metric: createMetric(key),
    })),
  );
}

const ageRanges = /** @type {const} */ ([
  ["0-1h", "under1h"],
  ["1h to 1d", "_1hTo1d"],
  ["1d to 1w", "_1dTo1w"],
  ["1w to 1m", "_1wTo1m"],
  ["1m to 2m", "_1mTo2m"],
  ["2m to 3m", "_2mTo3m"],
  ["3m to 4m", "_3mTo4m"],
  ["4m to 5m", "_4mTo5m"],
  ["5m to 6m", "_5mTo6m"],
  ["6m to 1y", "_6mTo1y"],
  ["1y to 2y", "_1yTo2y"],
  ["2y to 3y", "_2yTo3y"],
  ["3y to 4y", "_3yTo4y"],
  ["4y to 5y", "_4yTo5y"],
  ["5y to 6y", "_5yTo6y"],
  ["6y to 7y", "_6yTo7y"],
  ["7y to 8y", "_7yTo8y"],
  ["8y to 10y", "_8yTo10y"],
  ["10y to 12y", "_10yTo12y"],
  ["12y to 15y", "_12yTo15y"],
  ["15y+", "over15y"],
]);

const amountRanges = /** @type {const} */ ([
  ["0 sats", "_0sats"],
  ["1-10 sats", "_1satTo10sats"],
  ["10-100 sats", "_10satsTo100sats"],
  ["100-1k sats", "_100satsTo1kSats"],
  ["1k-10k sats", "_1kSatsTo10kSats"],
  ["10k-100k sats", "_10kSatsTo100kSats"],
  ["100k-1M sats", "_100kSatsTo1mSats"],
  ["1M-10M sats", "_1mSatsTo10mSats"],
  ["10M sats-1 BTC", "_10mSatsTo1btc"],
  ["1-10 BTC", "_1btcTo10btc"],
  ["10-100 BTC", "_10btcTo100btc"],
  ["100-1k BTC", "_100btcTo1kBtc"],
  ["1k-10k BTC", "_1kBtcTo10kBtc"],
  ["10k-100k BTC", "_10kBtcTo100kBtc"],
  ["100k+ BTC", "over100kBtc"],
]);

const types = /** @type {const} */ ([
  ["P2PK65", "p2pk65"],
  ["P2PK33", "p2pk33"],
  ["P2PKH", "p2pkh"],
  ["OP_RETURN", "opReturn"],
  ["P2MS", "p2ms"],
  ["P2SH", "p2sh"],
  ["P2WPKH", "p2wpkh"],
  ["P2WSH", "p2wsh"],
  ["P2TR", "p2tr"],
  ["P2A", "p2a"],
  ["Unknown", "unknown"],
  ["Empty", "empty"],
]);

const epochs = /** @type {const} */ ([
  ["Epoch 0", "_0"],
  ["Epoch 1", "_1"],
  ["Epoch 2", "_2"],
  ["Epoch 3", "_3"],
  ["Epoch 4", "_4"],
]);

const classes = /** @type {const} */ ([
  ["2009", "_2009"],
  ["2010", "_2010"],
  ["2011", "_2011"],
  ["2012", "_2012"],
  ["2013", "_2013"],
  ["2014", "_2014"],
  ["2015", "_2015"],
  ["2016", "_2016"],
  ["2017", "_2017"],
  ["2018", "_2018"],
  ["2019", "_2019"],
  ["2020", "_2020"],
  ["2021", "_2021"],
  ["2022", "_2022"],
  ["2023", "_2023"],
  ["2024", "_2024"],
  ["2025", "_2025"],
  ["2026", "_2026"],
]);

export const termSeries = createCohortSeries([
  {
    label: "STH",
    color: colors.sky,
    metric: (client) => client.series.cohorts.utxo.sth.supply.total.btc,
  },
  {
    label: "LTH",
    color: colors.orange,
    metric: (client) => client.series.cohorts.utxo.lth.supply.total.btc,
  },
]);

export const ageSeries = createCohortSeriesFromKeys(
  ageRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.ageRange[key].supply.total.btc,
);

export const utxoBalanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.utxo.amountRange[key].supply.total.btc,
);

export const addressBalanceSeries = createCohortSeriesFromKeys(
  amountRanges,
  (key) => (client) =>
    client.series.cohorts.addr.amountRange[key].supply.total.btc,
);

export const typeSeries = createCohortSeriesFromKeys(
  types,
  (key) => (client) =>
    key === "opReturn"
      ? client.series.outputs.value.opReturn.cumulative.btc
      : client.series.cohorts.utxo.type[key].supply.total.btc,
);

export const epochSeries = createCohortSeriesFromKeys(
  epochs,
  (key) => (client) => client.series.cohorts.utxo.epoch[key].supply.total.btc,
);

export const classSeries = createCohortSeriesFromKeys(
  classes,
  (key) => (client) => client.series.cohorts.utxo.class[key].supply.total.btc,
);
