/**
 * @import { AnySpecificSeriesBlueprint, CohortOption, CohortOptions, Color, DefaultCohortOption, DefaultCohortOptions, OptionPath, OptionsGroup, PartialChartOption, PartialOptionsGroup, PartialOptionsTree, RatioOption, RatioOptions, Series, SeriesBlueprint, SeriesBlueprintParam, SeriesBluePrintType, Signal, TimeScale } from "./types/self"
 */

const DATE_TO_PREFIX = "date-to-";
const HEIGHT_TO_PREFIX = "height-to-";

function initGroups() {
  const xTermHolders = /** @type {const} */ ([
    {
      id: "sth",
      key: "sth",
      name: "Short Term Holders",
      legend: "Short Term Holders - STH",
    },
    {
      id: "lth",
      key: "lth",
      name: "Long Term Holders",
      legend: "Long Term Holders - LTH",
    },
  ]);

  const upTo = /** @type {const} */ ([
    {
      id: "up-to-1d",
      key: "up_to_1d",
      name: "Up To 1 Day",
      legend: "1D",
    },
    {
      id: "up-to-1w",
      key: "up_to_1w",
      name: "Up To 1 Week",
      legend: "1W",
    },
    {
      id: "up-to-1m",
      key: "up_to_1m",
      name: "Up To 1 Month",
      legend: "1M",
    },
    {
      id: "up-to-2m",
      key: "up_to_2m",
      name: "Up To 2 Months",
      legend: "2M",
    },
    {
      id: "up-to-3m",
      key: "up_to_3m",
      name: "Up To 3 Months",
      legend: "3M",
    },
    {
      id: "up-to-4m",
      key: "up_to_4m",
      name: "Up To 4 Months",
      legend: "4M",
    },
    {
      id: "up-to-5m",
      key: "up_to_5m",
      name: "Up To 5 Months",
      legend: "5M",
    },
    {
      id: "up-to-6m",
      key: "up_to_6m",
      name: "Up To 6 Months",
      legend: "6M",
    },
    {
      id: "up-to-1y",
      key: "up_to_1y",
      name: "Up To 1 Year",
      legend: "1Y",
    },
    {
      id: "up-to-2y",
      key: "up_to_2y",
      name: "Up To 2 Years",
      legend: "2Y",
    },
    {
      id: "up-to-3y",
      key: "up_to_3y",
      name: "Up To 3 Years",
      legend: "3Y",
    },
    {
      id: "up-to-5y",
      key: "up_to_5y",
      name: "Up To 5 Years",
      legend: "5Y",
    },
    {
      id: "up-to-7y",
      key: "up_to_7y",
      name: "Up To 7 Years",
      legend: "7Y",
    },
    {
      id: "up-to-10y",
      key: "up_to_10y",
      name: "Up To 10 Years",
      legend: "10Y",
    },
    {
      id: "up-to-15y",
      key: "up_to_15y",
      name: "Up To 15 Years",
      legend: "15Y",
    },
  ]);

  const fromXToY = /** @type {const} */ ([
    {
      id: "up-to-1d",
      key: "up_to_1d",
      name: "24h",
      legend: "24h",
    },
    {
      id: "from-1d-to-1w",
      key: "from_1d_to_1w",
      name: "From 1 Day To 1 Week",
      legend: "1D — 1W",
    },
    {
      id: "from-1w-to-1m",
      key: "from_1w_to_1m",
      name: "From 1 Week To 1 Month",
      legend: "1W — 1M",
    },
    {
      id: "from-1m-to-3m",
      key: "from_1m_to_3m",
      name: "From 1 Month To 3 Months",
      legend: "1M — 3M",
    },
    {
      id: "from-3m-to-6m",
      key: "from_3m_to_6m",
      name: "From 3 Months To 6 Months",
      legend: "3M — 6M",
    },
    {
      id: "from-6m-to-1y",
      key: "from_6m_to_1y",
      name: "From 6 Months To 1 Year",
      legend: "6M — 1Y",
    },
    {
      id: "from-1y-to-2y",
      key: "from_1y_to_2y",
      name: "From 1 Year To 2 Years",
      legend: "1Y — 2Y",
    },
    {
      id: "from-2y-to-3y",
      key: "from_2y_to_3y",
      name: "From 2 Years To 3 Years",
      legend: "2Y — 3Y",
    },
    {
      id: "from-3y-to-5y",
      key: "from_3y_to_5y",
      name: "From 3 Years To 5 Years",
      legend: "3Y — 5Y",
    },
    {
      id: "from-5y-to-7y",
      key: "from_5y_to_7y",
      name: "From 5 Years To 7 Years",
      legend: "5Y — 7Y",
    },
    {
      id: "from-7y-to-10y",
      key: "from_7y_to_10y",
      name: "From 7 Years To 10 Years",
      legend: "7Y — 10Y",
    },
    {
      id: "from-10y-to-15y",
      key: "from_10y_to_15y",
      name: "From 10 Years To 15 Years",
      legend: "10Y — 15Y",
    },
    {
      id: "from-15y",
      key: "from_15y",
      name: "From 15 Years To End",
      legend: "15Y — End",
    },
  ]);

  const fromX = /** @type {const} */ ([
    {
      id: "from-1y",
      key: "from_1y",
      name: "From 1 Year",
      legend: "1Y+",
    },
    {
      id: "from-2y",
      key: "from_2y",
      name: "From 2 Years",
      legend: "2Y+",
    },
    {
      id: "from-4y",
      key: "from_4y",
      name: "From 4 Years",
      legend: "4Y+",
    },
    {
      id: "from-10y",
      key: "from_10y",
      name: "From 10 Years",
      legend: "10Y+",
    },
    {
      id: "from-15y",
      key: "from_15y",
      name: "From 15 Years",
      legend: "15Y+",
    },
  ]);

  const epochs = /** @type {const} */ ([
    { id: "epoch-1", key: "epoch_1", name: "Epoch 1" },
    { id: "epoch-2", key: "epoch_2", name: "Epoch 2" },
    { id: "epoch-3", key: "epoch_3", name: "Epoch 3" },
    { id: "epoch-4", key: "epoch_4", name: "Epoch 4" },
    { id: "epoch-5", key: "epoch_5", name: "Epoch 5" },
  ]);

  const age = /** @type {const} */ ([
    {
      key: "",
      id: "",
      name: "",
    },
    ...xTermHolders,
    ...upTo,
    ...fromXToY,
    ...fromX,
    ...epochs,
  ]);

  const size = /** @type {const} */ ([
    {
      key: "plankton",
      name: "Plankton",
      size: "1 sat to 0.1 BTC",
    },
    {
      key: "shrimp",
      name: "Shrimp",
      size: "0.1 sat to 1 BTC",
    },
    { key: "crab", name: "Crab", size: "1 BTC to 10 BTC" },
    { key: "fish", name: "Fish", size: "10 BTC to 100 BTC" },
    { key: "shark", name: "Shark", size: "100 BTC to 1000 BTC" },
    { key: "whale", name: "Whale", size: "1000 BTC to 10 000 BTC" },
    {
      key: "humpback",
      name: "Humpback",
      size: "10 000 BTC to 100 000 BTC",
    },
    {
      key: "megalodon",
      name: "Megalodon",
      size: "More than 100 000 BTC",
    },
  ]);

  const type = /** @type {const} */ ([
    { key: "p2pk", name: "P2PK" },
    { key: "p2pkh", name: "P2PKH" },
    { key: "p2sh", name: "P2SH" },
    { key: "p2wpkh", name: "P2WPKH" },
    { key: "p2wsh", name: "P2WSH" },
    { key: "p2tr", name: "P2TR" },
  ]);

  const address = /** @type {const} */ ([...size, ...type]);

  const liquidities = /** @type {const} */ ([
    {
      key: "illiquid",
      id: "illiquid",
      name: "Illiquid",
    },
    { key: "liquid", id: "liquid", name: "Liquid" },
    {
      key: "highly_liquid",
      id: "highly-liquid",
      name: "Highly Liquid",
    },
  ]);

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

  const totalReturns = /** @type {const} */ ([
    { name: "1 Day", key: "1d" },
    { name: "1 Month", key: "1m" },
    { name: "6 Months", key: "6m" },
    { name: "1 Year", key: "1y" },
    { name: "2 Years", key: "2y" },
    { name: "3 Years", key: "3y" },
    { name: "4 Years", key: "4y" },
    { name: "6 Years", key: "6y" },
    { name: "8 Years", key: "8y" },
    { name: "10 Years", key: "10y" },
  ]);

  const compoundReturns = /** @type {const} */ ([
    { name: "4 Years", key: "4y" },
  ]);

  const percentiles = /** @type {const} */ ([
    {
      key: "median_price_paid",
      id: "median-price-paid",
      name: "Median",
      title: "Median Paid",
      value: 50,
    },
    {
      key: "95p_price_paid",
      id: "95p-price-paid",
      name: `95%`,
      title: `95th Percentile Paid`,
      value: 95,
    },
    {
      key: "90p_price_paid",
      id: "90p-price-paid",
      name: `90%`,
      title: `90th Percentile Paid`,
      value: 90,
    },
    {
      key: "85p_price_paid",
      id: "85p-price-paid",
      name: `85%`,
      title: `85th Percentile Paid`,
      value: 85,
    },
    {
      key: "80p_price_paid",
      id: "80p-price-paid",
      name: `80%`,
      title: `80th Percentile Paid`,
      value: 80,
    },
    {
      key: "75p_price_paid",
      id: "75p-price-paid",
      name: `75%`,
      title: `75th Percentile Paid`,
      value: 75,
    },
    {
      key: "70p_price_paid",
      id: "70p-price-paid",
      name: `70%`,
      title: `70th Percentile Paid`,
      value: 70,
    },
    {
      key: "65p_price_paid",
      id: "65p-price-paid",
      name: `65%`,
      title: `65th Percentile Paid`,
      value: 65,
    },
    {
      key: "60p_price_paid",
      id: "60p-price-paid",
      name: `60%`,
      title: `60th Percentile Paid`,
      value: 60,
    },
    {
      key: "55p_price_paid",
      id: "55p-price-paid",
      name: `55%`,
      title: `55th Percentile Paid`,
      value: 55,
    },
    {
      key: "45p_price_paid",
      id: "45p-price-paid",
      name: `45%`,
      title: `45th Percentile Paid`,
      value: 45,
    },
    {
      key: "40p_price_paid",
      id: "40p-price-paid",
      name: `40%`,
      title: `40th Percentile Paid`,
      value: 40,
    },
    {
      key: "35p_price_paid",
      id: "35p-price-paid",
      name: `35%`,
      title: `35th Percentile Paid`,
      value: 35,
    },
    {
      key: "30p_price_paid",
      id: "30p-price-paid",
      name: `30%`,
      title: `30th Percentile Paid`,
      value: 30,
    },
    {
      key: "25p_price_paid",
      id: "25p-price-paid",
      name: `25%`,
      title: `25th Percentile Paid`,
      value: 25,
    },
    {
      key: "20p_price_paid",
      id: "20p-price-paid",
      name: `20%`,
      title: `20th Percentile Paid`,
      value: 20,
    },
    {
      key: "15p_price_paid",
      id: "15p-price-paid",
      name: `15%`,
      title: `15th Percentile Paid`,
      value: 15,
    },
    {
      key: "10p_price_paid",
      id: "10p-price-paid",
      name: `10%`,
      title: `10th Percentile Paid`,
      value: 10,
    },
    {
      key: "05p_price_paid",
      id: "05p-price-paid",
      name: `5%`,
      title: `5th Percentile Paid`,
      value: 5,
    },
  ]);

  return {
    xTermHolders,
    upTo,
    fromX,
    fromXToY,
    epochs,
    age,
    type,
    size,
    address,
    liquidities,
    averages,
    totalReturns,
    compoundReturns,
    percentiles,
  };
}
/**
 * @typedef {ReturnType<typeof initGroups>} Groups
 *
 * @typedef {Groups["age"][number]["id"]} AgeCohortId
 *
 * @typedef {Exclude<AgeCohortId, "">} AgeCohortIdSub
 *
 * @typedef {Groups["address"][number]["key"]} AddressCohortId
 *
 * @typedef {Groups["liquidities"][number]["id"]} LiquidityId
 *
 * @typedef {AgeCohortId | AddressCohortId} AnyCohortId
 *
 * @typedef {AddressCohortId | LiquidityId} AnyAddressCohortId
 *
 * @typedef {AnyCohortId | LiquidityId} AnyPossibleCohortId
 *
 * @typedef {'' | `${AgeCohortIdSub | AddressCohortId | LiquidityId}-`} AnyDatasetPrefix
 *
 * @typedef {Groups["averages"][number]["key"]} AverageName
 *
 * @typedef {Groups["totalReturns"][number]["key"]} TotalReturnKey
 *
 * @typedef {Groups["compoundReturns"][number]["key"]} CompoundReturnKey
 *
 * @typedef {Groups["percentiles"][number]["id"]} PercentileId
 */

/**
 * @param {Colors} colors
 * @returns {PartialOptionsTree}
 */
