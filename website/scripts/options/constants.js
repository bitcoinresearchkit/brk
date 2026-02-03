/** Constant helpers for creating price lines and reference lines */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
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
 * @param {{ number?: number, name?: string } & Omit<(Parameters<typeof line>)[0], 'name' | 'metric'>} args
 */
export function priceLine(args) {
  return line({
    ...args,
    metric: getConstant(brk.metrics.constants, args.number || 0),
    name: args.name || `${args.number ?? 0}`,
    color: args.color ?? colors.gray,
    options: {
      lineStyle: args.style ?? 4,
      lastValueVisible: false,
      crosshairMarkerVisible: false,
      ...args.options,
    },
  });
}

/**
 * @param {{ numbers: number[] } & Omit<(Parameters<typeof priceLine>)[0], 'number'>} args
 */
export function priceLines(args) {
  return args.numbers.map((number) =>
    priceLine({
      ...args,
      number,
    }),
  );
}
