import {
  fromSumStatsPattern,
  fromBaseStatsPattern,
  fromFullStatsPattern,
  fromStatsPattern,
  fromCoinbasePattern,
  fromValuePattern,
  fromBitcoinPatternWithUnit,
  fromCountPattern,
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
    fromSumStatsPattern: bind(fromSumStatsPattern),
    fromBaseStatsPattern: bind(fromBaseStatsPattern),
    fromFullStatsPattern: bind(fromFullStatsPattern),
    fromStatsPattern: bind(fromStatsPattern),
    fromCoinbasePattern: bind(fromCoinbasePattern),
    fromValuePattern,
    fromBitcoinPatternWithUnit,
    fromCountPattern,
    fromSupplyPattern,
  };
}