function createPartialOptions(colors) {
  const groups = initGroups();

  const bases = {
    /**
     * @param {TimeScale} scale
     * @param {string} [title]
     */
    0(scale, title) {
      return /** @type {const} */ ({
        title: title || `Base`,
        color: colors.off,
        datasetPath: `${scale}-to-0`,
        options: {
          lineStyle: 3,
          lastValueVisible: false,
        },
      });
    },
    /**
     * @param {TimeScale} scale
     * @param {string} [title]
     */
    1(scale, title) {
      return /** @type {const} */ ({
        title: title || `Base`,
        color: colors.off,
        datasetPath: `${scale}-to-1`,
        options: {
          lineStyle: 3,
          lastValueVisible: false,
        },
      });
    },
    /**
     * @param {TimeScale} scale
     * @param {string} [title]
     */
    100(scale, title) {
      return /** @type {const} */ ({
        title: title || `Base`,
        color: colors.off,
        datasetPath: `${scale}-to-100`,
        options: {
          lineStyle: 3,
          lastValueVisible: false,
        },
      });
    },
  };

  /**
   * @param {AnyPossibleCohortId} datasetId
   * @returns {AnyDatasetPrefix}
   */
  function datasetIdToPrefix(datasetId) {
    return datasetId
      ? /** @type {const} */ (`${datasetId}-`)
      : /** @type {const} */ ("");
  }

  /**
   *
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {string} args.title
   * @param {Color} args.color
   * @param {Unit} args.unit
   * @param {AnyDatasetPath} [args.keySum]
   * @param {AnyDatasetPath} [args.keyAverage]
   * @param {AnyDatasetPath} [args.keyMax]
   * @param {AnyDatasetPath} [args.key90p]
   * @param {AnyDatasetPath} [args.key75p]
   * @param {AnyDatasetPath} [args.keyMedian]
   * @param {AnyDatasetPath} [args.key25p]
   * @param {AnyDatasetPath} [args.key10p]
   * @param {AnyDatasetPath} [args.keyMin]
   * @returns {PartialOptionsTree}
   */
  function createRecapOptions({
    scale,
    unit,
    title,
    keyAverage,
    color,
    keySum,
    keyMax,
    key90p,
    key75p,
    keyMedian,
    key25p,
    key10p,
    keyMin,
  }) {
    return [
      ...(keySum
        ? [
            {
              scale,
              name: "Daily Sum",
              title: `${title} Daily Sum`,
              description: "",
              unit,
              bottom: [
                {
                  title: "Sum",
                  color,
                  datasetPath: keySum,
                },
              ],
            },
          ]
        : []),
      ...(keyAverage
        ? [
            {
              scale,
              name: "Daily Average",
              title: `${title} Daily Average`,
              description: "",
              unit,
              bottom: [
                {
                  title: "Average",
                  color,
                  datasetPath: keyAverage,
                },
              ],
            },
          ]
        : []),
      ...(keyMax || key90p || key75p || keyMedian || key25p || key10p || keyMin
        ? [
            {
              scale,
              name: "Daily Percentiles",
              title: `${title} Daily Percentiles`,
              description: "",
              unit,
              bottom: [
                ...(keyMax
                  ? [
                      {
                        title: "Max",
                        color,
                        datasetPath: keyMax,
                      },
                    ]
                  : []),
                ...(key90p
                  ? [
                      {
                        title: "90%",
                        color,
                        datasetPath: key90p,
                      },
                    ]
                  : []),
                ...(key75p
                  ? [
                      {
                        title: "75%",
                        color,
                        datasetPath: key75p,
                      },
                    ]
                  : []),
                ...(keyMedian
                  ? [
                      {
                        title: "Median",
                        color,
                        datasetPath: keyMedian,
                      },
                    ]
                  : []),
                ...(key25p
                  ? [
                      {
                        title: "25%",
                        color,
                        datasetPath: key25p,
                      },
                    ]
                  : []),
                ...(key10p
                  ? [
                      {
                        title: "10%",
                        color,
                        datasetPath: key10p,
                      },
                    ]
                  : []),
                ...(keyMin
                  ? [
                      {
                        title: "Min",
                        color,
                        datasetPath: keyMin,
                      },
                    ]
                  : []),
              ],
            },
          ]
        : []),
      ...(keyMax
        ? [
            {
              scale,
              name: "Daily Max",
              title: `${title} Daily Max`,
              description: "",
              unit,
              bottom: [
                {
                  title: "Max",
                  color,
                  datasetPath: keyMax,
                },
              ],
            },
          ]
        : []),
      ...(key90p
        ? [
            {
              scale,
              name: "Daily 90th Percentile",
              title: `${title} Daily 90th Percentile`,
              description: "",
              unit,
              bottom: [
                {
                  title: "90%",
                  color,
                  datasetPath: key90p,
                },
              ],
            },
          ]
        : []),
      ...(key75p
        ? [
            {
              scale,
              name: "Daily 75th Percentile",
              title: `${title} Size 75th Percentile`,
              description: "",
              unit,
              bottom: [
                {
                  title: "75%",
                  color,
                  datasetPath: key75p,
                },
              ],
            },
          ]
        : []),
      ...(keyMedian
        ? [
            {
              scale,
              name: "Daily Median",
              title: `${title} Daily Median`,
              description: "",
              unit,
              bottom: [
                {
                  title: "Median",
                  color,
                  datasetPath: keyMedian,
                },
              ],
            },
          ]
        : []),
      ...(key25p
        ? [
            {
              scale,
              name: "Daily 25th Percentile",
              title: `${title} Daily 25th Percentile`,
              description: "",
              unit,
              bottom: [
                {
                  title: "25%",
                  color,
                  datasetPath: key25p,
                },
              ],
            },
          ]
        : []),
      ...(key10p
        ? [
            {
              scale,
              name: "Daily 10th Percentile",
              title: `${title} Daily 10th Percentile`,
              description: "",
              unit,
              bottom: [
                {
                  title: "10%",
                  color,
                  datasetPath: key10p,
                },
              ],
            },
          ]
        : []),
      ...(keyMin
        ? [
            {
              scale,
              name: "Daily Min",
              title: `${title} Daily Min`,
              description: "",
              unit,
              bottom: [
                {
                  title: "Min",
                  color,
                  datasetPath: keyMin,
                },
              ],
            },
          ]
        : []),
    ];
  }

  /**
   * @param {RatioOption | RatioOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createRatioOptions(arg) {
    const { scale, title } = arg;

    const isSingle = !("list" in arg);

    /**
     * @param {RatioOption | RatioOptions} arg
     */
    function toList(arg) {
      return "list" in arg ? arg.list : [arg];
    }

    /**
     * @param {RatioOption | RatioOptions} arg
     * @param {string} cohortName
     * @param {string} legendName
     */
    function toLegendName(arg, cohortName, legendName) {
      return "list" in arg ? `${cohortName} ${legendName}` : legendName;
    }

    return {
      name: "Ratio",
      tree: [
        {
          scale,
          name: "Basic",
          title: `Market Price To ${title} Ratio`,
          unit: "Ratio",
          description: "",
          top: toList(arg).map(
            ({ title, color, valueDatasetPath: datasetPath }) => ({
              title,
              color,
              datasetPath,
            }),
          ),
          bottom: [
            ...toList(arg).map(
              ({ title, color, ratioDatasetPath: datasetPath }) => ({
                title: toLegendName(arg, title, "Ratio"),
                color: isSingle ? undefined : color,
                type: /** @type {const} */ ("Baseline"),
                datasetPath,
                options: {
                  baseValue: {
                    price: 1,
                  },
                },
              }),
            ),
            bases[1](scale),
          ],
        },
        ...(isSingle
          ? /** @satisfies {PartialChartOption[]} */ ([
              {
                scale,
                name: "Averages",
                description: "",
                unit: "Ratio",
                title: `Market Price To ${title} Ratio Averages`,
                top: [
                  {
                    title,
                    color: arg.color,
                    datasetPath: arg.valueDatasetPath,
                  },
                ],
                bottom: [
                  {
                    title: `1Y SMA`,
                    color: colors.red,
                    datasetPath: /** @type {any} */ (
                      `${arg.ratioDatasetPath}-1y-sma`
                    ),
                  },
                  {
                    title: `1M SMA`,
                    color: colors.orange,
                    datasetPath: `${arg.ratioDatasetPath}-1m-sma`,
                  },
                  {
                    title: `1W SMA`,
                    color: colors.yellow,
                    datasetPath: `${arg.ratioDatasetPath}-1w-sma`,
                  },
                  {
                    title: `Raw`,
                    color: colors.default,
                    datasetPath: arg.ratioDatasetPath,
                  },
                  {
                    title: `Even`,
                    color: colors.off,
                    datasetPath: `${scale}-to-1`,
                    options: {
                      lineStyle: 3,
                      lastValueVisible: false,
                    },
                  },
                ],
              },
            ])
          : []),
        {
          scale,
          name: "Momentum Oscillator",
          title: `Market Price To ${title} Ratio 1Y SMA Momentum Oscillator`,
          description: "",
          unit: "Ratio",
          top: toList(arg).map(
            ({ title, color, valueDatasetPath: datasetPath }) => ({
              title: toLegendName(arg, title, ""),
              color,
              datasetPath,
            }),
          ),
          bottom: [
            ...toList(arg).map(
              ({ title, color, ratioDatasetPath: datasetPath }) => ({
                title: toLegendName(arg, title, "Momentum"),
                color: isSingle ? undefined : color,
                type: /** @type {const} */ ("Baseline"),
                datasetPath: /** @type {any} */ (
                  `${datasetPath}-1y-sma-momentum-oscillator`
                ),
              }),
            ),
            bases[0](scale),
          ],
        },
        {
          name: "Top",
          tree: [
            ...(isSingle
              ? /** @satisfies {PartialChartOption[]} */ ([
                  {
                    scale,
                    name: "Percentiles",
                    title: `Market Price To ${title} Ratio Top Percentiles`,
                    description: "",
                    unit: "Ratio",
                    top: [
                      {
                        title,
                        color: arg.color,
                        datasetPath: arg.valueDatasetPath,
                      },
                    ],
                    bottom: [
                      {
                        title: `99.9%`,
                        color: colors.probability0_1p,
                        datasetPath: /** @type {any} */ (
                          `${arg.ratioDatasetPath}-99-9p`
                        ),
                      },
                      {
                        title: `99.5%`,
                        color: colors.probability0_5p,
                        datasetPath: `${arg.ratioDatasetPath}-99-5p`,
                      },
                      {
                        title: `99%`,
                        color: colors.probability1p,
                        datasetPath: `${arg.ratioDatasetPath}-99p`,
                      },
                      {
                        title: `Raw`,
                        color: colors.default,
                        datasetPath: arg.ratioDatasetPath,
                      },
                    ],
                  },
                ])
              : []),
            {
              name: "Prices",
              tree: [
                ...(isSingle
                  ? /** @satisfies {PartialChartOption[]} */ ([
                      {
                        scale,
                        name: "All",
                        title: `${title} Top Prices`,
                        description: "",
                        unit: "US Dollars",
                        top: [
                          {
                            title: `99.9%`,
                            color: colors.probability0_1p,
                            datasetPath: /** @type {any} */ (
                              `${arg.valueDatasetPath}-99-9p`
                            ),
                          },
                          {
                            title: `99.5%`,
                            color: colors.probability0_5p,
                            datasetPath: `${arg.valueDatasetPath}-99-5p`,
                          },
                          {
                            title: `99%`,
                            color: colors.probability1p,
                            datasetPath: `${arg.valueDatasetPath}-99p`,
                          },
                        ],
                      },
                    ])
                  : []),
                {
                  scale,
                  name: "99%",
                  title: `${title} Top 99% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `99%`),
                    color: isSingle ? colors.probability1p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-99p`
                    ),
                  })),
                },
                {
                  scale,
                  name: "99.5%",
                  title: `${title} Top 99.5% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `99.5%`),
                    color: isSingle ? colors.probability0_5p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-99-5p`
                    ),
                  })),
                },
                {
                  scale,
                  name: "99.9%",
                  title: `${title} Top 99.9% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `99.9%`),
                    color: isSingle ? colors.probability0_1p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-99-9p`
                    ),
                  })),
                },
              ],
            },
          ],
        },
        {
          name: "Bottom",
          tree: [
            ...(isSingle
              ? /** @satisfies {PartialChartOption[]} */ ([
                  {
                    scale,
                    name: "Percentiles",
                    title: `Market Price To ${title} Ratio Bottom Percentiles`,
                    description: "",
                    unit: "Ratio",
                    top: [
                      {
                        title,
                        color: arg.color,
                        datasetPath: arg.valueDatasetPath,
                      },
                    ],
                    bottom: [
                      {
                        title: `0.1%`,
                        color: colors.probability0_1p,
                        datasetPath: `${arg.ratioDatasetPath}-0-1p`,
                      },
                      {
                        title: `0.5%`,
                        color: colors.probability0_5p,
                        datasetPath: `${arg.ratioDatasetPath}-0-5p`,
                      },
                      {
                        title: `1%`,
                        color: colors.probability1p,
                        datasetPath: /** @type {any} */ (
                          `${arg.ratioDatasetPath}-1p`
                        ),
                      },
                      {
                        title: `Raw`,
                        color: colors.default,
                        datasetPath: arg.ratioDatasetPath,
                      },
                    ],
                  },
                ])
              : []),
            {
              name: "Prices",
              tree: [
                ...(isSingle
                  ? /** @satisfies {PartialChartOption[]} */ ([
                      {
                        scale,
                        name: "All",
                        title: `${title} Bottom Prices`,
                        description: "",
                        unit: "US Dollars",
                        top: [
                          {
                            title: `0.1%`,
                            color: colors.probability0_1p,
                            datasetPath: /** @type {any} */ (
                              `${arg.valueDatasetPath}-0-1p`
                            ),
                          },
                          {
                            title: `0.5%`,
                            color: colors.probability0_5p,
                            datasetPath: `${arg.valueDatasetPath}-0-5p`,
                          },
                          {
                            title: `1%`,
                            color: colors.probability1p,
                            datasetPath: `${arg.valueDatasetPath}-1p`,
                          },
                        ],
                      },
                    ])
                  : []),
                {
                  scale,
                  name: "1%",
                  title: `${title} Bottom 1% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `1%`),
                    color: isSingle ? colors.probability1p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-1p`
                    ),
                  })),
                },
                {
                  scale,
                  name: "0.5%",
                  title: `${title} Bottom 0.5% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `0.5%`),
                    color: isSingle ? colors.probability0_5p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-0-5p`
                    ),
                  })),
                },
                {
                  scale,
                  name: "0.1%",
                  title: `${title} Bottom 0.1% Price`,
                  description: "",
                  unit: "US Dollars",
                  top: toList(arg).map((cohort) => ({
                    title: toLegendName(arg, cohort.title, `0.1%`),
                    color: isSingle ? colors.probability0_1p : cohort.color,
                    datasetPath: /** @type {any} */ (
                      `${cohort.valueDatasetPath}-0-1p`
                    ),
                  })),
                },
              ],
            },
          ],
        },
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createMarketOptions(scale) {
    /**
     * @param {TimeScale} scale
     * @returns {PartialOptionsGroup}
     */
    function createAllTimeHighOptions(scale) {
      return {
        name: "All Time High",
        tree: [
          {
            scale,
            name: "Value",
            title: "All Time High",
            description: "",
            unit: "US Dollars",
            top: [
              {
                title: `ATH`,
                color: colors.dollars,
                datasetPath: `${scale}-to-all-time-high`,
              },
            ],
          },
          {
            scale,
            name: "Drawdown",
            title: "All Time High Drawdown",
            description: "",
            unit: "Percentage",
            top: [
              {
                title: `ATH`,
                color: colors.dollars,
                datasetPath: `${scale}-to-all-time-high`,
              },
            ],
            bottom: [
              {
                title: `Drawdown`,
                color: colors.loss,
                datasetPath: `${scale}-to-drawdown`,
              },
            ],
          },
          ...(scale === "date"
            ? /** @type {PartialChartOption[]} */ ([
                {
                  scale,
                  name: "Days Since",
                  title: "Days Since All Time High",
                  description: "",
                  unit: "Count",
                  bottom: [
                    {
                      title: `Days`,
                      color: colors.red,
                      datasetPath: `date-to-days-since-all-time-high`,
                    },
                  ],
                },
                {
                  name: "Max Between",
                  tree: [
                    {
                      scale,
                      name: "Days",
                      title: "Max Number Of Days Between All Time Highs",
                      shortTitle: "Max Days Between",
                      description: "",
                      unit: "Count",
                      bottom: [
                        {
                          title: `Max`,
                          color: colors.red,
                          datasetPath:
                            "date-to-max-days-between-all-time-highs",
                        },
                      ],
                    },
                    {
                      scale,
                      name: "Years",
                      title: "Max Number Of Years Between All Time Highs",
                      shortTitle: "Max Years Between",
                      description: "",
                      bottom: [
                        {
                          title: `Max`,
                          color: colors.red,
                          datasetPath:
                            "date-to-max-years-between-all-time-highs",
                        },
                      ],
                    },
                  ],
                },
              ])
            : []),
        ],
      };
    }

    /**
     * @param {TimeScale} scale
     * @returns {PartialOptionsGroup}
     */
    function createAveragesOptions(scale) {
      return {
        name: "Averages",
        tree: [
          {
            scale,
            name: "All",
            title: "All Moving Averages",
            description: "",
            unit: "US Dollars",
            top: groups.averages.map((average) => ({
              title: average.key.toUpperCase(),
              color: colors[`_${average.key}`],
              datasetPath: `${scale}-to-price-${average.key}-sma`,
            })),
          },
          ...groups.averages.map(({ name, key }) =>
            createAverageOptions({
              scale,
              color: colors[`_${key}`],
              name,
              title: `${name} Market Price Moving Average`,
              key,
            }),
          ),
        ],
      };
    }

    /**
     *
     * @param {Object} args
     * @param {TimeScale} args.scale
     * @param {Color} args.color
     * @param {string} args.name
     * @param {string} args.title
     * @param {AverageName} args.key
     * @returns {PartialOptionsGroup}
     */
    function createAverageOptions({ scale, color, name, title, key }) {
      return {
        name,
        tree: [
          {
            scale,
            name: "Average",
            title,
            description: "",
            unit: "US Dollars",
            top: [
              {
                title: `SMA`,
                color,
                datasetPath: `${scale}-to-price-${key}-sma`,
              },
            ],
          },
          createRatioOptions({
            scale,
            color,
            ratioDatasetPath: `${scale}-to-market-price-to-price-${key}-sma-ratio`,
            valueDatasetPath: `${scale}-to-price-${key}-sma`,
            title,
          }),
        ],
      };
    }

    /**
     * @returns {PartialOptionsGroup}
     */
    function createReturnsOptions() {
      return {
        name: "Returns",
        tree: [
          {
            name: "Total",
            tree: [
              ...groups.totalReturns.map(({ name, key }) =>
                createReturnsOption({
                  scale: "date",
                  name,
                  title: `${name} Total`,
                  key: `${key}-total`,
                }),
              ),
            ],
          },
          {
            name: "Compound",
            tree: [
              ...groups.compoundReturns.map(({ name, key }) =>
                createReturnsOption({
                  scale: "date",
                  name,
                  title: `${name} Compound`,
                  key: `${key}-compound`,
                }),
              ),
            ],
          },
        ],
      };
    }

    /**
     * @param {Object} args
     * @param {TimeScale} args.scale
     * @param {string} args.name
     * @param {string} args.title
     * @param {`${TotalReturnKey}-total` | `${CompoundReturnKey}-compound`} args.key
     * @returns {PartialChartOption}
     */
    function createReturnsOption({ scale, name, title, key }) {
      return {
        scale,
        name,
        description: "",
        title: `${title} Return`,
        unit: "Percentage",
        bottom: [
          {
            title: `Return`,
            type: "Baseline",
            datasetPath: `date-to-price-${key}-return`,
          },
          bases[0](scale),
        ],
      };
    }

    /**
     * @returns {PartialOptionsGroup}
     */
    function createIndicatorsOptions() {
      return {
        name: "Indicators",
        tree: [],
      };
    }

    return {
      name: "Market",
      tree: [
        {
          name: "Price",
          tree: [
            {
              scale,
              name: "Dollars Per Bitcoin",
              title: "Dollars Per Bitcoin",
              description: "",
              unit: "US Dollars",
            },
            {
              scale,
              name: "Sats Per Dollar",
              title: "Satoshis Per Dollar",
              description: "",
              unit: "Satoshis",
              bottom: [
                {
                  title: "Sats",
                  datasetPath: `${scale}-to-sats-per-dollar`,
                  color: colors.bitcoin,
                },
              ],
            },
          ],
        },
        {
          scale,
          name: "Capitalization",
          title: "Market Capitalization",
          description: "",
          unit: "US Dollars",
          bottom: [
            {
              title: "Capitalization",
              datasetPath: `${scale}-to-market-cap`,
              color: colors.bitcoin,
            },
          ],
        },
        createAllTimeHighOptions(scale),
        createAveragesOptions(scale),
        ...(scale === "date"
          ? [createReturnsOptions(), createIndicatorsOptions()]
          : []),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createBlocksOptions(scale) {
    return {
      name: "Blocks",
      tree: [
        ...(scale === "date"
          ? /** @type {PartialOptionsTree} */ ([
              {
                scale,
                name: "Height",
                title: "Block Height",
                description: "",
                unit: "Height",
                bottom: [
                  {
                    title: "Height",
                    color: colors.bitcoin,
                    datasetPath: `date-to-last-height`,
                  },
                ],
              },
              {
                scale,
                name: "Mined",
                tree: [
                  {
                    scale,
                    name: "Daily Sum",
                    title: "Daily Sum Of Blocks Mined",
                    description: "",
                    unit: "Count",
                    bottom: [
                      {
                        title: "Target",
                        color: colors.off,
                        datasetPath: `date-to-blocks-mined-1d-target`,
                        options: {
                          lineStyle: 3,
                        },
                      },
                      {
                        title: "1W Avg.",
                        color: colors.momentumYellow,
                        datasetPath: `date-to-blocks-mined-1w-sma`,
                        defaultActive: false,
                      },
                      {
                        title: "1M Avg.",
                        color: colors.bitcoin,
                        datasetPath: `date-to-blocks-mined-1m-sma`,
                      },
                      {
                        title: "Mined",
                        color: colors.darkBitcoin,
                        datasetPath: `date-to-blocks-mined`,
                        main: true,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Weekly Sum",
                    title: "Weekly Sum Of Blocks Mined",
                    description: "",
                    unit: "Count",
                    bottom: [
                      {
                        title: "Target",
                        color: colors.off,
                        datasetPath: `date-to-blocks-mined-1w-target`,
                        options: {
                          lineStyle: 3,
                        },
                      },
                      {
                        title: "Sum Mined",
                        color: colors.bitcoin,
                        datasetPath: `date-to-blocks-mined-1w-sum`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Monthly Sum",
                    title: "Monthly Sum Of Blocks Mined",
                    description: "",
                    unit: "Count",
                    bottom: [
                      {
                        title: "Target",
                        color: colors.off,
                        datasetPath: `date-to-blocks-mined-1m-target`,
                        options: {
                          lineStyle: 3,
                        },
                      },
                      {
                        title: "Sum Mined",
                        color: colors.bitcoin,
                        datasetPath: `date-to-blocks-mined-1m-sum`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly Sum",
                    title: "Yearly Sum Of Blocks Mined",
                    description: "",
                    unit: "Count",
                    bottom: [
                      {
                        title: "Target",
                        color: colors.off,
                        datasetPath: `date-to-blocks-mined-1y-target`,
                        options: {
                          lineStyle: 3,
                        },
                      },
                      {
                        title: "Sum Mined",
                        color: colors.bitcoin,
                        datasetPath: `date-to-blocks-mined-1y-sum`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Total",
                    title: "Total Blocks Mined",
                    description: "",
                    unit: "Count",
                    bottom: [
                      {
                        title: "Mined",
                        color: colors.bitcoin,
                        datasetPath: `date-to-total-blocks-mined`,
                      },
                    ],
                  },
                ],
              },
              {
                scale,
                name: "Size",
                tree: [
                  {
                    scale,
                    name: "Cumulative",
                    title: "Cumulative Block Size",
                    description: "",
                    unit: "Gigabytes",
                    bottom: [
                      {
                        title: "Size",
                        color: colors.off,
                        datasetPath: `${scale}-to-cumulative-block-size-gigabytes`,
                      },
                    ],
                  },
                  ...createRecapOptions({
                    scale,
                    title: "Block Size",
                    color: colors.off,
                    unit: "Megabytes",
                    keySum: "date-to-block-size-1d-sum",
                    keyAverage: "date-to-block-size-1d-average",
                    keyMax: "date-to-block-size-1d-max",
                    key90p: "date-to-block-size-1d-90p",
                    key75p: "date-to-block-size-1d-75p",
                    keyMedian: "date-to-block-size-1d-median",
                    key25p: "date-to-block-size-1d-25p",
                    key10p: "date-to-block-size-1d-10p",
                    keyMin: "date-to-block-size-1d-min",
                  }),
                ],
              },
              {
                scale,
                name: "Weight",
                tree: createRecapOptions({
                  scale,
                  title: "Block Weight",
                  color: colors.off,
                  unit: "Weight",
                  keyAverage: "date-to-block-weight-1d-average",
                  keyMax: "date-to-block-weight-1d-max",
                  key90p: "date-to-block-weight-1d-90p",
                  key75p: "date-to-block-weight-1d-75p",
                  keyMedian: "date-to-block-weight-1d-median",
                  key25p: "date-to-block-weight-1d-25p",
                  key10p: "date-to-block-weight-1d-10p",
                  keyMin: "date-to-block-weight-1d-min",
                }),
              },
              {
                scale,
                name: "VBytes",
                tree: createRecapOptions({
                  scale,
                  title: "Block VBytes",
                  color: colors.off,
                  unit: "Virtual Bytes",
                  keyAverage: "date-to-block-vbytes-1d-average",
                  keyMax: "date-to-block-vbytes-1d-max",
                  key90p: "date-to-block-vbytes-1d-90p",
                  key75p: "date-to-block-vbytes-1d-75p",
                  keyMedian: "date-to-block-vbytes-1d-median",
                  key25p: "date-to-block-vbytes-1d-25p",
                  key10p: "date-to-block-vbytes-1d-10p",
                  keyMin: "date-to-block-vbytes-1d-min",
                }),
              },
              {
                scale,
                name: "Interval",
                tree: createRecapOptions({
                  scale,
                  title: "Block Interval",
                  color: colors.off,
                  unit: "Seconds",
                  keyAverage: "date-to-block-interval-1d-average",
                  keyMax: "date-to-block-interval-1d-max",
                  key90p: "date-to-block-interval-1d-90p",
                  key75p: "date-to-block-interval-1d-75p",
                  keyMedian: "date-to-block-interval-1d-median",
                  key25p: "date-to-block-interval-1d-25p",
                  key10p: "date-to-block-interval-1d-10p",
                  keyMin: "date-to-block-interval-1d-min",
                }),
              },
            ])
          : /** @type {PartialOptionsTree} */ ([
              {
                scale,
                name: "Size",
                title: "Block Size",
                description: "",
                unit: "Megabytes",
                bottom: [
                  {
                    title: "Size",
                    color: colors.off,
                    datasetPath: `height-to-block-size`,
                  },
                ],
              },
              {
                scale,
                name: "Weight",
                title: "Block Weight",
                description: "",
                unit: "Weight",
                bottom: [
                  {
                    title: "Weight",
                    color: colors.off,
                    datasetPath: `height-to-block-weight`,
                  },
                ],
              },
              {
                scale,
                name: "VBytes",
                title: "Block VBytes",
                description: "",
                unit: "Virtual Bytes",
                bottom: [
                  {
                    title: "VBytes",
                    color: colors.off,
                    datasetPath: `height-to-block-vbytes`,
                  },
                ],
              },
              {
                scale,
                name: "Interval",
                title: "Block Interval",
                description: "",
                unit: "Seconds",
                bottom: [
                  {
                    title: "Interval",
                    color: colors.off,
                    datasetPath: `height-to-block-interval`,
                  },
                ],
              },
            ])),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createMinersOptions(scale) {
    return {
      name: "Miners",
      tree: [
        {
          name: "Coinbases",
          tree: [
            ...(scale === "date"
              ? /** @satisfies {PartialOptionsTree} */ ([
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Daily Sum Of Coinbases In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-coinbase-1d-sum`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Daily Sum Of Coinbases In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-coinbase-in-dollars-1d-sum`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Yearly Sum Of Coinbases In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-coinbase-1y-sum`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Yearly Sum Of Coinbases In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-coinbase-in-dollars-1y-sum`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Cumulative",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Cumulative Coinbases In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Coinbases",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-cumulative-coinbase`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Cumulative Coinbases In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Coinbases",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-cumulative-coinbase-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    name: "Last Block",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Last Coinbase In Bitcoin",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-last-coinbase`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Last Coinbase In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-last-coinbase-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                ])
              : []),
          ],
        },

        {
          name: "Subsidies",
          tree: [
            ...(scale === "date"
              ? /** @type {PartialOptionsTree} */ ([
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Daily Sum Of Subsidies In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-subsidy`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Daily Sum Of Subsidies In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-subsidy-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Yearly Sum Of Subsidies In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-subsidy-1y-sum`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Yearly Sum Of Subsidies In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-subsidy-in-dollars-1y-sum`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Cumulative",

                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Cumulative Subsidies In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Subsidies",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-cumulative-subsidy`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Cumulative Subsidies In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Subsidies",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-cumulative-subsidy-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    name: "Last Block",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Last Subsidy In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-last-subsidy`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Last Subsidy In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-last-subsidy-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                ])
              : []),
          ],
        },

        {
          name: "Fees",
          tree: [
            ...(scale === "date"
              ? /** @type {PartialOptionsTree} */ ([
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Daily Sum Of Fees In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-fees`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Daily Sum Of Fees In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-fees-in-dollars-1d-sum`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly Sum",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Yearly Sum Of Fees In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-fees-1y-sum`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Yearly Sum Of Fees In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-fees-in-dollars-1y-sum`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Cumulative",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Cumulative Fees In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Fees",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-cumulative-fees`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Cumulative Fees In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Fees",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-cumulative-fees-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                  {
                    name: "Last Block",
                    tree: [
                      {
                        scale,
                        name: "In Bitcoin",
                        title: "Last Fees In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-last-fees`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "In Dollars",
                        title: "Last Fees In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Last",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-last-fees-in-dollars`,
                          },
                        ],
                      },
                    ],
                  },
                ])
              : []),
          ],
        },

        {
          scale,
          name: "Subsidy V. Fees",
          title: "Subsidy V. Fees",
          description: "",
          unit: "Percentage",
          bottom: [
            {
              title: "Subsidy",
              color: colors.bitcoin,
              datasetPath:
                scale === "date"
                  ? `${scale}-to-subsidy-to-coinbase-1d-ratio`
                  : `${scale}-to-subsidy-to-coinbase-ratio`,
            },
            {
              title: "Fees",
              color: colors.darkBitcoin,
              datasetPath:
                scale === "date"
                  ? `${scale}-to-fees-to-coinbase-1d-ratio`
                  : `${scale}-to-fees-to-coinbase-ratio`,
            },
          ],
        },

        ...(scale === "date"
          ? /** @type {PartialOptionsTree} */ ([
              {
                scale,
                name: "Puell Multiple",
                title: "Puell Multiple",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Multiple",
                    color: colors.bitcoin,
                    datasetPath: `date-to-puell-multiple`,
                  },
                ],
              },

              {
                name: "Hash",
                tree: [
                  {
                    scale,
                    name: "Rate",
                    title: "Hash Rate",
                    description: "",
                    unit: "ExaHash / Second",
                    bottom: [
                      {
                        title: "1M SMA",
                        color: colors.momentumYellow,
                        datasetPath: `date-to-hash-rate-1m-sma`,
                      },
                      {
                        title: "1W SMA",
                        color: colors.bitcoin,
                        datasetPath: `date-to-hash-rate-1w-sma`,
                      },
                      {
                        title: "Rate",
                        color: colors.darkBitcoin,
                        datasetPath: `date-to-hash-rate`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Ribbon",
                    title: "Hash Ribbon",
                    description: "",
                    unit: "ExaHash / Second",
                    bottom: [
                      {
                        title: "1M SMA",
                        color: colors.profit,
                        datasetPath: `date-to-hash-rate-1m-sma`,
                      },
                      {
                        title: "2M SMA",
                        color: colors.loss,
                        datasetPath: `date-to-hash-rate-2m-sma`,
                      },
                    ],
                  },
                  {
                    name: "Price",
                    tree: [
                      {
                        scale,
                        name: "Price",
                        title: "Hash Price",
                        description: "",
                        unit: "Dollars / (PetaHash / Second)",
                        bottom: [
                          {
                            title: "Hash Price",
                            color: colors.dollars,
                            datasetPath: `date-to-hash-price`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "Min",
                        title: "Min Hash Price",
                        description: "",
                        unit: "Dollars / (PetaHash / Second)",
                        bottom: [
                          {
                            title: "Min Hash Price",
                            color: colors.dollars,
                            datasetPath: `date-to-hash-price-min`,
                          },
                        ],
                      },
                      {
                        scale,
                        name: "Rebound",
                        title: "Hash Price Rebound",
                        description: "",
                        unit: "Percentage",
                        bottom: [
                          {
                            title: "Rebound",
                            color: colors.yellow,
                            datasetPath: `date-to-hash-price-rebound`,
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
            ])
          : []),

        {
          scale,
          name: "Difficulty",
          title: "Difficulty",
          description: "",
          unit: "",
          bottom: [
            {
              title: "Difficulty",
              color: colors.bitcoin,
              datasetPath: `${scale}-to-difficulty`,
            },
          ],
        },

        ...(scale === "date"
          ? /** @type {PartialOptionsTree} */ ([
              {
                scale,
                name: "Difficulty Adjustment",
                title: "Difficulty Adjustment",
                description: "",
                unit: "Percentage",
                bottom: [
                  {
                    title: "Adjustment",
                    type: "Baseline",
                    datasetPath: `${scale}-to-difficulty-adjustment`,
                  },
                  bases[0](scale),
                ],
              },
              {
                scale,
                name: "Annualized Issuance",
                title: "Annualized Issuance",
                description: "",
                unit: "Bitcoin",
                bottom: [
                  {
                    title: "Issuance",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-annualized-issuance`,
                  },
                ],
              },
              {
                name: "Inflation Rate",
                tree: [
                  {
                    scale,
                    name: "Today",
                    title: "Inflation Rate",
                    description: "",
                    unit: "Percentage",
                    bottom: [
                      {
                        title: "Rate",
                        color: colors.bitcoin,
                        datasetPath: `${scale}-to-inflation-rate`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly",
                    title: "Yearly Inflation Rate",
                    description: "",
                    unit: "Percentage",
                    bottom: [
                      {
                        title: "Rate",
                        color: colors.bitcoin,
                        datasetPath: `${scale}-to-yearly-inflation-rate`,
                      },
                    ],
                  },
                ],
              },
            ])
          : []),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createTransactionsOptions(scale) {
    return {
      name: "Transactions",
      tree: [
        {
          scale,
          name: "Count",
          title: "Transaction Count",
          description: "",
          unit: "Count",
          bottom: [
            {
              title: "1M SMA",
              color: colors.momentumYellow,
              datasetPath:
                scale === "date"
                  ? `${scale}-to-transaction-count-1d-sum-1m-sma`
                  : `${scale}-to-transaction-count-1m-sma`,
            },
            {
              title: "1W SMA",
              color: colors.bitcoin,
              datasetPath:
                scale === "date"
                  ? `${scale}-to-transaction-count-1d-sum-1w-sma`
                  : `${scale}-to-transaction-count-1w-sma`,
            },
            {
              title: "Raw",
              color: colors.darkBitcoin,
              datasetPath:
                scale === "date"
                  ? `${scale}-to-transaction-count-1d-sum`
                  : `${scale}-to-transaction-count`,
              main: true,
            },
          ],
        },

        {
          name: "Volume",
          tree: [
            {
              scale,
              name: "In Bitcoin",
              title: "Transaction Volume",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "1M SMA",
                  color: colors.momentumYellow,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-1d-sum-1m-sma`
                      : `${scale}-to-transaction-volume-1m-sma`,
                },
                {
                  title: "1W SMA",
                  color: colors.bitcoin,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-1d-sum-1w-sma`
                      : `${scale}-to-transaction-volume-1w-sma`,
                },
                {
                  title: "Raw",
                  color: colors.darkBitcoin,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-1d-sum`
                      : `${scale}-to-transaction-volume`,
                },
              ],
            },
            {
              scale,
              name: "In Dollars",
              title: "Transaction Volume In Dollars",
              description: "",
              unit: "US Dollars",
              bottom: [
                {
                  title: "1M SMA",
                  color: colors.lightDollars,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-in-dollars-1d-sum-1m-sma`
                      : `${scale}-to-transaction-volume-in-dollars-1m-sma`,
                },
                {
                  title: "1W SMA",
                  color: colors.dollars,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-in-dollars-1d-sum-1w-sma`
                      : `${scale}-to-transaction-volume-in-dollars-1w-sma`,
                },
                {
                  title: "Raw",
                  color: colors.darkDollars,
                  datasetPath:
                    scale === "date"
                      ? `${scale}-to-transaction-volume-in-dollars-1d-sum`
                      : `${scale}-to-transaction-volume-in-dollars`,
                },
              ],
            },
          ],
        },

        ...(scale === "date"
          ? /** @type {PartialOptionsTree} */ ([
              {
                name: "Annualized Volume",
                tree: [
                  {
                    scale,
                    name: "In Bitcoin",
                    title: "Annualized Transaction Volume",
                    description: "",
                    unit: "Bitcoin",
                    bottom: [
                      {
                        title: "Volume",
                        color: colors.bitcoin,
                        datasetPath: `date-to-annualized-transaction-volume`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "In Dollars",
                    title: "Annualized Transaction Volume In Dollars",
                    description: "",
                    unit: "US Dollars",
                    bottom: [
                      {
                        title: "Volume",
                        color: colors.dollars,
                        datasetPath: `date-to-annualized-transaction-volume-in-dollars`,
                      },
                    ],
                  },
                ],
              },
              {
                scale,
                name: "Velocity",
                title: "Transactions Velocity",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Transactions Velocity",
                    color: colors.bitcoin,
                    datasetPath: `date-to-transaction-velocity`,
                  },
                ],
              },
            ])
          : []),
        {
          scale,
          name: "Per Second",
          title: "Transactions Per Second",
          description: "",
          unit: "Transactions",
          bottom: [
            {
              title: "1M SMA",
              color: colors.lightBitcoin,
              datasetPath: `${scale}-to-transactions-per-second-1m-sma`,
            },
            {
              title: "1W SMA",
              color: colors.bitcoin,
              datasetPath: `${scale}-to-transactions-per-second-1w-sma`,
            },
            {
              title: "Raw",
              color: colors.darkBitcoin,
              datasetPath: `${scale}-to-transactions-per-second`,
              main: true,
            },
          ],
        },
      ],
    };
  }

  const cohortOptionOrOptions = {
    /**
     * @param {DefaultCohortOption | DefaultCohortOptions} arg
     */
    toList(arg) {
      return "list" in arg ? arg.list : [arg];
    },
    /**
     * @param {DefaultCohortOption | DefaultCohortOptions} arg
     * @param {string} cohortName
     * @param {string} legendName
     */
    toLegendName(arg, cohortName, legendName) {
      return "list" in arg ? `${cohortName} ${legendName}` : legendName;
    },
    /**
     * @template {AnyPossibleCohortId} T
     * @param {CohortOption<T> | CohortOptions<T>} arg
     * @param {SeriesBlueprintParam<T> & Omit<AnySpecificSeriesBlueprint, 'color'>} blueprint
     */
    toSeriesBlueprints(arg, blueprint) {
      return this.toList(arg).map(
        ({ scale, datasetId, color, name }) =>
          /** @satisfies {SeriesBlueprint} */ ({
            title: cohortOptionOrOptions.toLegendName(
              arg,
              name,
              blueprint.title,
            ),
            color: "list" in arg ? color : blueprint.singleColor || color,
            type: blueprint.type || "Line",
            // Don't get why TS is annoying here
            // @ts-ignore
            datasetPath: blueprint.genPath(datasetId, scale),
            options: blueprint.options,
          }),
      );
    },
    /**
     * @param {DefaultCohortOption | DefaultCohortOptions} arg
     */
    shouldShowAll(arg) {
      return "list" in arg || arg.datasetId;
    },
    /**
     * @param {DefaultCohortOption | DefaultCohortOptions} arg
     * @param {(prefix: AnyDatasetPrefix, arg: DefaultCohortOption) => PartialChartOption[]} genOption
     */
    genOptionsIfSingle(arg, genOption) {
      if (!("list" in arg)) {
        const prefix = datasetIdToPrefix(arg.datasetId);
        return genOption(prefix, arg);
      } else {
        return [];
      }
    },
    /**
     * @param {DefaultCohortOption | DefaultCohortOptions} arg
     * @param {(scale: 'date') => PartialChartOption[]} genOption
     */
    genOptionsIfDate(arg, genOption) {
      if (arg.scale === "date") {
        return genOption("date");
      } else {
        return [];
      }
    },
  };

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortUTXOOptions(arg) {
    const { scale, title } = arg;

    return {
      name: "UTXOs",
      tree: [
        {
          scale,
          name: `Count`,
          title: `Number Of ${title} Unspent Transaction Outputs`,
          description: "",
          unit: "Count",
          bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Count",
            genPath: (id, scale) =>
              /** @type {const} */ (
                `${scale}-to-${datasetIdToPrefix(id)}utxo-count`
              ),
          }),
        },
      ],
    };
  }

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortRealizedOptions(arg) {
    const { scale, title } = arg;

    /**
     * @param {DefaultCohortOption} arg
     * @returns {RatioOption}
     */
    function argToRatioArg(arg) {
      return {
        scale,
        color: arg.color,
        ratioDatasetPath: `${scale}-to-market-price-to-${datasetIdToPrefix(
          arg.datasetId,
        )}realized-price-ratio`,
        valueDatasetPath: `${scale}-to-${datasetIdToPrefix(
          arg.datasetId,
        )}realized-price`,
        title: `${arg.title} Realized Price`,
      };
    }

    return {
      name: "Realized",
      tree: [
        {
          scale,
          name: `Price`,
          title: `${title} Realized Price`,
          description: "",
          unit: "US Dollars",
          top: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Realized Price",
            genPath: (id, scale) =>
              /** @type {const} */ (
                `${scale}-to-${datasetIdToPrefix(id)}realized-price`
              ),
          }),
        },
        createRatioOptions(
          "list" in arg
            ? {
                ...arg,
                list: arg.list.map(argToRatioArg),
              }
            : argToRatioArg(arg),
        ),
        {
          scale,
          name: `Capitalization`,
          title: `${title} Realized Capitalization`,
          description: "",
          unit: "US Dollars",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(
              arg,

              {
                title: "Realized Cap.",
                genPath: (id, scale) =>
                  /** @type {const} */ (
                    `${scale}-to-${datasetIdToPrefix(id)}realized-cap`
                  ),
              },
            ),
            ...(cohortOptionOrOptions.shouldShowAll(arg)
              ? /** @type {const} */ ([
                  {
                    title: "Realized Cap.",
                    color: colors.bitcoin,
                    datasetPath: `${scale}-to-realized-cap`,
                    defaultActive: false,
                  },
                ])
              : []),
          ],
        },
        {
          scale,
          name: `Capitalization 1M Net Change`,
          title: `${title} Realized Capitalization 1 Month Net Change`,
          description: "",
          unit: "US Dollars",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(
              arg,

              {
                title: "Net Change",
                type: "Baseline",
                genPath: (id, scale) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}realized-cap-1m-net-change`,
              },
            ),
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Profit`,
          title: `${title} Realized Profit`,
          description: "",
          unit: "US Dollars",
          bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Realized Profit",
            singleColor: colors.profit,
            genPath: (id, scale) => {
              const prefix = datasetIdToPrefix(id);
              return scale === "date"
                ? `date-to-${prefix}realized-profit-1d-sum`
                : `height-to-${prefix}realized-profit`;
            },
          }),
        },
        {
          scale,
          name: "Loss",
          title: `${title} Realized Loss`,
          description: "",
          unit: "US Dollars",
          bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Realized Loss",
            singleColor: colors.loss,
            genPath: (id, scale) => {
              const prefix = datasetIdToPrefix(id);
              return scale === "date"
                ? `date-to-${prefix}realized-loss-1d-sum`
                : `height-to-${prefix}realized-loss`;
            },
          }),
        },
        ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix) => [
          {
            scale,
            name: `PNL - Profit And Loss`,
            title: `${title} Realized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            bottom: [
              {
                title: "Profit",
                color: colors.profit,
                datasetPath:
                  scale === "date"
                    ? `date-to-${prefix}realized-profit-1d-sum`
                    : `height-to-${prefix}realized-profit`,
                type: "Baseline",
              },
              {
                title: "Loss",
                color: colors.loss,
                datasetPath:
                  scale === "date"
                    ? `date-to-${prefix}negative-realized-loss-1d-sum`
                    : `height-to-${prefix}negative-realized-loss`,
                type: "Baseline",
              },
              bases[0](scale),
            ],
          },
        ]),
        {
          scale,
          name: `Net PNL - Net Profit And Loss`,
          title: `${title} Net Realized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
              title: "Net PNL",
              type: "Baseline",
              genPath: (id, scale) => {
                const prefix = datasetIdToPrefix(id);
                return scale === "date"
                  ? `date-to-${prefix}net-realized-profit-and-loss-1d-sum`
                  : `height-to-${prefix}net-realized-profit-and-loss`;
              },
            }),
            bases[0](scale),
          ],
        },
        ...cohortOptionOrOptions.genOptionsIfDate(arg, (scale) => [
          {
            scale,
            name: `Net PNL Relative To Market Cap`,
            title: `${title} Net Realized Profit And Loss Relative To Market Capitalization`,
            description: "",
            unit: "Percentage",
            bottom: [
              ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Net",
                type: "Baseline",
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}net-realized-profit-and-loss-to-market-cap-ratio`,
              }),
              bases[0](scale),
            ],
          },
        ]),
        {
          name: "Cumulative",
          tree: [
            {
              scale,
              name: `Profit`,
              title: `${title} Cumulative Realized Profit`,
              description: "",
              unit: "US Dollars",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Cumulative Realized Profit",
                singleColor: colors.profit,
                genPath: (id, scale) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}cumulative-realized-profit`,
              }),
            },
            {
              scale,
              name: "Loss",
              title: `${title} Cumulative Realized Loss`,
              description: "",
              unit: "US Dollars",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Cumulative Realized Loss",
                singleColor: colors.loss,
                genPath: (id, scale) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}cumulative-realized-loss`,
              }),
            },
            {
              scale,
              name: `Net PNL`,
              title: `${title} Cumulative Net Realized Profit And Loss`,
              description: "",
              unit: "US Dollars",
              bottom: [
                ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
                  title: "Cumulative Net Realized PNL",
                  type: "Baseline",
                  genPath: (id, scale) =>
                    `${scale}-to-${datasetIdToPrefix(
                      id,
                    )}cumulative-net-realized-profit-and-loss`,
                }),
                bases[0](scale),
              ],
            },
            {
              scale,
              name: `Net PNL 30 Day Change`,
              title: `${title} Cumulative Net Realized Profit And Loss 30 Day Change`,
              description: "",
              unit: "US Dollars",
              bottom: [
                ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
                  title: "Cumulative Net Realized PNL 30d Change",
                  type: "Baseline",
                  genPath: (id, scale) =>
                    `${scale}-to-${datasetIdToPrefix(
                      id,
                    )}cumulative-net-realized-profit-and-loss-1m-net-change`,
                }),
                bases[0](scale),
              ],
            },
          ],
        },
        {
          scale,
          name: "Profit To Loss Ratio",
          title: `${title} Profit To Loss Ratio`,
          description: "",
          unit: "Ratio",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
              title: "Ratio",
              type: "Baseline",
              options: {
                baseValue: {
                  price: 1,
                },
              },
              genPath: (id, scale) => {
                const prefix = datasetIdToPrefix(id);
                return scale === "date"
                  ? `date-to-${prefix}realized-profit-to-loss-1d-sum-ratio`
                  : `height-to-${prefix}realized-profit-to-loss-ratio`;
              },
            }),
            bases[1](scale, "Even"),
          ],
        },
        {
          name: `Spent Output Profit Ratio`,
          tree: [
            {
              scale,
              name: `Normal - SOPR`,
              title: `${title} Spent Output Profit Ratio`,
              description: "",
              unit: "Percentage",
              bottom: [
                ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
                  title: "SOPR",
                  type: "Baseline",
                  options: {
                    baseValue: {
                      price: 1,
                    },
                  },
                  genPath: (id, scale) =>
                    `${scale}-to-${datasetIdToPrefix(
                      id,
                    )}spent-output-profit-ratio`,
                }),
                bases[1](scale),
              ],
            },
            {
              scale,
              name: `Adjusted - aSOPR`,
              title: `${title} Adjusted Spent Output Profit Ratio`,
              description: "",
              unit: "Percentage",
              bottom: [
                ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
                  title: "aSOPR",
                  type: "Baseline",
                  options: {
                    baseValue: {
                      price: 1,
                    },
                  },
                  genPath: (id, scale) =>
                    `${scale}-to-${datasetIdToPrefix(
                      id,
                    )}adjusted-spent-output-profit-ratio`,
                }),
                bases[1](scale),
              ],
            },
          ],
        },
        {
          name: "Value",
          tree: [
            {
              name: "Created",
              tree: [
                {
                  scale,
                  name: `Normal`,
                  title: `${title} Value Created`,
                  description: "",
                  unit: "US Dollars",
                  bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                    title: "Value",
                    singleColor: colors.profit,
                    genPath: (id, scale) => {
                      const prefix = datasetIdToPrefix(id);
                      return scale === "date"
                        ? `date-to-${prefix}value-created-1d-sum`
                        : `height-to-${prefix}value-created`;
                    },
                  }),
                },
                {
                  scale,
                  name: `Adjusted`,
                  title: `${title} Adjusted Value Created`,
                  description: "",
                  unit: "US Dollars",
                  bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                    title: "Adjusted Value",
                    singleColor: colors.profit,
                    genPath: (id, scale) => {
                      const prefix = datasetIdToPrefix(id);
                      return scale === "date"
                        ? `date-to-${prefix}adjusted-value-created-1d-sum`
                        : `height-to-${prefix}adjusted-value-created`;
                    },
                  }),
                },
              ],
            },
            {
              name: "Destroyed",
              tree: [
                {
                  scale,
                  name: `Normal`,
                  title: `${title} Value Destroyed`,
                  description: "",
                  unit: "US Dollars",
                  bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                    title: "Value",
                    singleColor: colors.loss,
                    genPath: (id, scale) => {
                      const prefix = datasetIdToPrefix(id);
                      return scale === "date"
                        ? `date-to-${prefix}value-destroyed-1d-sum`
                        : `height-to-${prefix}value-destroyed`;
                    },
                  }),
                },
                {
                  scale,
                  name: `Adjusted`,
                  title: `${title} Adjusted Value Destroyed`,
                  description: "",
                  unit: "US Dollars",
                  bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                    title: "Adjusted Value",
                    singleColor: colors.loss,
                    genPath: (id, scale) => {
                      const prefix = datasetIdToPrefix(id);
                      return scale === "date"
                        ? `date-to-${prefix}adjusted-value-destroyed-1d-sum`
                        : `height-to-${prefix}adjusted-value-destroyed`;
                    },
                  }),
                },
              ],
            },
          ],
        },
        ...cohortOptionOrOptions.genOptionsIfDate(arg, (scale) => [
          /** @satisfies {PartialChartOption} */ ({
            scale,
            name: `Sell Side Risk Ratio`,
            title: `${title} Sell Side Risk Ratio`,
            description: "",
            unit: "Percentage",
            bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
              title: "Ratio",
              genPath: (id) =>
                `${scale}-to-${datasetIdToPrefix(id)}sell-side-risk-ratio`,
            }),
          }),
        ]),
      ],
    };
  }

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortUnrealizedOptions(arg) {
    const { scale, title } = arg;

    return {
      name: "Unrealized",
      tree: [
        {
          scale,
          name: `Profit`,
          title: `${title} Unrealized Profit`,
          description: "",
          unit: "US Dollars",
          bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Profit",
            genPath: (id) =>
              `${scale}-to-${datasetIdToPrefix(id)}unrealized-profit`,
            singleColor: colors.profit,
          }),
        },
        {
          scale,
          name: "Loss",
          title: `${title} Unrealized Loss`,
          description: "",
          unit: "US Dollars",
          bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Loss",
            genPath: (id) =>
              `${scale}-to-${datasetIdToPrefix(id)}unrealized-loss`,
            singleColor: colors.loss,
          }),
        },
        ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix) => [
          {
            scale,
            name: `PNL - Profit And Loss`,
            title: `${title} Unrealized Profit And Loss`,
            description: "",
            unit: "US Dollars",
            bottom: [
              {
                title: "Profit",
                color: colors.profit,
                datasetPath: `${scale}-to-${prefix}unrealized-profit`,
                type: "Baseline",
              },
              {
                title: "Loss",
                color: colors.loss,
                datasetPath: `${scale}-to-${prefix}negative-unrealized-loss`,
                type: "Baseline",
              },
              bases[0](scale),
            ],
          },
        ]),
        {
          scale,
          name: `Net PNL - Net Profit And Loss`,
          title: `${title} Net Unrealized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
              title: "Net Unrealized PNL",
              genPath: (id) =>
                `${scale}-to-${datasetIdToPrefix(
                  id,
                )}net-unrealized-profit-and-loss`,
              type: "Baseline",
            }),
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Net PNL Relative To Market Cap - NUPL`,
          title: `${title} Net Unrealized Profit And Loss Relative To Total Market Capitalization - NUPL`,
          description: "",
          unit: "Percentage",
          bottom: [
            ...cohortOptionOrOptions.toSeriesBlueprints(arg, {
              title: "Relative Net Unrealized PNL",
              genPath: (id) =>
                `${scale}-to-${datasetIdToPrefix(
                  id,
                )}net-unrealized-profit-and-loss-to-market-cap-ratio`,
              type: "Baseline",
            }),
            bases[0](scale),
          ],
        },
      ],
    };
  }

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortSupplyOptions(arg) {
    const { scale, title } = arg;

    return {
      name: "Supply",
      tree: [
        {
          name: "Absolute",
          tree: [
            ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix) => [
              {
                scale,
                name: "All",
                title: `${title} Profit And Loss`,
                description: "",
                unit: "US Dollars",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${prefix}supply-in-profit`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${prefix}supply-in-loss`,
                  },
                  {
                    title: "Total",
                    color: colors.default,
                    datasetPath: `${scale}-to-${prefix}supply`,
                  },
                  {
                    title: "Halved Total",
                    color: colors.off,
                    datasetPath: `${scale}-to-${prefix}halved-supply`,
                    options: {
                      lineStyle: 4,
                    },
                  },
                ],
              },
            ]),
            {
              scale,
              name: `Total`,
              title: `${title} Total supply`,
              description: "",
              unit: "Bitcoin",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                genPath: (id) => `${scale}-to-${datasetIdToPrefix(id)}supply`,
              }),
            },
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit`,
              description: "",
              unit: "Bitcoin",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.profit,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(id)}supply-in-profit`,
              }),
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss`,
              description: "",
              unit: "Bitcoin",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.loss,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(id)}supply-in-loss`,
              }),
            },
          ],
        },
        {
          name: "Relative To Circulating",
          tree: [
            ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix) => [
              {
                scale,
                name: "All",
                title: `${title} Profit And Loss Relative To Circulating Supply`,
                description: "",
                unit: "Percentage",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${prefix}supply-in-profit-to-circulating-supply-ratio`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${prefix}supply-in-loss-to-circulating-supply-ratio`,
                  },
                  {
                    title: "100%",
                    color: colors.default,
                    datasetPath: `${scale}-to-${prefix}supply-to-circulating-supply-ratio`,
                  },
                  {
                    title: "50%",
                    color: colors.off,
                    datasetPath: `${scale}-to-${prefix}halved-supply-to-circulating-supply-ratio`,
                    options: {
                      lineStyle: 4,
                    },
                  },
                ],
              },
            ]),
            {
              scale,
              name: `Total`,
              title: `${title} Total supply Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}supply-to-circulating-supply-ratio`,
              }),
            },
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.profit,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}supply-in-profit-to-circulating-supply-ratio`,
              }),
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.loss,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}supply-in-loss-to-circulating-supply-ratio`,
              }),
            },
          ],
        },
        {
          name: "Relative To Own",
          tree: [
            ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix) => [
              {
                scale,
                name: "All",
                title: `${title} Supply In Profit And Loss Relative To Own Supply`,
                description: "",
                unit: "Percentage",
                bottom: [
                  {
                    title: "In Profit",
                    color: colors.profit,
                    datasetPath: `${scale}-to-${prefix}supply-in-profit-to-own-supply-ratio`,
                  },
                  {
                    title: "In Loss",
                    color: colors.loss,
                    datasetPath: `${scale}-to-${prefix}supply-in-loss-to-own-supply-ratio`,
                  },
                  {
                    title: "100%",
                    color: colors.default,
                    datasetPath: `${scale}-to-100`,
                    options: {
                      lastValueVisible: false,
                    },
                  },
                  {
                    title: "50%",
                    color: colors.off,
                    datasetPath: `${scale}-to-50`,
                    options: {
                      lineStyle: 4,
                      lastValueVisible: false,
                    },
                  },
                ],
              },
            ]),
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit Relative To Own Supply`,
              description: "",
              unit: "Percentage",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.profit,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}supply-in-profit-to-own-supply-ratio`,
              }),
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss Relative To Own Supply`,
              description: "",
              unit: "Percentage",
              bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
                title: "Supply",
                singleColor: colors.loss,
                genPath: (id) =>
                  `${scale}-to-${datasetIdToPrefix(
                    id,
                  )}supply-in-loss-to-own-supply-ratio`,
              }),
            },
          ],
        },
        // createMomentumPresetFolder({
        //   datasets: datasets[scale],
        //   scale,
        //   id: `${scale}-${id}-supply-in-profit-and-loss-percentage-self`,
        //   title: `${title} Supply In Profit And Loss (% Self)`,
        //   datasetId: `${datasetId}SupplyPNL%Self`,
        // }),
      ],
    };
  }

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortPricesPaidOptions(arg) {
    const { scale, title } = arg;

    /**
     * @param {Object} args
     * @param {TimeScale} args.scale
     * @param {AnyDatasetPrefix} args.prefix
     * @param {PercentileId} args.id
     * @returns {AnyDatasetPath}
     */
    function generatePath({ scale, prefix, id }) {
      return /** @type {const} */ (`${scale}-to-${prefix}${id}`);
    }

    return {
      name: "Prices Paid",
      tree: [
        {
          scale,
          name: `Average`,
          title: `${title} Average Price Paid - Realized Price`,
          description: "",
          unit: "US Dollars",
          top: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Average",
            genPath: (id) =>
              `${scale}-to-${datasetIdToPrefix(id)}realized-price`,
          }),
        },
        ...cohortOptionOrOptions.genOptionsIfSingle(arg, (prefix, arg) => [
          {
            scale,
            name: `Deciles`,
            title: `${title} deciles`,
            description: "",
            unit: "US Dollars",
            top: groups.percentiles
              .filter(({ value }) => Number(value) % 10 === 0)
              .map(({ name, id }) => {
                const datasetPath = generatePath({
                  scale,
                  prefix,
                  id,
                });

                return {
                  datasetPath,
                  color: arg.color,
                  title: name,
                };
              }),
          },
        ]),
        ...groups.percentiles.map((percentile) => ({
          scale,
          name: percentile.name,
          title: `${title} ${percentile.title}`,
          description: "",
          unit: /** @type {const} */ ("US Dollars"),
          top: cohortOptionOrOptions.toSeriesBlueprints(arg, {
            title: "Relative Net Unrealized PNL",
            genPath: (id) =>
              generatePath({
                scale,
                prefix: datasetIdToPrefix(id),
                id: percentile.id,
              }),
          }),
        })),
      ],
    };
  }

  /**
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsTree}
   */
  function createCohortOptions(arg) {
    return [
      createCohortUTXOOptions(arg),
      createCohortRealizedOptions(arg),
      createCohortUnrealizedOptions(arg),
      createCohortSupplyOptions(arg),
      createCohortPricesPaidOptions(arg),
    ];
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createAddressLiquidityOptions(scale) {
    return createAddressCohortOptionGroups({
      scale,
      name: `By Liquidity`,
      list: groups.liquidities.map(({ name, id, key }) => ({
        name,
        title: name,
        scale,
        color: colors[key],
        datasetId: id,
      })),
    });
  }

  /**
   *
   * @param {DefaultCohortOption | DefaultCohortOptions} arg
   * @returns {PartialOptionsGroup}
   */
  function createCohortOptionsGroup(arg) {
    return {
      name: "list" in arg ? "Compare" : arg.name,
      tree: createCohortOptions(arg),
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createHodlersOptions(scale) {
    /**
     * @param {Object} arg
     * @param {string} arg.name
     * @param {string} [arg.title]
     * @param {DefaultCohortOption[]} arg.list
     */
    function createFolder({ name, title, list }) {
      return {
        name,
        tree: [
          createCohortOptionsGroup({
            scale,
            name,
            title: title || name,
            list,
          }),
          ...list.map((cohort) => createCohortOptionsGroup(cohort)),
        ],
      };
    }

    return {
      name: "Hodlers",
      tree: [
        createFolder({
          name: "X Term Holders",
          list: groups.xTermHolders.map(({ key, id, name, legend }) => ({
            scale,
            color: colors[key],
            name: legend,
            datasetId: id,
            title: name,
          })),
        }),
        createFolder({
          name: "Up To X",
          list: groups.upTo.map(({ key, id, name }) => ({
            scale,
            color: colors[key],
            name,
            datasetId: id,
            title: name,
          })),
        }),
        createFolder({
          name: "From X To Y",
          list: groups.fromXToY.map(({ key, id, name }) => ({
            scale,
            color: colors[key],
            name,
            datasetId: id,
            title: name,
          })),
        }),
        createFolder({
          name: "From X",
          list: groups.fromX.map(({ key, id, name }) => ({
            scale,
            color: colors[key],
            name,
            datasetId: id,
            title: name,
          })),
        }),
        createFolder({
          name: "Epoch X",
          title: "Epochs",
          list: groups.epochs.map(({ key, id, name }) => ({
            scale,
            color: colors[key],
            name,
            datasetId: id,
            title: name,
          })),
        }),
      ],
    };
  }

  /**
   * @param {CohortOption<AnyAddressCohortId> | CohortOptions<AnyAddressCohortId>} arg
   * @returns {PartialChartOption}
   */
  function createAddressCountOption(arg) {
    const { scale, title } = arg;

    return {
      scale,
      name: `Address Count`,
      title: `${title} Address Count`,
      description: "",
      unit: "Count",
      bottom: cohortOptionOrOptions.toSeriesBlueprints(arg, {
        title: "Address Count",
        genPath: (id) => `${scale}-to-${id}-address-count`,
      }),
    };
  }

  /**
   * @param {CohortOption<AnyAddressCohortId> | CohortOptions<AnyAddressCohortId>} arg
   * @returns {PartialOptionsGroup}
   */
  function createAddressCohortOptionGroup(arg) {
    return {
      name:
        "list" in arg
          ? "Compare"
          : arg.filenameAddon
            ? `${arg.name} - ${arg.filenameAddon}`
            : arg.name,
      tree: [
        {
          name: "Addresses",
          tree: [createAddressCountOption(arg)],
        },
        ...createCohortOptions(arg),
      ],
    };
  }

  /**
   * @param {Object} arg
   * @param {TimeScale} arg.scale
   * @param {string} arg.name
   * @param {string} [arg.title]
   * @param {CohortOption<AnyAddressCohortId>[]} arg.list
   */
  function createAddressCohortOptionGroups({ scale, name, title, list }) {
    return {
      name,
      tree: [
        createAddressCohortOptionGroup({
          scale,
          name,
          title: title || name,
          list,
        }),
        ...list.map((cohort) => createAddressCohortOptionGroup(cohort)),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createAddressesOptions(scale) {
    return {
      name: "Addresses",
      tree: [
        {
          scale,
          name: `Total Non Empty Addresses`,
          title: `Total Non Empty Address`,
          description: "",
          unit: "Count",
          bottom: [
            {
              title: `Total Non Empty Address`,
              color: colors.bitcoin,
              datasetPath: `${scale}-to-address-count`,
            },
          ],
        },
        {
          scale,
          name: `New Addresses`,
          title: `New Addresses`,
          description: "",
          unit: "Count",
          bottom: [
            {
              title: `New Addresses`,
              color: colors.bitcoin,
              datasetPath: `${scale}-to-new-addresses`,
            },
          ],
        },
        {
          scale,
          name: `Total Addresses Created`,
          title: `Total Addresses Created`,
          description: "",
          unit: "Count",
          bottom: [
            {
              title: `Total Addresses Created`,
              color: colors.bitcoin,
              datasetPath: `${scale}-to-created-addresses`,
            },
          ],
        },
        {
          scale,
          name: `Total Empty Addresses`,
          title: `Total Empty Addresses`,
          description: "",
          unit: "Count",
          bottom: [
            {
              title: `Total Empty Addresses`,
              color: colors.off,
              datasetPath: `${scale}-to-empty-addresses`,
            },
          ],
        },
        createAddressCohortOptionGroups({
          scale,
          name: "By Size",
          title: "Address Sizes",
          list: groups.size.map(({ key, name, size }) => ({
            scale,
            color: colors[key],
            name,
            title: name,
            filenameAddon: size,
            datasetId: key,
          })),
        }),
        createAddressCohortOptionGroups({
          scale,
          name: "By Type",
          title: "Address Types",
          list: groups.type.map(({ key, name }) => ({
            scale,
            color: colors[key],
            name,
            title: name,
            datasetId: key,
          })),
        }),
        createAddressLiquidityOptions(scale),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createCointimeOptions(scale) {
    return {
      name: "Cointime Economics",
      tree: [
        {
          name: "Prices",
          tree: [
            {
              scale,
              name: "All",
              title: "All Cointime Prices",
              description: "",
              unit: "US Dollars",
              top: [
                {
                  title: "Vaulted Price",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaulted-price`,
                },
                {
                  title: "Active Price",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-price`,
                },
                {
                  title: "True Market Mean",
                  color: colors.trueMarketMeanPrice,
                  datasetPath: `${scale}-to-true-market-mean`,
                },
                {
                  title: "Realized Price",
                  color: colors.bitcoin,
                  datasetPath: `${scale}-to-realized-price`,
                },
                {
                  title: "Cointime",
                  color: colors.cointimePrice,
                  datasetPath: `${scale}-to-cointime-price`,
                },
              ],
            },
            {
              name: "Active",
              tree: [
                {
                  scale,
                  name: "Price",
                  title: "Active Price",
                  description: "",
                  unit: "US Dollars",
                  top: [
                    {
                      title: "Active Price",
                      color: colors.liveliness,
                      datasetPath: `${scale}-to-active-price`,
                    },
                  ],
                },
                createRatioOptions({
                  color: colors.liveliness,
                  ratioDatasetPath: `${scale}-to-market-price-to-active-price-ratio`,
                  scale,
                  title: "Active Price",
                  valueDatasetPath: `${scale}-to-active-price`,
                }),
              ],
            },
            {
              name: "Vaulted",
              tree: [
                {
                  scale,
                  name: "Price",
                  title: "Vaulted Price",
                  description: "",
                  unit: "US Dollars",
                  top: [
                    {
                      title: "Vaulted Price",
                      color: colors.vaultedness,
                      datasetPath: `${scale}-to-vaulted-price`,
                    },
                  ],
                },
                createRatioOptions({
                  color: colors.vaultedness,
                  ratioDatasetPath: `${scale}-to-market-price-to-vaulted-price-ratio`,
                  scale,
                  title: "Vaulted Price",
                  valueDatasetPath: `${scale}-to-vaulted-price`,
                }),
              ],
            },
            {
              name: "True Market Mean",
              tree: [
                {
                  scale,
                  name: "Price",
                  title: "True Market Mean",
                  description: "",
                  unit: "US Dollars",
                  top: [
                    {
                      title: "True Market Mean",
                      color: colors.trueMarketMeanPrice,
                      datasetPath: `${scale}-to-true-market-mean`,
                    },
                  ],
                },
                createRatioOptions({
                  color: colors.liveliness,
                  ratioDatasetPath: `${scale}-to-market-price-to-true-market-mean-ratio`,
                  scale,
                  title: "True Market Mean",
                  valueDatasetPath: `${scale}-to-true-market-mean`,
                }),
              ],
            },
            {
              name: "Cointime Price",
              tree: [
                {
                  scale,
                  name: "Price",
                  title: "Cointime Price",
                  description: "",
                  unit: "US Dollars",
                  top: [
                    {
                      title: "Cointime",
                      color: colors.cointimePrice,
                      datasetPath: `${scale}-to-cointime-price`,
                    },
                  ],
                },
                createRatioOptions({
                  color: colors.cointimePrice,
                  ratioDatasetPath: `${scale}-to-market-price-to-cointime-price-ratio`,
                  scale,
                  title: "Cointime",
                  valueDatasetPath: `${scale}-to-cointime-price`,
                }),
              ],
            },
          ],
        },
        {
          name: "Capitalizations",
          tree: [
            {
              scale,
              name: "All",
              title: "Cointime Capitalizations",
              description: "",
              unit: "US Dollars",
              bottom: [
                {
                  title: "Market Cap",
                  color: colors.default,
                  datasetPath: `${scale}-to-market-cap`,
                },
                {
                  title: "Realized Cap",
                  color: colors.realizedCap,
                  datasetPath: `${scale}-to-realized-cap`,
                },
                {
                  title: "Investor Cap",
                  color: colors.investorCap,
                  datasetPath: `${scale}-to-investor-cap`,
                },
                {
                  title: "Thermo Cap",
                  color: colors.thermoCap,
                  datasetPath: `${scale}-to-thermo-cap`,
                },
              ],
            },
            {
              scale,
              name: "Thermo Cap",
              title: "Thermo Cap",
              description: "",
              unit: "US Dollars",
              bottom: [
                {
                  title: "Thermo Cap",
                  color: colors.thermoCap,
                  datasetPath: `${scale}-to-thermo-cap`,
                },
              ],
            },
            {
              scale,
              name: "Investor Cap",
              title: "Investor Cap",
              description: "",
              unit: "US Dollars",
              bottom: [
                {
                  title: "Investor Cap",
                  color: colors.investorCap,
                  datasetPath: `${scale}-to-investor-cap`,
                },
              ],
            },
            {
              scale,
              name: "Thermo Cap To Investor Cap Ratio",
              title: "Thermo Cap To Investor Cap Ratio",
              description: "",
              unit: "Percentage",
              bottom: [
                {
                  title: "Ratio",
                  color: colors.bitcoin,
                  datasetPath: `${scale}-to-thermo-cap-to-investor-cap-ratio`,
                },
              ],
            },
          ],
        },
        {
          name: "Coinblocks",
          tree: [
            {
              scale,
              name: "All",
              title: "All Coinblocks",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Coinblocks Created",
                  color: colors.coinblocksCreated,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-created-1d-sum`
                      : `height-to-coinblocks-created`,
                },
                {
                  title: "Coinblocks Destroyed",
                  color: colors.coinblocksDestroyed,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-destroyed-1d-sum`
                      : `height-to-coinblocks-destroyed`,
                },
                {
                  title: "Coinblocks Stored",
                  color: colors.coinblocksStored,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-stored-1d-sum`
                      : `height-to-coinblocks-stored`,
                },
              ],
            },
            {
              scale,
              name: "Created",
              title: "Coinblocks Created",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Coinblocks Created",
                  color: colors.coinblocksCreated,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-created-1d-sum`
                      : `height-to-coinblocks-created`,
                },
              ],
            },
            {
              scale,
              name: "Destroyed",
              title: "Coinblocks Destroyed",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Coinblocks Destroyed",
                  color: colors.coinblocksDestroyed,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-destroyed-1d-sum`
                      : `height-to-coinblocks-destroyed`,
                },
              ],
            },
            {
              scale,
              name: "Stored",
              title: "Coinblocks Stored",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Coinblocks Stored",
                  color: colors.coinblocksStored,
                  datasetPath:
                    scale === "date"
                      ? `date-to-coinblocks-stored-1d-sum`
                      : `height-to-coinblocks-stored`,
                },
              ],
            },
          ],
        },
        {
          name: "Cumulative Coinblocks",
          tree: [
            {
              scale,
              name: "All",
              title: "All Cumulative Coinblocks",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Cumulative Coinblocks Created",
                  color: colors.coinblocksCreated,
                  datasetPath: `${scale}-to-cumulative-coinblocks-created`,
                },
                {
                  title: "Cumulative Coinblocks Destroyed",
                  color: colors.coinblocksDestroyed,
                  datasetPath: `${scale}-to-cumulative-coinblocks-destroyed`,
                },
                {
                  title: "Cumulative Coinblocks Stored",
                  color: colors.coinblocksStored,
                  datasetPath: `${scale}-to-cumulative-coinblocks-stored`,
                },
              ],
            },
            {
              scale,
              name: "Created",
              title: "Cumulative Coinblocks Created",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Cumulative Coinblocks Created",
                  color: colors.coinblocksCreated,
                  datasetPath: `${scale}-to-cumulative-coinblocks-created`,
                },
              ],
            },
            {
              scale,
              name: "Destroyed",
              title: "Cumulative Coinblocks Destroyed",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Cumulative Coinblocks Destroyed",
                  color: colors.coinblocksDestroyed,
                  datasetPath: `${scale}-to-cumulative-coinblocks-destroyed`,
                },
              ],
            },
            {
              scale,
              name: "Stored",
              title: "Cumulative Coinblocks Stored",
              description: "",
              unit: "Coinblocks",
              bottom: [
                {
                  title: "Cumulative Coinblocks Stored",
                  color: colors.coinblocksStored,
                  datasetPath: `${scale}-to-cumulative-coinblocks-stored`,
                },
              ],
            },
          ],
        },
        {
          name: "Liveliness & Vaultedness",
          tree: [
            {
              scale,
              name: "Liveliness - Activity",
              title: "Liveliness (Activity)",
              description: "",
              unit: "",
              bottom: [
                {
                  title: "Liveliness",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-liveliness`,
                },
              ],
            },
            {
              scale,
              name: "Vaultedness",
              title: "Vaultedness",
              description: "",
              unit: "",
              bottom: [
                {
                  title: "Vaultedness",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaultedness`,
                },
              ],
            },
            {
              scale,
              name: "Versus",
              title: "Liveliness V. Vaultedness",
              description: "",
              unit: "",
              bottom: [
                {
                  title: "Liveliness",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-liveliness`,
                },
                {
                  title: "Vaultedness",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaultedness`,
                },
              ],
            },
            {
              scale,
              name: "Activity To Vaultedness Ratio",
              title: "Activity To Vaultedness Ratio",
              description: "",
              unit: "Percentage",
              bottom: [
                {
                  title: "Activity To Vaultedness Ratio",
                  color: colors.activityToVaultednessRatio,
                  datasetPath: `${scale}-to-activity-to-vaultedness-ratio`,
                },
              ],
            },
            ...(scale === "date"
              ? /** @satisfies {PartialOptionsTree} */ ([
                  {
                    scale,
                    name: "Concurrent Liveliness - Supply Adjusted Coindays Destroyed",
                    title:
                      "Concurrent Liveliness - Supply Adjusted Coindays Destroyed",
                    description: "",
                    unit: "",
                    bottom: [
                      {
                        title: "Concurrent Liveliness 14d Median",
                        color: colors.liveliness,
                        datasetPath: `${scale}-to-concurrent-liveliness-2w-median`,
                      },
                      {
                        title: "Concurrent Liveliness",
                        color: colors.darkLiveliness,
                        datasetPath: `${scale}-to-concurrent-liveliness`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Liveliness Incremental Change",
                    title: "Liveliness Incremental Change",
                    description: "",
                    unit: "",
                    bottom: [
                      {
                        title: "Liveliness Incremental Change",
                        color: colors.darkLiveliness,
                        type: "Baseline",
                        datasetPath: `date-to-liveliness-net-change`,
                      },
                      {
                        title: "Liveliness Incremental Change 14 Day Median",
                        color: colors.liveliness,
                        type: "Baseline",
                        datasetPath: `date-to-liveliness-net-change-2w-median`,
                      },
                      bases[0]("date"),
                    ],
                  },
                ])
              : []),
          ],
        },
        {
          name: "Supply",
          tree: [
            {
              scale,
              name: "Vaulted",
              title: "Vaulted Supply",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Vaulted Supply",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaulted-supply`,
                },
              ],
            },
            {
              scale,
              name: "Active",
              title: "Active Supply",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Active Supply",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-supply`,
                },
              ],
            },
            {
              scale,
              name: "Vaulted V. Active",
              title: "Vaulted V. Active",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Circulating Supply",
                  color: colors.coinblocksCreated,
                  datasetPath: `${scale}-to-supply`,
                },
                {
                  title: "Vaulted Supply",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaulted-supply`,
                },
                {
                  title: "Active Supply",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-supply`,
                },
              ],
            },
            // TODO: Fix, Bad data
            // {
            //   id: 'asymptomatic-supply-regions',
            //   name: 'Asymptomatic Supply Regions',
            //   title: 'Asymptomatic Supply Regions',
            //   description: '',
            //   applyPreset(params) {
            //     return applyMultipleSeries({
            //       ...params,
            //       priceScaleOptions: {
            //         halved: true,
            //       },
            //       list: [
            //         {
            //           id: 'min-vaulted',
            //           title: 'Min Vaulted Supply',
            //           color: colors.vaultedness,
            //           dataset: params.`/${scale}-to-dateToMinVaultedSupply,
            //         },
            //         {
            //           id: 'max-active',
            //           title: 'Max Active Supply',
            //           color: colors.liveliness,
            //           dataset: params.`/${scale}-to-dateToMaxActiveSupply,
            //         },
            //       ],
            //     })
            //   },
            // },
            {
              scale,
              name: "Vaulted Net Change",
              title: "Vaulted Supply Net Change",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Vaulted Supply Net Change",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaulted-supply-net-change`,
                },
              ],
            },
            {
              scale,
              name: "Active Net Change",
              title: "Active Supply Net Change",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Active Supply Net Change",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-supply-net-change`,
                },
              ],
            },
            {
              scale,
              name: "Active VS. Vaulted 90D Net Change",
              title: "Active VS. Vaulted 90 Day Supply Net Change",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Active Supply Net Change",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-supply-3m-net-change`,
                  type: "Baseline",
                },
                {
                  title: "Vaulted Supply Net Change",
                  color: colors.vaultedPrice,
                  type: "Baseline",
                  datasetPath: `${scale}-to-vaulted-supply-3m-net-change`,
                },
                bases[0](scale),
              ],
            },
            // TODO: Fix, Bad data
            // {
            //   id: 'vaulted-supply-annualized-net-change',
            //   name: 'Vaulted Annualized Net Change',
            //   title: 'Vaulted Supply Annualized Net Change',
            //   description: '',
            //   applyPreset(params) {
            //     return applyMultipleSeries({
            //       ...params,
            //       priceScaleOptions: {
            //         halved: true,
            //       },
            //       list: [
            //         {
            //           id: 'vaulted-annualized-supply-net-change',
            //           title: 'Vaulted Supply Annualized Net Change',
            //           color: colors.vaultedness,
            //           dataset:
            //             `/${scale}-to-vaultedAnnualizedSupplyNetChange,
            //         },
            //       ],
            //     })
            //   },
            // },

            // TODO: Fix, Bad data
            // {
            //   id: 'vaulting-rate',
            //   name: 'Vaulting Rate',
            //   title: 'Vaulting Rate',
            //   description: '',
            //   applyPreset(params) {
            //     return applyMultipleSeries({
            //       ...params,
            //       priceScaleOptions: {
            //         halved: true,
            //       },
            //       list: [
            //         {
            //           id: 'vaulting-rate',
            //           title: 'Vaulting Rate',
            //           color: colors.vaultedness,
            //           dataset: `${scale}-to-vaultingRate,
            //         },
            //         {
            //           id: 'nominal-inflation-rate',
            //           title: 'Nominal Inflation Rate',
            //           color: colors.orange,
            //           dataset: params.`/${scale}-to-dateToYearlyInflationRate,
            //         },
            //       ],
            //     })
            //   },
            // },

            // TODO: Fix, Bad data
            // {
            //   id: 'active-supply-net-change-decomposition',
            //   name: 'Active Supply Net Change Decomposition (90D)',
            //   title: 'Active Supply Net 90 Day Change Decomposition',
            //   description: '',
            //   applyPreset(params) {
            //     return applyMultipleSeries({
            //       ...params,
            //       priceScaleOptions: {
            //         halved: true,
            //       },
            //       list: [
            //         {
            //           id: 'issuance-change',
            //           title: 'Change From Issuance',
            //           color: colors.emerald,
            //           dataset:
            //             params.params.datasets[scale]
            //               [scale].activeSupplyChangeFromIssuance90dChange,
            //         },
            //         {
            //           id: 'transactions-change',
            //           title: 'Change From Transactions',
            //           color: colors.rose,
            //           dataset:
            //             params.params.datasets[scale]
            //               [scale].activeSupplyChangeFromTransactions90dChange,
            //         },
            //         // {
            //         //   id: 'active',
            //         //   title: 'Active Supply',
            //         //   color: colors.liveliness,
            //         //   dataset: `${scale}-to-activeSupply,
            //         // },
            //       ],
            //     })
            //   },
            // },

            {
              scale,
              name: "In Profit",
              title: "Cointime Supply In Profit",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Circulating Supply",
                  color: colors.coinblocksCreated,
                  datasetPath: `${scale}-to-supply`,
                },
                {
                  title: "Vaulted Supply",
                  color: colors.vaultedness,
                  datasetPath: `${scale}-to-vaulted-supply`,
                },
                {
                  title: "Supply in profit",
                  color: colors.bitcoin,
                  datasetPath: `${scale}-to-supply-in-profit`,
                },
              ],
            },
            {
              scale,
              name: "In Loss",
              title: "Cointime Supply In Loss",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Circulating Supply",
                  color: colors.coinblocksCreated,
                  datasetPath: `${scale}-to-supply`,
                },
                {
                  title: "Active Supply",
                  color: colors.liveliness,
                  datasetPath: `${scale}-to-active-supply`,
                },
                {
                  title: "Supply in Loss",
                  color: colors.bitcoin,
                  datasetPath: `${scale}-to-supply-in-loss`,
                },
              ],
            },
          ],
        },
        ...(scale === "date"
          ? /** @satisfies {PartialOptionsTree} */ ([
              {
                name: "Inflation Rate",
                tree: [
                  {
                    scale,
                    name: "Normal",
                    title: "Cointime Yearly Inflation Rate",
                    description: "",
                    unit: "Percentage",
                    bottom: [
                      {
                        title: "Cointime Adjusted",
                        color: colors.coinblocksCreated,
                        datasetPath: `date-to-cointime-adjusted-inflation-rate`,
                      },
                      {
                        title: "Nominal",
                        color: colors.bitcoin,
                        datasetPath: `date-to-inflation-rate`,
                      },
                    ],
                  },
                  {
                    scale,
                    name: "Yearly",
                    title: "Cointime-Adjusted Yearly Inflation Rate",
                    description: "",
                    unit: "Percentage",
                    bottom: [
                      {
                        title: "Cointime Adjusted",
                        color: colors.coinblocksCreated,
                        datasetPath: `date-to-cointime-adjusted-yearly-inflation-rate`,
                      },
                      {
                        title: "Nominal",
                        color: colors.bitcoin,
                        datasetPath: `date-to-yearly-inflation-rate`,
                      },
                    ],
                  },
                ],
              },

              {
                scale,
                name: "Cointime Velocity",
                title: "Cointime-Adjusted Transactions Velocity",
                description: "",
                unit: "",
                bottom: [
                  {
                    title: "Cointime Adjusted",
                    color: colors.coinblocksCreated,
                    datasetPath: `date-to-cointime-adjusted-velocity`,
                  },
                  {
                    title: "Nominal",
                    color: colors.bitcoin,
                    datasetPath: `date-to-transaction-velocity`,
                  },
                ],
              },
            ])
          : []),
      ],
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createResearchOptions(scale) {
    return {
      name: "Research",
      tree: [createCointimeOptions(scale)],
    };
  }

  return [
    {
      name: "Charts",
      tree: [
        {
          name: "By Date",
          tree: [
            createMarketOptions("date"),
            createBlocksOptions("date"),
            createMinersOptions("date"),
            createTransactionsOptions("date"),
            ...createCohortOptions({
              scale: "date",
              color: colors.bitcoin,
              datasetId: "",
              name: "",
              title: "",
            }),
            createHodlersOptions("date"),
            createAddressesOptions("date"),
            createResearchOptions("date"),
          ],
        },
        {
          name: "By Height",
          tree: [
            createMarketOptions("height"),
            createBlocksOptions("height"),
            createMinersOptions("height"),
            createTransactionsOptions("height"),
            ...createCohortOptions({
              scale: "height",
              color: colors.bitcoin,
              datasetId: "",
              name: "",
              title: "",
            }),
            createHodlersOptions("height"),
            createAddressesOptions("height"),
            createResearchOptions("height"),
          ],
        },
      ],
    },
    {
      name: "Simulations",
      tree: [
        {
          kind: "simulation",
          title: "Simulation: Save In Bitcoin",
          name: "Save In Bitcoin",
        },
      ],
    },
    {
      name: "Library",
      tree: [
        {
          name: "Satoshi Nakamoto",
          tree: [
            {
              name: "Whitepaper",
              pdf: "satoshi-nakamoto/whitepaper.pdf",
            },
          ],
        },
        {
          name: "Nydig",
          tree: [
            {
              name: "Bitcoin's Protection Under The First Amendment",
              pdf: "nydig/protection-under-first-amendment.pdf",
            },
          ],
        },
        {
          name: "Nakamoto Project",
          tree: [
            {
              name: "Understanding Bitcoin Adoption In The United States",
              pdf: "nakamoto-project/understanding-bitcoin-adoption-in-the-united-states.pdf",
            },
          ],
        },
        {
          name: "Block",
          tree: [
            {
              name: "Knowledge And Perceptions - 2022 Report",
              pdf: "block/2022-report.pdf",
            },
          ],
        },
        {
          name: "Square",
          tree: [
            {
              name: "Clean Energy Initiative - 2021 Report",
              pdf: "square/2021-bitcoin-clean-energy-initiative.pdf",
            },
          ],
        },
        {
          name: "Braiins",
          tree: [
            {
              name: "Building Bitcoin In Rust",
              pdf: "braiins/building-bitcoin-in-rust.pdf",
            },
          ],
        },
        {
          name: "Glassnode",
          tree: [
            {
              name: "Cointime Economics",
              pdf: "glassnode/cointime-economics.pdf",
            },
          ],
        },
      ],
    },
    {
      name: "Donations",
      tree: [
        {
          name: "Donate on-chain",
          qrcode: true,
          url: () => "bitcoin:bc1q950q4ukpxxm6wjjkv6cpq8jzpazaxrrwftctkt",
        },
        {
          name: "Donate lightning",
          qrcode: true,
          url: () =>
            "lightning:lnurl1dp68gurn8ghj7ampd3kx2ar0veekzar0wd5xjtnrdakj7tnhv4kxctttdehhwm30d3h82unvwqhkxmmww3jkuar8d35kgetj8yuq363hv4",
        },
        {
          name: "Fundraiser",
          url: () => "https://geyser.fund/project/kibo",
        },
        {
          name: "Goals",
          url: () => "https://geyser.fund/project/kibo/goals",
        },
        {
          name: "Leaderboard",
          url: () => "https://geyser.fund/project/kibo/leaderboard",
        },
      ],
    },
    {
      name: "Share",
      qrcode: true,
      url: () => window.location.href,
    },
    {
      name: "Social",
      url: () =>
        "https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44",
    },
    {
      name: "Source",
      url: () => "https://github.com/kibo-money/kibo",
    },
    {
      name: "API",
      url: () => "/api",
    },
  ];
}

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Signals} args.signals
 * @param {Ids} args.ids
 * @param {Env} args.env
 * @param {Utilities} args.utils
 * @param {Signal<Record<LastPath, number> | null>} args.lastValues
 * @param {WebSockets} args.webSockets
 * @param {Signal<string | null>} args.qrcode
 */
