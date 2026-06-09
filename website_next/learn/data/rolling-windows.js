import { createCohortSeries } from "./cohort-series.js";
import { colors } from "../../utils/colors.js";

const rollingWindows = /** @type {const} */ ([
  ["24h", "_24h", colors.sky],
  ["1w", "_1w", colors.cyan],
  ["1m", "_1m", colors.blue],
  ["1y", "_1y", colors.violet],
]);

/** @param {(key: RollingWindowKey) => Metric} createMetric */
export function createRollingWindowSeries(createMetric) {
  return createCohortSeries(
    rollingWindows.map(([label, key, color]) => ({
      label,
      color,
      metric: createMetric(key),
    })),
  );
}

/** @typedef {(typeof rollingWindows)[number][1]} RollingWindowKey */
/** @typedef {import("./cohort-series.js").Metric} Metric */
