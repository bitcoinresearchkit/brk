import { renderBarPlot } from "./bar/index.js";
import { renderDotsPlot } from "./dots/index.js";
import { renderLinePlot } from "./line/index.js";
import { renderStackedPlot } from "./stacked/index.js";

/**
 * @param {ChartView} view
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {ChartScale} scale
 */
export function renderPlot(view, group, loadedSeries, height, highlight, scale) {
  switch (view) {
    case "line":
      return renderLinePlot(group, loadedSeries, height, highlight, scale);
    case "bar":
    case "bar-reversed":
      return renderBarPlot(
        group,
        loadedSeries,
        height,
        highlight,
        { reversed: view === "bar-reversed" },
        scale,
      );
    case "dots":
      return renderDotsPlot(group, loadedSeries, height, highlight, scale);
    default:
      return renderStackedPlot(
        group,
        loadedSeries,
        height,
        highlight,
        { reversed: view === "stacked-reversed" },
        scale,
      );
  }
}

/** @typedef {import("./highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("./index.js").LoadedSeries} LoadedSeries */
/** @typedef {import("./scale.js").ChartScale} ChartScale */
/** @typedef {import("./views.js").ChartView} ChartView */
