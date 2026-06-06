import { formatCoordinate } from "../path.js";
import { createSvgElement } from "../svg.js";
import { createLineSeries } from "../line/series.js";

const radius = 1;

/** @param {{ x: number, y: number }[]} points */
function createDotsPathData(points) {
  return points
    .map(
      ({ x, y }) =>
        `M${formatCoordinate(x - radius)} ${formatCoordinate(y)}` +
        `a${radius} ${radius} 0 1 0 ${radius * 2} 0` +
        `a${radius} ${radius} 0 1 0 ${radius * -2} 0`,
    )
    .join(" ");
}

/**
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {import("../scale.js").ChartScale} scale
 */
export function renderDotsPlot(group, loadedSeries, height, highlight, scale) {
  const plottedSeries = createLineSeries(loadedSeries, height, scale);

  plottedSeries.forEach(({ color, points }, index) => {
    const path = createSvgElement("path");

    path.dataset.chart = "dots";
    path.dataset.series = index.toString();
    path.style.setProperty("--color", color);
    path.setAttribute("d", createDotsPathData(points));
    highlight.addNode(path, index);
    group.append(path);
  });

  return plottedSeries;
}

/** @typedef {import("../highlight.js").SeriesHighlight} SeriesHighlight */
/** @typedef {import("../index.js").LoadedSeries} LoadedSeries */
