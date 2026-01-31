/** Shared helpers for options */

import { Unit } from "../utils/units.js";
import { line, baseline, price } from "./series.js";
import { priceLine, priceLines } from "./constants.js";

/**
 * Create a title formatter for chart titles
 * @param {string} [cohortTitle]
 * @returns {(metric: string) => string}
 */
export const formatCohortTitle = (cohortTitle) =>
  (metric) => cohortTitle ? `${metric}: ${cohortTitle}` : metric;

/**
 * Create sats/btc/usd line series from a pattern with .sats/.bitcoin/.dollars
 * @param {Object} args
 * @param {{ sats: AnyMetricPattern, bitcoin: AnyMetricPattern, dollars: AnyMetricPattern }} args.pattern
 * @param {string} args.name
 * @param {Color} [args.color]
 * @param {boolean} [args.defaultActive]
 * @returns {FetchedLineSeriesBlueprint[]}
 */
export function satsBtcUsd({ pattern, name, color, defaultActive }) {
  return [
    line({
      metric: pattern.bitcoin,
      name,
      color,
      unit: Unit.btc,
      defaultActive,
    }),
    line({ metric: pattern.sats, name, color, unit: Unit.sats, defaultActive }),
    line({
      metric: pattern.dollars,
      name,
      color,
      unit: Unit.usd,
      defaultActive,
    }),
  ];
}

/**
 * Build percentile USD mappings from a ratio pattern
 * @param {Colors} colors
 * @param {AnyRatioPattern} ratio
 */
export function percentileUsdMap(colors, ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.cyan },
    { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.pink },
    { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.sky },
    { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.rose },
    { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.blue },
  ]);
}

/**
 * Build percentile ratio mappings from a ratio pattern
 * @param {Colors} colors
 * @param {AnyRatioPattern} ratio
 */
export function percentileMap(colors, ratio) {
  return /** @type {const} */ ([
    { name: "pct95", prop: ratio.ratioPct95, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5, color: colors.cyan },
    { name: "pct98", prop: ratio.ratioPct98, color: colors.pink },
    { name: "pct2", prop: ratio.ratioPct2, color: colors.sky },
    { name: "pct99", prop: ratio.ratioPct99, color: colors.rose },
    { name: "pct1", prop: ratio.ratioPct1, color: colors.blue },
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
 * @param {Colors} colors
 * @param {Ratio1ySdPattern} sd
 */
export function sdBandsUsd(colors, sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd._0sdUsd, color: colors.lime },
    { name: "+0.5σ", prop: sd.p05sdUsd, color: colors.yellow },
    { name: "−0.5σ", prop: sd.m05sdUsd, color: colors.teal },
    { name: "+1σ", prop: sd.p1sdUsd, color: colors.amber },
    { name: "−1σ", prop: sd.m1sdUsd, color: colors.cyan },
    { name: "+1.5σ", prop: sd.p15sdUsd, color: colors.orange },
    { name: "−1.5σ", prop: sd.m15sdUsd, color: colors.sky },
    { name: "+2σ", prop: sd.p2sdUsd, color: colors.red },
    { name: "−2σ", prop: sd.m2sdUsd, color: colors.blue },
    { name: "+2.5σ", prop: sd.p25sdUsd, color: colors.rose },
    { name: "−2.5σ", prop: sd.m25sdUsd, color: colors.indigo },
    { name: "+3σ", prop: sd.p3sdUsd, color: colors.pink },
    { name: "−3σ", prop: sd.m3sdUsd, color: colors.violet },
  ]);
}

/**
 * Build SD band mappings (ratio) from an SD pattern
 * @param {Colors} colors
 * @param {Ratio1ySdPattern} sd
 */
export function sdBandsRatio(colors, sd) {
  return /** @type {const} */ ([
    { name: "0σ", prop: sd.sma, color: colors.lime },
    { name: "+0.5σ", prop: sd.p05sd, color: colors.yellow },
    { name: "−0.5σ", prop: sd.m05sd, color: colors.teal },
    { name: "+1σ", prop: sd.p1sd, color: colors.amber },
    { name: "−1σ", prop: sd.m1sd, color: colors.cyan },
    { name: "+1.5σ", prop: sd.p15sd, color: colors.orange },
    { name: "−1.5σ", prop: sd.m15sd, color: colors.sky },
    { name: "+2σ", prop: sd.p2sd, color: colors.red },
    { name: "−2σ", prop: sd.m2sd, color: colors.blue },
    { name: "+2.5σ", prop: sd.p25sd, color: colors.rose },
    { name: "−2.5σ", prop: sd.m25sd, color: colors.indigo },
    { name: "+3σ", prop: sd.p3sd, color: colors.pink },
    { name: "−3σ", prop: sd.m3sd, color: colors.violet },
  ]);
}

