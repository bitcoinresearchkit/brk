/** Cointime section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";

/**
 * Create price with ratio options for cointime prices
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.legend
 * @param {AnyMetricPattern} args.price
 * @param {ActivePriceRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
function createCointimePriceWithRatioOptions(
  ctx,
  { title, legend, price, ratio, color },
) {
  const { line, colors, createPriceLine } = ctx;

  // Percentile USD mappings
  const percentileUsdMap = [
    { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.rose },
    { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.pink },
    { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.cyan },
    { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.sky },
    { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.blue },
  ];

  // Percentile ratio mappings
  const percentileMap = [
    { name: "pct99", prop: ratio.ratioPct99, color: colors.rose },
    { name: "pct98", prop: ratio.ratioPct98, color: colors.pink },
    { name: "pct95", prop: ratio.ratioPct95, color: colors.fuchsia },
    { name: "pct5", prop: ratio.ratioPct5, color: colors.cyan },
    { name: "pct2", prop: ratio.ratioPct2, color: colors.sky },
    { name: "pct1", prop: ratio.ratioPct1, color: colors.blue },
  ];

  // SD patterns by window
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
      top: [line({ metric: price, name: legend, color, unit: Unit.usd })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        line({ metric: price, name: legend, color, unit: Unit.usd }),
        ...percentileUsdMap.map(({ name: pctName, prop, color: pctColor }) =>
          line({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: Unit.usd,
            options: { lineStyle: 1 },
          }),
        ),
      ],
      bottom: [
        line({ metric: ratio.ratio, name: "Ratio", color, unit: Unit.ratio }),
        line({
          metric: ratio.ratio1wSma,
          name: "1w SMA",
          color: colors.lime,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio1mSma,
          name: "1m SMA",
          color: colors.teal,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio1ySd.sma,
          name: "1y SMA",
          color: colors.sky,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio2ySd.sma,
          name: "2y SMA",
          color: colors.indigo,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratio4ySd.sma,
          name: "4y SMA",
          color: colors.purple,
          unit: Unit.ratio,
        }),
        line({
          metric: ratio.ratioSd.sma,
          name: "All SMA",
          color: colors.rose,
          unit: Unit.ratio,
        }),
        ...percentileMap.map(({ name: pctName, prop, color: pctColor }) =>
          line({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: Unit.ratio,
            options: { lineStyle: 1 },
          }),
        ),
        createPriceLine({ unit: Unit.ratio, number: 1 }),
      ],
    },
    {
      name: "ZScores",
      tree: sdPatterns.map(({ nameAddon, titleAddon, sd }) => ({
        name: nameAddon,
        title: `${title} ${titleAddon} Z-Score`,
        top: getSdBands(sd).map(({ name: bandName, prop, color: bandColor }) =>
          line({
            metric: prop,
            name: bandName,
            color: bandColor,
            unit: Unit.usd,
          }),
        ),
        bottom: [
          line({ metric: sd.zscore, name: "Z-Score", color, unit: Unit.sd }),
          createPriceLine({ unit: Unit.sd, number: 3 }),
          createPriceLine({ unit: Unit.sd, number: 2 }),
          createPriceLine({ unit: Unit.sd, number: 1 }),
          createPriceLine({ unit: Unit.sd, number: 0 }),
          createPriceLine({ unit: Unit.sd, number: -1 }),
          createPriceLine({ unit: Unit.sd, number: -2 }),
          createPriceLine({ unit: Unit.sd, number: -3 }),
        ],
      })),
    },
  ];
}

/**
 * Create Cointime section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection(ctx) {
  const { colors, brk, line } = ctx;
  const { cointime, distribution, supply } = brk.metrics;
  const { pricing, cap, activity, supply: cointimeSupply, adjusted } = cointime;
  const { all } = distribution.utxoCohorts;

  // Cointime prices data
  const cointimePrices = [
    {
      price: pricing.trueMarketMean,
      ratio: pricing.trueMarketMeanRatio,
      name: "True market mean",
      title: "true market mean",
      color: colors.blue,
    },
    {
      price: pricing.vaultedPrice,
      ratio: pricing.vaultedPriceRatio,
      name: "Vaulted",
      title: "vaulted price",
      color: colors.lime,
    },
    {
      price: pricing.activePrice,
      ratio: pricing.activePriceRatio,
      name: "Active",
      title: "active price",
      color: colors.rose,
    },
    {
      price: pricing.cointimePrice,
      ratio: pricing.cointimePriceRatio,
      name: "cointime",
      title: "cointime price",
      color: colors.yellow,
    },
  ];

  // Cointime capitalizations data
  const cointimeCapitalizations = [
    {
      metric: cap.vaultedCap,
      name: "vaulted",
      title: "vaulted Capitalization",
      color: colors.lime,
    },
    {
      metric: cap.activeCap,
      name: "active",
      title: "active Capitalization",
      color: colors.rose,
    },
    {
      metric: cap.cointimeCap,
      name: "cointime",
      title: "cointime Capitalization",
      color: colors.yellow,
    },
    {
      metric: cap.investorCap,
      name: "investor",
      title: "investor Capitalization",
      color: colors.fuchsia,
    },
    {
      metric: cap.thermoCap,
      name: "thermo",
      title: "thermo Capitalization",
      color: colors.emerald,
    },
  ];

  return {
    name: "Cointime",
    tree: [
      // Prices
      {
        name: "Prices",
        tree: [
          {
            name: "Compare",
            title: "Compare Cointime Prices",
            top: cointimePrices.map(({ price, name, color }) =>
              line({ metric: price, name, color, unit: Unit.usd }),
            ),
          },
          ...cointimePrices.map(({ price, ratio, name, color, title }) => ({
            name,
            tree: createCointimePriceWithRatioOptions(ctx, {
              price,
              ratio,
              legend: name,
              color,
              title,
            }),
          })),
        ],
      },

      // Capitalization
      {
        name: "Capitalization",
        tree: [
          {
            name: "Compare",
            title: "Compare Cointime Capitalizations",
            bottom: [
              line({
                metric: supply.marketCap,
                name: "Market",
                color: colors.default,
                unit: Unit.usd,
              }),
              line({
                metric: all.realized.realizedCap,
                name: "Realized",
                color: colors.orange,
                unit: Unit.usd,
              }),
              ...cointimeCapitalizations.map(({ metric, name, color }) =>
                line({ metric, name, color, unit: Unit.usd }),
              ),
            ],
          },
          ...cointimeCapitalizations.map(({ metric, name, color, title }) => ({
            name,
            title,
            bottom: [
              line({ metric, name, color, unit: Unit.usd }),
              line({
                metric: supply.marketCap,
                name: "Market",
                color: colors.default,
                unit: Unit.usd,
              }),
              line({
                metric: all.realized.realizedCap,
                name: "Realized",
                color: colors.orange,
                unit: Unit.usd,
              }),
            ],
          })),
        ],
      },

      // Supply
      {
        name: "Supply",
        title: "Cointime Supply",
        bottom: [
          // All supply (different pattern structure)
          line({
            metric: all.supply.total.sats,
            name: "All",
            color: colors.orange,
            unit: Unit.sats,
          }),
          line({
            metric: all.supply.total.bitcoin,
            name: "All",
            color: colors.orange,
            unit: Unit.btc,
          }),
          line({
            metric: all.supply.total.dollars,
            name: "All",
            color: colors.orange,
            unit: Unit.usd,
          }),
          // Cointime supplies (ActiveSupplyPattern)
          .../** @type {const} */ ([
            [cointimeSupply.vaultedSupply, "Vaulted", colors.lime],
            [cointimeSupply.activeSupply, "Active", colors.rose],
          ]).flatMap(([supplyItem, name, color]) => [
            line({ metric: supplyItem.sats, name, color, unit: Unit.sats }),
            line({ metric: supplyItem.bitcoin, name, color, unit: Unit.btc }),
            line({ metric: supplyItem.dollars, name, color, unit: Unit.usd }),
          ]),
        ],
      },

      // Liveliness & Vaultedness
      {
        name: "Liveliness & Vaultedness",
        title: "Liveliness & Vaultedness",
        bottom: [
          line({
            metric: activity.liveliness,
            name: "Liveliness",
            color: colors.rose,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.vaultedness,
            name: "Vaultedness",
            color: colors.lime,
            unit: Unit.ratio,
          }),
          line({
            metric: activity.activityToVaultednessRatio,
            name: "Liveliness / Vaultedness",
            color: colors.purple,
            unit: Unit.ratio,
          }),
        ],
      },

      // Coinblocks
      {
        name: "Coinblocks",
        title: "Coinblocks",
        bottom: [
          // Destroyed comes from the all cohort's activity
          line({
            metric: all.activity.coinblocksDestroyed.sum,
            name: "Destroyed",
            color: colors.red,
            unit: Unit.coinblocks,
          }),
          line({
            metric: all.activity.coinblocksDestroyed.cumulative,
            name: "Cumulative Destroyed",
            color: colors.red,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
          // Created and stored from cointime
          line({
            metric: activity.coinblocksCreated.sum,
            name: "Created",
            color: colors.orange,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksCreated.cumulative,
            name: "Cumulative Created",
            color: colors.orange,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksStored.sum,
            name: "Stored",
            color: colors.green,
            unit: Unit.coinblocks,
          }),
          line({
            metric: activity.coinblocksStored.cumulative,
            name: "Cumulative Stored",
            color: colors.green,
            defaultActive: false,
            unit: Unit.coinblocks,
          }),
        ],
      },

      // Adjusted metrics
      {
        name: "Adjusted",
        tree: [
          // Inflation
          {
            name: "Inflation",
            title: "Cointime-Adjusted Inflation Rate",
            bottom: [
              line({
                metric: supply.inflation,
                name: "Base",
                color: colors.orange,
                unit: Unit.percentage,
              }),
              line({
                metric: adjusted.cointimeAdjInflationRate,
                name: "Adjusted",
                color: colors.purple,
                unit: Unit.percentage,
              }),
            ],
          },
          // Velocity
          {
            name: "Velocity",
            title: "Cointime-Adjusted Transactions Velocity",
            bottom: [
              line({
                metric: supply.velocity.btc,
                name: "BTC",
                color: colors.orange,
                unit: Unit.ratio,
              }),
              line({
                metric: adjusted.cointimeAdjTxBtcVelocity,
                name: "Adj. BTC",
                color: colors.red,
                unit: Unit.ratio,
              }),
              line({
                metric: supply.velocity.usd,
                name: "USD",
                color: colors.emerald,
                unit: Unit.ratio,
              }),
              line({
                metric: adjusted.cointimeAdjTxUsdVelocity,
                name: "Adj. USD",
                color: colors.lime,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },
    ],
  };
}
