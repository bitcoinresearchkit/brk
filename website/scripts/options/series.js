/** Series helpers for creating chart series blueprints */

import { colors } from "../utils/colors.js";
import { Unit } from "../utils/units.js";

// ============================================================================
// Rolling window constants
// ============================================================================

/** @typedef {'_24h' | '_1w' | '_1m' | '_1y'} RollingWindowKey */

/** @type {ReadonlyArray<{key: RollingWindowKey, name: string, title: string, color: Color}>} */
export const ROLLING_WINDOWS = [
  { key: "_24h", name: "24h", title: "Daily", color: colors.time._24h },
  { key: "_1w", name: "1w", title: "Weekly", color: colors.time._1w },
  { key: "_1m", name: "1m", title: "Monthly", color: colors.time._1m },
  { key: "_1y", name: "1y", title: "Yearly", color: colors.time._1y },
];

/** @type {ReadonlyArray<{key: '_24h' | '_1w' | '_1m', name: string, title: string, color: Color}>} */
export const ROLLING_WINDOWS_TO_1M = [
  { key: "_24h", name: "24h", title: "Daily", color: colors.time._24h },
  { key: "_1w", name: "1w", title: "Weekly", color: colors.time._1w },
  { key: "_1m", name: "1m", title: "Monthly", color: colors.time._1m },
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
 * @param {Object} args
 * @param {DistributionStats} args.pattern
 * @param {Unit} args.unit
 * @param {string} [args.title]
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function percentileSeries({ pattern, unit, title = "" }) {
  const { stat } = colors;
  return [
    line({ series: pattern.max, name: `${title} max`.trim(), color: stat.max, unit }),
    line({ series: pattern.pct90, name: `${title} pct90`.trim(), color: stat.pct90, unit }),
    line({ series: pattern.pct75, name: `${title} pct75`.trim(), color: stat.pct75, unit }),
    line({ series: pattern.median, name: `${title} median`.trim(), color: stat.median, unit }),
    line({ series: pattern.pct25, name: `${title} pct25`.trim(), color: stat.pct25, unit }),
    line({ series: pattern.pct10, name: `${title} pct10`.trim(), color: stat.pct10, unit }),
    line({ series: pattern.min, name: `${title} min`.trim(), color: stat.min, unit }),
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
export function dashed(args) {
  const _args = /** @type {Parameters<typeof line>[0]} */ (args);
  _args.style = 2;
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
}) {
  const stats = statsAtWindow(pattern, window);
  return [
    dots({
      series: pattern.height,
      name: title || "base",
      color: baseColor,
      unit,
    }),
    ...percentileSeries({ pattern: stats, unit, title }),
  ];
}

/**
 * Extract stats at a specific rolling window
 * @param {Record<string, any>} pattern - Pattern with pct10/pct25/pct75/pct90 and median/max/min as _1y24h30d7dPattern
 * @param {string} window
 */
export function statsAtWindow(pattern, window) {
  return {
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
 * @param {string} args.title - Compare chart title
 * @param {(w: typeof ROLLING_WINDOWS[number]) => string} args.windowTitle - Individual window chart title
 * @param {Unit} args.unit
 * @param {string} args.name
 * @param {(args: {series: AnySeriesPattern, name: string, color: Color, unit: Unit}) => AnyFetchedSeriesBlueprint} [args.series]
 * @returns {PartialOptionsGroup}
 */
function rollingWindowsTree({
  windows,
  title,
  windowTitle,
  unit,
  name,
  series = line,
}) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title,
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
        title: windowTitle(w),
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
 * Flat array of rolling sum charts (one per window)
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.windows
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialChartOption[]}
 */
export function sumsArray({ windows, title, unit }) {
  return ROLLING_WINDOWS.map((w) => ({
    name: w.name,
    title: `${title} ${w.title} Sum`,
    bottom: [
      line({ series: windows[w.key], name: w.name, color: w.color, unit }),
    ],
  }));
}

/**
 * Generic helper: compare + per-window sum+avg + cumulative.
 * @template P
 * @param {Object} args
 * @param {{ _24h: P, _1w: P, _1m: P, _1y: P }} args.sum
 * @param {{ _24h: P, _1w: P, _1m: P, _1y: P }} args.average
 * @param {P} args.cumulative
 * @param {string} args.title
 * @param {Color} [args.color]
 * @param {(args: { pattern: P, name: string, color?: Color, defaultActive?: boolean }) => AnyFetchedSeriesBlueprint[]} args.series
 * @returns {PartialChartOption[]}
 */
export function sumsAndAveragesCumulativeWith({
  sum,
  average,
  cumulative,
  title,
  color,
  series,
}) {
  return [
    {
      name: "Compare",
      title: `${title} Averages`,
      bottom: ROLLING_WINDOWS.flatMap((w) =>
        series({
          pattern: average[w.key],
          name: w.name,
          color: w.color,
        }),
      ),
    },
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: `${title} ${w.title}`,
      bottom: [
        ...series({ pattern: sum[w.key], name: "Sum", color: w.color }),
        ...series({
          pattern: average[w.key],
          name: "Avg",
          color: w.color,
          defaultActive: false,
        }),
      ],
    })),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: series({ pattern: cumulative, name: "all-time", color }),
    },
  ];
}

