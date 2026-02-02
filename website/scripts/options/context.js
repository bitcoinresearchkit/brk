import {
  fromBaseStatsPattern,
  fromStatsPattern,
  fromSupplyPattern,
  chartsFromFull,
  chartsFromSum,
  chartsFromCount,
  chartsFromValue,
  chartsFromValueFull,
} from "./series.js";
import { colors } from "../chart/colors.js";
import { Unit } from "../utils/units.js";

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
 * Create distribution series for btc/sats/usd from a value pattern with stats (average + percentiles)
 * @param {FullValuePattern | SumValuePattern} source
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
const distributionBtcSatsUsd = (source) => [
  ...fromStatsPattern(colors, { pattern: source.bitcoin, unit: Unit.btc }),
  ...fromStatsPattern(colors, { pattern: source.sats, unit: Unit.sats }),
  ...fromStatsPattern(colors, { pattern: source.dollars, unit: Unit.usd }),
];


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
    fromSupplyPattern,
    distributionBtcSatsUsd,
    // Chart helpers (return chart trees for Sum/Distribution/Cumulative folders)
    chartsFromFull: bind(chartsFromFull),
    chartsFromSum: bind(chartsFromSum),
    chartsFromCount,
    chartsFromValue,
    chartsFromValueFull: bind(chartsFromValueFull),
  };
}
