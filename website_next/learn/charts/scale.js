import { createChartSetting } from "./setting.js";

export const scales = /** @type {const} */ ([
  { value: "linear", label: "Lin" },
  { value: "log", label: "Log" },
]);
const defaultScale = "linear";
const setting = createChartSetting({
  storageKey: "scale",
  legend: "Scale",
  options: scales,
  defaultValue: defaultScale,
});

/**
 * @param {string} chartKey
 * @param {ChartScale} [fallback]
 */
export function getDefaultScale(chartKey, fallback = defaultScale) {
  return setting.get(chartKey, fallback);
}

/**
 * @param {string} chartKey
 * @param {ChartScale} scale
 */
export function saveScale(chartKey, scale) {
  setting.save(chartKey, scale);
}

/**
 * @param {ChartScale} currentScale
 * @param {(scale: ChartScale) => void} onChange
 */
export function createScaleControl(currentScale, onChange) {
  return setting.create(currentScale, onChange);
}

export function createBounds() {
  return {
    min: Infinity,
    max: -Infinity,
    minPositive: Infinity,
  };
}

/**
 * @param {ScaleBounds} bounds
 * @param {number} value
 */
export function includeBoundValue(bounds, value) {
  bounds.min = Math.min(bounds.min, value);
  bounds.max = Math.max(bounds.max, value);
  if (value > 0) bounds.minPositive = Math.min(bounds.minPositive, value);
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
