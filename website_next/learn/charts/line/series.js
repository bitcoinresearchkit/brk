import { VIEWBOX_WIDTH } from "../viewbox.js";

/** @param {LoadedSeries[]} series */
function createValueBounds(series) {
  let min = Infinity;
  let max = -Infinity;

  for (const { entries } of series) {
    for (const { value } of entries) {
      min = Math.min(min, value);
      max = Math.max(max, value);
    }
  }

  return { min, max };
}

/**
 * @param {{ date: Date, value: number }[]} entries
 * @param {{ min: number, max: number }} bounds
 * @param {number} height
 */
function createPoints(entries, bounds, height) {
  const xScale = VIEWBOX_WIDTH / (entries.length - 1);
  const yScale =
    bounds.max === bounds.min ? 0 : height / (bounds.max - bounds.min);

  return entries.map(({ date, value }, index) => ({
    date,
    value,
    x: index * xScale,
    y:
      bounds.max === bounds.min
        ? height / 2
        : height - (value - bounds.min) * yScale,
  }));
}

/**
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 */
export function createLineSeries(loadedSeries, height) {
  const bounds = createValueBounds(loadedSeries);

  return loadedSeries.map(({ series, color, entries }) => ({
    series,
    color,
    points: createPoints(entries, bounds, height),
  }));
}

/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
