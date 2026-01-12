/** Constant helpers for creating price lines and reference lines */

import { line } from "./series.js";

/**
 * Get constant pattern by number dynamically from tree
 * Examples: 0 → constant0, 38.2 → constant382, -1 → constantMinus1
 * @param {BrkClient["metrics"]["constants"]} constants
 * @param {number} num
 * @returns {AnyMetricPattern}
 */
export function getConstant(constants, num) {
  const key =
    num >= 0
      ? `constant${String(num).replace(".", "")}`
      : `constantMinus${Math.abs(num)}`;
  const constant = /** @type {AnyMetricPattern | undefined} */ (
    /** @type {Record<string, AnyMetricPattern>} */ (constants)[key]
  );
  if (!constant) throw new Error(`Unknown constant: ${num} (key: ${key})`);
  return constant;
}

/**
 * Create a price line series (horizontal reference line)
 * @param {Object} args
 * @param {BrkClient["metrics"]["constants"]} args.constants
 * @param {Colors} args.colors
 * @param {number} [args.number]
 * @param {string} [args.name]
 * @param {boolean} [args.defaultActive]
 * @param {number} [args.lineStyle]
 * @param {Color} [args.color]
 * @param {Unit} args.unit
 * @returns {FetchedLineSeriesBlueprint}
 */
export function createPriceLine({
  constants,
  colors,
  number = 0,
  unit,
  defaultActive,
  color,
  name,
  lineStyle,
}) {
  return {
    metric: getConstant(constants, number),
    title: name ?? `${number}`,
    unit,
    defaultActive,
    color: color ?? colors.gray,
    options: {
      lineStyle: lineStyle ?? 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  };
}

/**
 * Create multiple price lines from an array of numbers
 * @param {Object} args
 * @param {BrkClient["metrics"]["constants"]} args.constants
 * @param {Colors} args.colors
 * @param {number[]} args.numbers
 * @param {Unit} args.unit
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function createPriceLines({ constants, colors, numbers, unit }) {
  return numbers.map((number) => ({
    metric: getConstant(constants, number),
    title: `${number}`,
    unit,
    defaultActive: !number,
    color: colors.gray,
    options: {
      lineStyle: 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  }));
}

/**
 * Create a constant line series
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {AnyMetricPattern} args.constant
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @param {number} [args.lineStyle]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint}
 */
export function constantLine({
  colors,
  constant,
  name,
  unit,
  color,
  lineStyle,
  defaultActive,
}) {
  return line({
    metric: constant,
    name,
    unit,
    defaultActive,
    color: color ?? colors.gray,
    options: {
      lineStyle: lineStyle ?? 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
    },
  });
}
