import {
  s,
  fromBlockCount,
  fromBitcoin,
  fromBlockSize,
  fromSizePattern,
  fromFullnessPattern,
  fromFeeRatePattern,
  fromCoinbasePattern,
  fromValuePattern,
  fromBlockCountWithUnit,
  fromIntervalPattern,
} from "./series.js";
import { createPriceLine, createPriceLines, line } from "./constants.js";

/**
 * Create a context object with all dependencies for building partial options
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {BrkClient} args.brk
 * @returns {PartialContext}
 */
export function createContext({ colors, brk }) {
  const constants = brk.tree.constants;

  return {
    colors,
    brk,

    // Series helpers
    s,
    fromBlockCount: (pattern, title, color) =>
      fromBlockCount(colors, pattern, title, color),
    fromBitcoin: (pattern, title, color) =>
      fromBitcoin(colors, pattern, title, color),
    fromBlockSize: (pattern, title, color) =>
      fromBlockSize(colors, pattern, title, color),
    fromSizePattern: (pattern, title, unit) =>
      fromSizePattern(colors, pattern, title, unit),
    fromFullnessPattern: (pattern, title, unit) =>
      fromFullnessPattern(colors, pattern, title, unit),
    fromFeeRatePattern: (pattern, title, unit) =>
      fromFeeRatePattern(colors, pattern, title, unit),
    fromCoinbasePattern: (pattern, title) =>
      fromCoinbasePattern(colors, pattern, title),
    fromValuePattern: (pattern, title, sumColor, cumulativeColor) =>
      fromValuePattern(colors, pattern, title, sumColor, cumulativeColor),
    fromBlockCountWithUnit: (pattern, title, unit, sumColor, cumulativeColor) =>
      fromBlockCountWithUnit(colors, pattern, title, unit, sumColor, cumulativeColor),
    fromIntervalPattern: (pattern, title, unit, color) =>
      fromIntervalPattern(colors, pattern, title, unit, color),

    createPriceLine: (args) => createPriceLine({ constants, colors, ...args }),
    createPriceLines: (args) =>
      createPriceLines({ constants, colors, ...args }),
    line: (args) => line({ colors, ...args }),
  };
}
