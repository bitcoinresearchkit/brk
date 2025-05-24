// @ts-check

/**
 * @typedef {Height | DateIndex | WeekIndex | DifficultyEpoch | MonthIndex | QuarterIndex | YearIndex | HalvingEpoch | DecadeIndex} ChartableIndex
 */
/**
 * @template {readonly unknown[]} T
 * @typedef {Extract<T[number], ChartableIndex> extends never ? false : true} IncludesChartableIndex
 */
/**
 * @typedef {{[K in VecId]: IncludesChartableIndex<VecIdToIndexes[K]> extends true ? K : never}[VecId]} ChartableVecId
 */

/**
 * @typedef {Object} BaseSeriesBlueprint
 * @property {string} title
 * @property {boolean} [defaultActive]
 *
 * @typedef {Object} CreatePriceLine
 * @property {number} value
 *
 * @typedef {Object} CreatePriceLineOptions
 * @property {CreatePriceLine} createPriceLine
 *
 * @typedef {Object} BaselineSeriesBlueprintSpecific
 * @property {"Baseline"} type
 * @property {Color} [color]
 * @property {[Color, Color]} [colors]
 * @property {DeepPartial<BaselineStyleOptions & SeriesOptionsCommon & CreatePriceLineOptions>} [options]
 * @property {Accessor<BaselineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & BaselineSeriesBlueprintSpecific} BaselineSeriesBlueprint
 *
 * @typedef {Object} CandlestickSeriesBlueprintSpecific
 * @property {"Candlestick"} type
 * @property {Color} [color]
 * @property {DeepPartial<CandlestickStyleOptions & SeriesOptionsCommon>} [options]
 * @property {Accessor<CandlestickData[]>} [data]
 * @typedef {BaseSeriesBlueprint & CandlestickSeriesBlueprintSpecific} CandlestickSeriesBlueprint
 *
 * @typedef {Object} LineSeriesBlueprintSpecific
 * @property {"Line"} [type]
 * @property {Color} [color]
 * @property {DeepPartial<LineStyleOptions & SeriesOptionsCommon & CreatePriceLineOptions>} [options]
 * @property {Accessor<LineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & LineSeriesBlueprintSpecific} LineSeriesBlueprint
 *
 * @typedef {BaselineSeriesBlueprint | CandlestickSeriesBlueprint | LineSeriesBlueprint} AnySeriesBlueprint
 *
 * @typedef {AnySeriesBlueprint["type"]} SeriesType
 *
 * @typedef {{ key: ChartableVecId }} FetchedAnySeriesOptions
 *
 * @typedef {BaselineSeriesBlueprint & FetchedAnySeriesOptions} FetchedBaselineSeriesBlueprint
 * @typedef {CandlestickSeriesBlueprint & FetchedAnySeriesOptions} FetchedCandlestickSeriesBlueprint
 * @typedef {LineSeriesBlueprint & FetchedAnySeriesOptions} FetchedLineSeriesBlueprint
 * @typedef {AnySeriesBlueprint & FetchedAnySeriesOptions} AnyFetchedSeriesBlueprint
 *
 * @typedef {Object} PartialOption
 * @property {string} name
 *
 * @typedef {Object} ProcessedOptionAddons
 * @property {string} id
 * @property {string} title
 * @property {string[]} path
 *
 * @typedef {Object} PartialChartOptionSpecific
 * @property {"chart"} [kind]
 * @property {string} title
 * @property {AnyFetchedSeriesBlueprint[]} [top]
 * @property {AnyFetchedSeriesBlueprint[]} [bottom]
 *
 * @typedef {PartialOption & PartialChartOptionSpecific} PartialChartOption
 *
 * @typedef {Object} ProcessedChartOptionAddons
 * @property {Record<Unit, AnyFetchedSeriesBlueprint[]>} top
 * @property {Record<Unit, AnyFetchedSeriesBlueprint[]>} bottom
 *
 * @typedef {Required<Omit<PartialChartOption, "top" | "bottom">> & ProcessedChartOptionAddons & ProcessedOptionAddons} ChartOption
 *
 * @typedef {Object} PartialTableOptionSpecific
 * @property {"table"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialTableOptionSpecific} PartialTableOption
 *
 * @typedef {Required<PartialTableOption> & ProcessedOptionAddons} TableOption
 *
 * @typedef {Object} PartialSimulationOptionSpecific
 * @property {"simulation"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialSimulationOptionSpecific} PartialSimulationOption
 *
 * @typedef {Required<PartialSimulationOption> & ProcessedOptionAddons} SimulationOption
 *
 * @typedef {Object} PartialUrlOptionSpecific
 * @property {"url"} [kind]
 * @property {() => string} url
 * @property {boolean} [qrcode]
 *
 * @typedef {PartialOption & PartialUrlOptionSpecific} PartialUrlOption
 *
 * @typedef {Required<PartialUrlOption> & ProcessedOptionAddons} UrlOption
 *
 * @typedef {PartialChartOption | PartialTableOption | PartialSimulationOption | PartialUrlOption} AnyPartialOption
 *
 * @typedef {ChartOption | TableOption | SimulationOption | UrlOption} Option
 *
 * @typedef {Object} PartialOptionsGroup
 * @property {string} name
 * @property {PartialOptionsTree} tree
 *
 * @typedef {Object} OptionsGroup
 * @property {string} id
 * @property {string} name
 * @property {OptionsTree} tree
 *
 * @typedef {(AnyPartialOption | PartialOptionsGroup)[]} PartialOptionsTree
 *
 * @typedef {(Option | OptionsGroup)[]} OptionsTree
 *
 */

/**
 * @param {Colors} colors
 * @returns {PartialOptionsTree}
 */
