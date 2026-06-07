import { createRadioGroup } from "./radio.js";
import { createChartStorage } from "./storage.js";

const storage = createChartStorage("scale");
const defaultScale = "linear";
const scales = /** @type {const} */ ([
  { value: "linear", label: "Lin" },
  { value: "log", label: "Log" },
]);

/**
 * @param {string} chartKey
 * @param {ChartScale} [fallback]
 */
export function getDefaultScale(chartKey, fallback = defaultScale) {
  const value = storage.get(chartKey);

  return scales.find((scale) => scale.value === value)?.value ?? fallback;
}

/**
 * @param {string} chartKey
 * @param {ChartScale} scale
 */
export function saveScale(chartKey, scale) {
  storage.set(chartKey, scale);
}

/**
 * @param {ChartScale} currentScale
 * @param {(scale: ChartScale) => void} onChange
 */
export function createScaleControl(currentScale, onChange) {
  return createRadioGroup({
    legend: "Scale",
    options: scales,
    currentValue: currentScale,
    onChange,
  });
}

/**
 * @param {number} value
 * @param {ScaleBounds} bounds
 * @param {number} height
 * @param {ChartScale} scale
 */
export function scaleY(value, bounds, height, scale) {
  if (bounds.max === bounds.min) return height / 2;

  if (scale === "log") {
    if (bounds.max <= bounds.minPositive) {
      return value > 0 ? height / 2 : height;
    }

    const nextValue = Math.max(value, bounds.minPositive);
    return (
      height -
      ((Math.log10(nextValue) - Math.log10(bounds.minPositive)) /
        (Math.log10(bounds.max) - Math.log10(bounds.minPositive))) *
        height
    );
  }

  return height - ((value - bounds.min) / (bounds.max - bounds.min)) * height;
}

/**
 * @typedef {Object} ScaleBounds
 * @property {number} min
 * @property {number} max
 * @property {number} minPositive
 */

/** @typedef {(typeof scales)[number]["value"]} ChartScale */
