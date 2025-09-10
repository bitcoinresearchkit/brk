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
 * @property {BaselineSeriesPartialOptions} [options]
 * @property {Accessor<BaselineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & BaselineSeriesBlueprintSpecific} BaselineSeriesBlueprint
 *
 * @typedef {Object} CandlestickSeriesBlueprintSpecific
 * @property {"Candlestick"} type
 * @property {Color} [color]
 * @property {CandlestickSeriesPartialOptions} [options]
 * @property {Accessor<CandlestickData[]>} [data]
 * @typedef {BaseSeriesBlueprint & CandlestickSeriesBlueprintSpecific} CandlestickSeriesBlueprint
 *
 * @typedef {Object} LineSeriesBlueprintSpecific
 * @property {"Line"} [type]
 * @property {Color} [color]
 * @property {LineSeriesPartialOptions} [options]
 * @property {Accessor<LineData[]>} [data]
 * @typedef {BaseSeriesBlueprint & LineSeriesBlueprintSpecific} LineSeriesBlueprint
 *
 * @typedef {Object} HistogramSeriesBlueprintSpecific
 * @property {"Histogram"} type
 * @property {Color} color
 * @property {HistogramSeriesPartialOptions} [options]
 * @property {Accessor<HistogramData[]>} [data]
 * @typedef {BaseSeriesBlueprint & HistogramSeriesBlueprintSpecific} HistogramSeriesBlueprint
 *
 * @typedef {BaselineSeriesBlueprint | CandlestickSeriesBlueprint | LineSeriesBlueprint | HistogramSeriesBlueprint} AnySeriesBlueprint
 *
 * @typedef {AnySeriesBlueprint["type"]} SeriesType
 *
 * @typedef {{ key: VecId, unit?: Unit }} FetchedAnySeriesOptions
 *
 * @typedef {BaselineSeriesBlueprint & FetchedAnySeriesOptions} FetchedBaselineSeriesBlueprint
 * @typedef {CandlestickSeriesBlueprint & FetchedAnySeriesOptions} FetchedCandlestickSeriesBlueprint
 * @typedef {LineSeriesBlueprint & FetchedAnySeriesOptions} FetchedLineSeriesBlueprint
 * @typedef {HistogramSeriesBlueprint & FetchedAnySeriesOptions} FetchedHistogramSeriesBlueprint
 * @typedef {AnySeriesBlueprint & FetchedAnySeriesOptions} AnyFetchedSeriesBlueprint
 *
 * @typedef {Object} PartialOption
 * @property {string} name
 *
 * @typedef {Object} ProcessedOptionAddons
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
 * @property {string} title
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
 * @typedef {(AnyPartialOption | PartialOptionsGroup)[]} PartialOptionsTree
 *
 * @typedef {Object} PartialOptionsGroup
 * @property {string} name
 * @property {PartialOptionsTree} tree
 *
 * @typedef {Object} OptionsGroup
 * @property {string} name
 * @property {OptionsTree} tree
 *
 * @typedef {(Option | OptionsGroup)[]} OptionsTree
 *
 */

/**
 * @param {Object} args
 * @param {Env} args.env
 * @param {Colors} args.colors
 * @param {VecIdToIndexes} args.vecIdToIndexes
 * @param {Pools} args.pools
 * @returns {PartialOptionsTree}
 */
