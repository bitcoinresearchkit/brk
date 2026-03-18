/** Series helpers for creating chart series blueprints */

import { colors } from "../utils/colors.js";
import { Unit } from "../utils/units.js";

// ============================================================================
// Rolling window constants
// ============================================================================

/** @typedef {'_24h' | '_1w' | '_1m' | '_1y'} RollingWindowKey */

/** @type {ReadonlyArray<{key: RollingWindowKey, name: string, color: Color}>} */
export const ROLLING_WINDOWS = [
  { key: "_24h", name: "24h", color: colors.time._24h },
  { key: "_1w", name: "1w", color: colors.time._1w },
  { key: "_1m", name: "1m", color: colors.time._1m },
  { key: "_1y", name: "1y", color: colors.time._1y },
];

/** @type {ReadonlyArray<{key: '_24h' | '_1w' | '_1m', name: string, color: Color}>} */
export const ROLLING_WINDOWS_TO_1M = [
  { key: "_24h", name: "24h", color: colors.time._24h },
  { key: "_1w", name: "1w", color: colors.time._1w },
  { key: "_1m", name: "1m", color: colors.time._1m },
];

/**
 * Extract a series from each rolling window via a mapping function
 * @template T
 * @param {{ _24h: T, _1w: T, _1m: T, _1y: T }} windows
 * @param {(v: T) => AnySeriesPattern} extract
 * @returns {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }}
 */
export function mapWindows(windows, extract) {
  return {
    _24h: extract(windows._24h),
    _1w: extract(windows._1w),
    _1m: extract(windows._1m),
    _1y: extract(windows._1y),
  };
}

// ============================================================================
// Price helper for top pane (auto-expands to USD + sats)
// ============================================================================

/**
 * Create a price series for the top pane (auto-expands to USD + sats versions)
 * @param {Object} args
 * @param {AnyPricePattern} args.series - Price pattern with usd and sats
 * @param {string} args.name
 * @param {string} [args.key]
 * @param {LineStyle} [args.style]
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedPriceSeriesBlueprint}
 */
export function price({
  series,
  name,
  key,
  style,
  color,
  defaultActive,
  options,
}) {
  return {
    series,
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
      series: pattern.max,
      name: `${title} max`.trim(),
      color: stat.max,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.min,
      name: `${title} min`.trim(),
      color: stat.min,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.median,
      name: `${title} median`.trim(),
      color: stat.median,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.pct75,
      name: `${title} pct75`.trim(),
      color: stat.pct75,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.pct25,
      name: `${title} pct25`.trim(),
      color: stat.pct25,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.pct90,
      name: `${title} pct90`.trim(),
      color: stat.pct90,
      unit,
      defaultActive: false,
    }),
    dots({
      series: pattern.pct10,
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
 * @param {AnySeriesPattern} args.series
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
  series,
  name,
  key,
  style,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    series,
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
 * @param {AnySeriesPattern} args.series
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {LineSeriesPartialOptions} [args.options]
 * @returns {FetchedDotsSeriesBlueprint}
 */
export function dots({
  series,
  name,
  key,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Dots"),
    series,
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
 * @param {AnySeriesPattern} args.series
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {[Color, Color]} [args.colors] - [upColor, downColor] for legend
 * @param {boolean} [args.defaultActive]
 * @param {CandlestickSeriesPartialOptions} [args.options]
 * @returns {FetchedCandlestickSeriesBlueprint}
 */
export function candlestick({
  series,
  name,
  key,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Candlestick"),
    series,
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
 * @param {AnySeriesPattern} args.series
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
  series,
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
    series,
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
 * @param {AnySeriesPattern} args.series
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
  series,
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
    series,
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
 * @param {AnySeriesPattern} args.series
 * @param {string} args.name
 * @param {Unit} args.unit
 * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {HistogramSeriesPartialOptions} [args.options]
 * @returns {FetchedHistogramSeriesBlueprint}
 */
export function histogram({
  series,
  name,
  key,
  color,
  defaultActive,
  unit,
  options,
}) {
  return {
    type: /** @type {const} */ ("Histogram"),
    series,
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
 * @param {{ height: AnySeriesPattern } & Record<string, any>} args.pattern - Pattern with .height and rolling stats (p10/p25/p75/p90 as _1y24h30d7dPattern)
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
      series: pattern.height,
      name: title || "base",
      color: baseColor,
      unit,
    }),
    dots({
      series: stats.average,
      name: `${title} avg`.trim(),
      color: stat.avg,
      unit,
      defaultActive: avgActive,
    }),
    ...percentileSeries(stats, unit, title),
  ];
}

/**
 * Create series from a flat stats pattern (average + pct percentiles as single series)
 * Use statsAtWindow() to extract from patterns with _1y24h30d7dPattern stats
 * @param {Object} args
 * @param {{ average: AnySeriesPattern, median: AnySeriesPattern, max: AnySeriesPattern, min: AnySeriesPattern, pct75: AnySeriesPattern, pct25: AnySeriesPattern, pct90: AnySeriesPattern, pct10: AnySeriesPattern }} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function fromStatsPattern({ pattern, unit, title = "" }) {
  return [
    {
      type: "Dots",
      series: pattern.average,
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
 * Create a Rolling folder tree from a _1m1w1y24hPattern (4 rolling windows)
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.windows
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {string} [args.name]
 * @param {(args: {series: AnySeriesPattern, name: string, color: Color, unit: Unit}) => AnyFetchedSeriesBlueprint} [args.series]
 * @returns {PartialOptionsGroup}
 */
export function rollingWindowsTree({ windows, title, unit, name = "Sums", series = line }) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${title} Rolling`,
        bottom: ROLLING_WINDOWS.map((w) =>
          series({
            series: windows[w.key],
            name: w.name,
            color: w.color,
            unit,
          }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} ${w.name}`,
        bottom: [
          series({
            series: windows[w.key],
            name: w.name,
            color: w.color,
            unit,
          }),
        ],
      })),
    ],
  };
}

