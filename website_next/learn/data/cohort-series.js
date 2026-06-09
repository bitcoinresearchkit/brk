import { colors } from "../../utils/colors.js";

const palette = [
  colors.red,
  colors.orange,
  colors.amber,
  colors.yellow,
  colors.avocado,
  colors.lime,
  colors.green,
  colors.emerald,
  colors.teal,
  colors.cyan,
  colors.sky,
  colors.blue,
  colors.indigo,
  colors.violet,
  colors.purple,
  colors.fuchsia,
  colors.pink,
  colors.rose,
];

/** @param {number} index */
function colorAt(index) {
  return palette[index % palette.length];
}

/** @param {readonly { label: string, color?: () => string, metric: ChartMetric }[]} items */
export function createCohortSeries(items) {
  return items.map(({ label, color, metric }, index) => ({
    label,
    color: color ?? colorAt(index),
    metric,
  }));
}

/**
 * @template {string} Key
 * @param {readonly (readonly [string, Key])[]} items
 * @param {(key: Key) => ChartMetric} createMetric
 */
export function createCohortSeriesFromKeys(items, createMetric) {
  return createCohortSeries(
    items.map(([label, key]) => ({
      label,
      metric: createMetric(key),
    })),
  );
}