export function initOptions({
  colors,
  signals,
  ids,
  env,
  utils,
  lastValues,
  webSockets,
  qrcode,
}) {
  const urlSelected = utils.url.pathnameToSelectedId();
  const savedSelectedId = localStorage.getItem(ids.selectedId);

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
   * @param {SeriesBlueprint[]} array
   */
  function getMainIdFromBlueprints(array) {
    const searchArray = array.filter(
      (blueprint) =>
        blueprint.options?.lastValueVisible !== false &&
        /** @type {LineStyleOptions | undefined} */ (blueprint.options)
          ?.lineStyle === undefined,
    );

    const blueprint =
      searchArray.length === 1
        ? searchArray[0]
        : searchArray.find((blueprint) => blueprint.main);

    if (!blueprint) return undefined;

    const id = blueprint.datasetPath
      .replace(DATE_TO_PREFIX, "")
      .replace(HEIGHT_TO_PREFIX, "");

    return /** @type {LastPath} */ (id);
  }

  /**
   * @param {Number | undefined} value
   * @param {Unit} unit
   */
  function formatValue(value, unit) {
    if (!value) return "";

    const s =
      unit !== "Count"
        ? utils.locale.numberToShortUSFormat(value)
        : utils.locale.numberToUSFormat(
            value,
            unit === "Count" ? 0 : undefined,
          );

    switch (unit) {
      case "US Dollars": {
        return `$${s}`;
      }
      case "Bitcoin": {
        return `₿${s}`;
      }
      case "Percentage": {
        return `${s}%`;
      }
      case "Seconds": {
        return `${s} sec`;
      }
      case "Megabytes": {
        return `${s} MB`;
      }
      default: {
        return s;
      }
    }
  }

  /**
   * @param {Object} args
   * @param {Option} args.option
   * @param {string} args.frame
   * @param {Signal<string | null>} args.qrcode
   * @param {string} [args.name]
   * @param {string} [args.top]
   * @param {string} [args.id]
   * @param {Owner | null} [args.owner]
   */
  function createOptionElement({
    option,
    frame,
    name,
    top,
    id,
    owner,
    qrcode,
  }) {
    switch (option.kind) {
      case "pdf": {
        return utils.dom.createAnchorElement({
          href: option.pdf,
          blank: true,
          text: option.name,
        });
      }
      case "url": {
        const href = option.url();

        if (option.qrcode) {
          return utils.dom.createButtonElement({
            text: option.name,
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
      }
      default: {
        const { input, label } = utils.dom.createLabeledInput({
          inputId: `${option.id}-${frame}${id || ""}-selector`,
          inputValue: option.id,
          inputName: `option-${frame}${id || ""}`,
          labelTitle: option.title,
          // name: name || option.name,
          onClick: () => {
            selected.set(option);
          },
        });

        const anchor = utils.dom.createAnchorElement({
          href: `/${option.id}`,
          text: name || option.name,
          onClick: () => {},
        });

        label.append(anchor);

        if (top) {
          const small = window.document.createElement("small");
          small.innerHTML = top;
          label.insertBefore(small, anchor);
        }

        if (option.kind === "chart") {
          const valueElement = window.document.createElement("small");
          valueElement.classList.add("value");

          if (!option.top?.length && !option.bottom?.length) {
            anchor.append(valueElement);

            signals.createEffect(
              () =>
                webSockets.krakenCandle.latest()?.close ?? lastValues()?.close,
              (close) => {
                if (close) {
                  valueElement.innerHTML = formatValue(close, "US Dollars");
                }
              },
            );
          } else if (option.bottom?.length) {
            const bottom = option.bottom;
            const id = getMainIdFromBlueprints(bottom);

            if (id) {
              anchor.append(valueElement);

              signals.createEffect(lastValues, (lastValues) => {
                if (lastValues) {
                  valueElement.innerHTML = formatValue(
                    lastValues[id],
                    option.unit,
                  );
                }
              });
            }
          } else if (option.top?.length) {
            const top = option.top;
            const id = getMainIdFromBlueprints(top);

            if (id) {
              anchor.append(valueElement);

              signals.createEffect(lastValues, (lastValues) => {
                if (lastValues) {
                  valueElement.innerHTML = formatValue(
                    lastValues[id],
                    option.unit,
                  );
                }
              });
            }
          }

          function createCheckEffect() {
            signals.createEffect(selected, (selected) => {
              if (selected?.id === option.id) {
                input.checked = true;
                localStorage.setItem(ids.selectedId, option.id);
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
        } else {
          console.log(option);
        }
      }
    }
  }

  /**
   * @param {PartialOptionsTree} partialTree
   * @param {Accessor<HTMLDivElement | HTMLDetailsElement | null>} parent
   * @param {OptionPath | undefined} path
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
        const folderId = ids.fromString(
          `${(path || [])?.map(({ name }) => name).join(" ")} ${
            anyPartial.name
          } folder`,
        );

        /** @type {Omit<OptionsGroup, keyof PartialOptionsGroup>} */
        const groupAddons = {
          id: folderId,
        };

        Object.assign(anyPartial, groupAddons);

        optionsIds?.push(groupAddons.id);

        const thisPath = {
          name: anyPartial.name,
          id: groupAddons.id,
        };

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
        /** @type {string} */
        let id;
        /** @type {Option["kind"]} */
        let kind;
        /** @type {string} */
        let title;

        if ("kind" in anyPartial && anyPartial.kind === "home") {
          kind = anyPartial.kind;
          id = anyPartial.kind;
          title = anyPartial.title;
        } else if ("pdf" in anyPartial) {
          kind = "pdf";
          id = `${ids.fromString(anyPartial.name)}-pdf`;
          title = anyPartial.name;
          anyPartial.pdf = `/assets/pdfs/${anyPartial.pdf}`;
        } else if ("url" in anyPartial) {
          kind = "url";
          id = `${ids.fromString(anyPartial.name)}-url`;
          title = anyPartial.name;
        } else if ("scale" in anyPartial) {
          kind = "chart";
          id = `chart-${anyPartial.scale}-to-${ids.fromString(
            anyPartial.title,
          )}`;
          title = anyPartial.title;
        } else {
          kind = anyPartial.kind;
          title = anyPartial.title;
          console.log("Unprocessed", anyPartial);
          id = `${kind}-${ids.fromString(anyPartial.title)}`;
          // return;
        }

        /** @type {ProcessedOptionAddons} */
        const optionAddons = {
          id,
          path: path || [],
          serializedPath: `/ ${[
            ...(path || []).map(({ name }) => name),
            anyPartial.name,
          ].join(" / ")}`,
          title,
        };

        Object.assign(anyPartial, optionAddons, { kind });

        const option = /** @type {Option} */ (anyPartial);

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

          if (element) {
            li.append(element);
          }
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
    /**
     * @param {Option} option
     * @param {Series | SeriesBlueprint} series
     */
    optionAndSeriesToKey(option, series) {
      return `${option.id}-${ids.fromString(series.title)}`;
    },
  };
}
/** @typedef {ReturnType<typeof initOptions>} Options */
