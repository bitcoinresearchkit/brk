import { createAreaPathData, createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createOrderedIndexes } from "../order.js";
import { createLineSeries } from "../line/series.js";

/**
 * @param {number} height
 * @param {{ date: Date, value: number, x: number, y: number }[]} points
 */
function createAreaPoints(height, points) {
  return points.map((point) => ({
    ...point,
    y0: height,
    y1: point.y,
  }));
}

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {import("../scale.js").ChartScale} scale
 * @param {import("../order.js").ChartOrder} order
 */
export function renderAreaPlot(
  group,
  loadedSeries,
  height,
  highlight,
  scale,
  order,
) {
  const plottedSeries = createLineSeries(loadedSeries, height, scale);
  const indexes = createOrderedIndexes(plottedSeries.length, order);

  for (const index of indexes) {
    const { color, points } = plottedSeries[index];
    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "area",
      color,
      d: createAreaPathData(createAreaPoints(height, points)),
    });

    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "line",
      color,
      d: createLinePathData(points),
    });
  }

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
