/** Shared helpers for options */

import { Unit } from "../utils/units.js";
import { line, baseline, price } from "./series.js";
import { priceLine, priceLines } from "./constants.js";
import { colors } from "../utils/colors.js";

/**
 * Create a title formatter for chart titles
 * @param {string} [cohortTitle]
 * @returns {(metric: string) => string}
 */
export const formatCohortTitle = (cohortTitle) => (metric) =>
  cohortTitle ? `${metric}: ${cohortTitle}` : metric;

/**
 * Create sats/btc/usd line series from a pattern with .sats/.bitcoin/.dollars
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
      metric: pattern.bitcoin,
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
      metric: pattern.dollars,
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
 * @param {{ bitcoin: AnyMetricPattern, sats: AnyMetricPattern, dollars: AnyMetricPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedBaselineSeriesBlueprint[]}
 */
export function satsBtcUsdBaseline({ pattern, name, color, defaultActive }) {
  return [
    baseline({
      metric: pattern.bitcoin,
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
      metric: pattern.dollars,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
    }),
  ];
}

/**
 * Create sats/btc/usd series from any value pattern using sum or cumulative key
 * @param {Object} args
 * @param {AnyValuePatternType} args.source
 * @param {'sum' | 'cumulative'} args.key
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsdFrom({ source, key, name, color, defaultActive }) {
  return satsBtcUsd({
    pattern: {
      bitcoin: source.bitcoin[key],
      sats: source.sats[key],
      dollars: source.dollars[key],
    },
    name,
    color,
    defaultActive,
  });
}

/**
 * Create sats/btc/usd series from a full value pattern using base or average key
 * @param {Object} args
 * @param {FullValuePattern} args.source
 * @param {'base' | 'average'} args.key
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsdFromFull({
  source,
  key,
  name,
  color,
  defaultActive,
}) {
  return satsBtcUsd({
    pattern: {
      bitcoin: source.bitcoin[key],
      sats: source.sats[key],
      dollars: source.dollars[key],
    },
    name,
    color,
    defaultActive,
  });
}

/**
 * Create coinbase/subsidy/fee series from separate sources
 * @param {Object} args
 * @param {AnyValuePatternType} args.coinbase
 * @param {AnyValuePatternType} args.subsidy
 * @param {AnyValuePatternType} args.fee
 * @param {'sum' | 'cumulative'} args.key
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
 * Build percentile USD mappings from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function percentileUsdMap(ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.ratioPct._95 },
    { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.ratioPct._5 },
    { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.ratioPct._98 },
    { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.ratioPct._2 },
    { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.ratioPct._99 },
    { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.ratioPct._1 },
  ]);
}

/**
 * Build percentile ratio mappings from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function percentileMap(ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95, color: colors.ratioPct._95 },
    { name: "pct5", prop: ratio.ratioPct5, color: colors.ratioPct._5 },
    { name: "pct98", prop: ratio.ratioPct98, color: colors.ratioPct._98 },
    { name: "pct2", prop: ratio.ratioPct2, color: colors.ratioPct._2 },
    { name: "pct99", prop: ratio.ratioPct99, color: colors.ratioPct._99 },
    { name: "pct1", prop: ratio.ratioPct1, color: colors.ratioPct._1 },
  ]);
}

/**
 * Build SD patterns from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function sdPatterns(ratio) {
  return /** @type {const} */ ([
    { nameAddon: "all", titleAddon: "", sd: ratio.ratioSd },
    { nameAddon: "4y", titleAddon: "4y", sd: ratio.ratio4ySd },
    { nameAddon: "2y", titleAddon: "2y", sd: ratio.ratio2ySd },
    { nameAddon: "1y", titleAddon: "1y", sd: ratio.ratio1ySd },
  ]);
}

/**
 * Build SD band mappings from an SD pattern
 * @param {Ratio1ySdPattern} sd
 */
export function sdBandsUsd(sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd._0sdUsd, color: colors.sd._0 },
    { name: "+0.5σ", prop: sd.p05sdUsd, color: colors.sd.p05 },
    { name: "−0.5σ", prop: sd.m05sdUsd, color: colors.sd.m05 },
    { name: "+1σ", prop: sd.p1sdUsd, color: colors.sd.p1 },
    { name: "−1σ", prop: sd.m1sdUsd, color: colors.sd.m1 },
    { name: "+1.5σ", prop: sd.p15sdUsd, color: colors.sd.p15 },
    { name: "−1.5σ", prop: sd.m15sdUsd, color: colors.sd.m15 },
    { name: "+2σ", prop: sd.p2sdUsd, color: colors.sd.p2 },
    { name: "−2σ", prop: sd.m2sdUsd, color: colors.sd.m2 },
    { name: "+2.5σ", prop: sd.p25sdUsd, color: colors.sd.p25 },
    { name: "−2.5σ", prop: sd.m25sdUsd, color: colors.sd.m25 },
    { name: "+3σ", prop: sd.p3sdUsd, color: colors.sd.p3 },
    { name: "−3σ", prop: sd.m3sdUsd, color: colors.sd.m3 },
  ]);
}

