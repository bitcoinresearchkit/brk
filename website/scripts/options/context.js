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
 * @template {(arg: any, ...args: any[]) => any} F
 * @typedef {F extends (arg: any, ...args: infer P) => infer R ? (...args: P) => R : never} OmitFirstArg
 */

/** @typedef {ReturnType<typeof createContext>} PartialContext */

/**
 * Create a context object with all dependencies for building partial options
 * @param {Object} args
 * @param {BrkClient} args.brk
 */
export function createContext({ brk }) {
  return {
    colors,
    brk,

    /** @type {OmitFirstArg<typeof fromBlockCount>} */
    fromBlockCount: (pattern, title, color) =>
      fromBlockCount(colors, pattern, title, color),
    /** @type {OmitFirstArg<typeof fromBitcoin>} */
    fromBitcoin: (pattern, title, color) =>
      fromBitcoin(colors, pattern, title, color),
    /** @type {OmitFirstArg<typeof fromBlockSize>} */
    fromBlockSize: (pattern, title, color) =>
      fromBlockSize(colors, pattern, title, color),
    /** @type {OmitFirstArg<typeof fromSizePattern>} */
    fromSizePattern: (pattern, unit, title) =>
      fromSizePattern(colors, pattern, unit, title),
    /** @type {OmitFirstArg<typeof fromFullnessPattern>} */
    fromFullnessPattern: (pattern, unit, title) =>
      fromFullnessPattern(colors, pattern, unit, title),
    /** @type {OmitFirstArg<typeof fromDollarsPattern>} */
    fromDollarsPattern: (pattern, unit, title) =>
      fromDollarsPattern(colors, pattern, unit, title),
    /** @type {OmitFirstArg<typeof fromFeeRatePattern>} */
    fromFeeRatePattern: (pattern, unit, title) =>
      fromFeeRatePattern(colors, pattern, unit, title),
    /** @type {OmitFirstArg<typeof fromCoinbasePattern>} */
    fromCoinbasePattern: (pattern, title) =>
      fromCoinbasePattern(colors, pattern, title),
    /** @type {OmitFirstArg<typeof fromValuePattern>} */
    fromValuePattern: (pattern, title, sumColor, cumulativeColor) =>
      fromValuePattern(colors, pattern, title, sumColor, cumulativeColor),
    /** @type {OmitFirstArg<typeof fromBitcoinPatternWithUnit>} */
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
    /** @type {OmitFirstArg<typeof fromBlockCountWithUnit>} */
    fromBlockCountWithUnit: (pattern, unit, title, sumColor, cumulativeColor) =>
      fromBlockCountWithUnit(
        colors,
        pattern,
        unit,
        title,
        sumColor,
        cumulativeColor,
      ),
    /** @type {OmitFirstArg<typeof fromIntervalPattern>} */
    fromIntervalPattern: (pattern, unit, title, color) =>
      fromIntervalPattern(colors, pattern, unit, title, color),
    /** @type {fromSupplyPattern} */
    fromSupplyPattern: (pattern, title, color) =>
      fromSupplyPattern(pattern, title, color),
  };
}
