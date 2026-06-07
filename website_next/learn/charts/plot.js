import { renderAreaPlot } from "./area/index.js";
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
 * @param {ChartOrder} order
 */
export function renderPlot(
  view,
  group,
  loadedSeries,
  height,
  highlight,
  scale,
  order,
) {
  switch (view) {
    case "line":
      return renderLinePlot(
        group,
        loadedSeries,
        height,
        highlight,
        scale,
        order,
      );
    case "area":
      return renderAreaPlot(
        group,
        loadedSeries,
        height,
        highlight,
        scale,
        order,
      );
    case "bar":
      return renderBarPlot(group, loadedSeries, height, highlight, scale, order);
    case "dots":
      return renderDotsPlot(
        group,
        loadedSeries,
        height,
        highlight,
        scale,
        order,
      );
    case "stacked":
      return renderStackedPlot(
        group,
        loadedSeries,
        height,
        highlight,
        scale,
        order,
      );
  }
}

/** @typedef {import("./highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("./index.js").LoadedSeries} LoadedSeries */
/** @typedef {import("./order.js").ChartOrder} ChartOrder */
/** @typedef {import("./scale.js").ChartScale} ChartScale */
/** @typedef {import("./views.js").ChartView} ChartView */
