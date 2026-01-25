import {
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
 * @template {(colors: Colors, ...args: any[]) => any} T
 * @param {T} fn
 * @returns {OmitFirstArg<T>}
 */
const bind = (fn) =>
  /** @type {any} */ (
    // @ts-ignore
    (...args) => fn(colors, ...args)
  );

/**
 * Create a context object with all dependencies for building partial options
 * @param {Object} args
 * @param {BrkClient} args.brk
 */
export function createContext({ brk }) {
  return {
    colors,
    brk,
    fromSizePattern: bind(fromSizePattern),
    fromFullnessPattern: bind(fromFullnessPattern),
    fromDollarsPattern: bind(fromDollarsPattern),
    fromFeeRatePattern: bind(fromFeeRatePattern),
    fromCoinbasePattern: bind(fromCoinbasePattern),
    fromValuePattern: bind(fromValuePattern),
    fromBitcoinPatternWithUnit: bind(fromBitcoinPatternWithUnit),
    fromBlockCountWithUnit: bind(fromBlockCountWithUnit),
    fromIntervalPattern: bind(fromIntervalPattern),
    fromSupplyPattern,
  };
}
