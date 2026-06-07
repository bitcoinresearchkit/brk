import { createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createLineSeries } from "./series.js";

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {import("../scale.js").ChartScale} scale
 */
export function renderLinePlot(group, loadedSeries, height, highlight, scale) {
  const plottedSeries = createLineSeries(loadedSeries, height, scale);

  plottedSeries.forEach(({ color, points }, index) => {
    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "line",
      color,
      d: createLinePathData(points),
    });
  });

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
