const DATE_TO_PREFIX = "date-to-";
const HEIGHT_TO_PREFIX = "height-to-";

/**
 * @param {Colors} colors
 * @returns {PartialOptionsTree}
 */
function createPartialOptions(colors) {
  function initGroups() {
    const xth = /** @type {const} */ ([
      {
        id: "sth",
        key: "sth",
        name: "Short Term Holders",
        legend: "STH",
      },
      {
        id: "lth",
        key: "lth",
        name: "Long Term Holders",
        legend: "LTH",
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

    const year = /** @type {const} */ ([
      { id: "year-2009", key: "year_2009", name: "2009" },
      { id: "year-2010", key: "year_2010", name: "2010" },
      { id: "year-2011", key: "year_2011", name: "2011" },
      { id: "year-2012", key: "year_2012", name: "2012" },
      { id: "year-2013", key: "year_2013", name: "2013" },
      { id: "year-2014", key: "year_2014", name: "2014" },
      { id: "year-2015", key: "year_2015", name: "2015" },
      { id: "year-2016", key: "year_2016", name: "2016" },
      { id: "year-2017", key: "year_2017", name: "2017" },
      { id: "year-2018", key: "year_2018", name: "2018" },
      { id: "year-2019", key: "year_2019", name: "2019" },
      { id: "year-2020", key: "year_2020", name: "2020" },
      { id: "year-2021", key: "year_2021", name: "2021" },
      { id: "year-2022", key: "year_2022", name: "2022" },
      { id: "year-2023", key: "year_2023", name: "2023" },
      { id: "year-2024", key: "year_2024", name: "2024" },
    ]);

    const age = /** @type {const} */ ([
      {
        key: "",
        id: "",
        name: "",
      },
      ...xth,
      ...upTo,
      ...fromXToY,
      ...fromX,
      ...year,
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
      xth,
      upTo,
      fromX,
      fromXToY,
      year,
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
  const groups = initGroups();
  /**
   * @typedef {(typeof groups.age)[number]["id"]} AgeCohortId
   * @typedef {Exclude<AgeCohortId, "">} AgeCohortIdSub
   * @typedef {(typeof groups.address)[number]["key"]} AddressCohortId
   * @typedef {(typeof groups.liquidities[number]["id"])} LiquidityId
   * @typedef {AgeCohortId | AddressCohortId} AnyCohortId
   * @typedef {AnyCohortId | LiquidityId} AnyPossibleCohortId
   * @typedef {'' | `${AgeCohortIdSub | AddressCohortId | LiquidityId}-`} AnyDatasetPrefix
   * @typedef {(typeof groups.averages)[number]["key"]} AverageName
   * @typedef {(typeof groups.totalReturns)[number]["key"]} TotalReturnKey
   * @typedef {(typeof groups.compoundReturns)[number]["key"]} CompoundReturnKey
   * @typedef {(typeof groups.percentiles)[number]["id"]} PercentileId
   */

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
              icon: "➕",
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
              icon: "🌊",
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
              icon: "%",
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
              icon: "⬆️",
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
              icon: "9️⃣",
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
              icon: "7️⃣",
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
              icon: "5️⃣",
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
              icon: "2️⃣",
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
              icon: "1️⃣",
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
              icon: "⬇️",
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
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {Color} args.color
   * @param {AnyDatasetPath} args.valueDatasetPath
   * @param {AnyDatasetPath} args.ratioDatasetPath
   * @param {string} args.title
   * @returns {PartialOptionsGroup}
   */
  function createRatioOptions({
    scale,
    color,
    valueDatasetPath,
    ratioDatasetPath,
    title,
  }) {
    return {
      name: "Ratio",
      tree: [
        {
          scale,
          name: "Basic",
          icon: "➗",
          title: `Market Price To ${title} Ratio`,
          unit: "Ratio",
          description: "",
          top: [
            {
              title,
              color,
              datasetPath: valueDatasetPath,
            },
          ],
          bottom: [
            {
              title: `Ratio`,
              type: "Baseline",
              datasetPath: ratioDatasetPath,
              options: {
                baseValue: {
                  price: 1,
                },
              },
            },
            bases[1](scale),
          ],
        },
        {
          scale,
          name: "Averages",
          description: "",
          icon: "〰️",
          unit: "Ratio",
          title: `Market Price To ${title} Ratio Averages`,
          top: [
            {
              title,
              color,
              datasetPath: valueDatasetPath,
            },
          ],
          bottom: [
            {
              title: `1Y SMA`,
              color: colors.red,
              datasetPath: /** @type {any} */ (`${ratioDatasetPath}-1y-sma`),
            },
            {
              title: `1M SMA`,
              color: colors.orange,
              datasetPath: `${ratioDatasetPath}-1m-sma`,
            },
            {
              title: `1W SMA`,
              color: colors.yellow,
              datasetPath: `${ratioDatasetPath}-1w-sma`,
            },
            {
              title: `Raw`,
              color: colors.default,
              datasetPath: ratioDatasetPath,
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
        {
          scale,
          name: "Momentum Oscillator",
          title: `Market Price To ${title} Ratio 1Y SMA Momentum Oscillator`,
          description: "",
          unit: "Ratio",
          icon: "🔀",
          top: [
            {
              title: `SMA`,
              color,
              datasetPath: valueDatasetPath,
            },
          ],
          bottom: [
            {
              title: `Momentum`,
              type: "Baseline",
              datasetPath: /** @type {any} */ (
                `${ratioDatasetPath}-1y-sma-momentum-oscillator`
              ),
            },
            bases[0](scale),
          ],
        },
        {
          scale,
          name: "Top Percentiles",
          icon: "✈️",
          title: `Market Price To ${title} Ratio Top Percentiles`,
          description: "",
          unit: "Ratio",
          top: [
            {
              title,
              color,
              datasetPath: valueDatasetPath,
            },
          ],
          bottom: [
            {
              title: `99.9%`,
              color: colors.probability0_1p,
              datasetPath: /** @type {any} */ (`${ratioDatasetPath}-99-9p`),
            },
            {
              title: `99.5%`,
              color: colors.probability0_5p,
              datasetPath: `${ratioDatasetPath}-99-5p`,
            },
            {
              title: `99%`,
              color: colors.probability1p,
              datasetPath: `${ratioDatasetPath}-99p`,
            },
            {
              title: `Raw`,
              color: colors.default,
              datasetPath: ratioDatasetPath,
            },
          ],
        },
        {
          scale,
          name: "Bottom Percentiles",
          icon: "🤿",
          title: `Market Price To ${title} Ratio Bottom Percentiles`,
          description: "",
          unit: "Ratio",
          top: [
            {
              title: `SMA`,
              color,
              datasetPath: valueDatasetPath,
            },
          ],
          bottom: [
            {
              title: `0.1%`,
              color: colors.probability0_1p,
              datasetPath: `${ratioDatasetPath}-0-1p`,
            },
            {
              title: `0.5%`,
              color: colors.probability0_5p,
              datasetPath: `${ratioDatasetPath}-0-5p`,
            },
            {
              title: `1%`,
              color: colors.probability1p,
              datasetPath: /** @type {any} */ (`${ratioDatasetPath}-1p`),
            },
            {
              title: `Raw`,
              color: colors.default,
              datasetPath: ratioDatasetPath,
            },
          ],
        },
        {
          scale,
          name: "Top Probabilities",
          icon: "🚀",
          title: `${title} Top Probabilities`,
          description: "",
          unit: "US Dollars",
          top: [
            {
              title: `99.9%`,
              color: colors.probability0_1p,
              datasetPath: /** @type {any} */ (`${valueDatasetPath}-99-9p`),
            },
            {
              title: `99.5%`,
              color: colors.probability0_5p,
              datasetPath: `${valueDatasetPath}-99-5p`,
            },
            {
              title: `99%`,
              color: colors.probability1p,
              datasetPath: `${valueDatasetPath}-99p`,
            },
          ],
        },
        {
          scale,
          name: "Bottom Probabilities",
          icon: "🚇",
          title: `${title} Bottom Probabilities`,
          description: "",
          unit: "US Dollars",
          top: [
            {
              title: `99.9%`,
              color: colors.probability0_1p,
              datasetPath: `${valueDatasetPath}-0-1p`,
            },
            {
              title: `99.5%`,
              color: colors.probability0_5p,
              datasetPath: `${valueDatasetPath}-0-5p`,
            },
            {
              title: `99%`,
              color: colors.probability1p,
              datasetPath: `${valueDatasetPath}-1p`,
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
            icon: "🔝",
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
            icon: "🏂",
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
                  icon: "🗓️",
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
                      icon: "📆",
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
                      icon: "📆",
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
            icon: "🌊",
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
            icon: "~",
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
        icon: "🧾",
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
    function createIndicatorsOptinos() {
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
              icon: "💵",
              name: "Dollars Per Bitcoin",
              title: "Dollars Per Bitcoin",
              description: "",
              unit: "US Dollars",
            },
            {
              scale,
              icon: "🍊",
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
          icon: "♾️",
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
          ? [createReturnsOptions(), createIndicatorsOptinos()]
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
                icon: "🧱",
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
                    icon: "D",
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
                    icon: "W",
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
                    icon: "M",
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
                    icon: "Y",
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
                    icon: "🧱",
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
                    icon: "📏",
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
                icon: "📏",
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
                icon: "🏋️",
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
                icon: "👾",
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
                icon: "⏰",
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
              ? /** @type {PartialOptionsTree} */ ([
                  {
                    name: "Last",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
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
                        icon: "💵",
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
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
                        name: "In Bitcoin",
                        title: "Daily Sum Of Coinbases In Bitcoin",
                        description: "",
                        unit: "Bitcoin",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.bitcoin,
                            datasetPath: `${scale}-to-coinbase`,
                          },
                        ],
                      },
                      {
                        scale,
                        icon: "💵",
                        name: "In Dollars",
                        title: "Daily Sum Of Coinbases In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-coinbase-in-dollars`,
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
                        icon: "🍊",
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
                        icon: "💵",
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
                        icon: "🍊",
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
                        icon: "💵",
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
                    name: "Last",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
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
                        icon: "💵",
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
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
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
                        icon: "💵",
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
                        icon: "🍊",
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
                        icon: "💵",
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
                        icon: "🍊",
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
                        icon: "💵",
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
                    name: "Last",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
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
                        icon: "💵",
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
                  {
                    scale,
                    name: "Daily Sum",
                    tree: [
                      {
                        scale,
                        icon: "🍊",
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
                        icon: "💵",
                        name: "In Dollars",
                        title: "Daily Sum Of Fees In Dollars",
                        description: "",
                        unit: "US Dollars",
                        bottom: [
                          {
                            title: "Sum",
                            color: colors.dollars,
                            datasetPath: `${scale}-to-fees-in-dollars`,
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
                        icon: "🍊",
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
                        icon: "💵",
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
                        icon: "🍊",
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
                        icon: "💵",
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
                ])
              : []),
          ],
        },

        {
          scale,
          icon: "⚔️",
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
                icon: "🧮",
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
                    icon: "⛏️",
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
                    icon: "🎗️",
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
                        icon: "🏷️",
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
                        icon: "😢",
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
                        icon: "🤞",
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
          icon: "🏋️",
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
                icon: "📊",
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
                icon: "🏭",
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
                    icon: "🏗️",
                    name: "Normal",
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
                    icon: "🏗️",
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
          icon: "🖐️",
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
              icon: "🍊",
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
              icon: "💵",
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
                    icon: "🍊",
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
                    icon: "💵",
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
                icon: "💨",
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
          icon: "⏰",
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

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortUTXOOptions({ scale, color, datasetId, title }) {
    const datasetPrefix = datasetIdToPrefix(datasetId);

    return {
      name: "UTXOs",
      tree: [
        {
          scale,
          name: `Count`,
          title: `${title} Unspent Transaction Outputs Count`,
          description: "",
          unit: "Count",
          icon: "🎫",
          bottom: [
            {
              title: "Count",
              color,
              datasetPath: `${scale}-to-${datasetPrefix}utxo-count`,
            },
          ],
        },
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortRealizedOptions({ scale, color, datasetId, title }) {
    const datasetPrefix = datasetIdToPrefix(datasetId);

    return {
      name: "Realized",
      tree: [
        {
          scale,
          name: `Price`,
          title: `${title} Realized Price`,
          description: "",
          unit: "US Dollars",
          icon: "🏷️",
          top: [
            {
              title: "Realized Price",
              color,
              datasetPath: `${scale}-to-${datasetPrefix}realized-price`,
            },
          ],
        },
        createRatioOptions({
          scale,
          color,
          ratioDatasetPath: `${scale}-to-market-price-to-${datasetPrefix}realized-price-ratio`,
          valueDatasetPath: `${scale}-to-${datasetPrefix}realized-price`,
          title: `${title} Realized Price`,
        }),
        {
          scale,
          name: `Capitalization`,
          title: `${title} Realized Capitalization`,
          description: "",
          unit: "US Dollars",
          icon: "💰",
          bottom: [
            {
              title: `${name} Realized Cap.`,
              color,
              datasetPath: `${scale}-to-${datasetPrefix}realized-cap`,
            },
            ...(datasetId
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
          icon: "🔀",
          bottom: [
            {
              title: `Net Change`,
              type: "Baseline",
              datasetPath: `${scale}-to-${datasetPrefix}realized-cap-1m-net-change`,
            },
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Profit`,
          title: `${title} Realized Profit`,
          description: "",
          unit: "US Dollars",
          icon: "🎉",
          bottom: [
            {
              title: "Realized Profit",
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}realized-profit-1d-sum`
                  : `height-to-${datasetPrefix}realized-profit`,
              color: colors.profit,
            },
          ],
        },
        {
          scale,
          name: "Loss",
          title: `${title} Realized Loss`,
          description: "",
          unit: "US Dollars",
          icon: "⚰️",
          bottom: [
            {
              title: "Realized Loss",
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}realized-loss-1d-sum`
                  : `height-to-${datasetPrefix}realized-loss`,
              color: colors.loss,
            },
          ],
        },
        {
          scale,
          name: `PNL - Profit And Loss`,
          title: `${title} Realized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          icon: "⚖️",
          bottom: [
            {
              title: "Profit",
              color: colors.profit,
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}realized-profit-1d-sum`
                  : `height-to-${datasetPrefix}realized-profit`,
              type: "Baseline",
            },
            {
              title: "Loss",
              color: colors.loss,
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}negative-realized-loss-1d-sum`
                  : `height-to-${datasetPrefix}negative-realized-loss`,
              type: "Baseline",
            },
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Net PNL - Net Profit And Loss`,
          title: `${title} Net Realized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          icon: "⚖️",
          bottom: [
            {
              title: "Net PNL",
              type: "Baseline",
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}net-realized-profit-and-loss-1d-sum`
                  : `height-to-${datasetPrefix}net-realized-profit-and-loss`,
            },
            bases[0](scale),
          ],
        },
        ...(scale === "date"
          ? /** @type {PartialOptionsTree} */ ([
              {
                scale,
                name: `Net PNL Relative To Market Cap`,
                title: `${title} Net Realized Profit And Loss Relative To Market Capitalization`,
                description: "",
                unit: "Percentage",
                icon: "➗",
                bottom: [
                  {
                    title: "Net",
                    type: "Baseline",
                    datasetPath: `${scale}-to-${datasetPrefix}net-realized-profit-and-loss-to-market-cap-ratio`,
                  },
                  bases[0](scale),
                ],
              },
            ])
          : []),
        {
          name: "Cumulative",
          tree: [
            {
              scale,
              name: `Profit`,
              title: `${title} Cumulative Realized Profit`,
              description: "",
              unit: "US Dollars",
              icon: "🎊",
              bottom: [
                {
                  title: "Cumulative Realized Profit",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}cumulative-realized-profit`,
                },
              ],
            },
            {
              scale,
              name: "Loss",
              title: `${title} Cumulative Realized Loss`,
              description: "",
              unit: "US Dollars",
              icon: "☠️",
              bottom: [
                {
                  title: "Cumulative Realized Loss",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}cumulative-realized-loss`,
                },
              ],
            },
            {
              scale,
              name: `Net PNL`,
              title: `${title} Cumulative Net Realized Profit And Loss`,
              description: "",
              unit: "US Dollars",
              icon: "➕",
              bottom: [
                {
                  title: "Cumulative Net Realized PNL",
                  type: "Baseline",
                  datasetPath: `${scale}-to-${datasetPrefix}cumulative-net-realized-profit-and-loss`,
                },
                bases[0](scale),
              ],
            },
            {
              scale,
              name: `Net PNL 30 Day Change`,
              title: `${title} Cumulative Net Realized Profit And Loss 30 Day Change`,
              description: "",
              unit: "US Dollars",
              icon: "🗓️",
              bottom: [
                {
                  title: "Cumulative Net Realized PNL 30d Change",
                  datasetPath: `${scale}-to-${datasetPrefix}cumulative-net-realized-profit-and-loss-1m-net-change`,
                  type: "Baseline",
                },
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
          icon: "∶",
          bottom: [
            {
              title: "Ratio",
              datasetPath:
                scale === "date"
                  ? `date-to-${datasetPrefix}realized-profit-to-loss-1d-sum-ratio`
                  : `height-to-${datasetPrefix}realized-profit-to-loss-ratio`,
              type: "Baseline",
              options: {
                baseValue: {
                  price: 1,
                },
              },
            },
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
              icon: "➗",
              bottom: [
                {
                  title: "SOPR",
                  datasetPath: `${scale}-to-${datasetPrefix}spent-output-profit-ratio`,
                  type: "Baseline",
                  options: {
                    baseValue: {
                      price: 1,
                    },
                  },
                },
                bases[1](scale),
              ],
            },
            {
              scale,
              name: `Adjusted - aSOPR`,
              title: `${title} Adjusted Spent Output Profit Ratio`,
              description: "",
              unit: "Percentage",
              icon: "➗",
              bottom: [
                {
                  title: "aSOPR",
                  datasetPath: `${scale}-to-${datasetPrefix}adjusted-spent-output-profit-ratio`,
                  type: "Baseline",
                  options: {
                    baseValue: {
                      price: 1,
                    },
                  },
                },
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
                  icon: "➕",
                  bottom: [
                    {
                      title: "Value",
                      color: colors.profit,
                      datasetPath:
                        scale === "date"
                          ? `date-to-${datasetPrefix}value-created-1d-sum`
                          : `height-to-${datasetPrefix}value-created`,
                    },
                  ],
                },
                {
                  scale,
                  name: `Adjusted`,
                  title: `${title} Adjusted Value Created`,
                  description: "",
                  unit: "US Dollars",
                  icon: "➕",
                  bottom: [
                    {
                      title: "Adjusted Value",
                      color: colors.profit,
                      datasetPath:
                        scale === "date"
                          ? `date-to-${datasetPrefix}adjusted-value-created-1d-sum`
                          : `height-to-${datasetPrefix}adjusted-value-created`,
                    },
                  ],
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
                  icon: "☄️",
                  bottom: [
                    {
                      title: "Value",
                      color: colors.loss,
                      datasetPath:
                        scale === "date"
                          ? `date-to-${datasetPrefix}value-destroyed-1d-sum`
                          : `height-to-${datasetPrefix}value-destroyed`,
                    },
                  ],
                },
                {
                  scale,
                  name: `Adjusted`,
                  title: `${title} Adjusted Value Destroyed`,
                  description: "",
                  unit: "US Dollars",
                  icon: "☄️",
                  bottom: [
                    {
                      title: "Adjusted Value",
                      color: colors.loss,
                      datasetPath:
                        scale === "date"
                          ? `date-to-${datasetPrefix}adjusted-value-destroyed-1d-sum`
                          : `height-to-${datasetPrefix}adjusted-value-destroyed`,
                    },
                  ],
                },
              ],
            },
          ],
        },
        .../** @satisfies {PartialOptionsTree} */ (
          scale === "date"
            ? [
                {
                  scale,
                  name: `Sell Side Risk Ratio`,
                  title: `${title} Sell Side Risk Ratio`,
                  description: "",
                  unit: "Percentage",
                  icon: "🥵",
                  bottom: [
                    {
                      title: "Ratio",
                      datasetPath: `${scale}-to-${datasetPrefix}sell-side-risk-ratio`,
                      color,
                    },
                  ],
                },
              ]
            : []
        ),
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortUnrealizedOptions({ scale, color, datasetId, title }) {
    const datasetPrefix = datasetIdToPrefix(datasetId);

    return {
      name: "Unrealized",
      tree: [
        {
          scale,
          name: `Profit`,
          title: `${title} Unrealized Profit`,
          description: "",
          unit: "US Dollars",
          icon: "🤑",
          bottom: [
            {
              title: "Profit",
              datasetPath: `${scale}-to-${datasetPrefix}unrealized-profit`,
              color: colors.profit,
            },
          ],
        },
        {
          scale,
          name: "Loss",
          title: `${title} Unrealized Loss`,
          description: "",
          unit: "US Dollars",
          icon: "😭",
          bottom: [
            {
              title: "Loss",
              datasetPath: `${scale}-to-${datasetPrefix}unrealized-loss`,
              color: colors.loss,
            },
          ],
        },
        {
          scale,
          name: `PNL - Profit And Loss`,
          title: `${title} Unrealized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          icon: "🤔",
          bottom: [
            {
              title: "Profit",
              color: colors.profit,
              datasetPath: `${scale}-to-${datasetPrefix}unrealized-profit`,
              type: "Baseline",
            },
            {
              title: "Loss",
              color: colors.loss,
              datasetPath: `${scale}-to-${datasetPrefix}negative-unrealized-loss`,
              type: "Baseline",
            },
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Net PNL - Net Profit And Loss`,
          title: `${title} Net Unrealized Profit And Loss`,
          description: "",
          unit: "US Dollars",
          icon: "⚖️",
          bottom: [
            {
              title: "Net Unrealized PNL",
              datasetPath: `${scale}-to-${datasetPrefix}net-unrealized-profit-and-loss`,
              type: "Baseline",
            },
            bases[0](scale),
          ],
        },
        {
          scale,
          name: `Net PNL Relative To Market Cap - NUPL`,
          title: `${title} Net Unrealized Profit And Loss Relative To Total Market Capitalization - NUPL`,
          description: "",
          unit: "Percentage",
          icon: "➗",
          bottom: [
            {
              title: "Relative Net Unrealized PNL",
              datasetPath: `${scale}-to-${datasetPrefix}net-unrealized-profit-and-loss-to-market-cap-ratio`,
              type: "Baseline",
            },
            bases[0](scale),
          ],
        },
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortSupplyOptions({ scale, color, datasetId, title }) {
    const datasetPrefix = datasetIdToPrefix(datasetId);

    return {
      name: "Supply",
      tree: [
        {
          name: "Absolute",
          tree: [
            {
              scale,
              name: "All",
              title: `${title} Profit And Loss`,
              icon: "❌",
              description: "",
              unit: "US Dollars",
              bottom: [
                {
                  title: "In Profit",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit`,
                },
                {
                  title: "In Loss",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss`,
                },
                {
                  title: "Total",
                  color: colors.default,
                  datasetPath: `${scale}-to-${datasetPrefix}supply`,
                },
                {
                  title: "Halved Total",
                  color: colors.off,
                  datasetPath: `${scale}-to-${datasetPrefix}halved-supply`,
                  options: {
                    lineStyle: 4,
                  },
                },
              ],
            },
            {
              scale,
              name: `Total`,
              title: `${title} Total supply`,
              icon: "∑",
              description: "",
              unit: "Bitcoin",
              bottom: [
                {
                  title: "Supply",
                  color,
                  datasetPath: `${scale}-to-${datasetPrefix}supply`,
                },
              ],
            },
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit`,
              description: "",
              unit: "Bitcoin",
              icon: "📈",
              bottom: [
                {
                  title: "Supply",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit`,
                },
              ],
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss`,
              description: "",
              unit: "Bitcoin",
              icon: "📉",
              bottom: [
                {
                  title: "Supply",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss`,
                },
              ],
            },
          ],
        },
        {
          name: "Relative To Circulating",
          tree: [
            {
              scale,
              name: "All",
              title: `${title} Profit And Loss Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              icon: "🔀",
              bottom: [
                {
                  title: "In Profit",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-circulating-supply-ratio`,
                },
                {
                  title: "In Loss",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-circulating-supply-ratio`,
                },
                {
                  title: "100%",
                  color: colors.default,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-to-circulating-supply-ratio`,
                },
                {
                  title: "50%",
                  color: colors.off,
                  datasetPath: `${scale}-to-${datasetPrefix}halved-supply-to-circulating-supply-ratio`,
                  options: {
                    lineStyle: 4,
                  },
                },
              ],
            },
            {
              scale,
              name: `Total`,
              title: `${title} Total supply Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              icon: "∑",
              bottom: [
                {
                  title: "Supply",
                  color,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-to-circulating-supply-ratio`,
                },
              ],
            },
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              icon: "📈",
              bottom: [
                {
                  title: "Supply",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-circulating-supply-ratio`,
                },
              ],
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss Relative To Circulating Supply`,
              description: "",
              unit: "Percentage",
              icon: "📉",
              bottom: [
                {
                  title: "Supply",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-circulating-supply-ratio`,
                },
              ],
            },
          ],
        },
        {
          name: "Relative To Own",
          tree: [
            {
              scale,
              name: "All",
              title: `${title} Supply In Profit And Loss Relative To Own Supply`,
              description: "",
              unit: "Percentage",
              icon: "🔀",
              bottom: [
                {
                  title: "In Profit",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-own-supply-ratio`,
                },
                {
                  title: "In Loss",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-own-supply-ratio`,
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
            {
              scale,
              name: "In Profit",
              title: `${title} Supply In Profit Relative To Own Supply`,
              description: "",
              unit: "Percentage",
              icon: "📈",
              bottom: [
                {
                  title: "Supply",
                  color: colors.profit,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-profit-to-own-supply-ratio`,
                },
              ],
            },
            {
              scale,
              name: "In Loss",
              title: `${title} Supply In Loss Relative To Own Supply`,
              description: "",
              unit: "Percentage",
              icon: "📉",
              bottom: [
                {
                  title: "Supply",
                  color: colors.loss,
                  datasetPath: `${scale}-to-${datasetPrefix}supply-in-loss-to-own-supply-ratio`,
                },
              ],
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
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortPricesPaidOptions({ scale, color, datasetId, title }) {
    /**
     * @param {Object} args
     * @param {TimeScale} args.scale
     * @param {AnyPossibleCohortId} args.cohortId
     * @param {PercentileId} args.id
     * @returns {AnyDatasetPath}
     */
    function generatePath({ scale, cohortId, id }) {
      const datasetPrefix = datasetIdToPrefix(cohortId);
      return /** @type {const} */ (`${scale}-to-${datasetPrefix}${id}`);
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
          icon: "~",
          top: [
            {
              title: "Average",
              color,
              datasetPath: `${scale}-to-${datasetIdToPrefix(
                datasetId,
              )}realized-price`,
            },
          ],
        },
        {
          scale,
          name: `Deciles`,
          title: `${title} deciles`,
          icon: "🌗",
          description: "",
          unit: "US Dollars",
          top: groups.percentiles
            .filter(({ value }) => Number(value) % 10 === 0)
            .map(({ name, id }) => {
              const datasetPath = generatePath({
                scale,
                cohortId: datasetId,
                id,
              });

              return {
                datasetPath,
                color,
                title: name,
              };
            }),
        },
        ...groups.percentiles.map((percentile) => ({
          scale,
          name: percentile.name,
          title: `${title} ${percentile.title}`,
          description: "",
          unit: /** @type {const} */ ("US Dollars"),
          icon: "🌓",
          top: [
            {
              title: percentile.name,
              color,
              datasetPath: generatePath({
                scale,
                cohortId: datasetId,
                id: percentile.id,
              }),
            },
          ],
        })),
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {string} args.name
   * @param {TimeScale} args.scale
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsTree}
   */
  function createCohortOptions({ name, scale, color, datasetId, title }) {
    return [
      createCohortUTXOOptions({
        color,
        datasetId,
        scale,
        title,
      }),
      createCohortRealizedOptions({
        color,
        datasetId,
        scale,
        title,
      }),
      createCohortUnrealizedOptions({
        color,
        datasetId,
        scale,
        title,
      }),
      createCohortSupplyOptions({
        color,
        datasetId,
        scale,
        title,
      }),
      createCohortPricesPaidOptions({
        color,
        datasetId,
        scale,
        title,
      }),
    ];
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createLiquidityOptions({ scale, color }) {
    return {
      name: `Split By Liquidity`,
      tree: groups.liquidities.map((liquidity) => {
        /** @type {PartialOptionsGroup} */
        const folder = {
          name: liquidity.name,
          tree: createCohortOptions({
            title: `${liquidity.name}`,
            name: `${liquidity.name}`,
            scale,
            color,
            datasetId: liquidity.id,
          }),
        };
        return folder;
      }),
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {string} args.name
   * @param {AnyPossibleCohortId} args.datasetId
   * @param {string} args.title
   * @param {Color} args.color
   * @returns {PartialOptionsGroup}
   */
  function createCohortOptionsGroup({ scale, color, name, datasetId, title }) {
    return {
      name,

      tree: createCohortOptions({
        title,
        name,
        scale,
        color,
        datasetId: datasetId,
      }),
    };
  }

  /**
   * @param {TimeScale} scale
   * @returns {PartialOptionsGroup}
   */
  function createHodlersOptions(scale) {
    return {
      name: "Hodlers",
      tree: [
        {
          scale,
          name: `Hodl Supply`,
          title: `Hodl Supply`,
          description: "",
          icon: "🌊",
          unit: "Percentage",
          bottom: [
            {
              title: `24h`,
              color: colors.up_to_1d,
              datasetPath: `${scale}-to-up-to-1d-supply-to-circulating-supply-ratio`,
            },

            ...groups.fromXToY.map(({ key, id, name, legend }) => ({
              title: legend,
              color: colors[key],
              datasetPath: /** @type {const} */ (
                `${scale}-to-${id}-supply-to-circulating-supply-ratio`
              ),
            })),

            {
              title: `15y+`,
              color: colors.from_15y,
              datasetPath: `${scale}-to-from-15y-supply-to-circulating-supply-ratio`,
            },
          ],
        },
        ...groups.xth.map(({ key, id, name, legend }) =>
          createCohortOptionsGroup({
            scale,
            color: colors[key],
            name: legend,
            datasetId: id,
            title: name,
          }),
        ),
        {
          name: "Up To X",
          tree: groups.upTo.map(({ key, id, name }) =>
            createCohortOptionsGroup({
              scale,
              color: colors[key],
              name,
              datasetId: id,
              title: name,
            }),
          ),
        },
        {
          name: "From X To Y",
          tree: groups.fromXToY.map(({ key, id, name }) =>
            createCohortOptionsGroup({
              scale,
              color: colors[key],
              name,
              datasetId: id,
              title: name,
            }),
          ),
        },
        {
          name: "From X",
          tree: groups.fromX.map(({ key, id, name }) =>
            createCohortOptionsGroup({
              scale,
              color: colors[key],
              name,
              datasetId: id,
              title: name,
            }),
          ),
        },
        {
          name: "Years",
          tree: groups.year.map(({ key, id, name }) =>
            createCohortOptionsGroup({
              scale,
              color: colors[key],
              name,
              datasetId: id,
              title: name,
            }),
          ),
        },
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {string} args.name
   * @param {AddressCohortId} args.datasetId
   * @param {Color} args.color
   * @returns {PartialChartOption}
   */
  function createAddressCountOption({ scale, color, name, datasetId }) {
    return {
      scale,
      name: `Address Count`,
      title: `${name} Address Count`,
      description: "",
      unit: "Count",
      icon: "📕",
      bottom: [
        {
          title: "Address Count",
          color,
          datasetPath: `${scale}-to-${datasetId}-address-count`,
        },
      ],
    };
  }

  /**
   * @param {Object} args
   * @param {TimeScale} args.scale
   * @param {string} args.name
   * @param {AddressCohortId} args.datasetId
   * @param {Color} args.color
   * @param {string} [args.filenameAddon]
   * @returns {PartialOptionsGroup}
   */
  function createAddressOptions({
    scale,
    color,
    name,
    filenameAddon,
    datasetId,
  }) {
    return {
      name: filenameAddon ? `${name} - ${filenameAddon}` : name,
      tree: [
        createAddressCountOption({ scale, name, datasetId, color }),
        ...createCohortOptions({
          title: name,
          scale,
          name,
          color,
          datasetId,
        }),
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
          icon: "💳",
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
          icon: "🏡",
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
          icon: "🏠",
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
          icon: "🗑️",
          bottom: [
            {
              title: `Total Empty Addresses`,
              color: colors.off,
              datasetPath: `${scale}-to-empty-addresses`,
            },
          ],
        },
        {
          name: "By Size",
          tree: groups.size.map(({ key, name, size }) =>
            createAddressOptions({
              scale,
              color: colors[key],
              name,
              filenameAddon: size,
              datasetId: key,
            }),
          ),
        },
        {
          scale,
          name: "By Type",
          tree: groups.type.map(({ key, name }) =>
            createAddressOptions({
              scale,
              color: colors[key],
              name,
              datasetId: key,
            }),
          ),
        },
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
              icon: "🔀",
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
                  icon: "❤️",
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
                  icon: "🏦",
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
                  icon: "〰️",
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
                  icon: "⏱️",
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
              icon: "🔀",
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
              icon: "⛏️",
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
              icon: "🧑‍💼",
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
              icon: "➗",
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
              icon: "🧱",
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
              icon: "🧊",
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
              icon: "⛓️‍💥",
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
              icon: "🗄️",
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
              icon: "🔀",
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
              icon: "🧊",
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
              icon: "⛓️‍💥",
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
              icon: "🗄️",
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
              icon: "❤️",
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
              icon: "🏦",
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
              icon: "⚔️",
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
              icon: "➗",
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
              ? /** @type {PartialOptionsTree} */ ([
                  {
                    scale,
                    icon: "❤️",
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
                    icon: "📊",
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
              icon: "🏦",
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
              icon: "❤️",
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
              icon: "⚔️",
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
            //   icon: IconTablerDirections,
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
              icon: "🏦",
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
              icon: "❤️",
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
              icon: "⚔️",
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
            //   icon: IconTablerBuildingBank,
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
            //   icon: IconTablerBuildingBank,
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
            //   icon: IconTablerArrowsCross,
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
              icon: "📈",
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
              icon: "📉",
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
          ? /** @type {PartialOptionsTree} */ ([
              {
                name: "Inflation Rate",
                tree: [
                  {
                    scale,
                    icon: "🏭",
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
                    icon: "🏭",
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
                icon: "💨",
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
            createLiquidityOptions({
              scale: "date",
              color: colors.bitcoin,
            }),
            createCointimeOptions("date"),
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
            createLiquidityOptions({
              scale: "height",
              color: colors.bitcoin,
            }),
            createCointimeOptions("height"),
          ],
        },
      ],
    },
    {
      name: "Simulations",
      tree: [
        {
          icon: "🧪",
          kind: "simulation",
          title: "Dollar Cost Average Simulation",
          name: "Dollar Cost Average",
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
              icon: "📄",
              name: "Whitepaper",
              file: "satoshi-nakamoto/whitepaper.pdf",
            },
          ],
        },
        {
          name: "Nydig",
          tree: [
            {
              icon: "⚖️",
              name: "Bitcoin's Protection Under The First Amendment",
              file: "nydig/protection-under-first-amendment.pdf",
            },
          ],
        },
        {
          name: "Nakamoto Project",
          tree: [
            {
              icon: "🇺🇸",
              name: "Understanding Bitcoin Adoption In The United States",
              file: "nakamoto-project/understanding-bitcoin-adoption-in-the-united-states.pdf",
            },
          ],
        },
        {
          name: "Block",
          tree: [
            {
              icon: "🤔",
              name: "Knowledge And Perceptions - 2022 Report",
              file: "block/2022-report.pdf",
            },
          ],
        },
        {
          name: "Square",
          tree: [
            {
              icon: "⚡️",
              name: "Clean Energy Initiative - 2021 Report",
              file: "square/2021-bitcoin-clean-energy-initiative.pdf",
            },
          ],
        },
        {
          name: "Braiins",
          tree: [
            {
              icon: "🦀",
              name: "Building Bitcoin In Rust",
              file: "braiins/building-bitcoin-in-rust.pdf",
            },
          ],
        },
        {
          name: "Glassnode",
          tree: [
            {
              icon: "🏦",
              name: "Cointime Economics",
              file: "glassnode/cointime-economics.pdf",
            },
          ],
        },
      ],
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
 */
export function initOptions({
  colors,
  signals,
  ids,
  env,
  utils,
  lastValues,
  webSockets,
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

  const filter = signals.createSignal(
    /** @type {FoldersFilter} */ (
      localStorage.getItem(ids.foldersFilter) || "all"
    ),
  );

  function initCounters() {
    const favoritesCount = signals.createSignal(0);
    const newCount = signals.createSignal(0);

    /** @param {Option} option  */
    function createCountersEffects(option) {
      let firstFavoritesRun = true;

      signals.createEffect(() => {
        if (option.isFavorite()) {
          favoritesCount.set((c) => c + 1);
        } else if (!firstFavoritesRun) {
          favoritesCount.set((c) => c - 1);
        }
        firstFavoritesRun = false;
      });

      let firstNewRun = true;

      signals.createEffect(() => {
        if (!option.visited()) {
          newCount.set((c) => c + 1);
        } else if (!firstNewRun) {
          newCount.set((c) => c - 1);
        }
        firstNewRun = false;
      });
    }

    return {
      favorites: favoritesCount,
      new: newCount,
      createEffect: createCountersEffects,
    };
  }
  const counters = initCounters();

  const treeElement = signals.createSignal(
    /** @type {HTMLDivElement | null} */ (null),
  );

  /** @type {string[] | undefined} */
  const optionsIds = env.localhost ? [] : undefined;

  /**
   * @param {Option} option
   */
  function optionToVisitedKey(option) {
    return `${option.id}-visited`;
  }

  /**
   * @param {Option} option
   */
  function optionToFavoriteKey(option) {
    return `${option.id}-favorite`;
  }

  /**
   * @param {SeriesBlueprint[]} array
   */
  function getMainValueFromBlueprints(array) {
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

    return /** @type {number | undefined} */ (
      lastValues()?.[/** @type {LastPath} */ (id)]
    );
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
   * @param {string} [args.name]
   * @param {string} [args.top]
   * @param {string} [args.id]
   * @param {Owner | null} [args.owner]
   */
  function createOptionLabeledInput({ option, frame, name, top, id, owner }) {
    const { input, label, spanMain } = utils.dom.createComplexLabeledInput({
      inputId: `${option.id}-${frame}${id || ""}-selector`,
      inputValue: option.id,
      inputName: `option-${frame}${id || ""}`,
      labelTitle: option.title,
      name: name || option.name,
      onClick: () => {
        // if (option.kind !== "pdf") {
        selected.set(option);
        // }
      },
      href: `/${option.kind === "pdf" ? option.file : option.id}`,
    });

    if (top) {
      const small = window.document.createElement("small");
      small.innerHTML = top;
      label.insertBefore(small, spanMain);
    }

    const spanEmoji = window.document.createElement("span");
    spanEmoji.classList.add("emoji");
    spanEmoji.innerHTML = option.icon;
    spanMain.prepend(spanEmoji);

    if (option.kind === "chart") {
      const spanValue = window.document.createElement("span");
      spanValue.classList.add("value");
      spanMain.append(spanValue);

      if (!option.top?.length && !option.bottom?.length) {
        signals.createEffect(() => {
          const close =
            webSockets.krakenCandle.latest()?.close ?? lastValues()?.close;

          if (close) {
            spanValue.innerHTML = formatValue(close, "US Dollars");
          }
        });
      } else if (option.bottom?.length) {
        const bottom = option.bottom;
        signals.createEffect(() => {
          spanValue.innerHTML = formatValue(
            getMainValueFromBlueprints(bottom),
            option.unit,
          );
        });
      } else if (option.top?.length) {
        const top = option.top;
        signals.createEffect(() => {
          spanValue.innerHTML = formatValue(
            getMainValueFromBlueprints(top),
            option.unit,
          );
        });
      }
    }

    /** @type {HTMLSpanElement | undefined} */
    let spanNew;

    if (!option.visited()) {
      spanNew = window.document.createElement("span");
      spanNew.classList.add("new");
      spanMain.append(spanNew);
    }

    function createFavoriteEffect() {
      signals.createEffect(
        // @ts-ignore
        (_wasFavorite) => {
          const wasFavorite = /** @type {boolean} */ (_wasFavorite);
          const isFavorite = option.isFavorite();

          if (!wasFavorite && isFavorite) {
            const iconFavorite = window.document.createElement("svg");
            spanMain.append(iconFavorite);
            iconFavorite.outerHTML =
              '<svg viewBox="0 0 20 20" class="favorite"><path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" /></svg>';
          } else if (wasFavorite && !isFavorite) {
            spanMain.lastElementChild?.remove();
          }

          return isFavorite;
        },
        false,
      );
    }

    function createCheckEffect() {
      signals.createEffect(() => {
        if (selected()?.id === option.id) {
          input.checked = true;
          spanNew?.remove();
          option.visited.set(true);
          localStorage.setItem(optionToVisitedKey(option), "1");
          localStorage.setItem(ids.selectedId, option.id);
        } else if (input.checked) {
          input.checked = false;
        }
      });
    }

    if (owner !== undefined) {
      signals.runWithOwner(owner, () => {
        createCheckEffect();
        createFavoriteEffect();
      });
    } else {
      createCheckEffect();
      createFavoriteEffect();
    }

    return label;
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

      const li = signals.createMemo(
        // @ts-ignored
        (_previous) => {
          const previous = /** @type {HTMLLIElement | null} */ (_previous);
          previous?.remove();

          const _ul = ul();

          if (renderLi() && _ul) {
            const li = window.document.createElement("li");
            utils.dom.insertElementAtIndex(_ul, li, partialIndex);
            return li;
          } else {
            return null;
          }
        },
        undefined,
      );

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

        signals.createEffect(() => {
          const _li = li();

          if (!_li) {
            passedDetails.set(null);
            return;
          }

          signals.createEffect(() => {
            if (selected().path.includes(thisPath)) {
              _li.dataset.highlight = "";
            } else {
              delete _li.dataset.highlight;
            }
          });

          const details = window.document.createElement("details");
          const folderOpenLocalStorageKey = `${folderId}-open`;
          details.open = !!localStorage.getItem(folderOpenLocalStorageKey);
          details.id = folderId;
          detailsList.push(details);
          _li.appendChild(details);

          const summary = window.document.createElement("summary");
          details.appendChild(summary);

          const spanMarker = window.document.createElement("span");
          spanMarker.classList.add("marker");
          summary.append(spanMarker);

          const spanName = utils.dom.createSpanName(anyPartial.name);
          summary.append(spanName);

          const supCount = window.document.createElement("sup");

          signals.createEffect(() => {
            supCount.innerHTML = childOptionsCount().toLocaleString();
          });
          spanName.append(supCount);

          details.addEventListener("toggle", () => {
            const open = details.open;

            if (open) {
              localStorage.setItem(folderOpenLocalStorageKey, "1");
              passedDetails.set(details);
            } else {
              localStorage.removeItem(folderOpenLocalStorageKey);
              passedDetails.set(null);
            }
          });
        });

        function createRenderLiEffect() {
          signals.createEffect(() => {
            const count = childOptionsCount();
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
        } else if ("file" in anyPartial) {
          kind = "pdf";
          id = `${ids.fromString(anyPartial.name)}-pdf`;
          title = anyPartial.name;
          anyPartial.file = `assets/pdfs/${anyPartial.file}`;
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
          isFavorite: signals.createSignal(false),
          visited: signals.createSignal(false),
        };

        Object.assign(anyPartial, optionAddons, { kind, title });

        const option = /** @type {Option} */ (anyPartial);

        if (urlSelected === option.id) {
          selected.set(option);
        } else if (!selected() && savedSelectedId === option.id) {
          selected.set(option);
        }

        option.isFavorite.set(
          !!localStorage.getItem(optionToFavoriteKey(option)),
        );
        option.visited.set(!!localStorage.getItem(optionToVisitedKey(option)));

        counters.createEffect(option);

        list.push(option);
        optionsIds?.push(option.id);

        const hidden = signals.createSignal(true);

        function createHiddenEffect() {
          signals.createEffect(() => {
            switch (filter()) {
              case "all": {
                hidden.set(false);
                break;
              }
              case "favorites": {
                hidden.set(!option.isFavorite());
                break;
              }
              case "new": {
                hidden.set(option.visited());
                break;
              }
            }
          });
        }
        createHiddenEffect();

        function createRenderLiEffect() {
          signals.createEffect(() => {
            renderLi.set(!hidden());
          });
        }
        createRenderLiEffect();

        signals.createEffect(() => {
          const _li = li();

          if (!_li) {
            return;
          }

          signals.createEffect(() => {
            if (selected() === option) {
              _li.dataset.highlight = "";
            } else {
              delete _li.dataset.highlight;
            }
          });

          signals.untrack(() => {
            const label = createOptionLabeledInput({
              option,
              frame: "folders",
            });
            _li.append(label);
          });
        });

        const memo = signals.createMemo(() => (hidden() ? 0 : 1));
        listForSum.push(memo);
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
    counters,
    filter,
    treeElement,
    optionToVisitedKey,
    createOptionLabeledInput,
    optionToFavoriteKey,
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
