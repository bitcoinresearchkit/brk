/** Shared helpers for options */

import { Unit } from "../utils/units.js";
import { line, baseline, price, ROLLING_WINDOWS } from "./series.js";
import { priceLine, priceLines } from "./constants.js";
import { colors } from "../utils/colors.js";

// ============================================================================
// Grouped Cohort Helpers
// ============================================================================

/**
 * Map cohorts to series (without "all" cohort)
 * Use for charts where "all" doesn't have required properties
 * @template T
 * @template R
 * @param {readonly T[]} list
 * @param {(item: T) => R} fn
 * @returns {R[]}
 */
export function mapCohorts(list, fn) {
  return list.map(fn);
}

/**
 * FlatMap cohorts to series (without "all" cohort)
 * Use for charts where "all" doesn't have required properties
 * @template T
 * @template R
 * @param {readonly T[]} list
 * @param {(item: T) => R[]} fn
 * @returns {R[]}
 */
export function flatMapCohorts(list, fn) {
  return list.flatMap(fn);
}

/**
 * Map cohorts to series, with "all" cohort added as defaultActive: false
 * @template T
 * @template A
 * @template R
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(item: T | A) => R} fn
 * @returns {R[]}
 */
export function mapCohortsWithAll(list, all, fn) {
  return [
    ...list.map(fn),
    { ...fn({ ...all, name: "All" }), defaultActive: false },
  ];
}

/**
 * FlatMap cohorts to series, with "all" cohort added as defaultActive: false
 * @template T
 * @template A
 * @template R
 * @param {readonly T[]} list
 * @param {A} all
 * @param {(item: T | A) => R[]} fn
 * @returns {R[]}
 */
export function flatMapCohortsWithAll(list, all, fn) {
  return [
    ...list.flatMap(fn),
    ...fn({ ...all, name: "All" }).map((s) => ({ ...s, defaultActive: false })),
  ];
}

/**
 * Create a title formatter for chart titles
 * @param {string} [cohortTitle]
 * @returns {(metric: string) => string}
 */
export const formatCohortTitle = (cohortTitle) => (metric) =>
  cohortTitle ? `${metric}: ${cohortTitle}` : metric;

/**
 * Create sats/btc/usd line series from a pattern with .sats/.btc/.usd
 * @param {Object} args
 * @param {AnyValuePattern} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @param {number} [args.style]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsd({ pattern, name, color, defaultActive, style }) {
  return [
    line({
      metric: pattern.btc,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
      style,
    }),
    line({
      metric: pattern.sats,
      name,
      color,
      unit: Unit.sats,
      defaultActive,
      style,
    }),
    line({
      metric: pattern.usd,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
      style,
    }),
  ];
}

/**
 * Create sats/btc/usd baseline series from a value pattern
 * @param {Object} args
 * @param {{ btc: AnyMetricPattern, sats: AnyMetricPattern, usd: AnyMetricPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedBaselineSeriesBlueprint[]}
 */
export function satsBtcUsdBaseline({ pattern, name, color, defaultActive }) {
  return [
    baseline({
      metric: pattern.btc,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
    }),
    baseline({
      metric: pattern.sats,
      name,
      color,
      unit: Unit.sats,
      defaultActive,
    }),
    baseline({
      metric: pattern.usd,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
    }),
  ];
}

