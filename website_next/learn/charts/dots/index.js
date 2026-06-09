import { formatCoordinate } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createOrderedIndexes } from "../order.js";
import { createLineSeries } from "../line/series.js";

const radius = 1;

/** @param {ChartPoint[]} points */
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
 * @param {PlotContext} context
 */
export function renderDotsPlot({
  group,
  loadedSeries,
  height,
  highlight,
  scale,
  order,
}) {
  const plottedSeries = createLineSeries(loadedSeries, height, scale);
  const indexes = createOrderedIndexes(plottedSeries.length, order);

  for (const index of indexes) {
    const { color, points } = plottedSeries[index];
    appendSeriesPath({
      group,
      highlight,
      index,
      chart: "dots",
      color,
      d: createDotsPathData(points),
    });
  }

  return plottedSeries;
}
