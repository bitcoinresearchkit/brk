import { renderBarPlot } from "./bar/index.js";
import { createSeriesHighlight } from "./highlight.js";
import { createSeriesLoader } from "./loader.js";
import { renderLinePlot } from "./line/index.js";
import { createScrubber } from "./scrubber.js";
import { renderDotsPlot } from "./dots/index.js";
import { createSvgElement } from "./svg.js";
import { renderStackedPlot } from "./stacked/index.js";
import { getViewBoxHeight, VIEWBOX_WIDTH } from "./viewbox.js";

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
 * @param {Object} args
 * @param {SVGSVGElement} args.svg
 * @param {Readout} args.readout
 * @param {HTMLElement} args.menu
 * @param {HTMLElement[]} args.items
 * @param {HTMLElement} args.status
 * @param {Chart} args.chart
 * @param {() => ChartView} args.getView
 * @param {() => ChartScale} args.getScale
 * @param {() => TimeframeValue} args.getTimeframe
 */
export function createChartRenderer({
  svg,
  readout,
  menu,
  items,
  status,
  chart,
  getView,
  getScale,
  getTimeframe,
}) {
  const group = createSvgElement("g");
  const highlight = createSeriesHighlight(items, menu);
  const loadSeries = createSeriesLoader(chart);
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
      renderPlot(
        getView(),
        group,
        loadedSeries,
        height,
        highlight,
        getScale(),
      ),
      height,
    );
  }

  async function loadCurrent() {
    const id = (loadId += 1);
    svg.setAttribute("aria-busy", "true");

    try {
      const nextSeries = await loadSeries(getTimeframe());

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
    loadedSeries = [];
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

/** @typedef {import("./index.js").Chart} Chart */
/** @typedef {import("./index.js").LoadedSeries} LoadedSeries */
/** @typedef {import("./legend.js").Readout} Readout */
/** @typedef {import("./scale.js").ChartScale} ChartScale */
/** @typedef {import("./timeframes.js").TimeframeValue} TimeframeValue */
/** @typedef {import("./views.js").ChartView} ChartView */
/** @typedef {import("./highlight.js").SeriesHighlight} SeriesHighlight */
