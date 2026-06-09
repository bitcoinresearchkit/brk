import { brk } from "../../utils/client.js";
import { fetchTimeframe } from "./timeframes.js";

/**
 * @param {ChartResult} result
 * @returns {ChartEntry[]}
 */
function createEntries(result) {
  /** @type {ChartEntry[]} */
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
 */
function loadSeries(chart, timeframe) {
  return Promise.all(
    chart.series.map(async (item) => ({
      series: item,
      color: item.color(),
      entries: createEntries(await fetchTimeframe(item.metric(brk), timeframe)),
    })),
  );
}

/** @param {Chart} chart */
export function createSeriesLoader(chart) {
  /** @type {TimeframeValue | undefined} */
  let cachedTimeframe;
  /** @type {Promise<LoadedSeries[]> | undefined} */
  let cachedPromise;

  /** @param {TimeframeValue} timeframe */
  return function loadTimeframe(timeframe) {
    if (timeframe !== cachedTimeframe || !cachedPromise) {
      cachedTimeframe = timeframe;
      cachedPromise = loadSeries(chart, timeframe).catch((error) => {
        if (timeframe === cachedTimeframe) cachedPromise = undefined;
        throw error;
      });
    }

    return cachedPromise;
  };
}
