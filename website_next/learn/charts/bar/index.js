import { createLinePathData, formatCoordinate } from "../path.js";
import { VIEWBOX_WIDTH } from "../viewbox.js";
import { createStackedSeries } from "../stacked/series.js";
import { clamp } from "../math.js";
import { appendSeriesPath } from "../series-path.js";

/** @param {StackedPoint[]} points */
function getBarWidth(points) {
  return points.length > 1 ? (VIEWBOX_WIDTH / (points.length - 1)) * 0.8 : 1;
}

/**
 * @param {StackedPoint[]} points
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
 * @param {PlotContext} context
 */
export function renderBarPlot({
  group,
  loadedSeries,
  height,
  highlight,
  scale,
  order,
}) {
  const { lineIndexes, plottedSeries, stackIndexes } = createStackedSeries(
    loadedSeries,
    height,
    order,
    scale,
  );

  for (const index of stackIndexes) {
    const { color, points } = plottedSeries[index];
    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "bar",
      color,
      d: createBarPathData(points, getBarWidth(points)),
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
