import { createChartSetting } from "./setting.js";

export const timeframes = /** @type {const} */ ({
  "1d": { index: "minute10", count: 144 },
  "1w": { index: "hour1", count: 168 },
  "1m": { index: "hour4", count: 186 },
  "1y": { index: "day1", count: 366 },
  "4y": { index: "day3", count: 488 },
  "8y": { index: "week1", count: 418 },
  all: { index: "week1" },
});
export const timeframeOptions = /** @type {const} */ ([
  { value: "1d", label: "1d" },
  { value: "1w", label: "1w" },
  { value: "1m", label: "1m" },
  { value: "1y", label: "1y" },
  { value: "4y", label: "4y" },
  { value: "8y", label: "8y" },
  { value: "all", label: "all" },
]);
const setting = createChartSetting({
  storageKey: "timeframe",
  legend: "Time",
  options: timeframeOptions,
  defaultValue: "all",
});

/** @param {string} chartKey */
export function getDefaultTimeframe(chartKey) {
  return setting.get(chartKey);
}

/**
 * @param {string} chartKey
 * @param {TimeframeValue} timeframe
 */
export function saveTimeframe(chartKey, timeframe) {
  setting.save(chartKey, timeframe);
}

/**
 * @param {TimeframeValue} currentTimeframe
 * @param {(timeframe: TimeframeValue) => void} onChange
 */
export function createTimeframeControl(currentTimeframe, onChange) {
  return setting.create(currentTimeframe, onChange);
}

/**
 * @param {TimeframeMetric} metric
 * @param {TimeframeValue} timeframe
 */
export function fetchTimeframe(metric, timeframe) {
  const config = timeframes[timeframe];
  const endpoint = metric.by[config.index];

  return "count" in config
    ? endpoint.last(config.count).fetch()
    : endpoint.fetch();
}
