import {
  fromBaseStatsPattern,
  fromStatsPattern,
  chartsFromFull,
  chartsFromSum,
  chartsFromValueFull,
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
    // Series helpers (return series arrays for a single chart)
    fromBaseStatsPattern: bind(fromBaseStatsPattern),
    fromStatsPattern: bind(fromStatsPattern),
    // Chart helpers (return chart trees for Sum/Distribution/Cumulative folders)
    chartsFromFull: bind(chartsFromFull),
    chartsFromSum: bind(chartsFromSum),
    chartsFromValueFull: bind(chartsFromValueFull),
  };
}
