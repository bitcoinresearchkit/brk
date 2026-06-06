import { createLinePathData } from "../path.js";
import { createSvgElement } from "../svg.js";
import { createLineSeries } from "./series.js";

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 */
export function renderLinePlot(group, loadedSeries, height, highlight) {
  const plottedSeries = createLineSeries(loadedSeries, height);

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
