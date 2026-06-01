import { brk } from "../../scripts/utils/client.js";
import { numberToShortUSFormat } from "../../scripts/utils/format.js";
import { createAverageGrid } from "./grid.js";
import { INFERNO_LUT, logIntensityColor } from "./lut.js";
import { defaultTooltip } from "./tooltip/index.js";

const COHORT = "all";
const AGGREGATION = "raw";
const MIN_LOG = -2;
const MAX_LOG = 6;
const DEFAULT_MIN_LOG = Math.log10(1_000);
const DEFAULT_MAX_LOG = Math.log10(250_000);
const PRICE_CHOICES = [
  { label: "$0.01", value: Math.log10(0.01) },
  { label: "$0.1", value: Math.log10(0.1) },
  { label: "$1", value: 0 },
  { label: "$10", value: 1 },
  { label: "$100", value: 2 },
  { label: "$250", value: Math.log10(250) },
  { label: "$1k", value: Math.log10(1_000) },
  { label: "$2.5k", value: Math.log10(2_500) },
  { label: "$5k", value: Math.log10(5_000) },
  { label: "$10k", value: Math.log10(10_000) },
  { label: "$25k", value: Math.log10(25_000) },
  { label: "$50k", value: Math.log10(50_000) },
  { label: "$100k", value: Math.log10(100_000) },
  { label: "$250k", value: Math.log10(250_000) },
  { label: "$500k", value: Math.log10(500_000) },
  { label: "$1M", value: Math.log10(1_000_000) },
];

/** @satisfies {PartialHeatmapOption} */
export const urpdSupplyHeatmapOption = {
  kind: "heatmap",
  name: "Supply",
  title: "URPD Supply",
  points: {
    fetch: (date, signal, onPoints) =>
      fetchUrpdSupplyPoints(date, signal, onPoints),
  },
  grid: createAverageGrid({
    yMin: MIN_LOG,
    yMax: MAX_LOG,
  }),
  color: logIntensityColor(INFERNO_LUT),
  axis: {
    y: {
      label: "price",
      choices: PRICE_CHOICES,
      format: formatPrice,
    },
  },
  defaults: {
    from: "2017",
    to: "today",
    yMin: DEFAULT_MIN_LOG,
    yMax: DEFAULT_MAX_LOG,
  },
  tooltip: defaultTooltip({
    valueLabel: "Supply",
    averageLabel: "Avg supply",
    formatValue: formatBitcoin,
  }),
};

/**
 * @param {string} date
 * @param {AbortSignal} signal
 * @param {(points: HeatmapPoints) => void} [onPoints]
 * @returns {Promise<HeatmapPoints>}
 */
async function fetchUrpdSupplyPoints(date, signal, onPoints) {
  /** @type {HeatmapPoints | undefined} */
  let points;
  const urpd = await brk.getUrpdAt(COHORT, date, AGGREGATION, {
    signal,
    cache: false,
    onValue: onPoints
      ? (value) => {
          points = toSupplyPoints(value);
          onPoints(points);
        }
      : undefined,
  });

  return points ?? toSupplyPoints(urpd);
}

/**
 * @param {Urpd} urpd
 * @returns {HeatmapPoints}
 */
function toSupplyPoints(urpd) {
  const buckets = urpd.buckets;
  const y = new Float64Array(buckets.length);
  const values = new Float64Array(buckets.length);
  let length = 0;

  for (let i = 0; i < buckets.length; i++) {
    const bucket = buckets[i];
    if (bucket.priceFloor <= 0 || !Number.isFinite(bucket.supply)) continue;
    y[length] = Math.log10(bucket.priceFloor);
    values[length] = bucket.supply;
    length++;
  }

  return {
    kind: "explicit",
    y: y.subarray(0, length),
    values: values.subarray(0, length),
  };
}

/** @param {number} value */
function formatPrice(value) {
  const rounded = Math.round(value);
  if (Math.abs(value - rounded) < 0.001) {
    const choice = PRICE_CHOICES.find((choice) => choice.value === rounded);
    if (choice) return choice.label;
  }

  const price = 10 ** value;
  if (price >= 1_000_000) return `$${formatCompact(price / 1_000_000)}M`;
  if (price >= 1_000) return `$${formatCompact(price / 1_000)}k`;
  return `$${formatCompact(price)}`;
}

/** @param {number} value */
function formatBitcoin(value) {
  return `${numberToShortUSFormat(value)} BTC`;
}

/** @param {number} value */
function formatCompact(value) {
  if (value >= 1000) return `${formatNumber(value / 1000)}k`;
  return formatNumber(value);
}

/** @param {number} value */
function formatNumber(value) {
  if (value >= 100) return String(Math.round(value));
  if (value >= 10) return trimNumber(value.toFixed(1));
  return trimNumber(value.toFixed(2));
}

/** @param {string} value */
function trimNumber(value) {
  return value.replace(/\.?0+$/, "");
}
