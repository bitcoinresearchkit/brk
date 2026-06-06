import { createLinePathData } from "../path.js";
import { createSvgElement } from "../svg.js";
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
    const path = createSvgElement("path");

    path.dataset.chart = "line";
    path.dataset.series = index.toString();
    path.style.setProperty("--color", color);
    path.setAttribute("d", createLinePathData(points));
    highlight.add(path, index);
    group.append(path);
  });

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