function createPartialOptions({ env, colors, vecIdToIndexes, pools }) {
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
   * @typedef {"_30d_delta"} _30DChageSubString
   * @typedef {StartsWith<CumulativePrefix>} CumulativeVecId
   * @typedef {ExcludeSubstring<WithoutPrefix<CumulativeVecId, CumulativePrefix>, _30DChageSubString>} CumulativeVecIdBase
   * @typedef {"_avg"} AverageSuffix
   * @typedef {EndsWith<AverageSuffix>} VecIdAverage
   * @typedef {WithoutSuffix<VecIdAverage, AverageSuffix>} VecIdAverageBase
   * @typedef {"_median"} MedianSuffix
   * @typedef {EndsWith<MedianSuffix>} VecIdMedian
   * @typedef {WithoutSuffix<VecIdMedian, MedianSuffix>} VecIdMedianBase
   * @typedef {"_pct90"} _pct90Suffix
   * @typedef {EndsWith<_pct90Suffix>} VecIdpct90
   * @typedef {WithoutSuffix<VecIdpct90, _pct90Suffix>} VecIdpct90Base
   * @typedef {"_pct75"} _pct75Suffix
   * @typedef {EndsWith<_pct75Suffix>} VecIdpct75
   * @typedef {WithoutSuffix<VecIdpct75, _pct75Suffix>} VecIdpct75Base
   * @typedef {"_pct25"} _pct25Suffix
   * @typedef {EndsWith<_pct25Suffix>} VecIdpct25
   * @typedef {WithoutSuffix<VecIdpct25, _pct25Suffix>} VecIdpct25Base
   * @typedef {"_pct10"} _pct10Suffix
   * @typedef {EndsWith<_pct10Suffix>} VecIdpct10
   * @typedef {WithoutSuffix<VecIdpct10, _pct10Suffix>} VecIdpct10Base
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

  const cohortAll = /** @type {const} */ ({
    key: "",
    name: "",
    title: "",
    color: colors.orange,
  });
  const cohortAllForComparaison = /** @type {const} */ ({
    key: "",
    name: "all",
    title: "",
    color: colors.default,
  });

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
      key: "utxos_up_to_1d_old",
      name: "1d",
      title: "UTXOs Up to 1 Day old",
      color: colors.pink,
    },
    {
      key: "utxos_up_to_1w_old",
      name: "1w",
      title: "UTXOs Up to 1 Week old",
      color: colors.red,
    },
    {
      key: "utxos_up_to_1m_old",
      name: "1m",
      title: "UTXOs Up to 1 Month old",
      color: colors.orange,
    },
    {
      key: "utxos_up_to_2m_old",
      name: "2m",
      title: "UTXOs Up to 2 Months old",
      color: colors.amber,
    },
    {
      key: "utxos_up_to_3m_old",
      name: "3m",
      title: "UTXOs Up to 3 Months old",
      color: colors.yellow,
    },
    {
      key: "utxos_up_to_4m_old",
      name: "4m",
      title: "UTXOs Up to 4 Months old",
      color: colors.lime,
    },
    {
      key: "utxos_up_to_5m_old",
      name: "5m",
      title: "UTXOs Up to 5 Months old",
      color: colors.green,
    },
    {
      key: "utxos_up_to_6m_old",
      name: "6m",
      title: "UTXOs Up to 6 Months old",
      color: colors.teal,
    },
    {
      key: "utxos_up_to_1y_old",
      name: "1y",
      title: "UTXOs Up to 1 Year old",
      color: colors.sky,
    },
    {
      key: "utxos_up_to_2y_old",
      name: "2y",
      title: "UTXOs Up to 2 Years old",
      color: colors.indigo,
    },
    {
      key: "utxos_up_to_3y_old",
      name: "3y",
      title: "UTXOs Up to 3 Years old",
      color: colors.violet,
    },
    {
      key: "utxos_up_to_4y_old",
      name: "4y",
      title: "UTXOs Up to 4 Years old",
      color: colors.purple,
    },
    {
      key: "utxos_up_to_5y_old",
      name: "5y",
      title: "UTXOs Up to 5 Years old",
      color: colors.fuchsia,
    },
    {
      key: "utxos_up_to_6y_old",
      name: "6y",
      title: "UTXOs Up to 6 Years old",
      color: colors.pink,
    },
    {
      key: "utxos_up_to_7y_old",
      name: "7y",
      title: "UTXOs Up to 7 Years old",
      color: colors.red,
    },
    {
      key: "utxos_up_to_8y_old",
      name: "8y",
      title: "UTXOs Up to 8 Years old",
      color: colors.orange,
    },
    {
      key: "utxos_up_to_10y_old",
      name: "10y",
      title: "UTXOs Up to 10 Years old",
      color: colors.amber,
    },
    {
      key: "utxos_up_to_12y_old",
      name: "12y",
      title: "UTXOs Up to 12 Years old",
      color: colors.yellow,
    },
    {
      key: "utxos_up_to_15y_old",
      name: "15y",
      title: "UTXOs Up to 15 Years old",
      color: colors.lime,
    },
  ]);

  const fromDate = /** @type {const} */ ([
    {
      key: "utxos_at_least_1d_old",
      name: "1d",
      title: "UTXOs at least 1 Day old",
      color: colors.red,
    },
    {
      key: "utxos_at_least_1w_old",
      name: "1w",
      title: "UTXOs at least 1 Week old",
      color: colors.orange,
    },
    {
      key: "utxos_at_least_1m_old",
      name: "1m",
      title: "UTXOs at least 1 Month old",
      color: colors.yellow,
    },
    {
      key: "utxos_at_least_2m_old",
      name: "2m",
      title: "UTXOs at least 2 Months old",
      color: colors.lime,
    },
    {
      key: "utxos_at_least_3m_old",
      name: "3m",
      title: "UTXOs at least 3 Months old",
      color: colors.green,
    },
    {
      key: "utxos_at_least_4m_old",
      name: "4m",
      title: "UTXOs at least 4 Months old",
      color: colors.teal,
    },
    {
      key: "utxos_at_least_5m_old",
      name: "5m",
      title: "UTXOs at least 5 Months old",
      color: colors.cyan,
    },
    {
      key: "utxos_at_least_6m_old",
      name: "6m",
      title: "UTXOs at least 6 Months old",
      color: colors.blue,
    },
    {
      key: "utxos_at_least_1y_old",
      name: "1y",
      title: "UTXOs at least 1 Year old",
      color: colors.indigo,
    },
    {
      key: "utxos_at_least_2y_old",
      name: "2y",
      title: "UTXOs at least 2 Years old",
      color: colors.violet,
    },
    {
      key: "utxos_at_least_3y_old",
      name: "3y",
      title: "UTXOs at least 3 Years old",
      color: colors.purple,
    },
    {
      key: "utxos_at_least_4y_old",
      name: "4y",
      title: "UTXOs at least 4 Years old",
      color: colors.fuchsia,
    },
    {
      key: "utxos_at_least_5y_old",
      name: "5y",
      title: "UTXOs at least 5 Years old",
      color: colors.pink,
    },
    {
      key: "utxos_at_least_6y_old",
      name: "6y",
      title: "UTXOs at least 6 Years old",
      color: colors.rose,
    },
    {
      key: "utxos_at_least_7y_old",
      name: "7y",
      title: "UTXOs at least 7 Years old",
      color: colors.red,
    },
    {
      key: "utxos_at_least_8y_old",
      name: "8y",
      title: "UTXOs at least 8 Years old",
      color: colors.orange,
    },
    {
      key: "utxos_at_least_10y_old",
      name: "10y",
      title: "UTXOs at least 10 Years old",
      color: colors.yellow,
    },
    {
      key: "utxos_at_least_12y_old",
      name: "12y",
      title: "UTXOs at least 12 Years old",
      color: colors.lime,
    },
    {
      key: "utxos_at_least_15y_old",
      name: "15y",
      title: "UTXOs at least 15 Years old",
      color: colors.green,
    },
  ]);

  const dateRange = /** @type {const} */ ([
    {
      key: "utxos_up_to_1d_old",
      name: "1d",
      title: "UTXOs New Up to 1 Day old",
      color: colors.pink,
    },
    {
      key: "utxos_at_least_1d_up_to_1w_old",
      name: "1d..1w",
      title: "UTXOs At least 1 Day ago Up to 1 Week old",
      color: colors.red,
    },
    {
      key: "utxos_at_least_1w_up_to_1m_old",
      name: "1w..1m",
      title: "UTXOs At least 1 Week ago Up to 1 Month old",
      color: colors.orange,
    },
    {
      key: "utxos_at_least_1m_up_to_2m_old",
      name: "1m..2m",
      title: "UTXOs At least 1 Month ago Up to 2 Months old",
      color: colors.yellow,
    },
    {
      key: "utxos_at_least_2m_up_to_3m_old",
      name: "2m..3m",
      title: "UTXOs At least 2 Month ago Up to 3 Months old",
      color: colors.yellow,
    },
    {
      key: "utxos_at_least_3m_up_to_4m_old",
      name: "3m..4m",
      title: "UTXOs At least 3 Month ago Up to 4 Months old",
      color: colors.lime,
    },
    {
      key: "utxos_at_least_4m_up_to_5m_old",
      name: "4m..5m",
      title: "UTXOs At least 4 Month ago Up to 5 Months old",
      color: colors.lime,
    },
    {
      key: "utxos_at_least_5m_up_to_6m_old",
      name: "5m..6m",
      title: "UTXOs At least 5 Month ago Up to 6 Months old",
      color: colors.lime,
    },
    {
      key: "utxos_at_least_6m_up_to_1y_old",
      name: "6m..1y",
      title: "UTXOs At least 6 Months ago Up to 1 Year old",
      color: colors.green,
    },
    {
      key: "utxos_at_least_1y_up_to_2y_old",
      name: "1y..2y",
      title: "UTXOs At least 1 Year ago Up to 2 Years old",
      color: colors.cyan,
    },
    {
      key: "utxos_at_least_2y_up_to_3y_old",
      name: "2y..3y",
      title: "UTXOs At least 2 Years ago Up to 3 Years old",
      color: colors.blue,
    },
    {
      key: "utxos_at_least_3y_up_to_4y_old",
      name: "3y..4y",
      title: "UTXOs At least 3 Years ago Up to 4 Years old",
      color: colors.indigo,
    },
    {
      key: "utxos_at_least_4y_up_to_5y_old",
      name: "4y..5y",
      title: "UTXOs At least 4 Years ago Up to 5 Years old",
      color: colors.violet,
    },
    {
      key: "utxos_at_least_5y_up_to_6y_old",
      name: "5y..6y",
      title: "UTXOs At least 5 Years ago Up to 6 Years old",
      color: colors.purple,
    },
    {
      key: "utxos_at_least_6y_up_to_7y_old",
      name: "6y..7y",
      title: "UTXOs At least 6 Years ago Up to 7 Years old",
      color: colors.purple,
    },
    {
      key: "utxos_at_least_7y_up_to_8y_old",
      name: "7y..8y",
      title: "UTXOs At least 7 Years ago Up to 8 Years old",
      color: colors.fuchsia,
    },
    {
      key: "utxos_at_least_8y_up_to_10y_old",
      name: "8y..10y",
      title: "UTXOs At least 8 Years ago Up to 10 Years old",
      color: colors.fuchsia,
    },
    {
      key: "utxos_at_least_10y_up_to_12y_old",
      name: "10y..12y",
      title: "UTXOs At least 10 Years ago Up to 12 Years old",
      color: colors.pink,
    },
    {
      key: "utxos_at_least_12y_up_to_15y_old",
      name: "12y..15y",
      title: "UTXOs At least 12 Years ago Up to 15 Years old",
      color: colors.red,
    },
    {
      key: "utxos_at_least_15y_old",
      name: "15y+",
      title: "UTXOs At least 15 Years old up to genesis",
      color: colors.orange,
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

  const aboveAmount = /** @type {const} */ ([
    {
      key: "above_1sat",
      name: ">=1 sat",
      title: "Above 1 sat",
      color: colors.orange,
    },
    {
      key: "above_10sats",
      name: ">=10 sats",
      title: "Above 10 sats",
      color: colors.orange,
    },
    {
      key: "above_100sats",
      name: ">=100 sats",
      title: "Above 100 sats",
      color: colors.yellow,
    },
    {
      key: "above_1k_sats",
      name: ">=1K sats",
      title: "Above 1K sats",
      color: colors.lime,
    },
    {
      key: "above_10k_sats",
      name: ">=10K sats",
      title: "Above 10K sats",
      color: colors.green,
    },
    {
      key: "above_100k_sats",
      name: ">=100K sats",
      title: "Above 100K sats",
      color: colors.cyan,
    },
    {
      key: "above_1m_sats",
      name: ">=1M sats",
      title: "Above 1M sats",
      color: colors.blue,
    },
    {
      key: "above_10m_sats",
      name: ">=10M sats",
      title: "Above 10M sats",
      color: colors.indigo,
    },
    {
      key: "above_1btc",
      name: ">=1 btc",
      title: "Above 1 BTC",
      color: colors.purple,
    },
    {
      key: "above_10btc",
      name: ">=10 btc",
      title: "Above 10 BTC",
      color: colors.violet,
    },
    {
      key: "above_100btc",
      name: ">=100 btc",
      title: "Above 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "above_1k_btc",
      name: ">=1K btc",
      title: "Above 1K BTC",
      color: colors.pink,
    },
    {
      key: "above_10k_btc",
      name: ">=10K btc",
      title: "Above 10K BTC",
      color: colors.red,
    },
  ]);

  const utxosAboveAmount = aboveAmount.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `utxos_${key}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesAboveAmount = aboveAmount.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `addrs_${key}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

  const underAmount = /** @type {const} */ ([
    {
      key: "under_10sats",
      name: "<10 sats",
      title: "under 10 sats",
      color: colors.orange,
    },
    {
      key: "under_100sats",
      name: "<100 sats",
      title: "under 100 sats",
      color: colors.yellow,
    },
    {
      key: "under_1k_sats",
      name: "<1k sats",
      title: "under 1k sats",
      color: colors.lime,
    },
    {
      key: "under_10k_sats",
      name: "<10k sats",
      title: "under 10k sats",
      color: colors.green,
    },
    {
      key: "under_100k_sats",
      name: "<100k sats",
      title: "under 100k sats",
      color: colors.cyan,
    },
    {
      key: "under_1m_sats",
      name: "<1m sats",
      title: "under 1m sats",
      color: colors.blue,
    },
    {
      key: "under_10m_sats",
      name: "<10m sats",
      title: "under 10m sats",
      color: colors.indigo,
    },
    {
      key: "under_1btc",
      name: "<1 btc",
      title: "under 1 btc",
      color: colors.purple,
    },
    {
      key: "under_10btc",
      name: "<10 btc",
      title: "under 10 btc",
      color: colors.violet,
    },
    {
      key: "under_100btc",
      name: "<100 btc",
      title: "under 100 btc",
      color: colors.fuchsia,
    },
    {
      key: "under_1k_btc",
      name: "<1k btc",
      title: "under 1k btc",
      color: colors.pink,
    },
    {
      key: "under_10k_btc",
      name: "<10k btc",
      title: "under 10k btc",
      color: colors.red,
    },
    {
      key: "under_100k_btc",
      name: "<100k btc",
      title: "under 100k btc",
      color: colors.orange,
    },
  ]);

  const utxosUnderAmount = underAmount.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `utxos_${key}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesUnderAmount = underAmount.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `addrs_${key}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

  const amountRanges = /** @type {const} */ ([
    {
      key: "with_0sats",
      name: "0 sats",
      title: "valued 0 sats",
      color: colors.red,
    },
    {
      key: "above_1sat_under_10sats",
      name: "1 sat..10 sats",
      title: "Above 1 sat Under 10 sats",
      color: colors.orange,
    },
    {
      key: "above_10sats_under_100sats",
      name: "10 sats..100 sats",
      title: "Above 10 sats Under 100 sats",
      color: colors.yellow,
    },
    {
      key: "above_100sats_under_1k_sats",
      name: "100 sats..1K sats",
      title: "Above 100 sats Under 1K sats",
      color: colors.lime,
    },
    {
      key: "above_1k_sats_under_10k_sats",
      name: "1K sats..10K sats",
      title: "Above 1K sats Under 10K sats",
      color: colors.green,
    },
    {
      key: "above_10k_sats_under_100k_sats",
      name: "10K sats..100K sats",
      title: "Above 10K sats Under 100K sats",
      color: colors.cyan,
    },
    {
      key: "above_100k_sats_under_1m_sats",
      name: "100K sats .. 1M sats",
      title: "Above 100K sats Under 1M sats",
      color: colors.blue,
    },
    {
      key: "above_1m_sats_under_10m_sats",
      name: "1M sats..10M sats",
      title: "Above 1M sats Under 10M sats",
      color: colors.indigo,
    },
    {
      key: "above_10m_sats_under_1btc",
      name: "10M sats..1 btc",
      title: "Above 10M sats Under 1 BTC",
      color: colors.purple,
    },
    {
      key: "above_1btc_under_10btc",
      name: "1 btc..10 btc",
      title: "Above 1 BTC Under 10 BTC",
      color: colors.violet,
    },
    {
      key: "above_10btc_under_100btc",
      name: "10 btc..100 btc",
      title: "Above 10 BTC Under 100 BTC",
      color: colors.fuchsia,
    },
    {
      key: "above_100btc_under_1k_btc",
      name: "100 btc..1K btc",
      title: "Above 100 BTC Under 1K BTC",
      color: colors.pink,
    },
    {
      key: "above_1k_btc_under_10k_btc",
      name: "1K btc..10K btc",
      title: "Above 1K BTC Under 10K BTC",
      color: colors.red,
    },
    {
      key: "above_10k_btc_under_100k_btc",
      name: "10K btc..100K btc",
      title: "Above 10K BTC Under 100K BTC",
      color: colors.orange,
    },
    {
      key: "above_100k_btc",
      name: "100K+ btc",
      title: "Above 100K BTC",
      color: colors.yellow,
    },
  ]);

  const utxosAmountRanges = amountRanges.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `utxos_${key}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesAmountRanges = amountRanges.map(
    ({ key, name, title, color }) =>
      /** @type {const} */ ({
        key: `addrs_${key}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

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
      key: "p2ms_outputs",
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
      key: "unknown_outputs",
      name: "unknown",
      title: "Pay To Unknown",
      color: colors.violet,
    },
    {
      key: "empty_outputs",
      name: "empty",
      title: "Pay To Empty",
      color: colors.fuchsia,
    },
  ]);

  const cointimePrices = /** @type {const} */ ([
    {
      key: `true_market_mean`,
      name: "True market mean",
      title: "True market mean",
      color: colors.blue,
    },
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
      key: `cointime_price`,
      name: "cointime",
      title: "Cointime Price",
      color: colors.yellow,
    },
  ]);

  const cointimeCapitalizations = /** @type {const} */ ([
    {
      key: `vaulted_cap`,
      name: "Vaulted",
      title: "Vaulted Capitalization",
      color: colors.lime,
    },
    {
      key: `active_cap`,
      name: "Active",
      title: "Active Capitalization",
      color: colors.rose,
    },
    {
      key: `cointime_cap`,
      name: "Cointime",
      title: "Cointime Capitalization",
      color: colors.yellow,
    },
    {
      key: `investor_cap`,
      name: "Investor",
      title: "Investor Capitalization",
      color: colors.fuchsia,
    },
    {
      key: `thermo_cap`,
      name: "Thermo",
      title: "Thermo Capitalization",
      color: colors.emerald,
    },
  ]);

  /**
   * @param {Object} args
   * @param {number} [args.number]
   * @param {string} [args.name]
   * @param {boolean} [args.defaultActive]
   * @param {number} [args.lineStyle]
   * @param {Color} [args.color]
   * @param {Unit} args.unit
   */
  function createPriceLine({
    number = 0,
    unit,
    defaultActive,
    color,
    name,
    lineStyle,
  }) {
    return /** @satisfies {FetchedLineSeriesBlueprint} */ ({
      key: `constant_${number >= 0 ? number : `minus_${Math.abs(number)}`}`.replace(
        ".",
        "_",
      ),
      title: name ?? `${number}`,
      unit,
      defaultActive,
      color: color ?? colors.gray,
      options: {
        lineStyle: lineStyle ?? 4,
        lastValueVisible: false,
        crosshairMarkerVisible: false,
      },
    });
  }

  /**
   * @param {Object} args
   * @param {number[]} args.numbers
   * @param {boolean} [args.defaultActive]
   * @param {Unit} args.unit
   */
  function createPriceLines({ numbers, unit }) {
    return numbers.map(
      (number) =>
        /** @satisfies {FetchedLineSeriesBlueprint} */ ({
          key: `constant_${number >= 0 ? number : `minus_${Math.abs(number)}`}`,
          title: `${number}`,
          unit,
          defaultActive: !number,
          color: colors.gray,
          options: {
            lineStyle: 4,
            lastValueVisible: false,
            crosshairMarkerVisible: false,
          },
        }),
    );
  }

  /**
   * @param {Object} args
   * @param {VecId} args.key
   * @param {string} args.name
   * @param {Color} [args.color]
   * @param {Unit} [args.unit]
   * @param {boolean} [args.defaultActive]
   * @param {LineSeriesPartialOptions} [args.options]
   */
  function createBaseSeries({
    key,
    name,
    color,
    defaultActive,
    unit,
    options,
  }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key,
      title: name,
      color,
      unit,
      defaultActive,
      options,
    });
  }

  /**
   * @param {Object} args
   * @param {VecIdAverageBase} args.concat
   * @param {string} [args.title]
   */
  function createAverageSeries({ concat, title = "" }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key: `${concat}_avg`,
      title: `Average ${title}`,
    });
  }

  /**
   * @param {Object} args
   * @param {CumulativeVecIdBase} args.concat
   * @param {Color} [args.sumColor]
   * @param {Color} [args.cumulativeColor]
   * @param {string} [args.common]
   */
  function createSumCumulativeSeries({
    concat,
    common,
    sumColor,
    cumulativeColor,
  }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      createSumSeries({
        key: concat,
        title: common,
        color: sumColor,
      }),
      createCumulativeSeries({
        concat,
        title: common,
        color: cumulativeColor,
      }),
    ]);
  }

  /**
   * @param {Object} args
   * @param {CumulativeVecIdBase} args.key
   * @param {string} [args.title]
   * @param {Color} [args.color]
   */
  function createSumSeries({ key, title = "", color }) {
    const sumKey = `${key}_sum`;
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key: `${key}_sum` in vecIdToIndexes ? sumKey : key,
      title: `Sum ${title}`,
      color: color ?? colors.red,
    });
  }

  /**
   * @param {Object} args
   * @param {CumulativeVecIdBase} args.concat
   * @param {string} [args.title]
   * @param {Color} [args.color]
   */
  function createCumulativeSeries({ concat, title = "", color }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      key: `${concat}_cumulative`,
      title: `Cumulative ${title}`,
      color: color ?? colors.cyan,
      defaultActive: false,
    });
  }

  /**
   * @param {Object} args
   * @param {VecIdMinBase & VecIdMaxBase & VecIdpct90Base & VecIdpct75Base & VecIdMedianBase & VecIdpct25Base & VecIdpct10Base} args.concat
   * @param {string} [args.title]
   */
  function createMinMaxPercentilesSeries({ concat, title = "" }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        key: `${concat}_max`,
        title: `Max ${title}`,
        color: colors.pink,
        defaultActive: false,
      },
      {
        key: `${concat}_min`,
        title: `Min ${title}`,
        color: colors.green,
        defaultActive: false,
      },
      {
        key: `${concat}_median`,
        title: `Median ${title}`,
        color: colors.amber,
        defaultActive: false,
      },
      {
        key: `${concat}_pct75`,
        title: `pct75 ${title}`,
        color: colors.red,
        defaultActive: false,
      },
      {
        key: `${concat}_pct25`,
        title: `pct25 ${title}`,
        color: colors.yellow,
        defaultActive: false,
      },
      {
        key: `${concat}_pct90`,
        title: `pct90 ${title}`,
        color: colors.rose,
        defaultActive: false,
      },
      {
        key: `${concat}_pct10`,
        title: `pct10 ${title}`,
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * @param {VecIdAverageBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecIdpct90Base & VecIdpct75Base & VecIdMedianBase & VecIdpct25Base & VecIdpct10Base} key
   */
  function createSumCumulativeMinMaxPercentilesSeries(key) {
    return [
      ...createSumCumulativeSeries({ concat: key }),
      ...createMinMaxPercentilesSeries({ concat: key }),
    ];
  }

  /**
   * @param {VecIdAverageBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecIdpct90Base & VecIdpct75Base & VecIdMedianBase & VecIdpct25Base & VecIdpct10Base} key
   */
  function createAverageSumCumulativeMinMaxPercentilesSeries(key) {
    return [
      createAverageSeries({ concat: key }),
      ...createSumCumulativeMinMaxPercentilesSeries(key),
    ];
  }

  /**
   * @param {Object} args
   * @param {VecId & VecIdAverageBase & CumulativeVecIdBase & VecIdMinBase & VecIdMaxBase & VecIdpct90Base & VecIdpct75Base & VecIdMedianBase & VecIdpct25Base & VecIdpct10Base} args.key
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
  function createPriceWithRatioOptions({ name, title, legend, key, color }) {
    const percentiles = [
      {
        name: "pct1",
        color: colors.indigo,
      },
      {
        name: "pct2",
        color: colors.violet,
      },
      {
        name: "pct5",
        color: colors.purple,
      },
      {
        name: "pct95",
        color: colors.amber,
      },
      {
        name: "pct98",
        color: colors.orange,
      },
      {
        name: "pct99",
        color: colors.red,
      },
    ];

    const ratioAverages = /** @type {const} */ ([
      {
        name: "1w sma",
        key: "1w_sma",
        color: colors.lime,
      },
      {
        name: "1m sma",
        key: "1m_sma",
        color: colors.teal,
      },
      {
        name: "1y sma",
        key: "1y_sma",
        color: colors.sky,
      },
      {
        name: "2y sma",
        key: "2y_sma",
        color: colors.indigo,
      },
      {
        name: "4y sma",
        key: "4y_sma",
        color: colors.purple,
      },
      {
        name: "all sma",
        key: "sma",
        color: colors.rose,
      },
    ]);

    return [
      {
        name: "price",
        title,
        top: [
          createBaseSeries({
            key,
            name: legend,
            color,
          }),
        ],
      },
      {
        name: "Ratio",
        title: `${title} Ratio`,
        top: [
          createBaseSeries({
            key,
            name: legend,
            color,
          }),
          ...(`${key}_ratio_p1sd_in_usd` in vecIdToIndexes
            ? percentiles.map(({ name, color }) =>
                createBaseSeries({
                  key: `${key}_ratio_${name}_in_usd`,
                  name,
                  color,
                  defaultActive: false,
                  options: {
                    lineStyle: 1,
                  },
                }),
              )
            : []),
        ],
        bottom: [
          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
            key: `${key}_ratio`,
            title: "Ratio",
            type: "Baseline",
            options: {
              baseValue: { price: 1 },
            },
          }),
          ...(`${key}_ratio_p1sd` in vecIdToIndexes
            ? percentiles.map(({ name, color }) =>
                createBaseSeries({
                  key: `${key}_ratio_${name}`,
                  name,
                  color,
                  defaultActive: false,
                  options: {
                    lineStyle: 1,
                  },
                }),
              )
            : []),
          ...(`${key}_ratio_sma` in vecIdToIndexes
            ? ratioAverages.map(({ name, key: keyAddon, color }) =>
                createBaseSeries({
                  key: `${key}_ratio_${keyAddon}`,
                  name,
                  color,
                  defaultActive: false,
                  options: {
                    lineStyle: 1,
                  },
                }),
              )
            : []),
          createPriceLine({
            number: 1,
            unit: "Ratio",
          }),
        ],
      },
      ...(`${key}_ratio_zscore` in vecIdToIndexes
        ? [
            {
              name: "ZScores",
              tree: [
                {
                  name: "compare",
                  title: `Compare ${title} ZScores`,
                  top: [
                    createBaseSeries({
                      key,
                      name: legend,
                      color,
                    }),
                    createBaseSeries({
                      key: `${key}_ratio_1y_0sd_in_usd`,
                      name: "1y 0sd",
                      color: colors.fuchsia,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: `${key}_ratio_2y_0sd_in_usd`,
                      name: "2y 0sd",
                      color: colors.purple,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: `${key}_ratio_4y_0sd_in_usd`,
                      name: "4y 0sd",
                      color: colors.violet,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: `${key}_ratio_0sd_in_usd`,
                      name: "0sd",
                      color: colors.indigo,
                      defaultActive: false,
                    }),
                  ],
                  bottom: [
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      key: `${key}_ratio_zscore`,
                      title: "All",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      key: `${key}_ratio_4y_zscore`,
                      colors: [colors.lime, colors.rose],
                      title: "4y",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      key: `${key}_ratio_2y_zscore`,
                      colors: [colors.avocado, colors.pink],
                      title: "2y",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      key: `${key}_ratio_1y_zscore`,
                      colors: [colors.yellow, colors.fuchsia],
                      title: "1Y",
                      type: "Baseline",
                    }),
                    ...createPriceLines({
                      numbers: [0, 1, 2, 3, 4, -1, -2, -3, -4],
                      unit: "Ratio",
                    }),
                  ],
                },
                ...[
                  {
                    nameAddon: "all",
                    titleAddon: "",
                    keyAddon: "",
                  },
                  {
                    nameAddon: "4y",
                    titleAddon: "4y",
                    keyAddon: "4y_",
                  },
                  {
                    nameAddon: "2y",
                    titleAddon: "2y",
                    keyAddon: "2y_",
                  },
                  {
                    nameAddon: "1y",
                    titleAddon: "1y",
                    keyAddon: "1y_",
                  },
                ].flatMap(({ nameAddon, titleAddon, keyAddon }) => ({
                  name: nameAddon,
                  title: `${title} ${titleAddon} ZScore`,
                  top: [
                    createBaseSeries({
                      key,
                      name: legend,
                      color,
                    }),
                    ...[
                      { sdKey: "0sd", name: "0σ", color: colors.lime },
                      {
                        sdKey: `p0_5sd`,
                        name: "+0.5σ",
                        color: colors.yellow,
                      },
                      {
                        sdKey: `p1sd`,
                        name: "+1σ",
                        color: colors.amber,
                      },
                      {
                        sdKey: `p1_5sd`,
                        name: "+1.5σ",
                        color: colors.orange,
                      },
                      {
                        sdKey: `p2sd`,
                        name: "+2σ",
                        color: colors.red,
                      },
                      {
                        sdKey: `p2_5sd`,
                        name: "+2.5σ",
                        color: colors.rose,
                      },
                      {
                        sdKey: `p3sd`,
                        name: "+3σ",
                        color: colors.pink,
                      },
                      {
                        sdKey: `m0_5sd`,
                        name: "−0.5σ",
                        color: colors.teal,
                      },
                      {
                        sdKey: `m1sd`,
                        name: "−1σ",
                        color: colors.cyan,
                      },
                      {
                        sdKey: `m1_5sd`,
                        name: "−1.5σ",
                        color: colors.sky,
                      },
                      {
                        sdKey: `m2sd`,
                        name: "−2σ",
                        color: colors.blue,
                      },
                      {
                        sdKey: `m2_5sd`,
                        name: "−2.5σ",
                        color: colors.indigo,
                      },
                      {
                        sdKey: `m3sd`,
                        name: "−3σ",
                        color: colors.violet,
                      },
                    ].map(({ sdKey, name, color }) =>
                      createBaseSeries({
                        key: `${key}_ratio_${keyAddon}${sdKey}_in_usd`,
                        name,
                        color,
                        defaultActive: false,
                        options: {
                          lineStyle: 1,
                        },
                      }),
                    ),
                  ],
                  bottom: [
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      key: `${key}_ratio_${keyAddon}zscore`,
                      title: "score",
                      type: "Baseline",
                    }),
                    ...createPriceLines({
                      numbers: [0, 1, 2, 3, 4, -1, -2, -3, -4],
                      unit: "Ratio",
                    }),
                  ],
                })),
              ],
            },
          ]
        : []),
    ];
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

    const title = args.title
      ? `${useGroupName ? "by" : "of"} ${args.title}`
      : "";

    return /** @satisfies {PartialOptionsGroup} */ ({
      name: args.name || "all",
      tree: [
        !("list" in args)
          ? {
              name: "supply",
              title: `Supply ${title}`,
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
                          key: `${key}supply_rel_to_circulating_supply`,
                          name: "Supply",
                          color: colors.default,
                        }),
                      ]
                    : []),
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
                    key: `${key}supply_breakeven`,
                    name: useGroupName ? name : "breakeven",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}supply_breakeven_in_btc`,
                    name: useGroupName ? name : "breakeven",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}supply_breakeven_in_usd`,
                    name: useGroupName ? name : "breakeven",
                    color: colors.yellow,
                  }),
                  createBaseSeries({
                    key: `${key}supply_half`,
                    name: "half",
                    color: colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    key: `${key}supply_half_in_btc`,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    key: `${key}supply_half_in_usd`,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  ...(key
                    ? [
                        createBaseSeries({
                          key: `${key}supply_in_profit_rel_to_circulating_supply`,
                          name: "In Profit",
                          color: colors.green,
                        }),
                        createBaseSeries({
                          key: `${key}supply_in_loss_rel_to_circulating_supply`,
                          name: "In Loss",
                          color: colors.red,
                        }),
                        createBaseSeries({
                          key: `${key}supply_breakeven_rel_to_circulating_supply`,
                          name: "breakeven",
                          color: colors.yellow,
                        }),
                      ]
                    : []),
                  createBaseSeries({
                    key: `${key}supply_in_profit_rel_to_own_supply`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    key: `${key}supply_in_loss_rel_to_own_supply`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    key: `${key}supply_breakeven_rel_to_own_supply`,
                    name: "breakeven",
                    color: colors.yellow,
                  }),
                  createPriceLine({
                    unit: "%self",
                    number: 100,
                    lineStyle: 0,
                    color: colors.default,
                  }),
                  createPriceLine({
                    unit: "%self",
                    number: 50,
                  }),
                ]);
              }),
            }
          : {
              name: "supply",
              tree: [
                {
                  name: "total",
                  title: `Supply ${title}`,
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
                      key
                        ? createBaseSeries({
                            key: `${key}supply_rel_to_circulating_supply`,
                            name,
                            color,
                          })
                        : createBaseSeries({
                            unit: "%all",
                            key: "constant_100",
                            name,
                            color,
                          }),
                    ]);
                  }),
                },
                {
                  name: "in profit",
                  title: `Supply In Profit ${title}`,
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
                              key: `${key}supply_in_profit_rel_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                    ]);
                  }),
                },
                {
                  name: "in loss",
                  title: `Supply In loss ${title}`,
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
                              key: `${key}supply_in_loss_rel_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                    ]);
                  }),
                },
                {
                  name: "breakeven",
                  title: `Supply At Breaken ${title}`,
                  bottom: list.flatMap(({ color, name, key: _key }) => {
                    const key = fixKey(_key);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        key: `${key}supply_breakeven`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_breakeven_in_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        key: `${key}supply_breakeven_in_usd`,
                        name,
                        color,
                      }),
                      ...(key
                        ? [
                            createBaseSeries({
                              key: `${key}supply_breakeven_rel_to_circulating_supply`,
                              name,
                              color,
                            }),
                          ]
                        : []),
                      createBaseSeries({
                        key: `${key}supply_breakeven_rel_to_own_supply`,
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
          title: `UTXO Count ${title}`,
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
          ({ key }) => `${fixKey(key)}addr_count` in vecIdToIndexes,
        ).length > ("list" in args ? 1 : 0)
          ? !("list" in args) ||
            list.filter(
              ({ key }) => `${fixKey(key)}empty_addr_count` in vecIdToIndexes,
            ).length <= 1
            ? [
                {
                  name: "address count",
                  title: `Address Count ${title}`,
                  bottom: list.flatMap(({ name, color, key: _key }) => {
                    const key = fixKey(_key);
                    return [
                      ...(`${key}addr_count` in vecIdToIndexes
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}addr_count`,
                              name: useGroupName ? name : "Loaded",
                              color: useGroupName ? color : colors.orange,
                            }),
                          ])
                        : []),
                      ...(`${key}empty_addr_count` in vecIdToIndexes
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}empty_addr_count`,
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
                      title: `Loaded Address Count ${title}`,
                      bottom: list
                        .filter(
                          ({ key }) =>
                            `${fixKey(key)}addr_count` in vecIdToIndexes,
                        )
                        .flatMap(({ name, color, key: _key }) => {
                          const key = fixKey(_key);
                          return [
                            createBaseSeries({
                              key: `${key}addr_count`,
                              name,
                              color,
                            }),
                          ];
                        }),
                    },
                    ...(list.filter(
                      ({ key }) =>
                        `${fixKey(key)}empty_addr_count` in vecIdToIndexes,
                    ).length
                      ? [
                          {
                            name: "empty",
                            title: `Empty Address Count ${title}`,
                            bottom: list
                              .filter(
                                ({ key }) =>
                                  `${fixKey(key)}empty_addr_count` in
                                  vecIdToIndexes,
                              )
                              .flatMap(({ name, color, key: _key }) => {
                                const key = fixKey(_key);
                                return [
                                  createBaseSeries({
                                    key: `${key}empty_addr_count`,
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
        {
          name: "Realized",
          tree: [
            ...("list" in args
              ? [
                  {
                    name: "Price",
                    title: `Realized Price ${title}`,
                    top: list.map(({ color, name, key }) =>
                      createBaseSeries({
                        key: `${fixKey(key)}realized_price`,
                        name,
                        color,
                      }),
                    ),
                  },
                  {
                    name: "Ratio",
                    title: `Realized Price Ratio ${title}`,
                    bottom: [
                      ...list.map(({ color, name, key }) =>
                        createBaseSeries({
                          key: `${fixKey(key)}realized_price_ratio`,
                          name,
                          color,
                        }),
                      ),
                      createPriceLine({
                        unit: "Ratio",
                        number: 1,
                      }),
                    ],
                  },
                ]
              : createPriceWithRatioOptions({
                  title: `Realized Price ${title}`,
                  key: `${fixKey(args.key)}realized_price`,
                  name: "price",
                  legend: "realized",
                  color: args.color,
                })),
            {
              name: "capitalization",
              title: `Realized Capitalization ${title}`,
              bottom: list.flatMap(({ color, name, key: _key }) => {
                const key = fixKey(_key);
                return /** @type {const} */ ([
                  createBaseSeries({
                    key: `${key}realized_cap`,
                    name: useGroupName ? name : "Capitalization",
                    color,
                  }),
                  ...(!("list" in args)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${key}realized_cap_30d_delta`,
                          title: "30d change",
                          defaultActive: false,
                        }),
                        createPriceLine({
                          unit: "USD",
                          defaultActive: false,
                        }),
                      ]
                    : []),
                ]);
              }),
            },
            ...(!("list" in args)
              ? [
                  {
                    name: "pnl",
                    title: `Realized Profit And Loss ${title}`,
                    bottom: [
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized_profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized_loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}total_realized_pnl`,
                        name: "Total",
                        color: colors.default,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}neg_realized_loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized_profit_cumulative`,
                        name: "Cumulative Profit",
                        color: colors.green,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}realized_loss_cumulative`,
                        name: "Cumulative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}neg_realized_loss_cumulative`,
                        name: "Cumulative Negative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          args.key,
                        )}realized_profit_rel_to_realized_cap`,
                        title: "Profit",
                        color: colors.green,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          args.key,
                        )}realized_loss_rel_to_realized_cap`,
                        title: "Loss",
                        color: colors.red,
                      }),
                      createPriceLine({
                        unit: "%rcap",
                      }),
                      createPriceLine({
                        unit: "USD",
                        defaultActive: false,
                      }),
                    ],
                  },
                  {
                    name: "Net pnl",
                    title: `Net Realized Profit And Loss ${title}`,
                    bottom: list.flatMap(({ color, name, key }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}net_realized_pnl`,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(key)}net_realized_pnl_cumulative`,
                        title: "Cumulative",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_pnl_cumulative_30d_delta`,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_pnl_rel_to_realized_cap`,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        key: `${fixKey(
                          key,
                        )}net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`,
                        title: "cumulative 30d change",
                      }),
                      createPriceLine({
                        unit: "%mcap",
                      }),
                      createPriceLine({
                        unit: "%rcap",
                      }),
                      createPriceLine({
                        unit: "USD",
                      }),
                    ]),
                  },
                  {
                    name: "sopr",
                    title: `Spent Output Profit Ratio ${title}`,
                    bottom: list.flatMap(({ color, name, key }) => {
                      const soprKey = `${fixKey(key)}sopr`;
                      const asoprKey = `${fixKey(key)}adjusted_sopr`;
                      return [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: soprKey,
                          title: "normal",
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(asoprKey in vecIdToIndexes
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                key: asoprKey,
                                title: "adjusted",
                                colors: [colors.yellow, colors.fuchsia],
                                defaultActive: false,
                                options: {
                                  baseValue: {
                                    price: 1,
                                  },
                                },
                              }),
                            ]
                          : []),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${soprKey}_7d_ema`,
                          title: "7d ema",
                          colors: [colors.lime, colors.rose],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(asoprKey in vecIdToIndexes
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                key: `${asoprKey}_7d_ema`,
                                title: "adj. 7d ema",
                                colors: [colors.amber, colors.purple],
                                defaultActive: false,
                                options: {
                                  baseValue: {
                                    price: 1,
                                  },
                                },
                              }),
                            ]
                          : []),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${soprKey}_30d_ema`,
                          title: "30d ema",
                          colors: [colors.avocado, colors.pink],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(asoprKey in vecIdToIndexes
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                key: `${asoprKey}_30d_ema`,
                                title: "adj. 30d ema",
                                colors: [colors.orange, colors.violet],
                                defaultActive: false,
                                options: {
                                  baseValue: {
                                    price: 1,
                                  },
                                },
                              }),
                            ]
                          : []),
                        createPriceLine({
                          number: 1,
                          unit: "Ratio",
                        }),
                      ];
                    }),
                  },
                ]
              : [
                  {
                    name: "profit",
                    title: `Realized Profit ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, key: _key }) => {
                        const key = fixKey(_key);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            key: `${key}realized_profit`,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${key}realized_profit_rel_to_realized_cap`,
                            title: name,
                            color,
                          }),
                        ]);
                      }),
                      createPriceLine({
                        unit: "USD",
                      }),
                    ],
                  },
                  {
                    name: "loss",
                    title: `Realized Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, key: _key }) => {
                        const key = fixKey(_key);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            key: `${key}realized_loss`,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${key}realized_loss_rel_to_realized_cap`,
                            title: name,
                            color,
                          }),
                        ]);
                      }),
                      createPriceLine({
                        unit: "USD",
                      }),
                    ],
                  },
                  {
                    name: "Total pnl",
                    title: `Total Realized Profit And Loss Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, key: _key }) => {
                        const key = fixKey(_key);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            key: `${key}total_realized_pnl`,
                            name,
                            color,
                          }),
                        ]);
                      }),
                    ],
                  },
                  {
                    name: "Net pnl",
                    title: `Net Realized Profit And Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, key }) => [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${fixKey(key)}net_realized_pnl`,
                          title: name,
                          color,
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${fixKey(
                            key,
                          )}net_realized_pnl_rel_to_realized_cap`,
                          title: name,
                          color,
                        }),
                      ]),
                      createPriceLine({
                        unit: "USD",
                      }),
                      createPriceLine({
                        unit: "%rcap",
                      }),
                    ],
                  },
                  {
                    name: "cumulative",
                    tree: [
                      {
                        name: "profit",
                        title: `Cumulative Realized Profit ${title}`,
                        bottom: list.flatMap(({ color, name, key: _key }) => {
                          const key = fixKey(_key);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}realized_profit_cumulative`,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "loss",
                        title: `Cumulative Realized Loss ${title}`,
                        bottom: list.flatMap(({ color, name, key: _key }) => {
                          const key = fixKey(_key);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              key: `${key}realized_loss_cumulative`,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "Net pnl",
                        title: `Cumulative Net Realized Profit And Loss ${title}`,
                        bottom: [
                          ...list.flatMap(({ color, name, key }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              key: `${fixKey(key)}net_realized_pnl_cumulative`,
                              title: name,
                              color,
                              defaultActive: false,
                            }),
                          ]),
                          createPriceLine({
                            unit: "USD",
                          }),
                        ],
                      },
                      {
                        name: "Net pnl 30d change",
                        title: `Cumulative Net Realized Profit And Loss 30 Day Change ${title}`,
                        bottom: [
                          ...list.flatMap(({ color, name, key }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              key: `${fixKey(
                                key,
                              )}net_realized_pnl_cumulative_30d_delta`,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              key: `${fixKey(
                                key,
                              )}net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              key: `${fixKey(
                                key,
                              )}net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`,
                              title: name,
                              color,
                            }),
                          ]),
                          createPriceLine({
                            unit: "USD",
                          }),
                          createPriceLine({
                            unit: "%mcap",
                          }),
                          createPriceLine({
                            unit: "%rcap",
                          }),
                        ],
                      },
                    ],
                  },
                  {
                    name: "sopr",
                    tree: [
                      {
                        name: "Normal",
                        title: `Spent Output Profit Ratio ${title}`,
                        bottom: [
                          ...list.flatMap(({ color, name, key }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              key: `${fixKey(key)}sopr`,
                              title: name,
                              color,
                            }),
                          ]),
                          createPriceLine({
                            number: 1,
                            unit: "Ratio",
                          }),
                        ],
                      },
                      ...(() => {
                        const reducedList = list
                          .map(({ color, name, key }) => ({
                            color,
                            name,
                            key: `${fixKey(key)}adjusted_sopr`,
                          }))
                          .filter(({ key }) => key in vecIdToIndexes);

                        return reducedList.length
                          ? [
                              {
                                name: "Adjusted",
                                title: `Adjusted Spent Output Profit Ratio ${title}`,
                                bottom: [
                                  ...reducedList.flatMap(
                                    ({ color, name, key }) => [
                                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                        type: "Baseline",
                                        key,
                                        title: name,
                                        color,
                                      }),
                                    ],
                                  ),
                                  createPriceLine({
                                    number: 1,
                                    unit: "Ratio",
                                  }),
                                ],
                              },
                            ]
                          : [];
                      })(),
                    ],
                  },
                ]),

            {
              name: "Sell Side Risk",
              title: `Sell Side Risk Ratio ${title}`,
              bottom: !("list" in args)
                ? list.flatMap(({ key }) => [
                    createBaseSeries({
                      key: `${fixKey(key)}sell_side_risk_ratio`,
                      name: "raw",
                      color: colors.orange,
                    }),
                    createBaseSeries({
                      key: `${fixKey(key)}sell_side_risk_ratio_7d_ema`,
                      name: "7d ema",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: `${fixKey(key)}sell_side_risk_ratio_30d_ema`,
                      name: "30d ema",
                      color: colors.rose,
                      defaultActive: false,
                    }),
                  ])
                : list.flatMap(({ color, name, key }) => [
                    createBaseSeries({
                      key: `${fixKey(key)}sell_side_risk_ratio`,
                      name,
                      color: color,
                    }),
                  ]),
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
                    title: `Unrealized Profit And Loss ${title}`,
                    bottom: [
                      createBaseSeries({
                        key: `${fixKey(args.key)}total_unrealized_pnl`,
                        name: "total",
                        color: colors.default,
                      }),
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
                        key: `${fixKey(args.key)}neg_unrealized_loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}unrealized_profit_rel_to_market_cap`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}unrealized_loss_rel_to_market_cap`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        key: `${fixKey(args.key)}neg_unrealized_loss_rel_to_market_cap`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      ...(`${fixKey(args.key)}unrealized_profit_rel_to_own_market_cap` in
                      vecIdToIndexes
                        ? [
                            createBaseSeries({
                              key: `${fixKey(args.key)}unrealized_profit_rel_to_own_market_cap`,
                              name: "Profit",
                              color: colors.green,
                            }),
                            createBaseSeries({
                              key: `${fixKey(args.key)}unrealized_loss_rel_to_own_market_cap`,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${fixKey(args.key)}neg_unrealized_loss_rel_to_own_market_cap`,
                              name: "Negative Loss",
                              color: colors.red,
                            }),
                            createPriceLine({
                              unit: "%cmcap",
                              number: 100,
                            }),
                            createPriceLine({
                              unit: "%cmcap",
                            }),
                          ]
                        : []),
                      ...(`${fixKey(args.key)}unrealized_profit_rel_to_own_total_unrealized_pnl` in
                      vecIdToIndexes
                        ? [
                            createBaseSeries({
                              key: `${fixKey(args.key)}unrealized_profit_rel_to_own_total_unrealized_pnl`,
                              name: "Profit",
                              color: colors.green,
                            }),
                            createBaseSeries({
                              key: `${fixKey(args.key)}unrealized_loss_rel_to_own_total_unrealized_pnl`,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${fixKey(args.key)}neg_unrealized_loss_rel_to_own_total_unrealized_pnl`,
                              name: "Negative Loss",
                              color: colors.red,
                            }),
                            createPriceLine({
                              unit: "%cp+l",
                              number: 100,
                            }),
                            createPriceLine({
                              unit: "%cp+l",
                            }),
                          ]
                        : []),
                      createPriceLine({
                        unit: "USD",
                        defaultActive: false,
                      }),
                      createPriceLine({
                        unit: "%mcap",
                        defaultActive: false,
                      }),
                    ],
                  },
                ]
              : [
                  {
                    name: "profit",
                    title: `Unrealized Profit ${title}`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}unrealized_profit`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "loss",
                    title: `Unrealized Loss ${title}`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}unrealized_loss`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "total pnl",
                    title: `Unrealized Total Profit And Loss ${title}`,
                    bottom: list.flatMap(({ color, name, key: _key }) => {
                      const key = fixKey(_key);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          key: `${key}total_unrealized_pnl`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                ]),
            {
              name: "Net pnl",
              title: `Net Unrealized Profit And Loss ${title}`,
              bottom: [
                ...list.flatMap(({ color, name, key }) => [
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    key: `${fixKey(key)}net_unrealized_pnl`,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    key: `${fixKey(key)}net_unrealized_pnl_rel_to_market_cap`,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  ...(`${fixKey(key)}net_unrealized_pnl_rel_to_own_market_cap` in
                  vecIdToIndexes
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${fixKey(
                            key,
                          )}net_unrealized_pnl_rel_to_own_market_cap`,
                          title: useGroupName ? name : "Net",
                          color: useGroupName ? color : undefined,
                        }),
                        createPriceLine({
                          unit: "%cmcap",
                        }),
                      ]
                    : []),
                  ...(`${fixKey(key)}net_unrealized_pnl_rel_to_own_total_unrealized_pnl` in
                  vecIdToIndexes
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          key: `${fixKey(
                            key,
                          )}net_unrealized_pnl_rel_to_own_total_unrealized_pnl`,
                          title: useGroupName ? name : "Net",
                          color: useGroupName ? color : undefined,
                        }),
                        createPriceLine({
                          unit: "%cp+l",
                        }),
                      ]
                    : []),
                ]),
                createPriceLine({
                  unit: "USD",
                }),
                createPriceLine({
                  unit: "%mcap",
                }),
              ],
            },
          ],
        },
        ...("list" in args
          ? [
              {
                name: "Cost Basis",
                tree: [
                  {
                    name: "Average",
                    title: `Average Cost Basis ${title}`,
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
                    title: `Min Cost Basis ${title}`,
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
                    title: `Max Cost Basis ${title}`,
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
                name: "Cost Basis",
                title: `Costs Basis ${title}`,
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
          title: `Coins Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, key: _key }) => {
            const key = fixKey(_key);
            return /** @type {const} */ ([
              createBaseSeries({
                key: `${key}coinblocks_destroyed`,
                name: useGroupName ? name : "sum",
                color,
              }),
              createBaseSeries({
                key: `${key}coinblocks_destroyed_cumulative`,
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
                key: `${key}coindays_destroyed_cumulative`,
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
          name: "Market",
          tree: [
            {
              name: "Price",
              title: "Bitcoin Price",
            },
            {
              name: "Capitalization",
              title: "Market Capitalization",
              bottom: [
                createBaseSeries({
                  key: "market_cap",
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
                  key: "price_ath",
                  name: "ath",
                }),
              ],
              bottom: [
                createBaseSeries({
                  key: "price_drawdown",
                  name: "Drawdown",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "days_since_price_ath",
                  name: "since",
                }),
                createBaseSeries({
                  key: "max_days_between_price_aths",
                  name: "Max",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "max_years_between_price_aths",
                  name: "Max",
                  color: colors.red,
                }),
              ],
            },
            {
              name: "Averages",
              tree: [
                {
                  nameAddon: "Simple",
                  keyAddon: "sma",
                },
                {
                  nameAddon: "Exponential",
                  keyAddon: "ema",
                },
              ].map(({ nameAddon, keyAddon }) => ({
                name: nameAddon,
                tree: [
                  {
                    name: "Compare",
                    title: `Market Price ${nameAddon} Moving Averages`,
                    top: averages.map(({ days, key, name, color }) =>
                      createBaseSeries({
                        key: `price_${key}_${keyAddon}`,
                        name: key,
                        color,
                      }),
                    ),
                  },
                  ...averages.map(({ key, name, color }) => ({
                    name,
                    tree: createPriceWithRatioOptions({
                      key: `price_${key}_${keyAddon}`,
                      name,
                      title: `${name} Market Price ${nameAddon} Moving Average`,
                      legend: "average",
                      color,
                    }),
                  })),
                ],
              })),
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
                    key: `${key}_price_returns`,
                    title: "Returns",
                    type: "Baseline",
                  }),
                  createPriceLine({
                    unit: "percentage",
                  }),
                  ...(`${key}_cagr` in vecIdToIndexes
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          key: `${key}_cagr`,
                          title: "cagr",
                          type: "Baseline",
                        }),
                        createPriceLine({
                          unit: "percentage",
                        }),
                      ]
                    : []),
                ],
              })),
            },
            {
              name: "Indicators",
              tree: [
                {
                  name: "Volatility",
                  title: "Bitcoin Price Volatility Index",
                  bottom: [
                    createBaseSeries({
                      key: "price_1w_volatility",
                      name: "1w",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      key: "price_1m_volatility",
                      name: "1m",
                      color: colors.orange,
                    }),
                    createBaseSeries({
                      key: "price_1y_volatility",
                      name: "1y",
                      color: colors.lime,
                    }),
                  ],
                },
                {
                  name: "MinMax",
                  tree: [
                    {
                      key: "1w",
                      title: "1 Week",
                    },
                    {
                      key: "2w",
                      title: "2 Week",
                    },
                    {
                      key: "1m",
                      title: "1 Month",
                    },
                    {
                      key: "1y",
                      title: "1 Year",
                    },
                  ].map(({ key, title }) => ({
                    name: key,
                    title: `Bitcoin Price ${title} MinMax Bands`,
                    top: [
                      createBaseSeries({
                        key: `price_${key}_min`,
                        name: "min",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        key: `price_${key}_max`,
                        name: "max",
                        color: colors.green,
                      }),
                    ],
                  })),
                },
                {
                  name: "True range",
                  title: "Bitcoin Price True Range",
                  bottom: [
                    createBaseSeries({
                      key: "price_true_range",
                      name: "value",
                      color: colors.yellow,
                    }),
                  ],
                },
                {
                  name: "Choppiness",
                  title: "Bitcoin Price Choppiness Index",
                  bottom: [
                    createBaseSeries({
                      key: "price_2w_choppiness_index",
                      name: "2w",
                      color: colors.red,
                    }),
                    createPriceLine({
                      unit: "Index",
                      number: 61.8,
                    }),
                    createPriceLine({
                      unit: "Index",
                      number: 38.2,
                    }),
                  ],
                },
                {
                  name: "Mayer multiple",
                  title: "Mayer multiple",
                  top: [
                    createBaseSeries({
                      key: `price_200d_sma`,
                      name: "200d sma",
                      color: colors.yellow,
                    }),
                    createBaseSeries({
                      key: `price_200d_sma_x2_4`,
                      name: "200d sma x2.4",
                      color: colors.green,
                    }),
                    createBaseSeries({
                      key: `price_200d_sma_x0_8`,
                      name: "200d sma x0.8",
                      color: colors.red,
                    }),
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
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              key: `${key}_price_returns`,
                              title: "lump sum",
                              type: "Baseline",
                            }),
                            createPriceLine({
                              unit: "percentage",
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
                            }),

                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              key: `${key}_price_returns`,
                              title: "lump sum",
                              type: "Baseline",
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              key: `${key}_dca_cagr`,
                              title: "dca cagr",
                              type: "Baseline",
                              colors: [colors.yellow, colors.pink],
                              defaultActive: false,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              key: `${key}_cagr`,
                              title: "lump sum cagr",
                              type: "Baseline",
                              defaultActive: false,
                            }),
                            createPriceLine({
                              unit: "percentage",
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
                              name: "cost basis",
                              color,
                            }),
                          ],
                          bottom: [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              key: `dca_class_${year}_returns`,
                              title: "dca",
                              type: "Baseline",
                            }),
                            createPriceLine({
                              unit: "percentage",
                            }),
                          ],
                        }),
                    ),
                  ],
                },
              ],
            },
          ],
        },
        {
          name: "Chain",
          tree: [
            {
              name: "Block",
              tree: [
                {
                  name: "Count",
                  title: "Block Count",
                  bottom: [
                    ...createSumCumulativeSeries({
                      concat: "block_count",
                    }),
                    createBaseSeries({
                      key: "block_count_target",
                      name: "Target",
                      color: colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    createBaseSeries({
                      key: "1w_block_count",
                      name: "1w sum",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: "1m_block_count",
                      name: "1m sum",
                      color: colors.pink,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      key: "1y_block_count",
                      name: "1y sum",
                      color: colors.purple,
                      defaultActive: false,
                    }),
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
                    createPriceLine({
                      unit: "secs",
                      name: "Target",
                      number: 600,
                    }),
                  ],
                },
                {
                  name: "Size",
                  title: "Block Size",
                  bottom: [
                    createBaseSeries({
                      key: "total_size",
                      name: "raw",
                    }),
                    createBaseSeries({
                      key: "vbytes",
                      name: "raw",
                    }),
                    createBaseSeries({
                      key: "weight",
                      name: "raw",
                    }),
                    // createBaseSeries({
                    //   key: "weight",
                    //   name: "Weight",
                    // }),
                    ...createAverageSumCumulativeMinMaxPercentilesSeries(
                      "block_size",
                    ),
                    ...createAverageSumCumulativeMinMaxPercentilesSeries(
                      "block_weight",
                    ),
                    ...createAverageSumCumulativeMinMaxPercentilesSeries(
                      "block_vbytes",
                    ),
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
                  bottom: createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                    {
                      key: "tx_count",
                      name: "Count",
                    },
                  ),
                },
                {
                  name: "Size",
                  title: "Transaction Size",
                  bottom: [
                    createAverageSeries({
                      concat: "tx_weight",
                      title: "Weight",
                    }),
                    ...createMinMaxPercentilesSeries({
                      concat: "tx_weight",
                      title: "Weight",
                    }),
                    createAverageSeries({ concat: "tx_vsize", title: "VSize" }),
                    ...createMinMaxPercentilesSeries({
                      concat: "tx_vsize",
                      title: "VSize",
                    }),
                  ],
                },
                {
                  name: "Versions",
                  title: "Transaction Versions",
                  bottom: [
                    [colors.orange, colors.red],
                    [colors.cyan, colors.blue],
                    [colors.lime, colors.green],
                  ].flatMap(([sumColor, cumulativeColor], index) =>
                    createSumCumulativeSeries({
                      concat: `tx_v${index + 1}`,
                      common: `v${index + 1}`,
                      sumColor,
                      cumulativeColor,
                    }),
                  ),
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
                    createCumulativeSeries({
                      concat: "input_count",
                    }),
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
                    createCumulativeSeries({
                      concat: "input_count",
                    }),
                    ...createMinMaxPercentilesSeries({
                      concat: "output_count",
                    }),
                  ],
                },
                // {
                //   name: "Value",
                //   title: "Transaction Output Value",
                //   bottom: [
                //     createAverageSeries({ concat: "output_value" }),
                //     ...createSumCumulativeSeries({ concat: "output_value" }),
                //   ],
                // },
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
                      key: "supply",
                      name: "Mined",
                    }),
                    createBaseSeries({
                      key: "supply_in_btc",
                      name: "Mined",
                    }),
                    createBaseSeries({
                      key: "supply_in_usd",
                      name: "Mined",
                    }),
                  ],
                },
                {
                  name: "Rewards",
                  tree: [
                    {
                      name: "Coinbase",
                      title: "Coinbase",
                      bottom: [
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "coinbase",
                            name: "Coinbase",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "coinbase_in_btc",
                            name: "Coinbase",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "coinbase_in_usd",
                            name: "Coinbase",
                          },
                        ),
                      ],
                    },
                    {
                      name: "Subsidy",
                      title: "Subsidy",
                      bottom: [
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "subsidy",
                            name: "Subsidy",
                          },
                        ),
                        createBaseSeries({
                          key: "subsidy_usd_1y_sma",
                          name: "1y sma",
                        }),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "subsidy_in_btc",
                            name: "Subsidy",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            key: "subsidy_in_usd",
                            name: "Subsidy",
                          },
                        ),
                      ],
                    },
                    {
                      name: "Fee",
                      title: "Transaction Fee",
                      bottom: [
                        ...createAverageSumCumulativeMinMaxPercentilesSeries(
                          "fee",
                        ),
                        ...createAverageSumCumulativeMinMaxPercentilesSeries(
                          "fee_in_btc",
                        ),
                        ...createAverageSumCumulativeMinMaxPercentilesSeries(
                          "fee_in_usd",
                        ),
                      ],
                    },
                    {
                      name: "Dominance",
                      title: "Reward Dominance",
                      bottom: [
                        createBaseSeries({
                          key: "fee_dominance",
                          name: "Fee",
                          color: colors.amber,
                        }),
                        createBaseSeries({
                          key: "subsidy_dominance",
                          name: "Subsidy",
                          color: colors.red,
                        }),
                      ],
                    },
                    {
                      name: "Unclaimed",
                      title: "Unclaimed Rewards",
                      bottom: [
                        ...createSumCumulativeSeries({
                          concat: "unclaimed_rewards",
                        }),
                        ...createSumCumulativeSeries({
                          concat: "unclaimed_rewards_in_btc",
                        }),
                        ...createSumCumulativeSeries({
                          concat: "unclaimed_rewards_in_usd",
                        }),
                      ],
                    },
                    {
                      name: "Puell multiple",
                      title: "Puell multiple",
                      bottom: [
                        createBaseSeries({
                          key: "puell_multiple",
                          name: "Multiple",
                        }),
                      ],
                    },
                  ],
                },
                {
                  name: "Feerate",
                  title: "Transaction Fee Rate",
                  bottom: [
                    createAverageSeries({ concat: "fee_rate" }),
                    ...createMinMaxPercentilesSeries({
                      concat: "fee_rate",
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
                    createBaseSeries({
                      key: "difficultyepoch",
                      name: "Epoch",
                    }),
                    {
                      key: "difficulty_adjustment",
                      title: "Adjustment",
                      type: "Baseline",
                    },
                    createBaseSeries({
                      key: "blocks_before_next_difficulty_adjustment",
                      name: "Before next",
                    }),
                    createBaseSeries({
                      key: "days_before_next_difficulty_adjustment",
                      name: "Before next",
                    }),
                  ],
                },
                {
                  name: "hash",
                  tree: [
                    {
                      name: "Rate",
                      title: "Hash Rate",
                      bottom: [
                        createBaseSeries({
                          key: "hash_rate",
                          name: "Raw",
                        }),
                        createBaseSeries({
                          key: "hash_rate_1w_sma",
                          name: "1w sma",
                          color: colors.red,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          key: "hash_rate_1m_sma",
                          name: "1m sma",
                          color: colors.pink,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          key: "hash_rate_2m_sma",
                          name: "2m sma",
                          color: colors.purple,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          key: "hash_rate_1y_sma",
                          name: "1y sma",
                          color: colors.indigo,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          key: "difficulty_as_hash",
                          name: "difficulty",
                          color: colors.default,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                      ],
                    },
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
                {
                  name: "Pools",
                  tree: Object.entries(pools).map(([_key, name]) => {
                    const key = /** @type {Pool} */ (_key);
                    return {
                      name,
                      tree: [
                        {
                          name: "Dominance",
                          title: `Mining Dominance of ${name}`,
                          bottom: [
                            createBaseSeries({
                              key: `${key}_1d_dominance`,
                              name: "1d",
                              color: colors.rose,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${key}_1w_dominance`,
                              name: "1w",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${key}_1m_dominance`,
                              name: "1m",
                            }),
                            createBaseSeries({
                              key: `${key}_1y_dominance`,
                              name: "1y",
                              color: colors.lime,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${key}_dominance`,
                              name: "all time",
                              color: colors.teal,
                              defaultActive: false,
                            }),
                          ],
                        },
                        {
                          name: "Blocks mined",
                          title: `Blocks mined by ${name}`,
                          bottom: [
                            createBaseSeries({
                              key: `${key}_blocks_mined`,
                              name: "Sum",
                            }),
                            createBaseSeries({
                              key: `${key}_blocks_mined_cumulative`,
                              name: "Cumulative",
                              color: colors.blue,
                            }),
                            createBaseSeries({
                              key: `${key}_1w_blocks_mined`,
                              name: "1w Sum",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${key}_1m_blocks_mined`,
                              name: "1m Sum",
                              color: colors.pink,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              key: `${key}_1y_blocks_mined`,
                              name: "1y Sum",
                              color: colors.purple,
                              defaultActive: false,
                            }),
                          ],
                        },
                        {
                          name: "Rewards",
                          title: `Rewards collected by ${name}`,
                          bottom: [
                            {
                              keyAddon: "coinbase",
                              cumulativeColor: colors.red,
                              sumColor: colors.orange,
                            },
                            {
                              keyAddon: "subsidy",
                              cumulativeColor: colors.emerald,
                              sumColor: colors.lime,
                            },
                            {
                              keyAddon: "fee",
                              cumulativeColor: colors.indigo,
                              sumColor: colors.cyan,
                            },
                          ].flatMap(
                            ({ keyAddon, sumColor, cumulativeColor }) => [
                              ...createSumCumulativeSeries({
                                concat: `${key}_${keyAddon}`,
                                common: keyAddon,
                                sumColor,
                                cumulativeColor,
                              }),
                              ...createSumCumulativeSeries({
                                concat: `${key}_${keyAddon}_in_btc`,
                                common: keyAddon,
                                sumColor,
                                cumulativeColor,
                              }),
                              ...createSumCumulativeSeries({
                                concat: `${key}_${keyAddon}_in_usd`,
                                common: keyAddon,
                                sumColor,
                                cumulativeColor,
                              }),
                            ],
                          ),
                        },
                        {
                          name: "Days since block",
                          title: `Days since ${name} mined a block`,
                          bottom: [
                            createBaseSeries({
                              key: `${key}_days_since_block`,
                              name: "Since block",
                            }),
                          ],
                        },
                      ],
                    };
                  }),
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
                        createBaseSeries({
                          key: "opreturn_count",
                          name: "Count",
                        }),
                        createBaseSeries({
                          key: "opreturn_count",
                          name: "sum",
                        }),
                        createBaseSeries({
                          key: "opreturn_count_cumulative",
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
          ],
        },
        {
          name: "Cohorts",
          tree: [
            createCohortGroupFolder(cohortAll),
            {
              name: "terms",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs Term",
                  list: [...terms, cohortAllForComparaison],
                }),
                ...terms.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Epochs",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Epoch",
                  list: [...epoch, cohortAllForComparaison],
                }),
                ...epoch.map(createCohortGroupFolder),
              ],
            },
            {
              name: "types",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Type",
                  list: [...type, cohortAllForComparaison],
                }),
                ...type.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs Up to age",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs Up To Age",
                  list: [...upToDate, cohortAllForComparaison],
                }),
                ...upToDate.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs from age",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs from age",
                  list: [...fromDate, cohortAllForComparaison],
                }),
                ...fromDate.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs age Ranges",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs Age Range",
                  list: [...dateRange, cohortAllForComparaison],
                }),
                ...dateRange.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs under amounts",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs under amount",
                  list: [...utxosUnderAmount, cohortAllForComparaison],
                }),
                ...utxosUnderAmount.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs Above Amounts",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs Above Amount",
                  list: [...utxosAboveAmount, cohortAllForComparaison],
                }),
                ...utxosAboveAmount.map(createCohortGroupFolder),
              ],
            },
            {
              name: "UTXOs between amounts",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs between amounts",
                  list: [...utxosAmountRanges, cohortAllForComparaison],
                }),
                ...utxosAmountRanges.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses under amount",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Addresses under Amount",
                  list: [...addressesUnderAmount, cohortAllForComparaison],
                }),
                ...addressesUnderAmount.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses above amount",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Addresses above amount",
                  list: [...addressesAboveAmount, cohortAllForComparaison],
                }),
                ...addressesAboveAmount.map(createCohortGroupFolder),
              ],
            },
            {
              name: "Addresses between amounts",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "Addresses between amounts",
                  list: [...addressesAmountRanges, cohortAllForComparaison],
                }),
                ...addressesAmountRanges.map(createCohortGroupFolder),
              ],
            },
          ],
        },
        {
          name: "Cointime",
          tree: [
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
                ...cointimePrices.map(({ key, name, color, title }) => ({
                  name,
                  tree: createPriceWithRatioOptions({
                    key,
                    legend: name,
                    color,
                    name,
                    title,
                  }),
                })),
              ],
            },
            {
              name: "Capitalization",
              tree: [
                {
                  name: "Compare",
                  title: "Compare Cointime Capitalizations",
                  bottom: [
                    createBaseSeries({
                      key: `market_cap`,
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
                        key: `market_cap`,
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
              name: "Coinblocks",
              title: "Coinblocks",
              bottom: [
                createBaseSeries({
                  key: "coinblocks_destroyed",
                  name: "Destroyed",
                  color: colors.red,
                }),
                createBaseSeries({
                  key: "coinblocks_destroyed_cumulative",
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
                  key: "coinblocks_created_cumulative",
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
                  key: "coinblocks_stored_cumulative",
                  name: "Cumulative stored",
                  color: colors.green,
                  defaultActive: false,
                }),
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
          title: "Link to API documentation",
        },
        {
          name: "MCP",
          url: () =>
            "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_mcp#brk-mcp",
          title: "Link to MCP documentation",
        },
        {
          name: "Crates",
          url: () => "/crates",
          title: "Link to BRK on crates.io",
        },
        {
          name: "Source",
          url: () => "/github",
          title: "Link to BRK's repository",
        },
      ],
    },
    {
      name: "Hosting",
      tree: [
        {
          name: "Status",
          url: () => "/status",
          title: "Link to servers status",
        },
        {
          name: "Self",
          url: () => "/cli",
          title: "Link to self-hosting documentation",
        },
        {
          name: "As a service",
          url: () => "/hosting",
          title: "Link to hosting service",
        },
      ],
    },
    {
      name: "Social",
      tree: [
        {
          name: "GitHub",
          url: () => "/github",
          title: "Link to Github",
        },
        {
          name: "Nostr",
          url: () => "/nostr",
          title: "Link to BRK's nostr account",
        },
        {
          name: "Discord",
          url: () => "/discord",
          title: "Link to BRK's discord server",
        },
      ],
    },
    {
      name: "Donate",
      qrcode: true,
      url: () => "bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve",
      title: "Bitcoin address for donations",
    },
    {
      name: "Share",
      qrcode: true,
      url: () => window.location.href,
      title: "Share",
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
 * @param {Pools} args.pools
 * @param {Signal<string | null>} args.qrcode
 */
export function initOptions({
  colors,
  signals,
  env,
  utils,
  qrcode,
  vecIdToIndexes,
  pools,
}) {
  const LS_SELECTED_KEY = `selected_path`;

  const urlPath_ = window.document.location.pathname
    .split("/")
    .filter((v) => v);
  const urlPath = urlPath_.length ? urlPath_ : undefined;
  const savedPath = /** @type {string[]} */ (
    JSON.parse(utils.storage.read(LS_SELECTED_KEY) || "[]") || []
  ).filter((v) => v);
  console.log(savedPath);

  /** @type {Signal<Option>} */
  const selected = signals.createSignal(/** @type {any} */ (undefined));

  const partialOptions = createPartialOptions({
    env,
    colors,
    vecIdToIndexes,
    pools,
  });

  /** @type {Option[]} */
  const list = [];

  const parent = signals.createSignal(/** @type {HTMLElement | null} */ (null));

  /**
   * @param {AnyFetchedSeriesBlueprint[]} [arr]
   */
  function arrayToRecord(arr = []) {
    return (arr || []).reduce((record, blueprint) => {
      if (env.localhost && !(blueprint.key in vecIdToIndexes)) {
        throw Error(`${blueprint.key} not recognized`);
      }
      const unit = blueprint.unit ?? utils.vecidToUnit(blueprint.key);
      record[unit] ??= [];
      record[unit].push(blueprint);
      return record;
    }, /** @type {Record<Unit, AnyFetchedSeriesBlueprint[]>} */ ({}));
  }

  /**
   * @param {Option} option
   */
  function selectOption(option) {
    utils.url.pushHistory(option.path);
    utils.url.resetParams(option);
    utils.storage.write(LS_SELECTED_KEY, JSON.stringify(option.path));
    selected.set(option);
  }

  /**
   * @param {Object} args
   * @param {Option} args.option
   * @param {string} args.frame
   * @param {Signal<string | null>} args.qrcode
   * @param {string} [args.name]
   */
  function createOptionElement({ option, frame, name, qrcode }) {
    const title = option.title;
    if (option.kind === "url") {
      const href = option.url();

      if (option.qrcode) {
        return utils.dom.createButtonElement({
          inside: option.name,
          title,
          onClick: () => {
            qrcode.set(option.url);
          },
        });
      } else {
        return utils.dom.createAnchorElement({
          href,
          blank: true,
          text: option.name,
          title,
        });
      }
    } else {
      return utils.dom.createAnchorElement({
        href: `/${option.path.join("/")}`,
        title,
        text: name || option.name,
        onClick: () => {
          selectOption(option);
        },
      });
    }
  }

  /** @type {Option | undefined} */
  let savedOption;

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {Accessor<HTMLElement | null>} parent
   * @param {string[] | undefined} parentPath
   * @returns {Accessor<number>}
   */
  function recursiveProcessPartialTree(
    partialTree,
    parent,
    parentPath = [],
    depth = 0,
  ) {
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
        /** @type {Omit<OptionsGroup, keyof PartialOptionsGroup>} */
        const groupAddons = {};

        Object.assign(anyPartial, groupAddons);

        const passedDetails = signals.createSignal(
          /** @type {HTMLDivElement | HTMLDetailsElement | null} */ (null),
        );

        const serName = utils.stringToId(anyPartial.name);
        const path = [...parentPath, serName];
        const childOptionsCount = recursiveProcessPartialTree(
          anyPartial.tree,
          passedDetails,
          path,
          depth + 1,
        );

        listForSum.push(childOptionsCount);

        signals.createEffect(li, (li) => {
          if (!li) {
            passedDetails.set(null);
            return;
          }

          signals.createEffect(selected, (selected) => {
            if (
              path.length <= selected.path.length &&
              path.every((v, i) => selected.path.at(i) === v)
            ) {
              li.dataset.highlight = "";
            } else {
              delete li.dataset.highlight;
            }
          });

          const details = window.document.createElement("details");
          details.dataset.name = serName;
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
        const option = /** @type {Option} */ (anyPartial);

        const name = option.name;
        const path = [...parentPath, utils.stringToId(option.name)];

        if ("kind" in anyPartial && anyPartial.kind === "explorer") {
          Object.assign(
            option,
            /** @satisfies {ExplorerOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: option.title,
            }),
          );
        } else if ("kind" in anyPartial && anyPartial.kind === "table") {
          Object.assign(
            option,
            /** @satisfies {TableOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: option.title,
            }),
          );
        } else if ("kind" in anyPartial && anyPartial.kind === "simulation") {
          Object.assign(
            option,
            /** @satisfies {SimulationOption} */ ({
              kind: anyPartial.kind,
              path,
              name,
              title: anyPartial.title,
            }),
          );
        } else if ("url" in anyPartial) {
          Object.assign(
            option,
            /** @satisfies {UrlOption} */ ({
              kind: "url",
              path,
              name,
              title: name,
              qrcode: !!anyPartial.qrcode,
              url: anyPartial.url,
            }),
          );
        } else {
          const title = option.title || option.name;
          Object.assign(
            option,
            /** @satisfies {ChartOption} */ ({
              kind: "chart",
              name,
              title,
              path,
              top: arrayToRecord(anyPartial.top),
              bottom: arrayToRecord(anyPartial.bottom),
            }),
          );
        }

        list.push(option);

        if (urlPath) {
          const sameAsURLPath =
            urlPath.length === path.length &&
            urlPath.every((val, i) => val === path[i]);
          if (sameAsURLPath) {
            selected.set(option);
          }
        } else if (savedPath) {
          const sameAsSavedPath =
            savedPath.length === path.length &&
            savedPath.every((val, i) => val === path[i]);
          if (sameAsSavedPath) {
            savedOption = option;
          }
        }

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
  recursiveProcessPartialTree(partialOptions, parent);

  if (!selected()) {
    const option =
      savedOption || list.find((option) => option.kind === "chart");
    if (option) {
      selected.set(option);
    }
  }

  return {
    selected,
    list,
    tree: /** @type {OptionsTree} */ (partialOptions),
    parent,
    createOptionElement,
    selectOption,
  };
}
/** @typedef {ReturnType<typeof initOptions>} Options */
