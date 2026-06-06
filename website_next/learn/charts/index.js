import { brk } from "../../utils/client.js";
import { renderBarPlot } from "./bar/index.js";
import { createFullscreenButton } from "./fullscreen.js";
import { createSeriesHighlight } from "./highlight.js";
import { onChartVisibility } from "./intersection.js";
import { createLegend } from "./legend.js";
import { renderLinePlot } from "./line/index.js";
import {
  createScaleControl,
  getDefaultScale,
  saveScale,
} from "./scale.js";
import { createScrubber } from "./scrubber.js";
import { renderDotsPlot } from "./dots/index.js";
import { createSvgElement } from "./svg.js";
import { renderStackedPlot } from "./stacked/index.js";
import {
  createTimeframeControl,
  fetchTimeframe,
  getDefaultTimeframe,
  saveTimeframe,
} from "./timeframes.js";
import {
  createViewControl,
  getDefaultView,
  saveView,
} from "./views.js";
import {
  FALLBACK_VIEWBOX_HEIGHT,
  getViewBoxHeight,
  VIEWBOX_WIDTH,
} from "./viewbox.js";

/** @typedef {import("./legend.js").Readout} Readout */
/** @typedef {import("./scale.js").ChartScale} ChartScale */
/** @typedef {import("./timeframes.js").TimeframeValue} TimeframeValue */
/** @typedef {import("./views.js").ChartView} ChartView */

/**
 * @param {ChartResult} result
 * @returns {{ date: Date, value: number }[]}
 */
function createEntries(result) {
  /** @type {{ date: Date, value: number }[]} */
  const entries = [];
  /** @type {number | undefined} */
  let lastValue;

  for (const [date, value] of result.dateEntries()) {
    if (typeof value === "number" && Number.isFinite(value)) lastValue = value;
    if (lastValue !== undefined) entries.push({ date, value: lastValue });
  }

  return entries;
}

/**
 * @param {Chart} chart
 * @param {TimeframeValue} timeframe
 * @returns {Promise<LoadedSeries[]>}
 */
async function loadSeries(chart, timeframe) {
  return Promise.all(
    chart.series.map(async (item) => ({
      series: item,
      color: item.color(),
      entries: createEntries(await fetchTimeframe(item.metric(brk), timeframe)),
    })),
  );
}

/** @param {Chart} chart */
function createLoadedSeriesCache(chart) {
  /** @type {Map<TimeframeValue, Promise<LoadedSeries[]>>} */
  const cache = new Map();

  /** @param {TimeframeValue} timeframe */
  return function getLoadedSeries(timeframe) {
    let promise = cache.get(timeframe);

    if (!promise) {
      promise = loadSeries(chart, timeframe).catch((error) => {
        cache.delete(timeframe);
        throw error;
      });
      cache.set(timeframe, promise);
    }

    return promise;
  };
}

/**
 * @param {ChartView} view
 * @param {SVGGElement} group
 * @param {LoadedSeries[]} loadedSeries
 * @param {number} height
 * @param {SeriesHighlight} highlight
 * @param {ChartScale} scale
 */
function renderPlot(view, group, loadedSeries, height, highlight, scale) {
  switch (view) {
    case "line":
      return renderLinePlot(group, loadedSeries, height, highlight, scale);
    case "bar":
    case "bar-reversed":
      return renderBarPlot(
        group,
        loadedSeries,
        height,
        highlight,
        { reversed: view === "bar-reversed" },
        scale,
      );
    case "dots":
      return renderDotsPlot(group, loadedSeries, height, highlight, scale);
    default:
      return renderStackedPlot(
        group,
        loadedSeries,
        height,
        highlight,
        { reversed: view === "stacked-reversed" },
        scale,
      );
  }
}

/**
 * @param {SVGSVGElement} svg
 * @param {Readout} readout
 * @param {HTMLElement[]} items
 * @param {HTMLElement} status
 * @param {Chart} chart
 * @param {() => ChartView} getView
 * @param {() => ChartScale} getScale
 * @param {() => TimeframeValue} getTimeframe
 */
function createChartRenderer(
  svg,
  readout,
  items,
  status,
  chart,
  getView,
  getScale,
  getTimeframe,
) {
  const group = createSvgElement("g");
  const highlight = createSeriesHighlight(items);
  const getLoadedSeries = createLoadedSeriesCache(chart);
  /** @type {LoadedSeries[]} */
  let loadedSeries = [];
  /** @type {ReturnType<typeof createScrubber> | undefined} */
  let scrubber;
  const resizeObserver = new ResizeObserver(renderCurrent);
  let active = false;
  let loadId = 0;

  svg.append(group);

  function renderCurrent() {
    if (!active || !loadedSeries.length) return;

    const height = getViewBoxHeight(svg);

    svg.setAttribute("viewBox", `0 0 ${VIEWBOX_WIDTH} ${height}`);
    group.replaceChildren();
    highlight.clearNodes();
    scrubber ??= createScrubber(svg, readout, highlight);
    scrubber.setSeries(
      renderPlot(getView(), group, loadedSeries, height, highlight, getScale()),
      height,
    );
  }

  async function loadCurrent() {
    const id = (loadId += 1);
    svg.setAttribute("aria-busy", "true");

    try {
      const nextSeries = await getLoadedSeries(getTimeframe());

      if (id !== loadId || !active) return;

      loadedSeries = nextSeries;
      renderCurrent();
      status.textContent = "";
    } catch (error) {
      if (id !== loadId) return;
      console.error(error);
      status.textContent = "Chart unavailable";
    } finally {
      if (id === loadId) svg.removeAttribute("aria-busy");
    }
  }

  function resume() {
    if (active) return;

    active = true;
    resizeObserver.observe(svg);
    void loadCurrent();
  }

  function suspend() {
    if (!active) return;

    active = false;
    loadId += 1;
    resizeObserver.disconnect();
    group.replaceChildren();
    highlight.clearNodes();
    scrubber?.clear();
    svg.removeAttribute("aria-busy");
  }

  return {
    loadCurrent,
    renderCurrent,
    resume,
    suspend,
  };
}

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

  const renderer = createChartRenderer(
    svg,
    readout,
    items,
    status,
    chart,
    () => currentView,
    () => currentScale,
    () => currentTimeframe,
  );
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
 * @property {(client: typeof brk) => import("./timeframes.js").TimeframeMetric} metric
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

/** @typedef {import("./highlight.js").SeriesHighlight} SeriesHighlight */