function createPartialOptions(colors) {
  /**
   * @template {string} S
   * @typedef {Extract<ChartableVecId, `${S}${string}`>} StartsWith
   */
  /**
   * @template {string} S
   * @typedef {Extract<ChartableVecId, `${string}${S}`>} EndsWith
   */
  /**
   * @template {string} K
   * @template {string} S
   * @typedef {K extends `${S}${infer Rest}` ? Rest : never} WithoutPrefix
   */
  /**
   * @template {string} K
   * @template {string} S
   * @typedef {K extends `${infer Rest}${S}` ? Rest : never} WithoutSuffix
   */

  /**
   * @typedef {"total-"} TotalPrefix
   * @typedef {StartsWith<TotalPrefix>} TotalVecId
   * @typedef {WithoutPrefix<TotalVecId, TotalPrefix>} TotalVecIdBase
   * @typedef {"-sum"} SumSuffix
   * @typedef {EndsWith<SumSuffix>} VecIdSum
   * @typedef {WithoutSuffix<VecIdSum, SumSuffix>} VecIdSumBase
   * @typedef {"-average"} AverageSuffix
   * @typedef {EndsWith<AverageSuffix>} VecIdAverage
   * @typedef {WithoutSuffix<VecIdAverage, AverageSuffix>} VecIdAverageBase
   * @typedef {"-median"} MedianSuffix
   * @typedef {EndsWith<MedianSuffix>} VecIdMedian
   * @typedef {WithoutSuffix<VecIdMedian, MedianSuffix>} VecIdMedianBase
   * @typedef {"-90p"} _90pSuffix
   * @typedef {EndsWith<_90pSuffix>} VecId90p
   * @typedef {WithoutSuffix<VecId90p, _90pSuffix>} VecId90pBase
   * @typedef {"-75p"} _75pSuffix
   * @typedef {EndsWith<_75pSuffix>} VecId75p
   * @typedef {WithoutSuffix<VecId75p, _75pSuffix>} VecId75pBase
   * @typedef {"-25p"} _25pSuffix
   * @typedef {EndsWith<_25pSuffix>} VecId25p
   * @typedef {WithoutSuffix<VecId25p, _25pSuffix>} VecId25pBase
   * @typedef {"-10p"} _10pSuffix
   * @typedef {EndsWith<_10pSuffix>} VecId10p
   * @typedef {WithoutSuffix<VecId10p, _10pSuffix>} VecId10pBase
   * @typedef {"-max"} MaxSuffix
   * @typedef {EndsWith<MaxSuffix>} VecIdMax
   * @typedef {WithoutSuffix<VecIdMax, MaxSuffix>} VecIdMaxBase
   * @typedef {"-min"} MinSuffix
   * @typedef {EndsWith<MinSuffix>} VecIdMin
   * @typedef {WithoutSuffix<VecIdMin, MinSuffix>} VecIdMinBase
   */

  const averages = /** @type {const} */ ([
    { name: "1 Week", key: "1w", days: 7 },
    { name: "8 Days", key: "8d", days: 8 },
    { name: "13 Days", key: "13d", days: 13 },
    { name: "21 Days", key: "21d", days: 21 },
    { name: "1 Month", key: "1m", days: 30 },
    { name: "34 Days", key: "34d", days: 34 },
    { name: "55 Days", key: "55d", days: 55 },
    { name: "89 Days", key: "89d", days: 89 },
    { name: "144 Days", key: "144d", days: 144 },
    { name: "1 Year", key: "1y", days: 365 },
    { name: "2 Years", key: "2y", days: 2 * 365 },
    { name: "200 Weeks", key: "200w", days: 200 * 7 },
    { name: "4 Years", key: "4y", days: 4 * 365 },
  ]);

  const dcaClasses = /** @type {const} */ ([
    2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025,
  ]);

  /**
   * @param {Object} args
   * @param {ChartableVecId} args.key
   * @param {string} args.name
   * @param {Color} [args.color]
   * @param {boolean} [args.defaultActive]
   * @param {DeepPartial<LineStyleOptions & SeriesOptionsCommon>} [args.options]
   */
  function createBaseSeries({ key, name, color, defaultActive, options }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key,
      title: name,
      color,
      defaultActive,
      options,
    });
  }

  /**
   * @param {Object} args
   * @param {VecIdAverageBase} args.concat
   */
  function createAverageSeries({ concat }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key: `${concat}-average`,
      title: "Average",
    });
  }

  /**
   * @param {Object} args
   * @param {VecIdSumBase & TotalVecIdBase} args.concat
   * @param {string} [args.name]
   */
  function createSumTotalSeries({ concat, name }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        key: `${concat}-sum`,
        title: name ? `${name} Sum` : "Sum",
        color: colors.bitcoin,
      },
      {
        key: `total-${concat}`,
        title: name ? `Total ${name}` : "Total",
        color: colors.offBitcoin,
        defaultActive: false,
      },
    ]);
  }

  /**
   * @param {Object} args
   * @param {VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} args.concat
   */
  function createMinMaxPercentilesSeries({ concat }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        key: `${concat}-max`,
        title: "Max",
        color: colors.pink,
        defaultActive: false,
      },
      {
        key: `${concat}-min`,
        title: "Min",
        color: colors.green,
        defaultActive: false,
      },
      {
        key: `${concat}-median`,
        title: "Median",
        color: colors.amber,
        defaultActive: false,
      },
      {
        key: `${concat}-75p`,
        title: "75p",
        color: colors.red,
        defaultActive: false,
      },
      {
        key: `${concat}-25p`,
        title: "25p",
        color: colors.yellow,
        defaultActive: false,
      },
      {
        key: `${concat}-90p`,
        title: "90p",
        color: colors.rose,
        defaultActive: false,
      },
      {
        key: `${concat}-10p`,
        title: "10p",
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * @param {VecIdAverageBase & VecIdSumBase & TotalVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} key
   */
  function createAverageSumTotalMinMaxPercentilesSeries(key) {
    return [
      createAverageSeries({ concat: key }),
      ...createSumTotalSeries({ concat: key }),
      ...createMinMaxPercentilesSeries({ concat: key }),
    ];
  }

  /**
   * @param {Object} args
   * @param {ChartableVecId & VecIdAverageBase & VecIdSumBase & TotalVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} args.key
   * @param {string} args.name
   */
  function createBaseAverageSumTotalMinMaxPercentilesSeries({ key, name }) {
    return [
      createBaseSeries({
        key,
        name,
      }),
      ...createAverageSumTotalMinMaxPercentilesSeries(key),
    ];
  }

  /**
   * @param {Object} args
   * @param {ChartableVecId & VecIdSumBase & TotalVecIdBase} args.key
   * @param {string} args.name
   */
  function createBaseSumTotalSeries({ key, name }) {
    return [
      createBaseSeries({
        key,
        name,
      }),
      ...createSumTotalSeries({
        concat: key,
      }),
    ];
  }

  /**
   * @typedef {"-ratio"} RatioCapSuffix
   * @typedef {EndsWith<RatioCapSuffix>} VecIdRatioCap
   * @typedef {WithoutSuffix<VecIdRatioCap, RatioCapSuffix>} VecIdRatioCapBase
   */

  /**
   *
   * @param {Object} args
   * @param {string} args.name
   * @param {string} args.legend
   * @param {string} args.title
   * @param {VecIdRatioCapBase} args.key
   * @param {Color} [args.color]
   */
  function createPriceWithRatio({ name, title, legend, key, color }) {
    return {
      name,
      title,
      top: [
        createBaseSeries({
          key,
          name: legend,
          color,
        }),
        createBaseSeries({
          key: `${key}-ratio-p1sd-as-price`,
          name: "+1σ",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p2sd-as-price`,
          name: "+2σ",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p3sd-as-price`,
          name: "+3σ",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m1sd-as-price`,
          name: "−1σ",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m2sd-as-price`,
          name: "−2σ",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m3sd-as-price`,
          name: "−3σ",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99-as-price`,
          name: "p99",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99-5-as-price`,
          name: "p99.5",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99-9-as-price`,
          name: "p99.9",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p1-as-price`,
          name: "p1",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p0-5-as-price`,
          name: "p0.5",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p0-1-as-price`,
          name: "p0.1",
          color: colors.violet,
          defaultActive: false,
        }),
      ],
      bottom: [
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}-ratio`,
          title: "Ratio",
          type: "Baseline",
          options: {
            baseValue: { price: 1 },
            createPriceLine: {
              value: 1,
            },
          },
        }),
        createBaseSeries({
          key: `${key}-ratio-sma`,
          name: "sma",
          color: colors.yellow,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p1sd`,
          name: "+1σ",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p2sd`,
          name: "+2σ",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p3sd`,
          name: "+3σ",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m1sd`,
          name: "−1σ",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m2sd`,
          name: "−2σ",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-m3sd`,
          name: "−3σ",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99`,
          name: "p99",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99-5`,
          name: "p99.5",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p99-9`,
          name: "p99.9",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p1`,
          name: "p1",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p0-5`,
          name: "p0.5",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-p0-1`,
          name: "p0.1",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-1w-sma`,
          name: "1w sma",
          color: colors.fuchsia,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-1m-sma`,
          name: "1m sma",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}-ratio-1y-sma`,
          name: "1y sma",
          color: colors.rose,
          defaultActive: false,
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}-ratio-1y-sma-momentum-oscillator`,
          title: "1Y Momentum",
          type: "Baseline",
          options: {
            createPriceLine: {
              value: 0,
            },
          },
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}-ratio-zscore`,
          title: "Score",
          type: "Baseline",
          options: {
            createPriceLine: {
              value: 0,
            },
          },
        }),
      ],
    };
  }

  /**
   * @typedef {"-realized-cap"} RealizedCapSuffix
   * @typedef {EndsWith<RealizedCapSuffix>} VecIdRealizedCap
   * @typedef {WithoutSuffix<VecIdRealizedCap, RealizedCapSuffix>} VecIdRealizedCapBase
   */

  /**
   *
   * @param {Object} args
   * @param {string} args.name
   * @param {string} args.title
   * @param {"" | VecIdRealizedCapBase} args.key
   */
  function createUTXOGroupFolder({ name, title, key: _key }) {
    const key = _key
      ? /** @type {const} */ (`${_key}-`)
      : /** @type {const} */ ("");

    return /** @satisfies {PartialOptionsGroup} */ ({
      name: name || "all",
      tree: [
        {
          name: "supply",
          title: `${title} Supply`,
          bottom: [
            createBaseSeries({
              key: `${key}supply`,
              name: "Supply",
            }),
            createBaseSeries({
              key: `${key}supply-in-btc`,
              name: "Supply",
            }),
            createBaseSeries({
              key: `${key}supply-in-usd`,
              name: "Supply",
            }),
          ],
        },
        {
          name: "utxo count",
          title: `${title} UTXO Count`,
          bottom: [
            createBaseSeries({
              key: `${key}utxo-count`,
              name: "Count",
            }),
          ],
        },
        {
          name: "realized cap",
          title: `${title} Realized Capitalization`,
          bottom: [
            createBaseSeries({
              key: `${key}realized-cap`,
              name: "Realized Cap",
            }),
          ],
        },
        createPriceWithRatio({
          key: `${key}realized-price`,
          name: "realized price",
          legend: "realized",
          title: `${title} Realized Price`,
        }),
      ],
    });
  }

  return [
    {
      name: "Charts",
      tree: [
        {
          name: "Price",
          title: "Bitcoin Price",
        },
        {
          name: "Market",
          tree: [
            {
              name: "Capitalization",
              title: "Market Capitalization",
              bottom: [
                createBaseSeries({
                  key: "marketcap",
                  name: "Capitalization",
                }),
              ],
            },
            {
              name: "All Time High",
              tree: [
                {
                  name: "Value",
                  title: "All Time High",
                  top: [
                    createBaseSeries({
                      key: "ath",
                      name: "ath",
                    }),
                  ],
                },
                {
                  name: "drawdown",
                  title: "All Time High Drawdown",
                  top: [
                    createBaseSeries({
                      key: "ath",
                      name: "ath",
                    }),
                  ],
                  bottom: [
                    createBaseSeries({
                      key: "drawdown",
                      name: "Drawdown",
                      color: colors.red,
                    }),
                  ],
                },
                {
                  name: "days since",
                  title: "Number of days Since All Time High",
                  top: [
                    createBaseSeries({
                      key: "ath",
                      name: "ath",
                    }),
                  ],
                  bottom: [
                    createBaseSeries({
                      key: "days-since-ath",
                      name: "Days",
                    }),
                  ],
                },
                {
                  name: "max between",
                  title: "Maximum time between All Time Highs",
                  top: [
                    createBaseSeries({
                      key: "ath",
                      name: "ath",
                    }),
                  ],
                  bottom: [
                    createBaseSeries({
                      key: "max-days-between-ath",
                      name: "Days",
                    }),
                    createBaseSeries({
                      key: "max-years-between-ath",
                      name: "Years",
                    }),
                  ],
                },
              ],
            },
            {
              name: "Average",
              tree: [
                {
                  name: "Compare",
                  title: "Moving Averages",
                  top: averages.map(({ days, key, name }) =>
                    createBaseSeries({
                      key: `${key}-sma`,
                      name: key,
                      color: colors[`_${key}`],
                    }),
                  ),
                },
                ...averages.map(({ key, name }) =>
                  createPriceWithRatio({
                    key: `${key}-sma`,
                    name,
                    title: `${name} Market Price Moving Average`,
                    legend: "average",
                    color: colors[`_${key}`],
                  }),
                ),
              ],
            },
            {
              name: "Performance",
              tree: /** @type {const} */ ([
                { name: "1 Day", key: "1d" },
                { name: "1 Week", key: "1w" },
                { name: "1 Month", key: "1m" },
                { name: "3 Month", key: "3m" },
                { name: "6 Month", key: "6m" },
                { name: "1 Year", key: "1y" },
                { name: "2 Year", key: "2y" },
                { name: "3 Year", key: "3y" },
                { name: "4 Year", key: "4y" },
                { name: "5 Year", key: "5y" },
                { name: "6 Year", key: "6y" },
                { name: "8 Year", key: "8y" },
                { name: "10 Year", key: "10y" },
              ]).map(({ name, key }) => ({
                name,
                title: `${name} Performance`,
                bottom: [
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    key: `${key}-returns`,
                    title: "Returns",
                    type: "Baseline",
                    options: {
                      createPriceLine: {
                        value: 0,
                      },
                    },
                  }),
                ],
              })),
            },
          ],
        },
        {
          name: "Investing",
          tree: [
            {
              name: "DCA vs Lump sum",
              tree: [
                .../** @type {const} */ ([
                  { name: "1 Week", key: "1w" },
                  { name: "1 Month", key: "1m" },
                  { name: "3 Month", key: "3m" },
                  { name: "6 Month", key: "6m" },
                  { name: "1 Year", key: "1y" },
                ]).map(
                  ({ name, key }) =>
                    /** @satisfies {PartialChartOption} */ ({
                      name,
                      title: `${name} DCA vs Lump Sum Returns`,
                      top: [
                        createBaseSeries({
                          key: `${key}-dca-avg-price`,
                          name: `dca`,
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          key: `price-${key}-ago`,
                          name: `lump sum`,
                          color: colors.cyan,
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-dca-returns`,
                          title: "dca",
                          type: "Baseline",
                          colors: [colors.yellow, colors.pink],
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-returns`,
                          title: "lump sum",
                          type: "Baseline",
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                      ],
                    }),
                ),
                .../** @type {const} */ ([
                  { name: "2 Year", key: "2y" },
                  { name: "3 Year", key: "3y" },
                  { name: "4 Year", key: "4y" },
                  { name: "5 Year", key: "5y" },
                  { name: "6 Year", key: "6y" },
                  { name: "8 Year", key: "8y" },
                  { name: "10 Year", key: "10y" },
                ]).map(
                  ({ name, key }) =>
                    /** @satisfies {PartialChartOption} */ ({
                      name,
                      title: `${name} DCA vs Lump Sum Returns`,
                      top: [
                        createBaseSeries({
                          key: `${key}-dca-avg-price`,
                          name: `dca avg. price`,
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          key: `price-${key}-ago`,
                          name: `lump sum price`,
                          color: colors.cyan,
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-dca-returns`,
                          title: "dca",
                          type: "Baseline",
                          colors: [colors.yellow, colors.pink],
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-dca-cagr`,
                          title: "dca",
                          type: "Baseline",
                          colors: [colors.yellow, colors.pink],
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-returns`,
                          title: "lump sum",
                          type: "Baseline",
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}-cagr`,
                          title: "lump sum",
                          type: "Baseline",
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                      ],
                    }),
                ),
              ],
            },
            {
              name: "DCA Class",
              tree: [
                {
                  name: "Compare",
                  title: "DCA Classes",
                  top: dcaClasses.map((year) =>
                    createBaseSeries({
                      key: `dca-class-${year}-avg-price`,
                      name: `${year}`,
                      color: colors[year],
                    }),
                  ),
                },
                ...dcaClasses.map(
                  (year) =>
                    /** @satisfies {PartialChartOption} */ ({
                      name: `${year}`,
                      title: `DCA Since ${year}`,
                      top: [
                        createBaseSeries({
                          key: `dca-class-${year}-avg-price`,
                          name: `avg. price`,
                          color: colors[year],
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `dca-class-${year}-returns`,
                          title: "dca",
                          type: "Baseline",
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                      ],
                    }),
                ),
              ],
            },
          ],
        },
        {
          name: "Mining",
          tree: [
            {
              name: "Supply",
              title: "Circulating Supply",
              bottom: [
                createBaseSeries({
                  key: "total-subsidy-in-btc",
                  name: "Mined",
                }),
              ],
            },
            {
              name: "Coinbase",
              title: "Coinbase",
              bottom: [
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "coinbase",
                  name: "Coinbase",
                }),
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "coinbase-in-btc",
                  name: "Coinbase",
                }),
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "coinbase-in-usd",
                  name: "Coinbase",
                }),
              ],
            },
            {
              name: "Subsidy",
              title: "Subsidy",
              bottom: [
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "subsidy",
                  name: "Subsidy",
                }),
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "subsidy-in-btc",
                  name: "Subsidy",
                }),
                ...createBaseAverageSumTotalMinMaxPercentilesSeries({
                  key: "subsidy-in-usd",
                  name: "Subsidy",
                }),
              ],
            },
            {
              name: "Fee",
              title: "Transaction Fee",
              bottom: [
                ...createAverageSumTotalMinMaxPercentilesSeries("fee"),
                ...createAverageSumTotalMinMaxPercentilesSeries("fee-in-btc"),
                ...createAverageSumTotalMinMaxPercentilesSeries("fee-in-usd"),
              ],
            },
            {
              name: "Unclaimed Rewards",
              title: "Unclaimed Rewards",
              bottom: [
                ...createBaseSumTotalSeries({
                  key: "unclaimed-rewards",
                  name: "unclaimed",
                }),
                ...createBaseSumTotalSeries({
                  key: "unclaimed-rewards-in-btc",
                  name: "unclaimed",
                }),
                ...createBaseSumTotalSeries({
                  key: "unclaimed-rewards-in-usd",
                  name: "unclaimed",
                }),
              ],
            },
            {
              name: "Feerate",
              title: "Transaction Fee Rate",
              bottom: [
                createAverageSeries({ concat: "feerate" }),
                ...createMinMaxPercentilesSeries({
                  concat: "feerate",
                }),
              ],
            },
            {
              name: "Difficulty",
              title: "Difficulty",
              bottom: [
                createBaseSeries({
                  key: "difficulty",
                  name: "Value",
                }),
              ],
            },
            {
              name: "Difficulty Epoch",
              title: "Difficulty Epoch",
              bottom: [
                createBaseSeries({
                  key: "difficultyepoch",
                  name: "Epoch",
                }),
              ],
            },
            {
              name: "Halving Epoch",
              title: "Halving Epoch",
              bottom: [
                createBaseSeries({
                  key: "halvingepoch",
                  name: "Epoch",
                }),
              ],
            },
          ],
        },
        {
          name: "Block",
          tree: [
            {
              name: "Count",
              title: "Block Count",
              bottom: [
                createBaseSeries({
                  key: "block-count",
                  name: "Count",
                }),
                ...createSumTotalSeries({ concat: "block-count" }),
              ],
            },
            {
              name: "Interval",
              title: "Block Interval",
              bottom: [
                createBaseSeries({
                  key: "interval",
                  name: "Interval",
                }),
                createAverageSeries({ concat: "block-interval" }),
                ...createMinMaxPercentilesSeries({
                  concat: "block-interval",
                }),
              ],
            },
            {
              name: "Size",
              title: "Block Size",
              bottom: [
                createBaseSeries({
                  key: "total-size",
                  name: "Size",
                }),
                ...createSumTotalSeries({ concat: "block-size" }),
              ],
            },
            {
              name: "Weight",
              title: "Block Weight",
              bottom: [
                createBaseSeries({
                  key: "weight",
                  name: "Weight",
                }),
                ...createSumTotalSeries({ concat: "block-weight" }),
              ],
            },
            {
              name: "Vbytes",
              title: "Block Virtual Bytes",
              bottom: [
                createBaseSeries({
                  key: "vbytes",
                  name: "Vbytes",
                }),
                ...createSumTotalSeries({ concat: "block-vbytes" }),
              ],
            },
          ],
        },
        {
          name: "Transaction",
          tree: [
            {
              name: "Count",
              title: "Transaction Count",
              bottom: createBaseAverageSumTotalMinMaxPercentilesSeries({
                key: "tx-count",
                name: "Count",
              }),
            },
            {
              name: "Weight",
              title: "Transaction Weight",
              bottom: [
                createAverageSeries({ concat: "tx-weight" }),
                ...createMinMaxPercentilesSeries({
                  concat: "tx-weight",
                }),
              ],
            },
            {
              name: "vsize",
              title: "Transaction Virtual Size",
              bottom: [
                createAverageSeries({ concat: "tx-vsize" }),
                ...createMinMaxPercentilesSeries({
                  concat: "tx-vsize",
                }),
              ],
            },
            {
              name: "Versions",
              title: "Transaction Versions",
              bottom: [
                createBaseSeries({
                  key: "tx-v1",
                  name: "v1 Count",
                }),
                ...createSumTotalSeries({ concat: "tx-v1", name: "v1" }),
                createBaseSeries({
                  key: "tx-v2",
                  name: "v2 Count",
                }),
                ...createSumTotalSeries({ concat: "tx-v2", name: "v2" }),
                createBaseSeries({
                  key: "tx-v3",
                  name: "v3 Count",
                }),
                ...createSumTotalSeries({ concat: "tx-v3", name: "v3" }),
              ],
            },
          ],
        },
        {
          name: "Input",
          tree: [
            {
              name: "Count",
              title: "Transaction Input Count",
              bottom: [
                createAverageSeries({ concat: "input-count" }),
                ...createSumTotalSeries({ concat: "input-count" }),
                ...createMinMaxPercentilesSeries({
                  concat: "input-count",
                }),
              ],
            },
            // {
            //   name: "Value",
            //   title: "Transaction Input Value",
            //   bottom: [
            //     createAverageSeries({ concat: "input-value" }),
            //     ...createSumTotalSeries({ concat: "input-value" }),
            //   ],
            // },
          ],
        },
        {
          name: "Output",
          tree: [
            {
              name: "Count",
              title: "Transaction Output Count",
              bottom: [
                createAverageSeries({ concat: "output-count" }),
                ...createSumTotalSeries({ concat: "output-count" }),
                ...createMinMaxPercentilesSeries({
                  concat: "output-count",
                }),
              ],
            },
            {
              name: "Unspent Count",
              title: "Unspent Transaction Output Count",
              bottom: [
                createBaseSeries({
                  key: "exact-utxo-count",
                  name: "total",
                }),
              ],
            },
            // {
            //   name: "Value",
            //   title: "Transaction Output Value",
            //   bottom: [
            //     createAverageSeries({ concat: "output-value" }),
            //     ...createSumTotalSeries({ concat: "output-value" }),
            //   ],
            // },
            {
              name: "types",
              tree: [
                {
                  name: "p2pk",
                  title: "Pay To Public Key Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2pk33-count",
                      name: "33B Count",
                    }),
                    createBaseSeries({
                      key: "p2pk33-count-sum",
                      name: "33B sum",
                    }),
                    createBaseSeries({
                      key: "total-p2pk33-count",
                      name: "33B total",
                    }),
                    createBaseSeries({
                      key: "p2pk65-count",
                      name: "65B Count",
                    }),
                    createBaseSeries({
                      key: "p2pk65-count-sum",
                      name: "65B sum",
                    }),
                    createBaseSeries({
                      key: "total-p2pk65-count",
                      name: "65B total",
                    }),
                  ],
                },
                {
                  name: "p2pkh",
                  title: "Pay To Public Key Hash Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2pkh-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2pkh-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2pkh-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2ms",
                  title: "Pay To Multisig Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2ms-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2ms-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2ms-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2sh",
                  title: "Pay To Script Hash Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2sh-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2sh-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2sh-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "op_return",
                  title: "op_return outputs",
                  bottom: [
                    createBaseSeries({ key: "opreturn-count", name: "Count" }),
                    createBaseSeries({
                      key: "opreturn-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-opreturn-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2wpkh",
                  title: "Pay To Witness Public Key Hash Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2wpkh-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2wpkh-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2wpkh-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2wsh",
                  title: "Pay To Witness Script Hash Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2wsh-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2wsh-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2wsh-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2tr",
                  title: "Pay To Taproot Outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2tr-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2tr-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2tr-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "p2a",
                  title: "Pay To Anchor outputs",
                  bottom: [
                    createBaseSeries({
                      key: "p2a-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "p2a-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-p2a-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "empty",
                  title: "empty outputs",
                  bottom: [
                    createBaseSeries({
                      key: "emptyoutput-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "emptyoutput-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-emptyoutput-count",
                      name: "total",
                    }),
                  ],
                },
                {
                  name: "unknown",
                  title: "unknown outputs",
                  bottom: [
                    createBaseSeries({
                      key: "unknownoutput-count",
                      name: "Count",
                    }),
                    createBaseSeries({
                      key: "unknownoutput-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "total-unknownoutput-count",
                      name: "total",
                    }),
                  ],
                },
              ],
              // title: "Transaction Output Value",
              // bottom: [
              //   createAverageSeries({ concat: "output-value" }),
              //   ...createSumTotalSeries({ concat: "output-value" }),
              // ],
            },
          ],
        },
        {
          name: "UTXOs",
          tree: [
            createUTXOGroupFolder({
              key: "",
              name: "",
              title: "",
            }),
            {
              name: "term",
              tree: [
                createUTXOGroupFolder({
                  key: "sth",
                  name: "short",
                  title: "Short Term Holders",
                }),
                createUTXOGroupFolder({
                  key: "lth",
                  name: "long",
                  title: "Long Term Holders",
                }),
              ],
            },
            {
              name: "From",
              tree: [
                createUTXOGroupFolder({
                  key: "from-1d",
                  name: "1d",
                  title: "From 1 Day",
                }),
                createUTXOGroupFolder({
                  key: "from-1w",
                  name: "1w",
                  title: "From 1 Week",
                }),
                createUTXOGroupFolder({
                  key: "from-1m",
                  name: "1m",
                  title: "From 1 Month",
                }),
                createUTXOGroupFolder({
                  key: "from-2m",
                  name: "2m",
                  title: "From 2 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-3m",
                  name: "3m",
                  title: "From 3 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-4m",
                  name: "4m",
                  title: "From 4 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-5m",
                  name: "5m",
                  title: "From 5 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-6m",
                  name: "6m",
                  title: "From 6 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-1y",
                  name: "1y",
                  title: "From 1 Year",
                }),
                createUTXOGroupFolder({
                  key: "from-2y",
                  name: "2y",
                  title: "From 2 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-3y",
                  name: "3y",
                  title: "From 3 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-4y",
                  name: "4y",
                  title: "From 4 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-5y",
                  name: "5y",
                  title: "From 5 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-6y",
                  name: "6y",
                  title: "From 6 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-7y",
                  name: "7y",
                  title: "From 7 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-8y",
                  name: "8y",
                  title: "From 8 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-10y",
                  name: "10y",
                  title: "From 10 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-15y",
                  name: "15y",
                  title: "From 15 Years",
                }),
              ],
            },
            {
              name: "Range",
              tree: [
                createUTXOGroupFolder({
                  key: "from-1d-to-1w",
                  name: "1d..1w",
                  title: "Between 1 Day and 1 Week",
                }),
                createUTXOGroupFolder({
                  key: "from-1w-to-1m",
                  name: "1w..1m",
                  title: "Between 1 Week and 1 Month",
                }),
                createUTXOGroupFolder({
                  key: "from-1m-to-3m",
                  name: "1m..3m",
                  title: "Between 1 Month and 3 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-3m-to-6m",
                  name: "3m..6m",
                  title: "Between 3 Month and 6 Months",
                }),
                createUTXOGroupFolder({
                  key: "from-6m-to-1y",
                  name: "6m..1y",
                  title: "Between 6 Months and 1 Year",
                }),
                createUTXOGroupFolder({
                  key: "from-1y-to-2y",
                  name: "1y..2y",
                  title: "Between 1 Year and 2 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-2y-to-3y",
                  name: "2y..3y",
                  title: "Between 2 Years and 3 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-3y-to-4y",
                  name: "3y..4y",
                  title: "Between 3 Years and 4 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-4y-to-5y",
                  name: "4y..5y",
                  title: "Between 4 Years and 5 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-5y-to-7y",
                  name: "5y..7y",
                  title: "Between 5 Years and 7 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-7y-to-10y",
                  name: "7y..10y",
                  title: "Between 7 Years and 10 Years",
                }),
                createUTXOGroupFolder({
                  key: "from-10y-to-15y",
                  name: "10y..15y",
                  title: "Between 10 Years and 15 Years",
                }),
              ],
            },
            {
              name: "Epoch",
              tree: [
                createUTXOGroupFolder({
                  key: "epoch-0",
                  name: "0",
                  title: "Epoch 0",
                }),
                createUTXOGroupFolder({
                  key: "epoch-1",
                  name: "1",
                  title: "Epoch 1",
                }),
                createUTXOGroupFolder({
                  key: "epoch-2",
                  name: "2",
                  title: "Epoch 2",
                }),
                createUTXOGroupFolder({
                  key: "epoch-3",
                  name: "3",
                  title: "Epoch 3",
                }),
                createUTXOGroupFolder({
                  key: "epoch-4",
                  name: "4",
                  title: "Epoch 4",
                }),
              ],
            },
            {
              name: "size",
              tree: [
                createUTXOGroupFolder({
                  key: "0sat",
                  name: "0sat",
                  title: "0 sat",
                }),
                createUTXOGroupFolder({
                  key: "from-1sat-to-10sats",
                  name: "1sat..10sats",
                  title: "From 1 sat to 10 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-10sats-to-100sats",
                  name: "10sat..100sats",
                  title: "From 10 sats to 100 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-100sats-to-1-000sats",
                  name: "100sat..1_000sats",
                  title: "From 100 sats to 1,000 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-1-000sats-to-10-000sats",
                  name: "1_000sat..10_000sats",
                  title: "From 1,000 sats to 10,000 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-10-000sats-to-100-000sats",
                  name: "10_000sat..100_000sats",
                  title: "From 10,000 sats to 100,000 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-100-000sats-to-1-000-000sats",
                  name: "100_000sat..1_000_000sats",
                  title: "From 100,000 sats to 1,000,000 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-1-000-000sats-to-10-000-000sats",
                  name: "1_000_000sat..10_000_000sats",
                  title: "From 1,000,000 sats to 10,000,000 sats",
                }),
                createUTXOGroupFolder({
                  key: "from-10-000-000sats-to-1btc",
                  name: "10_000_000sat..1btc",
                  title: "From 10,000,000 sats to 1 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-1btc-to-10btc",
                  name: "1btc..10btc",
                  title: "From 1 BTC to 10 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-10btc-to-100btc",
                  name: "10btc..100btc",
                  title: "From 10 BTC to 100 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-100btc-to-1-000btc",
                  name: "100btc..1_000btc",
                  title: "From 100 BTC to 1,000 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-1-000btc-to-10-000btc",
                  name: "1_000btc..10_000btc",
                  title: "From 1,000 BTC to 10,000 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-10-000btc-to-100-000btc",
                  name: "10_000btc..100_000btc",
                  title: "From 10,000 BTC to 100,000 BTC",
                }),
                createUTXOGroupFolder({
                  key: "from-100-000btc",
                  name: "100_000btc+",
                  title: "From 100,000 BTC",
                }),
              ],
            },
            {
              name: "type",
              tree: [
                createUTXOGroupFolder({
                  key: "p2pk65",
                  name: "p2pk65",
                  title: "Pay To Long Public Key",
                }),
                createUTXOGroupFolder({
                  key: "p2pk33",
                  name: "p2pk33",
                  title: "Pay To Short Public Key",
                }),
                createUTXOGroupFolder({
                  key: "p2pkh",
                  name: "p2pkh",
                  title: "Pay To Public Key Hash",
                }),
                createUTXOGroupFolder({
                  key: "p2ms",
                  name: "p2ms",
                  title: "Pay To Bare Multisig",
                }),
                createUTXOGroupFolder({
                  key: "p2sh",
                  name: "p2sh",
                  title: "Pay To Script Hash",
                }),
                createUTXOGroupFolder({
                  key: "p2wpkh",
                  name: "p2wpkh",
                  title: "Pay To Witness Public Key Hash",
                }),
                createUTXOGroupFolder({
                  key: "p2wsh",
                  name: "p2wsh",
                  title: "Pay To Witness Script Hash",
                }),
                createUTXOGroupFolder({
                  key: "p2tr",
                  name: "p2tr",
                  title: "Pay To Taproot",
                }),
                createUTXOGroupFolder({
                  key: "p2a",
                  name: "p2a",
                  title: "Pay To Anchor",
                }),
                createUTXOGroupFolder({
                  key: "unknown",
                  name: "unknown",
                  title: "Pay To Unknown",
                }),
                createUTXOGroupFolder({
                  key: "empty",
                  name: "empty",
                  title: "Pay To Empty",
                }),
              ],
            },
            {
              name: "Unspendable",
              tree: [
                {
                  name: "supply",
                  title: "Unspendable Supply",
                  bottom: [
                    createBaseSeries({
                      key: "unspendable-supply",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "unspendable-supply-in-btc",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "unspendable-supply-in-usd",
                      name: "Supply",
                    }),
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
    {
      kind: "table",
      title: "Table",
      name: "Table",
    },
    {
      name: "Simulations",
      tree: [
        {
          kind: "simulation",
          title: "Save In Bitcoin",
          name: "Save In Bitcoin",
        },
      ],
    },
    {
      name: "Social",
      tree: [
        {
          name: "Github",
          url: () => "https://github.com/bitcoinresearchkit/brk",
        },
        {
          name: "Nostr",
          url: () =>
            "https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44",
        },
        {
          name: "Discord",
          url: () => "https://discord.com/invite/HaR3wpH3nr",
        },
        {
          name: "Bluesky",
          url: () => "https://bsky.app/profile/bitcoinresearchkit.org",
        },
        {
          name: "x",
          url: () => "https://x.com/brkdotorg",
        },
      ],
    },
    {
      name: "Developers",
      tree: [
        {
          name: "API",
          url: () => "/api",
        },
        {
          name: "Source",
          url: () => "https://github.com/bitcoinresearchkit/brk",
        },
        {
          name: "Status",
          url: () => "https://status.kibo.money/",
        },
        {
          name: "Crates",
          url: () => "https://crates.io/crates/brk",
        },
      ],
    },
    {
      name: "Donations",
      tree: [
        {
          name: "Bitcoin QR Code",
          qrcode: true,
          url: () => "bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve",
        },
        {
          name: "Lightning QR Code",
          qrcode: true,
          url: () =>
            "lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4",
        },
        {
          name: "Geyser",
          url: () => "https://geyser.fund/project/brk",
        },
        {
          name: "OpenSats",
          url: () => "https://opensats.org/",
        },
      ],
    },
    {
      name: "Share",
      qrcode: true,
      url: () => window.location.href,
    },
  ];
}

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Signals} args.signals
 * @param {Env} args.env
 * @param {Utilities} args.utils
 * @param {WebSockets} args.webSockets
 * @param {Signal<string | null>} args.qrcode
 */
export function initOptions({
  colors,
  signals,
  env,
  utils,
  webSockets,
  qrcode,
}) {
  const LS_SELECTED_KEY = `selected-id`;

  const urlSelected = utils.url.pathnameToSelectedId();
  const savedSelectedId = localStorage.getItem(LS_SELECTED_KEY);

  /** @type {Signal<Option>} */
  const selected = signals.createSignal(/** @type {any} */ (undefined));

  const partialOptions = createPartialOptions(colors);

  /** @type {Option[]} */
  const list = [];

  /** @type {HTMLDetailsElement[]} */
  const detailsList = [];

  const treeElement = signals.createSignal(
    /** @type {HTMLDivElement | null} */ (null),
  );

  /** @type {string[] | undefined} */
  const optionsIds = env.localhost ? [] : undefined;

  /**
   * @param {AnyFetchedSeriesBlueprint[]} [arr]
   * @param {string} id
   */
  function arrayToRecord(id, arr = []) {
    return (arr || []).reduce((record, blueprint) => {
      const unit = utils.vecidToUnit(blueprint.key);
      record[unit] ??= [];
      record[unit].push(blueprint);
      return record;
    }, /** @type {Record<Unit, AnyFetchedSeriesBlueprint[]>} */ ({}));
  }

  /**
   * @param {Object} args
   * @param {Option} args.option
   * @param {string} args.frame
   * @param {Signal<string | null>} args.qrcode
   * @param {string} [args.name]
   * @param {string} [args.id]
   * @param {Owner | null} [args.owner]
   */
  function createOptionElement({ option, frame, name, id, owner, qrcode }) {
    if (option.kind === "url") {
      const href = option.url();

      if (option.qrcode) {
        return utils.dom.createButtonElement({
          inside: option.name,
          title: option.title,
          onClick: () => {
            qrcode.set(option.url);
          },
        });
      } else {
        return utils.dom.createAnchorElement({
          href,
          blank: true,
          text: option.name,
        });
      }
    } else {
      const { input, label } = utils.dom.createLabeledInput({
        inputId: `${option.id}-${frame}${id || ""}-selector`,
        inputValue: option.id,
        inputName: `option-${frame}${id || ""}`,
        labelTitle: option.title,
        onClick: () => {
          selected.set(option);
        },
        type: "radio",
      });

      const anchor = utils.dom.createAnchorElement({
        href: `/${option.id}`,
        text: name || option.name,
        onClick: () => {},
      });

      label.append(anchor);

      function createCheckEffect() {
        signals.createEffect(selected, (selected) => {
          if (selected?.id === option.id) {
            input.checked = true;
            localStorage.setItem(LS_SELECTED_KEY, option.id);
          } else if (input.checked) {
            input.checked = false;
          }
        });
      }

      if (owner !== undefined) {
        signals.runWithOwner(owner, () => {
          createCheckEffect();
        });
      } else {
        createCheckEffect();
      }

      return label;
    }
  }

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {Accessor<HTMLDivElement | HTMLDetailsElement | null>} parent
   * @param {string[] | undefined} path
   * @returns {Accessor<number>}
   */
  function recursiveProcessPartialTree(partialTree, parent, path = undefined) {
    /** @type {Accessor<number>[]} */
    const listForSum = [];

    const ul = signals.createMemo(
      // @ts-ignore
      (_previous) => {
        const previous = /** @type {HTMLUListElement | null} */ (_previous);
        previous?.remove();

        const _parent = parent();
        if (_parent) {
          if ("open" in _parent && !_parent.open) {
            throw "Set accesor to null instead";
          }

          const ul = window.document.createElement("ul");
          _parent.append(ul);
          return ul;
        } else {
          return null;
        }
      },
      null,
    );

    partialTree.forEach((anyPartial, partialIndex) => {
      const renderLi = signals.createSignal(true);

      const li = signals.createMemo((_previous) => {
        const previous = _previous;
        previous?.remove();

        const _ul = ul();

        if (renderLi() && _ul) {
          const li = window.document.createElement("li");
          utils.dom.insertElementAtIndex(_ul, li, partialIndex);
          return li;
        } else {
          return null;
        }
      }, /** @type {HTMLLIElement | null} */ (null));

      if ("tree" in anyPartial) {
        const folderId = utils.stringToId(
          `${(path || []).join(" ")} ${anyPartial.name} folder`,
        );

        /** @type {Omit<OptionsGroup, keyof PartialOptionsGroup>} */
        const groupAddons = {
          id: folderId,
        };

        Object.assign(anyPartial, groupAddons);

        optionsIds?.push(groupAddons.id);

        const thisPath = groupAddons.id;

        const passedDetails = signals.createSignal(
          /** @type {HTMLDivElement | HTMLDetailsElement | null} */ (null),
        );

        const childOptionsCount = recursiveProcessPartialTree(
          anyPartial.tree,
          passedDetails,
          [...(path || []), thisPath],
        );

        listForSum.push(childOptionsCount);

        signals.createEffect(li, (li) => {
          if (!li) {
            passedDetails.set(null);
            return;
          }

          signals.createEffect(selected, (selected) => {
            if (selected.path.includes(thisPath)) {
              li.dataset.highlight = "";
            } else {
              delete li.dataset.highlight;
            }
          });

          const details = window.document.createElement("details");
          details.id = folderId;
          detailsList.push(details);
          li.appendChild(details);

          const summary = window.document.createElement("summary");
          details.append(summary);
          summary.append(anyPartial.name);

          const supCount = window.document.createElement("sup");
          summary.append(supCount);

          signals.createEffect(childOptionsCount, (childOptionsCount) => {
            supCount.innerHTML = childOptionsCount.toLocaleString();
          });

          details.addEventListener("toggle", () => {
            const open = details.open;

            if (open) {
              passedDetails.set(details);
            } else {
              passedDetails.set(null);
            }
          });
        });

        function createRenderLiEffect() {
          signals.createEffect(childOptionsCount, (count) => {
            renderLi.set(!!count);
          });
        }
        createRenderLiEffect();
      } else {
        /** @type {Option} */
        let option;

        if ("kind" in anyPartial && anyPartial.kind === "table") {
          option = /** @satisfies {TableOption} */ ({
            kind: anyPartial.kind,
            id: anyPartial.kind,
            name: anyPartial.name,
            path: path || [],
            title: anyPartial.title,
          });
        } else if ("kind" in anyPartial && anyPartial.kind === "simulation") {
          option = /** @satisfies {SimulationOption} */ ({
            kind: anyPartial.kind,
            id: anyPartial.kind,
            name: anyPartial.name,
            path: path || [],
            title: anyPartial.title,
          });
        } else if ("url" in anyPartial) {
          option = /** @satisfies {UrlOption} */ ({
            kind: "url",
            id: `${utils.stringToId(anyPartial.name)}-url`,
            name: anyPartial.name,
            path: path || [],
            title: anyPartial.name,
            qrcode: !!anyPartial.qrcode,
            url: anyPartial.url,
          });
        } else {
          const title = anyPartial.title || anyPartial.name;
          const id = `chart-${utils.stringToId(title)}`;
          option = /** @satisfies {ChartOption} */ ({
            kind: "chart",
            id,
            name: anyPartial.name,
            title,
            path: path || [],
            top: arrayToRecord(id, anyPartial.top),
            bottom: arrayToRecord(id, anyPartial.bottom),
          });
        }

        if (urlSelected === option.id) {
          selected.set(option);
        } else if (!selected() && savedSelectedId === option.id) {
          selected.set(option);
        }

        list.push(option);
        optionsIds?.push(option.id);

        signals.createEffect(li, (li) => {
          if (!li) {
            return;
          }

          signals.createEffect(selected, (selected) => {
            if (selected === option) {
              li.dataset.highlight = "";
            } else {
              delete li.dataset.highlight;
            }
          });

          const element = createOptionElement({
            option,
            frame: "nav",
            qrcode,
          });

          li.append(element);
        });

        listForSum.push(() => 1);
      }
    });

    return signals.createMemo(() =>
      listForSum.reduce((acc, s) => acc + s(), 0),
    );
  }
  recursiveProcessPartialTree(partialOptions, treeElement);

  function setDefaultSelectedIfNeeded() {
    if (!selected()) {
      selected.set(list[0]);
    }
  }
  setDefaultSelectedIfNeeded();

  if (env.localhost) {
    function checkUniqueIds() {
      if (!optionsIds) {
        throw "Should be set";
      } else if (optionsIds.length !== new Set(optionsIds).size) {
        /** @type {Map<string, number>} */
        const m = new Map();

        optionsIds.forEach((id) => {
          m.set(id, (m.get(id) || 0) + 1);
        });

        console.log(
          [...m.entries()]
            .filter(([_, value]) => value > 1)
            .map(([key, _]) => key),
        );

        throw Error("ID duplicate");
      }
    }
    checkUniqueIds();
  }

  return {
    selected,
    list,
    details: detailsList,
    tree: /** @type {OptionsTree} */ (partialOptions),
    treeElement,
    createOptionElement,
  };
}
/** @typedef {ReturnType<typeof initOptions>} Options */

// /**
//  * @param {Number | undefined} value
//  * @param {Unit} unit
//  */
// function formatValue(value, unit) {
//   if (!value) return "";

//   const s =
//     unit !== "Count"
//       ? utils.locale.numberToShortUSFormat(value)
//       : utils.locale.numberToUSFormat(
//           value,
//           unit === "Count" ? 0 : undefined,
//         );

//   switch (unit) {
//     case "USD": {
//       return `$${s}`;
//     }
//     case "Bitcoin": {
//       return `₿${s}`;
//     }
//     case "Percentage": {
//       return `${s}%`;
//     }
//     case "Seconds": {
//       return `${s} sec`;
//     }
//     case "Megabytes": {
//       return `${s} MB`;
//     }
//     default: {
//       return s;
//     }
//   }
// }

/**
 type DefaultCohortOption = CohortOption<AnyPossibleCohortId>;

 interface CohortOption<Id extends AnyPossibleCohortId> {
   name: string;
   title: string;
   datasetId: Id;
   color: Color;
   filenameAddon?: string;
 }

 type DefaultCohortOptions = CohortOptions<AnyPossibleCohortId>;

 interface CohortOptions<Id extends AnyPossibleCohortId> {
   name: string;
   title: string;
   list: CohortOption<Id>[];
 }

 interface RatioOption {
   color: Color;
   // valueDatasetPath: AnyDatasetPath;
   // ratioDatasetPath: AnyDatasetPath;
   title: string;
 }

 interface RatioOptions {
   title: string;
   list: RatioOption[];
 }
*/

// function initGroups() {
//   const xTermHolders = /** @type {const} */ ([
//     {
//       id: "sth",
//       key: "sth",
//       name: "Short Term Holders",
//       legend: "Short Term Holders - STH",
//     },
//     {
//       id: "lth",
//       key: "lth",
//       name: "Long Term Holders",
//       legend: "Long Term Holders - LTH",
//     },
//   ]);

//   const upTo = /** @type {const} */ ([
//     {
//       id: "up-to-1d",
//       key: "up_to_1d",
//       name: "Up To 1 Day",
//       legend: "1D",
//     },
//     {
//       id: "up-to-1w",
//       key: "up_to_1w",
//       name: "Up To 1 Week",
//       legend: "1W",
//     },
//     {
//       id: "up-to-1m",
//       key: "up_to_1m",
//       name: "Up To 1 Month",
//       legend: "1M",
//     },
//     {
//       id: "up-to-2m",
//       key: "up_to_2m",
//       name: "Up To 2 Months",
//       legend: "2M",
//     },
//     {
//       id: "up-to-3m",
//       key: "up_to_3m",
//       name: "Up To 3 Months",
//       legend: "3M",
//     },
//     {
//       id: "up-to-4m",
//       key: "up_to_4m",
//       name: "Up To 4 Months",
//       legend: "4M",
//     },
//     {
//       id: "up-to-5m",
//       key: "up_to_5m",
//       name: "Up To 5 Months",
//       legend: "5M",
//     },
//     {
//       id: "up-to-6m",
//       key: "up_to_6m",
//       name: "Up To 6 Months",
//       legend: "6M",
//     },
//     {
//       id: "up-to-1y",
//       key: "up_to_1y",
//       name: "Up To 1 Year",
//       legend: "1Y",
//     },
//     {
//       id: "up-to-2y",
//       key: "up_to_2y",
//       name: "Up To 2 Years",
//       legend: "2Y",
//     },
//     {
//       id: "up-to-3y",
//       key: "up_to_3y",
//       name: "Up To 3 Years",
//       legend: "3Y",
//     },
//     {
//       id: "up-to-5y",
//       key: "up_to_5y",
//       name: "Up To 5 Years",
//       legend: "5Y",
//     },
//     {
//       id: "up-to-7y",
//       key: "up_to_7y",
//       name: "Up To 7 Years",
//       legend: "7Y",
//     },
//     {
//       id: "up-to-10y",
//       key: "up_to_10y",
//       name: "Up To 10 Years",
//       legend: "10Y",
//     },
//     {
//       id: "up-to-15y",
//       key: "up_to_15y",
//       name: "Up To 15 Years",
//       legend: "15Y",
//     },
//   ]);

//   const fromXToY = /** @type {const} */ ([
//     {
//       id: "up-to-1d",
//       key: "up_to_1d",
//       name: "24h",
//       legend: "24h",
//     },
//     {
//       id: "from-1d-to-1w",
//       key: "from_1d_to_1w",
//       name: "From 1 Day To 1 Week",
//       legend: "1D — 1W",
//     },
//     {
//       id: "from-1w-to-1m",
//       key: "from_1w_to_1m",
//       name: "From 1 Week To 1 Month",
//       legend: "1W — 1M",
//     },
//     {
//       id: "from-1m-to-3m",
//       key: "from_1m_to_3m",
//       name: "From 1 Month To 3 Months",
//       legend: "1M — 3M",
//     },
//     {
//       id: "from-3m-to-6m",
//       key: "from_3m_to_6m",
//       name: "From 3 Months To 6 Months",
//       legend: "3M — 6M",
//     },
//     {
//       id: "from-6m-to-1y",
//       key: "from_6m_to_1y",
//       name: "From 6 Months To 1 Year",
//       legend: "6M — 1Y",
//     },
//     {
//       id: "from-1y-to-2y",
//       key: "from_1y_to_2y",
//       name: "From 1 Year To 2 Years",
//       legend: "1Y — 2Y",
//     },
//     {
//       id: "from-2y-to-3y",
//       key: "from_2y_to_3y",
//       name: "From 2 Years To 3 Years",
//       legend: "2Y — 3Y",
//     },
//     {
//       id: "from-3y-to-5y",
//       key: "from_3y_to_5y",
//       name: "From 3 Years To 5 Years",
//       legend: "3Y — 5Y",
//     },
//     {
//       id: "from-5y-to-7y",
//       key: "from_5y_to_7y",
//       name: "From 5 Years To 7 Years",
//       legend: "5Y — 7Y",
//     },
//     {
//       id: "from-7y-to-10y",
//       key: "from_7y_to_10y",
//       name: "From 7 Years To 10 Years",
//       legend: "7Y — 10Y",
//     },
//     {
//       id: "from-10y-to-15y",
//       key: "from_10y_to_15y",
//       name: "From 10 Years To 15 Years",
//       legend: "10Y — 15Y",
//     },
//     {
//       id: "from-15y",
//       key: "from_15y",
//       name: "From 15 Years To End",
//       legend: "15Y — End",
//     },
//   ]);

//   const fromX = /** @type {const} */ ([
//     {
//       id: "from-1y",
//       key: "from_1y",
//       name: "From 1 Year",
//       legend: "1Y+",
//     },
//     {
//       id: "from-2y",
//       key: "from_2y",
//       name: "From 2 Years",
//       legend: "2Y+",
//     },
//     {
//       id: "from-4y",
//       key: "from_4y",
//       name: "From 4 Years",
//       legend: "4Y+",
//     },
//     {
//       id: "from-10y",
//       key: "from_10y",
//       name: "From 10 Years",
//       legend: "10Y+",
//     },
//     {
//       id: "from-15y",
//       key: "from_15y",
//       name: "From 15 Years",
//       legend: "15Y+",
//     },
//   ]);

//   const epochs = /** @type {const} */ ([
//     { id: "epoch-1", key: "epoch_1", name: "Epoch 1" },
//     { id: "epoch-2", key: "epoch_2", name: "Epoch 2" },
//     { id: "epoch-3", key: "epoch_3", name: "Epoch 3" },
//     { id: "epoch-4", key: "epoch_4", name: "Epoch 4" },
//     { id: "epoch-5", key: "epoch_5", name: "Epoch 5" },
//   ]);

//   const age = /** @type {const} */ ([
//     {
//       key: "",
//       id: "",
//       name: "",
//     },
//     ...xTermHolders,
//     ...upTo,
//     ...fromXToY,
//     ...fromX,
//     ...epochs,
//   ]);

//   const size = /** @type {const} */ ([
//     {
//       key: "plankton",
//       name: "Plankton",
//       size: "1 sat to 0.1 BTC",
//     },
//     {
//       key: "shrimp",
//       name: "Shrimp",
//       size: "0.1 sat to 1 BTC",
//     },
//     { key: "crab", name: "Crab", size: "1 BTC to 10 BTC" },
//     { key: "fish", name: "Fish", size: "10 BTC to 100 BTC" },
//     { key: "shark", name: "Shark", size: "100 BTC to 1000 BTC" },
//     { key: "whale", name: "Whale", size: "1000 BTC to 10 000 BTC" },
//     {
//       key: "humpback",
//       name: "Humpback",
//       size: "10 000 BTC to 100 000 BTC",
//     },
//     {
//       key: "megalodon",
//       name: "Megalodon",
//       size: "More than 100 000 BTC",
//     },
//   ]);

//   const type = /** @type {const} */ ([
//     { key: "p2pk", name: "P2PK" },
//     { key: "p2pkh", name: "P2PKH" },
//     { key: "p2sh", name: "P2SH" },
//     { key: "p2wpkh", name: "P2WPKH" },
//     { key: "p2wsh", name: "P2WSH" },
//     { key: "p2tr", name: "P2TR" },
//   ]);

//   const address = /** @type {const} */ ([...size, ...type]);

//   const liquidities = /** @type {const} */ ([
//     {
//       key: "illiquid",
//       id: "illiquid",
//       name: "Illiquid",
//     },
//     { key: "liquid", id: "liquid", name: "Liquid" },
//     {
//       key: "highly_liquid",
//       id: "highly-liquid",
//       name: "Highly Liquid",
//     },
//   ]);

//   const totalReturns = /** @type {const} */ ([
//     { name: "1 Day", key: "1d" },
//     { name: "1 Month", key: "1m" },
//     { name: "6 Months", key: "6m" },
//     { name: "1 Year", key: "1y" },
//     { name: "2 Years", key: "2y" },
//     { name: "3 Years", key: "3y" },
//     { name: "4 Years", key: "4y" },
//     { name: "6 Years", key: "6y" },
//     { name: "8 Years", key: "8y" },
//     { name: "10 Years", key: "10y" },
//   ]);

//   const compoundReturns = /** @type {const} */ ([
//     { name: "4 Years", key: "4y" },
//   ]);

//   const percentiles = /** @type {const} */ ([
//     {
//       key: "median_price_paid",
//       id: "median-price-paid",
//       name: "Median",
//       title: "Median Paid",
//       value: 50,
//     },
//     {
//       key: "95p_price_paid",
//       id: "95p-price-paid",
//       name: `95%`,
//       title: `95th Percentile Paid`,
//       value: 95,
//     },
//     {
//       key: "90p_price_paid",
//       id: "90p-price-paid",
//       name: `90%`,
//       title: `90th Percentile Paid`,
//       value: 90,
//     },
//     {
//       key: "85p_price_paid",
//       id: "85p-price-paid",
//       name: `85%`,
//       title: `85th Percentile Paid`,
//       value: 85,
//     },
//     {
//       key: "80p_price_paid",
//       id: "80p-price-paid",
//       name: `80%`,
//       title: `80th Percentile Paid`,
//       value: 80,
//     },
//     {
//       key: "75p_price_paid",
//       id: "75p-price-paid",
//       name: `75%`,
//       title: `75th Percentile Paid`,
//       value: 75,
//     },
//     {
//       key: "70p_price_paid",
//       id: "70p-price-paid",
//       name: `70%`,
//       title: `70th Percentile Paid`,
//       value: 70,
//     },
//     {
//       key: "65p_price_paid",
//       id: "65p-price-paid",
//       name: `65%`,
//       title: `65th Percentile Paid`,
//       value: 65,
//     },
//     {
//       key: "60p_price_paid",
//       id: "60p-price-paid",
//       name: `60%`,
//       title: `60th Percentile Paid`,
//       value: 60,
//     },
//     {
//       key: "55p_price_paid",
//       id: "55p-price-paid",
//       name: `55%`,
//       title: `55th Percentile Paid`,
//       value: 55,
//     },
//     {
//       key: "45p_price_paid",
//       id: "45p-price-paid",
//       name: `45%`,
//       title: `45th Percentile Paid`,
//       value: 45,
//     },
//     {
//       key: "40p_price_paid",
//       id: "40p-price-paid",
//       name: `40%`,
//       title: `40th Percentile Paid`,
//       value: 40,
//     },
//     {
//       key: "35p_price_paid",
//       id: "35p-price-paid",
//       name: `35%`,
//       title: `35th Percentile Paid`,
//       value: 35,
//     },
//     {
//       key: "30p_price_paid",
//       id: "30p-price-paid",
//       name: `30%`,
//       title: `30th Percentile Paid`,
//       value: 30,
//     },
//     {
//       key: "25p_price_paid",
//       id: "25p-price-paid",
//       name: `25%`,
//       title: `25th Percentile Paid`,
//       value: 25,
//     },
//     {
//       key: "20p_price_paid",
//       id: "20p-price-paid",
//       name: `20%`,
//       title: `20th Percentile Paid`,
//       value: 20,
//     },
//     {
//       key: "15p_price_paid",
//       id: "15p-price-paid",
//       name: `15%`,
//       title: `15th Percentile Paid`,
//       value: 15,
//     },
//     {
//       key: "10p_price_paid",
//       id: "10p-price-paid",
//       name: `10%`,
//       title: `10th Percentile Paid`,
//       value: 10,
//     },
//     {
//       key: "05p_price_paid",
//       id: "05p-price-paid",
//       name: `5%`,
//       title: `5th Percentile Paid`,
//       value: 5,
//     },
//   ]);

//   return {
//     xTermHolders,
//     upTo,
//     fromX,
//     fromXToY,
//     epochs,
//     age,
//     type,
//     size,
//     address,
//     liquidities,
//     averages,
//     totalReturns,
//     compoundReturns,
//     percentiles,
//   };
// }
// /**
//  * @typedef {ReturnType<typeof initGroups>} Groups
//  *
//  * @typedef {Groups["age"][number]["id"]} AgeCohortId
//  *
//  * @typedef {Exclude<AgeCohortId, "">} AgeCohortIdSub
//  *
//  * @typedef {Groups["address"][number]["key"]} AddressCohortId
//  *
//  * @typedef {Groups["liquidities"][number]["id"]} LiquidityId
//  *
//  * @typedef {AgeCohortId | AddressCohortId} AnyCohortId
//  *
//  * @typedef {AddressCohortId | LiquidityId} AnyAddressCohortId
//  *
//  * @typedef {AnyCohortId | LiquidityId} AnyPossibleCohortId
//  *
//  * @typedef {'' | `${AgeCohortIdSub | AddressCohortId | LiquidityId}-`} AnyDatasetPrefix
//  *
//  * @typedef {Groups["averages"][number]["key"]} AverageName
//  *
//  * @typedef {Groups["totalReturns"][number]["key"]} TotalReturnKey
//  *
//  * @typedef {Groups["compoundReturns"][number]["key"]} CompoundReturnKey
//  *
//  * @typedef {Groups["percentiles"][number]["id"]} PercentileId
//  */
