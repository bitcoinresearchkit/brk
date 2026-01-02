// @ts-nocheck

/**
 * A tree accessor - an object with a `.by` property containing MetricNodes keyed by index name.
 * Example: brk.tree.computed.price.priceClose has { by: { dateindex: MetricNode, height: MetricNode, ... } }
 * @template T
 * @typedef {{ by: Partial<Record<Index, MetricNode<T>>>, indexes: () => Index[] }} MetricAccessor
 */

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
 * @typedef {{ metric: MetricAccessor<any>, unit?: Unit }} FetchedAnySeriesOptions
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

import { localhost } from "../utils/env";

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {BrkClient} args.brk
 * @returns {PartialOptionsTree}
 */
export function createPartialOptions({ colors, brk }) {
  /** @type {<T extends object>(obj: T) => [keyof T, T[keyof T]][]} */
  const entries = Object.entries;

  /**
   * @param {string} id
   * @param {boolean}  compoundAdjective
   */
  function periodIdToName(id, compoundAdjective) {
    const suffix = compoundAdjective || parseInt(id) === 1 ? "" : "s";
    return id
      .replace("d", ` day${suffix}`)
      .replace("w", ` week${suffix}`)
      .replace("m", ` month${suffix}`)
      .replace("y", ` year${suffix}`);
  }

  const market = brk.tree.computed.market;
  const averages = /** @type {const} */ ([
    ["1w", 7, "red", market.price1wSma, market.price1wEma],
    ["8d", 8, "orange", market.price8dSma, market.price8dEma],
    ["13d", 13, "amber", market.price13dSma, market.price13dEma],
    ["21d", 21, "yellow", market.price21dSma, market.price21dEma],
    ["1m", 30, "lime", market.price1mSma, market.price1mEma],
    ["34d", 34, "green", market.price34dSma, market.price34dEma],
    ["55d", 55, "emerald", market.price55dSma, market.price55dEma],
    ["89d", 89, "teal", market.price89dSma, market.price89dEma],
    ["144d", 144, "cyan", market.price144dSma, market.price144dEma],
    ["200d", 200, "sky", market.price200dSma, market.price200dEma],
    ["1y", 365, "blue", market.price1ySma, market.price1yEma],
    ["2y", 730, "indigo", market.price2ySma, market.price2yEma],
    ["200w", 1400, "violet", market.price200wSma, market.price200wEma],
    ["4y", 1460, "purple", market.price4ySma, market.price4yEma],
  ]).map(
    ([id, days, colorKey, sma, ema]) =>
      /** @type {const} */ ({
        id,
        name: periodIdToName(id, true),
        days,
        color: colors[colorKey],
        sma,
        ema,
      }),
  );

  const dcaClasses = /** @type {const} */ ([
    [2015, "pink", false],
    [2016, "red", false],
    [2017, "orange", true],
    [2018, "yellow", true],
    [2019, "green", true],
    [2020, "teal", true],
    [2021, "sky", true],
    [2022, "blue", true],
    [2023, "purple", true],
    [2024, "fuchsia", true],
    [2025, "pink", true],
  ]).map(
    ([year, colorKey, defaultActive]) =>
      /** @type {const} */ ({
        year,
        color: colors[colorKey],
        defaultActive,
      }),
  );

  const utxoCohorts = brk.tree.computed.stateful.utxoCohorts;
  const addressCohorts = brk.tree.computed.stateful.addressCohorts;
  const {
    TERM_NAMES,
    EPOCH_NAMES,
    MAX_AGE_NAMES,
    MIN_AGE_NAMES,
    AGE_RANGE_NAMES,
    GE_AMOUNT_NAMES,
    LT_AMOUNT_NAMES,
    AMOUNT_RANGE_NAMES,
    SPENDABLE_TYPE_NAMES,
  } = brk;

  const cohortAll = /** @type {const} */ ({
    name: "",
    title: "",
    color: colors.orange,
    tree: utxoCohorts.all,
  });
  const cohortAllForComparaison = /** @type {const} */ ({
    name: "all",
    title: "",
    color: colors.default,
    tree: utxoCohorts.all,
  });

  const constant100 = flattenConstant(brk.tree.computed.constants.constant100);

  const termColors = /** @type {const} */ ({
    short: "yellow",
    long: "fuchsia",
  });
  const terms = entries(utxoCohorts.term).map(([key, tree]) => {
    const names = TERM_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[termColors[key]],
      tree,
    });
  });

  const maxAgeColors = /** @type {const} */ ({
    _1w: "red",
    _1m: "orange",
    _2m: "amber",
    _3m: "yellow",
    _4m: "lime",
    _5m: "green",
    _6m: "teal",
    _1y: "sky",
    _2y: "indigo",
    _3y: "violet",
    _4y: "purple",
    _5y: "fuchsia",
    _6y: "pink",
    _7y: "red",
    _8y: "orange",
    _10y: "amber",
    _12y: "yellow",
    _15y: "lime",
  });
  const upToDate = entries(utxoCohorts.maxAge).map(([key, tree]) => {
    const names = MAX_AGE_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[maxAgeColors[key]],
      tree,
    });
  });

  const minAgeColors = /** @type {const} */ ({
    _1d: "red",
    _1w: "orange",
    _1m: "yellow",
    _2m: "lime",
    _3m: "green",
    _4m: "teal",
    _5m: "cyan",
    _6m: "blue",
    _1y: "indigo",
    _2y: "violet",
    _3y: "purple",
    _4y: "fuchsia",
    _5y: "pink",
    _6y: "rose",
    _7y: "red",
    _8y: "orange",
    _10y: "yellow",
    _12y: "lime",
  });
  const fromDate = entries(utxoCohorts.minAge).map(([key, tree]) => {
    const names = MIN_AGE_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[minAgeColors[key]],
      tree,
    });
  });

  const ageRangeColors = /** @type {const} */ ({
    upTo1d: "pink",
    _1dTo1w: "red",
    _1wTo1m: "orange",
    _1mTo2m: "yellow",
    _2mTo3m: "yellow",
    _3mTo4m: "lime",
    _4mTo5m: "lime",
    _5mTo6m: "lime",
    _6mTo1y: "green",
    _1yTo2y: "cyan",
    _2yTo3y: "blue",
    _3yTo4y: "indigo",
    _4yTo5y: "violet",
    _5yTo6y: "purple",
    _6yTo7y: "purple",
    _7yTo8y: "fuchsia",
    _8yTo10y: "fuchsia",
    _10yTo12y: "pink",
    _12yTo15y: "red",
    from15y: "orange",
  });
  const dateRange = entries(utxoCohorts.ageRange).map(([key, tree]) => {
    const names = AGE_RANGE_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[ageRangeColors[key]],
      tree,
    });
  });

  const epochColors = /** @type {const} */ ({
    _0: "red",
    _1: "yellow",
    _2: "orange",
    _3: "lime",
    _4: "green",
  });
  const epoch = entries(utxoCohorts.epoch).map(([key, tree]) => {
    const names = EPOCH_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[epochColors[key]],
      tree,
    });
  });

  const geAmountColors = /** @type {const} */ ({
    _1sat: "orange",
    _10sats: "orange",
    _100sats: "yellow",
    _1kSats: "lime",
    _10kSats: "green",
    _100kSats: "cyan",
    _1mSats: "blue",
    _10mSats: "indigo",
    _1btc: "purple",
    _10btc: "violet",
    _100btc: "fuchsia",
    _1kBtc: "pink",
    _10kBtc: "red",
  });
  const utxosAboveAmount = entries(utxoCohorts.geAmount).map(([key, tree]) => {
    const names = GE_AMOUNT_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[geAmountColors[key]],
      tree,
    });
  });
  /** @type {readonly AddressCohortObject[]} */
  const addressesAboveAmount = entries(addressCohorts.geAmount).map(
    ([key, tree]) => {
      const names = GE_AMOUNT_NAMES[key];
      return /** @type {const} */ ({
        name: names.short,
        title: names.long,
        color: colors[geAmountColors[key]],
        tree,
      });
    },
  );

  const ltAmountColors = /** @type {const} */ ({
    _10sats: "orange",
    _100sats: "yellow",
    _1kSats: "lime",
    _10kSats: "green",
    _100kSats: "cyan",
    _1mSats: "blue",
    _10mSats: "indigo",
    _1btc: "purple",
    _10btc: "violet",
    _100btc: "fuchsia",
    _1kBtc: "pink",
    _10kBtc: "red",
    _100kBtc: "orange",
  });
  const utxosUnderAmount = entries(utxoCohorts.ltAmount).map(([key, tree]) => {
    const names = LT_AMOUNT_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[ltAmountColors[key]],
      tree,
    });
  });
  /** @type {readonly AddressCohortObject[]} */
  const addressesUnderAmount = entries(addressCohorts.ltAmount).map(
    ([key, tree]) => {
      const names = LT_AMOUNT_NAMES[key];
      return /** @type {const} */ ({
        name: names.short,
        title: names.long,
        color: colors[ltAmountColors[key]],
        tree,
      });
    },
  );

  const amountRangeColors = /** @type {const} */ ({
    _0sats: "red",
    _1satTo10sats: "orange",
    _10satsTo100sats: "yellow",
    _100satsTo1kSats: "lime",
    _1kSatsTo10kSats: "green",
    _10kSatsTo100kSats: "cyan",
    _100kSatsTo1mSats: "blue",
    _1mSatsTo10mSats: "indigo",
    _10mSatsTo1btc: "purple",
    _1btcTo10btc: "violet",
    _10btcTo100btc: "fuchsia",
    _100btcTo1kBtc: "pink",
    _1kBtcTo10kBtc: "red",
    _10kBtcTo100kBtc: "orange",
    _100kBtcOrMore: "yellow",
  });
  const utxosAmountRanges = entries(utxoCohorts.amountRange).map(
    ([key, tree]) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return /** @type {const} */ ({
        name: names.short,
        title: names.long,
        color: colors[amountRangeColors[key]],
        tree,
      });
    },
  );
  /** @type {readonly AddressCohortObject[]} */
  const addressesAmountRanges = entries(addressCohorts.amountRange).map(
    ([key, tree]) => {
      const names = AMOUNT_RANGE_NAMES[key];
      return /** @type {const} */ ({
        name: names.short,
        title: names.long,
        color: colors[amountRangeColors[key]],
        tree,
      });
    },
  );

  const spendableTypeColors = /** @type {const} */ ({
    p2pk65: "red",
    p2pk33: "orange",
    p2pkh: "yellow",
    p2ms: "lime",
    p2sh: "green",
    p2wpkh: "teal",
    p2wsh: "blue",
    p2tr: "indigo",
    p2a: "purple",
    unknown: "violet",
    empty: "fuchsia",
  });
  const type = entries(utxoCohorts.type).map(([key, tree]) => {
    const names = SPENDABLE_TYPE_NAMES[key];
    return /** @type {const} */ ({
      name: names.short,
      title: names.long,
      color: colors[spendableTypeColors[key]],
      tree,
    });
  });

  const cointime = brk.tree.computed.cointime;
  const cointimePrices = /** @type {const} */ ([
    {
      price: cointime.trueMarketMean,
      ratio: cointime.trueMarketMeanRatio,
      name: "True market mean",
      title: "true market mean",
      color: colors.blue,
    },
    {
      price: cointime.vaultedPrice,
      ratio: cointime.vaultedPriceRatio,
      name: "Vaulted",
      title: "vaulted price",
      color: colors.lime,
    },
    {
      price: cointime.activePrice,
      ratio: cointime.activePriceRatio,
      name: "Active",
      title: "active price",
      color: colors.rose,
    },
    {
      price: cointime.cointimePrice,
      ratio: cointime.cointimePriceRatio,
      name: "cointime",
      title: "cointime price",
      color: colors.yellow,
    },
  ]);
  const cointimeCapitalizations = /** @type {const} */ ([
    [cointime.vaultedCap, "vaulted", "lime"],
    [cointime.activeCap, "active", "rose"],
    [cointime.cointimeCap, "cointime", "yellow"],
    [cointime.investorCap, "investor", "fuchsia"],
    [cointime.thermoCap, "thermo", "emerald"],
  ]).map(([metric, name, colorKey]) => {
    return /** @type {const} */ ({
      metric,
      name,
      title: `${name} Capitalization`,
      color: colors[colorKey],
    });
  });

  /**
   * Get constant pattern by number dynamically from tree
   * Examples: 0 → constant0, 38.2 → constant382, -1 → constantMinus1
   * @param {number} num
   * @returns {Constant0Pattern<any>}
   */
  function getConstant(num) {
    const constants = brk.tree.computed.constants;
    const key =
      num >= 0
        ? `constant${String(num).replace(".", "")}`
        : `constantMinus${Math.abs(num)}`;
    const constant = constants[key];
    if (!constant) throw new Error(`Unknown constant: ${num} (key: ${key})`);
    return constant;
  }

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
      metric: flattenConstant(getConstant(number)),
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
          metric: flattenConstant(getConstant(number)),
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

  // ============================================================================
  // Tree-first helper functions
  // These accept typed pattern objects from brk.tree and return series blueprints
  // ============================================================================

  /**
   * Create a single series from a tree accessor
   * @param {Object} args
   * @param {MetricAccessor<any>} args.metric - Tree accessor with .by property
   * @param {string} args.name - Display name for the series
   * @param {Color} [args.color]
   * @param {Unit} [args.unit]
   * @param {boolean} [args.defaultActive]
   * @param {LineSeriesPartialOptions} [args.options]
   */
  function s({ metric, name, color, defaultActive, unit, options }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      metric,
      title: name,
      color,
      unit,
      defaultActive,
      options,
    });
  }

  /**
   * Create series from a BlockCountPattern ({ base, sum, cumulative })
   * @template T
   * @param {BlockCountPattern<T>} pattern
   * @param {string} title
   * @param {Color} [color]
   */
  function fromBlockCount(pattern, title, color) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      { metric: pattern.base, title, color: color ?? colors.default },
      {
        metric: pattern.sum,
        title: `${title} (sum)`,
        color: colors.red,
        defaultActive: false,
      },
      {
        metric: pattern.cumulative,
        title: `${title} (cum.)`,
        color: colors.cyan,
        defaultActive: false,
      },
    ]);
  }

  /**
   * Create series from a BitcoinPattern ({ base, sum, cumulative, average, min, max, median, pct* })
   * @template T
   * @param {BitcoinPattern<T>} pattern
   * @param {string} title
   * @param {Color} [color]
   */
  function fromBitcoin(pattern, title, color) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      { metric: pattern.base, title, color: color ?? colors.default },
      { metric: pattern.average, title: "Average", defaultActive: false },
      {
        metric: pattern.sum,
        title: `${title} (sum)`,
        color: colors.red,
        defaultActive: false,
      },
      {
        metric: pattern.cumulative,
        title: `${title} (cum.)`,
        color: colors.cyan,
        defaultActive: false,
      },
      {
        metric: pattern.max,
        title: "Max",
        color: colors.pink,
        defaultActive: false,
      },
      {
        metric: pattern.min,
        title: "Min",
        color: colors.green,
        defaultActive: false,
      },
      {
        metric: pattern.median,
        title: "Median",
        color: colors.amber,
        defaultActive: false,
      },
      {
        metric: pattern.pct75,
        title: "pct75",
        color: colors.red,
        defaultActive: false,
      },
      {
        metric: pattern.pct25,
        title: "pct25",
        color: colors.yellow,
        defaultActive: false,
      },
      {
        metric: pattern.pct90,
        title: "pct90",
        color: colors.rose,
        defaultActive: false,
      },
      {
        metric: pattern.pct10,
        title: "pct10",
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * Create series from a BlockSizePattern ({ sum, cumulative, average, min, max, median, pct* })
   * @template T
   * @param {BlockSizePattern<T>} pattern
   * @param {string} title
   * @param {Color} [color]
   */
  function fromBlockSize(pattern, title, color) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      { metric: pattern.sum, title, color: color ?? colors.default },
      { metric: pattern.average, title: "Average", defaultActive: false },
      {
        metric: pattern.cumulative,
        title: `${title} (cum.)`,
        color: colors.cyan,
        defaultActive: false,
      },
      {
        metric: pattern.max,
        title: "Max",
        color: colors.pink,
        defaultActive: false,
      },
      {
        metric: pattern.min,
        title: "Min",
        color: colors.green,
        defaultActive: false,
      },
      {
        metric: pattern.median,
        title: "Median",
        color: colors.amber,
        defaultActive: false,
      },
      {
        metric: pattern.pct75,
        title: "pct75",
        color: colors.red,
        defaultActive: false,
      },
      {
        metric: pattern.pct25,
        title: "pct25",
        color: colors.yellow,
        defaultActive: false,
      },
      {
        metric: pattern.pct90,
        title: "pct90",
        color: colors.rose,
        defaultActive: false,
      },
      {
        metric: pattern.pct10,
        title: "pct10",
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * Flatten a Constant0Pattern into a simple MetricAccessor
   * Constant0Pattern has { dateindex: { by: {...} }, height: { by: {...} }, ... }
   * This flattens it to { by: { dateindex: MetricNode, height: MetricNode, ... } }
   * @param {Constant0Pattern<any>} pattern
   * @returns {MetricAccessor<any>}
   */
  function flattenConstant(pattern) {
    return {
      by: {
        dateindex: pattern.dateindex.by.dateindex,
        decadeindex: pattern.decadeindex.by.decadeindex,
        height: pattern.height.by.height,
        monthindex: pattern.monthindex.by.monthindex,
        quarterindex: pattern.quarterindex.by.quarterindex,
        semesterindex: pattern.semesterindex.by.semesterindex,
        weekindex: pattern.weekindex.by.weekindex,
        yearindex: pattern.yearindex.by.yearindex,
      },
    };
  }

  /**
   * Create a constant line series
   * @param {Object} args
   * @param {Constant0Pattern<any>} args.constant - The constant pattern from tree.constants
   * @param {string} args.name
   * @param {Unit} args.unit
   * @param {Color} [args.color]
   * @param {number} [args.lineStyle]
   * @param {boolean} [args.defaultActive]
   */
  function line({ constant, name, unit, color, lineStyle, defaultActive }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      metric: flattenConstant(constant),
      title: name,
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

  // Shorthand for tree access
  const tree = brk.tree.computed;
  const constants = tree.constants;

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
      metric: "1w_sma",
      color: colors.lime,
    },
    {
      name: "1m sma",
      metric: "1m_sma",
      color: colors.teal,
    },
    {
      name: "1y sma",
      metric: "1y_sma",
      color: colors.sky,
    },
    {
      name: "2y sma",
      metric: "2y_sma",
      color: colors.indigo,
    },
    {
      name: "4y sma",
      metric: "4y_sma",
      color: colors.purple,
    },
    {
      name: "all sma",
      metric: "sma",
      color: colors.rose,
    },
  ]);

  /**
   * @param {Object} args
   * @param {string} args.name
   * @param {string} args.legend
   * @param {string} args.title
   * @param {Indexes3<Dollars>} [args.price] - Separate price pattern (for ActivePriceRatioPattern style)
   * @param {ActivePriceRatioPattern | EmaRatioPattern} args.ratio - Ratio pattern (tree-first)
   * @param {Color} [args.color]
   */
  function createPriceWithRatioOptions({
    name,
    title,
    legend,
    price,
    ratio,
    color,
  }) {
    // Support both patterns: EmaRatioPattern has .price, ActivePriceRatioPattern needs separate price arg
    const priceMetric = price ?? ratio.price;

    // Map percentile names to ratio pattern properties
    const percentileUsdMap = /** @type {const} */ ([
      { name: "pct99", prop: ratio.ratioPct99Usd, color: colors.rose },
      { name: "pct98", prop: ratio.ratioPct98Usd, color: colors.pink },
      { name: "pct95", prop: ratio.ratioPct95Usd, color: colors.fuchsia },
      { name: "pct5", prop: ratio.ratioPct5Usd, color: colors.cyan },
      { name: "pct2", prop: ratio.ratioPct2Usd, color: colors.sky },
      { name: "pct1", prop: ratio.ratioPct1Usd, color: colors.blue },
    ]);

    const percentileMap = /** @type {const} */ ([
      { name: "pct99", prop: ratio.ratioPct99, color: colors.rose },
      { name: "pct98", prop: ratio.ratioPct98, color: colors.pink },
      { name: "pct95", prop: ratio.ratioPct95, color: colors.fuchsia },
      { name: "pct5", prop: ratio.ratioPct5, color: colors.cyan },
      { name: "pct2", prop: ratio.ratioPct2, color: colors.sky },
      { name: "pct1", prop: ratio.ratioPct1, color: colors.blue },
    ]);

    // SD patterns by window
    const sdPatterns = /** @type {const} */ ([
      { nameAddon: "all", titleAddon: "", sd: ratio.ratioSd },
      { nameAddon: "4y", titleAddon: "4y", sd: ratio.ratio4ySd },
      { nameAddon: "2y", titleAddon: "2y", sd: ratio.ratio2ySd },
      { nameAddon: "1y", titleAddon: "1y", sd: ratio.ratio1ySd },
    ]);

    // SD band definitions with their pattern property accessors
    /** @param {Ratio1ySdPattern} sd */
    const getSdBands = (sd) => [
      { name: "0σ", prop: sd._0sdUsd, color: colors.lime },
      { name: "+0.5σ", prop: sd.p05sdUsd, color: colors.yellow },
      { name: "+1σ", prop: sd.p1sdUsd, color: colors.amber },
      { name: "+1.5σ", prop: sd.p15sdUsd, color: colors.orange },
      { name: "+2σ", prop: sd.p2sdUsd, color: colors.red },
      { name: "+2.5σ", prop: sd.p25sdUsd, color: colors.rose },
      { name: "+3σ", prop: sd.p3sd, color: colors.pink }, // No USD variant for ±3σ
      { name: "−0.5σ", prop: sd.m05sdUsd, color: colors.teal },
      { name: "−1σ", prop: sd.m1sdUsd, color: colors.cyan },
      { name: "−1.5σ", prop: sd.m15sdUsd, color: colors.sky },
      { name: "−2σ", prop: sd.m2sdUsd, color: colors.blue },
      { name: "−2.5σ", prop: sd.m25sdUsd, color: colors.indigo },
      { name: "−3σ", prop: sd.m3sd, color: colors.violet }, // No USD variant for ±3σ
    ];

    return [
      {
        name: "price",
        title,
        top: [
          s({
            metric: priceMetric,
            name: legend,
            color,
          }),
        ],
      },
      {
        name: "Ratio",
        title: `${title} Ratio`,
        top: [
          s({
            metric: priceMetric,
            name: legend,
            color,
          }),
          ...percentileUsdMap.map(({ name, prop, color }) =>
            s({
              metric: prop,
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
            metric: ratio.ratio,
            title: "Ratio",
            type: "Baseline",
            options: {
              baseValue: { price: 1 },
            },
          }),
          ...percentileMap.map(({ name, prop, color }) =>
            s({
              metric: prop,
              name,
              color,
              defaultActive: false,
              options: {
                lineStyle: 1,
              },
            }),
          ),
          s({
            metric: ratio.ratio1wSma,
            name: "1w SMA",
            color: colors.orange,
            defaultActive: false,
            options: {
              lineStyle: 1,
            },
          }),
          s({
            metric: ratio.ratio1mSma,
            name: "1m SMA",
            color: colors.red,
            defaultActive: false,
            options: {
              lineStyle: 1,
            },
          }),
          createPriceLine({
            number: 1,
            unit: "ratio",
          }),
        ],
      },
      {
        name: "ZScores",
        tree: [
          {
            name: "compare",
            title: `Compare ${title} ZScores`,
            top: [
              s({
                metric: priceMetric,
                name: legend,
                color,
              }),
              s({
                metric: ratio.ratio1ySd._0sdUsd,
                name: "1y 0sd",
                color: colors.fuchsia,
                defaultActive: false,
              }),
              s({
                metric: ratio.ratio2ySd._0sdUsd,
                name: "2y 0sd",
                color: colors.purple,
                defaultActive: false,
              }),
              s({
                metric: ratio.ratio4ySd._0sdUsd,
                name: "4y 0sd",
                color: colors.violet,
                defaultActive: false,
              }),
              s({
                metric: ratio.ratioSd._0sdUsd,
                name: "0sd",
                color: colors.indigo,
                defaultActive: false,
              }),
            ],
            bottom: [
              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                metric: ratio.ratioSd.zscore,
                title: "All",
                type: "Baseline",
              }),
              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                metric: ratio.ratio4ySd.zscore,
                colors: [colors.lime, colors.rose],
                title: "4y",
                type: "Baseline",
              }),
              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                metric: ratio.ratio2ySd.zscore,
                colors: [colors.avocado, colors.pink],
                title: "2y",
                type: "Baseline",
              }),
              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                metric: ratio.ratio1ySd.zscore,
                colors: [colors.yellow, colors.fuchsia],
                title: "1Y",
                type: "Baseline",
              }),
              ...createPriceLines({
                numbers: [0, 1, 2, 3, 4, -1, -2, -3, -4],
                unit: "ratio",
              }),
            ],
          },
          ...sdPatterns.flatMap(({ nameAddon, titleAddon, sd }) => ({
            name: nameAddon,
            title: `${title} ${titleAddon} ZScore`,
            top: [
              s({
                metric: priceMetric,
                name: legend,
                color,
              }),
              ...getSdBands(sd).map(({ name, prop, color }) =>
                s({
                  metric: prop,
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
                metric: sd.zscore,
                title: "score",
                type: "Baseline",
              }),
              ...createPriceLines({
                numbers: [0, 1, 2, 3, 4, -1, -2, -3, -4],
                unit: "ratio",
              }),
            ],
          })),
        ],
      },
    ];
  }

  /**
   * @typedef {Object} UtxoCohortObject
   * @property {string} name
   * @property {string} title
   * @property {Color} color
   * @property {UtxoCohortPattern} tree
   */

  /**
   * @typedef {Object} AddressCohortObject
   * @property {string} name
   * @property {string} title
   * @property {Color} color
   * @property {AddressCohortPattern} tree
   */

  /** @typedef {UtxoCohortObject | AddressCohortObject} CohortObject */

  /**
   * @typedef {Object} UtxoCohortGroupObject
   * @property {string} name
   * @property {string} title
   * @property {readonly UtxoCohortObject[]} list
   */

  /**
   * @typedef {Object} AddressCohortGroupObject
   * @property {string} name
   * @property {string} title
   * @property {readonly AddressCohortObject[]} list
   */

  /** @typedef {UtxoCohortGroupObject | AddressCohortGroupObject} CohortGroupObject */

  /**
   * @param {CohortObject | CohortGroupObject} args
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
              bottom: list.flatMap(({ color, name, tree }) => {
                return /** @type {const} */ ([
                  s({
                    metric: tree.supply.supply.sats,
                    name: "Supply",
                    color: colors.default,
                  }),
                  s({
                    metric: tree.supply.supply.bitcoin,
                    name: "Supply",
                    color: colors.default,
                  }),
                  s({
                    metric: tree.supply.supply.dollars,
                    name: "Supply",
                    color: colors.default,
                  }),
                  ...("supplyRelToCirculatingSupply" in tree.relative
                    ? [
                        s({
                          metric: tree.relative.supplyRelToCirculatingSupply,
                          name: "Supply",
                          color: colors.default,
                        }),
                      ]
                    : []),
                  s({
                    metric: tree.unrealized.supplyInProfit.sats,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  s({
                    metric: tree.unrealized.supplyInProfit.bitcoin,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  s({
                    metric: tree.unrealized.supplyInProfit.dollars,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  s({
                    metric: tree.unrealized.supplyInLoss.sats,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  s({
                    metric: tree.unrealized.supplyInLoss.bitcoin,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  s({
                    metric: tree.unrealized.supplyInLoss.dollars,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  s({
                    metric: tree.supply.supplyHalf.sats,
                    name: "half",
                    color: colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  s({
                    metric: tree.supply.supplyHalf.bitcoin,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  s({
                    metric: tree.supply.supplyHalf.dollars,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  ...("supplyInProfitRelToCirculatingSupply" in tree.relative
                    ? [
                        s({
                          metric:
                            tree.relative.supplyInProfitRelToCirculatingSupply,
                          name: "In Profit",
                          color: colors.green,
                        }),
                        s({
                          metric:
                            tree.relative.supplyInLossRelToCirculatingSupply,
                          name: "In Loss",
                          color: colors.red,
                        }),
                      ]
                    : []),
                  s({
                    metric: tree.relative.supplyInProfitRelToOwnSupply,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  s({
                    metric: tree.relative.supplyInLossRelToOwnSupply,
                    name: "In Loss",
                    color: colors.red,
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
                  bottom: list.flatMap(({ color, name, tree }) => {
                    return /** @type {const} */ ([
                      s({
                        metric: tree.supply.supply.sats,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.supply.supply.bitcoin,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.supply.supply.dollars,
                        name,
                        color,
                      }),
                      "supplyRelToCirculatingSupply" in tree.relative
                        ? s({
                            metric: tree.relative.supplyRelToCirculatingSupply,
                            name,
                            color,
                          })
                        : s({
                            unit: "%all",
                            metric: constant100,
                            name,
                            color,
                          }),
                    ]);
                  }),
                },
                {
                  name: "in profit",
                  title: `Supply In Profit ${title}`,
                  bottom: list.flatMap(({ color, name, tree }) => {
                    return /** @type {const} */ ([
                      s({
                        metric: tree.unrealized.supplyInProfit.sats,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.unrealized.supplyInProfit.bitcoin,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.unrealized.supplyInProfit.dollars,
                        name,
                        color,
                      }),
                      ...("supplyInProfitRelToCirculatingSupply" in
                      tree.relative
                        ? [
                            s({
                              metric:
                                tree.relative
                                  .supplyInProfitRelToCirculatingSupply,
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
                  bottom: list.flatMap(({ color, name, tree }) => {
                    return /** @type {const} */ ([
                      s({
                        metric: tree.unrealized.supplyInLoss.sats,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.unrealized.supplyInLoss.bitcoin,
                        name,
                        color,
                      }),
                      s({
                        metric: tree.unrealized.supplyInLoss.dollars,
                        name,
                        color,
                      }),
                      ...("supplyInLossRelToCirculatingSupply" in tree.relative
                        ? [
                            s({
                              metric:
                                tree.relative
                                  .supplyInLossRelToCirculatingSupply,
                              name,
                              color,
                            }),
                          ]
                        : []),
                    ]);
                  }),
                },
              ],
            },
        {
          name: "utxo count",
          title: `UTXO Count ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => {
            return /** @type {const} */ ([
              s({
                metric: tree.supply.utxoCount,
                name: useGroupName ? name : "Count",
                color,
              }),
            ]);
          }),
        },
        ...(list.filter(({ tree }) => tree.addrCount).length >
        ("list" in args ? 1 : 0)
          ? !("list" in args) ||
            list.filter(({ tree }) => tree.emptyAddrCount).length <= 1
            ? [
                {
                  name: "address count",
                  title: `Address Count ${title}`,
                  bottom: list.flatMap(({ name, color, tree }) => {
                    return [
                      ...(tree.addrCount
                        ? /** @type {const} */ ([
                            s({
                              metric: tree.addrCount,
                              name: useGroupName ? name : "Loaded",
                              color: useGroupName ? color : colors.orange,
                            }),
                          ])
                        : []),
                      ...(tree.emptyAddrCount
                        ? /** @type {const} */ ([
                            s({
                              metric: tree.emptyAddrCount,
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
                        .filter(({ tree }) => tree.addrCount)
                        .flatMap(({ name, color, tree }) => {
                          return [
                            s({
                              metric: tree.addrCount,
                              name,
                              color,
                            }),
                          ];
                        }),
                    },
                    ...(list.filter(({ tree }) => tree.emptyAddrCount).length
                      ? [
                          {
                            name: "empty",
                            title: `Empty Address Count ${title}`,
                            bottom: list
                              .filter(({ tree }) => tree.emptyAddrCount)
                              .flatMap(({ name, color, tree }) => {
                                return [
                                  s({
                                    metric: tree.emptyAddrCount,
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
                    top: list.map(({ color, name, tree }) =>
                      s({
                        metric: tree.realizedPrice,
                        name,
                        color,
                      }),
                    ),
                  },
                  {
                    name: "Ratio",
                    title: `Realized Price Ratio ${title}`,
                    bottom: [
                      ...list.map(({ color, name, tree }) =>
                        s({
                          metric: tree.realizedPriceExtra.ratio,
                          name,
                          color,
                        }),
                      ),
                      createPriceLine({
                        unit: "ratio",
                        number: 1,
                      }),
                    ],
                  },
                ]
              : createPriceWithRatioOptions({
                  title: `Realized Price ${title}`,
                  price: args.tree.realizedPrice,
                  ratio: args.tree.realizedPriceExtra,
                  name: "price",
                  legend: "realized",
                  color: args.color,
                })),
            {
              name: "capitalization",
              title: `Realized Capitalization ${title}`,
              bottom: list.flatMap(({ color, name, tree }) => {
                return /** @type {const} */ ([
                  s({
                    metric: tree.realized.realizedCap,
                    name: useGroupName ? name : "Capitalization",
                    color,
                  }),
                  ...(!("list" in args)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: tree.realized.realizedCap30dDelta,
                          title: "30d change",
                          defaultActive: false,
                        }),
                        createPriceLine({
                          unit: "usd",
                          defaultActive: false,
                        }),
                      ]
                    : []),
                  ...(!("list" in args) &&
                  tree.realized?.realizedCapRelToOwnMarketCap
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: tree.realized.realizedCapRelToOwnMarketCap,
                          title: "ratio",
                          options: { baseValue: { price: 100 } },
                          colors: [colors.red, colors.green],
                        }),
                        createPriceLine({
                          unit: "%cmcap",
                          defaultActive: true,
                          number: 100,
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
                      s({
                        metric: args.tree.realized.realizedProfit.base,
                        name: "Profit",
                        color: colors.green,
                      }),
                      s({
                        metric: args.tree.realized.realizedLoss.base,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      ...(args.tree.realized?.realizedProfitToLossRatio
                        ? [
                            s({
                              metric:
                                args.tree.realized.realizedProfitToLossRatio,
                              name: "proft / loss",
                              color: colors.yellow,
                            }),
                          ]
                        : []),
                      s({
                        metric: args.tree.realized.totalRealizedPnl.base,
                        name: "Total",
                        color: colors.default,
                        defaultActive: false,
                      }),
                      s({
                        metric: args.tree.realized.negRealizedLoss.base,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      s({
                        metric: args.tree.realized.realizedProfit.cumulative,
                        name: "Cumulative Profit",
                        color: colors.green,
                        defaultActive: false,
                      }),
                      s({
                        metric: args.tree.realized.realizedLoss.cumulative,
                        name: "Cumulative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      s({
                        metric: args.tree.realized.negRealizedLoss.cumulative,
                        name: "Cumulative Negative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric:
                          args.tree.realized.realizedProfitRelToRealizedCap,
                        title: "Profit",
                        color: colors.green,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: args.tree.realized.realizedLossRelToRealizedCap,
                        title: "Loss",
                        color: colors.red,
                      }),
                      createPriceLine({
                        unit: "%rcap",
                      }),
                      createPriceLine({
                        unit: "usd",
                        defaultActive: false,
                      }),
                    ],
                  },
                  {
                    name: "Net pnl",
                    title: `Net Realized Profit And Loss ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: tree.realized.netRealizedPnl.base,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: tree.realized.netRealizedPnl.cumulative,
                        title: "Cumulative",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: tree.realized.netRealizedPnlCumulative30dDelta,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: tree.realized.netRealizedPnlRelToRealizedCap,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric:
                          tree.realized
                            .netRealizedPnlCumulative30dDeltaRelToRealizedCap,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric:
                          tree.realized
                            .netRealizedPnlCumulative30dDeltaRelToMarketCap,
                        title: "cumulative 30d change",
                      }),
                      createPriceLine({
                        unit: "%mcap",
                      }),
                      createPriceLine({
                        unit: "%rcap",
                      }),
                      createPriceLine({
                        unit: "usd",
                      }),
                    ]),
                  },
                  {
                    name: "sopr",
                    title: `Spent Output Profit Ratio ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: tree.realized.sopr,
                          title: "normal",
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(tree.realized?.adjustedSopr
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: tree.realized.adjustedSopr,
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
                          metric: tree.realized.sopr7dEma,
                          title: "7d ema",
                          colors: [colors.lime, colors.rose],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(tree.realized?.adjustedSopr7dEma
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: tree.realized.adjustedSopr7dEma,
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
                          metric: tree.realized.sopr30dEma,
                          title: "30d ema",
                          colors: [colors.avocado, colors.pink],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(tree.realized?.adjustedSopr30dEma
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: tree.realized.adjustedSopr30dEma,
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
                          unit: "ratio",
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
                      ...list.flatMap(({ color, name, tree }) => {
                        return /** @type {const} */ ([
                          s({
                            metric: tree.realized.realizedProfit.base,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            metric:
                              tree.realized.realizedProfitRelToRealizedCap,
                            title: name,
                            color,
                          }),
                        ]);
                      }),
                      createPriceLine({
                        unit: "usd",
                      }),
                    ],
                  },
                  {
                    name: "loss",
                    title: `Realized Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, tree }) => {
                        return /** @type {const} */ ([
                          s({
                            metric: tree.realized.realizedLoss.base,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            metric: tree.realized.realizedLossRelToRealizedCap,
                            title: name,
                            color,
                          }),
                        ]);
                      }),
                      createPriceLine({
                        unit: "usd",
                      }),
                    ],
                  },
                  {
                    name: "Total pnl",
                    title: `Total Realized Profit And Loss Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, tree }) => {
                        return /** @type {const} */ ([
                          s({
                            metric: tree.realized.totalRealizedPnl.base,
                            name,
                            color,
                          }),
                          ...(tree.realized?.realizedProfitToLossRatio
                            ? [
                                s({
                                  metric:
                                    tree.realized.realizedProfitToLossRatio,
                                  name,
                                  color,
                                }),
                              ]
                            : []),
                        ]);
                      }),
                    ],
                  },
                  {
                    name: "Net pnl",
                    title: `Net Realized Profit And Loss ${title}`,
                    bottom: [
                      ...list.flatMap(({ color, name, tree }) => [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: tree.realized.netRealizedPnl.base,
                          title: name,
                          color,
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: tree.realized.netRealizedPnlRelToRealizedCap,
                          title: name,
                          color,
                        }),
                      ]),
                      createPriceLine({
                        unit: "usd",
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
                        bottom: list.flatMap(({ color, name, tree }) => {
                          return /** @type {const} */ ([
                            s({
                              metric: tree.realized.realizedProfit.cumulative,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "loss",
                        title: `Cumulative Realized Loss ${title}`,
                        bottom: list.flatMap(({ color, name, tree }) => {
                          return /** @type {const} */ ([
                            s({
                              metric: tree.realized.realizedLoss.cumulative,
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
                          ...list.flatMap(({ color, name, tree }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: tree.realized.netRealizedPnl.cumulative,
                              title: name,
                              color,
                              defaultActive: false,
                            }),
                          ]),
                          createPriceLine({
                            unit: "usd",
                          }),
                        ],
                      },
                      {
                        name: "Net pnl 30d change",
                        title: `Cumulative Net Realized Profit And Loss 30 Day Change ${title}`,
                        bottom: [
                          ...list.flatMap(({ color, name, tree }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric:
                                tree.realized.netRealizedPnlCumulative30dDelta,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric:
                                tree.realized
                                  .netRealizedPnlCumulative30dDeltaRelToRealizedCap,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric:
                                tree.realized
                                  .netRealizedPnlCumulative30dDeltaRelToMarketCap,
                              title: name,
                              color,
                            }),
                          ]),
                          createPriceLine({
                            unit: "usd",
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
                          ...list.flatMap(({ color, name, tree }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: tree.realized.sopr,
                              title: name,
                              color,
                            }),
                          ]),
                          createPriceLine({
                            number: 1,
                            unit: "ratio",
                          }),
                        ],
                      },
                      ...(() => {
                        const reducedList = list
                          .filter(({ tree }) => tree.realized?.adjustedSopr)
                          .map(({ color, name, tree }) => ({
                            color,
                            name,
                            metric: tree.realized.adjustedSopr,
                          }));

                        return reducedList.length
                          ? [
                              {
                                name: "Adjusted",
                                title: `Adjusted Spent Output Profit Ratio ${title}`,
                                bottom: [
                                  ...reducedList.flatMap(
                                    ({ color, name, metric }) => [
                                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                        type: "Baseline",
                                        metric,
                                        title: name,
                                        color,
                                      }),
                                    ],
                                  ),
                                  createPriceLine({
                                    number: 1,
                                    unit: "ratio",
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
                ? list.flatMap(({ tree }) => [
                    s({
                      metric: tree.realized.sellSideRiskRatio,
                      name: "raw",
                      color: colors.orange,
                    }),
                    s({
                      metric: tree.realized.sellSideRiskRatio7dEma,
                      name: "7d ema",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    s({
                      metric: tree.realized.sellSideRiskRatio30dEma,
                      name: "30d ema",
                      color: colors.rose,
                      defaultActive: false,
                    }),
                  ])
                : list.flatMap(({ color, name, tree }) => [
                    s({
                      metric: tree.realized.sellSideRiskRatio,
                      name,
                      color: color,
                    }),
                  ]),
            },
            {
              name: "value",
              tree: [
                ...(!("list" in args)
                  ? [
                      {
                        name: "created",
                        title: `value created ${title}`,
                        bottom: list.flatMap(({ color, name, tree }) => {
                          return [
                            s({
                              metric: tree.realized.valueCreated,
                              name: "normal",
                              color: colors.emerald,
                            }),
                            ...(tree.realized?.adjustedValueCreated
                              ? [
                                  s({
                                    metric: tree.realized.adjustedValueCreated,
                                    name: "adjusted",
                                    color: colors.lime,
                                  }),
                                ]
                              : []),
                          ];
                        }),
                      },
                      {
                        name: "destroyed",
                        title: `value destroyed ${title}`,
                        bottom: list.flatMap(({ color, name, tree }) => {
                          return [
                            s({
                              metric: tree.realized.valueDestroyed,
                              name: "normal",
                              color: colors.red,
                            }),
                            ...(tree.realized?.adjustedValueDestroyed
                              ? [
                                  s({
                                    metric:
                                      tree.realized.adjustedValueDestroyed,
                                    name: "adjusted",
                                    color: colors.pink,
                                  }),
                                ]
                              : []),
                          ];
                        }),
                      },
                    ]
                  : [
                      {
                        name: "created",
                        tree: [
                          {
                            name: "Normal",
                            title: `Value Created ${title}`,
                            bottom: list.flatMap(({ color, name, tree }) => [
                              s({
                                metric: tree.realized.valueCreated,
                                name,
                                color,
                              }),
                            ]),
                          },
                          ...(() => {
                            const reducedList = list
                              .filter(
                                ({ tree }) =>
                                  tree.realized?.adjustedValueCreated,
                              )
                              .map(({ color, name, tree }) => ({
                                color,
                                name,
                                metric: tree.realized.adjustedValueCreated,
                              }));
                            return reducedList.length
                              ? [
                                  {
                                    name: "Adjusted",
                                    title: `Adjusted value created ${title}`,
                                    bottom: reducedList.map(
                                      ({ color, name, metric }) =>
                                        s({
                                          metric,
                                          name,
                                          color,
                                        }),
                                    ),
                                  },
                                ]
                              : [];
                          })(),
                        ],
                      },
                      {
                        name: "destroyed",
                        tree: [
                          {
                            name: "Normal",
                            title: `Value destroyed ${title}`,
                            bottom: list.flatMap(({ color, name, tree }) => [
                              s({
                                metric: tree.realized.valueDestroyed,
                                name,
                                color,
                              }),
                            ]),
                          },
                          ...(() => {
                            const reducedList = list
                              .filter(
                                ({ tree }) =>
                                  tree.realized?.adjustedValueDestroyed,
                              )
                              .map(({ color, name, tree }) => ({
                                color,
                                name,
                                metric: tree.realized.adjustedValueDestroyed,
                              }));
                            return reducedList.length
                              ? [
                                  {
                                    name: "Adjusted",
                                    title: `Adjusted value destroyed ${title}`,
                                    bottom: reducedList.map(
                                      ({ color, name, metric }) =>
                                        s({
                                          metric,
                                          name,
                                          color,
                                        }),
                                    ),
                                  },
                                ]
                              : [];
                          })(),
                        ],
                      },
                    ]),
              ],
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
                      s({
                        metric: args.tree.unrealized.totalUnrealizedPnl,
                        name: "total",
                        color: colors.default,
                      }),
                      s({
                        metric: args.tree.unrealized.unrealizedProfit,
                        name: "Profit",
                        color: colors.green,
                      }),
                      s({
                        metric: args.tree.unrealized.unrealizedLoss,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      s({
                        metric: args.tree.unrealized.negUnrealizedLoss,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      s({
                        metric:
                          args.tree.relative.unrealizedProfitRelToMarketCap,
                        name: "Profit",
                        color: colors.green,
                      }),
                      s({
                        metric: args.tree.relative.unrealizedLossRelToMarketCap,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      s({
                        metric:
                          args.tree.relative.negUnrealizedLossRelToMarketCap,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      ...("unrealizedProfitRelToOwnMarketCap" in
                      args.tree.relative
                        ? [
                            s({
                              metric:
                                args.tree.relative
                                  .unrealizedProfitRelToOwnMarketCap,
                              name: "Profit",
                              color: colors.green,
                            }),
                            s({
                              metric:
                                args.tree.relative
                                  .unrealizedLossRelToOwnMarketCap,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            s({
                              metric:
                                args.tree.relative
                                  .negUnrealizedLossRelToOwnMarketCap,
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
                      ...("unrealizedProfitRelToOwnTotalUnrealizedPnl" in
                      args.tree.relative
                        ? [
                            s({
                              metric:
                                args.tree.relative
                                  .unrealizedProfitRelToOwnTotalUnrealizedPnl,
                              name: "Profit",
                              color: colors.green,
                            }),
                            s({
                              metric:
                                args.tree.relative
                                  .unrealizedLossRelToOwnTotalUnrealizedPnl,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            s({
                              metric:
                                args.tree.relative
                                  .negUnrealizedLossRelToOwnTotalUnrealizedPnl,
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
                        unit: "usd",
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
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.unrealized.unrealizedProfit,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "loss",
                    title: `Unrealized Loss ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.unrealized.unrealizedLoss,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "total pnl",
                    title: `Unrealized Total Profit And Loss ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.unrealized.totalUnrealizedPnl,
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
                ...list.flatMap(({ color, name, tree }) => [
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    metric: tree.unrealized.netUnrealizedPnl,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    metric: tree.relative.netUnrealizedPnlRelToMarketCap,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  ...("netUnrealizedPnlRelToOwnMarketCap" in tree.relative
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric:
                            tree.relative.netUnrealizedPnlRelToOwnMarketCap,
                          title: useGroupName ? name : "Net",
                          color: useGroupName ? color : undefined,
                        }),
                        createPriceLine({
                          unit: "%cmcap",
                        }),
                      ]
                    : []),
                  ...("netUnrealizedPnlRelToOwnTotalUnrealizedPnl" in
                  tree.relative
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric:
                            tree.relative
                              .netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
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
                  unit: "usd",
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
                    top: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.realizedPrice,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Min",
                    title: `Min Cost Basis ${title}`,
                    top: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.pricePaid.minPricePaid,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Max",
                    title: `Max Cost Basis ${title}`,
                    top: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.pricePaid.maxPricePaid,
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
                  s({
                    metric: args.tree.realizedPrice,
                    name: "Average",
                    color: args.color,
                  }),
                  s({
                    metric: args.tree.pricePaid.minPricePaid,
                    name: "Min",
                    color: colors.green,
                    defaultActive: false,
                  }),
                  s({
                    metric: args.tree.pricePaid.maxPricePaid,
                    name: "Max",
                    color: colors.red,
                  }),
                ],
              },
            ]),
        ...("list" in args
          ? [
              {
                name: "Coins Destroyed",
                tree: [
                  {
                    name: "Sum",
                    title: `Sum of Coins Destroyed ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.activity.coinblocksDestroyed.base,
                          name,
                          color,
                        }),
                        s({
                          metric: tree.activity.coindaysDestroyed.base,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Cumulative",
                    title: `Cumulative Coins Destroyed ${title}`,
                    bottom: list.flatMap(({ color, name, tree }) => {
                      return /** @type {const} */ ([
                        s({
                          metric: tree.activity.coinblocksDestroyed.cumulative,
                          name,
                          color,
                        }),
                        s({
                          metric: tree.activity.coindaysDestroyed.cumulative,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                ],
              },
            ]
          : [
              {
                name: "Coins Destroyed",
                title: `Coins Destroyed ${title}`,
                bottom: list.flatMap(({ color, name, tree }) => {
                  return /** @type {const} */ ([
                    s({
                      metric: tree.activity.coinblocksDestroyed.base,
                      name: "sum",
                      color,
                    }),
                    s({
                      metric: tree.activity.coinblocksDestroyed.cumulative,
                      name: "cumulative",
                      color,
                      defaultActive: false,
                    }),
                    s({
                      metric: tree.activity.coindaysDestroyed.base,
                      name: "sum",
                      color,
                    }),
                    s({
                      metric: tree.activity.coindaysDestroyed.cumulative,
                      name: "cumulative",
                      color,
                      defaultActive: false,
                    }),
                  ]);
                }),
              },
            ]),
      ],
    });
  }

  return [
    ...(localhost
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
                s({
                  metric: "market_cap",
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
                s({
                  metric: "price_ath",
                  name: "ath",
                }),
              ],
              bottom: [
                s({
                  metric: "price_drawdown",
                  name: "Drawdown",
                  color: colors.red,
                }),
                s({
                  metric: "days_since_price_ath",
                  name: "since",
                }),
                s({
                  metric: "max_days_between_price_aths",
                  name: "Max",
                  color: colors.red,
                }),
                s({
                  metric: "max_years_between_price_aths",
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
                  metricAddon: "sma",
                },
                {
                  nameAddon: "Exponential",
                  metricAddon: "ema",
                },
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
                      }),
                    ),
                  },
                  ...averages.map(({ name, color, sma, ema }) => ({
                    name,
                    tree: createPriceWithRatioOptions({
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
            {
              name: "Performance",
              tree: /** @type {const} */ ([
                ["1d", market._1dPriceReturns],
                ["1w", market._1wPriceReturns],
                ["1m", market._1mPriceReturns],
                ["3m", market._3mPriceReturns],
                ["6m", market._6mPriceReturns],
                ["1y", market._1yPriceReturns],
                ["2y", market._2yPriceReturns, market._2yCagr],
                ["3y", market._3yPriceReturns, market._3yCagr],
                ["4y", market._4yPriceReturns, market._4yCagr],
                ["5y", market._5yPriceReturns, market._5yCagr],
                ["6y", market._6yPriceReturns, market._6yCagr],
                ["8y", market._8yPriceReturns, market._8yCagr],
                ["10y", market._10yPriceReturns, market._10yCagr],
              ]).map(([id, priceReturns, cagr]) => {
                const name = periodIdToName(id, true);
                return {
                  name,
                  title: `${name} Performance`,
                  bottom: [
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: priceReturns,
                      title: "total",
                      type: "Baseline",
                    }),
                    ...(cagr
                      ? [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: cagr,
                            title: "cagr",
                            type: "Baseline",
                            colors: [colors.lime, colors.pink],
                          }),
                        ]
                      : []),
                    createPriceLine({
                      unit: "percentage",
                    }),
                  ],
                };
              }),
            },
            {
              name: "Indicators",
              tree: [
                {
                  name: "Volatility",
                  title: "Bitcoin Price Volatility Index",
                  bottom: [
                    s({
                      metric: "price_1w_volatility",
                      name: "1w",
                      color: colors.red,
                    }),
                    s({
                      metric: "price_1m_volatility",
                      name: "1m",
                      color: colors.orange,
                    }),
                    s({
                      metric: "price_1y_volatility",
                      name: "1y",
                      color: colors.lime,
                    }),
                  ],
                },
                {
                  name: "MinMax",
                  tree: [
                    {
                      metric: "1w",
                      title: "1 Week",
                    },
                    {
                      metric: "2w",
                      title: "2 Week",
                    },
                    {
                      metric: "1m",
                      title: "1 Month",
                    },
                    {
                      metric: "1y",
                      title: "1 Year",
                    },
                  ].map(({ metric, title }) => ({
                    name: metric,
                    title: `Bitcoin Price ${title} MinMax Bands`,
                    top: [
                      s({
                        metric: `price_${metric}_min`,
                        name: "min",
                        color: colors.red,
                      }),
                      s({
                        metric: `price_${metric}_max`,
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
                    s({
                      metric: "price_true_range",
                      name: "value",
                      color: colors.yellow,
                    }),
                  ],
                },
                {
                  name: "Choppiness",
                  title: "Bitcoin Price Choppiness Index",
                  bottom: [
                    s({
                      metric: "price_2w_choppiness_index",
                      name: "2w",
                      color: colors.red,
                    }),
                    createPriceLine({
                      unit: "index",
                      number: 61.8,
                    }),
                    createPriceLine({
                      unit: "index",
                      number: 38.2,
                    }),
                  ],
                },
                {
                  name: "Mayer multiple",
                  title: "Mayer multiple",
                  top: [
                    s({
                      metric: `price_200d_sma`,
                      name: "200d sma",
                      color: colors.yellow,
                    }),
                    s({
                      metric: `price_200d_sma_x2_4`,
                      name: "200d sma x2.4",
                      color: colors.green,
                    }),
                    s({
                      metric: `price_200d_sma_x0_8`,
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
                      [
                        "1w",
                        market._1wDcaAvgPrice,
                        market.price1wAgo,
                        market._1wDcaReturns,
                        market._1wPriceReturns,
                      ],
                      [
                        "1m",
                        market._1mDcaAvgPrice,
                        market.price1mAgo,
                        market._1mDcaReturns,
                        market._1mPriceReturns,
                      ],
                      [
                        "3m",
                        market._3mDcaAvgPrice,
                        market.price3mAgo,
                        market._3mDcaReturns,
                        market._3mPriceReturns,
                      ],
                      [
                        "6m",
                        market._6mDcaAvgPrice,
                        market.price6mAgo,
                        market._6mDcaReturns,
                        market._6mPriceReturns,
                      ],
                      [
                        "1y",
                        market._1yDcaAvgPrice,
                        market.price1yAgo,
                        market._1yDcaReturns,
                        market._1yPriceReturns,
                      ],
                    ]).map(
                      ([
                        id,
                        dcaAvgPrice,
                        priceAgo,
                        dcaReturns,
                        priceReturns,
                      ]) => {
                        const name = periodIdToName(id, true);

                        return /** @satisfies {PartialChartOption} */ ({
                          name,
                          title: `${name} DCA vs Lump Sum Returns`,
                          top: [
                            s({
                              metric: dcaAvgPrice,
                              name: `dca`,
                              color: colors.orange,
                            }),
                            s({
                              metric: priceAgo,
                              name: `lump sum`,
                              color: colors.cyan,
                            }),
                          ],
                          bottom: [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: dcaReturns,
                              title: "dca",
                              type: "Baseline",
                              colors: [colors.yellow, colors.pink],
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: priceReturns,
                              title: "lump sum",
                              type: "Baseline",
                            }),
                            createPriceLine({
                              unit: "percentage",
                            }),
                          ],
                        });
                      },
                    ),
                    .../** @type {const} */ ([
                      [
                        "2y",
                        market._2yDcaAvgPrice,
                        market.price2yAgo,
                        market._2yDcaReturns,
                        market._2yPriceReturns,
                        market._2yDcaCagr,
                        market._2yCagr,
                      ],
                      [
                        "3y",
                        market._3yDcaAvgPrice,
                        market.price3yAgo,
                        market._3yDcaReturns,
                        market._3yPriceReturns,
                        market._3yDcaCagr,
                        market._3yCagr,
                      ],
                      [
                        "4y",
                        market._4yDcaAvgPrice,
                        market.price4yAgo,
                        market._4yDcaReturns,
                        market._4yPriceReturns,
                        market._4yDcaCagr,
                        market._4yCagr,
                      ],
                      [
                        "5y",
                        market._5yDcaAvgPrice,
                        market.price5yAgo,
                        market._5yDcaReturns,
                        market._5yPriceReturns,
                        market._5yDcaCagr,
                        market._5yCagr,
                      ],
                      [
                        "6y",
                        market._6yDcaAvgPrice,
                        market.price6yAgo,
                        market._6yDcaReturns,
                        market._6yPriceReturns,
                        market._6yDcaCagr,
                        market._6yCagr,
                      ],
                      [
                        "8y",
                        market._8yDcaAvgPrice,
                        market.price8yAgo,
                        market._8yDcaReturns,
                        market._8yPriceReturns,
                        market._8yDcaCagr,
                        market._8yCagr,
                      ],
                      [
                        "10y",
                        market._10yDcaAvgPrice,
                        market.price10yAgo,
                        market._10yDcaReturns,
                        market._10yPriceReturns,
                        market._10yDcaCagr,
                        market._10yCagr,
                      ],
                    ]).map(
                      ([
                        id,
                        dcaAvgPrice,
                        priceAgo,
                        dcaReturns,
                        priceReturns,
                        dcaCagr,
                        cagr,
                      ]) => {
                        const name = periodIdToName(id, true);
                        return /** @satisfies {PartialChartOption} */ ({
                          name,
                          title: `${name} DCA vs Lump Sum Returns`,
                          top: [
                            s({
                              metric: dcaAvgPrice,
                              name: `dca`,
                              color: colors.orange,
                            }),
                            s({
                              metric: priceAgo,
                              name: `lump sum`,
                              color: colors.cyan,
                            }),
                          ],
                          bottom: [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: dcaReturns,
                              title: "dca",
                              type: "Baseline",
                              colors: [colors.yellow, colors.pink],
                            }),

                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: priceReturns,
                              title: "lump sum",
                              type: "Baseline",
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: dcaCagr,
                              title: "dca cagr",
                              type: "Baseline",
                              colors: [colors.yellow, colors.pink],
                              defaultActive: false,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: cagr,
                              title: "lump sum cagr",
                              type: "Baseline",
                              defaultActive: false,
                            }),
                            createPriceLine({
                              unit: "percentage",
                            }),
                          ],
                        });
                      },
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
                        s({
                          metric: `dca_class_${year}_avg_price`,
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
                            s({
                              metric: `dca_class_${year}_avg_price`,
                              name: "cost basis",
                              color,
                            }),
                          ],
                          bottom: [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              metric: `dca_class_${year}_returns`,
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
                      metric: "block_count",
                    }),
                    s({
                      metric: "block_count_target",
                      name: "Target",
                      color: colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    s({
                      metric: "1w_block_count",
                      name: "1w sum",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    s({
                      metric: "1m_block_count",
                      name: "1m sum",
                      color: colors.pink,
                      defaultActive: false,
                    }),
                    s({
                      metric: "1y_block_count",
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
                    s({
                      metric: "interval",
                      name: "Interval",
                    }),
                    createAverageSeries("block_interval"),
                    ...createMinMaxPercentilesSeries("block_interval"),
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
                    s({
                      metric: "total_size",
                      name: "raw",
                    }),
                    s({
                      metric: "vbytes",
                      name: "raw",
                    }),
                    s({
                      metric: "weight",
                      name: "raw",
                    }),
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
                      metric: "tx_count",
                      name: "Count",
                    },
                  ),
                },
                {
                  name: "Volume",
                  title: "Transaction Volume",
                  bottom: [
                    s({
                      metric: "sent",
                      name: "Sent",
                    }),
                    s({
                      metric: "sent_btc",
                      name: "Sent",
                    }),
                    s({
                      metric: "sent_usd",
                      name: "Sent",
                    }),
                    s({
                      metric: "annualized_volume",
                      name: "annualized",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    s({
                      metric: "annualized_volume_btc",
                      name: "annualized",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    s({
                      metric: "annualized_volume_usd",
                      name: "annualized",
                      color: colors.lime,
                      defaultActive: false,
                    }),
                  ],
                },
                {
                  name: "Size",
                  title: "Transaction Size",
                  bottom: [
                    createAverageSeries("tx_weight"),
                    ...createMinMaxPercentilesSeries("tx_weight"),
                    createAverageSeries("tx_vsize"),
                    ...createMinMaxPercentilesSeries("tx_vsize"),
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
                      metric: `tx_v${index + 1}`,
                      common: `v${index + 1}`,
                      sumColor,
                      cumulativeColor,
                    }),
                  ),
                },
                {
                  name: "Velocity",
                  title: "Transactions Velocity",
                  bottom: [
                    s({
                      metric: "tx_btc_velocity",
                      name: "bitcoin",
                    }),
                    s({
                      metric: "tx_usd_velocity",
                      name: "dollars",
                      color: colors.emerald,
                    }),
                  ],
                },
                {
                  name: "Speed",
                  title: "Transactions Per Second",
                  bottom: [
                    s({
                      metric: "tx_per_sec",
                      name: "Transactions",
                    }),
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
                    createAverageSeries("input_count"),
                    createCumulativeSeries({ metric: "input_count" }),
                    ...createMinMaxPercentilesSeries("input_count"),
                  ],
                },
                {
                  name: "Speed",
                  title: "Inputs Per Second",
                  bottom: [
                    s({
                      metric: "inputs_per_sec",
                      name: "Inputs",
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
                    createAverageSeries("output_count"),
                    createCumulativeSeries({ metric: "output_count" }),
                    ...createMinMaxPercentilesSeries("output_count"),
                  ],
                },
                {
                  name: "Speed",
                  title: "Outputs Per Second",
                  bottom: [
                    s({
                      metric: "outputs_per_sec",
                      name: "Outputs",
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
                    s({
                      metric: "supply",
                      name: "Mined",
                    }),
                    s({
                      metric: "supply_btc",
                      name: "Mined",
                    }),
                    s({
                      metric: "supply_usd",
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
                            metric: "coinbase",
                            name: "Coinbase",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            metric: "coinbase_btc",
                            name: "Coinbase",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            metric: "coinbase_usd",
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
                            metric: "subsidy",
                            name: "Subsidy",
                          },
                        ),
                        s({
                          metric: "subsidy_usd_1y_sma",
                          name: "1y sma",
                        }),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            metric: "subsidy_btc",
                            name: "Subsidy",
                          },
                        ),
                        ...createBaseAverageSumCumulativeMinMaxPercentilesSeries(
                          {
                            metric: "subsidy_usd",
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
                          "fee_btc",
                        ),
                        ...createAverageSumCumulativeMinMaxPercentilesSeries(
                          "fee_usd",
                        ),
                      ],
                    },
                    {
                      name: "Dominance",
                      title: "Reward Dominance",
                      bottom: [
                        s({
                          metric: "fee_dominance",
                          name: "Fee",
                          color: colors.amber,
                        }),
                        s({
                          metric: "subsidy_dominance",
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
                          metric: "unclaimed_rewards",
                        }),
                        ...createSumCumulativeSeries({
                          metric: "unclaimed_rewards_btc",
                        }),
                        ...createSumCumulativeSeries({
                          metric: "unclaimed_rewards_usd",
                        }),
                      ],
                    },
                    {
                      name: "Inflation",
                      title: "Inflation Rate",
                      bottom: [
                        s({
                          metric: "inflation_rate",
                          name: "Inflation",
                        }),
                      ],
                    },
                    {
                      name: "Puell multiple",
                      title: "Puell multiple",
                      bottom: [
                        s({
                          metric: "puell_multiple",
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
                    createAverageSeries("fee_rate"),
                    ...createMinMaxPercentilesSeries("fee_rate"),
                  ],
                },
                {
                  name: "Halving",
                  title: "Halving Epoch",
                  bottom: [
                    s({
                      metric: "halvingepoch",
                      name: "Halving",
                    }),
                    s({
                      metric: "blocks_before_next_halving",
                      name: "Before next",
                    }),
                    s({
                      metric: "days_before_next_halving",
                      name: "Before next",
                    }),
                  ],
                },
                {
                  name: "Difficulty",
                  title: "Difficulty",
                  bottom: [
                    s({
                      metric: "difficulty",
                      name: "Value",
                    }),
                    s({
                      metric: "difficultyepoch",
                      name: "Difficulty",
                    }),
                    s({
                      metric: "blocks_before_next_difficulty_adjustment",
                      name: "Before next",
                    }),
                    s({
                      metric: "days_before_next_difficulty_adjustment",
                      name: "Before next",
                    }),
                  ],
                },
                {
                  name: "adjustment",
                  title: "Difficulty adjustment",
                  bottom: [
                    {
                      metric: "difficulty_adjustment",
                      title: "difficulty change",
                      type: "Baseline",
                    },
                  ],
                },
                {
                  name: "hash",
                  tree: [
                    {
                      name: "Rate",
                      title: "Hash Rate",
                      bottom: [
                        s({
                          metric: "hash_rate",
                          name: "Raw",
                        }),
                        s({
                          metric: "hash_rate_1w_sma",
                          name: "1w sma",
                          color: colors.red,
                          defaultActive: false,
                        }),
                        s({
                          metric: "hash_rate_1m_sma",
                          name: "1m sma",
                          color: colors.pink,
                          defaultActive: false,
                        }),
                        s({
                          metric: "hash_rate_2m_sma",
                          name: "2m sma",
                          color: colors.purple,
                          defaultActive: false,
                        }),
                        s({
                          metric: "hash_rate_1y_sma",
                          name: "1y sma",
                          color: colors.indigo,
                          defaultActive: false,
                        }),
                        s({
                          metric: "difficulty_as_hash",
                          name: "difficulty",
                          color: colors.default,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                      ],
                    },
                    {
                      name: "Price",
                      title: "Hash Price",
                      bottom: [
                        s({
                          metric: "hash_price_ths",
                          name: "Dollars",
                          color: colors.emerald,
                        }),
                        s({
                          metric: "hash_price_phs",
                          name: "Dollars",
                          color: colors.emerald,
                        }),
                        s({
                          metric: "hash_price_rebound",
                          name: "Rebound",
                          color: colors.yellow,
                        }),
                        s({
                          metric: "hash_price_ths_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                        s({
                          metric: "hash_price_phs_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                      ],
                    },
                    {
                      name: "Value",
                      title: "Hash Value",
                      bottom: [
                        s({
                          metric: "hash_value_ths",
                          name: "Sats",
                          color: colors.orange,
                        }),
                        s({
                          metric: "hash_value_phs",
                          name: "Sats",
                          color: colors.orange,
                        }),
                        s({
                          metric: "hash_value_rebound",
                          name: "Rebound",
                          color: colors.yellow,
                        }),
                        s({
                          metric: "hash_value_ths_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                        s({
                          metric: "hash_value_phs_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                      ],
                    },
                  ],
                },
                {
                  name: "Pools",
                  tree: entries(brk.tree.computed.pools.vecs).map(
                    ([key, pool]) => {
                      const name =
                        brk.POOL_ID_TO_POOL_NAME[key.toLowerCase()] || key;
                      return {
                        name,
                        tree: [
                          {
                            name: "Dominance",
                            title: `Mining Dominance of ${name}`,
                            bottom: [
                              s({
                                metric: pool._1dDominance.base,
                                name: "1d",
                                color: colors.rose,
                                defaultActive: false,
                              }),
                              s({
                                metric: pool._1wDominance,
                                name: "1w",
                                color: colors.red,
                                defaultActive: false,
                              }),
                              s({
                                metric: pool._1mDominance,
                                name: "1m",
                              }),
                              s({
                                metric: pool._1yDominance,
                                name: "1y",
                                color: colors.lime,
                                defaultActive: false,
                              }),
                              s({
                                metric: pool.dominance.base,
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
                              s({
                                metric: pool.blocksMined.base,
                                name: "Sum",
                              }),
                              s({
                                metric: pool.blocksMined.cumulative,
                                name: "Cumulative",
                                color: colors.blue,
                              }),
                              s({
                                metric: pool._1wBlocksMined,
                                name: "1w Sum",
                                color: colors.red,
                                defaultActive: false,
                              }),
                              s({
                                metric: pool._1mBlocksMined,
                                name: "1m Sum",
                                color: colors.pink,
                                defaultActive: false,
                              }),
                              s({
                                metric: pool._1yBlocksMined,
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
                                pattern: pool.coinbase,
                                label: "coinbase",
                                cumulativeColor: colors.red,
                                sumColor: colors.orange,
                              },
                              {
                                pattern: pool.subsidy,
                                label: "subsidy",
                                cumulativeColor: colors.emerald,
                                sumColor: colors.lime,
                              },
                              {
                                pattern: pool.fee,
                                label: "fee",
                                cumulativeColor: colors.indigo,
                                sumColor: colors.cyan,
                              },
                            ].flatMap(
                              ({
                                pattern,
                                label,
                                sumColor,
                                cumulativeColor,
                              }) => [
                                ...createSumCumulativeSeries({
                                  metric: pattern.sats,
                                  common: label,
                                  sumColor,
                                  cumulativeColor,
                                }),
                                ...createSumCumulativeSeries({
                                  metric: pattern.bitcoin,
                                  common: label,
                                  sumColor,
                                  cumulativeColor,
                                }),
                                ...createSumCumulativeSeries({
                                  metric: pattern.dollars,
                                  common: label,
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
                              s({
                                metric: pool.daysSinceBlock,
                                name: "Since block",
                              }),
                            ],
                          },
                        ],
                      };
                    },
                  ),
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
                    s({
                      metric: "unspendable_supply",
                      name: "Supply",
                    }),
                    s({
                      metric: "unspendable_supply_btc",
                      name: "Supply",
                    }),
                    s({
                      metric: "unspendable_supply_usd",
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
                        s({
                          metric: "opreturn_count",
                          name: "Count",
                        }),
                        s({
                          metric: "opreturn_count",
                          name: "sum",
                        }),
                        s({
                          metric: "opreturn_count_cumulative",
                          name: "cumulative",
                          color: colors.red,
                        }),
                      ],
                    },
                    {
                      name: "supply",
                      title: "OP_return Supply",
                      bottom: [
                        s({
                          metric: "opreturn_supply",
                          name: "Supply",
                        }),
                        s({
                          metric: "opreturn_supply_btc",
                          name: "Supply",
                        }),
                        s({
                          metric: "opreturn_supply_usd",
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
                  top: cointimePrices.map(({ price, name, color }) =>
                    s({
                      metric: price,
                      name,
                      color,
                    }),
                  ),
                },
                ...cointimePrices.map(
                  ({ price, ratio, name, color, title }) => ({
                    name,
                    tree: createPriceWithRatioOptions({
                      price,
                      ratio,
                      legend: name,
                      color,
                      name,
                      title,
                    }),
                  }),
                ),
              ],
            },
            {
              name: "Capitalization",
              tree: [
                {
                  name: "Compare",
                  title: "Compare Cointime Capitalizations",
                  bottom: [
                    s({
                      metric: `market_cap`,
                      name: "Market",
                      color: colors.default,
                    }),
                    s({
                      metric: `realized_cap`,
                      name: "Realized",
                      color: colors.orange,
                    }),
                    ...cointimeCapitalizations.map(({ metric, name, color }) =>
                      s({
                        metric,
                        name,
                        color,
                      }),
                    ),
                  ],
                },
                ...cointimeCapitalizations.map(
                  ({ metric, name, color, title }) => ({
                    name,
                    title,
                    bottom: [
                      s({
                        metric,
                        name,
                        color,
                      }),
                      s({
                        metric: `market_cap`,
                        name: "Market",
                        color: colors.default,
                      }),
                      s({
                        metric: `realized_cap`,
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
                [utxoCohorts.all.supply.supply, "all", colors.orange],
                [cointime.vaultedSupply, "vaulted", colors.lime],
                [cointime.activeSupply, "active", colors.rose],
              ]).flatMap(([supply, name, color]) => [
                s({ metric: supply.sats, name, color }),
                s({ metric: supply.bitcoin, name, color }),
                s({ metric: supply.dollars, name, color }),
              ]),
            },
            {
              name: "Liveliness & Vaultedness",
              title: "Liveliness & Vaultedness",
              bottom: [
                s({
                  metric: "liveliness",
                  name: "Liveliness",
                  color: colors.rose,
                }),
                s({
                  metric: "vaultedness",
                  name: "Vaultedness",
                  color: colors.lime,
                }),
                s({
                  metric: "activity_to_vaultedness_ratio",
                  name: "Liveliness / Vaultedness",
                  color: colors.purple,
                }),
              ],
            },
            {
              name: "Coinblocks",
              title: "Coinblocks",
              bottom: [
                s({
                  metric: "coinblocks_destroyed",
                  name: "Destroyed",
                  color: colors.red,
                }),
                s({
                  metric: "coinblocks_destroyed_cumulative",
                  name: "Cumulative Destroyed",
                  color: colors.red,
                  defaultActive: false,
                }),
                s({
                  metric: "coinblocks_created",
                  name: "created",
                  color: colors.orange,
                }),
                s({
                  metric: "coinblocks_created_cumulative",
                  name: "Cumulative created",
                  color: colors.orange,
                  defaultActive: false,
                }),
                s({
                  metric: "coinblocks_stored",
                  name: "stored",
                  color: colors.green,
                }),
                s({
                  metric: "coinblocks_stored_cumulative",
                  name: "Cumulative stored",
                  color: colors.green,
                  defaultActive: false,
                }),
              ],
            },
            {
              name: "Adjusted",
              tree: [
                {
                  name: "inflation",
                  title: "Cointime-Adjusted inflation rate",
                  bottom: [
                    s({
                      metric: "inflation_rate",
                      name: "base",
                      color: colors.orange,
                    }),
                    s({
                      metric: "cointime_adj_inflation_rate",
                      name: "base",
                      color: colors.purple,
                    }),
                  ],
                },
                {
                  name: "Velocity",
                  title: "Cointime-Adjusted transactions velocity",
                  bottom: [
                    s({
                      metric: "tx_btc_velocity",
                      name: "btc",
                      color: colors.orange,
                    }),
                    s({
                      metric: "cointime_adj_tx_btc_velocity",
                      name: "adj. btc",
                      color: colors.red,
                    }),
                    s({
                      metric: "tx_usd_velocity",
                      name: "usd",
                      color: colors.emerald,
                    }),
                    s({
                      metric: "cointime_adj_tx_usd_velocity",
                      name: "adj. usd",
                      color: colors.lime,
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
      name: "Tools",
      tree: [
        {
          name: "API",
          url: () => "/api",
          title: "API documentation",
        },
        {
          name: "MCP",
          url: () =>
            "https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_mcp/README.md#brk_mcp",
          title: "Model Context Protocol documentation",
        },
        {
          name: "Crate",
          url: () => "/crate",
          title: "View on crates.io",
        },
        {
          name: "Source",
          url: () => "/github",
          title: "Source code and issues",
        },
        {
          name: "Changelog",
          url: () => "/changelog",
          title: "Release notes and changelog",
        },
      ],
    },
    {
      name: "Hosting",
      tree: [
        {
          name: "Status",
          url: () => "/status",
          title: "Service status and uptime",
        },
        {
          name: "Self-host",
          url: () => "/install",
          title: "Install and run yourself",
        },
        {
          name: "Service",
          url: () => "/service",
          title: "Hosted service offering",
        },
      ],
    },
    {
      name: "Community",
      tree: [
        {
          name: "Discord",
          url: () => "/discord",
          title: "Join the Discord server",
        },
        {
          name: "GitHub",
          url: () => "/github",
          title: "Source code and issues",
        },
        {
          name: "Nostr",
          url: () => "/nostr",
          title: "Follow on Nostr",
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
