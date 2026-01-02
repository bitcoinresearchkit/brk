/** Cointime section builder - typed tree-based patterns */

/**
 * Create price with ratio options for cointime prices
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.name
 * @param {string} args.title
 * @param {string} args.legend
 * @param {MetricAccessor<any>} args.price
 * @param {ActivePriceRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
function createCointimePriceWithRatioOptions(ctx, { name, title, legend, price, ratio, color }) {
  const { s, colors, createPriceLine } = ctx;

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
      top: [s({ metric: price, name: legend, color, unit: "usd" })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        s({ metric: price, name: legend, color, unit: "usd" }),
        ...percentileUsdMap.map(({ name: pctName, prop, color: pctColor }) =>
          s({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: "usd",
            options: { lineStyle: 1 },
          }),
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
          s({
            metric: prop,
            name: pctName,
            color: pctColor,
            defaultActive: false,
            unit: "ratio",
            options: { lineStyle: 1 },
          }),
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
 * Create Cointime section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createCointimeSection(ctx) {
  const { colors, brk, s } = ctx;
  const { cointime, distribution, supply } = brk.tree.computed;
  const { pricing, cap, activity, supply: cointimeSupply, adjusted } = cointime;
  const utxoCohorts = distribution.utxoCohorts;

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
    { metric: cap.vaultedCap, name: "vaulted", title: "vaulted Capitalization", color: colors.lime },
    { metric: cap.activeCap, name: "active", title: "active Capitalization", color: colors.rose },
    { metric: cap.cointimeCap, name: "cointime", title: "cointime Capitalization", color: colors.yellow },
    { metric: cap.investorCap, name: "investor", title: "investor Capitalization", color: colors.fuchsia },
    { metric: cap.thermoCap, name: "thermo", title: "thermo Capitalization", color: colors.emerald },
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
              s({ metric: price, name, color, unit: "usd" }),
            ),
          },
          ...cointimePrices.map(({ price, ratio, name, color, title }) => ({
            name,
            tree: createCointimePriceWithRatioOptions(ctx, {
              price,
              ratio,
              legend: name,
              color,
              name,
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
              s({ metric: supply.marketCap.height, name: "Market", color: colors.default, unit: "usd" }),
              s({ metric: utxoCohorts.all.realized.realizedCap, name: "Realized", color: colors.orange, unit: "usd" }),
              ...cointimeCapitalizations.map(({ metric, name, color }) =>
                s({ metric, name, color, unit: "usd" }),
              ),
            ],
          },
          ...cointimeCapitalizations.map(({ metric, name, color, title }) => ({
            name,
            title,
            bottom: [
              s({ metric, name, color, unit: "usd" }),
              s({ metric: supply.marketCap.height, name: "Market", color: colors.default, unit: "usd" }),
              s({ metric: utxoCohorts.all.realized.realizedCap, name: "Realized", color: colors.orange, unit: "usd" }),
            ],
          })),
        ],
      },

      // Supply
      {
        name: "Supply",
        title: "Cointime Supply",
        bottom: /** @type {const} */ ([
          [utxoCohorts.all.supply.supply, "all", colors.orange],
          [cointimeSupply.vaultedSupply, "vaulted", colors.lime],
          [cointimeSupply.activeSupply, "active", colors.rose],
        ]).flatMap(([supplyItem, name, color]) => [
          s({ metric: supplyItem.sats, name, color, unit: "sats" }),
          s({ metric: supplyItem.bitcoin, name, color, unit: "btc" }),
          s({ metric: supplyItem.dollars, name, color, unit: "usd" }),
        ]),
      },

      // Liveliness & Vaultedness
      {
        name: "Liveliness & Vaultedness",
        title: "Liveliness & Vaultedness",
        bottom: [
          s({ metric: activity.liveliness, name: "Liveliness", color: colors.rose, unit: "ratio" }),
          s({ metric: activity.vaultedness, name: "Vaultedness", color: colors.lime, unit: "ratio" }),
          s({ metric: activity.activityToVaultednessRatio, name: "Liveliness / Vaultedness", color: colors.purple, unit: "ratio" }),
        ],
      },

      // Coinblocks
      {
        name: "Coinblocks",
        title: "Coinblocks",
        bottom: [
          // Destroyed comes from the all cohort's activity
          s({ metric: utxoCohorts.all.activity.coinblocksDestroyed.base, name: "Destroyed", color: colors.red, unit: "coinblocks" }),
          s({ metric: utxoCohorts.all.activity.coinblocksDestroyed.cumulative, name: "Cumulative Destroyed", color: colors.red, defaultActive: false, unit: "coinblocks" }),
          // Created and stored from cointime
          s({ metric: activity.coinblocksCreated.base, name: "created", color: colors.orange, unit: "coinblocks" }),
          s({ metric: activity.coinblocksCreated.cumulative, name: "Cumulative created", color: colors.orange, defaultActive: false, unit: "coinblocks" }),
          s({ metric: activity.coinblocksStored.base, name: "stored", color: colors.green, unit: "coinblocks" }),
          s({ metric: activity.coinblocksStored.cumulative, name: "Cumulative stored", color: colors.green, defaultActive: false, unit: "coinblocks" }),
        ],
      },

      // Adjusted metrics
      {
        name: "Adjusted",
        tree: [
          // Inflation
          {
            name: "inflation",
            title: "Cointime-Adjusted inflation rate",
            bottom: [
              s({ metric: supply.inflation.indexes, name: "base", color: colors.orange, unit: "percentage" }),
              s({ metric: adjusted.cointimeAdjInflationRate, name: "adjusted", color: colors.purple, unit: "percentage" }),
            ],
          },
          // Velocity
          {
            name: "Velocity",
            title: "Cointime-Adjusted transactions velocity",
            bottom: [
              s({ metric: supply.velocity.btc, name: "btc", color: colors.orange, unit: "ratio" }),
              s({ metric: adjusted.cointimeAdjTxBtcVelocity, name: "adj. btc", color: colors.red, unit: "ratio" }),
              s({ metric: supply.velocity.usd, name: "usd", color: colors.emerald, unit: "ratio" }),
              s({ metric: adjusted.cointimeAdjTxUsdVelocity, name: "adj. usd", color: colors.lime, unit: "ratio" }),
            ],
          },
        ],
      },
    ],
  };
}
