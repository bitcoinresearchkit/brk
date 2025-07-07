// @ts_check

/**
 * @typedef {Object} BaseSeriesBlueprint
 * @property {string} title
 * @property {boolean} [defaultActive]
 *
 * @typedef {Object} BaselineSeriesBlueprintSpecific
 * @property {"Baseline"} type
 * @property {Color} [color]
 * @property {[Color, Color]} [colors]
 * @property {PartialBaselineStyleOptions} [options]
 * @property {Accessor<BaselineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & BaselineSeriesBlueprintSpecific} BaselineSeriesBlueprint
 *
 * @typedef {Object} CandlestickSeriesBlueprintSpecific
 * @property {"Candlestick"} type
 * @property {Color} [color]
 * @property {PartialCandlestickStyleOptions} [options]
 * @property {Accessor<CandlestickData[]>} [data]
 * @typedef {BaseSeriesBlueprint & CandlestickSeriesBlueprintSpecific} CandlestickSeriesBlueprint
 *
 * @typedef {Object} LineSeriesBlueprintSpecific
 * @property {"Line"} [type]
 * @property {Color} [color]
 * @property {PartialLineStyleOptions} [options]
 * @property {Accessor<LineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & LineSeriesBlueprintSpecific} LineSeriesBlueprint
 *
 * @typedef {BaselineSeriesBlueprint | CandlestickSeriesBlueprint | LineSeriesBlueprint} AnySeriesBlueprint
 *
 * @typedef {AnySeriesBlueprint["type"]} SeriesType
 *
 * @typedef {{ key: VecId, unit?: Unit | Unit[] }} FetchedAnySeriesOptions
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
 * @typedef {Object} PartialExplorerOptionSpecific
 * @property {"explorer"} kind
 * @property {string} title
 *
 * @typedef {PartialOption & PartialExplorerOptionSpecific} PartialExplorerOption
 *
 * @typedef {Required<PartialExplorerOption> & ProcessedOptionAddons} ExplorerOption
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
 * @typedef {PartialExplorerOption | PartialChartOption | PartialTableOption | PartialSimulationOption | PartialUrlOption} AnyPartialOption
 *
 * @typedef {ExplorerOption | ChartOption | TableOption | SimulationOption | UrlOption} Option
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
 * @param {Object} args
 * @param {Env} args.env
 * @param {Colors} args.colors
 * @param {VecIdToIndexes} args.vecIdToIndexes
 * @returns {PartialOptionsTree}
 */
