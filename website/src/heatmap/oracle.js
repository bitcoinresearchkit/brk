/** @import { PartialHeatmapOption } from "../../scripts/options/types.js" */
/** @import { HeatmapPoints } from "./types.js" */

import { brk } from "../../scripts/utils/client.js";
import { createAverageGrid } from "./grid.js";
import { INFERNO_LUT, logIntensityColor } from "./lut.js";
import { defaultTooltip } from "./tooltip.js";

const BINS = 2400;
const MIN_LOG = -8;
const BINS_PER_DECADE = 200;
const MAX_LOG = MIN_LOG + (BINS - 1) / BINS_PER_DECADE;

export const oracleRawHeatmapOption = createOracleHeatmapOption("raw", "Raw");
export const oracleEmaHeatmapOption = createOracleHeatmapOption("ema", "EMA");

/**
 * @param {"raw" | "ema"} mode
 * @param {string} name
 * @returns {PartialHeatmapOption}
 */
function createOracleHeatmapOption(mode, name) {
  return {
    kind: "heatmap",
    name,
    title: `Oracle ${name} Histogram`,
    points: {
      fetch: (date, signal, onPoints) =>
        fetchOraclePoints(mode, date, signal, onPoints),
    },
    grid: createAverageGrid({
      yStart: MIN_LOG,
      yEnd: MIN_LOG + BINS / BINS_PER_DECADE,
      nativeRows: BINS,
    }),
    color: logIntensityColor({ light: INFERNO_LUT, dark: INFERNO_LUT }),
    tooltip: defaultTooltip,
  };
}

/**
 * @param {"raw" | "ema"} mode
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
 * @param {"raw" | "ema"} mode
 * @param {string} date
 * @param {AbortSignal} signal
 * @param {(values: number[]) => void} [onValue]
 * @returns {Promise<number[]>}
 */
function fetchOracleValues(mode, date, signal, onValue) {
  return (
    mode === "raw"
      ? brk.getOracleHistogramRaw(date, { signal, onValue })
      : brk.getOracleHistogramEma(date, { signal, onValue })
  );
}

/**
 * @param {number[]} values
 * @returns {HeatmapPoints}
 */
function toOraclePoints(values) {
  return {
    kind: "implicit",
    yStart: MAX_LOG,
    yStep: -1 / BINS_PER_DECADE,
    values,
  };
}