/**
 * Create sats/btc/usd series from any value pattern using base or cumulative key
 * @param {Object} args
 * @param {{ base: AnyValuePattern, cumulative: AnyValuePattern }} args.source
 * @param {'base' | 'cumulative'} args.key
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsdFrom({ source, key, name, color, defaultActive }) {
  return satsBtcUsd({
    pattern: source[key],
    name,
    color,
    defaultActive,
  });
}


/**
 * Create coinbase/subsidy/fee series from separate sources
 * @param {Object} args
 * @param {{ base: AnyValuePattern, cumulative: AnyValuePattern }} args.coinbase
 * @param {{ base: AnyValuePattern, cumulative: AnyValuePattern }} args.subsidy
 * @param {{ base: AnyValuePattern, cumulative: AnyValuePattern }} args.fee
 * @param {'base' | 'cumulative'} args.key
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function revenueBtcSatsUsd({ coinbase, subsidy, fee, key }) {
  return [
    ...satsBtcUsdFrom({
      source: coinbase,
      key,
      name: "Coinbase",
      color: colors.mining.coinbase,
    }),
    ...satsBtcUsdFrom({
      source: subsidy,
      key,
      name: "Subsidy",
      color: colors.mining.subsidy,
    }),
    ...satsBtcUsdFrom({
      source: fee,
      key,
      name: "Fees",
      color: colors.mining.fee,
    }),
  ];
}

/**
 * Create sats/btc/usd series from a rolling window (24h/7d/30d/1y sum)
 * @param {Object} args
 * @param {AnyValuePattern} args.pattern - A BtcSatsUsdPattern (e.g., source.rolling._24h.sum)
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsdRolling({ pattern, name, color, defaultActive }) {
  return satsBtcUsd({ pattern, name, color, defaultActive });
}

/**
 * Build a full Sum / Rolling / Cumulative tree from a FullValuePattern
 * @param {Object} args
 * @param {FullValuePattern} args.pattern
 * @param {string} args.name
 * @param {string} args.title
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function satsBtcUsdFullTree({ pattern, name, title, color }) {
  return [
    {
      name: "Sum",
      title,
      bottom: satsBtcUsd({ pattern: pattern.base, name, color }),
    },
    {
      name: "Rolling",
      tree: [
        {
          name: "Compare",
          title: `${title} Rolling Sum`,
          bottom: ROLLING_WINDOWS.flatMap((w) =>
            satsBtcUsd({
              pattern: pattern.sum[w.key],
              name: w.name,
              color: w.color,
            }),
          ),
        },
        ...ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `${title} ${w.name} Rolling Sum`,
          bottom: satsBtcUsd({
            pattern: pattern.sum[w.key],
            name: w.name,
            color: w.color,
          }),
        })),
      ],
    },
    {
      name: "Cumulative",
      title: `${title} (Total)`,
      bottom: satsBtcUsd({
        pattern: pattern.cumulative,
        name: "all-time",
        color,
      }),
    },
  ];
}

/**
 * Create Price + Ratio charts from a simple price pattern (BpsCentsRatioSatsUsdPattern)
 * @param {Object} args
 * @param {AnyPricePattern & { ratio: AnyMetricPattern }} args.pattern
 * @param {string} args.title
 * @param {string} args.legend
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function simplePriceRatioTree({ pattern, title, legend, color }) {
  return [
    {
      name: "Price",
      title,
      top: [price({ metric: pattern, name: legend, color })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [price({ metric: pattern, name: legend, color })],
      bottom: [
        baseline({
          metric: pattern.ratio,
          name: "Ratio",
          unit: Unit.ratio,
          base: 1,
        }),
      ],
    },
  ];
}

/**
 * Create Price + Ratio charts with percentile bands (no SMAs/z-scores)
 * @param {Object} args
 * @param {PriceRatioPercentilesPattern} args.pattern
 * @param {string} args.title
 * @param {string} args.legend
 * @param {Color} [args.color]
 * @param {FetchedPriceSeriesBlueprint[]} [args.priceReferences]
 * @returns {PartialOptionsTree}
 */
