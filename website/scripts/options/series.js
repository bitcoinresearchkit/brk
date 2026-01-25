/** Series helpers for creating chart series blueprints */

import { Unit } from "../utils/units.js";

/**
 * Create a Line series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {LineStyle} [args.style]
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedLineSeriesBlueprint}
 */
export function line({
  metric,
  name,
  key,
  style,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    metric,
    title: name,
    key,
    color,
    unit,
    defaultActive,
    options: {
      lineStyle: style,
      ...options,
    },
  };
}

/**
 * Create a Dots series (line with only point markers visible)
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedDotsSeriesBlueprint}
 */
export function dots({
  metric,
  name,
  key,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Dots"),
    metric,
    title: name,
    key,
    color,
    unit,
    defaultActive,
    options,
  };
}

/**
 * Create a Candlestick series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {[Color, Color]} [args.colors] - [upColor, downColor] for legend
 * @param {boolean} [args.defaultActive]
 * @param {CandlestickSeriesPartialOptions} [args.options]
 * @returns {FetchedCandlestickSeriesBlueprint}
 */
export function candlestick({
  metric,
  name,
  key,
  colors,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Candlestick"),
    metric,
    title: name,
    key,
    colors,
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
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {number | undefined} [args.base]
 * @param {BaselineSeriesPartialOptions} [args.options]
 * @returns {FetchedBaselineSeriesBlueprint}
 */
export function baseline({
  metric,
  name,
  key,
  color,
  defaultActive,
  unit,
  base,
  options,
}) {
  const isTuple = Array.isArray(color);
  return {
    type: /** @type {const} */ ("Baseline"),
    metric,
    title: name,
    key,
    color: isTuple ? undefined : color,
    colors: isTuple ? color : undefined,
    unit,
    defaultActive,
    options: {
      baseValue: {
        price: base,
      },
      ...options,
    },
  };
}

/**
 * Create a Histogram series
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {HistogramSeriesPartialOptions} [args.options]
 * @returns {FetchedHistogramSeriesBlueprint}
 */
export function histogram({
  metric,
  name,
  key,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Histogram"),
    metric,
    title: name,
    key,
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
      title: `${title} cumulative`,
      color: colors.stat.cumulative,
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
  const { stat } = colors;
  return [
    { metric: pattern.base, title, color: color ?? colors.default },
    {
      metric: pattern.average,
      title: `${title} avg`,
      color: stat.avg,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: stat.max,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: stat.min,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: stat.median,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: stat.pct75,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: stat.pct25,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: stat.pct90,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: stat.pct10,
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
  const { stat } = colors;
  return [
    { metric: pattern.sum, title, color: color ?? colors.default },
    {
      metric: pattern.average,
      title: `${title} avg`,
      color: stat.avg,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`,
      color: stat.cumulative,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`,
      color: stat.max,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`,
      color: stat.min,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`,
      color: stat.median,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`,
      color: stat.pct75,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`,
      color: stat.pct25,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`,
      color: stat.pct90,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`,
      color: stat.pct10,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a SizePattern ({ average, sum, cumulative, min, max, percentiles })
 * @param {Colors} colors
 * @param {AnyStatsPattern} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSizePattern(colors, pattern, unit, title = "") {
  const { stat } = colors;
  return [
    { metric: pattern.average, title: `${title} avg`.trim(), unit },
    {
      metric: pattern.sum,
      title: `${title} sum`.trim(),
      color: stat.sum,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`.trim(),
      color: stat.cumulative,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a FullnessPattern ({ base, average, sum, cumulative, min, max, percentiles })
 * @param {Colors} colors
 * @param {FullnessPattern<any>} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFullnessPattern(colors, pattern, unit, title = "") {
  const { stat } = colors;
  return [
    { metric: pattern.base, title: title || "base", unit },
    {
      metric: pattern.average,
      title: `${title} avg`.trim(),
      color: stat.avg,
      unit,
    },
    {
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a DollarsPattern ({ base, sum, cumulative, average, min, max, percentiles })
 * @param {Colors} colors
 * @param {DollarsPattern<any>} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromDollarsPattern(colors, pattern, unit, title = "") {
  const { stat } = colors;
  return [
    { metric: pattern.base, title: title || "base", unit },
    {
      metric: pattern.sum,
      title: `${title} sum`.trim(),
      color: stat.sum,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`.trim(),
      color: stat.cumulative,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.average,
      title: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a FeeRatePattern ({ average, min, max, percentiles })
 * @param {Colors} colors
 * @param {FeeRatePattern<any>} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFeeRatePattern(colors, pattern, unit, title = "") {
  const { stat } = colors;
  return [
    {
      type: "Dots",
      metric: pattern.average,
      title: `${title} avg`.trim(),
      unit,
    },
    {
      type: "Dots",
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    },
    {
      type: "Dots",
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a CoinbasePattern ({ sats, bitcoin, dollars } each as FullnessPattern)
 * @param {Colors} colors
 * @param {CoinbasePattern} pattern
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromCoinbasePattern(colors, pattern, title) {
  return [
    ...fromFullnessPattern(colors, pattern.bitcoin, Unit.btc, title),
    ...fromFullnessPattern(colors, pattern.sats, Unit.sats, title),
    ...fromFullnessPattern(colors, pattern.dollars, Unit.usd, title),
  ];
}

/**
 * Create series from a ValuePattern ({ sats, bitcoin, dollars } each as BlockCountPattern with sum + cumulative)
 * @param {Colors} colors
 * @param {ValuePattern} pattern
 * @param {string} [title]
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromValuePattern(
  colors,
  pattern,
  title = "",
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.bitcoin.sum,
      title: title || "sum",
      color: sumColor,
      unit: Unit.btc,
    },
    {
      metric: pattern.bitcoin.cumulative,
      title: `${title} cumulative`.trim(),
      color: cumulativeColor ?? colors.stat.cumulative,
      unit: Unit.btc,
      defaultActive: false,
    },
    {
      metric: pattern.sats.sum,
      title: title || "sum",
      color: sumColor,
      unit: Unit.sats,
    },
    {
      metric: pattern.sats.cumulative,
      title: `${title} cumulative`.trim(),
      color: cumulativeColor ?? colors.stat.cumulative,
      unit: Unit.sats,
      defaultActive: false,
    },
    {
      metric: pattern.dollars.sum,
      title: title || "sum",
      color: sumColor,
      unit: Unit.usd,
    },
    {
      metric: pattern.dollars.cumulative,
      title: `${title} cumulative`.trim(),
      color: cumulativeColor ?? colors.stat.cumulative,
      unit: Unit.usd,
      defaultActive: false,
    },
  ];
}

/**
 * Create sum/cumulative series from a BitcoinPattern ({ sum, cumulative }) with explicit unit and colors
 * @param {Colors} colors
 * @param {{ sum: AnyMetricPattern, cumulative: AnyMetricPattern }} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoinPatternWithUnit(
  colors,
  pattern,
  unit,
  title = "",
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.sum,
      title: `${title} sum`.trim(),
      color: sumColor,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`.trim(),
      color: cumulativeColor ?? colors.stat.cumulative,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create sum/cumulative series from a BlockCountPattern with explicit unit and colors
 * @param {Colors} colors
 * @param {BlockCountPattern<any>} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @param {Color} [sumColor]
 * @param {Color} [cumulativeColor]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBlockCountWithUnit(
  colors,
  pattern,
  unit,
  title = "",
  sumColor,
  cumulativeColor,
) {
  return [
    {
      metric: pattern.sum,
      title: `${title} sum`.trim(),
      color: sumColor,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: `${title} cumulative`.trim(),
      color: cumulativeColor ?? colors.stat.cumulative,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from an IntervalPattern (base + average/min/max/median/percentiles, no sum/cumulative)
 * @param {Colors} colors
 * @param {IntervalPattern} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromIntervalPattern(colors, pattern, unit, title = "", color) {
  const { stat } = colors;
  return [
    { metric: pattern.base, title: title ?? "base", color, unit },
    {
      metric: pattern.average,
      title: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    },
  ];
}

/**
 * Create series from a SupplyPattern (sats/bitcoin/dollars, no sum/cumulative)
 * @param {SupplyPattern} pattern
 * @param {string} title
 * @param {Color} [color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSupplyPattern(pattern, title, color) {
  return [
    {
      metric: pattern.bitcoin,
      title,
      color,
      unit: Unit.btc,
    },
    {
      metric: pattern.sats,
      title,
      color,
      unit: Unit.sats,
    },
    {
      metric: pattern.dollars,
      title,
      color,
      unit: Unit.usd,
    },
  ];
}
