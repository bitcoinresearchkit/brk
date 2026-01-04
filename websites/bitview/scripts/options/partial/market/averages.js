/** Moving averages section */

import { periodIdToName } from "./utils.js";

/**
 * Build averages data array from market patterns
 * @param {Colors} colors
 * @param {MarketMovingAverage} ma
 */
export function buildAverages(colors, ma) {
  return /** @type {const} */ ([
    ["1w", 7, "red", ma.price1wSma, ma.price1wEma],
    ["8d", 8, "orange", ma.price8dSma, ma.price8dEma],
    ["13d", 13, "amber", ma.price13dSma, ma.price13dEma],
    ["21d", 21, "yellow", ma.price21dSma, ma.price21dEma],
    ["1m", 30, "lime", ma.price1mSma, ma.price1mEma],
    ["34d", 34, "green", ma.price34dSma, ma.price34dEma],
    ["55d", 55, "emerald", ma.price55dSma, ma.price55dEma],
    ["89d", 89, "teal", ma.price89dSma, ma.price89dEma],
    ["144d", 144, "cyan", ma.price144dSma, ma.price144dEma],
    ["200d", 200, "sky", ma.price200dSma, ma.price200dEma],
    ["1y", 365, "blue", ma.price1ySma, ma.price1yEma],
    ["2y", 730, "indigo", ma.price2ySma, ma.price2yEma],
    ["200w", 1400, "violet", ma.price200wSma, ma.price200wEma],
    ["4y", 1460, "purple", ma.price4ySma, ma.price4yEma],
  ]).map(([id, days, colorKey, sma, ema]) => ({
    id,
    name: periodIdToName(id, true),
    days,
    color: colors[colorKey],
    sma,
    ema,
  }));
}

