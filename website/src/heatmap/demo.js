/** @import { PartialHeatmapOption } from "../../scripts/options/types.js" */
/** @import { HeatmapPoints } from "./types.js" */

import { createAverageGrid } from "./grid.js";
import { INFERNO_LUT, intensityColor } from "./lut.js";
import { GENESIS_DATE, todayISODate } from "./time.js";
import { defaultTooltip } from "./tooltip/index.js";

const ROWS = 160;
const DAY_MS = 86_400_000;
const GENESIS_TIME = Date.parse(`${GENESIS_DATE}T00:00:00Z`);

/** @satisfies {PartialHeatmapOption} */
export const demoHeatmapOption = {
  kind: "heatmap",
  name: "Demo",
  title: "Heatmap Demo",
  points: {
    fetch: fetchDemoPoints,
  },
  grid: createAverageGrid({ yMin: 0, yMax: 1, nativeRows: ROWS }),
  color: intensityColor(INFERNO_LUT),
  tooltip: defaultTooltip(),
};

/**
 * @param {string} date
 * @param {AbortSignal} signal
 * @returns {Promise<HeatmapPoints>}
 */
async function fetchDemoPoints(date, signal) {
  throwIfAborted(signal);

  const values = new Float32Array(ROWS);
  const endTime = Date.parse(`${todayISODate()}T00:00:00Z`);
  const time = Date.parse(`${date}T00:00:00Z`);
  const x = Math.min(
    1,
    Math.max(
      0,
      (time - GENESIS_TIME) / Math.max(DAY_MS, endTime - GENESIS_TIME),
    ),
  );

  for (let row = 0; row < ROWS; row++) {
    const y = row / (ROWS - 1);
    const ridge = Math.exp(-((y - (0.75 - x * 0.45)) ** 2) / 0.01);
    const blob = Math.exp(
      -(((x - 0.72) ** 2) / 0.018 + ((y - 0.28) ** 2) / 0.028),
    );
    const floor = x * 0.18 + (1 - y) * 0.12;
    values[row] = Math.min(1, Math.max(0, ridge * 0.65 + blob * 0.45 + floor));
  }

  return {
    kind: "implicit",
    yStart: 0,
    yStep: 1 / (ROWS - 1),
    values,
  };
}

/** @param {AbortSignal} signal */
function throwIfAborted(signal) {
  if (signal.aborted) {
    throw new DOMException("The operation was aborted.", "AbortError");
  }
}