/**
 * Flat array of per-window charts with both sum (active) and average (off by default)
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.sum
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.average
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialChartOption[]}
 */
export function sumsAndAveragesArray({ sum, average, title, unit }) {
  return ROLLING_WINDOWS.map((w) => ({
    name: w.name,
    title: `${title} ${w.title}`,
    bottom: [
      line({ series: sum[w.key], name: "Sum", color: w.color, unit }),
      line({
        series: average[w.key],
        name: "Avg",
        color: w.color,
        unit,
        defaultActive: false,
      }),
    ],
  }));
}

/**
 * Compare + windowed sum+avg + cumulative (single unit)
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.sum
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.average
 * @param {AnySeriesPattern} args.cumulative
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @returns {PartialChartOption[]}
 */
export function sumsAndAveragesCumulative({ sum, average, cumulative, title, unit, color }) {
  return sumsAndAveragesCumulativeWith({
    sum,
    average,
    cumulative,
    title,
    color,
    series: ({ pattern, name, color, defaultActive }) => [
      line({ series: pattern, name, color, unit, defaultActive }),
    ],
  });
}

/**
 * Rolling sums tree (Compare + individual windows in a folder)
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.windows
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {(args: {series: AnySeriesPattern, name: string, color: Color, unit: Unit}) => AnyFetchedSeriesBlueprint} [args.series]
 * @returns {PartialOptionsGroup}
 */
export function sumsTree({ windows, title, unit, series }) {
  return rollingWindowsTree({
    windows,
    title,
    windowTitle: (w) => `${title} ${w.title} Sum`,
    unit,
    name: "Sums",
    ...(series ? { series } : {}),
  });
}

/**
 * Flat array of per-window average charts
 * @param {Object} args
 * @param {{ _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }} args.windows
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialChartOption[]}
 */
