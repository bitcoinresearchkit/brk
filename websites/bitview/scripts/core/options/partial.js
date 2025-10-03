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
 * @typedef {{ metric: Metric, unit?: Unit }} FetchedAnySeriesOptions
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

import { localhost } from "../env";

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {BRK} args.brk
 * @returns {PartialOptionsTree}
 */
export function createPartialOptions({ colors, brk }) {
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

  const averages = /** @type {const} */ ([
    ["1w", 7, "red"],
    ["8d", 8, "orange"],
    ["13d", 13, "amber"],
    ["21d", 21, "yellow"],
    ["1m", 30, "lime"],
    ["34d", 34, "green"],
    ["55d", 55, "emerald"],
    ["89d", 89, "teal"],
    ["144d", 144, "cyan"],
    ["200d", 200, "sky"],
    ["1y", 365, "blue"],
    ["2y", 730, "indigo"],
    ["200w", 1400, "violet"],
    ["4y", 1460, "purple"],
  ]).map(
    ([id, days, colorKey]) =>
      /** @type {const} */ ({
        id,
        name: periodIdToName(id, true),
        days,
        color: colors[colorKey],
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

  const cohortAll = /** @type {const} */ ({
    id: "",
    name: "",
    title: "",
    color: colors.orange,
  });
  const cohortAllForComparaison = /** @type {const} */ ({
    id: "",
    name: "all",
    title: "",
    color: colors.default,
  });

  const terms = /** @type {const} */ ([
    ["sth", "short", "yellow"],
    ["lth", "long", "fuchsia"],
  ]).map(
    ([id, name, colorKey]) =>
      /** @type {const} */ ({
        id,
        name,
        title: `${name} term holders`,
        color: colors[colorKey],
      }),
  );

  const upToDate = /** @type {const} */ ([
    ["1d", "pink"],
    ["1w", "red"],
    ["1m", "orange"],
    ["2m", "amber"],
    ["3m", "yellow"],
    ["4m", "lime"],
    ["5m", "green"],
    ["6m", "teal"],
    ["1y", "sky"],
    ["2y", "indigo"],
    ["3y", "violet"],
    ["4y", "purple"],
    ["5y", "fuchsia"],
    ["6y", "pink"],
    ["7y", "red"],
    ["8y", "orange"],
    ["10y", "amber"],
    ["12y", "yellow"],
    ["15y", "lime"],
  ]).map(
    ([name, colorKey]) =>
      /** @type {const} */ ({
        id: `utxos_up_to_${name}_old`,
        name,
        title: `utxos up to ${periodIdToName(name, false)} old`,
        color: colors[colorKey],
      }),
  );

  const fromDate = /** @type {const} */ ([
    ["1d", "red"],
    ["1w", "orange"],
    ["1m", "yellow"],
    ["2m", "lime"],
    ["3m", "green"],
    ["4m", "teal"],
    ["5m", "cyan"],
    ["6m", "blue"],
    ["1y", "indigo"],
    ["2y", "violet"],
    ["3y", "purple"],
    ["4y", "fuchsia"],
    ["5y", "pink"],
    ["6y", "rose"],
    ["7y", "red"],
    ["8y", "orange"],
    ["10y", "yellow"],
    ["12y", "lime"],
    ["15y", "green"],
  ]).map(
    ([name, colorKey]) =>
      /** @type {const} */ ({
        id: `utxos_at_least_${name}_old`,
        name,
        title: `UTXOs at least ${periodIdToName(name, false)} old`,
        color: colors[colorKey],
      }),
  );

  const dateRange = /** @type {const} */ ([
    {
      id: "utxos_up_to_1d_old",
      name: "1d",
      title: "UTXOs New Up to 1 Day old",
      color: colors.pink,
    },
    .../** @type {const} */ ([
      [["1d", "1w"], "red"],
      [["1w", "1m"], "orange"],
      [["1m", "2m"], "yellow"],
      [["2m", "3m"], "yellow"],
      [["3m", "4m"], "lime"],
      [["4m", "5m"], "lime"],
      [["5m", "6m"], "lime"],
      [["6m", "1y"], "green"],
      [["1y", "2y"], "cyan"],
      [["2y", "3y"], "blue"],
      [["3y", "4y"], "indigo"],
      [["4y", "5y"], "violet"],
      [["5y", "6y"], "purple"],
      [["6y", "7y"], "purple"],
      [["7y", "8y"], "fuchsia"],
      [["8y", "10y"], "fuchsia"],
      [["10y", "12y"], "pink"],
      [["12y", "15y"], "red"],
    ]).map(
      ([[start, end], colorKey]) =>
        /** @type {const} */ ({
          id: `utxos_at_least_${start}_up_to_${end}_old`,
          name: `${start}..${end}`,
          title: `utxos at least ${periodIdToName(start, false)} ago up to ${periodIdToName(end, false)} old`,
          color: colors[colorKey],
        }),
    ),
    {
      id: "utxos_at_least_15y_old",
      name: "15y+",
      title: "UTXOs At least 15 Years old up to genesis",
      color: colors.orange,
    },
  ]);

  const epoch = /** @type {const} */ ([
    ["0", "red"],
    ["1", "yellow"],
    ["2", "orange"],
    ["3", "lime"],
    ["4", "green"],
  ]).map(
    ([name, colorKey]) =>
      /** @type {const} */ ({
        id: `epoch_${name}`,
        name,
        title: `Epoch ${name}`,
        color: colors[colorKey],
      }),
  );

  /**
   * @param {string} amount
   */
  function formatAmount(amount) {
    return amount
      .replace("sats", " sats")
      .replace("btc", " btc")
      .replace("_", "");
  }

  const aboveAmount = /** @type {const} */ ([
    ["1sat", "orange"],
    ["10sats", "orange"],
    ["100sats", "yellow"],
    ["1k_sats", "lime"],
    ["10k_sats", "green"],
    ["100k_sats", "cyan"],
    ["1m_sats", "blue"],
    ["10m_sats", "indigo"],
    ["1btc", "purple"],
    ["10btc", "violet"],
    ["100btc", "fuchsia"],
    ["1k_btc", "pink"],
    ["10k_btc", "red"],
  ]).map(([amount, colorKey]) => {
    const text = formatAmount(amount);
    return /** @type {const} */ ({
      id: `above_${amount}`,
      name: `>=${text}`,
      title: `Above ${text}`,
      color: colors[colorKey],
    });
  });

  const utxosAboveAmount = aboveAmount.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `utxos_${id}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesAboveAmount = aboveAmount.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `addrs_${id}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

  const underAmount = /** @type {const} */ ([
    ["10sats", "orange"],
    ["100sats", "yellow"],
    ["1k_sats", "lime"],
    ["10k_sats", "green"],
    ["100k_sats", "cyan"],
    ["1m_sats", "blue"],
    ["10m_sats", "indigo"],
    ["1btc", "purple"],
    ["10btc", "violet"],
    ["100btc", "fuchsia"],
    ["1k_btc", "pink"],
    ["10k_btc", "red"],
    ["100k_btc", "orange"],
  ]).map(([amount, colorKey]) => {
    const text = formatAmount(amount);
    return /** @type {const} */ ({
      id: `under_${amount}`,
      name: `<${text}`,
      title: `under ${text}`,
      color: colors[colorKey],
    });
  });

  const utxosUnderAmount = underAmount.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `utxos_${id}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesUnderAmount = underAmount.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `addrs_${id}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

  const amountRanges = /** @type {const} */ ([
    {
      id: "with_0sats",
      name: "0 sats",
      title: "valued 0 sats",
      color: colors.red,
    },
    .../** @type {const} */ ([
      [["1sat", "10sats"], "orange"],
      [["10sats", "100sats"], "yellow"],
      [["100sats", "1k_sats"], "lime"],
      [["1k_sats", "10k_sats"], "green"],
      [["10k_sats", "100k_sats"], "cyan"],
      [["100k_sats", "1m_sats"], "blue"],
      [["1m_sats", "10m_sats"], "indigo"],
      [["10m_sats", "1btc"], "purple"],
      [["1btc", "10btc"], "violet"],
      [["10btc", "100btc"], "fuchsia"],
      [["100btc", "1k_btc"], "pink"],
      [["1k_btc", "10k_btc"], "red"],
      [["10k_btc", "100k_btc"], "orange"],
    ]).map(([[start, end], colorKey]) => {
      const startText = formatAmount(start);
      const endText = formatAmount(end);
      return /** @type {const} */ ({
        id: `above_${start}_under_${end}`,
        name: `${startText}..${endText}`,
        title: `Above ${startText} Under ${endText}`,
        color: colors[colorKey],
      });
    }),
    {
      id: "above_100k_btc",
      name: "100K+ btc",
      title: "Above 100K BTC",
      color: colors.yellow,
    },
  ]);

  const utxosAmountRanges = amountRanges.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `utxos_${id}`,
        name,
        title: `UTXOs ${title}`,
        color,
      }),
  );

  const addressesAmountRanges = amountRanges.map(
    ({ id, name, title, color }) =>
      /** @type {const} */ ({
        id: `addrs_${id}`,
        name,
        title: `Addresses ${title}`,
        color,
      }),
  );

  const type = /** @type {const} */ ([
    ["p2pk65", "Pay To Long Public id", "red"],
    ["p2pk33", "Pay To Short Public id", "orange"],
    ["p2pkh", "Pay To Public id Hash", "yellow"],
    ["p2ms_outputs", "Pay To Bare Multisig", "lime"],
    ["p2sh", "Pay To Script Hash", "green"],
    ["p2wpkh", "Pay To Witness Public id Hash", "teal"],
    ["p2wsh", "Pay To Witness Script Hash", "blue"],
    ["p2tr", "Pay To Taproot", "indigo"],
    ["p2a", "Pay To Anchor", "purple"],
    ["unknown_outputs", "Pay To Unknown", "violet"],
    ["empty_outputs", "Pay To Empty", "fuchsia"],
  ]).map(
    ([id, title, colorKey]) =>
      /** @type {const} */ ({
        id,
        name: id.split("_")[0],
        title,
        color: colors[colorKey],
      }),
  );

  const cointimePrices = /** @type {const} */ ([
    ["true_market_mean", "True market mean", "blue"],
    ["vaulted_price", "Vaulted", "lime"],
    ["active_price", "Active", "rose"],
    ["cointime_price", "cointime", "yellow"],
  ]).map(
    ([metric, name, colorKey]) =>
      /** @type {const} */ ({
        metric,
        name,
        title: metric.replace(/_/g, " "),
        color: colors[colorKey],
      }),
  );

  const cointimeCapitalizations = /** @type {const} */ ([
    ["vaulted", "lime"],
    ["active", "rose"],
    ["cointime", "yellow"],
    ["investor", "fuchsia"],
    ["thermo", "emerald"],
  ]).map(([id, colorKey]) => {
    return /** @type {const} */ ({
      metric: `${id}_cap`,
      name: id,
      title: `${id} Capitalization`,
      color: colors[colorKey],
    });
  });

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
      metric: `constant_${
        number >= 0 ? number : `minus_${Math.abs(number)}`
      }`.replace(".", "_"),
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
          metric: `constant_${number >= 0 ? number : `minus_${Math.abs(number)}`}`,
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
   * @param {Metric} args.metric
   * @param {string} args.name
   * @param {Color} [args.color]
   * @param {Unit} [args.unit]
   * @param {boolean} [args.defaultActive]
   * @param {LineSeriesPartialOptions} [args.options]
   */
  function createBaseSeries({
    metric,
    name,
    color,
    defaultActive,
    unit,
    options,
  }) {
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
   * @param {Metric} metric
   */
  function createAverageSeries(metric) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      metric: `${metric}_avg`,
      title: "Average",
    });
  }

  /**
   * @param {Object} args
   * @param {Metric} args.metric
   * @param {Color} [args.sumColor]
   * @param {Color} [args.cumulativeColor]
   * @param {string} [args.common]
   */
  function createSumCumulativeSeries({
    metric,
    common,
    sumColor,
    cumulativeColor,
  }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      createSumSeries({
        metric,
        title: common,
        color: sumColor,
      }),
      createCumulativeSeries({
        metric,
        title: common,
        color: cumulativeColor,
      }),
    ]);
  }

  /**
   * @param {Object} args
   * @param {Metric} args.metric
   * @param {string} [args.title]
   * @param {Color} [args.color]
   */
  function createSumSeries({ metric, title = "", color }) {
    const metric_sum = `${metric}_sum`;
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      metric: brk.hasMetric(metric_sum) ? metric_sum : metric,
      title: `Sum ${title}`,
      color: color ?? colors.red,
    });
  }

  /**
   * @param {Object} args
   * @param {Metric} args.metric
   * @param {string} [args.title]
   * @param {Color} [args.color]
   */
  function createCumulativeSeries({ metric, title = "", color }) {
    return /** @satisfies {AnyFetchedSeriesBlueprint} */ ({
      metric: `${metric}_cumulative`,
      title: `Cumulative ${title}`,
      color: color ?? colors.cyan,
      defaultActive: false,
    });
  }

  /**
   * @param {Metric} metric
   */
  function createMinMaxPercentilesSeries(metric) {
    return /** @satisfies {AnyFetchedSeriesBlueprint[]} */ ([
      {
        metric: `${metric}_max`,
        title: "Max",
        color: colors.pink,
        defaultActive: false,
      },
      {
        metric: `${metric}_min`,
        title: "Min",
        color: colors.green,
        defaultActive: false,
      },
      {
        metric: `${metric}_median`,
        title: "Median",
        color: colors.amber,
        defaultActive: false,
      },
      {
        metric: `${metric}_pct75`,
        title: "pct75",
        color: colors.red,
        defaultActive: false,
      },
      {
        metric: `${metric}_pct25`,
        title: "pct25",
        color: colors.yellow,
        defaultActive: false,
      },
      {
        metric: `${metric}_pct90`,
        title: "pct90",
        color: colors.rose,
        defaultActive: false,
      },
      {
        metric: `${metric}_pct10`,
        title: "pct10",
        color: colors.lime,
        defaultActive: false,
      },
    ]);
  }

  /**
   * @param {Metric} metric
   */
  function createSumCumulativeMinMaxPercentilesSeries(metric) {
    return [
      ...createSumCumulativeSeries({ metric }),
      ...createMinMaxPercentilesSeries(metric),
    ];
  }

  /**
   * @param {Metric} metric
   */
  function createAverageSumCumulativeMinMaxPercentilesSeries(metric) {
    return [
      createAverageSeries(metric),
      ...createSumCumulativeMinMaxPercentilesSeries(metric),
    ];
  }

  /**
   * @param {Object} args
   * @param {Metric} args.metric
   * @param {string} args.name
   */
  function createBaseAverageSumCumulativeMinMaxPercentilesSeries({
    metric,
    name,
  }) {
    return [
      createBaseSeries({
        metric,
        name,
      }),
      ...createAverageSumCumulativeMinMaxPercentilesSeries(metric),
    ];
  }

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
   * @param {Metric} args.metric
   * @param {Color} [args.color]
   */
  function createPriceWithRatioOptions({ name, title, legend, metric, color }) {
    return [
      {
        name: "price",
        title,
        top: [
          createBaseSeries({
            metric: metric,
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
            metric: metric,
            name: legend,
            color,
          }),
          ...(brk.hasMetric(`${metric}_ratio_p1sd_usd`)
            ? percentiles.map(({ name, color }) =>
                createBaseSeries({
                  metric: `${metric}_ratio_${name}_usd`,
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
            metric: `${metric}_ratio`,
            title: "Ratio",
            type: "Baseline",
            options: {
              baseValue: { price: 1 },
            },
          }),
          ...(brk.hasMetric(`${metric}_ratio_p1sd`)
            ? percentiles.map(({ name, color }) =>
                createBaseSeries({
                  metric: `${metric}_ratio_${name}`,
                  name,
                  color,
                  defaultActive: false,
                  options: {
                    lineStyle: 1,
                  },
                }),
              )
            : []),
          ...(brk.hasMetric(`${metric}_ratio_sma`)
            ? ratioAverages.map(({ name, metric: metricAddon, color }) =>
                createBaseSeries({
                  metric: `${metric}_ratio_${metricAddon}`,
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
            unit: "ratio",
          }),
        ],
      },
      ...(brk.hasMetric(`${metric}_ratio_zscore`)
        ? [
            {
              name: "ZScores",
              tree: [
                {
                  name: "compare",
                  title: `Compare ${title} ZScores`,
                  top: [
                    createBaseSeries({
                      metric: metric,
                      name: legend,
                      color,
                    }),
                    createBaseSeries({
                      metric: `${metric}_ratio_1y_0sd_usd`,
                      name: "1y 0sd",
                      color: colors.fuchsia,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: `${metric}_ratio_2y_0sd_usd`,
                      name: "2y 0sd",
                      color: colors.purple,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: `${metric}_ratio_4y_0sd_usd`,
                      name: "4y 0sd",
                      color: colors.violet,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: `${metric}_ratio_0sd_usd`,
                      name: "0sd",
                      color: colors.indigo,
                      defaultActive: false,
                    }),
                  ],
                  bottom: [
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: `${metric}_ratio_zscore`,
                      title: "All",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: `${metric}_ratio_4y_zscore`,
                      colors: [colors.lime, colors.rose],
                      title: "4y",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: `${metric}_ratio_2y_zscore`,
                      colors: [colors.avocado, colors.pink],
                      title: "2y",
                      type: "Baseline",
                    }),
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: `${metric}_ratio_1y_zscore`,
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
                ...[
                  {
                    nameAddon: "all",
                    titleAddon: "",
                    metricAddon: "",
                  },
                  {
                    nameAddon: "4y",
                    titleAddon: "4y",
                    metricAddon: "4y_",
                  },
                  {
                    nameAddon: "2y",
                    titleAddon: "2y",
                    metricAddon: "2y_",
                  },
                  {
                    nameAddon: "1y",
                    titleAddon: "1y",
                    metricAddon: "1y_",
                  },
                ].flatMap(({ nameAddon, titleAddon, metricAddon }) => ({
                  name: nameAddon,
                  title: `${title} ${titleAddon} ZScore`,
                  top: [
                    createBaseSeries({
                      metric,
                      name: legend,
                      color,
                    }),
                    ...[
                      { sd: "0sd", name: "0σ", color: colors.lime },
                      {
                        sd: `p0_5sd`,
                        name: "+0.5σ",
                        color: colors.yellow,
                      },
                      {
                        sd: `p1sd`,
                        name: "+1σ",
                        color: colors.amber,
                      },
                      {
                        sd: `p1_5sd`,
                        name: "+1.5σ",
                        color: colors.orange,
                      },
                      {
                        sd: `p2sd`,
                        name: "+2σ",
                        color: colors.red,
                      },
                      {
                        sd: `p2_5sd`,
                        name: "+2.5σ",
                        color: colors.rose,
                      },
                      {
                        sd: `p3sd`,
                        name: "+3σ",
                        color: colors.pink,
                      },
                      {
                        sd: `m0_5sd`,
                        name: "−0.5σ",
                        color: colors.teal,
                      },
                      {
                        sd: `m1sd`,
                        name: "−1σ",
                        color: colors.cyan,
                      },
                      {
                        sd: `m1_5sd`,
                        name: "−1.5σ",
                        color: colors.sky,
                      },
                      {
                        sd: `m2sd`,
                        name: "−2σ",
                        color: colors.blue,
                      },
                      {
                        sd: `m2_5sd`,
                        name: "−2.5σ",
                        color: colors.indigo,
                      },
                      {
                        sd: `m3sd`,
                        name: "−3σ",
                        color: colors.violet,
                      },
                    ].map(({ sd, name, color }) =>
                      createBaseSeries({
                        metric: `${metric}_ratio_${metricAddon}${sd}_usd`,
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
                      metric: `${metric}_ratio_${metricAddon}zscore`,
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
          ]
        : []),
    ];
  }

  /**
   * @typedef {Object} UTXOGroupObject
   * @property {string} args.name
   * @property {string} args.title
   * @property {Color} args.color
   * @property {string} args.id
   */

  /**
   * @typedef {Object} UTXOGroupsObject
   * @property {string} args.name
   * @property {string} args.title
   * @property {readonly UTXOGroupObject[]} args.list
   */

  /**
   * @param {string} id
   */
  function fixId(id) {
    return id !== "" ? `${id}_` : "";
  }

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
              bottom: list.flatMap(({ color, name, id: _id }) => {
                const id = fixId(_id);
                return /** @type {const} */ ([
                  createBaseSeries({
                    metric: `${id}supply`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_btc`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_usd`,
                    name: "Supply",
                    color: colors.default,
                  }),
                  ...(id
                    ? [
                        createBaseSeries({
                          metric: `${id}supply_rel_to_circulating_supply`,
                          name: "Supply",
                          color: colors.default,
                        }),
                      ]
                    : []),
                  createBaseSeries({
                    metric: `${id}supply_in_profit`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_profit_btc`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_profit_usd`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_loss`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_loss_btc`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_loss_usd`,
                    name: "In Loss",
                    color: colors.red,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_half`,
                    name: "half",
                    color: colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    metric: `${id}supply_half_btc`,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  createBaseSeries({
                    metric: `${id}supply_half_usd`,
                    name: useGroupName ? name : "half",
                    color: "list" in args ? color : colors.gray,
                    options: {
                      lineStyle: 4,
                    },
                  }),
                  ...(id
                    ? [
                        createBaseSeries({
                          metric: `${id}supply_in_profit_rel_to_circulating_supply`,
                          name: "In Profit",
                          color: colors.green,
                        }),
                        createBaseSeries({
                          metric: `${id}supply_in_loss_rel_to_circulating_supply`,
                          name: "In Loss",
                          color: colors.red,
                        }),
                      ]
                    : []),
                  createBaseSeries({
                    metric: `${id}supply_in_profit_rel_to_own_supply`,
                    name: "In Profit",
                    color: colors.green,
                  }),
                  createBaseSeries({
                    metric: `${id}supply_in_loss_rel_to_own_supply`,
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
                  bottom: list.flatMap(({ color, name, id: _id }) => {
                    const id = fixId(_id);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        metric: `${id}supply`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_usd`,
                        name,
                        color,
                      }),
                      id
                        ? createBaseSeries({
                            metric: `${id}supply_rel_to_circulating_supply`,
                            name,
                            color,
                          })
                        : createBaseSeries({
                            unit: "%all",
                            metric: "constant_100",
                            name,
                            color,
                          }),
                    ]);
                  }),
                },
                {
                  name: "in profit",
                  title: `Supply In Profit ${title}`,
                  bottom: list.flatMap(({ color, name, id: _id }) => {
                    const id = fixId(_id);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        metric: `${id}supply_in_profit`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_in_profit_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_in_profit_usd`,
                        name,
                        color,
                      }),
                      ...(id
                        ? [
                            createBaseSeries({
                              metric: `${id}supply_in_profit_rel_to_circulating_supply`,
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
                  bottom: list.flatMap(({ color, name, id: _id }) => {
                    const id = fixId(_id);
                    return /** @type {const} */ ([
                      createBaseSeries({
                        metric: `${id}supply_in_loss`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_in_loss_btc`,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `${id}supply_in_loss_usd`,
                        name,
                        color,
                      }),
                      ...(id
                        ? [
                            createBaseSeries({
                              metric: `${id}supply_in_loss_rel_to_circulating_supply`,
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
          bottom: list.flatMap(({ color, name, id: _id }) => {
            const id = fixId(_id);
            return /** @type {const} */ ([
              createBaseSeries({
                metric: `${id}utxo_count`,
                name: useGroupName ? name : "Count",
                color,
              }),
            ]);
          }),
        },
        ...(list.filter(({ id }) => brk.hasMetric(`${fixId(id)}addr_count`))
          .length > ("list" in args ? 1 : 0)
          ? !("list" in args) ||
            list.filter(({ id }) =>
              brk.hasMetric(`${fixId(id)}empty_addr_count`),
            ).length <= 1
            ? [
                {
                  name: "address count",
                  title: `Address Count ${title}`,
                  bottom: list.flatMap(({ name, color, id: _id }) => {
                    const id = fixId(_id);
                    return [
                      ...(brk.hasMetric(`${id}addr_count`)
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              metric: `${id}addr_count`,
                              name: useGroupName ? name : "Loaded",
                              color: useGroupName ? color : colors.orange,
                            }),
                          ])
                        : []),
                      ...(brk.hasMetric(`${id}empty_addr_count`)
                        ? /** @type {const} */ ([
                            createBaseSeries({
                              metric: `${id}empty_addr_count`,
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
                        .filter(({ id }) =>
                          brk.hasMetric(`${fixId(id)}addr_count`),
                        )
                        .flatMap(({ name, color, id: _id }) => {
                          const id = fixId(_id);
                          return [
                            createBaseSeries({
                              metric: `${id}addr_count`,
                              name,
                              color,
                            }),
                          ];
                        }),
                    },
                    ...(list.filter(({ id }) =>
                      brk.hasMetric(`${fixId(id)}empty_addr_count`),
                    ).length
                      ? [
                          {
                            name: "empty",
                            title: `Empty Address Count ${title}`,
                            bottom: list
                              .filter(({ id }) =>
                                brk.hasMetric(`${fixId(id)}empty_addr_count`),
                              )
                              .flatMap(({ name, color, id: _id }) => {
                                const id = fixId(_id);
                                return [
                                  createBaseSeries({
                                    metric: `${id}empty_addr_count`,
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
                    top: list.map(({ color, name, id }) =>
                      createBaseSeries({
                        metric: `${fixId(id)}realized_price`,
                        name,
                        color,
                      }),
                    ),
                  },
                  {
                    name: "Ratio",
                    title: `Realized Price Ratio ${title}`,
                    bottom: [
                      ...list.map(({ color, name, id }) =>
                        createBaseSeries({
                          metric: `${fixId(id)}realized_price_ratio`,
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
                  metric: `${fixId(args.id)}realized_price`,
                  name: "price",
                  legend: "realized",
                  color: args.color,
                })),
            {
              name: "capitalization",
              title: `Realized Capitalization ${title}`,
              bottom: list.flatMap(({ color, name, id: _id }) => {
                const id = fixId(_id);
                return /** @type {const} */ ([
                  createBaseSeries({
                    metric: `${id}realized_cap`,
                    name: useGroupName ? name : "Capitalization",
                    color,
                  }),
                  ...(!("list" in args)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${id}realized_cap_30d_delta`,
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
                  brk.hasMetric(`${id}realized_cap_rel_to_own_market_cap`)
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${id}realized_cap_rel_to_own_market_cap`,
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
                      createBaseSeries({
                        metric: `${fixId(args.id)}realized_profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}realized_loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      ...(brk.hasMetric(
                        `${fixId(args.id)}realized_profit_to_loss_ratio`,
                      )
                        ? [
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}realized_profit_to_loss_ratio`,
                              name: "proft / loss",
                              color: colors.yellow,
                            }),
                          ]
                        : []),
                      createBaseSeries({
                        metric: `${fixId(args.id)}total_realized_pnl`,
                        name: "Total",
                        color: colors.default,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}neg_realized_loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}realized_profit_cumulative`,
                        name: "Cumulative Profit",
                        color: colors.green,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}realized_loss_cumulative`,
                        name: "Cumulative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}neg_realized_loss_cumulative`,
                        name: "Cumulative Negative Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          args.id,
                        )}realized_profit_rel_to_realized_cap`,
                        title: "Profit",
                        color: colors.green,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          args.id,
                        )}realized_loss_rel_to_realized_cap`,
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
                    bottom: list.flatMap(({ color, name, id }) => [
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(id)}net_realized_pnl`,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(id)}net_realized_pnl_cumulative`,
                        title: "Cumulative",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          id,
                        )}net_realized_pnl_cumulative_30d_delta`,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          id,
                        )}net_realized_pnl_rel_to_realized_cap`,
                        title: "Raw",
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          id,
                        )}net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`,
                        title: "cumulative 30d change",
                        defaultActive: false,
                      }),
                      /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                        type: "Baseline",
                        metric: `${fixId(
                          id,
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
                        unit: "usd",
                      }),
                    ]),
                  },
                  {
                    name: "sopr",
                    title: `Spent Output Profit Ratio ${title}`,
                    bottom: list.flatMap(({ color, name, id }) => {
                      const soprKey = `${fixId(id)}sopr`;
                      const asoprKey = `${fixId(id)}adjusted_sopr`;
                      return [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: soprKey,
                          title: "normal",
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(brk.hasMetric(asoprKey)
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: asoprKey,
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
                          metric: `${soprKey}_7d_ema`,
                          title: "7d ema",
                          colors: [colors.lime, colors.rose],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(brk.hasMetric(asoprKey)
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: `${asoprKey}_7d_ema`,
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
                          metric: `${soprKey}_30d_ema`,
                          title: "30d ema",
                          colors: [colors.avocado, colors.pink],
                          defaultActive: false,
                          options: {
                            baseValue: {
                              price: 1,
                            },
                          },
                        }),
                        ...(brk.hasMetric(asoprKey)
                          ? [
                              /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                                type: "Baseline",
                                metric: `${asoprKey}_30d_ema`,
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
                      ...list.flatMap(({ color, name, id: _id }) => {
                        const id = fixId(_id);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            metric: `${id}realized_profit`,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            metric: `${id}realized_profit_rel_to_realized_cap`,
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
                      ...list.flatMap(({ color, name, id: _id }) => {
                        const id = fixId(_id);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            metric: `${id}realized_loss`,
                            name,
                            color,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            type: "Baseline",
                            metric: `${id}realized_loss_rel_to_realized_cap`,
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
                      ...list.flatMap(({ color, name, id: _id }) => {
                        const id = fixId(_id);
                        return /** @type {const} */ ([
                          createBaseSeries({
                            metric: `${id}total_realized_pnl`,
                            name,
                            color,
                          }),
                          ...(brk.hasMetric(
                            `${id}realized_profit_to_loss_ratio`,
                          )
                            ? [
                                createBaseSeries({
                                  metric: `${id}realized_profit_to_loss_ratio`,
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
                      ...list.flatMap(({ color, name, id }) => [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${fixId(id)}net_realized_pnl`,
                          title: name,
                          color,
                        }),
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${fixId(
                            id,
                          )}net_realized_pnl_rel_to_realized_cap`,
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
                        bottom: list.flatMap(({ color, name, id: _id }) => {
                          const id = fixId(_id);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              metric: `${id}realized_profit_cumulative`,
                              name,
                              color,
                            }),
                          ]);
                        }),
                      },
                      {
                        name: "loss",
                        title: `Cumulative Realized Loss ${title}`,
                        bottom: list.flatMap(({ color, name, id: _id }) => {
                          const id = fixId(_id);
                          return /** @type {const} */ ([
                            createBaseSeries({
                              metric: `${id}realized_loss_cumulative`,
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
                          ...list.flatMap(({ color, name, id }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: `${fixId(id)}net_realized_pnl_cumulative`,
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
                          ...list.flatMap(({ color, name, id }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: `${fixId(
                                id,
                              )}net_realized_pnl_cumulative_30d_delta`,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: `${fixId(
                                id,
                              )}net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`,
                              title: name,
                              color,
                            }),
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: `${fixId(
                                id,
                              )}net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`,
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
                          ...list.flatMap(({ color, name, id }) => [
                            /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                              type: "Baseline",
                              metric: `${fixId(id)}sopr`,
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
                          .map(({ color, name, id }) => ({
                            color,
                            name,
                            metric: `${fixId(id)}adjusted_sopr`,
                          }))
                          .filter(({ metric }) => brk.hasMetric(metric));

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
                ? list.flatMap(({ id }) => [
                    createBaseSeries({
                      metric: `${fixId(id)}sell_side_risk_ratio`,
                      name: "raw",
                      color: colors.orange,
                    }),
                    createBaseSeries({
                      metric: `${fixId(id)}sell_side_risk_ratio_7d_ema`,
                      name: "7d ema",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: `${fixId(id)}sell_side_risk_ratio_30d_ema`,
                      name: "30d ema",
                      color: colors.rose,
                      defaultActive: false,
                    }),
                  ])
                : list.flatMap(({ color, name, id }) => [
                    createBaseSeries({
                      metric: `${fixId(id)}sell_side_risk_ratio`,
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
                        bottom: list.flatMap(({ color, name, id }) => {
                          const normalKey = `${fixId(id)}value_created`;
                          const adjKey = `${fixId(id)}adjusted_value_created`;
                          return [
                            createBaseSeries({
                              metric: normalKey,
                              name: "normal",
                              color: colors.emerald,
                            }),
                            ...(brk.hasMetric(adjKey)
                              ? [
                                  createBaseSeries({
                                    metric: adjKey,
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
                        bottom: list.flatMap(({ color, name, id }) => {
                          const normalKey = `${fixId(id)}value_destroyed`;
                          const adjKey = `${fixId(id)}adjusted_value_destroyed`;
                          return [
                            createBaseSeries({
                              metric: normalKey,
                              name: "normal",
                              color: colors.red,
                            }),
                            ...(brk.hasMetric(adjKey)
                              ? [
                                  createBaseSeries({
                                    metric: adjKey,
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
                            bottom: list.flatMap(({ color, name, id }) => [
                              createBaseSeries({
                                metric: `${fixId(id)}value_created`,
                                name,
                                color,
                              }),
                            ]),
                          },
                          ...(() => {
                            const reducedList = list
                              .map(({ color, name, id }) => ({
                                color,
                                name,
                                metric: `${fixId(id)}adjusted_value_created`,
                              }))
                              .filter(({ metric }) => brk.hasMetric(metric));
                            return reducedList.length
                              ? [
                                  {
                                    name: "Adjusted",
                                    title: `Adjusted value created ${title}`,
                                    bottom: reducedList.map(
                                      ({ color, name, metric }) =>
                                        createBaseSeries({
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
                            bottom: list.flatMap(({ color, name, id }) => [
                              createBaseSeries({
                                metric: `${fixId(id)}value_destroyed`,
                                name,
                                color,
                              }),
                            ]),
                          },
                          ...(() => {
                            const reducedList = list
                              .map(({ color, name, id }) => ({
                                color,
                                name,
                                metric: `${fixId(id)}adjusted_value_destroyed`,
                              }))
                              .filter(({ metric }) => brk.hasMetric(metric));
                            return reducedList.length
                              ? [
                                  {
                                    name: "Adjusted",
                                    title: `Adjusted value destroyed ${title}`,
                                    bottom: reducedList.map(
                                      ({ color, name, metric }) =>
                                        createBaseSeries({
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
                      createBaseSeries({
                        metric: `${fixId(args.id)}total_unrealized_pnl`,
                        name: "total",
                        color: colors.default,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}unrealized_profit`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}unrealized_loss`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        metric: `${fixId(args.id)}neg_unrealized_loss`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      createBaseSeries({
                        metric: `${fixId(
                          args.id,
                        )}unrealized_profit_rel_to_market_cap`,
                        name: "Profit",
                        color: colors.green,
                      }),
                      createBaseSeries({
                        metric: `${fixId(
                          args.id,
                        )}unrealized_loss_rel_to_market_cap`,
                        name: "Loss",
                        color: colors.red,
                        defaultActive: false,
                      }),
                      createBaseSeries({
                        metric: `${fixId(
                          args.id,
                        )}neg_unrealized_loss_rel_to_market_cap`,
                        name: "Negative Loss",
                        color: colors.red,
                      }),
                      ...(brk.hasMetric(
                        `${fixId(
                          args.id,
                        )}unrealized_profit_rel_to_own_market_cap`,
                      )
                        ? [
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}unrealized_profit_rel_to_own_market_cap`,
                              name: "Profit",
                              color: colors.green,
                            }),
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}unrealized_loss_rel_to_own_market_cap`,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}neg_unrealized_loss_rel_to_own_market_cap`,
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
                      ...(brk.hasMetric(
                        `${fixId(
                          args.id,
                        )}unrealized_profit_rel_to_own_total_unrealized_pnl`,
                      )
                        ? [
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}unrealized_profit_rel_to_own_total_unrealized_pnl`,
                              name: "Profit",
                              color: colors.green,
                            }),
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}unrealized_loss_rel_to_own_total_unrealized_pnl`,
                              name: "Loss",
                              color: colors.red,
                              defaultActive: false,
                            }),
                            createBaseSeries({
                              metric: `${fixId(
                                args.id,
                              )}neg_unrealized_loss_rel_to_own_total_unrealized_pnl`,
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
                    bottom: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}unrealized_profit`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "loss",
                    title: `Unrealized Loss ${title}`,
                    bottom: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}unrealized_loss`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "total pnl",
                    title: `Unrealized Total Profit And Loss ${title}`,
                    bottom: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}total_unrealized_pnl`,
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
                ...list.flatMap(({ color, name, id }) => [
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    metric: `${fixId(id)}net_unrealized_pnl`,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                    type: "Baseline",
                    metric: `${fixId(id)}net_unrealized_pnl_rel_to_market_cap`,
                    title: useGroupName ? name : "Net",
                    color: useGroupName ? color : undefined,
                  }),
                  ...(brk.hasMetric(
                    `${fixId(id)}net_unrealized_pnl_rel_to_own_market_cap`,
                  )
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${fixId(
                            id,
                          )}net_unrealized_pnl_rel_to_own_market_cap`,
                          title: useGroupName ? name : "Net",
                          color: useGroupName ? color : undefined,
                        }),
                        createPriceLine({
                          unit: "%cmcap",
                        }),
                      ]
                    : []),
                  ...(brk.hasMetric(
                    `${fixId(
                      id,
                    )}net_unrealized_pnl_rel_to_own_total_unrealized_pnl`,
                  )
                    ? [
                        /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                          type: "Baseline",
                          metric: `${fixId(
                            id,
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
                    top: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}realized_price`,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Min",
                    title: `Min Cost Basis ${title}`,
                    top: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}min_price_paid`,
                          name,
                          color: color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Max",
                    title: `Max Cost Basis ${title}`,
                    top: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}max_price_paid`,
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
                    metric: `${fixId(args.id)}realized_price`,
                    name: "Average",
                    color: args.color,
                  }),
                  createBaseSeries({
                    metric: `${fixId(args.id)}min_price_paid`,
                    name: "Min",
                    color: colors.green,
                    defaultActive: false,
                  }),
                  createBaseSeries({
                    metric: `${fixId(args.id)}max_price_paid`,
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
                    bottom: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}coinblocks_destroyed`,
                          name,
                          color,
                        }),
                        createBaseSeries({
                          metric: `${id}coindays_destroyed`,
                          name,
                          color,
                        }),
                      ]);
                    }),
                  },
                  {
                    name: "Cumulative",
                    title: `Cumulative Coins Destroyed ${title}`,
                    bottom: list.flatMap(({ color, name, id: _id }) => {
                      const id = fixId(_id);
                      return /** @type {const} */ ([
                        createBaseSeries({
                          metric: `${id}coinblocks_destroyed_cumulative`,
                          name,
                          color,
                        }),
                        createBaseSeries({
                          metric: `${id}coindays_destroyed_cumulative`,
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
                bottom: list.flatMap(({ color, name, id: _id }) => {
                  const id = fixId(_id);
                  return /** @type {const} */ ([
                    createBaseSeries({
                      metric: `${id}coinblocks_destroyed`,
                      name: "sum",
                      color,
                    }),
                    createBaseSeries({
                      metric: `${id}coinblocks_destroyed_cumulative`,
                      name: "cumulative",
                      color,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: `${id}coindays_destroyed`,
                      name: "sum",
                      color,
                    }),
                    createBaseSeries({
                      metric: `${id}coindays_destroyed_cumulative`,
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
                createBaseSeries({
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
                createBaseSeries({
                  metric: "price_ath",
                  name: "ath",
                }),
              ],
              bottom: [
                createBaseSeries({
                  metric: "price_drawdown",
                  name: "Drawdown",
                  color: colors.red,
                }),
                createBaseSeries({
                  metric: "days_since_price_ath",
                  name: "since",
                }),
                createBaseSeries({
                  metric: "max_days_between_price_aths",
                  name: "Max",
                  color: colors.red,
                }),
                createBaseSeries({
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
                    top: averages.map(({ days, id, name, color }) =>
                      createBaseSeries({
                        metric: `price_${id}_${metricAddon}`,
                        name: id,
                        color,
                      }),
                    ),
                  },
                  ...averages.map(({ id, name, color }) => ({
                    name,
                    tree: createPriceWithRatioOptions({
                      metric: `price_${id}_${metricAddon}`,
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
                "1d",
                "1w",
                "1m",
                "3m",
                "6m",
                "1y",
                "2y",
                "3y",
                "4y",
                "5y",
                "6y",
                "8y",
                "10y",
              ]).map((id) => {
                const name = periodIdToName(id, true);
                const cagr = `${id}_cagr`;
                return {
                  name,
                  title: `${name} Performance`,
                  bottom: [
                    /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                      metric: `${id}_price_returns`,
                      title: "total",
                      type: "Baseline",
                    }),
                    ...(brk.hasMetric(cagr)
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
                    createBaseSeries({
                      metric: "price_1w_volatility",
                      name: "1w",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      metric: "price_1m_volatility",
                      name: "1m",
                      color: colors.orange,
                    }),
                    createBaseSeries({
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
                      createBaseSeries({
                        metric: `price_${metric}_min`,
                        name: "min",
                        color: colors.red,
                      }),
                      createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: `price_200d_sma`,
                      name: "200d sma",
                      color: colors.yellow,
                    }),
                    createBaseSeries({
                      metric: `price_200d_sma_x2_4`,
                      name: "200d sma x2.4",
                      color: colors.green,
                    }),
                    createBaseSeries({
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
                      "1w",
                      "1m",
                      "3m",
                      "6m",
                      "1y",
                    ]).map((id) => {
                      const name = periodIdToName(id, true);

                      return /** @satisfies {PartialChartOption} */ ({
                        name,
                        title: `${name} DCA vs Lump Sum Returns`,
                        top: [
                          createBaseSeries({
                            metric: `${id}_dca_avg_price`,
                            name: `dca`,
                            color: colors.orange,
                          }),
                          createBaseSeries({
                            metric: `price_${id}_ago`,
                            name: `lump sum`,
                            color: colors.cyan,
                          }),
                        ],
                        bottom: [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_dca_returns`,
                            title: "dca",
                            type: "Baseline",
                            colors: [colors.yellow, colors.pink],
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_price_returns`,
                            title: "lump sum",
                            type: "Baseline",
                          }),
                          createPriceLine({
                            unit: "percentage",
                          }),
                        ],
                      });
                    }),
                    .../** @type {const} */ ([
                      "2y",
                      "3y",
                      "4y",
                      "5y",
                      "6y",
                      "8y",
                      "10y",
                    ]).map((id) => {
                      const name = periodIdToName(id, true);
                      return /** @satisfies {PartialChartOption} */ ({
                        name,
                        title: `${name} DCA vs Lump Sum Returns`,
                        top: [
                          createBaseSeries({
                            metric: `${id}_dca_avg_price`,
                            name: `dca`,
                            color: colors.orange,
                          }),
                          createBaseSeries({
                            metric: `price_${id}_ago`,
                            name: `lump sum`,
                            color: colors.cyan,
                          }),
                        ],
                        bottom: [
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_dca_returns`,
                            title: "dca",
                            type: "Baseline",
                            colors: [colors.yellow, colors.pink],
                          }),

                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_price_returns`,
                            title: "lump sum",
                            type: "Baseline",
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_dca_cagr`,
                            title: "dca cagr",
                            type: "Baseline",
                            colors: [colors.yellow, colors.pink],
                            defaultActive: false,
                          }),
                          /** @satisfies {FetchedBaselineSeriesBlueprint} */ ({
                            metric: `${id}_cagr`,
                            title: "lump sum cagr",
                            type: "Baseline",
                            defaultActive: false,
                          }),
                          createPriceLine({
                            unit: "percentage",
                          }),
                        ],
                      });
                    }),
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
                            createBaseSeries({
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
                    createBaseSeries({
                      metric: "block_count_target",
                      name: "Target",
                      color: colors.gray,
                      options: {
                        lineStyle: 4,
                      },
                    }),
                    createBaseSeries({
                      metric: "1w_block_count",
                      name: "1w sum",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: "1m_block_count",
                      name: "1m sum",
                      color: colors.pink,
                      defaultActive: false,
                    }),
                    createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: "total_size",
                      name: "raw",
                    }),
                    createBaseSeries({
                      metric: "vbytes",
                      name: "raw",
                    }),
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: "sent",
                      name: "Sent",
                    }),
                    createBaseSeries({
                      metric: "sent_btc",
                      name: "Sent",
                    }),
                    createBaseSeries({
                      metric: "sent_usd",
                      name: "Sent",
                    }),
                    createBaseSeries({
                      metric: "annualized_volume",
                      name: "annualized",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
                      metric: "annualized_volume_btc",
                      name: "annualized",
                      color: colors.red,
                      defaultActive: false,
                    }),
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: "tx_btc_velocity",
                      name: "bitcoin",
                    }),
                    createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: "supply",
                      name: "Mined",
                    }),
                    createBaseSeries({
                      metric: "supply_btc",
                      name: "Mined",
                    }),
                    createBaseSeries({
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
                        createBaseSeries({
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
                        createBaseSeries({
                          metric: "fee_dominance",
                          name: "Fee",
                          color: colors.amber,
                        }),
                        createBaseSeries({
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
                        createBaseSeries({
                          metric: "inflation_rate",
                          name: "Inflation",
                        }),
                      ],
                    },
                    {
                      name: "Puell multiple",
                      title: "Puell multiple",
                      bottom: [
                        createBaseSeries({
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
                    createBaseSeries({
                      metric: "halvingepoch",
                      name: "Halving",
                    }),
                    createBaseSeries({
                      metric: "blocks_before_next_halving",
                      name: "Before next",
                    }),
                    createBaseSeries({
                      metric: "days_before_next_halving",
                      name: "Before next",
                    }),
                  ],
                },
                {
                  name: "Difficulty",
                  title: "Difficulty",
                  bottom: [
                    createBaseSeries({
                      metric: "difficulty",
                      name: "Value",
                    }),
                    createBaseSeries({
                      metric: "difficultyepoch",
                      name: "Difficulty",
                    }),
                    createBaseSeries({
                      metric: "blocks_before_next_difficulty_adjustment",
                      name: "Before next",
                    }),
                    createBaseSeries({
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
                        createBaseSeries({
                          metric: "hash_rate",
                          name: "Raw",
                        }),
                        createBaseSeries({
                          metric: "hash_rate_1w_sma",
                          name: "1w sma",
                          color: colors.red,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          metric: "hash_rate_1m_sma",
                          name: "1m sma",
                          color: colors.pink,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          metric: "hash_rate_2m_sma",
                          name: "2m sma",
                          color: colors.purple,
                          defaultActive: false,
                        }),
                        createBaseSeries({
                          metric: "hash_rate_1y_sma",
                          name: "1y sma",
                          color: colors.indigo,
                          defaultActive: false,
                        }),
                        createBaseSeries({
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
                        createBaseSeries({
                          metric: "hash_price_ths",
                          name: "Dollars",
                          color: colors.emerald,
                        }),
                        createBaseSeries({
                          metric: "hash_price_phs",
                          name: "Dollars",
                          color: colors.emerald,
                        }),
                        createBaseSeries({
                          metric: "hash_price_rebound",
                          name: "Rebound",
                          color: colors.yellow,
                        }),
                        createBaseSeries({
                          metric: "hash_price_ths_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                        createBaseSeries({
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
                        createBaseSeries({
                          metric: "hash_value_ths",
                          name: "Sats",
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          metric: "hash_value_phs",
                          name: "Sats",
                          color: colors.orange,
                        }),
                        createBaseSeries({
                          metric: "hash_value_rebound",
                          name: "Rebound",
                          color: colors.yellow,
                        }),
                        createBaseSeries({
                          metric: "hash_value_ths_min",
                          name: "Min",
                          color: colors.red,
                          options: {
                            lineStyle: 1,
                          },
                        }),
                        createBaseSeries({
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
                  tree: Object.entries(brk.POOL_ID_TO_POOL_NAME).map(
                    ([_id, name]) => {
                      const id = /** @type {PoolId} */ (_id);
                      return {
                        name,
                        tree: [
                          {
                            name: "Dominance",
                            title: `Mining Dominance of ${name}`,
                            bottom: [
                              createBaseSeries({
                                metric: `${id}_1d_dominance`,
                                name: "1d",
                                color: colors.rose,
                                defaultActive: false,
                              }),
                              createBaseSeries({
                                metric: `${id}_1w_dominance`,
                                name: "1w",
                                color: colors.red,
                                defaultActive: false,
                              }),
                              createBaseSeries({
                                metric: `${id}_1m_dominance`,
                                name: "1m",
                              }),
                              createBaseSeries({
                                metric: `${id}_1y_dominance`,
                                name: "1y",
                                color: colors.lime,
                                defaultActive: false,
                              }),
                              createBaseSeries({
                                metric: `${id}_dominance`,
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
                                metric: `${id}_blocks_mined`,
                                name: "Sum",
                              }),
                              createBaseSeries({
                                metric: `${id}_blocks_mined_cumulative`,
                                name: "Cumulative",
                                color: colors.blue,
                              }),
                              createBaseSeries({
                                metric: `${id}_1w_blocks_mined`,
                                name: "1w Sum",
                                color: colors.red,
                                defaultActive: false,
                              }),
                              createBaseSeries({
                                metric: `${id}_1m_blocks_mined`,
                                name: "1m Sum",
                                color: colors.pink,
                                defaultActive: false,
                              }),
                              createBaseSeries({
                                metric: `${id}_1y_blocks_mined`,
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
                                metricAddon: "coinbase",
                                cumulativeColor: colors.red,
                                sumColor: colors.orange,
                              },
                              {
                                metricAddon: "subsidy",
                                cumulativeColor: colors.emerald,
                                sumColor: colors.lime,
                              },
                              {
                                metricAddon: "fee",
                                cumulativeColor: colors.indigo,
                                sumColor: colors.cyan,
                              },
                            ].flatMap(
                              ({ metricAddon, sumColor, cumulativeColor }) => [
                                ...createSumCumulativeSeries({
                                  metric: `${id}_${metricAddon}`,
                                  common: metricAddon,
                                  sumColor,
                                  cumulativeColor,
                                }),
                                ...createSumCumulativeSeries({
                                  metric: `${id}_${metricAddon}_btc`,
                                  common: metricAddon,
                                  sumColor,
                                  cumulativeColor,
                                }),
                                ...createSumCumulativeSeries({
                                  metric: `${id}_${metricAddon}_usd`,
                                  common: metricAddon,
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
                                metric: `${id}_days_since_block`,
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
                    createBaseSeries({
                      metric: "unspendable_supply",
                      name: "Supply",
                    }),
                    createBaseSeries({
                      metric: "unspendable_supply_btc",
                      name: "Supply",
                    }),
                    createBaseSeries({
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
                        createBaseSeries({
                          metric: "opreturn_count",
                          name: "Count",
                        }),
                        createBaseSeries({
                          metric: "opreturn_count",
                          name: "sum",
                        }),
                        createBaseSeries({
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
                        createBaseSeries({
                          metric: "opreturn_supply",
                          name: "Supply",
                        }),
                        createBaseSeries({
                          metric: "opreturn_supply_btc",
                          name: "Supply",
                        }),
                        createBaseSeries({
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
                  top: cointimePrices.map(({ metric, name, color }) =>
                    createBaseSeries({
                      metric,
                      name,
                      color,
                    }),
                  ),
                },
                ...cointimePrices.map(({ metric, name, color, title }) => ({
                  name,
                  tree: createPriceWithRatioOptions({
                    metric,
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
                      metric: `market_cap`,
                      name: "Market",
                      color: colors.default,
                    }),
                    createBaseSeries({
                      metric: `realized_cap`,
                      name: "Realized",
                      color: colors.orange,
                    }),
                    ...cointimeCapitalizations.map(({ metric, name, color }) =>
                      createBaseSeries({
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
                      createBaseSeries({
                        metric,
                        name,
                        color,
                      }),
                      createBaseSeries({
                        metric: `market_cap`,
                        name: "Market",
                        color: colors.default,
                      }),
                      createBaseSeries({
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
                      metric: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply`,
                      name,
                      color,
                    }),
                    createBaseSeries({
                      metric: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply_btc`,
                      name,
                      color,
                    }),
                    createBaseSeries({
                      metric: `${
                        name !== "all" ? /** @type {const} */ (`${name}_`) : ""
                      }supply_usd`,
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
                  metric: "liveliness",
                  name: "Liveliness",
                  color: colors.rose,
                }),
                createBaseSeries({
                  metric: "vaultedness",
                  name: "Vaultedness",
                  color: colors.lime,
                }),
                createBaseSeries({
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
                createBaseSeries({
                  metric: "coinblocks_destroyed",
                  name: "Destroyed",
                  color: colors.red,
                }),
                createBaseSeries({
                  metric: "coinblocks_destroyed_cumulative",
                  name: "Cumulative Destroyed",
                  color: colors.red,
                  defaultActive: false,
                }),
                createBaseSeries({
                  metric: "coinblocks_created",
                  name: "created",
                  color: colors.orange,
                }),
                createBaseSeries({
                  metric: "coinblocks_created_cumulative",
                  name: "Cumulative created",
                  color: colors.orange,
                  defaultActive: false,
                }),
                createBaseSeries({
                  metric: "coinblocks_stored",
                  name: "stored",
                  color: colors.green,
                }),
                createBaseSeries({
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
                    createBaseSeries({
                      metric: "inflation_rate",
                      name: "base",
                      color: colors.orange,
                    }),
                    createBaseSeries({
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
                    createBaseSeries({
                      metric: "tx_btc_velocity",
                      name: "btc",
                      color: colors.orange,
                    }),
                    createBaseSeries({
                      metric: "cointime_adj_tx_btc_velocity",
                      name: "adj. btc",
                      color: colors.red,
                    }),
                    createBaseSeries({
                      metric: "tx_usd_velocity",
                      name: "usd",
                      color: colors.emerald,
                    }),
                    createBaseSeries({
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
        {
          name: "Changelog",
          url: () =>
            "https://github.com/bitcoinresearchkit/brk/blob/main/docs/CHANGELOG.md#changelog",
          title: "BRK's changelog",
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
