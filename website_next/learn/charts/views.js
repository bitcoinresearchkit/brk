import { createChartSetting } from "./setting.js";

export const viewTypes = /** @type {const} */ ({
  line: "line",
  stacked: "stacked",
  stackedReversed: "stacked-reversed",
  bar: "bar",
  barReversed: "bar-reversed",
  dots: "dots",
});
const views = /** @type {const} */ ([
  { value: viewTypes.line, label: "Line" },
  { value: viewTypes.stacked, label: "Stack↑" },
  { value: viewTypes.stackedReversed, label: "Stack↓" },
  { value: viewTypes.bar, label: "Bars↑" },
  { value: viewTypes.barReversed, label: "Bars↓" },
  { value: viewTypes.dots, label: "Dots" },
]);
const defaultView = viewTypes.stacked;
const setting = createChartSetting({
  storageKey: "view",
  legend: "View",
  options: views,
  defaultValue: defaultView,
});

/**
 * @param {string} chartKey
 * @param {ChartView} [fallback]
 */
export function getDefaultView(chartKey, fallback = defaultView) {
  return setting.get(chartKey, fallback);
}

/**
 * @param {string} chartKey
 * @param {ChartView} view
 */
export function saveView(chartKey, view) {
  setting.save(chartKey, view);
}

/**
 * @param {ChartView} currentView
 * @param {(view: ChartView) => void} onChange
 */
export function createViewControl(currentView, onChange) {
  return setting.create(currentView, onChange);
}

/** @typedef {(typeof views)[number]["value"]} ChartView */
