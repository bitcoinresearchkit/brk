import { brk } from "../../utils/client.js";
import { renderBarPlot } from "./bar/index.js";
import { createFullscreenButton } from "./fullscreen.js";
import { createSeriesHighlight } from "./highlight.js";
import { onFirstIntersection } from "./intersection.js";
import { createLegend } from "./legend.js";
import { renderLinePlot } from "./line/index.js";
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
 */
function renderPlot(view, group, loadedSeries, height, highlight) {
  switch (view) {
    case "line":
      return renderLinePlot(group, loadedSeries, height, highlight);
    case "bar":
    case "bar-reversed":
      return renderBarPlot(group, loadedSeries, height, highlight, {
        reversed: view === "bar-reversed",
      });
    case "dots":
      return renderDotsPlot(group, loadedSeries, height, highlight);
    default:
      return renderStackedPlot(group, loadedSeries, height, highlight, {
        reversed: view === "stacked-reversed",
      });
  }
}

/**
 * @param {SVGSVGElement} svg
 * @param {Readout} readout
 * @param {HTMLElement[]} items
 * @param {HTMLElement} status
 * @param {Chart} chart
 * @param {() => ChartView} getView
 * @param {() => TimeframeValue} getTimeframe
 */
function createChartRenderer(
  svg,
  readout,
  items,
  status,
  chart,
  getView,
  getTimeframe,
) {
  const group = createSvgElement("g");
  const highlight = createSeriesHighlight(items);
  const getLoadedSeries = createLoadedSeriesCache(chart);
  /** @type {LoadedSeries[]} */
  let loadedSeries = [];
  /** @type {ReturnType<typeof createScrubber> | undefined} */
  let scrubber;
  let loadId = 0;

  svg.append(group);

  function renderCurrent() {
    if (!loadedSeries.length) return;

    const height = getViewBoxHeight(svg);

    svg.setAttribute("viewBox", `0 0 ${VIEWBOX_WIDTH} ${height}`);
    group.replaceChildren();
    highlight.clearNodes();
    scrubber ??= createScrubber(svg, readout, highlight);
    scrubber.setSeries(
      renderPlot(getView(), group, loadedSeries, height, highlight),
      height,
    );
  }

  async function loadCurrent() {
    const id = (loadId += 1);
    svg.setAttribute("aria-busy", "true");

    try {
      const nextSeries = await getLoadedSeries(getTimeframe());

      if (id !== loadId) return;

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

  new ResizeObserver(renderCurrent).observe(svg);

  return {
    loadCurrent,
    renderCurrent,
  };
}

/** @param {Chart} chart */
export function createChart(chart) {
  const figure = document.createElement("figure");
  const svg = createSvgElement("svg");
  const controls = document.createElement("footer");
  const timeControls = document.createElement("div");
  const status = document.createElement("p");
  const chartKey = chart.title;
  let currentTimeframe = getDefaultTimeframe(chartKey);
  let currentView = getDefaultView(chartKey);
  const { legend, items, readout } = createLegend(chart);

  figure.dataset.chart = "series";
  figure.dataset.timeframe = currentTimeframe;
  figure.dataset.view = currentView;
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
    () => currentTimeframe,
  );
  const viewControl = createViewControl(currentView, (view) => {
    currentView = view;
    saveView(chartKey, view);
    figure.dataset.view = view;
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
  timeControls.append(timeframeControl, createFullscreenButton(figure));
  controls.append(viewControl, timeControls);
  figure.append(legend, svg, controls, status);
  onFirstIntersection(figure, () => void renderer.loadCurrent());

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
