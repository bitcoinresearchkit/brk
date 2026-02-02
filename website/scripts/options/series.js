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
 * @param {Colors} colors
 * @param {StatsPattern<any> | BaseStatsPattern<any> | FullStatsPattern<any> | AnyStatsPattern} pattern
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
 * @param {Omit<Parameters<typeof line>[0], 'style'>} args
 */
export function dotted(args) {
  const _args = /** @type {Parameters<typeof line>[0]} */ (args);
  _args.style = 1;
  return line(_args);
}

/**
 * @param {Omit<Parameters<typeof line>[0], 'style'>} args
 */
export function sparseDotted(args) {
  const _args = /** @type {Parameters<typeof line>[0]} */ (args);
  _args.style = 4;
  return line(_args);
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
 * Create series from patterns with sum + cumulative + percentiles (NO base)
 * @param {Colors} colors
 * @param {Object} args
 * @param {AnyStatsPattern} args.pattern
 * @param {Unit} args.unit
 * @param {Unit} args.cumulativeUnit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSumStatsPattern(colors, { pattern, unit, cumulativeUnit, title = "" }) {
  const { stat } = colors;
  return [
    { metric: pattern.average, title: `${title} avg`.trim(), unit },
    {
      metric: pattern.sum,
      title: title || "sum",
      color: stat.sum,
      unit,
      defaultActive: false,
    },
    {
      metric: pattern.cumulative,
      title: title || "cumulative",
      unit: cumulativeUnit,
    },
    ...percentileSeries(colors, pattern, unit, title),
  ];
}

/**
 * Create series from a BaseStatsPattern (base + avg + percentiles, NO sum)
 * @param {Colors} colors
 * @param {Object} args
 * @param {BaseStatsPattern<any>} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @param {Color} [args.baseColor]
 * @param {boolean} [args.avgActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBaseStatsPattern(
  colors,
  { pattern, unit, title = "", baseColor, avgActive = true },
) {
  const { stat } = colors;
  return [
    { metric: pattern.base, title: title || "base", color: baseColor, unit },
    {
      metric: pattern.average,
      title: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: avgActive,
    },
    ...percentileSeries(colors, pattern, unit, title),
  ];
}

/**
 * Create series from a FullStatsPattern (base + sum + cumulative + avg + percentiles)
 * @param {Colors} colors
 * @param {Object} args
 * @param {FullStatsPattern<any>} args.pattern
 * @param {Unit} args.unit
 * @param {Unit} args.cumulativeUnit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromFullStatsPattern(colors, { pattern, unit, cumulativeUnit, title = "" }) {
  const { stat } = colors;
  return [
    { metric: pattern.base, title: title || "base", unit },
    {
      metric: pattern.sum,
      title: title || "sum",
      color: stat.sum,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: title || "cumulative",
      unit: cumulativeUnit,
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
 * Create series from a StatsPattern (avg + percentiles, NO base)
 * @param {Colors} colors
 * @param {Object} args
 * @param {StatsPattern<any>} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromStatsPattern(colors, { pattern, unit, title = "" }) {
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
 * Create series from AnyFullStatsPattern (base + sum + cumulative + avg + percentiles)
 * @param {Colors} colors
 * @param {Object} args
 * @param {AnyFullStatsPattern} args.pattern
 * @param {Unit} args.unit
 * @param {Unit} args.cumulativeUnit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromAnyFullStatsPattern(colors, { pattern, unit, cumulativeUnit, title = "" }) {
  const { stat } = colors;
  return [
    ...fromBaseStatsPattern(colors, { pattern, unit, title }),
    {
      metric: pattern.sum,
      title: title || "sum",
      color: stat.sum,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: title || "cumulative",
      unit: cumulativeUnit,
    },
  ];
}

/**
 * Create series from a CoinbasePattern ({ sats, bitcoin, dollars } each with stats + sum + cumulative)
 * @param {Colors} colors
 * @param {Object} args
 * @param {CoinbasePattern} args.pattern
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromCoinbasePattern(colors, { pattern, title = "" }) {
  return [
    ...fromAnyFullStatsPattern(colors, {
      pattern: pattern.bitcoin,
      unit: Unit.btc,
      cumulativeUnit: Unit.btcCumulative,
      title,
    }),
    ...fromAnyFullStatsPattern(colors, {
      pattern: pattern.sats,
      unit: Unit.sats,
      cumulativeUnit: Unit.satsCumulative,
      title,
    }),
    ...fromAnyFullStatsPattern(colors, {
      pattern: pattern.dollars,
      unit: Unit.usd,
      cumulativeUnit: Unit.usdCumulative,
      title,
    }),
  ];
}

/**
 * Create series from a ValuePattern ({ sats, bitcoin, dollars } each as CountPattern with sum + cumulative)
 * @param {Object} args
 * @param {ValuePattern} args.pattern
 * @param {string} [args.title]
 * @param {Color} [args.color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromValuePattern({ pattern, title = "", color }) {
  return [
    {
      metric: pattern.bitcoin.sum,
      title: title || "sum",
      color,
      unit: Unit.btc,
    },
    {
      metric: pattern.bitcoin.cumulative,
      title: title || "cumulative",
      color,
      unit: Unit.btcCumulative,
    },
    {
      metric: pattern.sats.sum,
      title: title || "sum",
      color,
      unit: Unit.sats,
    },
    {
      metric: pattern.sats.cumulative,
      title: title || "cumulative",
      color,
      unit: Unit.satsCumulative,
    },
    {
      metric: pattern.dollars.sum,
      title: title || "sum",
      color,
      unit: Unit.usd,
    },
    {
      metric: pattern.dollars.cumulative,
      title: title || "cumulative",
      color,
      unit: Unit.usdCumulative,
    },
  ];
}

/**
 * Create sum/cumulative series from a BitcoinPattern ({ sum, cumulative }) with explicit unit and colors
 * @param {Object} args
 * @param {{ sum: AnyMetricPattern, cumulative: AnyMetricPattern }} args.pattern
 * @param {Unit} args.unit
 * @param {Unit} args.cumulativeUnit
 * @param {string} [args.title]
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBitcoinPatternWithUnit({
  pattern,
  unit,
  cumulativeUnit,
  title = "",
  color,
  defaultActive,
}) {
  return [
    {
      metric: pattern.sum,
      title: title || "sum",
      color,
      unit,
      defaultActive,
    },
    {
      metric: pattern.cumulative,
      title: title || "cumulative",
      color,
      unit: cumulativeUnit,
    },
  ];
}

/**
 * Create sum/cumulative series from a CountPattern with explicit unit and colors
 * @param {Object} args
 * @param {CountPattern<any>} args.pattern
 * @param {Unit} args.unit
 * @param {Unit} args.cumulativeUnit
 * @param {string} [args.title]
 * @param {Color} [args.color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromCountPattern({ pattern, unit, cumulativeUnit, title = "", color }) {
  return [
    {
      metric: pattern.sum,
      title: title || "sum",
      color,
      unit,
    },
    {
      metric: pattern.cumulative,
      title: title || "cumulative",
      color,
      unit: cumulativeUnit,
    },
  ];
}

/**
 * Create series from a SupplyPattern (sats/bitcoin/dollars, no sum/cumulative)
 * @param {Object} args
 * @param {SupplyPattern} args.pattern
 * @param {string} args.title
 * @param {Color} [args.color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSupplyPattern({ pattern, title, color }) {
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