/**
 * Create price with ratio options (for moving averages)
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {EmaRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
export function createPriceWithRatioOptions(ctx, { title, legend, ratio, color }) {
  const { s, colors, createPriceLine } = ctx;
  const priceMetric = ratio.price;

  const percentileUsdMap = [
    { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.rose },
    { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.pink },
    { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.cyan },
    { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.sky },
    { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.blue },
  ];

  const percentileMap = [
    { name: "pct99", prop: ratio.ratioPct99, color: colors.rose },
    { name: "pct98", prop: ratio.ratioPct98, color: colors.pink },
    { name: "pct95", prop: ratio.ratioPct95, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5, color: colors.cyan },
    { name: "pct2", prop: ratio.ratioPct2, color: colors.sky },
    { name: "pct1", prop: ratio.ratioPct1, color: colors.blue },
  ];

  const sdPatterns = [
    { nameAddon: "all", titleAddon: "", sd: ratio.ratioSd },
    { nameAddon: "4y", titleAddon: "4y", sd: ratio.ratio4ySd },
    { nameAddon: "2y", titleAddon: "2y", sd: ratio.ratio2ySd },
    { nameAddon: "1y", titleAddon: "1y", sd: ratio.ratio1ySd },
  ];

  /** @param {Ratio1ySdPattern} sd */
  const getSdBands = (sd) => [
    { name: "0σ", prop: sd._0sdUsd, color: colors.lime },
    { name: "+0.5σ", prop: sd.p05sdUsd, color: colors.yellow },
    { name: "+1σ", prop: sd.p1sdUsd, color: colors.amber },
    { name: "+1.5σ", prop: sd.p15sdUsd, color: colors.orange },
    { name: "+2σ", prop: sd.p2sdUsd, color: colors.red },
    { name: "+2.5σ", prop: sd.p25sdUsd, color: colors.rose },
    { name: "+3σ", prop: sd.p3sd, color: colors.pink },
    { name: "−0.5σ", prop: sd.m05sdUsd, color: colors.teal },
    { name: "−1σ", prop: sd.m1sdUsd, color: colors.cyan },
    { name: "−1.5σ", prop: sd.m15sdUsd, color: colors.sky },
    { name: "−2σ", prop: sd.m2sdUsd, color: colors.blue },
    { name: "−2.5σ", prop: sd.m25sdUsd, color: colors.indigo },
    { name: "−3σ", prop: sd.m3sd, color: colors.violet },
  ];

  return [
    {
      name: "price",
      title,
      top: [s({ metric: priceMetric, name: legend, color, unit: "usd" })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        s({ metric: priceMetric, name: legend, color, unit: "usd" }),
        ...percentileUsdMap.map(({ name: pctName, prop, color: pctColor }) =>
          s({ metric: prop, name: pctName, color: pctColor, defaultActive: false, unit: "usd", options: { lineStyle: 1 } }),
        ),
      ],
      bottom: [
        s({ metric: ratio.ratio, name: "ratio", color, unit: "ratio" }),
        s({ metric: ratio.ratio1wSma, name: "1w sma", color: colors.lime, unit: "ratio" }),
        s({ metric: ratio.ratio1mSma, name: "1m sma", color: colors.teal, unit: "ratio" }),
        s({ metric: ratio.ratio1ySd.sma, name: "1y sma", color: colors.sky, unit: "ratio" }),
        s({ metric: ratio.ratio2ySd.sma, name: "2y sma", color: colors.indigo, unit: "ratio" }),
        s({ metric: ratio.ratio4ySd.sma, name: "4y sma", color: colors.purple, unit: "ratio" }),
        s({ metric: ratio.ratioSd.sma, name: "all sma", color: colors.rose, unit: "ratio" }),
        ...percentileMap.map(({ name: pctName, prop, color: pctColor }) =>
          s({ metric: prop, name: pctName, color: pctColor, defaultActive: false, unit: "ratio", options: { lineStyle: 1 } }),
        ),
        createPriceLine({ unit: "ratio", number: 1 }),
      ],
    },
    {
      name: "ZScores",
      tree: sdPatterns.map(({ nameAddon, titleAddon, sd }) => ({
        name: nameAddon,
        title: `${title} ${titleAddon} Z-Score`,
        top: getSdBands(sd).map(({ name: bandName, prop, color: bandColor }) =>
          s({ metric: prop, name: bandName, color: bandColor, unit: "usd" }),
        ),
        bottom: [
          s({ metric: sd.zscore, name: "zscore", color, unit: "sd" }),
          createPriceLine({ unit: "sd", number: 3 }),
          createPriceLine({ unit: "sd", number: 2 }),
          createPriceLine({ unit: "sd", number: 1 }),
          createPriceLine({ unit: "sd", number: 0 }),
          createPriceLine({ unit: "sd", number: -1 }),
          createPriceLine({ unit: "sd", number: -2 }),
          createPriceLine({ unit: "sd", number: -3 }),
        ],
      })),
    },
  ];
}

/**
 * Create Averages section
 * @param {PartialContext} ctx
 * @param {ReturnType<typeof buildAverages>} averages
 */
export function createAveragesSection(ctx, averages) {
  const { s } = ctx;

  return {
    name: "Averages",
    tree: [
      { nameAddon: "Simple", metricAddon: /** @type {const} */ ("sma") },
      { nameAddon: "Exponential", metricAddon: /** @type {const} */ ("ema") },
    ].map(({ nameAddon, metricAddon }) => ({
      name: nameAddon,
      tree: [
        {
          name: "Compare",
          title: `Market Price ${nameAddon} Moving Averages`,
          top: averages.map(({ id, color, sma, ema }) =>
            s({ metric: (metricAddon === "sma" ? sma : ema).price, name: id, color, unit: "usd" }),
          ),
        },
        ...averages.map(({ name, color, sma, ema }) => ({
          name,
          tree: createPriceWithRatioOptions(ctx, {
            ratio: metricAddon === "sma" ? sma : ema,
            title: `${name} Market Price ${nameAddon} Moving Average`,
            legend: "average",
            color,
          }),
        })),
      ],
    })),
  };
}
