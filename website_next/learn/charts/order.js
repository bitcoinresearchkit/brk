import { createChartSetting } from "./setting.js";

const orders = /** @type {const} */ ([
  { value: "ascending", label: "Asc" },
  { value: "descending", label: "Dsc" },
]);
const defaultOrder = "ascending";
const setting = createChartSetting({
  storageKey: "order",
  legend: "Order",
  options: orders,
  defaultValue: defaultOrder,
});

/** @param {string} chartKey */
export function getDefaultOrder(chartKey) {
  return setting.get(chartKey);
}

/**
 * @param {string} chartKey
 * @param {ChartOrder} order
 */
export function saveOrder(chartKey, order) {
  setting.save(chartKey, order);
}

/**
 * @param {ChartOrder} currentOrder
 * @param {(order: ChartOrder) => void} onChange
 */
export function createOrderControl(currentOrder, onChange) {
  return setting.create(currentOrder, onChange);
}

/**
 * @param {number[]} indexes
 * @param {ChartOrder} order
 */
export function orderIndexes(indexes, order) {
  const orderedIndexes = [...indexes];

  if (order === "descending") orderedIndexes.reverse();

  return orderedIndexes;
}

/**
 * @param {number} length
 * @param {ChartOrder} order
 */
export function createOrderedIndexes(length, order) {
  return orderIndexes(
    Array.from({ length }, (_, index) => index),
    order,
  );
}

/** @typedef {(typeof orders)[number]["value"]} ChartOrder */
