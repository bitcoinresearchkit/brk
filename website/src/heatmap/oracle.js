/** @import { PartialHeatmapOption } from "../../scripts/options/types.js" */
/** @import { HeatmapPoints } from "./types.js" */

import { brk } from "../../scripts/utils/client.js";
import { createAverageGrid } from "./grid.js";
import { INFERNO_LUT, logIntensityColor } from "./lut.js";
import { defaultTooltip } from "./tooltip.js";

const BINS = 2400;
const MIN_LOG = -8;
const BINS_PER_DECADE = 200;

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
      fetch: (date, signal) => fetchOraclePoints(mode, date, signal),
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
 * @returns {Promise<HeatmapPoints>}
 */
async function fetchOraclePoints(mode, date, signal) {
  const values = await firstAvailable((onValue) =>
    mode === "raw"
      ? brk.getOracleHistogramRaw(date, { signal, onValue })
      : brk.getOracleHistogramEma(date, { signal, onValue }),
  );

  return {
    kind: "implicit",
    yStart: MIN_LOG,
    yStep: 1 / BINS_PER_DECADE,
    values,
  };
}

/**
 * @param {(onValue: (value: number[]) => void) => Promise<number[]>} fetch
 * @returns {Promise<number[]>}
 */
function firstAvailable(fetch) {
  return new Promise((resolve, reject) => {
    let settled = false;

    /** @param {number[]} value */
    const resolveOnce = (value) => {
      if (settled) return;
      settled = true;
      resolve(value);
    };

    fetch(resolveOnce).then(resolveOnce, (error) => {
      if (!settled) reject(error);
    });
  });
}
