import { createAreaPathData, createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createStackedSeries } from "./series.js";

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {{ reversed: boolean }} options
 * @param {import("../scale.js").ChartScale} scale
 */
export function renderStackedPlot(
  group,
  loadedSeries,
  height,
  highlight,
  options,
  scale,
) {
  const { lineIndexes, plottedSeries, stackIndexes } = createStackedSeries(
    loadedSeries,
    height,
    options.reversed,
    scale,
  );

  for (const index of stackIndexes) {
    const { color, points } = plottedSeries[index];
    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "stacked",
      color,
      d: createAreaPathData(points),
    });
  }

  for (const index of lineIndexes) {
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