function createPartialOptions({ env, colors, vecIdToIndexes }) {
  /**
   * @template {string} S
   * @typedef {Extract<VecId, `${S}${string}`>} StartsWith
   */
  /**
   * @template {string} S
   * @typedef {Extract<VecId, `${string}${S}`>} EndsWith
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
   * @template {string} K
   * @template {string} S
   * @typedef {K extends `${infer _Prefix}${S}${infer _Suffix}` ? never : K} ExcludeSubstring
   */

  /**
   * @typedef {"cumulative_"} CumulativePrefix
   * @typedef {"_30d_change"} _30DChageSubString
   * @typedef {StartsWith<CumulativePrefix>} CumulativeVecId
   * @typedef {ExcludeSubstring<WithoutPrefix<CumulativeVecId, CumulativePrefix>, _30DChageSubString>} CumulativeVecIdBase
   * @typedef {"_average"} AverageSuffix
   * @typedef {EndsWith<AverageSuffix>} VecIdAverage
   * @typedef {WithoutSuffix<VecIdAverage, AverageSuffix>} VecIdAverageBase
   * @typedef {"_median"} MedianSuffix
   * @typedef {EndsWith<MedianSuffix>} VecIdMedian
   * @typedef {WithoutSuffix<VecIdMedian, MedianSuffix>} VecIdMedianBase
   * @typedef {"_90p"} _90pSuffix
   * @typedef {EndsWith<_90pSuffix>} VecId90p
   * @typedef {WithoutSuffix<VecId90p, _90pSuffix>} VecId90pBase
   * @typedef {"_75p"} _75pSuffix
   * @typedef {EndsWith<_75pSuffix>} VecId75p
   * @typedef {WithoutSuffix<VecId75p, _75pSuffix>} VecId75pBase
   * @typedef {"_25p"} _25pSuffix
   * @typedef {EndsWith<_25pSuffix>} VecId25p
   * @typedef {WithoutSuffix<VecId25p, _25pSuffix>} VecId25pBase
   * @typedef {"_10p"} _10pSuffix
   * @typedef {EndsWith<_10pSuffix>} VecId10p
   * @typedef {WithoutSuffix<VecId10p, _10pSuffix>} VecId10pBase
   * @typedef {"_max"} MaxSuffix
   * @typedef {EndsWith<MaxSuffix>} VecIdMax
   * @typedef {WithoutSuffix<VecIdMax, MaxSuffix>} VecIdMaxBase
   * @typedef {"_min"} MinSuffix
   * @typedef {EndsWith<MinSuffix>} VecIdMin
   * @typedef {WithoutSuffix<VecIdMin, MinSuffix>} VecIdMinBase
   */

  const averages = /** @type {const} */ ([
    { name: "1 Week", key: "1w", days: 7, color: colors.red },
    { name: "8 Day", key: "8d", days: 8, color: colors.orange },
    { name: "13 Day", key: "13d", days: 13, color: colors.amber },
    { name: "21 Day", key: "21d", days: 21, color: colors.yellow },
    { name: "1 Month", key: "1m", days: 30, color: colors.lime },
    { name: "34 Day", key: "34d", days: 34, color: colors.green },
    { name: "55 Day", key: "55d", days: 55, color: colors.emerald },
    { name: "89 Day", key: "89d", days: 89, color: colors.teal },
    { name: "144 Day", key: "144d", days: 144, color: colors.cyan },
    { name: "200 Day", key: "200d", days: 200, color: colors.sky },
    { name: "1 Year", key: "1y", days: 365, color: colors.blue },
    { name: "2 Year", key: "2y", days: 2 * 365, color: colors.indigo },
    { name: "200 Week", key: "200w", days: 200 * 7, color: colors.violet },
    { name: "4 Year", key: "4y", days: 4 * 365, color: colors.purple },
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

  const upToDate = /** @type {const} */ ([
    {
      key: "up_to_1d",
      name: "1d",
      title: "Up to 1 Day",
      color: colors.pink,
    },
    {
      key: "up_to_1w",
      name: "1w",
      title: "Up to 1 Week",
      color: colors.red,
    },
    {
      key: "up_to_1m",
      name: "1m",
      title: "Up to 1 Month",
      color: colors.orange,
    },
    {
      key: "up_to_2m",
      name: "2m",
      title: "Up to 2 Months",
      color: colors.amber,
    },
    {
      key: "up_to_3m",
      name: "3m",
      title: "Up to 3 Months",
      color: colors.yellow,
    },
    {
      key: "up_to_4m",
      name: "4m",
      title: "Up to 4 Months",
      color: colors.lime,
    },
    {
      key: "up_to_5m",
      name: "5m",
      title: "Up to 5 Months",
      color: colors.green,
    },
    {
      key: "up_to_6m",
      name: "6m",
      title: "Up to 6 Months",
      color: colors.teal,
    },
    {
      key: "up_to_1y",
      name: "1y",
      title: "Up to 1 Year",
      color: colors.sky,
    },
    {
      key: "up_to_2y",
      name: "2y",
      title: "Up to 2 Years",
      color: colors.indigo,
    },
    {
      key: "up_to_3y",
      name: "3y",
      title: "Up to 3 Years",
      color: colors.violet,
    },
    {
      key: "up_to_4y",
      name: "4y",
      title: "Up to 4 Years",
      color: colors.purple,
    },
    {
      key: "up_to_5y",
      name: "5y",
      title: "Up to 5 Years",
      color: colors.fuchsia,
    },
    {
      key: "up_to_6y",
      name: "6y",
      title: "Up to 6 Years",
      color: colors.pink,
    },
    {
      key: "up_to_7y",
      name: "7y",
      title: "Up to 7 Years",
      color: colors.red,
    },
    {
      key: "up_to_8y",
      name: "8y",
      title: "Up to 8 Years",
      color: colors.orange,
    },
    {
      key: "up_to_10y",
      name: "10y",
      title: "Up to 10 Years",
      color: colors.amber,
    },
    {
      key: "up_to_15y",
      name: "15y",
      title: "Up to 15 Years",
      color: colors.yellow,
    },
  ]);

  const fromDate = /** @type {const} */ ([
    {
      key: "from_1d",
      name: "1d",
      title: "From 1 Day",
      color: colors.red,
    },
    {
      key: "from_1w",
      name: "1w",
      title: "From 1 Week",
      color: colors.orange,
    },
    {
      key: "from_1m",
      name: "1m",
      title: "From 1 Month",
      color: colors.yellow,
    },
    {
      key: "from_2m",
      name: "2m",
      title: "From 2 Months",
      color: colors.lime,
    },
    {
      key: "from_3m",
      name: "3m",
      title: "From 3 Months",
      color: colors.green,
    },
    {
      key: "from_4m",
      name: "4m",
      title: "From 4 Months",
      color: colors.teal,
    },
    {
      key: "from_5m",
      name: "5m",
      title: "From 5 Months",
      color: colors.cyan,
    },
    {
      key: "from_6m",
      name: "6m",
      title: "From 6 Months",
      color: colors.blue,
    },
    {
      key: "from_1y",
      name: "1y",
      title: "From 1 Year",
      color: colors.indigo,
    },
    {
      key: "from_2y",
      name: "2y",
      title: "From 2 Years",
      color: colors.violet,
    },
    {
      key: "from_3y",
      name: "3y",
      title: "From 3 Years",
      color: colors.purple,
    },
    {
      key: "from_4y",
      name: "4y",
      title: "From 4 Years",
      color: colors.fuchsia,
    },
    {
      key: "from_5y",
      name: "5y",
      title: "From 5 Years",
      color: colors.pink,
    },
    {
      key: "from_6y",
      name: "6y",
      title: "From 6 Years",
      color: colors.rose,
    },
    {
      key: "from_7y",
      name: "7y",
      title: "From 7 Years",
      color: colors.red,
    },
    {
      key: "from_8y",
      name: "8y",
      title: "From 8 Years",
      color: colors.orange,
    },
    {
      key: "from_10y",
      name: "10y",
      title: "From 10 Years",
      color: colors.yellow,
    },
    {
      key: "from_15y",
      name: "15y",
      title: "From 15 Years",
      color: colors.lime,
    },
  ]);

  const dateRange = /** @type {const} */ ([
    {
      key: "start_to_1d",
      name: "24h",
      title: "Last 24 hours",
      color: colors.pink,
    },
    {
      key: "from_1d_to_1w",
      name: "1d..1w",
      title: "From 1 Day ago to 1 Week ago",
      color: colors.red,
    },
    {
      key: "from_1w_to_1m",
      name: "1w..1m",
      title: "From 1 Week ago to 1 Month ago",
      color: colors.orange,
    },
    {
      key: "from_1m_to_2m",
      name: "1m..2m",
      title: "From 1 Month ago to 2 Months ago",
      color: colors.yellow,
    },
    {
      key: "from_2m_to_3m",
      name: "2m..3m",
      title: "From 2 Month ago to 3 Months ago",
      color: colors.yellow,
    },
    {
      key: "from_3m_to_4m",
      name: "3m..4m",
      title: "From 3 Month ago to 4 Months ago",
      color: colors.lime,
    },
    {
      key: "from_4m_to_5m",
      name: "4m..5m",
      title: "From 4 Month ago to 5 Months ago",
      color: colors.lime,
    },
    {
      key: "from_5m_to_6m",
      name: "5m..6m",
      title: "From 5 Month ago to 6 Months ago",
      color: colors.lime,
    },
    {
      key: "from_6m_to_1y",
      name: "6m..1y",
      title: "From 6 Months ago to 1 Year ago",
      color: colors.green,
    },
    {
      key: "from_1y_to_2y",
      name: "1y..2y",
      title: "From 1 Year ago to 2 Years ago",
      color: colors.cyan,
    },
    {
      key: "from_2y_to_3y",
      name: "2y..3y",
      title: "From 2 Years ago to 3 Years ago",
      color: colors.blue,
    },
    {
      key: "from_3y_to_4y",
      name: "3y..4y",
      title: "From 3 Years ago to 4 Years ago",
      color: colors.indigo,
    },
    {
      key: "from_4y_to_5y",
      name: "4y..5y",
      title: "From 4 Years ago to 5 Years ago",
      color: colors.violet,
    },
    {
      key: "from_5y_to_6y",
      name: "5y..6y",
      title: "From 5 Years ago to 6 Years ago",
      color: colors.purple,
    },
    {
      key: "from_6y_to_7y",
      name: "6y..7y",
      title: "From 6 Years ago to 7 Years ago",
      color: colors.purple,
    },
    {
      key: "from_7y_to_8y",
      name: "7y..8y",
      title: "From 7 Years ago to 8 Years ago",
      color: colors.fuchsia,
    },
    {
      key: "from_8y_to_10y",
      name: "8y..10y",
      title: "From 8 Years ago to 10 Years ago",
      color: colors.fuchsia,
    },
    {
      key: "from_10y_to_15y",
      name: "10y..15y",
      title: "From 10 Years ago to 15 Years ago",
      color: colors.pink,
    },
    {
      key: "from_15y_to_end",
      name: "15y+",
      title: "From 15 Years ago to genesis (2009_01_03)",
      color: colors.red,
    },
  ]);

  const epoch = /** @type {const} */ ([
    {
      key: "epoch_0",
      name: "0",
      title: "Epoch 0",
      color: colors.red,
    },
    {
      key: "epoch_1",
      name: "1",
      title: "Epoch 1",
      color: colors.yellow,
    },
    {
      key: "epoch_2",
      name: "2",
      title: "Epoch 2",
      color: colors.orange,
    },
    {
      key: "epoch_3",
      name: "3",
      title: "Epoch 3",
      color: colors.lime,
    },
    {
      key: "epoch_4",
      name: "4",
      title: "Epoch 4",
      color: colors.green,
    },
  ]);

  const fromSize = /** @type {const} */ ([
    {
      key: "from_1sat",
      name: ">=1 sat",
      title: "From 1 sat",
      color: colors.orange,
    },
    {
      key: "from_10sats",
      name: ">=10 sats",
      title: "From 10 sats",
      color: colors.orange,
    },
    {
      key: "from_100sats",
      name: ">=100 sats",
      title: "From 100 sats",
      color: colors.yellow,
    },
    {
      key: "from_1k_sats",
      name: ">=1K sats",
      title: "From 1K sats",
      color: colors.lime,
    },
    {
      key: "from_10k_sats",
      name: ">=10K sats",
      title: "From 10K sats",
      color: colors.green,
    },
    {
      key: "from_100k_sats",
      name: ">=100K sats",
      title: "From 100K sats",
      color: colors.cyan,
    },
    {
      key: "from_1m_sats",
      name: ">=1M sats",
      title: "From 1M sats",
      color: colors.blue,
    },
    {
      key: "from_10m_sats",
      name: ">=10M sats",
      title: "From 10M sats",
      color: colors.indigo,
    },
    {
      key: "from_1btc",
      name: ">=1 btc",
      title: "From 1 BTC",
      color: colors.purple,
    },
    {
      key: "from_10btc",
      name: ">=10 btc",
      title: "From 10 BTC",
      color: colors.violet,
    },
    {
      key: "from_100btc",
      name: ">=100 btc",
      title: "From 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "from_1k_btc",
      name: ">=1K btc",
      title: "From 1K BTC",
      color: colors.pink,
    },
    {
      key: "from_10k_btc",
      name: ">=10K btc",
      title: "From 10K BTC",
      color: colors.red,
    },
  ]);

  const upToSize = /** @type {const} */ ([
    {
      key: "up_to_10sats",
      name: "<10 sats",
      title: "Up to 10 sats",
      color: colors.orange,
    },
    {
      key: "up_to_100sats",
      name: "<100 sats",
      title: "Up to 100 sats",
      color: colors.yellow,
    },
    {
      key: "up_to_1k_sats",
      name: "<1K sats",
      title: "Up to 1K sats",
      color: colors.lime,
    },
    {
      key: "up_to_10k_sats",
      name: "<10K sats",
      title: "Up to 10K sats",
      color: colors.green,
    },
    {
      key: "up_to_100k_sats",
      name: "<100K sats",
      title: "Up to 100K sats",
      color: colors.cyan,
    },
    {
      key: "up_to_1m_sats",
      name: "<1M sats",
      title: "Up to 1M sats",
      color: colors.blue,
    },
    {
      key: "up_to_10m_sats",
      name: "<10M sats",
      title: "Up to 10M sats",
      color: colors.indigo,
    },
    {
      key: "up_to_1btc",
      name: "<1 btc",
      title: "up to 1 BTC",
      color: colors.purple,
    },
    {
      key: "up_to_10btc",
      name: "<10 btc",
      title: "Up to 10 BTC",
      color: colors.violet,
    },
    {
      key: "up_to_100btc",
      name: "<100 btc",
      title: "Up to 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "up_to_1k_btc",
      name: "<1K btc",
      title: "up to 1K BTC",
      color: colors.pink,
    },
    {
      key: "up_to_10k_btc",
      name: "<10K btc",
      title: "up to 10K BTC",
      color: colors.red,
    },
    {
      key: "up_to_100k_btc",
      name: "<100K btc",
      title: "up to 100K BTC",
      color: colors.orange,
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
      key: "from_1sat_to_10sats",
      name: "1 sat..10 sats",
      title: "From 1 sat to 10 sats",
      color: colors.orange,
    },
    {
      key: "from_10sats_to_100sats",
      name: "10 sats..100 sats",
      title: "From 10 sats to 100 sats",
      color: colors.yellow,
    },
    {
      key: "from_100sats_to_1_000sats",
      name: "100 sats..1K sats",
      title: "From 100 sats to 1K sats",
      color: colors.lime,
    },
    {
      key: "from_1_000sats_to_10_000sats",
      name: "1K sats..10K sats",
      title: "From 1K sats to 10K sats",
      color: colors.green,
    },
    {
      key: "from_10_000sats_to_100_000sats",
      name: "10K sats..100K sats",
      title: "From 10K sats to 100K sats",
      color: colors.cyan,
    },
    {
      key: "from_100_000sats_to_1_000_000sats",
      name: "100K sats .. 1M sats",
      title: "From 100K sats to 1M sats",
      color: colors.blue,
    },
    {
      key: "from_1_000_000sats_to_10_000_000sats",
      name: "1M sats..10M sats",
      title: "From 1M sats to 10M sats",
      color: colors.indigo,
    },
    {
      key: "from_10_000_000sats_to_1btc",
      name: "10M sats..1 btc",
      title: "From 10M sats to 1 BTC",
      color: colors.purple,
    },
    {
      key: "from_1btc_to_10btc",
      name: "1 btc..10 btc",
      title: "From 1 BTC to 10 BTC",
      color: colors.violet,
    },
    {
      key: "from_10btc_to_100btc",
      name: "10 btc..100 btc",
      title: "From 10 BTC to 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "from_100btc_to_1_000btc",
      name: "100 btc..1K btc",
      title: "From 100 BTC to 1K BTC",
      color: colors.pink,
    },
    {
      key: "from_1_000btc_to_10_000btc",
      name: "1K btc..10K btc",
      title: "From 1K BTC to 10K BTC",
      color: colors.red,
    },
    {
      key: "from_10_000btc_to_100_000btc",
      name: "10K btc..100K btc",
      title: "From 10K BTC to 100K BTC",
      color: colors.orange,
    },
    {
      key: "from_100_000btc",
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

  const cointimePrices = /** @type {const} */ ([
    {
      key: `vaulted_price`,
      name: "Vaulted",
      title: "Vaulted Price",
      color: colors.lime,
    },
    {
      key: `active_price`,
      name: "Active",
      title: "Active Price",
      color: colors.rose,
    },
    {
      key: `true_market_mean`,
      name: "True market mean",
      title: "True market mean",
      color: colors.blue,
    },
    {
      key: `cointime_price`,
      name: "cointime",
      title: "Cointime Price",
      color: colors.yellow,
    },
  ]);

  const cointimeCapitalizations = /** @type {const} */ ([
    {
      key: `thermo_cap`,
      name: "Thermo",
      title: "Thermo Capitalization",
      color: colors.emerald,
    },
    {
      key: `investor_cap`,
      name: "Investor",
      title: "Investor Capitalization",
      color: colors.fuchsia,
    },
    {
      key: `active_cap`,
      name: "Active",
      title: "Active Capitalization",
      color: colors.rose,
    },
    {
      key: `vaulted_cap`,
      name: "Vaulted",
      title: "Vaulted Capitalization",
      color: colors.lime,
    },
    {
      key: `cointime_cap`,
      name: "Cointime",
      title: "Cointime Capitalization",
      color: colors.yellow,
    },
  ]);

  /**
   * @param {Object} args
   * @param {VecId} args.key
   * @param {string} args.name
   * @param {Color} [args.color]
   * @param {boolean} [args.defaultActive]
   * @param {PartialLineStyleOptions} [args.options]
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
      key: `${concat}_average`,
      title: "Average",
    });
  }

  /**
   * @param {Object} args
   * @param {CumulativeVecIdBase} args.concat
   * @param {string} [args.name]
   */
  function createSumCumulativeSeries({ concat, name }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        key: concat,
        title: name ? `${name} Sum` : "Sum",
        color: colors.orange,
      },
      {
        key: `cumulative_${concat}`,
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
        key: `${concat}_max`,
        title: "Max",
        color: colors.pink,
        defaultActive: false,
      },
      {
        key: `${concat}_min`,
        title: "Min",
        color: colors.green,
        defaultActive: false,
      },
      {
        key: `${concat}_median`,
        title: "Median",
        color: colors.amber,
        defaultActive: false,
      },
      {
        key: `${concat}_75p`,
        title: "75p",
        color: colors.red,
        defaultActive: false,
      },
      {
        key: `${concat}_25p`,
        title: "25p",
        color: colors.yellow,
        defaultActive: false,
      },
      {
        key: `${concat}_90p`,
        title: "90p",
        color: colors.rose,
        defaultActive: false,
      },
      {
        key: `${concat}_10p`,
        title: "10p",
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * @param {VecIdAverageBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} key
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
   * @param {VecId & VecIdAverageBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecId90pBase & VecId75pBase & VecIdMedianBase & VecId25pBase & VecId10pBase} args.key
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
   * @param {VecId & CumulativeVecIdBase} args.key
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
   * @typedef {"_ratio_zscore"} RatioZScoreCapSuffix
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
          key: `${key}_ratio_p1sd_as_price`,
          name: "+1σ",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p2sd_as_price`,
          name: "+2σ",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p3sd_as_price`,
          name: "+3σ",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m1sd_as_price`,
          name: "−1σ",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m2sd_as_price`,
          name: "−2σ",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m3sd_as_price`,
          name: "−3σ",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99_as_price`,
          name: "p99",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99_5_as_price`,
          name: "p99.5",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99_9_as_price`,
          name: "p99.9",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p1_as_price`,
          name: "p1",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p0_5_as_price`,
          name: "p0.5",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p0_1_as_price`,
          name: "p0.1",
          color: colors.violet,
          defaultActive: false,
        }),
      ],
      bottom: [
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}_ratio`,
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
          key: `${key}_ratio_p1sd`,
          name: "+1σ",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p2sd`,
          name: "+2σ",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p3sd`,
          name: "+3σ",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m1sd`,
          name: "−1σ",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m2sd`,
          name: "−2σ",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_m3sd`,
          name: "−3σ",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99`,
          name: "p99",
          color: colors.orange,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99_5`,
          name: "p99.5",
          color: colors.red,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p99_9`,
          name: "p99.9",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p1`,
          name: "p1",
          color: colors.cyan,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p0_5`,
          name: "p0.5",
          color: colors.blue,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_p0_1`,
          name: "p0.1",
          color: colors.violet,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_1w_sma`,
          name: "1w sma",
          color: colors.fuchsia,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_1m_sma`,
          name: "1m sma",
          color: colors.pink,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_1y_sma`,
          name: "1y sma",
          color: colors.rose,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_4y_sma`,
          name: "4y_sma",
          color: colors.purple,
          defaultActive: false,
        }),
        createBaseSeries({
          key: `${key}_ratio_sma`,
          name: "sma",
          color: colors.yellow,
          defaultActive: false,
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}_ratio_1y_sma_momentum_oscillator`,
          title: "1Y Momentum",
          type: "Baseline",
          options: {
            createPriceLine: {
              value: 0,
            },
          },
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}_ratio_zscore`,
          title: "All time",
          type: "Baseline",
          options: {
            createPriceLine: {
              value: 0,
            },
          },
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}_ratio_4y_zscore`,
          title: "4y",
          type: "Baseline",
          colors: [colors.yellow, colors.pink],
          options: {
            createPriceLine: {
              value: 0,
            },
          },
        }),
        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
          key: `${key}_ratio_1y_zscore`,
          title: "1y",
          type: "Baseline",
          colors: [colors.orange, colors.purple],
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
   * @typedef {"_supply_in_profit"} SupplyInProfitSuffix
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
   * @template {"" | CohortId} T
   * @param {T} _key
   */
  const fixKey = (_key) =>
    _key !== ""
      ? /** @type {Exclude<"" | `${T}_`, "_">} */ (`${_key}_`)
      : /** @type {const} */ ("");

  /**
   * @param {UTXOGroupObject | UTXOGroupsObject} args
   */
  function createCohortGroupFolder(args) {
    const list = "list" in args ? args.list : [args];
    const useGroupName = "list" in args;

    return /** @satisfies {PartialOptionsGroup} */ ({
      name: args.name || "all",
      tree: [
        !("list" in args)
          ? {
              name: "supply",
              title: `${args.title} Supply`,
              bottom: list.flatMap(({ color, name, key: _key }) => {
                const key = fixKey(_key);
                return /** @type {const} */ ([
                  createBaseSeries({
                    key: `${key}supply`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_btc`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_usd`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  ...(key
                    ? [
                        createBaseSeries({
                          key: `${key}supply_relative_to_circulating_supply`,
                          name: "Supply",
                          color: colors.default,
                        }),
                      ]
                    : []),
                  createBaseSeries({
                    key: `${key}halved_supply`,
                    name: "Halved",
                    color: colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_profit`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_profit_in_btc`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_profit_in_usd`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_loss`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_loss_in_btc`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_loss_in_usd`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    key: `${key}supply_even`,
                    name: useGroupName ? name : "Even",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}supply_even_in_btc`,
                    name: useGroupName ? name : "Even",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}supply_even_in_usd`,
                    name: useGroupName ? name : "Even",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}halved_supply_in_btc`,
                    name: useGroupName ? name : "Halved",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    key: `${key}halved_supply_in_usd`,
                    name: useGroupName ? name : "Halved",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  ...(key
                    ? [
                        createBaseSeries({
                          key: `${key}supply_in_profit_relative_to_circulating_supply`,
                          name: "In Profit",
                          color: colors.green,
                        }),
                        createBaseSeries({
                          key: `${key}supply_in_loss_relative_to_circulating_supply`,
                          name: "In Loss",
                          color: colors.red,
                        }),
                        createBaseSeries({
                          key: `${key}supply_even_relative_to_circulating_supply`,
                          name: "Even",
                          color: colors.yellow,
                        }),
                      ]
                    : []),
                  createBaseSeries({
                    key: `${key}supply_in_profit_relative_to_own_supply`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_loss_relative_to_own_supply`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    key: `${key}supply_even_relative_to_own_supply`,
                    name: "Even",
                    color: colors.yellow,
                  }),
                ]);
              }),
            }
          : {
              name: "supply",
              tree: [
                {
                  name: "total",
                  title: `${args.title} Supply`,
                  bottom: list.flatMap(({ color, name, key: _key }) => {
                    const key = fixKey(_key);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        key: `${key}supply`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_usd`,
                        name,
                        color,
                      }),
                      ...(key
                        ? [
                            createBaseSeries({
                              key: `${key}supply_relative_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                    ]);
                  }),
                },
                {
                  name: "in profit",
                  title: `${args.title} Supply In Profit`,
                  bottom: list.flatMap(({ color, name, key: _key }) => {
                    const key = fixKey(_key);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        key: `${key}supply_in_profit`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_profit_in_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_profit_in_usd`,
                        name,
                        color,
                      }),
                      ...(key
                        ? [
                            createBaseSeries({
                              key: `${key}supply_in_profit_relative_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                      createBaseSeries({
                        key: `${key}supply_in_profit_relative_to_own_supply`,
                        name,
                        color,
                      }),
                    ]);
                  }),
                },
                {
                  name: "in loss",
                  title: `${args.title} Supply In loss`,
                  bottom: list.flatMap(({ color, name, key: _key }) => {
                    const key = fixKey(_key);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        key: `${key}supply_in_loss`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_loss_in_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_in_loss_in_usd`,
                        name,
                        color,
                      }),
                      ...(key
                        ? [
                            createBaseSeries({
                              key: `${key}supply_in_loss_relative_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                      createBaseSeries({
                        key: `${key}supply_in_loss_relative_to_own_supply`,
                        name,
                        color,
                      }),
                    ]);
                  }),
                },
                {
                  name: "even",
                  title: `${args.title} Supply Even`,
                  bottom: list.flatMap(({ color, name, key: _key }) => {
                    const key = fixKey(_key);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        key: `${key}supply_even`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_even_in_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_even_in_usd`,
                        name,
                        color,
                      }),
                      ...(key
                        ? [
                            createBaseSeries({
                              key: `${key}supply_even_relative_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                      createBaseSeries({
                        key: `${key}supply_even_relative_to_own_supply`,
                        name,
                        color,
                      }),
                    ]);
                  }),
                },
              ],
            },
        {
          name: "utxo count",
          title: `${args.title} UTXO Count`,
          bottom: list.flatMap(({ color, name, key: _key }) => {
            const key = fixKey(_key);
            return /** @type {const} */ ([
              createBaseSeries({
                key: `${key}utxo_count`,
                name: useGroupName ? name : "Count",
                color,
              }),
            ]);
          }),
        },
        ...(list.filter(
          ({ key }) => `${fixKey(key)}address_count` in vecIdToIndexes,
        ).length
          ? !("list" in args) ||
            list.filter(
              ({ key }) =>
                `${fixKey(key)}empty_address_count` in vecIdToIndexes,
            ).length == 0
            ? [
                {
                  name: "address count",
                  title: `${args.title} Loaded Address Count`,
                  bottom: list.flatMap(({ name, color, key: _key }) => {
                    const key = fixKey(_key);
                    return [
                      ...(`${key}address_count` in vecIdToIndexes
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}address_count`,
                              name: useGroupName ? name : "Loaded",
                              color: useGroupName ? color : colors.orange,
                            }),
                          ])
                        : []),
                      ...(`${key}empty_address_count` in vecIdToIndexes
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}empty_address_count`,
                              name: "Empty",
                              color: colors.gray,
                              defaultActive: false,
                            }),
                          ])
                        : []),
                    ];
                  }),
                },
              ]
            : [
                {
                  name: "address count",
                  tree: [
                    {
                      name: "loaded",
                      title: `${args.title} Loaded Address Count`,
                      bottom: list
                        .filter(
                          ({ key }) =>
                            key !== "empty" &&
                            `${fixKey(key)}address_count` in vecIdToIndexes,
                        )
                        .flatMap(({ name, color, key: _key }) => {
                          const key = fixKey(_key);
                          return [
                            createBaseSeries({
                              key: `${key}address_count`,
                              name,
                              color,
                            }),
                          ];
                        }),
                    },
                    ...(list.filter(
                      ({ key }) =>
                        `${fixKey(key)}empty_address_count` in vecIdToIndexes,
                    ).length
                      ? [
                          {
                            name: "empty",
                            title: `${args.title} Empty Address Count`,
                            bottom: list
                              .filter(
                                ({ key }) =>
                                  `${fixKey(key)}empty_address_count` in
                                  vecIdToIndexes,
                              )
                              .flatMap(({ name, color, key: _key }) => {
                                const key = fixKey(_key);
                                return [
                                  createBaseSeries({
                                    key: `${key}empty_address_count`,
                                    name,
                                    color,
                                  }),
                                ];
                              }),
                          },
                        ]
                      : []),
                  ],
                },
              ]
          : []),
        // list.filter(({ key }) => key.endsWith("address_count")).map(callbackfn),
        // {
        //   name: "loaded",
        //   title: `${args.title} Loaded Address Count`,
        //   bottom: list.flatMap(({ color, name, key: _key }) => {
        //     const key = fixKey(_key);
        //     return /** @type {const} */ ([
        //       createBaseSeries({
        //         key: `${key}address_count`,
        //         name: useGroupName ? name : "Loaded",
        //         color,
        //       }),
        //       createBaseSeries({
        //         key: `${key}empty_address_count`,
        //         name: useGroupName ? name : "Empty",
        //         color,
        //       }),
        //     ]);
        //   }),
        // },
        // {
        //   name: "empty",
        //   title: `${args.title} Empty Address Count`,
        //   bottom: list.flatMap(({ color, name, key: _key }) => {
        //     const key = fixKey(_key);
        //     return /** @type {const} */ ([
        //     ]);
        //   }),
        // },
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
                    key: `${key}realized_cap`,
                    name: useGroupName ? name : "Cap",
                    color,
                  }),
                  ...(!("list" in args)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${key}realized_cap_30d_change`,
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
                      key: `${fixKey(key)}realized_price`,
                      name,
                      color,
                    }),
                  ),
                }
              : createPriceWithRatio({
                  title: `${args.title} Realized Price`,
                  key: `${fixKey(args.key)}realized_price`,
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
                        key: `${fixKey(args.key)}realized_profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `cumulative_${fixKey(args.key)}realized_profit`,
                        name: "Cumulative Profit",
                        color: colors.green,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized_loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `cumulative_${fixKey(args.key)}realized_loss`,
                        name: "Cumulative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}negative_realized_loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        key: `cumulative_${fixKey(
                          args.key,
                        )}negative_realized_loss`,
                        name: "Cumulative Negative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          args.key,
                        )}realized_profit_relative_to_realized_cap`,
                        title: "Profit",
                        color: colors.green,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          args.key,
                        )}realized_loss_relative_to_realized_cap`,
                        title: "Loss",
                        color: colors.red,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                    ],
                  },
                  {
                    name: "Net pnl",
                    title: `${args.title} Net Realized Profit And Loss`,
                    bottom: list.flatMap(({ color, name, key }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}net_realized_profit_and_loss`,
                        title: "Net",
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `cumulative_${fixKey(
                          key,
                        )}net_realized_profit_and_loss`,
                        title: "Cumulative net",
                        defaultActive: false,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `cumulative_${fixKey(
                          key,
                        )}net_realized_profit_and_loss_30d_change`,
                        title: "cum net 30d change",
                        defaultActive: false,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_profit_and_loss_relative_to_realized_cap`,
                        title: "Net",
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `cumulative_${fixKey(
                          key,
                        )}net_realized_profit_and_loss_30d_change_relative_to_realized_cap`,
                        title: "cum net 30d change",
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `cumulative_${fixKey(
                          key,
                        )}net_realized_profit_and_loss_30d_change_relative_to_market_cap`,
                        title: "cum net 30d change",
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                    ]),
                  },
                  {
                    name: "sopr",
                    title: `${args.title} Spent Output Profit Ratio`,
                    bottom: list.flatMap(({ color, name, key }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}spent_output_profit_ratio`,
                        title: "sopr",
                        options: {
                          createPriceLine: {
                            value: 1,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}adjusted_spent_output_profit_ratio`,
                        title: "asopr",
                        colors: [colors.yellow, colors.pink],
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
                    name: "profit",
                    title: `${args.title} Realized Profit`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}realized_profit`,
                          name,
                          color,
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${key}realized_profit_relative_to_realized_cap`,
                          title: name,
                          color,
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
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
                          key: `${key}realized_loss`,
                          name,
                          color,
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${key}realized_loss_relative_to_realized_cap`,
                          title: name,
                          color,
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Net pnl",
                    title: `${args.title} Net Realized Profit And Loss`,
                    bottom: list.flatMap(({ color, name, key }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}net_realized_profit_and_loss`,
                        title: name,
                        color,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_profit_and_loss_relative_to_realized_cap`,
                        title: name,
                        color,
                        options: {
                          createPriceLine: {
                            value: 0,
                          },
                        },
                      }),
                    ]),
                  },
                  {
                    name: "cumulative",
                    tree: [
                      {
                        name: "profit",
                        title: `Cumulative ${args.title} Realized Profit`,
                        bottom: list.flatMap(({ color, name, key: _key }) => {
                          const key = fixKey(_key);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              key: `cumulative_${key}realized_profit`,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "loss",
                        title: `Cumulative ${args.title} Realized Loss`,
                        bottom: list.flatMap(({ color, name, key: _key }) => {
                          const key = fixKey(_key);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              key: `cumulative_${key}realized_loss`,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "Net pnl",
                        title: `Cumulative ${args.title} Net Realized Profit And Loss`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `cumulative_${fixKey(
                              key,
                            )}net_realized_profit_and_loss`,
                            title: name,
                            color,
                            defaultActive: false,
                            options: {
                              createPriceLine: {
                                value: 0,
                              },
                            },
                          }),
                        ]),
                      },
                      {
                        name: "Net pnl 30d change",
                        title: `Cumulative ${args.title} Net Realized Profit And Loss 30 Day Change`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `cumulative_${fixKey(
                              key,
                            )}net_realized_profit_and_loss_30d_change`,
                            title: name,
                            color,
                            options: {
                              createPriceLine: {
                                value: 0,
                              },
                            },
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `cumulative_${fixKey(
                              key,
                            )}net_realized_profit_and_loss_30d_change_relative_to_realized_cap`,
                            title: name,
                            color,
                            options: {
                              createPriceLine: {
                                value: 0,
                              },
                            },
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `cumulative_${fixKey(
                              key,
                            )}net_realized_profit_and_loss_30d_change_relative_to_market_cap`,
                            title: name,
                            color,
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
                  {
                    name: "sopr",
                    tree: [
                      {
                        name: "Normal",
                        title: `${args.title} Spent Output Profit Ratio`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${fixKey(key)}spent_output_profit_ratio`,
                            title: name,
                            color,
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
                            key: `${fixKey(
                              key,
                            )}adjusted_spent_output_profit_ratio`,
                            title: name,
                            color,
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
                  key: `${fixKey(key)}sell_side_risk_ratio`,
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
                        key: `${fixKey(args.key)}unrealized_profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}unrealized_loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}negative_unrealized_loss`,
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
                          key: `${key}unrealized_profit`,
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
                          key: `${key}unrealized_loss`,
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
                  key: `${fixKey(key)}net_unrealized_profit_and_loss`,
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
                  key: `${fixKey(
                    key,
                  )}net_unrealized_profit_and_loss_relative_to_market_cap`,
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
                          key: `${key}realized_price`,
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
                          key: `${key}min_price_paid`,
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
                          key: `${key}max_price_paid`,
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
                    key: `${fixKey(args.key)}realized_price`,
                    name: "Average",
                    color: args.color,
                  }),
                  createBaseSeries({
                    key: `${fixKey(args.key)}min_price_paid`,
                    name: "Min",
                    color: colors.green,
                    // defaultActive: false,
                  }),
                  createBaseSeries({
                    key: `${fixKey(args.key)}max_price_paid`,
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
                key: `${key}coinblocks_destroyed`,
                name: useGroupName ? name : "sum",
                color,
              }),
              createBaseSeries({
                key: `cumulative_${key}coinblocks_destroyed`,
                name: useGroupName ? name : "cumulative",
                color,
                defaultActive: false,
              }),
              createBaseSeries({
                key: `${key}coindays_destroyed`,
                name: useGroupName ? name : "sum",
                color,
              }),
              createBaseSeries({
                key: `cumulative_${key}coindays_destroyed`,
                name: useGroupName ? name : "cumulative",
                color,
                defaultActive: false,
              }),
            ]);
          }),
        },
      ],
    });
  }

  return [
    ...(env.localhost
      ? /** @type {const} */ ([
          {
            name: "Explorer",
            title: "Explorer",
            kind: "explorer",
          },
        ])
      : []),
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
                  key: "days_since_ath",
                  name: "Days since",
                }),
                createBaseSeries({
                  key: "max_days_between_aths",
                  name: "Max",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "max_years_between_aths",
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
                  title: "Market Price Moving Averages",
                  top: averages.map(({ days, key, name, color }) =>
                    createBaseSeries({
                      key: `${key}_sma`,
                      name: key,
                      color,
                    }),
                  ),
                },
                ...averages.map(({ key, name, color }) =>
                  createPriceWithRatio({
                    key: `${key}_sma`,
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
                    key: `${key}_returns`,
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
            {
              name: "Indicators",
              tree: [
                {
                  name: "Mayer multiple",
                  title: "Mayer multiple",
                  top: [
                    createBaseSeries({
                      key: `200d_sma`,
                      name: "200d sma",
                      color: colors.yellow,
                    }),
                    createBaseSeries({
                      key: `200d_sma_x2_4`,
                      name: "200d sma x2.4",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `200d_sma_x0_8`,
                      name: "200d sma x0.8",
                      color: colors.red,
                    }),
                  ],
                },
              ],
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
                          key: `${key}_dca_avg_price`,
                          name: `dca`,
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          key: `price_${key}_ago`,
                          name: `lump sum`,
                          color: colors.cyan,
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}_dca_returns`,
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
                          key: `${key}_returns`,
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
                          key: `${key}_dca_avg_price`,
                          name: `dca`,
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          key: `price_${key}_ago`,
                          name: `lump sum price`,
                          color: colors.cyan,
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}_dca_returns`,
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
                          key: `${key}_dca_cagr`,
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
                          key: `${key}_returns`,
                          title: "lump sum",
                          type: "Baseline",
                          options: {
                            createPriceLine: {
                              value: 0,
                            },
                          },
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}_cagr`,
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
                      key: `dca_class_${year}_avg_price`,
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
                          key: `dca_class_${year}_avg_price`,
                          name: `avg. price`,
                          color,
                        }),
                      ],
                      bottom: [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `dca_class_${year}_returns`,
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
                  key: "cumulative_subsidy_in_btc",
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
                  key: "coinbase_in_btc",
                  name: "Coinbase",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "coinbase_in_usd",
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
                  key: "subsidy_in_btc",
                  name: "Subsidy",
                }),
                ...createBaseAverageSumCumulativeMinMaxPercentilesSeries({
                  key: "subsidy_in_usd",
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
                  "fee_in_btc",
                ),
                ...createAverageSumCumulativeMinMaxPercentilesSeries(
                  "fee_in_usd",
                ),
              ],
            },
            {
              name: "Unclaimed Rewards",
              title: "Unclaimed Rewards",
              bottom: [
                ...createBaseSumCumulativeSeries({
                  key: "unclaimed_rewards",
                  name: "unclaimed",
                }),
                ...createBaseSumCumulativeSeries({
                  key: "unclaimed_rewards_in_btc",
                  name: "unclaimed",
                }),
                ...createBaseSumCumulativeSeries({
                  key: "unclaimed_rewards_in_usd",
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
                  key: "block_count",
                  name: "Count",
                }),
                ...createSumCumulativeSeries({ concat: "block_count" }),
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
                createAverageSeries({ concat: "block_interval" }),
                ...createMinMaxPercentilesSeries({
                  concat: "block_interval",
                }),
              ],
            },
            {
              name: "Size",
              title: "Block Size",
              bottom: [
                createBaseSeries({
                  key: "total_size",
                  name: "Size",
                }),
                ...createSumCumulativeSeries({ concat: "block_size" }),
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
                ...createSumCumulativeSeries({ concat: "block_weight" }),
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
                ...createSumCumulativeSeries({ concat: "block_vbytes" }),
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
                key: "tx_count",
                name: "Count",
              }),
            },
            {
              name: "Weight",
              title: "Transaction Weight",
              bottom: [
                createAverageSeries({ concat: "tx_weight" }),
                ...createMinMaxPercentilesSeries({
                  concat: "tx_weight",
                }),
              ],
            },
            {
              name: "vsize",
              title: "Transaction Virtual Size",
              bottom: [
                createAverageSeries({ concat: "tx_vsize" }),
                ...createMinMaxPercentilesSeries({
                  concat: "tx_vsize",
                }),
              ],
            },
            {
              name: "Versions",
              title: "Transaction Versions",
              bottom: [
                createBaseSeries({
                  key: "tx_v1",
                  name: "v1 Count",
                }),
                ...createSumCumulativeSeries({ concat: "tx_v1", name: "v1" }),
                createBaseSeries({
                  key: "tx_v2",
                  name: "v2 Count",
                }),
                ...createSumCumulativeSeries({ concat: "tx_v2", name: "v2" }),
                createBaseSeries({
                  key: "tx_v3",
                  name: "v3 Count",
                }),
                ...createSumCumulativeSeries({ concat: "tx_v3", name: "v3" }),
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
                createAverageSeries({ concat: "input_count" }),
                ...createSumCumulativeSeries({ concat: "input_count" }),
                ...createMinMaxPercentilesSeries({
                  concat: "input_count",
                }),
              ],
            },
            // {
            //   name: "Value",
            //   title: "Transaction Input Value",
            //   bottom: [
            //     createAverageSeries({ concat: "input_value" }),
            //     ...createSumCumulativeSeries({ concat: "input_value" }),
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
                createAverageSeries({ concat: "output_count" }),
                ...createSumCumulativeSeries({ concat: "output_count" }),
                ...createMinMaxPercentilesSeries({
                  concat: "output_count",
                }),
              ],
            },
            // {
            //   name: "Unspent Count",
            //   title: "Unspent Transaction Output Count",
            //   bottom: [
            //     createBaseSeries({
            //       key: "exact_utxo_count",
            //       name: "cumulative",
            //     }),
            //   ],
            // },
            // {
            //   name: "Value",
            //   title: "Transaction Output Value",
            //   bottom: [
            //     createAverageSeries({ concat: "output_value" }),
            //     ...createSumCumulativeSeries({ concat: "output_value" }),
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
            //           key: "p2pk33_count",
            //           name: "33B Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk33_count_sum",
            //           name: "33B sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2pk33_count",
            //           name: "33B cumulative",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk65_count",
            //           name: "65B Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pk65_count_sum",
            //           name: "65B sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2pk65_count",
            //           name: "65B cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2pkh",
            //       title: "Pay To Public Key Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2pkh_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2pkh_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2pkh_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2ms",
            //       title: "Pay To Multisig Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2ms_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2ms_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2ms_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2sh",
            //       title: "Pay To Script Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2sh_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2sh_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2sh_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "op_return",
            //       title: "op_return outputs",
            //       bottom: [
            //         createBaseSeries({ key: "opreturn_count", name: "Count" }),
            //         createBaseSeries({
            //           key: "opreturn_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_opreturn_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2wpkh",
            //       title: "Pay To Witness Public Key Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2wpkh_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2wpkh_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2wpkh_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2wsh",
            //       title: "Pay To Witness Script Hash Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2wsh_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2wsh_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2wsh_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2tr",
            //       title: "Pay To Taproot Outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2tr_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2tr_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2tr_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "p2a",
            //       title: "Pay To Anchor outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "p2a_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "p2a_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_p2a_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "empty",
            //       title: "empty outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "emptyoutput_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "emptyoutput_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_emptyoutput_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //     {
            //       name: "unknown",
            //       title: "unknown outputs",
            //       bottom: [
            //         createBaseSeries({
            //           key: "unknownoutput_count",
            //           name: "Count",
            //         }),
            //         createBaseSeries({
            //           key: "unknownoutput_count_sum",
            //           name: "sum",
            //         }),
            //         createBaseSeries({
            //           key: "cumulative_unknownoutput_count",
            //           name: "cumulative",
            //         }),
            //       ],
            //     },
            //   ],
            //   // title: "Transaction Output Value",
            //   // bottom: [
            //   //   createAverageSeries({ concat: "output_value" }),
            //   //   ...createSumCumulativeSeries({ concat: "output_value" }),
            //   // ],
            // },
          ],
        },
        {
          name: "Cohorts",
          tree: [
            createCohortGroupFolder({
              key: "",
              name: "",
              title: "",
              color: colors.orange,
            }),
            {
              name: "term",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By Term",
                  list: terms,
                }),
                ...terms.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Up to date",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By Up To",
                  list: upToDate,
                }),
                ...upToDate.map(createCohortGroupFolder),
              ],
            },
            {
              name: "From Date",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By From",
                  list: fromDate,
                }),
                ...fromDate.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Date Range",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By Range",
                  list: dateRange,
                }),
                ...dateRange.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Epoch",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By Epoch",
                  list: epoch,
                }),
                ...epoch.map(createCohortGroupFolder),
              ],
            },
            {
              name: "type",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare By Type",
                  list: type,
                }),
                ...type.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs Up to size",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare UTXOs By Up To Size",
                  list: upToSize,
                }),
                ...upToSize.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs From size",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare UTXOs By From Size",
                  list: fromSize,
                }),
                ...fromSize.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs Size range",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare UTXOs By Size Range",
                  list: sizeRanges,
                }),
                ...sizeRanges.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses Up to size",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare Addresses By Up To Size",
                  list: upToSize.map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  ),
                }),
                ...upToSize
                  .map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  )
                  .map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses From size",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare Addresses By From Size",
                  list: fromSize.map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  ),
                }),
                ...fromSize
                  .map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  )
                  .map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses Size range",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Compare Addresses By Size Range",
                  list: sizeRanges.map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  ),
                }),
                ...sizeRanges
                  .map(
                    (obj) =>
                      /** @type {const} */ ({
                        ...obj,
                        key: `addresses_${obj.key}`,
                        title: `Addresses ${obj.title}`,
                      }),
                  )
                  .map(createCohortGroupFolder),
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
                  key: "unspendable_supply",
                  name: "Supply",
                }),
                createBaseSeries({
                  key: "unspendable_supply_in_btc",
                  name: "Supply",
                }),
                createBaseSeries({
                  key: "unspendable_supply_in_usd",
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
                    createBaseSeries({ key: "opreturn_count", name: "Count" }),
                    createBaseSeries({
                      key: "opreturn_count",
                      name: "sum",
                    }),
                    createBaseSeries({
                      key: "cumulative_opreturn_count",
                      name: "cumulative",
                      color: colors.red,
                    }),
                  ],
                },
                {
                  name: "supply",
                  title: "OP_return Supply",
                  bottom: [
                    createBaseSeries({
                      key: "opreturn_supply",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "opreturn_supply_in_btc",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      key: "opreturn_supply_in_usd",
                      name: "Supply",
                    }),
                  ],
                },
              ],
            },
          ],
        },
        {
          name: "Cointime",
          tree: [
            {
              name: "Coinblocks",
              title: "Coinblocks",
              bottom: [
                createBaseSeries({
                  key: "coinblocks_destroyed",
                  name: "Destroyed",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "cumulative_coinblocks_destroyed",
                  name: "Cumulative Destroyed",
                  color: colors.red,
                  defaultActive: false,
                }),
                createBaseSeries({
                  key: "coinblocks_created",
                  name: "created",
                  color: colors.orange,
                }),
                createBaseSeries({
                  key: "cumulative_coinblocks_created",
                  name: "Cumulative created",
                  color: colors.orange,
                  defaultActive: false,
                }),
                createBaseSeries({
                  key: "coinblocks_stored",
                  name: "stored",
                  color: colors.green,
                }),
                createBaseSeries({
                  key: "cumulative_coinblocks_stored",
                  name: "Cumulative stored",
                  color: colors.green,
                  defaultActive: false,
                }),
              ],
            },
            {
              name: "Liveliness & Vaultedness",
              title: "Liveliness & Vaultedness",
              bottom: [
                createBaseSeries({
                  key: "liveliness",
                  name: "Liveliness",
                  color: colors.rose,
                }),
                createBaseSeries({
                  key: "vaultedness",
                  name: "Vaultedness",
                  color: colors.lime,
                }),
              ],
            },
            {
              name: "Supply",
              title: "Cointime Supply",
              bottom: /** @type {const} */ ([
                {
                  name: "all",
                  color: colors.orange,
                },
                {
                  name: "vaulted",
                  color: colors.lime,
                },
                { name: "active", color: colors.rose },
              ]).flatMap(
                ({ name, color }) =>
                  /** @type {const} */ ([
                    createBaseSeries({
                      key: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply`,
                      name,
                      color,
                    }),
                    createBaseSeries({
                      key: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply_in_btc`,
                      name,
                      color,
                    }),
                    createBaseSeries({
                      key: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply_in_usd`,
                      name,
                      color,
                    }),
                  ]),
              ),
            },
            {
              name: "Capitalization",
              tree: [
                {
                  name: "Compare",
                  title: "Compare Cointime Capitalizations",
                  bottom: [
                    createBaseSeries({
                      key: `marketcap`,
                      name: "Market",
                      color: colors.default,
                    }),
                    createBaseSeries({
                      key: `realized_cap`,
                      name: "Realized",
                      color: colors.orange,
                    }),
                    ...cointimeCapitalizations.map(({ key, name, color }) =>
                      createBaseSeries({
                        key,
                        name,
                        color,
                      }),
                    ),
                  ],
                },
                ...cointimeCapitalizations.map(
                  ({ key, name, color, title }) => ({
                    name,
                    title,
                    bottom: [
                      createBaseSeries({
                        key,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `marketcap`,
                        name: "Market",
                        color: colors.default,
                      }),
                      createBaseSeries({
                        key: `realized_cap`,
                        name: "Realized",
                        color: colors.orange,
                      }),
                    ],
                  }),
                ),
              ],
            },
            {
              name: "Prices",
              tree: [
                {
                  name: "Compare",
                  title: "Compare Cointime Prices",
                  top: cointimePrices.map(({ key, name, color }) =>
                    createBaseSeries({
                      key,
                      name,
                      color,
                    }),
                  ),
                },
                ...cointimePrices.map(({ key, name, color, title }) =>
                  createPriceWithRatio({
                    key,
                    legend: name,
                    color,
                    name,
                    title,
                  }),
                ),
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
      name: "Tools",
      tree: [
        {
          name: "API",
          url: () => "/api",
        },
        {
          name: "MCP",
          url: () =>
            "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_mcp#brk-mcp",
        },
        {
          name: "Crates",
          url: () => "https://crates.io/crates/brk",
        },
        {
          name: "Source",
          url: () => "https://github.com/bitcoinresearchkit/brk",
        },
      ],
    },
    {
      name: "Hosting",
      tree: [
        {
          name: "Status",
          url: () => "https://status.bitcoinresearchkit.org/",
        },
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
      name: "Social",
      tree: [
        {
          name: "GitHub",
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
        {
          name: "Geyser",
          url: () => "https://geyser.fund/project/brk",
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
 * @param {VecIdToIndexes} args.vecIdToIndexes
 * @param {Signal<string | null>} args.qrcode
 */
export function initOptions({
  colors,
  signals,
  env,
  utils,
  qrcode,
  vecIdToIndexes,
}) {
  const LS_SELECTED_KEY = `selected_id`;

  const urlSelected = utils.url.pathnameToSelectedId();
  const savedSelectedId = utils.storage.read(LS_SELECTED_KEY);

  /** @type {Signal<Option>} */
  const selected = signals.createSignal(/** @type {any} */ (undefined));

  const partialOptions = createPartialOptions({ env, colors, vecIdToIndexes });

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
   */
  function createOptionElement({ option, frame, name, id, qrcode }) {
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
        inputName: `option_${frame}${id || ""}`,
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
            utils.storage.write(LS_SELECTED_KEY, option.id);
          } else if (input.checked) {
            input.checked = false;
          }
        });
      }

      createCheckEffect();

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
      // @ts_ignore
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
            supCount.innerHTML = childOptionsCount.toLocaleString("en-us");
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

        if ("kind" in anyPartial && anyPartial.kind === "explorer") {
          option = /** @satisfies {ExplorerOption} */ ({
            kind: anyPartial.kind,
            id: anyPartial.kind,
            name: anyPartial.name,
            path: path || [],
            title: anyPartial.title,
          });
        } else if ("kind" in anyPartial && anyPartial.kind === "table") {
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
      const firstChartOption = list.find((option) => option.kind === "chart");
      if (firstChartOption) {
        selected.set(firstChartOption);
      }
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
//       id: "highly_liquid",
//       name: "Highly Liquid",
//     },
//   ]);
