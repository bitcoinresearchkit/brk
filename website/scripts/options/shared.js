/** Shared helpers for options */

import { Unit } from "../utils/units.js";
import {
  ROLLING_WINDOWS,
  line,
  baseline,
  price,
  sumsAndAveragesCumulativeWith,
} from "./series.js";
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
 * @returns {(name: string) => string}
 */
export const formatCohortTitle = (cohortTitle) => (name) =>
  cohortTitle ? `${name}: ${cohortTitle}` : name;

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
      series: pattern.btc,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
      style,
    }),
    line({
      series: pattern.sats,
      name,
      color,
      unit: Unit.sats,
      defaultActive,
      style,
    }),
    line({
      series: pattern.usd,
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
 * @param {{ btc: AnySeriesPattern, sats: AnySeriesPattern, usd: AnySeriesPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedBaselineSeriesBlueprint[]}
 */
export function satsBtcUsdBaseline({ pattern, name, color, defaultActive }) {
  return [
    baseline({
      series: pattern.btc,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
    }),
    baseline({
      series: pattern.sats,
      name,
      color,
      unit: Unit.sats,
      defaultActive,
    }),
    baseline({
      series: pattern.usd,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
    }),
  ];
}

/**
 * Create sats/btc/usd series from a value pattern's cumulative
 * @param {Object} args
 * @param {{ cumulative: AnyValuePattern }} args.source
 * @param {'cumulative'} args.key
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
 * @param {{ cumulative: AnyValuePattern }} args.coinbase
 * @param {{ cumulative: AnyValuePattern }} args.subsidy
 * @param {{ cumulative: AnyValuePattern }} args.fee
 * @param {'cumulative'} args.key
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
 * Create sats/btc/usd series from a rolling window (24h/1w/1m/1y sum)
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
 * @param {(metric: string) => string} [args.title]
 * @param {string} args.metric
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function satsBtcUsdFullTree({ pattern, title, metric, color }) {
  return sumsAndAveragesCumulativeWith({
    sum: pattern.sum,
    average: pattern.average,
    cumulative: pattern.cumulative,
    title,
    metric,
    color,
    series: ({ pattern, name, color, defaultActive }) =>
      satsBtcUsd({ pattern, name, color, defaultActive }),
  });
}

/**
 * Create Price + Ratio charts from a simple price pattern (BpsCentsRatioSatsUsdPattern)
 * @param {Object} args
 * @param {AnyPricePattern & { ratio: AnySeriesPattern }} args.pattern
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
      top: [price({ series: pattern, name: legend, color })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [price({ series: pattern, name: legend, color })],
      bottom: [
        baseline({
          series: pattern.ratio,
          name: "Ratio",
          unit: Unit.ratio,
          base: 1,
        }),
      ],
    },
  ];
}

/**
 * @param {{ pct95: AnyPricePattern, pct5: AnyPricePattern, pct98: AnyPricePattern, pct2: AnyPricePattern, pct99: AnyPricePattern, pct1: AnyPricePattern, pct995: AnyPricePattern, pct05: AnyPricePattern }} p
 */
export function percentileBands(p) {
  return percentileBandsWith(p, (e) => e);
}

/**
 * @template E
 * @template T
 * @param {{ pct95: E, pct5: E, pct98: E, pct2: E, pct99: E, pct1: E, pct995: E, pct05: E }} p
 * @param {(entry: E) => T} extract
 */
export function percentileBandsWith(p, extract) {
  return [
    { name: "P95", prop: extract(p.pct95), color: colors.ratioPct._95 },
    { name: "P5", prop: extract(p.pct5), color: colors.ratioPct._5 },
    { name: "P98", prop: extract(p.pct98), color: colors.ratioPct._98 },
    { name: "P2", prop: extract(p.pct2), color: colors.ratioPct._2 },
    { name: "P99", prop: extract(p.pct99), color: colors.ratioPct._99 },
    { name: "P1", prop: extract(p.pct1), color: colors.ratioPct._1 },
    { name: "P99.5", prop: extract(p.pct995), color: colors.ratioPct._99_5 },
    { name: "P0.5", prop: extract(p.pct05), color: colors.ratioPct._0_5 },
  ];
}

/**
 * @param {{ name: string, prop: AnyPricePattern, color: Color }[]} bands
 * @param {{ defaultActive?: boolean }} [opts]
 */
export function priceBands(bands, opts) {
  return bands.map(({ name, prop, color }) =>
    price({
      series: prop,
      name,
      color,
      defaultActive: opts?.defaultActive ?? false,
      options: { lineStyle: 1 },
    }),
  );
}

/** @param {{ name: string, prop: AnySeriesPattern, color: Color }[]} bands */
function ratioBands(bands) {
  return bands.map(({ name, prop, color }) =>
    line({
      series: prop,
      name,
      color,
      defaultActive: false,
      unit: Unit.ratio,
      options: { lineStyle: 1 },
    }),
  );
}

/**
 * Price + Ratio charts with percentile bands
 * @param {Object} args
 * @param {PriceRatioPercentilesPattern} args.pattern
 * @param {string} args.title
 * @param {string} args.legend
 * @param {Color} [args.color]
 * @param {string} [args.ratioTitle]
 * @param {FetchedPriceSeriesBlueprint[]} [args.priceReferences]
 * @returns {PartialOptionsTree}
 */
export function priceRatioPercentilesTree({
  pattern,
  title,
  legend,
  color,
  ratioTitle,
  priceReferences,
}) {
  const p = pattern.percentiles;
  const pctUsd = percentileBandsWith(p, (e) => e.price);
  const pctRatio = percentileBandsWith(p, (e) => e.ratio);
  return [
    {
      name: "Price",
      title,
      top: [
        price({ series: pattern, name: legend, color }),
        ...(priceReferences ?? []),
        ...priceBands(pctUsd),
      ],
    },
    {
      name: "Ratio",
      title: ratioTitle ?? `${title} Ratio`,
      top: [
        price({ series: pattern, name: legend, color }),
        ...priceBands(pctUsd),
      ],
      bottom: [
        baseline({
          series: pattern.ratio,
          name: "Ratio",
          unit: Unit.ratio,
          base: 1,
        }),
        ...ratioBands(pctRatio),
      ],
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

/** @param {AnyRatioPattern} ratio */
export function percentileUsdMap(ratio) {
  return percentileBandsWith(ratio.percentiles, (e) => e.price);
}

/** @param {AnyRatioPattern} ratio */
export function percentileMap(ratio) {
  return percentileBandsWith(ratio.percentiles, (e) => e.ratio);
}

/**
 * Build SD patterns from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function sdPatterns(ratio) {
  return /** @type {const} */ ([
    {
      nameAddon: "All Time",
      titleAddon: "All Time",
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
 * @param {AnySeriesPattern} smaRatio
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
    { name: "1w SMA", series: ratio.sma._1w.ratio },
    { name: "1m SMA", series: ratio.sma._1m.ratio },
    { name: "1y SMA", series: ratio.sma._1y.ratio },
    { name: "2y SMA", series: ratio.sma._2y.ratio },
    { name: "4y SMA", series: ratio.sma._4y.ratio },
    {
      name: "All Time SMA",
      series: ratio.sma.all.ratio,
      color: colors.time.all,
    },
  ].map((s, i, arr) => ({ color: colors.at(i, arr.length), ...s }));
}

/**
 * Ratio bottom series: baseline + SMAs + percentiles
 * @param {AnyRatioPattern} ratio
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function ratioBottomSeries(ratio) {
  return [
    baseline({
      series: ratio.ratio,
      name: "Ratio",
      unit: Unit.ratio,
      base: 1,
    }),
    ...ratioSmas(ratio).map(({ name, series, color }) =>
      line({ series, name, color, unit: Unit.ratio, defaultActive: false }),
    ),
    ...percentileMap(ratio).map(({ name, prop, color }) =>
      line({
        series: prop,
        name,
        color,
        defaultActive: false,
        unit: Unit.ratio,
        options: { lineStyle: 1 },
      }),
    ),
  ];
}

/**
 * @param {Object} args
 * @param {(name: string) => string} args.title
 * @param {AnyPricePattern} args.pricePattern
 * @param {AnyRatioPattern} args.ratio
 * @param {Color} args.color
 * @param {string} [args.name]
 * @param {string} [args.legend]
 * @returns {PartialChartOption}
 */
export function createRatioChart({
  title,
  pricePattern,
  ratio,
  color,
  name,
  legend,
}) {
  return {
    name: name ?? "Ratio",
    title: title(name ?? "Ratio"),
    top: [
      price({ series: pricePattern, name: legend ?? "Price", color }),
      ...percentileUsdMap(ratio).map(({ name, prop, color }) =>
        price({
          series: prop,
          name,
          color,
          defaultActive: false,
          options: { lineStyle: 1 },
        }),
      ),
    ],
    bottom: ratioBottomSeries(ratio),
  };
}

/**
 * Create ZScores folder from ActivePriceRatioPattern
 * @param {Object} args
 * @param {(suffix: string) => string} args.formatTitle - Function that takes series suffix and returns full title
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
    { name: "All Time", sd: ratio.stdDev.all, color: colors.time.all },
  ].map((s, i, arr) => ({ color: colors.at(i, arr.length), ...s }));

  return {
    name: "Z-Scores",
    tree: [
      {
        name: "Compare",
        title: formatTitle("Z-Scores"),
        top: [
          price({ series: pricePattern, name: legend, color }),
          ...zscorePeriods.map((p) =>
            price({
              series: p.sd._0sd,
              name: `${p.name} 0σ`,
              color: p.color,
              defaultActive: false,
            }),
          ),
        ],
        bottom: [
          ...zscorePeriods.reverse().map((p) =>
            line({
              series: p.sd.zscore,
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
        const topPrice = price({ series: pricePattern, name: legend, color });
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
                      series: prop,
                      name: bandName,
                      color: bandColor,
                      defaultActive: false,
                    }),
                ),
              ],
              bottom: [
                baseline({
                  series: sd.zscore,
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
                  series: ratio.ratio,
                  name: "Ratio",
                  unit: Unit.ratio,
                  base: 1,
                }),
                ...sdBandsRatio(sd, smaRatio).map(
                  ({ name: bandName, prop, color: bandColor }) =>
                    line({
                      series: prop,
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
                  series: sd.sd,
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
 * Unified helper for averages, distribution, and other price-based series
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
  const pctUsd = percentileBandsWith(ratio.percentiles, (e) => e.price);
  return [
    {
      name: "Price",
      title: priceTitle ?? context,
      top: [
        price({ series: pricePattern, name: legend, color }),
        ...(priceReferences ?? []),
        ...priceBands(pctUsd),
      ],
    },
    createRatioChart({
      title: (name) => titleFn(titlePrefix ? `${titlePrefix} ${name}` : name),
      pricePattern,
      ratio,
      color,
      legend,
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

// ============================================================================
// Grouped Rolling Windows + Cumulative
// ============================================================================

/**
 * Generic: rolling window charts + cumulative for grouped cohorts
 * @template {{ name: string, color: Color }} T
 * @template {{ name: string, color: Color }} A
 * @param {Object} args
 * @param {readonly T[]} args.list
 * @param {A} args.all
 * @param {(name: string) => string} args.title
 * @param {string} args.metricTitle
 * @param {(c: T | A, windowKey: "_24h" | "_1w" | "_1m" | "_1y") => AnySeriesPattern} args.getWindowSeries
 * @param {(c: T | A) => AnySeriesPattern} args.getCumulativeSeries
 * @param {(args: { series: AnySeriesPattern, name: string, color: Color, unit: Unit }) => AnyFetchedSeriesBlueprint} args.seriesFn
 * @param {Unit} args.unit
 * @returns {PartialOptionsTree}
 */
export function groupedWindowsCumulative({
  list,
  all,
  title,
  metricTitle,
  getWindowSeries,
  getCumulativeSeries,
  seriesFn,
  unit,
}) {
  return [
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`${w.title} ${metricTitle}`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        seriesFn({
          series: getWindowSeries(c, w.key),
          name: c.name,
          color: c.color,
          unit,
        }),
      ),
    })),
    {
      name: "Cumulative",
      title: title(`Cumulative ${metricTitle}`),
      bottom: mapCohortsWithAll(list, all, (c) =>
        seriesFn({
          series: getCumulativeSeries(c),
          name: c.name,
          color: c.color,
          unit,
        }),
      ),
    },
  ];
}

/**
 * USD variant: windows access .sum[key].usd, cumulative accesses .cumulative.usd
 * @template {{ name: string, color: Color }} T
 * @template {{ name: string, color: Color }} A
 * @param {Object} args
 * @param {readonly T[]} args.list
 * @param {A} args.all
 * @param {(name: string) => string} args.title
 * @param {string} args.metricTitle
 * @param {(c: T | A) => { sum: Record<string, { usd: AnySeriesPattern }>, cumulative: { usd: AnySeriesPattern } }} args.getMetric
 * @param {(args: { series: AnySeriesPattern, name: string, color: Color, unit: Unit }) => AnyFetchedSeriesBlueprint} [args.seriesFn]
 * @returns {PartialOptionsTree}
 */
export function groupedWindowsCumulativeUsd({
  list,
  all,
  title,
  metricTitle,
  getMetric,
  seriesFn = line,
}) {
  return groupedWindowsCumulative({
    list,
    all,
    title,
    metricTitle,
    seriesFn,
    unit: Unit.usd,
    getWindowSeries: (c, key) => getMetric(c).sum[key].usd,
    getCumulativeSeries: (c) => getMetric(c).cumulative.usd,
  });
}

/**
 * Multi-unit variant: windows access .sum[key] as satsBtcUsd pattern, cumulative same
 * @template {{ name: string, color: Color }} T
 * @template {{ name: string, color: Color }} A
 * @param {Object} args
 * @param {readonly T[]} args.list
 * @param {A} args.all
 * @param {(name: string) => string} args.title
 * @param {string} args.metricTitle
 * @param {(c: T | A) => { sum: Record<string, AnyValuePattern>, cumulative: AnyValuePattern }} args.getMetric
 * @returns {PartialOptionsTree}
 */
export function groupedWindowsCumulativeSatsBtcUsd({
  list,
  all,
  title,
  metricTitle,
  getMetric,
}) {
  return [
    ...ROLLING_WINDOWS.map((w) => ({
      name: w.name,
      title: title(`${w.title} ${metricTitle}`),
      bottom: flatMapCohortsWithAll(list, all, (c) =>
        satsBtcUsd({
          pattern: getMetric(c).sum[w.key],
          name: c.name,
          color: c.color,
        }),
      ),
    })),
    {
      name: "Cumulative",
      title: title(`Cumulative ${metricTitle}`),
      bottom: flatMapCohortsWithAll(list, all, (c) =>
        satsBtcUsd({
          pattern: getMetric(c).cumulative,
          name: c.name,
          color: c.color,
        }),
      ),
    },
  ];
}