export function priceRatioPercentilesTree({
  pattern,
  title,
  legend,
  color,
  priceReferences,
}) {
  const p = pattern.percentiles;
  const pctUsd = [
    { name: "pct95", prop: p.pct95.price, color: colors.ratioPct._95 },
    { name: "pct5", prop: p.pct5.price, color: colors.ratioPct._5 },
    { name: "pct98", prop: p.pct98.price, color: colors.ratioPct._98 },
    { name: "pct2", prop: p.pct2.price, color: colors.ratioPct._2 },
    { name: "pct99", prop: p.pct99.price, color: colors.ratioPct._99 },
    { name: "pct1", prop: p.pct1.price, color: colors.ratioPct._1 },
  ];
  const pctRatio = [
    { name: "pct95", prop: p.pct95.ratio, color: colors.ratioPct._95 },
    { name: "pct5", prop: p.pct5.ratio, color: colors.ratioPct._5 },
    { name: "pct98", prop: p.pct98.ratio, color: colors.ratioPct._98 },
    { name: "pct2", prop: p.pct2.ratio, color: colors.ratioPct._2 },
    { name: "pct99", prop: p.pct99.ratio, color: colors.ratioPct._99 },
    { name: "pct1", prop: p.pct1.ratio, color: colors.ratioPct._1 },
  ];
  return [
    {
      name: "Price",
      title,
      top: [
        price({ metric: pattern, name: legend, color }),
        ...(priceReferences ?? []),
        ...pctUsd.map(({ name, prop, color }) =>
          price({
            metric: prop,
            name,
            color,
            defaultActive: false,
            options: { lineStyle: 1 },
          }),
        ),
      ],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        price({ metric: pattern, name: legend, color }),
        ...pctUsd.map(({ name, prop, color }) =>
          price({
            metric: prop,
            name,
            color,
            defaultActive: false,
            options: { lineStyle: 1 },
          }),
        ),
      ],
      bottom: [
        baseline({
          metric: pattern.ratio,
          name: "Ratio",
          unit: Unit.ratio,
          base: 1,
        }),
        ...pctRatio.map(({ name, prop, color }) =>
          line({
            metric: prop,
            name,
            color,
            defaultActive: false,
            unit: Unit.ratio,
            options: { lineStyle: 1 },
          }),
        ),
      ],
    },
  ];
}

/**
 * Create grouped Price + Ratio charts overlaying multiple series
 * @param {Object} args
 * @param {{ name: string, color?: Color, pattern: AnyPricePattern & { ratio: AnyMetricPattern } }[]} args.list
 * @param {string} args.title
 * @returns {PartialOptionsTree}
 */
export function groupedSimplePriceRatioTree({ list, title }) {
  return [
    {
      name: "Price",
      title,
      top: list.map(({ name, color, pattern }) =>
        price({ metric: pattern, name, color }),
      ),
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      bottom: list.map(({ name, color, pattern }) =>
        baseline({
          metric: pattern.ratio,
          name,
          color,
          unit: Unit.ratio,
          base: 1,
        }),
      ),
    },
  ];
}

/**
 * Create coinbase/subsidy/fee rolling sum series from separate sources
 * @param {Object} args
 * @param {AnyValuePattern} args.coinbase - Rolling sum pattern (e.g., mining.rewards.coinbase.rolling._24h.sum)
 * @param {AnyValuePattern} args.subsidy
 * @param {AnyValuePattern} args.fee
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function revenueRollingBtcSatsUsd({ coinbase, subsidy, fee }) {
  return [
    ...satsBtcUsd({
      pattern: coinbase,
      name: "Coinbase",
      color: colors.mining.coinbase,
    }),
    ...satsBtcUsd({
      pattern: subsidy,
      name: "Subsidy",
      color: colors.mining.subsidy,
    }),
    ...satsBtcUsd({
      pattern: fee,
      name: "Fees",
      color: colors.mining.fee,
    }),
  ];
}

/**
 * Build percentile USD mappings from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function percentileUsdMap(ratio) {
  const p = ratio.percentiles;
  return /** @type {const} */ ([
    { name: "pct95", prop: p.pct95.price, color: colors.ratioPct._95 },
    { name: "pct5", prop: p.pct5.price, color: colors.ratioPct._5 },
    { name: "pct98", prop: p.pct98.price, color: colors.ratioPct._98 },
    { name: "pct2", prop: p.pct2.price, color: colors.ratioPct._2 },
    { name: "pct99", prop: p.pct99.price, color: colors.ratioPct._99 },
    { name: "pct1", prop: p.pct1.price, color: colors.ratioPct._1 },
  ]);
}

