import { renderAreaPlot } from "./area/index.js";
import { renderBarPlot } from "./bar/index.js";
import { renderDotsPlot } from "./dots/index.js";
import { renderLinePlot } from "./line/index.js";
import { renderStackedPlot } from "./stacked/index.js";

/**
 * @param {ChartView} view
 * @param {PlotContext} context
 */
export function renderPlot(view, context) {
  switch (view) {
    case "line":
      return renderLinePlot(context);
    case "area":
      return renderAreaPlot(context);
    case "bar":
      return renderBarPlot(context);
    case "dots":
      return renderDotsPlot(context);
    case "stacked":
      return renderStackedPlot(context);
  }
}
