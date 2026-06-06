import { createAreaPathData, createLinePathData } from "../path.js";
import { createSvgElement } from "../svg.js";
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
    const path = createSvgElement("path");

    path.dataset.chart = "stacked";
    path.dataset.series = index.toString();
    path.style.setProperty("--color", color);
    path.setAttribute("d", createAreaPathData(points));
    highlight.add(path, index);
    group.append(path);
  }

  for (const index of lineIndexes) {
    const { color, points } = plottedSeries[index];
    const path = createSvgElement("path");

    path.dataset.chart = "line";
    path.dataset.series = index.toString();
    path.style.setProperty("--color", color);
    path.setAttribute("d", createLinePathData(points));
    highlight.add(path, index);
    group.append(path);
  }

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