/**
 * Build percentile ratio mappings from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function percentileMap(ratio) {
  const p = ratio.percentiles;
  return /** @type {const} */ ([
    { name: "pct95", prop: p.pct95.ratio, color: colors.ratioPct._95 },
    { name: "pct5", prop: p.pct5.ratio, color: colors.ratioPct._5 },
    { name: "pct98", prop: p.pct98.ratio, color: colors.ratioPct._98 },
    { name: "pct2", prop: p.pct2.ratio, color: colors.ratioPct._2 },
    { name: "pct99", prop: p.pct99.ratio, color: colors.ratioPct._99 },
    { name: "pct1", prop: p.pct1.ratio, color: colors.ratioPct._1 },
  ]);
}

/**
 * Build SD patterns from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function sdPatterns(ratio) {
  return /** @type {const} */ ([
    {
      nameAddon: "All Time",
      titleAddon: "",
      sd: ratio.stdDev.all,
      smaRatio: ratio.sma.all.ratio,
    },
    {
      nameAddon: "4y",
      titleAddon: "4y",
      sd: ratio.stdDev._4y,
      smaRatio: ratio.sma._4y.ratio,
    },
    {
      nameAddon: "2y",
      titleAddon: "2y",
      sd: ratio.stdDev._2y,
      smaRatio: ratio.sma._2y.ratio,
    },
    {
      nameAddon: "1y",
      titleAddon: "1y",
      sd: ratio.stdDev._1y,
      smaRatio: ratio.sma._1y.ratio,
    },
  ]);
}

/**
 * Build SD band mappings from an SD pattern
 * @param {Ratio1ySdPattern} sd
 */
export function sdBandsUsd(sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd._0sd, color: colors.sd._0 },
    { name: "+0.5σ", prop: sd.p05sd.price, color: colors.sd.p05 },
    { name: "−0.5σ", prop: sd.m05sd.price, color: colors.sd.m05 },
    { name: "+1σ", prop: sd.p1sd.price, color: colors.sd.p1 },
    { name: "−1σ", prop: sd.m1sd.price, color: colors.sd.m1 },
    { name: "+1.5σ", prop: sd.p15sd.price, color: colors.sd.p15 },
    { name: "−1.5σ", prop: sd.m15sd.price, color: colors.sd.m15 },
    { name: "+2σ", prop: sd.p2sd.price, color: colors.sd.p2 },
    { name: "−2σ", prop: sd.m2sd.price, color: colors.sd.m2 },
    { name: "+2.5σ", prop: sd.p25sd.price, color: colors.sd.p25 },
    { name: "−2.5σ", prop: sd.m25sd.price, color: colors.sd.m25 },
    { name: "+3σ", prop: sd.p3sd.price, color: colors.sd.p3 },
    { name: "−3σ", prop: sd.m3sd.price, color: colors.sd.m3 },
  ]);
}

/**
 * Build SD band mappings (ratio) from an SD pattern
 * @param {Ratio1ySdPattern} sd
 * @param {AnyMetricPattern} smaRatio
 */
export function sdBandsRatio(sd, smaRatio) {
  return /** @type {const} */ ([
    { name: "0σ", prop: smaRatio, color: colors.sd._0 },
    { name: "+0.5σ", prop: sd.p05sd.ratio, color: colors.sd.p05 },
    { name: "−0.5σ", prop: sd.m05sd.ratio, color: colors.sd.m05 },
    { name: "+1σ", prop: sd.p1sd.ratio, color: colors.sd.p1 },
    { name: "−1σ", prop: sd.m1sd.ratio, color: colors.sd.m1 },
    { name: "+1.5σ", prop: sd.p15sd.ratio, color: colors.sd.p15 },
    { name: "−1.5σ", prop: sd.m15sd.ratio, color: colors.sd.m15 },
    { name: "+2σ", prop: sd.p2sd.ratio, color: colors.sd.p2 },
    { name: "−2σ", prop: sd.m2sd.ratio, color: colors.sd.m2 },
    { name: "+2.5σ", prop: sd.p25sd.ratio, color: colors.sd.p25 },
    { name: "−2.5σ", prop: sd.m25sd.ratio, color: colors.sd.m25 },
    { name: "+3σ", prop: sd.p3sd.ratio, color: colors.sd.p3 },
    { name: "−3σ", prop: sd.m3sd.ratio, color: colors.sd.m3 },
  ]);
}

