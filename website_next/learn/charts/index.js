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
 * @param {Chart} chart
 * @param {string} chartKey
 */
export function createChart(chart, chartKey) {
  const figure = document.createElement("figure");
  /** @type {ReturnType<typeof createChartRenderer> | undefined} */
  let renderer;

  figure.dataset.chart = "series";

  function mount() {
    if (renderer) return renderer;

    const plot = document.createElement("div");
    const svg = createSvgElement("svg");
    const controls = document.createElement("footer");
    const chartControls = document.createElement("div");
    const timeControls = document.createElement("div");
    const status = document.createElement("p");
    let currentTimeframe = getDefaultTimeframe(chartKey);
    let currentView = getDefaultView(chartKey, chart.defaultType);
    let currentScale = getDefaultScale(chartKey, chart.defaultScale);
    let currentOrder = getDefaultOrder(chartKey);
    const { legend, menu, items, readout } = createLegend(chart);

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

    const nextRenderer = createChartRenderer({
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

    /**
     * @template {string} T
     * @param {string} dataKey
     * @param {(chartKey: string, value: T) => void} save
     * @param {T} value
     */
    function saveChartSetting(dataKey, save, value) {
      save(chartKey, value);
      figure.dataset[dataKey] = value;
    }

    const viewControl = createViewControl(currentView, (view) => {
      currentView = view;
      saveChartSetting("view", saveView, view);
      nextRenderer.renderCurrent();
    });
    const scaleControl = createScaleControl(currentScale, (scale) => {
      currentScale = scale;
      saveChartSetting("scale", saveScale, scale);
      nextRenderer.renderCurrent();
    });
    const orderControl = createOrderControl(currentOrder, (order) => {
      currentOrder = order;
      saveChartSetting("order", saveOrder, order);
      nextRenderer.renderCurrent();
    });
    const timeframeControl = createTimeframeControl(
      currentTimeframe,
      (timeframe) => {
        currentTimeframe = timeframe;
        saveChartSetting("timeframe", saveTimeframe, timeframe);
        void nextRenderer.loadCurrent();
      },
    );

    chartControls.append(viewControl, scaleControl, orderControl);
    timeControls.append(timeframeControl, createFullscreenButton(figure));
    controls.append(chartControls, timeControls);
    plot.append(svg, status);
    figure.replaceChildren(legend, plot, controls);
    renderer = nextRenderer;

    return renderer;
  }

  onChartVisibility(figure, {
    show: () => mount().resume(),
    hide: () => renderer?.suspend(),
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
