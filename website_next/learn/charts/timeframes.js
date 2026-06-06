import { createRadioGroup } from "./radio.js";
import { createChartStorage } from "./storage.js";

const storage = createChartStorage("timeframe");
/** @type {TimeframeValue} */
const defaultTimeframe = "all";
/** @type {Record<TimeframeValue, TimeframeConfig>} */
const timeframes = {
  "1d": { index: "minute10", count: 144 },
  "1w": { index: "hour1", count: 168 },
  "1m": { index: "hour4", count: 186 },
  "1y": { index: "day1", count: 366 },
  "4y": { index: "day3", count: 488 },
  "8y": { index: "week1", count: 418 },
  all: { index: "week1" },
};
/** @type {{ value: TimeframeValue, label: string }[]} */
const options = [
  { value: "1d", label: "1d" },
  { value: "1w", label: "1w" },
  { value: "1m", label: "1m" },
  { value: "1y", label: "1y" },
  { value: "4y", label: "4y" },
  { value: "8y", label: "8y" },
  { value: "all", label: "all" },
];

/** @param {string} chartKey */
export function getDefaultTimeframe(chartKey) {
  const value = storage.get(chartKey);

  return (
    options.find((timeframe) => timeframe.value === value)?.value ??
    defaultTimeframe
  );
}

/**
 * @param {string} chartKey
 * @param {TimeframeValue} timeframe
 */
export function saveTimeframe(chartKey, timeframe) {
  storage.set(chartKey, timeframe);
}

/**
 * @param {TimeframeValue} currentTimeframe
 * @param {(timeframe: TimeframeValue) => void} onChange
 */
export function createTimeframeControl(currentTimeframe, onChange) {
  return createRadioGroup({
    legend: "Time",
    options,
    currentValue: currentTimeframe,
    onChange,
  });
}

/**
 * @param {TimeframeMetric} metric
 * @param {TimeframeValue} timeframe
 */
export function fetchTimeframe(metric, timeframe) {
  const { count, index } = timeframes[timeframe];
  const endpoint = metric.by[index];

  return count ? endpoint.last(count).fetch() : endpoint.fetch();
}

/** @typedef {"1d" | "1w" | "1m" | "1y" | "4y" | "8y" | "all"} TimeframeValue */
/** @typedef {"minute10" | "hour1" | "hour4" | "day1" | "day3" | "week1"} TimeframeIndex */

/**
 * @typedef {Object} TimeframeConfig
 * @property {TimeframeIndex} index
 * @property {number} [count]
 */

/**
 * @typedef {Object} TimeframeEndpoint
 * @property {() => Promise<import("./index.js").ChartResult>} fetch
 * @property {(count: number) => { fetch: () => Promise<import("./index.js").ChartResult> }} last
 */

/**
 * @typedef {Object} TimeframeMetric
 * @property {Record<TimeframeIndex, TimeframeEndpoint>} by
 */
