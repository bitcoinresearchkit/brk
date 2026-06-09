import { createOrderedIndexes } from "../order.js";
import { createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createLineSeries } from "./series.js";

/**
 * @param {PlotContext} context
 */
export function renderLinePlot({
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
      chart: "line",
      color,
      d: createLinePathData(points),
    });
  }

  return plottedSeries;
}
