import { VIEWBOX_WIDTH } from "../viewbox.js";

/**
 * @param {LoadedSeries[]} series
 * @param {number[]} stackIndexes
 * @param {number[]} lineIndexes
 */
function createStackBounds(series, stackIndexes, lineIndexes) {
  const length = series[0].entries.length;
  let min = 0;
  let max = 0;

  for (let index = 0; index < length; index += 1) {
    let negative = 0;
    let positive = 0;

    for (const seriesIndex of stackIndexes) {
      const value = series[seriesIndex].entries[index].value;

      if (value < 0) negative += value;
      else positive += value;
    }

    min = Math.min(min, negative);
    max = Math.max(max, positive);

    for (const seriesIndex of lineIndexes) {
      const value = series[seriesIndex].entries[index].value;

      min = Math.min(min, value);
      max = Math.max(max, value);
    }
  }

  return { min, max };
}

/**
 * @param {number} value
 * @param {{ min: number, max: number }} bounds
 * @param {number} height
 */
function scaleY(value, bounds, height) {
  return bounds.max === bounds.min
    ? height / 2
    : height - ((value - bounds.min) / (bounds.max - bounds.min)) * height;
}

/** @returns {StackedPoint[]} */
function createStackedPoints() {
  return [];
}

/**
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {boolean} reversed
 */
export function createStackedSeries(loadedSeries, height, reversed) {
  const indexes = loadedSeries.map((_, index) => index);
  const lineIndexes = indexes.filter(
    (index) => loadedSeries[index].series.role === "line",
  );
  const stackIndexes = indexes.filter(
    (index) => loadedSeries[index].series.role !== "line",
  );

  const bounds = createStackBounds(loadedSeries, stackIndexes, lineIndexes);
  const length = loadedSeries[0].entries.length;
  const xScale = VIEWBOX_WIDTH / (length - 1);
  const order = [...stackIndexes];
  const plottedSeries = loadedSeries.map(({ series, color }) => ({
    series,
    color,
    points: createStackedPoints(),
  }));

  if (reversed) order.reverse();

  for (let index = 0; index < length; index += 1) {
    let negative = 0;
    let positive = 0;
    const x = index * xScale;

    for (const seriesIndex of order) {
      const { date, value } = loadedSeries[seriesIndex].entries[index];
      const start = value < 0 ? negative : positive;
      const end = start + value;

      if (value < 0) negative = end;
      else positive = end;

      plottedSeries[seriesIndex].points.push({
        date,
        value,
        x,
        y: scaleY(end, bounds, height),
        y0: scaleY(start, bounds, height),
        y1: scaleY(end, bounds, height),
      });
    }

    for (const seriesIndex of lineIndexes) {
      const { date, value } = loadedSeries[seriesIndex].entries[index];
      const y = scaleY(value, bounds, height);

      plottedSeries[seriesIndex].points.push({
        date,
        value,
        x,
        y,
        y0: y,
        y1: y,
      });
    }
  }

  return {
    lineIndexes,
    plottedSeries,
    stackIndexes,
  };
}

/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */

/**
 * @typedef {Object} StackedPoint
 * @property {Date} date
 * @property {number} value
 * @property {number} x
 * @property {number} y
 * @property {number} y0
 * @property {number} y1
 */
