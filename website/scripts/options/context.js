import {
  fromBlockCount,
  fromBitcoin,
  fromBlockSize,
  fromSizePattern,
  fromFullnessPattern,
  fromDollarsPattern,
  fromFeeRatePattern,
  fromCoinbasePattern,
  fromValuePattern,
  fromBitcoinPatternWithUnit,
  fromBlockCountWithUnit,
  fromIntervalPattern,
  fromSupplyPattern,
} from "./series.js";
import { colors } from "../chart/colors.js";

/**
 * Create a context object with all dependencies for building partial options
 * @param {Object} args
 * @param {BrkClient} args.brk
 * @returns {PartialContext}
 */
export function createContext({ brk }) {
  return {
    colors,
    brk,

    fromBlockCount: (pattern, title, color) =>
      fromBlockCount(colors, pattern, title, color),
    fromBitcoin: (pattern, title, color) =>
      fromBitcoin(colors, pattern, title, color),
    fromBlockSize: (pattern, title, color) =>
      fromBlockSize(colors, pattern, title, color),
    fromSizePattern: (pattern, unit, title) =>
      fromSizePattern(colors, pattern, unit, title),
    fromFullnessPattern: (pattern, unit, title) =>
      fromFullnessPattern(colors, pattern, unit, title),
    fromDollarsPattern: (pattern, unit, title) =>
      fromDollarsPattern(colors, pattern, unit, title),
    fromFeeRatePattern: (pattern, unit, title) =>
      fromFeeRatePattern(colors, pattern, unit, title),
    fromCoinbasePattern: (pattern, title) =>
      fromCoinbasePattern(colors, pattern, title),
    fromValuePattern: (pattern, title, sumColor, cumulativeColor) =>
      fromValuePattern(colors, pattern, title, sumColor, cumulativeColor),
    fromBitcoinPatternWithUnit: (
      pattern,
      title,
      unit,
      sumColor,
      cumulativeColor,
    ) =>
      fromBitcoinPatternWithUnit(
        colors,
        pattern,
        title,
        unit,
        sumColor,
        cumulativeColor,
      ),
    fromBlockCountWithUnit: (pattern, unit, title, sumColor, cumulativeColor) =>
      fromBlockCountWithUnit(
        colors,
        pattern,
        unit,
        title,
        sumColor,
        cumulativeColor,
      ),
    fromIntervalPattern: (pattern, unit, title, color) =>
      fromIntervalPattern(colors, pattern, unit, title, color),
    fromSupplyPattern: (pattern, title, color) =>
      fromSupplyPattern(colors, pattern, title, color),
  };
}