/**
 * Build SD band mappings (ratio) from an SD pattern
 * @param {Ratio1ySdPattern} sd
 */
export function sdBandsRatio(sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd.sma, color: colors.sd._0 },
    { name: "+0.5σ", prop: sd.p05sd, color: colors.sd.p05 },
    { name: "−0.5σ", prop: sd.m05sd, color: colors.sd.m05 },
    { name: "+1σ", prop: sd.p1sd, color: colors.sd.p1 },
    { name: "−1σ", prop: sd.m1sd, color: colors.sd.m1 },
    { name: "+1.5σ", prop: sd.p15sd, color: colors.sd.p15 },
    { name: "−1.5σ", prop: sd.m15sd, color: colors.sd.m15 },
    { name: "+2σ", prop: sd.p2sd, color: colors.sd.p2 },
    { name: "−2σ", prop: sd.m2sd, color: colors.sd.m2 },
    { name: "+2.5σ", prop: sd.p25sd, color: colors.sd.p25 },
    { name: "−2.5σ", prop: sd.m25sd, color: colors.sd.m25 },
    { name: "+3σ", prop: sd.p3sd, color: colors.sd.p3 },
    { name: "−3σ", prop: sd.m3sd, color: colors.sd.m3 },
  ]);
}

/**
 * Build ratio SMA series from a ratio pattern
 * @param {AnyRatioPattern} ratio
 */
export function ratioSmas(ratio) {
  return /** @type {const} */ ([
    { name: "1w SMA", metric: ratio.ratio1wSma, color: colors.ma._1w },
    { name: "1m SMA", metric: ratio.ratio1mSma, color: colors.ma._1m },
    { name: "1y SMA", metric: ratio.ratio1ySd.sma, color: colors.ma._1y },
    { name: "2y SMA", metric: ratio.ratio2ySd.sma, color: colors.ma._2y },
    { name: "4y SMA", metric: ratio.ratio4ySd.sma, color: colors.ma._4y },
    { name: "All SMA", metric: ratio.ratioSd.sma, color: colors.time.all },
  ]);
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

  return {
    name: "Z-Scores",
    tree: [
      {
        name: "Compare",
        title: formatTitle("Z-Scores"),
        top: [
          price({ metric: pricePattern, name: legend, color }),
          price({
            metric: ratio.ratio1ySd._0sdUsd,
            name: "1y 0σ",
            color: colors.ma._1y,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratio2ySd._0sdUsd,
            name: "2y 0σ",
            color: colors.ma._2y,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratio4ySd._0sdUsd,
            name: "4y 0σ",
            color: colors.ma._4y,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratioSd._0sdUsd,
            name: "all 0σ",
            color: colors.time.all,
            defaultActive: false,
          }),
        ],
        bottom: [
          line({
            metric: ratio.ratioSd.zscore,
            name: "All",
            color: colors.time.all,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio4ySd.zscore,
            name: "4y",
            color: colors.ma._4y,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio2ySd.zscore,
            name: "2y",
            color: colors.ma._2y,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio1ySd.zscore,
            name: "1y",
            color: colors.ma._1y,
            unit: Unit.sd,
          }),
          ...priceLines({
            unit: Unit.sd,
            numbers: [0, 1, -1, 2, -2, 3, -3],
            defaultActive: false,
          }),
        ],
      },
      ...sdPats.map(({ nameAddon, titleAddon, sd }) => ({
        name: nameAddon,
        title: formatTitle(`${titleAddon ? `${titleAddon} ` : ""}Z-Score`),
        top: [
          price({ metric: pricePattern, name: legend, color }),
          ...sdBandsUsd(sd).map(({ name: bandName, prop, color: bandColor }) =>
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
          baseline({
            metric: ratio.ratio,
            name: "Ratio",
            unit: Unit.ratio,
            base: 1,
          }),
          line({
            metric: sd.sd,
            name: "Volatility",
            color: colors.gray,
            unit: Unit.percentage,
          }),
          ...sdBandsRatio(sd).map(
            ({ name: bandName, prop, color: bandColor }) =>
              line({
                metric: prop,
                name: bandName,
                color: bandColor,
                unit: Unit.ratio,
                defaultActive: false,
              }),
          ),
          priceLine({
            unit: Unit.sd,
          }),
          ...priceLines({
            unit: Unit.sd,
            numbers: [1, -1, 2, -2, 3, -3],
            defaultActive: false,
          }),
        ],
      })),
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
