/** Series helpers for creating chart series blueprints */

import { colors } from "../utils/colors.js";
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
 * @param {StatsPattern<any> | BaseStatsPattern<any> | FullStatsPattern<any> | AnyStatsPattern} pattern
 * @param {Unit} unit
 * @param {string} title
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function percentileSeries(pattern, unit, title) {
  const { stat } = colors;
  return [
    dots({
      metric: pattern.max,
      name: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.min,
      name: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.median,
      name: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct75,
      name: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct25,
      name: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct90,
      name: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct10,
      name: `${title} pct10`.trim(),
      color: stat.pct10,
      unit,
      defaultActive: false,
    }),
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

  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Candlestick"),
    metric,
    title: name,
    key,

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
 * Create series from a BaseStatsPattern (base + avg + percentiles, NO sum)
 * @param {Object} args
 * @param {BaseStatsPattern<any>} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @param {Color} [args.baseColor]
 * @param {boolean} [args.avgActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBaseStatsPattern({
  pattern,
  unit,
  title = "",
  baseColor,
  avgActive = true,
}) {
  const { stat } = colors;
  return [
    dots({
      metric: pattern.base,
      name: title || "base",
      color: baseColor,
      unit,
    }),
    dots({
      metric: pattern.average,
      name: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: avgActive,
    }),
    ...percentileSeries(pattern, unit, title),
  ];
}

/**
 * Create series from any pattern with avg + percentiles (works with StatsPattern, SumStatsPattern, etc.)
 * @param {Object} args
 * @param {StatsPattern<any> | BaseStatsPattern<any> | FullStatsPattern<any> | AnyStatsPattern} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromStatsPattern({ pattern, unit, title = "" }) {
  return [
    {
      type: "Dots",
      metric: pattern.average,
      title: `${title} avg`.trim(),
      unit,
    },
    ...percentileSeries(pattern, unit, title),
  ];
}

/**
 * Create distribution series for btc/sats/usd from a value pattern with stats (average + percentiles)
 * @param {FullValuePattern | SumValuePattern} source
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export const distributionBtcSatsUsd = (source) => [
  ...fromStatsPattern({ pattern: source.bitcoin, unit: Unit.btc }),
  ...fromStatsPattern({ pattern: source.sats, unit: Unit.sats }),
  ...fromStatsPattern({ pattern: source.dollars, unit: Unit.usd }),
];

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

// ============================================================================
// Chart-generating helpers (return PartialOptionsTree for folder structures)
// ============================================================================
// These split patterns into separate Sum/Distribution/Cumulative charts

/**
 * Create distribution series (avg + percentiles)
 * @param {StatsPattern<any> | BaseStatsPattern<any> | FullStatsPattern<any> | AnyStatsPattern} pattern
 * @param {Unit} unit
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function distributionSeries(pattern, unit) {
  const { stat } = colors;
  return [
    dots({ metric: pattern.average, name: "avg", color: stat.avg, unit }),
    dots({
      metric: pattern.median,
      name: "median",
      color: stat.median,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.max,
      name: "max",
      color: stat.max,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.min,
      name: "min",
      color: stat.min,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct75,
      name: "pct75",
      color: stat.pct75,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct25,
      name: "pct25",
      color: stat.pct25,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct90,
      name: "pct90",
      color: stat.pct90,
      unit,
      defaultActive: false,
    }),
    dots({
      metric: pattern.pct10,
      name: "pct10",
      color: stat.pct10,
      unit,
      defaultActive: false,
    }),
  ];
}

/**
 * Create btc/sats/usd series from metrics
 * @param {Object} args
 * @param {{ bitcoin: AnyMetricPattern, sats: AnyMetricPattern, dollars: AnyMetricPattern }} args.metrics
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function btcSatsUsdSeries({ metrics, name, color, defaultActive }) {
  return [
    {
      metric: metrics.bitcoin,
      title: name,
      color,
      unit: Unit.btc,
      defaultActive,
    },
    {
      metric: metrics.sats,
      title: name,
      color,
      unit: Unit.sats,
      defaultActive,
    },
    {
      metric: metrics.dollars,
      title: name,
      color,
      unit: Unit.usd,
      defaultActive,
    },
  ];
}

/**
 * Split pattern with base + sum + distribution + cumulative into 3 charts
 * @param {Object} args
 * @param {FullStatsPattern<any>} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function chartsFromFull({ pattern, title, unit }) {
  return [
    {
      name: "Sum",
      title,
      bottom: [
        { metric: pattern.base, title: "sum", unit },
        { metric: pattern.sum, title: "sum", unit },
      ],
    },
    {
      name: "Distribution",
      title: `${title} Distribution`,
      bottom: distributionSeries(pattern, unit),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ metric: pattern.cumulative, title: "all-time", unit }],
    },
  ];
}

/**
 * Split pattern with sum + distribution + cumulative into 3 charts (no base)
 * @param {Object} args
 * @param {AnyStatsPattern} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function chartsFromSum({ pattern, title, unit }) {
  const { stat } = colors;
  return [
    {
      name: "Sum",
      title,
      bottom: [{ metric: pattern.sum, title: "sum", color: stat.sum, unit }],
    },
    {
      name: "Distribution",
      title: `${title} Distribution`,
      bottom: distributionSeries(pattern, unit),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ metric: pattern.cumulative, title: "all-time", unit }],
    },
  ];
}

/**
 * Split pattern with sum + cumulative into 2 charts
 * @param {Object} args
 * @param {CountPattern<any>} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function chartsFromCount({ pattern, title, unit, color }) {
  return [
    {
      name: "Sum",
      title,
      bottom: [{ metric: pattern.sum, title: "sum", color, unit }],
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ metric: pattern.cumulative, title: "all-time", color, unit }],
    },
  ];
}

/**
 * Split value pattern (btc/sats/usd with sum + cumulative) into 2 charts
 * @param {Object} args
 * @param {ValuePattern} args.pattern
 * @param {string} args.title
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function chartsFromValue({ pattern, title, color }) {
  return [
    {
      name: "Sum",
      title,
      bottom: btcSatsUsdSeries({
        metrics: {
          bitcoin: pattern.bitcoin.sum,
          sats: pattern.sats.sum,
          dollars: pattern.dollars.sum,
        },
        name: "sum",
        color,
      }),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: btcSatsUsdSeries({
        metrics: {
          bitcoin: pattern.bitcoin.cumulative,
          sats: pattern.sats.cumulative,
          dollars: pattern.dollars.cumulative,
        },
        name: "all-time",
        color,
      }),
    },
  ];
}

/**
 * Split btc/sats/usd pattern with full stats into 3 charts
 * @param {Object} args
 * @param {CoinbasePattern} args.pattern
 * @param {string} args.title
 * @returns {PartialOptionsTree}
 */
export function chartsFromValueFull({ pattern, title }) {
  return [
    {
      name: "Sum",
      title,
      bottom: [
        ...btcSatsUsdSeries({
          metrics: {
            bitcoin: pattern.bitcoin.base,
            sats: pattern.sats.base,
            dollars: pattern.dollars.base,
          },
          name: "sum",
        }),
        ...btcSatsUsdSeries({
          metrics: {
            bitcoin: pattern.bitcoin.sum,
            sats: pattern.sats.sum,
            dollars: pattern.dollars.sum,
          },
          name: "sum",
        }),
      ],
    },
    {
      name: "Distribution",
      title: `${title} Distribution`,
      bottom: [
        ...distributionSeries(pattern.bitcoin, Unit.btc),
        ...distributionSeries(pattern.sats, Unit.sats),
        ...distributionSeries(pattern.dollars, Unit.usd),
      ],
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: btcSatsUsdSeries({
        metrics: {
          bitcoin: pattern.bitcoin.cumulative,
          sats: pattern.sats.cumulative,
          dollars: pattern.dollars.cumulative,
        },
        name: "all-time",
      }),
    },
  ];
}
