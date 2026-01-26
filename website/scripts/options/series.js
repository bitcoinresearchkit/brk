/** Series helpers for creating chart series blueprints */

import { Unit } from "../utils/units.js";

// ============================================================================
// Price helper for top pane (auto-expands to USD + sats)
// ============================================================================

/**
 * Create a price series for the top pane (auto-expands to USD + sats versions)
 * @param {Object} args
 * @param {AnyPricePattern} args.metric - Price pattern with dollars and sats
 * @param {string} args.name
 * @param {string} [args.key]
 * @param {LineStyle} [args.style]
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedPriceSeriesBlueprint}
 */
export function price({
  metric,
  name,
  key,
  style,
  color,
  defaultActive,
  options,
}) {
  return {
    metric,
    title: name,
    key,
    color,
    defaultActive,
    options: {
      lineStyle: style,
      ...options,
    },
  };
}

// ============================================================================
// Shared percentile helper
// ============================================================================

/**
 * Create percentile series (max/min/median/pct75/pct25/pct90/pct10) from any stats pattern
 * Works with FullnessPattern, FeeRatePattern, AnyStatsPattern, DollarsPattern, etc.
 * @param {Colors} colors
 * @param {FullnessPattern<any> | FeeRatePattern<any> | AnyStatsPattern | DollarsPattern<any>} pattern
 * @param {Unit} unit
 * @param {string} title
 * @param {{ type?: "Dots" }} [options]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function percentileSeries(colors, pattern, unit, title, { type } = {}) {
  const { stat } = colors;
  const base = { unit, defaultActive: false };
  return [
    {
      type,
      metric: pattern.max,
      title: `${title} max`.trim(),
      color: stat.max,
      ...base,
    },
    {
      type,
      metric: pattern.min,
      title: `${title} min`.trim(),
      color: stat.min,
      ...base,
    },
    {
      type,
      metric: pattern.median,
      title: `${title} median`.trim(),
      color: stat.median,
      ...base,
    },
    {
      type,
      metric: pattern.pct75,
      title: `${title} pct75`.trim(),
      color: stat.pct75,
      ...base,
    },
    {
      type,
      metric: pattern.pct25,
      title: `${title} pct25`.trim(),
      color: stat.pct25,
      ...base,
    },
    {
      type,
      metric: pattern.pct90,
      title: `${title} pct90`.trim(),
      color: stat.pct90,
      ...base,
    },
    {
      type,
      metric: pattern.pct10,
      title: `${title} pct10`.trim(),
      color: stat.pct10,
      ...base,
    },
  ];
}

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
    ...percentileSeries(colors, pattern, unit, title),
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
    ...percentileSeries(colors, pattern, unit, title),
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
    ...percentileSeries(colors, pattern, unit, title),
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
  return [
    {
      type: "Dots",
      metric: pattern.average,
      title: `${title} avg`.trim(),
      unit,
    },
    ...percentileSeries(colors, pattern, unit, title, { type: "Dots" }),
  ];
}

/**
 * Create series from a pattern with sum and cumulative (fullness stats + sum + cumulative)
 * @param {Colors} colors
 * @param {FullnessPatternWithSumCumulative} pattern
 * @param {Unit} unit
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFullnessPatternWithSumCumulative(
  colors,
  pattern,
  unit,
  title = "",
) {
  const { stat } = colors;
  return [
    ...fromFullnessPattern(colors, pattern, unit, title),
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
  ];
}

/**
 * Create series from a CoinbasePattern ({ sats, bitcoin, dollars } each with stats + sum + cumulative)
 * @param {Colors} colors
 * @param {CoinbasePattern} pattern
 * @param {string} [title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromCoinbasePattern(colors, pattern, title = "") {
  return [
    ...fromFullnessPatternWithSumCumulative(
      colors,
      pattern.bitcoin,
      Unit.btc,
      title,
    ),
    ...fromFullnessPatternWithSumCumulative(
      colors,
      pattern.sats,
      Unit.sats,
      title,
    ),
    ...fromFullnessPatternWithSumCumulative(
      colors,
      pattern.dollars,
      Unit.usd,
      title,
    ),
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
 * @param {boolean} [defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoinPatternWithUnit(
  colors,
  pattern,
  unit,
  title = "",
  sumColor,
  cumulativeColor,
  defaultActive,
) {
  return [
    {
      metric: pattern.sum,
      title: `${title} sum`.trim(),
      color: sumColor,
      unit,
      defaultActive,
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
    ...percentileSeries(colors, pattern, unit, title),
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
