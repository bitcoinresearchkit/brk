import { createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createOrderedIndexes } from "../order.js";
import { createLineSeries } from "./series.js";

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {import("../scale.js").ChartScale} scale
 * @param {import("../order.js").ChartOrder} order
 */
export function renderLinePlot(
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
      chart: "line",
      color,
      d: createLinePathData(points),
    });
  }

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
