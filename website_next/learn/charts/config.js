/**
 * @param {ChartSeries[]} series
 * @returns {ChartSeries[]}
 */
export function createSeries(series) {
  return series;
}

/**
 * @param {ChartSeries} series
 * @returns {ChartSeries}
 */
export function referenceLine(series) {
  return { ...series, role: "line" };
}

/** @typedef {import("./index.js").ChartSeries} ChartSeries */