/**
 * Build ratio SMA series from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function ratioSmas(ratio) {
  return [
    { name: "1w SMA", metric: ratio.sma._1w.ratio },
    { name: "1m SMA", metric: ratio.sma._1m.ratio },
    { name: "1y SMA", metric: ratio.sma._1y.ratio },
    { name: "2y SMA", metric: ratio.sma._2y.ratio },
    { name: "4y SMA", metric: ratio.sma._4y.ratio },
    { name: "All SMA", metric: ratio.sma.all.ratio, color: colors.time.all },
  ].map((s, i, arr) => ({ color: colors.at(i, arr.length), ...s }));
}

/**
 * Create ratio chart from ActivePriceRatioPattern
 * @param {Object} args
 * @param {(metric: string) => string} args.title
 * @param {AnyPricePattern} args.pricePattern - The price pattern to show in top pane
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @param {string} [args.name] - Optional name override (default: "ratio")
 * @returns {PartialChartOption}
 */
export function createRatioChart({ title, pricePattern, ratio, color, name }) {
  return {
    name: name ?? "ratio",
    title: title(name ?? "Ratio"),
    top: [
      price({ metric: pricePattern, name: "Price", color }),
      ...percentileUsdMap(ratio).map(({ name, prop, color }) =>
        price({
          metric: prop,
          name,
          color,
          defaultActive: false,
          options: { lineStyle: 1 },
        }),
      ),
    ],
    bottom: [
      baseline({
        metric: ratio.ratio,
        name: "Ratio",
        unit: Unit.ratio,
        base: 1,
      }),
      ...ratioSmas(ratio).map(({ name, metric, color }) =>
        line({ metric, name, color, unit: Unit.ratio, defaultActive: false }),
      ),
      ...percentileMap(ratio).map(({ name, prop, color }) =>
        line({
          metric: prop,
          name,
          color,
          defaultActive: false,
          unit: Unit.ratio,
          options: { lineStyle: 1 },
        }),
      ),
    ],
  };
}

/**
 * Create ZScores folder from ActivePriceRatioPattern
 * @param {Object} args
 * @param {(suffix: string) => string} args.formatTitle - Function that takes metric suffix and returns full title
 * @param {string} args.legend
 * @param {AnyPricePattern} args.pricePattern - The price pattern to show in top pane
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @returns {PartialOptionsGroup}
 */
