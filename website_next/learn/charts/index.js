import { createFullscreenButton } from "./fullscreen.js";
import { onChartVisibility } from "./intersection.js";
import { createLegend } from "./legend.js";
import { createChartRenderer } from "./renderer.js";
import {
  createScaleControl,
  getDefaultScale,
  saveScale,
} from "./scale.js";
import { createSvgElement } from "./svg.js";
import {
  createTimeframeControl,
  getDefaultTimeframe,
  saveTimeframe,
} from "./timeframes.js";
import {
  createViewControl,
  getDefaultView,
  saveView,
} from "./views.js";
import { FALLBACK_VIEWBOX_HEIGHT, VIEWBOX_WIDTH } from "./viewbox.js";

/** @param {Chart} chart */
export function createChart(chart) {
  const figure = document.createElement("figure");
  const svg = createSvgElement("svg");
  const controls = document.createElement("footer");
  const chartControls = document.createElement("div");
  const timeControls = document.createElement("div");
  const status = document.createElement("p");
  const chartKey = chart.title;
  let currentTimeframe = getDefaultTimeframe(chartKey);
  let currentView = getDefaultView(chartKey);
  let currentScale = getDefaultScale(chartKey);
  const { legend, items, readout } = createLegend(chart);

  figure.dataset.chart = "series";
  figure.dataset.timeframe = currentTimeframe;
  figure.dataset.view = currentView;
  figure.dataset.scale = currentScale;
  svg.setAttribute(
    "viewBox",
    `0 0 ${VIEWBOX_WIDTH} ${FALLBACK_VIEWBOX_HEIGHT}`,
  );
  svg.setAttribute("role", "img");
  svg.setAttribute("aria-label", chart.title);
  svg.setAttribute("tabindex", "0");
  status.setAttribute("aria-live", "polite");
  status.setAttribute("role", "status");

  const renderer = createChartRenderer({
    svg,
    readout,
    items,
    status,
    chart,
    getView: () => currentView,
    getScale: () => currentScale,
    getTimeframe: () => currentTimeframe,
  });
  const viewControl = createViewControl(currentView, (view) => {
    currentView = view;
    saveView(chartKey, view);
    figure.dataset.view = view;
    renderer.renderCurrent();
  });
  const scaleControl = createScaleControl(currentScale, (scale) => {
    currentScale = scale;
    saveScale(chartKey, scale);
    figure.dataset.scale = scale;
    renderer.renderCurrent();
  });
  const timeframeControl = createTimeframeControl(
    currentTimeframe,
    (timeframe) => {
      currentTimeframe = timeframe;
      saveTimeframe(chartKey, timeframe);
      figure.dataset.timeframe = timeframe;
      void renderer.loadCurrent();
    },
  );
  chartControls.append(viewControl, scaleControl);
  timeControls.append(timeframeControl, createFullscreenButton(figure));
  controls.append(chartControls, timeControls);
  figure.append(legend, svg, controls, status);
  onChartVisibility(figure, {
    show: renderer.resume,
    hide: renderer.suspend,
  });

  return figure;
}

/**
 * @typedef {Object} Chart
 * @property {string} title
 * @property {ChartSeries[]} series
 */

/**
 * @typedef {Object} ChartSeries
 * @property {string} label
 * @property {() => string} color
 * @property {"line"} [role]
 * @property {(client: typeof import("../../utils/client.js").brk) => import("./timeframes.js").TimeframeMetric} metric
 */

/**
 * @typedef {Object} ChartResult
 * @property {() => Iterable<[Date, number | null | undefined]>} dateEntries
 */

/**
 * @typedef {Object} LoadedSeries
 * @property {ChartSeries} series
 * @property {string} color
 * @property {{ date: Date, value: number }[]} entries
 */
