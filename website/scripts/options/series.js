/** Series helpers for creating chart series blueprints */

import { colors } from "../utils/colors.js";
import { Unit } from "../utils/units.js";

// ============================================================================
// Price helper for top pane (auto-expands to USD + sats)
// ============================================================================

/**
 * Create a price series for the top pane (auto-expands to USD + sats versions)
 * @param {Object} args
 * @param {AnyPricePattern} args.metric - Price pattern with usd and sats
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
 * @param {DistributionStats} pattern
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
 * @param {number} [args.style] - Line style (0: Solid, 1: Dotted, 2: Dashed, 3: LargeDashed, 4: SparseDotted)
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
  style,
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
      lineStyle: style,
      ...options,
    },
  };
}

/**
 * @param {Omit<Parameters<typeof baseline>[0], 'style'>} args
 */
export function dottedBaseline(args) {
  const _args = /** @type {Parameters<typeof baseline>[0]} */ (args);
  _args.style = 1;
  return baseline(_args);
}

/**
 * Baseline series rendered as dots (points only, no line)
 * @param {Object} args
 * @param {AnyMetricPattern} args.metric
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key]
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {number | undefined} [args.base]
 * @param {BaselineSeriesPartialOptions} [args.options]
 * @returns {FetchedDotsBaselineSeriesBlueprint}
 */
