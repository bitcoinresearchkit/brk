import { createLinePathData, formatCoordinate } from "../path.js";
import { createSvgElement } from "../svg.js";
import { VIEWBOX_WIDTH } from "../viewbox.js";
import { createStackedSeries } from "../stacked/series.js";

/**
 * @param {number} value
 * @param {number} min
 * @param {number} max
 */
function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

/** @param {{ x: number, y0: number, y1: number }[]} points */
function getBarWidth(points) {
  return points.length > 1 ? (VIEWBOX_WIDTH / (points.length - 1)) * 0.8 : 1;
}

/**
 * @param {{ x: number, y0: number, y1: number }[]} points
 * @param {number} width
 */
function createBarPathData(points, width) {
  return points
    .map(({ x, y0, y1 }) => {
      const left = clamp(x - width / 2, 0, VIEWBOX_WIDTH - width);
      const right = left + width;
      const top = Math.min(y0, y1);
      const bottom = Math.max(y0, y1);

      return (
        `M${formatCoordinate(left)} ${formatCoordinate(top)}` +
        `H${formatCoordinate(right)}V${formatCoordinate(bottom)}` +
        `H${formatCoordinate(left)}Z`
      );
    })
    .join(" ");
}

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {{ reversed: boolean }} options
 */
export function renderBarPlot(group, loadedSeries, height, highlight, options) {
  const { lineIndexes, plottedSeries, stackIndexes } = createStackedSeries(
    loadedSeries,
    height,
    options.reversed,
  );

  for (const index of stackIndexes) {
    const { color, points } = plottedSeries[index];
    const path = createSvgElement("path");

    path.dataset.chart = "bar";
    path.dataset.series = index.toString();
    path.style.setProperty("--color", color);
    path.setAttribute("d", createBarPathData(points, getBarWidth(points)));
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
