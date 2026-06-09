import { VIEWBOX_WIDTH } from "../viewbox.js";
import { createBounds, includeBoundValue, scaleY } from "../scale.js";

/** @param {LoadedSeries[]} series */
function createValueBounds(series) {
  const bounds = createBounds();

  for (const { entries } of series) {
    for (const { value } of entries) {
      includeBoundValue(bounds, value);
    }
  }

  return bounds;
}

/**
 * @param {ChartEntry[]} entries
 * @param {ScaleBounds} bounds
 * @param {number} height
 * @param {ChartScale} scale
 * @returns {ChartPoint[]}
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
 * @param {ChartScale} scale
 */
export function createLineSeries(loadedSeries, height, scale) {
  const bounds = createValueBounds(loadedSeries);

  return loadedSeries.map(({ series, color, entries }) => ({
    series,
    color,
    points: createPoints(entries, bounds, height, scale),
  }));
}
