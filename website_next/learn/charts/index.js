import { createFullscreenButton } from "./fullscreen.js";
import { onChartVisibility } from "./intersection.js";
import { createLegend } from "./legend/index.js";
import {
  createOrderControl,
  getDefaultOrder,
  saveOrder,
} from "./order.js";
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

/**
 * @template {string} T
 * @param {Object} args
 * @param {T} args.currentValue
 * @param {(currentValue: T, onChange: (value: T) => void) => HTMLFieldSetElement} args.createControl
 * @param {(chartKey: string, value: T) => void} args.save
 * @param {string} args.chartKey
 * @param {HTMLElement} args.figure
 * @param {string} args.dataKey
 * @param {(value: T) => void} args.setValue
 * @param {() => void} args.render
 */
function createChartSettingControl({
  currentValue,
  createControl,
  save,
  chartKey,
  figure,
  dataKey,
  setValue,
  render,
}) {
  return createControl(currentValue, (value) => {
    setValue(value);
    save(chartKey, value);
    figure.dataset[dataKey] = value;
    render();
  });
}

/** @param {Chart} chart */
export function createChart(chart) {
  const figure = document.createElement("figure");
  const plot = document.createElement("div");
  const svg = createSvgElement("svg");
  const controls = document.createElement("footer");
  const chartControls = document.createElement("div");
  const timeControls = document.createElement("div");
  const status = document.createElement("p");
  const chartKey = chart.title;
  let currentTimeframe = getDefaultTimeframe(chartKey);
  let currentView = getDefaultView(chartKey, chart.defaultType);
  let currentScale = getDefaultScale(chartKey, chart.defaultScale);
  let currentOrder = getDefaultOrder(chartKey);
  const { legend, menu, items, readout } = createLegend(chart);

  figure.dataset.chart = "series";
  plot.dataset.chart = "plot";
  figure.dataset.timeframe = currentTimeframe;
  figure.dataset.view = currentView;
  figure.dataset.scale = currentScale;
  figure.dataset.order = currentOrder;
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
    menu,
    items,
    status,
    chart,
    getView: () => currentView,
    getScale: () => currentScale,
    getOrder: () => currentOrder,
    getTimeframe: () => currentTimeframe,
  });
  const viewControl = createChartSettingControl({
    currentValue: currentView,
    createControl: createViewControl,
    save: saveView,
    chartKey,
    figure,
    dataKey: "view",
    setValue: (view) => {
      currentView = view;
    },
    render: renderer.renderCurrent,
  });
  const scaleControl = createChartSettingControl({
    currentValue: currentScale,
    createControl: createScaleControl,
    save: saveScale,
    chartKey,
    figure,
    dataKey: "scale",
    setValue: (scale) => {
      currentScale = scale;
    },
    render: renderer.renderCurrent,
  });
  const orderControl = createChartSettingControl({
    currentValue: currentOrder,
    createControl: createOrderControl,
    save: saveOrder,
    chartKey,
    figure,
    dataKey: "order",
    setValue: (order) => {
      currentOrder = order;
    },
    render: renderer.renderCurrent,
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
  chartControls.append(viewControl, scaleControl, orderControl);
  timeControls.append(timeframeControl, createFullscreenButton(figure));
  controls.append(chartControls, timeControls);
  plot.append(svg, status);
  figure.append(legend, plot, controls);
  onChartVisibility(figure, {
    show: renderer.resume,
    hide: renderer.suspend,
  });

  return figure;
}

/**
 * @typedef {Object} Chart
 * @property {string} title
 * @property {import("./units.js").ChartUnit} unit
 * @property {import("./views.js").ChartView} [defaultType]
 * @property {import("./scale.js").ChartScale} [defaultScale]
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
