import { VIEWBOX_WIDTH } from "../viewbox.js";
import { orderIndexes } from "../order.js";
import { createBounds, includeBoundValue, scaleY } from "../scale.js";

/**
 * @param {LoadedSeries[]} series
 * @param {number[]} stackOrder
 * @param {number[]} lineIndexes
 */
function createStackBounds(series, stackOrder, lineIndexes) {
  const bounds = createBounds();
  const length = series[0].entries.length;

  includeBoundValue(bounds, 0);

  for (let index = 0; index < length; index += 1) {
    let negative = 0;
    let positive = 0;

    for (const seriesIndex of stackOrder) {
      const value = series[seriesIndex].entries[index].value;
      const end = value < 0 ? negative + value : positive + value;

      if (value < 0) negative = end;
      else positive = end;

      includeBoundValue(bounds, end);
    }

    for (const seriesIndex of lineIndexes) {
      const value = series[seriesIndex].entries[index].value;

      includeBoundValue(bounds, value);
    }
  }

  return bounds;
}

/** @returns {StackedPoint[]} */
function createStackedPoints() {
  return [];
}

/**
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {import("../order.js").ChartOrder} order
 * @param {import("../scale.js").ChartScale} scale
 */
export function createStackedSeries(loadedSeries, height, order, scale) {
  const indexes = loadedSeries.map((_, index) => index);
  const lineIndexes = orderIndexes(
    indexes.filter((index) => loadedSeries[index].series.role === "line"),
    order,
  );
  const stackIndexes = orderIndexes(
    indexes.filter((index) => loadedSeries[index].series.role !== "line"),
    order,
  );

  const length = loadedSeries[0].entries.length;
  const xScale = VIEWBOX_WIDTH / (length - 1);
  const plottedSeries = loadedSeries.map(({ series, color }) => ({
    series,
    color,
    points: createStackedPoints(),
  }));

  const bounds = createStackBounds(loadedSeries, stackIndexes, lineIndexes);

  for (let index = 0; index < length; index += 1) {
    let negative = 0;
    let positive = 0;
    const x = index * xScale;

    for (const seriesIndex of stackIndexes) {
      const { date, value } = loadedSeries[seriesIndex].entries[index];
      const start = value < 0 ? negative : positive;
      const end = start + value;

      if (value < 0) negative = end;
      else positive = end;

      const y0 = scaleY(start, bounds, height, scale);
      const y1 = scaleY(end, bounds, height, scale);

      plottedSeries[seriesIndex].points.push({
        date,
        value,
        x,
        y: y1,
        y0,
        y1,
      });
    }

    for (const seriesIndex of lineIndexes) {
      const { date, value } = loadedSeries[seriesIndex].entries[index];
      const y = scaleY(value, bounds, height, scale);

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