export function dotsBaseline({
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
    type: /** @type {const} */ ("DotsBaseline"),
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
 * Create series from an AverageHeightMaxMedianMinP10P25P75P90Pattern (height + rolling stats)
 * @param {Object} args
 * @param {{ height: AnyMetricPattern } & Record<string, any>} args.pattern - Pattern with .height and rolling stats (p10/p25/p75/p90 as _1y24h30d7dPattern)
 * @param {string} args.window - Rolling window key (e.g., '_24h', '_7d', '_30d', '_1y')
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @param {Color} [args.baseColor]
 * @param {boolean} [args.avgActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromBaseStatsPattern({
  pattern,
  window,
  unit,
  title = "",
  baseColor,
  avgActive = true,
}) {
  const { stat } = colors;
  const stats = statsAtWindow(pattern, window);
  return [
    dots({
      metric: pattern.height,
      name: title || "base",
      color: baseColor,
      unit,
    }),
    dots({
      metric: stats.average,
      name: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: avgActive,
    }),
    ...percentileSeries(stats, unit, title),
  ];
}

/**
 * Create series from a flat stats pattern (average + pct percentiles as single metrics)
 * Use statsAtWindow() to extract from patterns with _1y24h30d7dPattern stats
 * @param {Object} args
 * @param {{ average: AnyMetricPattern, median: AnyMetricPattern, max: AnyMetricPattern, min: AnyMetricPattern, pct75: AnyMetricPattern, pct25: AnyMetricPattern, pct90: AnyMetricPattern, pct10: AnyMetricPattern }} args.pattern
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
 * Extract stats at a specific rolling window from patterns with _1y24h30d7dPattern stats
 * @param {Record<string, any>} pattern - Pattern with pct10/pct25/pct75/pct90 and average/median/max/min as _1y24h30d7dPattern
 * @param {string} window
 */
export function statsAtWindow(pattern, window) {
  return {
    average: pattern.average[window],
    median: pattern.median[window],
    max: pattern.max[window],
    min: pattern.min[window],
    pct75: pattern.pct75[window],
    pct25: pattern.pct25[window],
    pct90: pattern.pct90[window],
    pct10: pattern.pct10[window],
  };
}

/**
 * Create a Rolling folder tree from a _1y24h30d7dPattern (4 rolling windows)
 * @param {Object} args
 * @param {{ _24h: AnyMetricPattern, _7d: AnyMetricPattern, _30d: AnyMetricPattern, _1y: AnyMetricPattern }} args.windows
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsGroup}
 */
export function rollingWindowsTree({ windows, title, unit }) {
  return {
    name: "Rolling",
    tree: [
      {
        name: "Compare",
        title: `${title} Rolling`,
        bottom: [
          line({ metric: windows._24h, name: "24h", color: colors.time._24h, unit }),
          line({ metric: windows._7d, name: "7d", color: colors.time._1w, unit }),
          line({ metric: windows._30d, name: "30d", color: colors.time._1m, unit }),
          line({ metric: windows._1y, name: "1y", color: colors.time._1y, unit }),
        ],
      },
      {
        name: "24h",
        title: `${title} 24h`,
        bottom: [line({ metric: windows._24h, name: "24h", color: colors.time._24h, unit })],
      },
      {
        name: "7d",
        title: `${title} 7d`,
        bottom: [line({ metric: windows._7d, name: "7d", color: colors.time._1w, unit })],
      },
      {
        name: "30d",
        title: `${title} 30d`,
        bottom: [line({ metric: windows._30d, name: "30d", color: colors.time._1m, unit })],
      },
      {
        name: "1y",
        title: `${title} 1y`,
        bottom: [line({ metric: windows._1y, name: "1y", color: colors.time._1y, unit })],
      },
    ],
  };
}

/**
 * Map a rolling window slot's stats to a specific unit, producing a stats-compatible pattern
 * @param {RollingWindowSlot} slot - Rolling window slot (e.g., pattern.rolling._24h)
 * @param {BtcSatsUsdKey} unitKey
 */
function rollingSlotForUnit(slot, unitKey) {
  return {
    average: slot.average[unitKey],
    median: slot.median[unitKey],
    max: slot.max[unitKey],
    min: slot.min[unitKey],
    pct75: slot.pct75[unitKey],
    pct25: slot.pct25[unitKey],
    pct90: slot.pct90[unitKey],
    pct10: slot.pct10[unitKey],
  };
}

/**
 * Create distribution series for btc/sats/usd from a rolling window slot
 * @param {RollingWindowSlot} slot - Rolling window slot (e.g., pattern.rolling._24h)
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export const distributionBtcSatsUsd = (slot) => [
  ...fromStatsPattern({ pattern: rollingSlotForUnit(slot, "btc"), unit: Unit.btc }),
  ...fromStatsPattern({ pattern: rollingSlotForUnit(slot, "sats"), unit: Unit.sats }),
  ...fromStatsPattern({ pattern: rollingSlotForUnit(slot, "usd"), unit: Unit.usd }),
];

/**
 * Create series from a SupplyPattern (sats/btc/usd, no sum/cumulative)
 * @param {Object} args
 * @param {SupplyPattern} args.pattern
 * @param {string} args.title
 * @param {Color} [args.color]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromSupplyPattern({ pattern, title, color }) {
  return [
    {
      metric: pattern.btc,
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
      metric: pattern.usd,
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
 * @param {DistributionStats} pattern
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
 * @param {{ btc: AnyMetricPattern, sats: AnyMetricPattern, usd: AnyMetricPattern }} args.metrics
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function btcSatsUsdSeries({ metrics, name, color, defaultActive }) {
  return [
    {
      metric: metrics.btc,
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
      metric: metrics.usd,
      title: name,
      color,
      unit: Unit.usd,
      defaultActive,
    },
  ];
}

/**
 * Split flat per-block pattern into charts (Sum/Rolling/Distribution/Cumulative)
 * Pattern has: .height, .cumulative, .sum (windowed), .average/.pct10/... (windowed, flat)
 * @param {Object} args
 * @param {FullPerBlockPattern} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {string} [args.distributionSuffix]
 * @returns {PartialOptionsTree}
 */
export function chartsFromFull({
  pattern,
  title,
  unit,
  distributionSuffix = "",
}) {
  const distTitle = distributionSuffix
    ? `${title} ${distributionSuffix} Distribution`
    : `${title} Distribution`;
  return [
    {
      name: "Sum",
      title,
      bottom: [{ metric: pattern.height, title: "base", unit }],
    },
    rollingWindowsTree({ windows: pattern.sum, title, unit }),
    {
      name: "Distribution",
      title: distTitle,
      bottom: distributionSeries(statsAtWindow(pattern, "_24h"), unit),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ metric: pattern.cumulative, title: "all-time", unit }],
    },
  ];
}

/**
 * Split pattern into 4 charts with "per Block" in distribution title
 * @param {Object} args
 * @param {FullPerBlockPattern} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export const chartsFromFullPerBlock = (args) =>
  chartsFromFull({ ...args, distributionSuffix: "per Block" });

/**
 * Split pattern with sum + distribution + cumulative into 3 charts (no base)
 * @param {Object} args
 * @param {FullStatsPattern} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {string} [args.distributionSuffix]
 * @returns {PartialOptionsTree}
 */
export function chartsFromSum({
  pattern,
  title,
  unit,
  distributionSuffix = "",
}) {
  const { stat } = colors;
  const distTitle = distributionSuffix
    ? `${title} ${distributionSuffix} Distribution`
    : `${title} Distribution`;
  return [
    {
      name: "Sum",
      title,
      bottom: [{ metric: pattern.sum, title: "sum", color: stat.sum, unit }],
    },
    {
      name: "Distribution",
      title: distTitle,
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
 * Split pattern into 3 charts with "per Block" in distribution title (no base)
 * @param {Object} args
 * @param {FullStatsPattern} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export const chartsFromSumPerBlock = (args) =>
  chartsFromSum({ ...args, distributionSuffix: "per Block" });

/**
 * Split pattern with rolling sum windows + cumulative into charts
 * @param {Object} args
 * @param {CountPattern<any>} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function chartsFromCount({ pattern, title, unit, color }) {
  return [
    rollingWindowsTree({ windows: pattern.sum, title, unit }),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ metric: pattern.cumulative, title: "all-time", color, unit }],
    },
  ];
}

/**
 * Split BaseCumulativeRollingPattern into 3 charts (Sum/Distribution/Cumulative)
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
        ...btcSatsUsdSeries({ metrics: pattern.base, name: "sum" }),
        ...btcSatsUsdSeries({
          metrics: pattern._24h.sum,
          name: "24h sum",
          defaultActive: false,
        }),
      ],
    },
    {
      name: "Distribution",
      title: `${title} Distribution`,
      bottom: distributionBtcSatsUsd(pattern._24h),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: btcSatsUsdSeries({ metrics: pattern.cumulative, name: "all-time" }),
    },
  ];
}
