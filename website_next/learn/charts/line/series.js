import { VIEWBOX_WIDTH } from "../viewbox.js";
import { scaleY } from "../scale.js";

/** @param {LoadedSeries[]} series */
function createValueBounds(series) {
  let min = Infinity;
  let max = -Infinity;
  let minPositive = Infinity;

  for (const { entries } of series) {
    for (const { value } of entries) {
      min = Math.min(min, value);
      max = Math.max(max, value);
      if (value > 0) minPositive = Math.min(minPositive, value);
    }
  }

  return { min, max, minPositive };
}

/**
 * @param {{ date: Date, value: number }[]} entries
 * @param {import("../scale.js").ScaleBounds} bounds
 * @param {number} height
 * @param {import("../scale.js").ChartScale} scale
 */
function createPoints(entries, bounds, height, scale) {
  const xScale = VIEWBOX_WIDTH / (entries.length - 1);

  return entries.map(({ date, value }, index) => ({
    date,
    value,
    x: index * xScale,
    y: scaleY(value, bounds, height, scale),
  }));
}

/**
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {import("../scale.js").ChartScale} scale
 */
export function createLineSeries(loadedSeries, height, scale) {
  const bounds = createValueBounds(loadedSeries);

  return loadedSeries.map(({ series, color, entries }) => ({
    series,
    color,
    points: createPoints(entries, bounds, height, scale),
  }));
}

/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
