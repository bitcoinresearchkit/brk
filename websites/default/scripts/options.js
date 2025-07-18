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
      key: "short_term_holders",
      name: "short",
      title: "Short Term Holders",
      color: colors.yellow,
    },
    {
      key: "long_term_holders",
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
      name: "100K btc+",
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
        key: `${concat}_cumulative`,
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
        ...(`${key}_ratio_p1sd_as_price` in vecIdToIndexes
          ? [
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
            ]
          : []),
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
        ...(`${key}_ratio_p1sd` in vecIdToIndexes
          ? [
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
            ]
          : []),
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
                  title: `Even Supply ${title}`,
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
                  title: `Address Count ${title}`,
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
                      title: `Loaded Address Count ${title}`,
                      bottom: list
                        .filter(
                          ({ key }) =>
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
                            title: `Empty Address Count ${title}`,
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
        {
          name: "Realized",
          tree: [
            {
              name: "cap",
              title: `Realized Capitalization ${title}`,
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
                  title: `Realized Price ${title}`,
                  top: args.list.map(({ color, name, key }) =>
                    createBaseSeries({
                      key: `${fixKey(key)}realized_price`,
                      name,
                      color,
                    }),
                  ),
                }
              : createPriceWithRatio({
                  title: `Realized Price ${title}`,
                  key: `${fixKey(args.key)}realized_price`,
                  name: "price",
                  legend: "realized",
                  color: args.color,
                }),
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
                        key: `${fixKey(args.key)}realized_profit_cumulative`,
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
                        key: `${fixKey(args.key)}realized_loss_cumulative`,
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
                        key: `${fixKey(
                          args.key,
                        )}negative_realized_loss_cumulative`,
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
                    title: `Net Realized Profit And Loss ${title}`,
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
                        key: `${fixKey(
                          key,
                        )}net_realized_profit_and_loss_cumulative`,
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
                        key: `${fixKey(
                          key,
                        )}net_realized_profit_and_loss_cumulative_30d_change`,
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
                        key: `${fixKey(
                          key,
                        )}net_realized_profit_and_loss_cumulative_30d_change_relative_to_realized_cap`,
                        title: "cum net 30d change",
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
                        )}net_realized_profit_and_loss_cumulative_30d_change_relative_to_market_cap`,
                        title: "cumulative net 30d change",
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
                    title: `Spent Output Profit Ratio ${title}`,
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
                    title: `Realized Profit ${title}`,
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
                    title: `Realized Loss ${title}`,
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
                    title: `Net Realized Profit And Loss ${title}`,
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
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${fixKey(
                              key,
                            )}net_realized_profit_and_loss_cumulative`,
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
                        title: `Cumulative Net Realized Profit And Loss 30 Day Change ${title}`,
                        bottom: list.flatMap(({ color, name, key }) => [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            key: `${fixKey(
                              key,
                            )}net_realized_profit_and_loss_cumulative_30d_change`,
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
                            )}net_realized_profit_and_loss_cumulative_30d_change_relative_to_realized_cap`,
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
                            )}net_realized_profit_and_loss_cumulative_30d_change_relative_to_market_cap`,
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
                        title: `Spent Output Profit Ratio ${title}`,
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
                        title: `Adjusted Spent Output Profit Ratio ${title}`,
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
              title: `Sell Side Risk Ratio ${title}`,
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
                    title: `Unrealized Profit And Loss ${title}`,
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
                    title: `Unrealized Profit ${title}`,
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
                    title: `Unrealized Loss ${title}`,
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
              title: `Net Unrealized Profit And Loss ${title}`,
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
                    title: `Average Price Paid ${title}`,
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
                    title: `Min Price Paid ${title}`,
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
                    title: `Max Price Paid ${title}`,
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
                title: `Prices Paid ${title}`,
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
                  key: "subsidy_in_btc_cumulative",
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
          name: "Cohorts",
          tree: [
            createCohortGroupFolder({
              key: "",
              name: "",
              title: "",
              color: colors.orange,
            }),
            {
              name: "terms",
              tree: [
                createCohortGroupFolder({
                  name: "Compare",
                  title: "UTXOs Term",
                  list: terms,
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
                  list: epoch,
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
                  list: type,
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
                  list: upToDate,
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
                  list: fromDate,
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
                  list: dateRange,
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
                  list: utxosUnderAmount,
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
                  list: utxosAboveAmount,
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
                  list: utxosAmountRanges,
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
                  list: addressesUnderAmount,
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
                  list: addressesAboveAmount,
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
                  list: addressesAmountRanges,
                }),
                ...addressesAmountRanges.map(createCohortGroupFolder),
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
      if (env.localhost && !(blueprint.key in vecIdToIndexes)) {
        throw Error(`${blueprint.key} not recognized`);
      }
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