/**
 * Build ratio SMA series from a ratio pattern
 * @param {Colors} colors
 * @param {AnyRatioPattern} ratio
 */
export function ratioSmas(colors, ratio) {
  return /** @type {const} */ ([
    { name: "1w SMA", metric: ratio.ratio1wSma, color: colors.lime },
    { name: "1m SMA", metric: ratio.ratio1mSma, color: colors.teal },
    { name: "1y SMA", metric: ratio.ratio1ySd.sma, color: colors.sky },
    { name: "2y SMA", metric: ratio.ratio2ySd.sma, color: colors.indigo },
    { name: "4y SMA", metric: ratio.ratio4ySd.sma, color: colors.purple },
    { name: "All SMA", metric: ratio.ratioSd.sma, color: colors.rose },
  ]);
}

/**
 * Create ratio chart from ActivePriceRatioPattern
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {(metric: string) => string} args.title
 * @param {AnyPricePattern} args.pricePattern - The price pattern to show in top pane
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @param {string} [args.name] - Optional name override (default: "ratio")
 * @returns {PartialChartOption}
 */
export function createRatioChart(ctx, { title, pricePattern, ratio, color, name }) {
  const { colors } = ctx;

  return {
    name: name ?? "ratio",
    title: title(name ?? "Ratio"),
    top: [
      price({ metric: pricePattern, name: "Price", color }),
      ...percentileUsdMap(colors, ratio).map(({ name, prop, color }) =>
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
      ...ratioSmas(colors, ratio).map(({ name, metric, color }) =>
        line({ metric, name, color, unit: Unit.ratio, defaultActive: false }),
      ),
      ...percentileMap(colors, ratio).map(({ name, prop, color }) =>
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
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {(suffix: string) => string} args.formatTitle - Function that takes metric suffix and returns full title
 * @param {string} args.legend
 * @param {AnyPricePattern} args.pricePattern - The price pattern to show in top pane
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @returns {PartialOptionsGroup}
 */
export function createZScoresFolder(
  ctx,
  { formatTitle, legend, pricePattern, ratio, color },
) {
  const { colors } = ctx;
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
            color: colors.orange,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratio2ySd._0sdUsd,
            name: "2y 0σ",
            color: colors.yellow,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratio4ySd._0sdUsd,
            name: "4y 0σ",
            color: colors.lime,
            defaultActive: false,
          }),
          price({
            metric: ratio.ratioSd._0sdUsd,
            name: "all 0σ",
            color: colors.blue,
            defaultActive: false,
          }),
        ],
        bottom: [
          line({
            metric: ratio.ratioSd.zscore,
            name: "All",
            color: colors.blue,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio4ySd.zscore,
            name: "4y",
            color: colors.lime,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio2ySd.zscore,
            name: "2y",
            color: colors.yellow,
            unit: Unit.sd,
          }),
          line({
            metric: ratio.ratio1ySd.zscore,
            name: "1y",
            color: colors.orange,
            unit: Unit.sd,
          }),
          ...priceLines({
            ctx,
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
          ...sdBandsUsd(colors, sd).map(
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
          ...sdBandsRatio(colors, sd).map(
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
            ctx,
            unit: Unit.sd,
          }),
          ...priceLines({
            ctx,
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
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.context - Context string for ratio/z-scores titles (e.g., "1 Week SMA", "STH")
 * @param {string} args.legend - Legend name for the price series
 * @param {AnyPricePattern} args.pricePattern - The price pattern
 * @param {AnyRatioPattern} args.ratio - The ratio pattern
 * @param {Color} args.color
 * @param {string} [args.ratioName] - Optional custom name for ratio chart (default: "ratio")
 * @param {string} [args.priceTitle] - Optional override for price chart title (default: context)
 * @param {string} [args.zScoresSuffix] - Optional suffix appended to context for z-scores (e.g., "MVRV" gives "2y Z-Score: STH MVRV")
 * @returns {PartialOptionsTree}
 */
export function createPriceRatioCharts(ctx, { context, legend, pricePattern, ratio, color, ratioName, priceTitle, zScoresSuffix }) {
  const titleFn = formatCohortTitle(context);
  const zScoresTitleFn = zScoresSuffix ? formatCohortTitle(`${context} ${zScoresSuffix}`) : titleFn;
  return [
    {
      name: "Price",
      title: priceTitle ?? context,
      top: [price({ metric: pricePattern, name: legend, color })],
    },
    createRatioChart(ctx, {
      title: titleFn,
      pricePattern,
      ratio,
      color,
      name: ratioName,
    }),
    createZScoresFolder(ctx, {
      formatTitle: zScoresTitleFn,
      legend,
      pricePattern,
      ratio,
      color,
    }),
  ];
}
