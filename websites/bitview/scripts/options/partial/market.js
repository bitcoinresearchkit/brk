/** Market section builder - typed tree-based patterns */

/**
 * Convert period ID to readable name
 * @param {string} id
 * @param {boolean} [compoundAdjective]
 */
function periodIdToName(id, compoundAdjective) {
  const suffix = compoundAdjective || parseInt(id) === 1 ? "" : "s";
  return id
    .replace("d", ` day${suffix}`)
    .replace("w", ` week${suffix}`)
    .replace("m", ` month${suffix}`)
    .replace("y", ` year${suffix}`);
}

/**
 * Create price with ratio options (for moving averages)
 * @param {PartialContext} ctx
 * @param {Object} args
 * @param {string} args.name
 * @param {string} args.title
 * @param {string} args.legend
 * @param {EmaRatioPattern} args.ratio
 * @param {Color} [args.color]
 * @returns {PartialOptionsTree}
 */
function createPriceWithRatioOptions(ctx, { name, title, legend, ratio, color }) {
  const { s, colors, createPriceLine } = ctx;
  const priceMetric = ratio.price;

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
      top: [s({ metric: priceMetric, name: legend, color, unit: "usd" })],
    },
    {
      name: "Ratio",
      title: `${title} Ratio`,
      top: [
        s({ metric: priceMetric, name: legend, color, unit: "usd" }),
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
 * Build averages data array from market patterns
 * @param {Colors} colors
 * @param {MarketMovingAverage} ma
 */
function buildAverages(colors, ma) {
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
 * Build DCA classes data array
 * @param {Colors} colors
 * @param {MarketDca} dca
 */
function buildDcaClasses(colors, dca) {
  return /** @type {const} */ ([
    [2015, "pink", false, dca.dcaClass2015AvgPrice, dca.dcaClass2015Returns, dca.dcaClass2015Stack],
    [2016, "red", false, dca.dcaClass2016AvgPrice, dca.dcaClass2016Returns, dca.dcaClass2016Stack],
    [2017, "orange", true, dca.dcaClass2017AvgPrice, dca.dcaClass2017Returns, dca.dcaClass2017Stack],
    [2018, "yellow", true, dca.dcaClass2018AvgPrice, dca.dcaClass2018Returns, dca.dcaClass2018Stack],
    [2019, "green", true, dca.dcaClass2019AvgPrice, dca.dcaClass2019Returns, dca.dcaClass2019Stack],
    [2020, "teal", true, dca.dcaClass2020AvgPrice, dca.dcaClass2020Returns, dca.dcaClass2020Stack],
    [2021, "sky", true, dca.dcaClass2021AvgPrice, dca.dcaClass2021Returns, dca.dcaClass2021Stack],
    [2022, "blue", true, dca.dcaClass2022AvgPrice, dca.dcaClass2022Returns, dca.dcaClass2022Stack],
    [2023, "purple", true, dca.dcaClass2023AvgPrice, dca.dcaClass2023Returns, dca.dcaClass2023Stack],
    [2024, "fuchsia", true, dca.dcaClass2024AvgPrice, dca.dcaClass2024Returns, dca.dcaClass2024Stack],
    [2025, "pink", true, dca.dcaClass2025AvgPrice, dca.dcaClass2025Returns, dca.dcaClass2025Stack],
  ]).map(([year, colorKey, defaultActive, avgPrice, returns, stack]) => ({
    year,
    color: colors[colorKey],
    defaultActive,
    avgPrice,
    returns,
    stack,
  }));
}

/**
 * Create Market section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createMarketSection(ctx) {
  const { colors, brk, s, createPriceLine } = ctx;
  const { market, supply } = brk.tree.computed;
  const { movingAverage, ath, returns, volatility, range, dca, lookback } = market;

  const averages = buildAverages(colors, movingAverage);
  const dcaClasses = buildDcaClasses(colors, dca);

  return {
    name: "Market",
    tree: [
      // Price (empty chart, shows candlesticks by default)
      {
        name: "Price",
        title: "Bitcoin Price",
      },

      // Capitalization
      {
        name: "Capitalization",
        title: "Market Capitalization",
        bottom: [s({ metric: supply.marketCap.indexes, name: "Capitalization", unit: "usd" })],
      },

      // All Time High
      {
        name: "All Time High",
        title: "All Time High",
        top: [s({ metric: ath.priceAth, name: "ath", unit: "usd" })],
        bottom: [
          s({ metric: ath.priceDrawdown, name: "Drawdown", color: colors.red, unit: "percentage" }),
          s({ metric: ath.daysSincePriceAth, name: "since", unit: "days" }),
          s({ metric: ath.maxDaysBetweenPriceAths, name: "Max", color: colors.red, unit: "days" }),
          s({ metric: ath.maxYearsBetweenPriceAths, name: "Max", color: colors.red, unit: "years" }),
        ],
      },

      // Averages
      {
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
                s({
                  metric: (metricAddon === "sma" ? sma : ema).price,
                  name: id,
                  color,
                  unit: "usd",
                }),
              ),
            },
            ...averages.map(({ name, color, sma, ema }) => ({
              name,
              tree: createPriceWithRatioOptions(ctx, {
                ratio: metricAddon === "sma" ? sma : ema,
                name,
                title: `${name} Market Price ${nameAddon} Moving Average`,
                legend: "average",
                color,
              }),
            })),
          ],
        })),
      },

      // Performance
      {
        name: "Performance",
        tree: /** @type {const} */ ([
          ["1d", returns._1dPriceReturns, undefined],
          ["1w", returns._1wPriceReturns, undefined],
          ["1m", returns._1mPriceReturns, undefined],
          ["3m", returns._3mPriceReturns, undefined],
          ["6m", returns._6mPriceReturns, undefined],
          ["1y", returns._1yPriceReturns, undefined],
          ["2y", returns._2yPriceReturns, returns._2yCagr],
          ["3y", returns._3yPriceReturns, returns._3yCagr],
          ["4y", returns._4yPriceReturns, returns._4yCagr],
          ["5y", returns._5yPriceReturns, returns._5yCagr],
          ["6y", returns._6yPriceReturns, returns._6yCagr],
          ["8y", returns._8yPriceReturns, returns._8yCagr],
          ["10y", returns._10yPriceReturns, returns._10yCagr],
        ]).map(([id, priceReturns, cagr]) => {
          const name = periodIdToName(id, true);
          return {
            name,
            title: `${name} Performance`,
            bottom: [
              /** @type {AnyFetchedSeriesBlueprint} */ ({
                metric: priceReturns,
                title: "total",
                type: "Baseline",
                unit: "percentage",
              }),
              ...(cagr
                ? [
                    /** @type {AnyFetchedSeriesBlueprint} */ ({
                      metric: cagr,
                      title: "cagr",
                      type: "Baseline",
                      colors: [colors.lime, colors.pink],
                      unit: "percentage",
                    }),
                  ]
                : []),
              createPriceLine({ unit: "percentage" }),
            ],
          };
        }),
      },

      // Indicators
      {
        name: "Indicators",
        tree: [
          // Volatility
          {
            name: "Volatility",
            title: "Bitcoin Price Volatility Index",
            bottom: [
              s({ metric: volatility.price1wVolatility, name: "1w", color: colors.red, unit: "percentage" }),
              s({ metric: volatility.price1mVolatility, name: "1m", color: colors.orange, unit: "percentage" }),
              s({ metric: volatility.price1yVolatility, name: "1y", color: colors.lime, unit: "percentage" }),
            ],
          },

          // MinMax
          {
            name: "MinMax",
            tree: [
              { id: "1w", title: "1 Week", min: range.price1wMin, max: range.price1wMax },
              { id: "2w", title: "2 Week", min: range.price2wMin, max: range.price2wMax },
              { id: "1m", title: "1 Month", min: range.price1mMin, max: range.price1mMax },
              { id: "1y", title: "1 Year", min: range.price1yMin, max: range.price1yMax },
            ].map(({ id, title, min, max }) => ({
              name: id,
              title: `Bitcoin Price ${title} MinMax Bands`,
              top: [
                s({ metric: min, name: "min", color: colors.red, unit: "usd" }),
                s({ metric: max, name: "max", color: colors.green, unit: "usd" }),
              ],
            })),
          },

          // True range
          {
            name: "True range",
            title: "Bitcoin Price True Range",
            bottom: [s({ metric: range.priceTrueRange, name: "value", color: colors.yellow, unit: "usd" })],
          },

          // Choppiness
          {
            name: "Choppiness",
            title: "Bitcoin Price Choppiness Index",
            bottom: [
              s({ metric: range.price2wChoppinessIndex, name: "2w", color: colors.red, unit: "index" }),
              createPriceLine({ unit: "index", number: 61.8 }),
              createPriceLine({ unit: "index", number: 38.2 }),
            ],
          },

          // Mayer multiple
          {
            name: "Mayer multiple",
            title: "Mayer multiple",
            top: [
              s({ metric: movingAverage.price200dSma.price, name: "200d sma", color: colors.yellow, unit: "usd" }),
              s({ metric: movingAverage.price200dSmaX24, name: "200d sma x2.4", color: colors.green, unit: "usd" }),
              s({ metric: movingAverage.price200dSmaX08, name: "200d sma x0.8", color: colors.red, unit: "usd" }),
            ],
          },
        ],
      },

      // Investing
      {
        name: "Investing",
        tree: [
          // DCA vs Lump sum
          {
            name: "DCA vs Lump sum",
            tree: [
              .../** @type {const} */ ([
                ["1w", dca._1wDcaAvgPrice, lookback.price1wAgo, dca._1wDcaReturns, returns._1wPriceReturns],
                ["1m", dca._1mDcaAvgPrice, lookback.price1mAgo, dca._1mDcaReturns, returns._1mPriceReturns],
                ["3m", dca._3mDcaAvgPrice, lookback.price3mAgo, dca._3mDcaReturns, returns._3mPriceReturns],
                ["6m", dca._6mDcaAvgPrice, lookback.price6mAgo, dca._6mDcaReturns, returns._6mPriceReturns],
                ["1y", dca._1yDcaAvgPrice, lookback.price1yAgo, dca._1yDcaReturns, returns._1yPriceReturns],
              ]).map(([id, dcaAvgPrice, priceAgo, dcaReturns, priceReturns]) => {
                const name = periodIdToName(id, true);
                return {
                  name,
                  tree: [
                    {
                      name: "price",
                      title: `${name} DCA vs Lump Sum (Price)`,
                      top: [
                        s({ metric: dcaAvgPrice, name: "DCA avg", color: colors.green, unit: "usd" }),
                        s({ metric: priceAgo, name: "Lump sum", color: colors.orange, unit: "usd" }),
                      ],
                    },
                    {
                      name: "returns",
                      title: `${name} DCA vs Lump Sum (Returns)`,
                      bottom: [
                        /** @type {AnyFetchedSeriesBlueprint} */ ({
                          metric: dcaReturns,
                          title: "DCA",
                          type: "Baseline",
                          unit: "percentage",
                        }),
                        /** @type {AnyFetchedSeriesBlueprint} */ ({
                          metric: priceReturns,
                          title: "Lump sum",
                          type: "Baseline",
                          colors: [colors.lime, colors.red],
                          unit: "percentage",
                        }),
                        createPriceLine({ unit: "percentage" }),
                      ],
                    },
                  ],
                };
              }),
            ],
          },

          // DCA classes
          {
            name: "DCA classes",
            tree: [
              {
                name: "Average price",
                title: "DCA Average Price by Year",
                top: dcaClasses.map(({ year, color, defaultActive, avgPrice }) =>
                  s({ metric: avgPrice, name: `${year}`, color, defaultActive, unit: "usd" }),
                ),
              },
              {
                name: "Returns",
                title: "DCA Returns by Year",
                bottom: dcaClasses.map(({ year, color, defaultActive, returns }) =>
                  /** @type {AnyFetchedSeriesBlueprint} */ ({
                    metric: returns,
                    title: `${year}`,
                    type: "Baseline",
                    color,
                    defaultActive,
                    unit: "percentage",
                  }),
                ),
              },
              {
                name: "Stack",
                title: "DCA Stack by Year",
                bottom: dcaClasses.map(({ year, color, defaultActive, stack }) =>
                  s({ metric: stack, name: `${year}`, color, defaultActive, unit: "sats" }),
                ),
              },
            ],
          },
        ],
      },
    ],
  };
}
