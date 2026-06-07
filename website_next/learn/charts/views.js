import { createRadioGroup } from "./radio.js";
import { createChartStorage } from "./storage.js";

const storage = createChartStorage("view");
const defaultView = "stacked";
const views = /** @type {const} */ ([
  { value: "line", label: "Line" },
  { value: "stacked", label: "Stack↑" },
  { value: "stacked-reversed", label: "Stack↓" },
  { value: "bar", label: "Bars↑" },
  { value: "bar-reversed", label: "Bars↓" },
  { value: "dots", label: "Dots" },
]);

/** @param {string} chartKey */
export function getDefaultView(chartKey) {
  const value = storage.get(chartKey);

  return views.find((view) => view.value === value)?.value ?? defaultView;
}

/**
 * @param {string} chartKey
 * @param {ChartView} view
 */
export function saveView(chartKey, view) {
  storage.set(chartKey, view);
}

/**
 * @param {ChartView} currentView
 * @param {(view: ChartView) => void} onChange
 */
export function createViewControl(currentView, onChange) {
  return createRadioGroup({
    legend: "View",
    options: views,
    currentValue: currentView,
    onChange,
  });
}

/** @typedef {(typeof views)[number]["value"]} ChartView */
