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
 * @typedef {{ key: ChartableVecId, unit?: Unit | Unit[] }} FetchedAnySeriesOptions
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
   * @typedef {"cumulative-"} CumulativePrefix
   * @typedef {StartsWith<CumulativePrefix>} CumulativeVecId
   * @typedef {WithoutPrefix<CumulativeVecId, CumulativePrefix>} CumulativeVecIdBase
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
    { name: "1 Week", key: "1w", days: 7, color: colors.orange },
    { name: "8 Days", key: "8d", days: 8, color: colors.amber },
    { name: "13 Days", key: "13d", days: 13, color: colors.yellow },
    { name: "21 Days", key: "21d", days: 21, color: colors.lime },
    { name: "1 Month", key: "1m", days: 30, color: colors.green },
    { name: "34 Days", key: "34d", days: 34, color: colors.emerald },
    { name: "55 Days", key: "55d", days: 55, color: colors.teal },
    { name: "89 Days", key: "89d", days: 89, color: colors.cyan },
    { name: "144 Days", key: "144d", days: 144, color: colors.sky },
    { name: "1 Year", key: "1y", days: 365, color: colors.blue },
    { name: "2 Years", key: "2y", days: 2 * 365, color: colors.indigo },
    { name: "200 Weeks", key: "200w", days: 200 * 7, color: colors.violet },
    { name: "4 Years", key: "4y", days: 4 * 365, color: colors.purple },
  ]);

  const dcaClasses = /** @type {const} */ ([
    { year: 2015, color: colors.pink, defaultActive: false },
    { year: 2016, color: colors.red, defaultActive: false },
    { year: 2017, color: colors.orange, defaultActive: true },
    { year: 2018, color: colors.yellow, defaultActive: true },
    { year: 2019, color: colors.green, defaultActive: true },
    { year: 2020, color: colors.teal, defaultActive: true },
    { year: 2021, color: colors.sky, defaultActive: true },
    { year: 2022, color: colors.blue, defaultActive: true },
    { year: 2023, color: colors.purple, defaultActive: true },
    { year: 2024, color: colors.fuchsia, defaultActive: true },
    { year: 2025, color: colors.pink, defaultActive: true },
  ]);

  const terms = /** @type {const} */ ([
    {
      key: "sth",
      name: "short",
      title: "Short Term Holders",
      color: colors.yellow,
    },
    {
      key: "lth",
      name: "long",
      title: "Long Term Holders",
      color: colors.fuchsia,
    },
  ]);

  const upTo = /** @type {const} */ ([
    {
      key: "up-to-1d",
      name: "1d",
      title: "Up to 1 Day",
      color: colors.pink,
    },
    {
      key: "up-to-1w",
      name: "1w",
      title: "Up to 1 Week",
      color: colors.red,
    },
    {
      key: "up-to-1m",
      name: "1m",
      title: "Up to 1 Month",
      color: colors.orange,
    },
    {
      key: "up-to-2m",
      name: "2m",
      title: "Up to 2 Months",
      color: colors.amber,
    },
    {
      key: "up-to-3m",
      name: "3m",
      title: "Up to 3 Months",
      color: colors.yellow,
    },
    {
      key: "up-to-4m",
      name: "4m",
      title: "Up to 4 Months",
      color: colors.lime,
    },
    {
      key: "up-to-5m",
      name: "5m",
      title: "Up to 5 Months",
      color: colors.green,
    },
    {
      key: "up-to-6m",
      name: "6m",
      title: "Up to 6 Months",
      color: colors.teal,
    },
    {
      key: "up-to-1y",
      name: "1y",
      title: "Up to 1 Year",
      color: colors.sky,
    },
    {
      key: "up-to-2y",
      name: "2y",
      title: "Up to 2 Years",
      color: colors.indigo,
    },
    {
      key: "up-to-3y",
      name: "3y",
      title: "Up to 3 Years",
      color: colors.violet,
    },
    {
      key: "up-to-4y",
      name: "4y",
      title: "Up to 4 Years",
      color: colors.purple,
    },
    {
      key: "up-to-5y",
      name: "5y",
      title: "Up to 5 Years",
      color: colors.fuchsia,
    },
    {
      key: "up-to-6y",
      name: "6y",
      title: "Up to 6 Years",
      color: colors.pink,
    },
    {
      key: "up-to-7y",
      name: "7y",
      title: "Up to 7 Years",
      color: colors.red,
    },
    {
      key: "up-to-8y",
      name: "8y",
      title: "Up to 8 Years",
      color: colors.orange,
    },
    {
      key: "up-to-10y",
      name: "10y",
      title: "Up to 10 Years",
      color: colors.amber,
    },
    {
      key: "up-to-15y",
      name: "15y",
      title: "Up to 15 Years",
      color: colors.yellow,
    },
  ]);

  const from = /** @type {const} */ ([
    {
      key: "from-1d",
      name: "1d",
      title: "From 1 Day",
      color: colors.red,
    },
    {
      key: "from-1w",
      name: "1w",
      title: "From 1 Week",
      color: colors.orange,
    },
    {
      key: "from-1m",
      name: "1m",
      title: "From 1 Month",
      color: colors.yellow,
    },
    {
      key: "from-2m",
      name: "2m",
      title: "From 2 Months",
      color: colors.lime,
    },
    {
      key: "from-3m",
      name: "3m",
      title: "From 3 Months",
      color: colors.green,
    },
    {
      key: "from-4m",
      name: "4m",
      title: "From 4 Months",
      color: colors.teal,
    },
    {
      key: "from-5m",
      name: "5m",
      title: "From 5 Months",
      color: colors.cyan,
    },
    {
      key: "from-6m",
      name: "6m",
      title: "From 6 Months",
      color: colors.blue,
    },
    {
      key: "from-1y",
      name: "1y",
      title: "From 1 Year",
      color: colors.indigo,
    },
    {
      key: "from-2y",
      name: "2y",
      title: "From 2 Years",
      color: colors.violet,
    },
    {
      key: "from-3y",
      name: "3y",
      title: "From 3 Years",
      color: colors.purple,
    },
    {
      key: "from-4y",
      name: "4y",
      title: "From 4 Years",
      color: colors.fuchsia,
    },
    {
      key: "from-5y",
      name: "5y",
      title: "From 5 Years",
      color: colors.pink,
    },
    {
      key: "from-6y",
      name: "6y",
      title: "From 6 Years",
      color: colors.rose,
    },
    {
      key: "from-7y",
      name: "7y",
      title: "From 7 Years",
      color: colors.red,
    },
    {
      key: "from-8y",
      name: "8y",
      title: "From 8 Years",
      color: colors.orange,
    },
    {
      key: "from-10y",
      name: "10y",
      title: "From 10 Years",
      color: colors.yellow,
    },
    {
      key: "from-15y",
      name: "15y",
      title: "From 15 Years",
      color: colors.lime,
    },
  ]);

  const range = /** @type {const} */ ([
    {
      key: "start-to-1d",
      name: "24h",
      title: "Last 24 hours",
      color: colors.pink,
    },
    {
      key: "from-1d-to-1w",
      name: "1d..1w",
      title: "From 1 Day ago to 1 Week ago",
      color: colors.red,
    },
    {
      key: "from-1w-to-1m",
      name: "1w..1m",
      title: "From 1 Week ago to 1 Month ago",
      color: colors.orange,
    },
    {
      key: "from-1m-to-2m",
      name: "1m..2m",
      title: "From 1 Month ago to 2 Months ago",
      color: colors.yellow,
    },
    {
      key: "from-2m-to-3m",
      name: "2m..3m",
      title: "From 2 Month ago to 3 Months ago",
      color: colors.yellow,
    },
    {
      key: "from-3m-to-4m",
      name: "3m..4m",
      title: "From 3 Month ago to 4 Months ago",
      color: colors.lime,
    },
    {
      key: "from-4m-to-5m",
      name: "4m..5m",
      title: "From 4 Month ago to 5 Months ago",
      color: colors.lime,
    },
    {
      key: "from-5m-to-6m",
      name: "5m..6m",
      title: "From 5 Month ago to 6 Months ago",
      color: colors.lime,
    },
    {
      key: "from-6m-to-1y",
      name: "6m..1y",
      title: "From 6 Months ago to 1 Year ago",
      color: colors.green,
    },
    {
      key: "from-1y-to-2y",
      name: "1y..2y",
      title: "From 1 Year ago to 2 Years ago",
      color: colors.cyan,
    },
    {
      key: "from-2y-to-3y",
      name: "2y..3y",
      title: "From 2 Years ago to 3 Years ago",
      color: colors.blue,
    },
    {
      key: "from-3y-to-4y",
      name: "3y..4y",
      title: "From 3 Years ago to 4 Years ago",
      color: colors.indigo,
    },
    {
      key: "from-4y-to-5y",
      name: "4y..5y",
      title: "From 4 Years ago to 5 Years ago",
      color: colors.violet,
    },
    {
      key: "from-5y-to-6y",
      name: "5y..6y",
      title: "From 5 Years ago to 6 Years ago",
      color: colors.purple,
    },
    {
      key: "from-6y-to-7y",
      name: "6y..7y",
      title: "From 6 Years ago to 7 Years ago",
      color: colors.purple,
    },
    {
      key: "from-7y-to-8y",
      name: "7y..8y",
      title: "From 7 Years ago to 8 Years ago",
      color: colors.fuchsia,
    },
    {
      key: "from-8y-to-10y",
      name: "8y..10y",
      title: "From 8 Years ago to 10 Years ago",
      color: colors.fuchsia,
    },
    {
      key: "from-10y-to-15y",
      name: "10y..15y",
      title: "From 10 Years ago to 15 Years ago",
      color: colors.pink,
    },
    {
      key: "from-15y-to-end",
      name: "15y+",
      title: "From 15 Years ago to genesis (2009-01-03)",
      color: colors.red,
    },
  ]);

  const epoch = /** @type {const} */ ([
    {
      key: "epoch-0",
      name: "0",
      title: "Epoch 0",
      color: colors.red,
    },
    {
      key: "epoch-1",
      name: "1",
      title: "Epoch 1",
      color: colors.yellow,
    },
    {
      key: "epoch-2",
      name: "2",
      title: "Epoch 2",
      color: colors.orange,
    },
    {
      key: "epoch-3",
      name: "3",
      title: "Epoch 3",
      color: colors.lime,
    },
    {
      key: "epoch-4",
      name: "4",
      title: "Epoch 4",
      color: colors.green,
    },
  ]);

  const fromSize = /** @type {const} */ ([
    {
      key: "from-1-000sats",
      name: "1K sats",
      title: "From 1K sats",
      color: colors.cyan,
    },
    {
      key: "from-1btc",
      name: "1btc",
      title: "From 1 BTC",
      color: colors.violet,
    },
    {
      key: "from-10btc",
      name: "10btc",
      title: "From 10 BTC",
      color: colors.purple,
    },
    {
      key: "from-100btc",
      name: "100btc",
      title: "From 100 BTC",
      color: colors.pink,
    },
  ]);

  const upToSize = /** @type {const} */ ([
    {
      key: "up-to-1-000sats",
      name: "1K sats",
      title: "Up to 1K sats",
      color: colors.yellow,
    },
    {
      key: "up-to-10-000sats",
      name: "10K sats",
      title: "Up to 10K sats",
      color: colors.green,
    },
    {
      key: "up-to-1btc",
      name: "1btc",
      title: "Up to 1 btc",
      color: colors.cyan,
    },
    {
      key: "up-to-10btc",
      name: "10btc",
      title: "Up to 10 btc",
      color: colors.blue,
    },
    {
      key: "up-to-100btc",
      name: "100btc",
      title: "Up to 100 btc",
      color: colors.violet,
    },
  ]);

  const sizeRanges = /** @type {const} */ ([
    {
      key: "0sats",
      name: "0 sats",
      title: "0 sats",
      color: colors.red,
    },
    {
      key: "from-1sat-to-10sats",
      name: "1 sat..10 sats",
      title: "From 1 sat to 10 sats",
      color: colors.orange,
    },
    {
      key: "from-10sats-to-100sats",
      name: "10 sats..100 sats",
      title: "From 10 sats to 100 sats",
      color: colors.yellow,
    },
    {
      key: "from-100sats-to-1-000sats",
      name: "100 sats..1K sats",
      title: "From 100 sats to 1K sats",
      color: colors.lime,
    },
    {
      key: "from-1-000sats-to-10-000sats",
      name: "1K sats..10K sats",
      title: "From 1K sats to 10K sats",
      color: colors.green,
    },
    {
      key: "from-10-000sats-to-100-000sats",
      name: "10K sats..100K sats",
      title: "From 10K sats to 100K sats",
      color: colors.cyan,
    },
    {
      key: "from-100-000sats-to-1-000-000sats",
      name: "100K sats .. 1M sats",
      title: "From 100K sats to 1M sats",
      color: colors.blue,
    },
    {
      key: "from-1-000-000sats-to-10-000-000sats",
      name: "1M sats..10M sats",
      title: "From 1M sats to 10M sats",
      color: colors.indigo,
    },
    {
      key: "from-10-000-000sats-to-1btc",
      name: "10M sats..1 btc",
      title: "From 10M sats to 1 BTC",
      color: colors.purple,
    },
    {
      key: "from-1btc-to-10btc",
      name: "1 btc..10 btc",
      title: "From 1 BTC to 10 BTC",
      color: colors.violet,
    },
    {
      key: "from-10btc-to-100btc",
      name: "10 btc..100 btc",
      title: "From 10 BTC to 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "from-100btc-to-1-000btc",
      name: "100 btc..1K btc",
      title: "From 100 BTC to 1K BTC",
      color: colors.pink,
    },
    {
      key: "from-1-000btc-to-10-000btc",
      name: "1K btc..10K btc",
      title: "From 1K BTC to 10K BTC",
      color: colors.red,
    },
    {
      key: "from-10-000btc-to-100-000btc",
      name: "10K btc..100K btc",
      title: "From 10K BTC to 100K BTC",
      color: colors.orange,
    },
    {
      key: "from-100-000btc",
      name: "100K btc+",
      title: "From 100K BTC",
      color: colors.yellow,
    },
  ]);

  const type = /** @type {const} */ ([
    {
      key: "p2pk65",
      name: "p2pk65",
      title: "Pay To Long Public Key",
      color: colors.red,
    },
    {
      key: "p2pk33",
      name: "p2pk33",
      title: "Pay To Short Public Key",
      color: colors.orange,
    },
    {
      key: "p2pkh",
      name: "p2pkh",
      title: "Pay To Public Key Hash",
      color: colors.yellow,
    },
    {
      key: "p2ms",
      name: "p2ms",
      title: "Pay To Bare Multisig",
      color: colors.lime,
    },
    {
      key: "p2sh",
      name: "p2sh",
      title: "Pay To Script Hash",
      color: colors.green,
    },
    {
      key: "p2wpkh",
      name: "p2wpkh",
      title: "Pay To Witness Public Key Hash",
      color: colors.teal,
    },
    {
      key: "p2wsh",
      name: "p2wsh",
      title: "Pay To Witness Script Hash",
      color: colors.blue,
    },
    {
      key: "p2tr",
      name: "p2tr",
      title: "Pay To Taproot",
      color: colors.indigo,
    },
    {
      key: "p2a",
      name: "p2a",
      title: "Pay To Anchor",
      color: colors.purple,
    },
    {
      key: "unknown",
      name: "unknown",
      title: "Pay To Unknown",
      color: colors.violet,
    },
    {
      key: "empty",
      name: "empty",
      title: "Pay To Empty",
      color: colors.fuchsia,
    },
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
   * @param {VecIdSumBase & CumulativeVecIdBase} args.concat
   * @param {string} [args.name]
   */
  function createSumCumulativeSeries({ concat, name }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        key: `${concat}-sum`,
        title: name ? `${name} Sum` : "Sum",
        color: colors.orange,
      },
      {
        key: `cumulative-${concat}`,
        title: name ? `Cumulative ${name}` : "Cumulative",
        color: colors.red,
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
   * @param {VecIdAverageBase & VecIdSumBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} key
   */
  function createAverageSumCumulativeMinMaxPercentilesSeries(key) {
    return [
      createAverageSeries({ concat: key }),
      ...createSumCumulativeSeries({ concat: key }),
      ...createMinMaxPercentilesSeries({ concat: key }),
    ];
  }

  /**
   * @param {Object} args
   * @param {ChartableVecId & VecIdAverageBase & VecIdSumBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} args.key
   * @param {string} args.name
   */
  function createBaseAverageSumCumulativeMinMaxPercentilesSeries({
    key,
    name,
  }) {
    return [
      createBaseSeries({
        key,
        name,
      }),
      ...createAverageSumCumulativeMinMaxPercentilesSeries(key),
    ];
  }

  /**
   * @param {Object} args
   * @param {ChartableVecId & VecIdSumBase & CumulativeVecIdBase} args.key
   * @param {string} args.name
   */
  function createBaseSumCumulativeSeries({ key, name }) {
    return [
      createBaseSeries({
        key,
        name,
      }),
      ...createSumCumulativeSeries({
        concat: key,
      }),
    ];
  }

  /**
   * @typedef {"-ratio-zscore"} RatioZScoreCapSuffix
   * @typedef {EndsWith<RatioZScoreCapSuffix>} VecIdRatioZScoreCap
   * @typedef {WithoutSuffix<VecIdRatioZScoreCap, RatioZScoreCapSuffix>} VecIdRatioZScoreCapBase
   */

  /**
   *
   * @param {Object} args
   * @param {string} args.name
   * @param {string} args.legend
   * @param {string} args.title
   * @param {VecIdRatioZScoreCapBase} args.key
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
   * @typedef {"-supply-in-profit"} SupplyInProfitSuffix
   * @typedef {EndsWith<SupplyInProfitSuffix>} VecIdSupplyInProfit
   * @typedef {WithoutSuffix<VecIdSupplyInProfit, SupplyInProfitSuffix>} CohortId
   */

  /**
   * @typedef {Object} UTXOGroupObject
   * @property {string} args.name
   * @property {string} args.title
   * @property {Color} args.color
   * @property {"" | CohortId} args.key
   */

  /**
   * @typedef {Object} UTXOGroupsObject
   * @property {string} args.name
   * @property {string} args.title
   * @property {readonly UTXOGroupObject[]} args.list
   */

  /**
   * @param {UTXOGroupObject | UTXOGroupsObject} args
   */
  function createUTXOGroupFolder(args) {
    /**
     * @template {"" | CohortId} T
     * @param {T} _key
     */
    const fixKey = (_key) =>
      _key !== ""
        ? /** @type {Exclude<"" | `${T}-`, "-">} */ (`${_key}-`)
        : /** @type {const} */ ("");
    0;

    const list = "list" in args ? args.list : [args];
    const useGroupName = "list" in args;

    return /** @satisfies {PartialOptionsGroup} */ ({
      name: args.name || "all",
      tree: [
        {
          name: "supply",
          title: `${args.title} Supply`,
          bottom: list.flatMap(({ color, name, key: _key }) => {
            const key = fixKey(_key);
            return /** @type {const} */ ([
              createBaseSeries({
                key: `${key}supply`,
                name: useGroupName ? name : "Supply",
                color: "list" in args ? color : colors.default,
              }),
              createBaseSeries({
                key: `${key}supply-in-btc`,
                name: useGroupName ? name : "Supply",
                color: "list" in args ? color : colors.default,
              }),
              createBaseSeries({
                key: `${key}supply-in-usd`,
                name: useGroupName ? name : "Supply",
                color: "list" in args ? color : colors.default,
              }),
              ...(key
                ? [
                    createBaseSeries({
                      key: `${key}supply-relative-to-circulating-supply`,
                      name: useGroupName ? name : "Supply",
                      color: "list" in args ? color : colors.default,
                    }),
                  ]
                : []),
              ...(!("list" in args)
                ? [
                    createBaseSeries({
                      key: `${key}halved-supply`,
                      name: useGroupName ? name : "Halved",
                      color: "list" in args ? color : colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-profit`,
                      name: useGroupName ? name : "In Profit",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-loss`,
                      name: useGroupName ? name : "In Loss",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      key: `${key}supply-even`,
                      name: useGroupName ? name : "Even",
                      color: colors.yellow,
                    }),
                    createBaseSeries({
                      key: `${key}halved-supply-in-btc`,
                      name: useGroupName ? name : "Halved",
                      color: "list" in args ? color : colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-profit-in-btc`,
                      name: useGroupName ? name : "In Profit",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-loss-in-btc`,
                      name: useGroupName ? name : "In Loss",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      key: `${key}supply-even-in-btc`,
                      name: useGroupName ? name : "Even",
                      color: colors.yellow,
                    }),
                    createBaseSeries({
                      key: `${key}halved-supply-in-usd`,
                      name: useGroupName ? name : "Halved",
                      color: "list" in args ? color : colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-profit-in-usd`,
                      name: useGroupName ? name : "In Profit",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-loss-in-usd`,
                      name: useGroupName ? name : "In Loss",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      key: `${key}supply-even-in-usd`,
                      name: useGroupName ? name : "Even",
                      color: colors.yellow,
                    }),
                    ...(key
                      ? [
                          createBaseSeries({
                            key: `${key}supply-in-profit-relative-to-circulating-supply`,
                            name: useGroupName ? name : "In Profit",
                            color: colors.green,
                          }),
                          createBaseSeries({
                            key: `${key}supply-in-loss-relative-to-circulating-supply`,
                            name: useGroupName ? name : "In Loss",
                            color: colors.red,
                          }),
                          createBaseSeries({
                            key: `${key}supply-even-relative-to-circulating-supply`,
                            name: useGroupName ? name : "Even",
                            color: colors.yellow,
                          }),
                        ]
                      : []),
                    createBaseSeries({
                      key: `${key}supply-in-profit-relative-to-own-supply`,
                      name: useGroupName ? name : "In Profit",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `${key}supply-in-loss-relative-to-own-supply`,
                      name: useGroupName ? name : "In Loss",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      key: `${key}supply-even-relative-to-own-supply`,
                      name: useGroupName ? name : "Even",
                      color: colors.yellow,
                    }),
                  ]
                : []),
            ]);
          }),
        },
        {
          name: "utxo count",
          title: `${args.title} UTXO Count`,
          bottom: list.flatMap(({ color, name, key: _key }) => {
            const key = fixKey(_key);
            return /** @type {const} */ ([
              createBaseSeries({
                key: `${key}utxo-count`,
                name: useGroupName ? name : "Count",
                color,
              }),
            ]);
          }),
        },
        {
          name: "Realized",
          tree: [
            {
              name: "cap",
              title: `${args.title} Realized Capitalization`,
              bottom: list.flatMap(({ color, name, key: _key }) => {
                const key = fixKey(_key);
                return /** @type {const} */ ([
                  createBaseSeries({
                    key: `${key}realized-cap`,
                    name: useGroupName ? name : "Cap",
                    color,
                  }),
                  ...(!("list" in args)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${key}realized-cap-30d-change`,
                          title: "30d change",
                          defaultActive: false,
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                      ]
                    : []),
                ]);
              }),
            },
            "list" in args
              ? {
                  name: "Price",
                  title: `${args.title} Realized Prices`,
                  top: args.list.map(({ color, name, key }) =>
                    createBaseSeries({
                      key: `${fixKey(key)}realized-price`,
                      name,
                      color,
                    }),
                  ),
                }
              : createPriceWithRatio({
                  title: `${args.title} Realized Price`,
                  key: `${fixKey(args.key)}realized-price`,
                  name: "price",
                  legend: "realized",
                  color: args.color,
                }),
            ...(!("list" in args)
              ? [
                  {
                    name: "pnl",
                    title: `${args.title} Realized Profit And Loss`,
                    bottom: [
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized-profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized-loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}negative-realized-loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                    ],
                  },
                ]
              : [
                  {
                    name: "profit",
                    title: `${args.title} Realized Profit`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}realized-profit`,
                          name: useGroupName ? name : "Profit",
                          color: useGroupName ? color : colors.green,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "loss",
                    title: `${args.title} Realized Loss`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}realized-loss`,
                          name: useGroupName ? name : "Loss",
                          color: useGroupName ? color : colors.red,
                        }),
                      ]);
                    }),
                  },
                ]),
            {
              name: "Net pnl",
              title: `${args.title} Net Realized Profit And Loss`,
              bottom: list.flatMap(({ color, name, key }) => [
                /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                  type: "Baseline",
                  key: `${fixKey(key)}net-realized-profit-and-loss`,
                  title: useGroupName ? name : "Net",
                  color: useGroupName ? color : undefined,
                  options: {
                    createPriceLine: {
                      value: 0,
                    },
                  },
                }),
                /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                  type: "Baseline",
                  key: `${fixKey(key)}net-realized-profit-and-loss-relative-to-realized-cap`,
                  title: useGroupName ? name : "Net",
                  color: useGroupName ? color : undefined,
                  options: {
                    createPriceLine: {
                      value: 0,
                    },
                  },
                }),
              ]),
            },
            ...(!("list" in args)
              ? [
                  {
                    name: "sopr",
                    title: `${args.title} Spent Output Profit Ratio`,
                    bottom: list.flatMap(({ color, name, key }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}spent-output-profit-ratio`,
                        title: useGroupName ? name : "sopr",
                        color: useGroupName ? color : undefined,
                        options: {
                          createPriceLine: {
                            value: 1,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}adjusted-spent-output-profit-ratio`,
                        title: useGroupName ? name : "asopr",
                        color: useGroupName ? color : undefined,
                        options: {
                          createPriceLine: {
                            value: 1,
                          },
                        },
                      }),
                    ]),
                  },
                ]
              : [
                  {
                    name: "sopr",
                    tree: [
                      {
                        name: "Normal",
                        title: `${args.title} Spent Output Profit Ratio`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${fixKey(key)}spent-output-profit-ratio`,
                            title: useGroupName ? name : "sopr",
                            color: useGroupName ? color : undefined,
                            options: {
                              createPriceLine: {
                                value: 1,
                              },
                            },
                          }),
                        ]),
                      },
                      {
                        name: "Adjusted",
                        title: `${args.title} Adjusted Spent Output Profit Ratio`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${fixKey(key)}adjusted-spent-output-profit-ratio`,
                            title: useGroupName ? name : "asopr",
                            color: useGroupName ? color : undefined,
                            options: {
                              createPriceLine: {
                                value: 1,
                              },
                            },
                          }),
                        ]),
                      },
                    ],
                  },
                ]),
            {
              name: "Sell Side Risk Ratio",
              title: `${args.title} Sell Side Risk Ratio`,
              bottom: list.flatMap(({ color, name, key }) =>
                createBaseSeries({
                  key: `${fixKey(key)}sell-side-risk-ratio`,
                  name: useGroupName ? name : "Risk",
                  color: color,
                }),
              ),
            },
          ],
        },
        {
          name: "Unrealized",
          tree: [
            ...(!("list" in args)
              ? [
                  {
                    name: "pnl",
                    title: `${args.title} Unrealized Profit And Loss`,
                    bottom: [
                      // createBaseSeries({
                      //   key: `0`,
                      //   name: "Base",
                      //   color: colors.gray,
                      //   options: {
                      //     lineStyle: 4,
                      //   },
                      // }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}unrealized-profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}unrealized-loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}negative-unrealized-loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                    ],
                  },
                ]
              : [
                  {
                    name: "profit",
                    title: `${args.title} Unrealized Profit`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}unrealized-profit`,
                          name: useGroupName ? name : "Profit",
                          color: useGroupName ? color : colors.green,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "loss",
                    title: `${args.title} Unrealized Loss`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}unrealized-loss`,
                          name: useGroupName ? name : "Loss",
                          color: useGroupName ? color : colors.red,
                        }),
                      ]);
                    }),
                  },
                ]),
            {
              name: "Net pnl",
              title: `${args.title} Net Unrealized Profit And Loss`,
              bottom: list.flatMap(({ color, name, key }) => [
                /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                  type: "Baseline",
                  key: `${fixKey(key)}net-unrealized-profit-and-loss`,
                  title: useGroupName ? name : "Net",
                  color: useGroupName ? color : undefined,
                  options: {
                    createPriceLine: {
                      value: 0,
                    },
                  },
                }),
                /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                  type: "Baseline",
                  key: `${fixKey(key)}net-unrealized-profit-and-loss-relative-to-market-cap`,
                  title: useGroupName ? name : "Net",
                  color: useGroupName ? color : undefined,
                  options: {
                    createPriceLine: {
                      value: 0,
                    },
                  },
                }),
              ]),
            },
          ],
        },
        ...("list" in args
          ? [
              {
                name: "Price paid",
                tree: [
                  {
                    name: "Average",
                    title: `${args.title} Average Price Paid`,
                    top: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}realized-price`,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Min",
                    title: `${args.title} Min Price Paid`,
                    top: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}min-price-paid`,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Max",
                    title: `${args.title} Max Price Paid`,
                    top: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}max-price-paid`,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                ],
              },
            ]
          : [
              {
                name: "Price paid",
                title: `${args.title} Prices Paid`,
                top: [
                  createBaseSeries({
                    key: `${fixKey(args.key)}realized-price`,
                    name: "Average",
                    color: args.color,
                  }),
                  createBaseSeries({
                    key: `${fixKey(args.key)}min-price-paid`,
                    name: "Min",
                    color: colors.green,
                    // defaultActive: false,
                  }),
                  createBaseSeries({
                    key: `${fixKey(args.key)}max-price-paid`,
                    name: "Max",
                    color: colors.red,
                    // defaultActive: false,
                  }),
                ],
              },
            ]),
        {
          name: "Coins Destroyed",
          title: `${args.title} Coins Destroyed`,
          bottom: list.flatMap(({ color, name, key: _key }) => {
            const key = fixKey(_key);
            return /** @type {const} */ ([
              createBaseSeries({
                key: `${key}coinblocks-destroyed`,
                name: useGroupName ? name : "destroyed",
                color,
              }),
              createBaseSeries({
                key: `${key}coindays-destroyed`,
                name: useGroupName ? name : "destroyed",
                color,
              }),
            ]);
          }),
        },
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
              title: "All Time High",
              // tree: [
              //   {
              //     name: "Value",
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
                createBaseSeries({
                  key: "days-since-ath",
                  name: "Days since",
                }),
                createBaseSeries({
                  key: "max-days-between-aths",
                  name: "Max",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "max-years-between-aths",
                  name: "Max",
                  color: colors.red,
                }),
              ],
            },
            {
              name: "Average",
              tree: [
                {
                  name: "Compare",
                  title: "Moving Averages",
                  top: averages.map(({ days, key, name, color }) =>
                    createBaseSeries({
                      key: `${key}-sma`,
                      name: key,
                      color,
                    }),
                  ),
                },
                ...averages.map(({ key, name, color }) =>
                  createPriceWithRatio({
                    key: `${key}-sma`,
                    name,
                    title: `${name} Market Price Moving Average`,
                    legend: "average",
                    color,
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
                  top: dcaClasses.map(({ year, color, defaultActive }) =>
                    createBaseSeries({
                      key: `dca-class-${year}-avg-price`,
                      name: `${year}`,
                      color,
                      defaultActive,
                    }),
                  ),
                },
                ...dcaClasses.map(
                  ({ year, color }) =>
                    /** @satisfies {PartialChartOption} */ ({
                      name: `${year}`,
                      title: `DCA Since ${year}`,
                      top: [
                        createBaseSeries({
                          key: `dca-class-${year}-avg-price`,
                          name: `avg. price`,
                          color,
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
                  key: "cumulative-subsidy-in-btc",
                  name: "Mined",
                }),
              ],
            },
            {
              name: "Coinbase",
              title: "Coinbase",
              bottom: [
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "coinbase",
                  name: "Coinbase",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "coinbase-in-btc",
                  name: "Coinbase",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "coinbase-in-usd",
                  name: "Coinbase",
                }),
              ],
            },
            {
              name: "Subsidy",
              title: "Subsidy",
              bottom: [
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "subsidy",
                  name: "Subsidy",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "subsidy-in-btc",
                  name: "Subsidy",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "subsidy-in-usd",
                  name: "Subsidy",
                }),
              ],
            },
            {
              name: "Fee",
              title: "Transaction Fee",
              bottom: [
                ...createAverageSumCumulativeMinMaxPercentilesSeries("fee"),
                ...createAverageSumCumulativeMinMaxPercentilesSeries(
                  "fee-in-btc",
                ),
                ...createAverageSumCumulativeMinMaxPercentilesSeries(
                  "fee-in-usd",
                ),
              ],
            },
            {
              name: "Unclaimed Rewards",
              title: "Unclaimed Rewards",
              bottom: [
                ...createBaseSumCumulativeSeries({
                  key: "unclaimed-rewards",
                  name: "unclaimed",
                }),
                ...createBaseSumCumulativeSeries({
                  key: "unclaimed-rewards-in-btc",
                  name: "unclaimed",
                }),
                ...createBaseSumCumulativeSeries({
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
                ...createSumCumulativeSeries({ concat: "block-count" }),
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
                ...createSumCumulativeSeries({ concat: "block-size" }),
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
                ...createSumCumulativeSeries({ concat: "block-weight" }),
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
                ...createSumCumulativeSeries({ concat: "block-vbytes" }),
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
              bottom: createBaseAverageSumCumulativeMinMaxPercentilesSeries({
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
                ...createSumCumulativeSeries({ concat: "tx-v1", name: "v1" }),
                createBaseSeries({
                  key: "tx-v2",
                  name: "v2 Count",
                }),
                ...createSumCumulativeSeries({ concat: "tx-v2", name: "v2" }),
                createBaseSeries({
                  key: "tx-v3",
                  name: "v3 Count",
                }),
                ...createSumCumulativeSeries({ concat: "tx-v3", name: "v3" }),
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
                ...createSumCumulativeSeries({ concat: "input-count" }),
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
            //     ...createSumCumulativeSeries({ concat: "input-value" }),
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
                ...createSumCumulativeSeries({ concat: "output-count" }),
                ...createMinMaxPercentilesSeries({
                  concat: "output-count",
                }),
              ],
            },
            // {
            //   name: "Unspent Count",
            //   title: "Unspent Transaction Output Count",
            //   bottom: [
            //     createBaseSeries({
            //       key: "exact-utxo-count",
            //       name: "cumulative",
            //     }),
            //   ],
            // },
            // {
            //   name: "Value",
            //   title: "Transaction Output Value",
            //   bottom: [
            //     createAverageSeries({ concat: "output-value" }),
            //     ...createSumCumulativeSeries({ concat: "output-value" }),
            //   ],
            // },
            // {
            //   name: "types",
            //   tree: [
            //     {
            //       name: "p2pk",
            //       title: "Pay To Public Key Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2pk33-count",
            //           name: "33B Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk33-count-sum",
            //           name: "33B sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2pk33-count",
            //           name: "33B cumulative",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk65-count",
            //           name: "65B Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk65-count-sum",
            //           name: "65B sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2pk65-count",
            //           name: "65B cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2pkh",
            //       title: "Pay To Public Key Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2pkh-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pkh-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2pkh-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2ms",
            //       title: "Pay To Multisig Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2ms-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2ms-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2ms-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2sh",
            //       title: "Pay To Script Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2sh-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2sh-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2sh-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "op_return",
            //       title: "op_return outputs",
            //       bottom: [
            //         createBaseSeries({ key: "opreturn-count", name: "Count" }),
            //         createBaseSeries({
            //           key: "opreturn-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-opreturn-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2wpkh",
            //       title: "Pay To Witness Public Key Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2wpkh-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2wpkh-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2wpkh-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2wsh",
            //       title: "Pay To Witness Script Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2wsh-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2wsh-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2wsh-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2tr",
            //       title: "Pay To Taproot Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2tr-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2tr-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2tr-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2a",
            //       title: "Pay To Anchor outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2a-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2a-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-p2a-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "empty",
            //       title: "empty outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "emptyoutput-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "emptyoutput-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-emptyoutput-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "unknown",
            //       title: "unknown outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "unknownoutput-count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "unknownoutput-count-sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative-unknownoutput-count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //   ],
            //   // title: "Transaction Output Value",
            //   // bottom: [
            //   //   createAverageSeries({ concat: "output-value" }),
            //   //   ...createSumCumulativeSeries({ concat: "output-value" }),
            //   // ],
            // },
          ],
        },
        {
          name: "UTXOs",
          tree: [
            createUTXOGroupFolder({
              key: "",
              name: "",
              title: "",
              color: colors.orange,
            }),
            {
              name: "term",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Term",
                  list: terms,
                }),
                ...terms.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "Up to date",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Up To",
                  list: upTo,
                }),
                ...upTo.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "From Date",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By From",
                  list: from,
                }),
                ...from.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "Date Range",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Range",
                  list: range,
                }),
                ...range.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "Epoch",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Epoch",
                  list: epoch,
                }),
                ...epoch.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "Up to size",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Up To Size",
                  list: upToSize,
                }),
                ...upToSize.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "From size",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By From Size",
                  list: fromSize,
                }),
                ...fromSize.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "Size range",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Size Range",
                  list: sizeRanges,
                }),
                ...sizeRanges.map(createUTXOGroupFolder),
              ],
            },
            {
              name: "type",
              tree: [
                createUTXOGroupFolder({
                  name: "Compare",
                  title: "Compare By Type",
                  list: type,
                }),
                ...type.map(createUTXOGroupFolder),
              ],
            },
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
            {
              name: "op_return",
              tree: [
                {
                  name: "outputs",
                  title: "op_return outputs",
                  bottom: [
                    createBaseSeries({ key: "opreturn-count", name: "Count" }),
                    createBaseSeries({
                      key: "opreturn-count-sum",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "cumulative-opreturn-count",
                      name: "cumulative",
                      color: colors.red,
                    }),
                  ],
                },
                {
                  name: "supply",
                  title: "OP-return Supply",
                  bottom: [
                    createBaseSeries({
                      key: "opreturn-supply",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "opreturn-supply-in-btc",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "opreturn-supply-in-usd",
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
      name: "Hosting",
      tree: [
        {
          name: "Self",
          url: () => "https://crates.io/crates/brk_cli",
        },
        {
          name: "As a service",
          url: () =>
            "https://github.com/bitcoinresearchkit/brk?tab=readme-ov-file#hosting-as-a-service",
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
      name: "Donate",
      tree: [
        {
          name: "Bitcoin",
          qrcode: true,
          url: () => "bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve",
        },
        {
          name: "Lightning",
          qrcode: true,
          url: () =>
            "lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4",
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