/**
 * Create a Distribution folder tree with stats at each rolling window (24h/7d/30d/1y)
 * @param {Object} args
 * @param {Record<string, any>} args.pattern - Pattern with pct10/pct25/... and average/median/... as _1y24h30d7dPattern
 * @param {AnySeriesPattern} [args.base] - Optional base series to show as dots on each chart
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsGroup}
 */
export function distributionWindowsTree({ pattern, base, title, unit }) {
  return {
    name: "Distributions",
    tree: [
      {
        name: "Compare",
        title: `${title} Average`,
        bottom: ROLLING_WINDOWS.map((w) =>
          line({ series: pattern.average[w.key], name: w.name, color: w.color, unit }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} Distribution (${w.name})`,
        bottom: [
          ...(base ? [line({ series: base, name: "base", unit })] : []),
          ...fromStatsPattern({
            pattern: statsAtWindow(pattern, w.key),
            unit,
          }),
        ],
      })),
    ],
  };
}

/**
 * Map a rolling window slot's stats to a specific unit, producing a stats-compatible pattern
 * @param {{ average: Record<string, AnySeriesPattern>, median: Record<string, AnySeriesPattern>, max: Record<string, AnySeriesPattern>, min: Record<string, AnySeriesPattern>, pct75: Record<string, AnySeriesPattern>, pct25: Record<string, AnySeriesPattern>, pct90: Record<string, AnySeriesPattern>, pct10: Record<string, AnySeriesPattern> }} slot - Rolling window slot with multi-currency stats
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
 * @param {{ average: Record<string, AnySeriesPattern>, median: Record<string, AnySeriesPattern>, max: Record<string, AnySeriesPattern>, min: Record<string, AnySeriesPattern>, pct75: Record<string, AnySeriesPattern>, pct25: Record<string, AnySeriesPattern>, pct90: Record<string, AnySeriesPattern>, pct10: Record<string, AnySeriesPattern> }} slot - Rolling window slot with multi-currency stats
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export const distributionBtcSatsUsd = (slot) => [
  ...fromStatsPattern({
    pattern: rollingSlotForUnit(slot, "btc"),
    unit: Unit.btc,
  }),
  ...fromStatsPattern({
    pattern: rollingSlotForUnit(slot, "sats"),
    unit: Unit.sats,
  }),
  ...fromStatsPattern({
    pattern: rollingSlotForUnit(slot, "usd"),
    unit: Unit.usd,
  }),
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
      series: pattern.btc,
      title,
      color,
      unit: Unit.btc,
    },
    {
      series: pattern.sats,
      title,
      color,
      unit: Unit.sats,
    },
    {
      series: pattern.usd,
      title,
      color,
      unit: Unit.usd,
    },
  ];
}

// ============================================================================
// Percent + Ratio helpers
// ============================================================================

/**
 * Create percent + ratio series from a BpsPercentRatioPattern
 * @param {Object} args
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function percentRatio({ pattern, name, color, defaultActive }) {
  return [
    line({ series: pattern.percent, name, color, defaultActive, unit: Unit.percentage }),
    line({ series: pattern.ratio, name, color, defaultActive, unit: Unit.ratio }),
  ];
}

/**
 * Create percent + ratio dots series from a BpsPercentRatioPattern
 * @param {Object} args
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function percentRatioDots({ pattern, name, color, defaultActive }) {
  return [
    dots({ series: pattern.percent, name, color, defaultActive, unit: Unit.percentage }),
    dots({ series: pattern.ratio, name, color, defaultActive, unit: Unit.ratio }),
  ];
}

/**
 * Create percent + ratio baseline series from a BpsPercentRatioPattern
 * @param {Object} args
 * @param {{ percent: AnySeriesPattern, ratio: AnySeriesPattern }} args.pattern
 * @param {string} args.name
 * @param {Color | [Color, Color]} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function percentRatioBaseline({ pattern, name, defaultActive }) {
  return [
    baseline({ series: pattern.percent, name, defaultActive, unit: Unit.percentage }),
    baseline({ series: pattern.ratio, name, defaultActive, unit: Unit.ratio }),
  ];
}

/**
 * Create a Rolling folder tree where each window is a BpsPercentRatioPattern (percent + ratio)
 * @param {Object} args
 * @param {{ _24h: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1w: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1m: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1y: { percent: AnySeriesPattern, ratio: AnySeriesPattern } }} args.windows
 * @param {string} args.title
 * @param {string} [args.name]
 * @param {(args: {pattern: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, name: string, color?: Color}) => AnyFetchedSeriesBlueprint[]} [args.series]
 * @returns {PartialOptionsGroup}
 */
export function rollingPercentRatioTree({ windows, title, name = "Sums", series = percentRatio }) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${title} Rolling`,
        bottom: ROLLING_WINDOWS.flatMap((w) =>
          percentRatio({ pattern: windows[w.key], name: w.name, color: w.color }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} ${w.name}`,
        bottom: series({ pattern: windows[w.key], name: w.name }),
      })),
    ],
  };
}

/**
 * Create Change + Growth Rate tree from a delta pattern (absolute + rate)
 * @template T
 * @param {Object} args
 * @param {{ absolute: { _24h: T, _1w: T, _1m: T, _1y: T }, rate: { _24h: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1w: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1m: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1y: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} args.delta
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {(v: T) => AnySeriesPattern} args.extract
 * @returns {PartialOptionsTree}
 */
export function deltaTree({ delta, title, unit, extract }) {
  return [
    {
      name: "Change",
      tree: [
        {
          name: "Compare",
          title: `${title} Change`,
          bottom: ROLLING_WINDOWS.map((w) =>
            baseline({
              series: extract(delta.absolute[w.key]),
              name: w.name,
              color: w.color,
              unit,
            }),
          ),
        },
        ...ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `${title} Change ${w.name}`,
          bottom: [
            baseline({
              series: extract(delta.absolute[w.key]),
              name: w.name,
              unit,
            }),
          ],
        })),
      ],
    },
    rollingPercentRatioTree({ windows: delta.rate, title: `${title} Growth Rate`, name: "Growth Rate", series: percentRatioBaseline }),
  ];
}

/**
 * deltaTree where absolute windows are directly AnySeriesPattern (no extract needed)
 * @param {Object} args
 * @param {{ absolute: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, rate: { _24h: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1w: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1m: { percent: AnySeriesPattern, ratio: AnySeriesPattern }, _1y: { percent: AnySeriesPattern, ratio: AnySeriesPattern } } }} args.delta
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function simpleDeltaTree({ delta, title, unit }) {
  return deltaTree({ delta, title, unit, extract: (v) => v });
}

// ============================================================================
// Chart-generating helpers (return PartialOptionsTree for folder structures)
// ============================================================================
// These split patterns into separate Sum/Distribution/Cumulative charts


/**
 * Create btc/sats/usd series from patterns
 * @param {Object} args
 * @param {{ btc: AnySeriesPattern, sats: AnySeriesPattern, usd: AnySeriesPattern }} args.patterns
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function btcSatsUsdSeries({ patterns, name, color, defaultActive }) {
  return [
    {
      series: patterns.btc,
      title: name,
      color,
      unit: Unit.btc,
      defaultActive,
    },
    {
      series: patterns.sats,
      title: name,
      color,
      unit: Unit.sats,
      defaultActive,
    },
    {
      series: patterns.usd,
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
    ? `${title} ${distributionSuffix}`
    : title;
  return [
    {
      name: "Per Block",
      title,
      bottom: [{ series: pattern.base, title: "base", unit }],
    },
    rollingWindowsTree({ windows: pattern.sum, title, unit }),
    distributionWindowsTree({ pattern, title: distTitle, unit }),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ series: pattern.cumulative, title: "all-time", unit }],
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
export function chartsFromAggregated({
  pattern,
  title,
  unit,
  distributionSuffix = "",
}) {
  const { stat } = colors;
  const distTitle = distributionSuffix
    ? `${title} ${distributionSuffix}`
    : title;
  return [
    {
      name: "Per Block",
      title,
      bottom: [{ series: pattern.sum, title: "base", color: stat.sum, unit }],
    },
    rollingWindowsTree({ windows: pattern.rolling.sum, title, unit }),
    distributionWindowsTree({ pattern: pattern.rolling, title: distTitle, unit }),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ series: pattern.cumulative, title: "all-time", unit }],
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
export const chartsFromAggregatedPerBlock = (args) =>
  chartsFromAggregated({ ...args, distributionSuffix: "per Block" });

/**
 * Create Per Block + Per 6 Blocks stats charts from a _6bBlockTxPattern
 * @param {Object} args
 * @param {{ block: DistributionStats, _6b: DistributionStats }} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function chartsFromBlockAnd6b({ pattern, title, unit }) {
  return [
    {
      name: "Block",
      title: `${title} (Block)`,
      bottom: fromStatsPattern({ pattern: pattern.block, unit }),
    },
    {
      name: "Hourly",
      title: `${title} (Hourly)`,
      bottom: fromStatsPattern({ pattern: pattern._6b, unit }),
    },
  ];
}

/**
 * Sums + Cumulative charts (no Per Block)
 * @param {Object} args
 * @param {{ sum: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, cumulative: AnySeriesPattern }} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function chartsFromSumsCumulative({ pattern, title, unit, color }) {
  return [
    rollingWindowsTree({ windows: pattern.sum, title, unit }),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: [{ series: pattern.cumulative, title: "all-time", color, unit }],
    },
  ];
}

/**
 * Per Block + Sums + Cumulative charts
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
      name: "Per Block",
      title,
      bottom: [{ series: pattern.base, title: "base", color, unit }],
    },
    ...chartsFromSumsCumulative({ pattern, title, unit, color }),
  ];
}

/**
 * Split multiple named entries (each with base/sum/cumulative) into Per Block/Sums/Cumulative charts
 * @param {Object} args
 * @param {Array<[string, CountPattern<any>]>} args.entries
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function chartsFromCountEntries({ entries, title, unit }) {
  return multiSeriesTree({
    entries: entries.map(([name, data], i, arr) => ({
      name,
      color: colors.at(i, arr.length),
      base: data.base,
      rolling: data.sum,
      cumulative: data.cumulative,
    })),
    title,
    unit,
  });
}

/**
 * Per Block + Sums + Cumulative tree for multiple named series shown side-by-side
 * @param {Object} args
 * @param {Array<{ name: string, color: Color, base: AnySeriesPattern, rolling: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, cumulative: AnySeriesPattern }>} args.entries
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function multiSeriesTree({ entries, title, unit }) {
  return [
    {
      name: "Per Block",
      title,
      bottom: entries.map((e) =>
        line({ series: e.base, name: e.name, color: e.color, unit }),
      ),
    },
    {
      name: "Sums",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} (${w.name})`,
        bottom: entries.map((e) =>
          line({ series: e.rolling[w.key], name: e.name, color: e.color, unit }),
        ),
      })),
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: entries.map((e) =>
        line({ series: e.cumulative, name: e.name, color: e.color, unit }),
      ),
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
        ...btcSatsUsdSeries({ patterns: pattern.base, name: "sum" }),
        ...btcSatsUsdSeries({
          patterns: pattern.sum._24h,
          name: "24h sum",
          defaultActive: false,
        }),
      ],
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: btcSatsUsdSeries({
        patterns: pattern.cumulative,
        name: "all-time",
      }),
    },
  ];
}
