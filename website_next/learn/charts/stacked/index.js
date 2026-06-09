import { createAreaPathData, createLinePathData } from "../path.js";
import { appendSeriesPath } from "../series-path.js";
import { createStackedSeries } from "./series.js";

/**
 * @param {PlotContext} context
 */
export function renderStackedPlot({
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
      chart: "stacked",
      color,
      d: createAreaPathData(points),
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
