/** Series helpers for creating chart series blueprints */

import { Unit } from "../utils/units.js";

/**
 * Create a Line series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedLineSeriesBlueprint}
 */
export function line({ metric, name, color, defaultActive, unit, options }) {
  return {
    metric,
    title: name,
    color,
    unit,
    defaultActive,
    options,
  };
}

/**
 * Create a Baseline series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {BaselineSeriesPartialOptions} [args.options]
 * @returns {FetchedBaselineSeriesBlueprint}
 */
export function baseline({
  metric,
  name,
  color,
  defaultActive,
  unit,
  options,
}) {
  const isTuple = Array.isArray(color);
  return {
    type: /** @type {const} */ ("Baseline"),
    metric,
    title: name,
    color: isTuple ? undefined : color,
    colors: isTuple ? color : undefined,
    unit,
    defaultActive,
    options,
  };
}

/**
 * Create a Histogram series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {HistogramSeriesPartialOptions} [args.options]
 * @returns {FetchedHistogramSeriesBlueprint}
 */
export function histogram({
  metric,
  name,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Histogram"),
    metric,
    title: name,
    color,
    unit,
    defaultActive,
    options,
  };
}

/**
 * Create series from a BlockCountPattern ({ base, sum, cumulative })
 * @param {Colors} colors
 * @param {BlockCountPattern<any>} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockCount(colors, pattern, title, color) {
  return [
    { metric: pattern.sum, title, color: color ?? colors.default },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a FullnessPattern ({ base, sum, cumulative, average, min, max, percentiles })
 * @param {Colors} colors
 * @param {FullnessPattern<any>} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoin(colors, pattern, title, color) {
  return [
    { metric: pattern.base, title, color: color ?? colors.default },
    { metric: pattern.average, title: "Average", defaultActive: false },
    {
      metric: pattern.sum,
      title: `${title} (sum)`,
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: "Max",
      color: colors.pink,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: "Min",
      color: colors.green,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: "Median",
      color: colors.amber,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: "pct75",
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: "pct25",
      color: colors.yellow,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: "pct90",
      color: colors.rose,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: "pct10",
      color: colors.lime,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a SizePattern ({ sum, cumulative, average, min, max, percentiles })
 * @param {Colors} colors
 * @param {AnyStatsPattern} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockSize(colors, pattern, title, color) {
  return [
    { metric: pattern.sum, title, color: color ?? colors.default },
    { metric: pattern.average, title: "Average", defaultActive: false },
    {
      metric: pattern.cumulative,
      title: `${title} (cum.)`,
      color: colors.cyan,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: "Max",
      color: colors.pink,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: "Min",
      color: colors.green,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: "Median",
      color: colors.amber,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: "pct75",
      color: colors.red,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: "pct25",
      color: colors.yellow,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: "pct90",
      color: colors.rose,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: "pct10",
      color: colors.lime,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a SizePattern ({ average, sum, cumulative, min, max, percentiles })
 * @param {Colors} colors
 * @param {AnyStatsPattern} pattern
 * @param {string} title
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSizePattern(colors, pattern, title, unit) {
  return [
    { metric: pattern.average, title: `${title} avg`, unit },
    {
      metric: pattern.sum,
      title: `${title} sum`,
      color: colors.blue,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`,
      color: colors.indigo,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: colors.purple,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: colors.violet,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a FullnessPattern ({ base, average, sum, cumulative, min, max, percentiles })
 * @param {Colors} colors
 * @param {FullnessPattern<any>} pattern
 * @param {string} title
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFullnessPattern(colors, pattern, title, unit) {
  return [
    { metric: pattern.base, title, unit },
    {
      metric: pattern.average,
      title: `${title} avg`,
      color: colors.purple,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.sum,
      title: `${title} sum`,
      color: colors.blue,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`,
      color: colors.indigo,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: colors.violet,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: colors.amber,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a FeeRatePattern ({ average, min, max, percentiles })
 * @param {Colors} colors
 * @param {FeeRatePattern<any>} pattern
 * @param {string} title
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFeeRatePattern(colors, pattern, title, unit) {
  return [
    { metric: pattern.average, title: `${title} avg`, unit },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: colors.purple,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: colors.violet,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a CoinbasePattern ({ sats, bitcoin, dollars } each as FullnessPattern)
 * @param {Colors} colors
 * @param {CoinbasePattern} pattern
 * @param {string} title
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromCoinbasePattern(colors, pattern, title) {
  return [
    ...fromFullnessPattern(colors, pattern.sats, title, Unit.sats),
    ...fromFullnessPattern(colors, pattern.bitcoin, title, Unit.btc),
    ...fromFullnessPattern(colors, pattern.dollars, title, Unit.usd),
  ];
}

/**
 * Create series from a ValuePattern ({ sats, bitcoin, dollars } each as BlockCountPattern with sum + cumulative)
 * @param {Colors} colors
 * @param {ValuePattern} pattern
 * @param {string} title
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromValuePattern(
  colors,
  pattern,
  title,
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.sats.sum,
      title,
      color: sumColor,
      unit: Unit.sats,
    },
    {
      metric: pattern.sats.cumulative,
      title: `${title} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit: Unit.sats,
      defaultActive: false,
    },
    {
      metric: pattern.bitcoin.sum,
      title,
      color: sumColor,
      unit: Unit.btc,
    },
    {
      metric: pattern.bitcoin.cumulative,
      title: `${title} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit: Unit.btc,
      defaultActive: false,
    },
    {
      metric: pattern.dollars.sum,
      title,
      color: sumColor,
      unit: Unit.usd,
    },
    {
      metric: pattern.dollars.cumulative,
      title: `${title} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit: Unit.usd,
      defaultActive: false,
    },
  ];
}

/**
 * Create sum/cumulative series from a BitcoinPattern ({ sum, cumulative }) with explicit unit and colors
 * @param {Colors} colors
 * @param {{ sum: AnyMetricPattern, cumulative: AnyMetricPattern }} pattern
 * @param {string} title
 * @param {Unit} unit
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoinPatternWithUnit(
  colors,
  pattern,
  title,
  unit,
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.sum,
      title: `${title} sum`,
      color: sumColor,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create sum/cumulative series from a BlockCountPattern with explicit unit and colors
 * @param {Colors} colors
 * @param {BlockCountPattern<any>} pattern
 * @param {string} title
 * @param {Unit} unit
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockCountWithUnit(
  colors,
  pattern,
  title,
  unit,
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.sum,
      title: `${title} sum`,
      color: sumColor,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from an IntervalPattern (base + average/min/max/median/percentiles, no sum/cumulative)
 * @param {Colors} colors
 * @param {IntervalPattern} pattern
 * @param {string} title
 * @param {Unit} unit
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromIntervalPattern(colors, pattern, title, unit, color) {
  return [
    { metric: pattern.base, title, color, unit },
    {
      metric: pattern.average,
      title: `${title} avg`,
      color: colors.purple,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: colors.violet,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: colors.amber,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a SupplyPattern (sats/bitcoin/dollars, no sum/cumulative)
 * @param {Colors} colors
 * @param {SupplyPattern} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSupplyPattern(colors, pattern, title, color) {
  return [
    {
      metric: pattern.sats,
      title,
      color: color ?? colors.default,
      unit: Unit.sats,
    },
    {
      metric: pattern.bitcoin,
      title,
      color: color ?? colors.default,
      unit: Unit.btc,
    },
    {
      metric: pattern.dollars,
      title,
      color: color ?? colors.default,
      unit: Unit.usd,
    },
  ];
}