export function createZScoresFolder({
  formatTitle,
  legend,
  pricePattern,
  ratio,
  color,
}) {
  const sdPats = sdPatterns(ratio);

  const zscorePeriods = [
    { name: "1y", sd: ratio.stdDev._1y },
    { name: "2y", sd: ratio.stdDev._2y },
    { name: "4y", sd: ratio.stdDev._4y },
    { name: "all", sd: ratio.stdDev.all, color: colors.time.all },
  ].map((s, i, arr) => ({ color: colors.at(i, arr.length), ...s }));

  return {
    name: "Z-Scores",
    tree: [
      {
        name: "Compare",
        title: formatTitle("Z-Scores"),
        top: [
          price({ metric: pricePattern, name: legend, color }),
          ...zscorePeriods.map((p) =>
            price({
              metric: p.sd._0sd,
              name: `${p.name} 0σ`,
              color: p.color,
              defaultActive: false,
            }),
          ),
        ],
        bottom: [
          ...zscorePeriods.reverse().map((p) =>
            line({
              metric: p.sd.zscore,
              name: p.name,
              color: p.color,
              unit: Unit.sd,
            }),
          ),
          ...priceLines({
            unit: Unit.sd,
            numbers: [0, 1, -1, 2, -2, 3, -3],
            defaultActive: false,
          }),
        ],
      },
      ...sdPats.map(({ nameAddon, titleAddon, sd, smaRatio }) => {
        const prefix = titleAddon ? `${titleAddon} ` : "";
        const topPrice = price({ metric: pricePattern, name: legend, color });
        return {
          name: nameAddon,
          tree: [
            {
              name: "Score",
              title: formatTitle(`${prefix}Z-Score`),
              top: [
                topPrice,
                ...sdBandsUsd(sd).map(
                  ({ name: bandName, prop, color: bandColor }) =>
                    price({
                      metric: prop,
                      name: bandName,
                      color: bandColor,
                      defaultActive: false,
                    }),
                ),
              ],
              bottom: [
                baseline({
                  metric: sd.zscore,
                  name: "Z-Score",
                  unit: Unit.sd,
                }),
                priceLine({
                  unit: Unit.sd,
                }),
                ...priceLines({
                  unit: Unit.sd,
                  numbers: [1, -1, 2, -2, 3, -3],
                  defaultActive: false,
                }),
              ],
            },
            {
              name: "Ratio",
              title: formatTitle(`${prefix}Ratio`),
              top: [topPrice],
              bottom: [
                baseline({
                  metric: ratio.ratio,
                  name: "Ratio",
                  unit: Unit.ratio,
                  base: 1,
                }),
                ...sdBandsRatio(sd, smaRatio).map(
                  ({ name: bandName, prop, color: bandColor }) =>
                    line({
                      metric: prop,
                      name: bandName,
                      color: bandColor,
                      unit: Unit.ratio,
                      defaultActive: false,
                    }),
                ),
              ],
            },
            {
              name: "Volatility",
              title: formatTitle(`${prefix}Volatility`),
              top: [topPrice],
              bottom: [
                line({
                  metric: sd.sd,
                  name: "Volatility",
                  color: colors.gray,
                  unit: Unit.percentage,
                }),
              ],
            },
          ],
        };
      }),
    ],
  };
}

/**
 * Create price + ratio + z-scores charts - flat array
 * Unified helper for averages, distribution, and other price-based metrics
 * @param {Object} args
 * @param {string} args.context - Context string for ratio/z-scores titles (e.g., "1 Week SMA", "STH")
 * @param {string} args.legend - Legend name for the price series
 * @param {AnyPricePattern} args.pricePattern - The price pattern
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @param {string} [args.priceTitle] - Optional override for price chart title (default: context)
 * @param {string} [args.titlePrefix] - Optional prefix for ratio/z-scores titles (e.g., "Realized Price" gives "Realized Price Ratio: STH")
 * @param {FetchedPriceSeriesBlueprint[]} [args.priceReferences] - Optional additional price series to show in Price chart
 * @returns {PartialOptionsTree}
 */
export function createPriceRatioCharts({
  context,
  legend,
  pricePattern,
  ratio,
  color,
  priceTitle,
  titlePrefix,
  priceReferences,
}) {
  const titleFn = formatCohortTitle(context);
  return [
    {
      name: "Price",
      title: priceTitle ?? context,
      top: [
        price({ metric: pricePattern, name: legend, color }),
        ...(priceReferences ?? []),
      ],
    },
    createRatioChart({
      title: (name) => titleFn(titlePrefix ? `${titlePrefix} ${name}` : name),
      pricePattern,
      ratio,
      color,
    }),
    createZScoresFolder({
      formatTitle: (name) =>
        titleFn(titlePrefix ? `${titlePrefix} ${name}` : name),
      legend,
      pricePattern,
      ratio,
      color,
    }),
  ];
}
