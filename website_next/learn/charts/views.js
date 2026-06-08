import { createChartSetting } from "./setting.js";

const views = /** @type {const} */ ([
  { value: "line", label: "Line" },
  { value: "area", label: "Area" },
  { value: "stacked", label: "Stack" },
  { value: "bar", label: "Bars" },
  { value: "dots", label: "Dots" },
]);
const defaultView = "stacked";
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