export function averagesArray({ windows, title, unit }) {
  return [
    {
      name: "Compare",
      title: `${title} Averages`,
      bottom: ROLLING_WINDOWS.map((w) =>
        line({ series: windows[w.key], name: w.name, color: w.color, unit }),
      ),
    },
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: `${title} ${w.title} Average`,
      bottom: [
        line({ series: windows[w.key], name: w.name, color: w.color, unit }),
      ],
    })),
  ];
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
    name: "Distribution",
    tree: [
      {
        name: "Compare",
        title: `${title} Median`,
        bottom: ROLLING_WINDOWS.map((w) =>
          line({
            series: pattern.median[w.key],
            name: w.name,
            color: w.color,
            unit,
          }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} Distribution (${w.title})`,
        bottom: [
          ...(base ? [line({ series: base, name: "base", unit })] : []),
          ...percentileSeries({ pattern: statsAtWindow(pattern, w.key), unit }),
        ],
      })),
    ],
  };
}

/**
 * Map a rolling window slot's stats to a specific unit, producing a stats-compatible pattern
 * @param {{ median: Record<string, AnySeriesPattern>, max: Record<string, AnySeriesPattern>, min: Record<string, AnySeriesPattern>, pct75: Record<string, AnySeriesPattern>, pct25: Record<string, AnySeriesPattern>, pct90: Record<string, AnySeriesPattern>, pct10: Record<string, AnySeriesPattern> }} slot - Rolling window slot with multi-currency stats
 * @param {BtcSatsUsdKey} unitKey
 */
function rollingSlotForUnit(slot, unitKey) {
  return {
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
 * @param {{ median: Record<string, AnySeriesPattern>, max: Record<string, AnySeriesPattern>, min: Record<string, AnySeriesPattern>, pct75: Record<string, AnySeriesPattern>, pct25: Record<string, AnySeriesPattern>, pct90: Record<string, AnySeriesPattern>, pct10: Record<string, AnySeriesPattern> }} slot - Rolling window slot with multi-currency stats
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export const distributionBtcSatsUsd = (slot) => [
  ...percentileSeries({
    pattern: rollingSlotForUnit(slot, "btc"),
    unit: Unit.btc,
  }),
  ...percentileSeries({
    pattern: rollingSlotForUnit(slot, "sats"),
    unit: Unit.sats,
  }),
  ...percentileSeries({
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
    line({
      series: pattern.percent,
      name,
      color,
      defaultActive,
      unit: Unit.percentage,
    }),
    line({
      series: pattern.ratio,
      name,
      color,
      defaultActive,
      unit: Unit.ratio,
    }),
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
    dots({
      series: pattern.percent,
      name,
      color,
      defaultActive,
      unit: Unit.percentage,
    }),
    dots({
      series: pattern.ratio,
      name,
      color,
      defaultActive,
      unit: Unit.ratio,
    }),
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
export function percentRatioBaseline({ pattern, name, color, defaultActive }) {
  return [
    baseline({
      series: pattern.percent,
      name,
      color,
      defaultActive,
      unit: Unit.percentage,
    }),
    baseline({
      series: pattern.ratio,
      name,
      color,
      defaultActive,
      unit: Unit.ratio,
    }),
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
export function rollingPercentRatioTree({
  windows,
  title,
  name = "Sums",
  series = percentRatio,
}) {
  return {
    name,
    tree: [
      {
        name: "Compare",
        title: `${title} Rolling`,
        bottom: ROLLING_WINDOWS.flatMap((w) =>
          percentRatio({
            pattern: windows[w.key],
            name: w.name,
            color: w.color,
          }),
        ),
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} (${w.title})`,
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
          title: `${title} Change (${w.title})`,
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
    rollingPercentRatioTree({
      windows: delta.rate,
      title: `${title} Growth Rate`,
      name: "Growth Rate",
      series: percentRatioBaseline,
    }),
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
 * Split flat per-block pattern into charts (Averages/Sums/Distribution/Cumulative)
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
    ...sumsAndAveragesCumulative({
      sum: pattern.sum,
      average: pattern.average,
      cumulative: pattern.cumulative,
      title,
      unit,
    }),
    distributionWindowsTree({ pattern, title: distTitle, unit }),
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
 * @param {AggregatedPattern} args.pattern
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
  const distTitle = distributionSuffix
    ? `${title} ${distributionSuffix}`
    : title;
  return [
    ...sumsAndAveragesCumulative({
      sum: pattern.rolling.sum,
      average: pattern.rolling.average,
      cumulative: pattern.cumulative,
      title,
      unit,
    }),
    distributionWindowsTree({
      pattern: pattern.rolling,
      title: distTitle,
      unit,
    }),
  ];
}

/**
 * Split pattern into 3 charts with "per Block" in distribution title (no base)
 * @param {Object} args
 * @param {AggregatedPattern} args.pattern
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
      bottom: percentileSeries({ pattern: pattern.block, unit }),
    },
    {
      name: "Hourly",
      title: `${title} (Hourly)`,
      bottom: percentileSeries({ pattern: pattern._6b, unit }),
    },
  ];
}

/**
 * Averages + Sums + Cumulative charts
 * @param {Object} args
 * @param {CountPattern<any>} args.pattern
 * @param {string} args.title
 * @param {Unit} args.unit
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function chartsFromCount({ pattern, title, unit, color }) {
  return sumsAndAveragesCumulative({
    sum: pattern.sum,
    average: pattern.average,
    cumulative: pattern.cumulative,
    title,
    unit,
    color,
  });
}

/**
 * Windowed sums + cumulative for multiple named entries (e.g. transaction versions)
 * @param {Object} args
 * @param {Array<[string, CountPattern<any>]>} args.entries
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
/**
 * Windowed sums + cumulative for multiple named entries (e.g. transaction versions)
 * @param {Object} args
 * @param {Array<[string, CountPattern<any>]>} args.entries
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function chartsFromCountEntries({ entries, title, unit }) {
  const items = entries.map(([name, data], i, arr) => ({
    name,
    color: colors.at(i, arr.length),
    sum: data.sum,
    cumulative: data.cumulative,
  }));
  return [
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: `${title} ${w.title} Sum`,
      bottom: items.map((e) =>
        line({ series: e.sum[w.key], name: e.name, color: e.color, unit }),
      ),
    })),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: items.map((e) =>
        line({ series: e.cumulative, name: e.name, color: e.color, unit }),
      ),
    },
  ];
}

/**
 * Windowed averages + sums + cumulative for multiple named series (e.g. UTXO flow)
 * @param {Object} args
 * @param {Array<{ name: string, color: Color, average: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, sum: { _24h: AnySeriesPattern, _1w: AnySeriesPattern, _1m: AnySeriesPattern, _1y: AnySeriesPattern }, cumulative: AnySeriesPattern }>} args.entries
 * @param {string} args.title
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function multiSeriesTree({ entries, title, unit }) {
  return [
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: `${title} ${w.title} Averages`,
      bottom: entries.map((e) =>
        line({ series: e.average[w.key], name: e.name, color: e.color, unit }),
      ),
    })),
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: entries.map((e) =>
        line({ series: e.cumulative, name: e.name, color: e.color, unit }),
      ),
    },
  ];
}

