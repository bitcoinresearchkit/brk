/** @import { PartialHeatmapOption } from "../../scripts/options/types.js" */
/** @import { HeatmapPoints } from "./types.js" */

import { brk } from "../../scripts/utils/client.js";
import { createAverageGrid } from "./grid.js";
import { INFERNO_LUT, logIntensityColor } from "./lut.js";
import { defaultTooltip } from "./tooltip.js";

const BINS = 2400;
const MIN_LOG = -8;
const BINS_PER_DECADE = 200;
const AMOUNT_CHOICES = [
  { label: "1 sat", value: -8 },
  { label: "10 sats", value: -7 },
  { label: "100 sats", value: -6 },
  { label: "1k sats", value: -5 },
  { label: "10k sats", value: -4 },
  { label: "100k sats", value: -3 },
  { label: "0.01 BTC", value: -2 },
  { label: "0.1 BTC", value: -1 },
  { label: "1 BTC", value: 0 },
  { label: "10 BTC", value: 1 },
  { label: "100 BTC", value: 2 },
  { label: "1k BTC", value: 3 },
  { label: "10k BTC", value: 4 },
];

export const oracleOutputsHeatmapOption = createOracleHeatmapOption(
  "outputs",
  "outputs",
);
export const oraclePaymentsHeatmapOption = createOracleHeatmapOption(
  "payments",
  "payments",
);

/**
 * @param {"outputs" | "payments"} mode
 * @param {string} name
 * @returns {PartialHeatmapOption}
 */
function createOracleHeatmapOption(mode, name) {
  return {
    kind: "heatmap",
    name,
    title:
      mode === "outputs" ? "Output Value Histogram" : "Payment Value Histogram",
    points: {
      fetch: (date, signal, onPoints) =>
        fetchOraclePoints(mode, date, signal, onPoints),
    },
    grid: createAverageGrid({
      yMin: MIN_LOG,
      yMax: MIN_LOG + BINS / BINS_PER_DECADE,
      nativeRows: BINS,
      yOrigin: "top",
    }),
    color: logIntensityColor({ light: INFERNO_LUT, dark: INFERNO_LUT }),
    axis: {
      y: {
        label: "amount",
        choices: AMOUNT_CHOICES,
        format: formatAmount,
      },
    },
    defaults:
      mode === "payments"
        ? {
            from: "2015",
            to: "today",
            yMin: -5,
            yMax: 2,
          }
        : undefined,
    tooltip: defaultTooltip,
  };
}

/**
 * @param {"outputs" | "payments"} mode
 * @param {string} date
 * @param {AbortSignal} signal
 * @param {(points: HeatmapPoints) => void} [onPoints]
 * @returns {Promise<HeatmapPoints>}
 */
async function fetchOraclePoints(mode, date, signal, onPoints) {
  const values = await fetchOracleValues(
    mode,
    date,
    signal,
    onPoints ? (values) => onPoints(toOraclePoints(values)) : undefined,
  );

  return toOraclePoints(values);
}

/**
 * @param {"outputs" | "payments"} mode
 * @param {string} date
 * @param {AbortSignal} signal
 * @param {(values: number[]) => void} [onValue]
 * @returns {Promise<number[]>}
 */
function fetchOracleValues(mode, date, signal, onValue) {
  return (
    mode === "outputs"
      ? brk.getOracleHistogramOutputs(date, { signal, onValue })
      : brk.getOracleHistogramPayments(date, { signal, onValue })
  );
}

/**
 * @param {number[]} values
 * @returns {HeatmapPoints}
 */
function toOraclePoints(values) {
  return {
    kind: "implicit",
    yStart: MIN_LOG,
    yStep: 1 / BINS_PER_DECADE,
    values,
  };
}

/** @param {number} value */
function formatAmount(value) {
  const rounded = Math.round(value);
  if (Math.abs(value - rounded) < 0.001) {
    const choice = AMOUNT_CHOICES.find((choice) => choice.value === rounded);
    if (choice) return choice.label;
  }
  const btc = 10 ** value;
  if (btc >= 1) return `${formatCompact(btc)} BTC`;
  return `${formatCompact(btc * 100_000_000)} sats`;
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
