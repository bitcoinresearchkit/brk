// @ts-check

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 */
export function init({
  colors,
  elements,
  lightweightCharts,
  signals,
  utils,
  vecsResources,
}) {
  /**
   * @typedef {Object} Frequency
   * @property {string} name
   * @property {string} value
   * @property {(date: Date) => boolean} isTriggerDay
   *
   * @typedef {Object} Frequencies
   * @property {string} name
   * @property {Frequency[]} list
   */

  const simulationElement = elements.simulation;

  const dom = {
    /**
     * @param {Object} args
     * @param {string} args.id
     * @param {string} args.title
     * @param {string} args.placeholder
     * @param {Signal<number | null>} args.signal
     * @param {number} args.min
     * @param {number} args.step
     * @param {number} [args.max]
     * @param {Signals} args.signals
     */
    createInputNumberElement({
      id,
      title,
      signal,
      min,
      max,
      step,
      placeholder,
      signals,
    }) {
      const input = window.document.createElement("input");
      if (!id || !title || !placeholder) throw Error("input attribute missing");
      input.id = id;
      input.title = title;
      input.placeholder = placeholder;
      input.type = "number";
      input.min = String(min);
      if (max) {
        input.max = String(max);
      }
      input.step = String(step);

      let stateValue = /** @type {string | null} */ (null);

      signals.createEffect(
        () => {
          const value = signal();
          return value ? String(value) : "";
        },
        (value) => {
          if (stateValue !== value) {
            input.value = value;
            stateValue = value;
          }
        },
      );

      input.addEventListener("input", () => {
        const valueSer = input.value;
        stateValue = valueSer;
        const value = Number(valueSer);
        if (value >= min && (max ? value <= max : true)) {
          signal.set(value);
        }
      });

      return { input, signal };
    },
    /**
     * @param {Object} args
     * @param {string} args.id
     * @param {string} args.title
     * @param {Signal<number | null>} args.signal
     * @param {Signals} args.signals
     */
    createInputDollar({ id, title, signal, signals }) {
      return this.createInputNumberElement({
        id,
        placeholder: "USD",
        min: 0,
        title,
        signal,
        signals,
        step: 1,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.id
     * @param {string} args.title
     * @param {Signal<Date | null>} args.signal
     * @param {Signals} args.signals
     */
    createInputDate({ id, title, signal, signals }) {
      const input = window.document.createElement("input");
      input.id = id;
      input.title = title;
      input.type = "date";
      const min = "2011-01-01";
      const minDate = new Date(min);
      const maxDate = new Date();
      const max = utils.date.toString(maxDate);
      input.min = min;
      input.max = max;

      let stateValue = /** @type {string | null} */ (null);

      signals.createEffect(
        () => {
          const dateSignal = signal();
          return dateSignal ? utils.date.toString(dateSignal) : "";
        },
        (value) => {
          if (stateValue !== value) {
            input.value = value;
            stateValue = value;
          }
        },
      );

      input.addEventListener("change", () => {
        const value = input.value;
        const date = new Date(value);
        if (date >= minDate && date <= maxDate) {
          stateValue = value;
          signal.set(value ? date : null);
        }
      });

      return { input, signal };
    },
    /**
     * @param {Object} param0
     * @param {Signal<any>} param0.signal
     * @param {HTMLInputElement} [param0.input]
     * @param {HTMLSelectElement} [param0.select]
     */
    createResetableInput({ input, select, signal }) {
      const div = window.document.createElement("div");

      const element = input || select;
      if (!element) throw "createResetableField element missing";
      div.append(element);

      const button = utils.dom.createButtonElement({
        onClick: signal.reset,
        inside: "Reset",
        title: "Reset field",
      });
      button.type = "reset";

      div.append(button);

      return div;
    },
  };

  const parametersElement = window.document.createElement("div");
  simulationElement.append(parametersElement);
  const resultsElement = window.document.createElement("div");
  simulationElement.append(resultsElement);

  function computeFrequencies() {
    const weekDays = [
      "Monday",
      "Tuesday",
      "Wednesday",
      "Thursday",
      "Friday",
      "Saturday",
      "Sunday",
    ];
    const maxDays = 28;

    /** @param {number} day  */
    function getOrdinalDay(day) {
      const rest = (day % 30) % 20;

      return `${day}${
        rest === 1 ? "st" : rest === 2 ? "nd" : rest === 3 ? "rd" : "th"
      }`;
    }

    /** @satisfies {([Frequency, Frequencies, Frequencies, Frequencies])} */
    const list = [
      {
        name: "Every day",
        value: "every-day",
        /** @param {Date} _  */
        isTriggerDay(_) {
          return true;
        },
      },
      {
        name: "Once a week",
        list: weekDays.map((day, index) => ({
          name: day,
          value: day.toLowerCase(),
          /** @param {Date} date  */
          isTriggerDay(date) {
            let day = date.getUTCDay() - 1;
            if (day === -1) {
              day = 6;
            }
            return day === index;
          },
        })),
      },
      {
        name: "Every two weeks",
        list: [...Array(Math.round(maxDays / 2)).keys()].map((day) => {
          const day1 = day + 1;
          const day2 = day + 15;

          return {
            value: `${day1}+${day2}`,
            name: `The ${getOrdinalDay(day1)} and the ${getOrdinalDay(day2)}`,
            /** @param {Date} date  */
            isTriggerDay(date) {
              const d = date.getUTCDate();
              return d === day1 || d === day2;
            },
          };
        }),
      },
      {
        name: "Once a month",
        list: [...Array(maxDays).keys()].map((day) => {
          day++;

          return {
            name: `The ${getOrdinalDay(day)}`,
            value: String(day),
            /** @param {Date} date  */
            isTriggerDay(date) {
              const d = date.getUTCDate();
              return d === day;
            },
          };
        }),
      },
    ];

    /** @type {Record<string, Frequency>} */
    const idToFrequency = {};

    list.forEach((anyFreq, index) => {
      if ("list" in anyFreq) {
        anyFreq.list?.forEach((freq) => {
          idToFrequency[freq.value] = freq;
        });
      } else {
        idToFrequency[anyFreq.value] = anyFreq;
      }
    });

    const serde = {
      /**
       * @param {Frequency} v
       */
      serialize(v) {
        return v.value;
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        const freq = idToFrequency[v];
        if (!freq) throw "Freq not found";
        return freq;
      },
    };

    return { list, serde };
  }

  const frequencies = computeFrequencies();

  const keyPrefix = "save-in-bitcoin";
  const settings = {
    dollars: {
      initial: {
        amount: signals.createSignal(/** @type {number | null} */ (1000), {
          save: {
            ...utils.serde.optNumber,
            keyPrefix,
            key: "initial-amount",
          },
        }),
      },
      topUp: {
        amount: signals.createSignal(/** @type {number | null} */ (150), {
          save: {
            ...utils.serde.optNumber,
            keyPrefix,
            key: "top-up-amount",
          },
        }),
        frenquency: signals.createSignal(
          /** @type {Frequency} */ (frequencies.list[3].list[0]),
          {
            save: {
              ...frequencies.serde,
              keyPrefix,
              key: "top-up-freq",
            },
          },
        ),
      },
    },
    bitcoin: {
      investment: {
        initial: signals.createSignal(/** @type {number | null} */ (1000), {
          save: {
            ...utils.serde.optNumber,
            keyPrefix,
            key: "initial-swap",
          },
        }),
        recurrent: signals.createSignal(/** @type {number | null} */ (5), {
          save: {
            ...utils.serde.optNumber,
            keyPrefix,
            key: "recurrent-swap",
          },
        }),
        frequency: signals.createSignal(
          /** @type {Frequency} */ (frequencies.list[0]),
          {
            save: {
              ...frequencies.serde,
              keyPrefix,
              key: "swap-freq",
            },
          },
        ),
      },
    },
    interval: {
      start: signals.createSignal(
        /** @type {Date | null} */ (new Date("2021-04-15")),
        {
          save: {
            ...utils.serde.optDate,
            keyPrefix,
            key: "interval-start",
          },
        },
      ),
      end: signals.createSignal(/** @type {Date | null} */ (new Date()), {
        save: {
          ...utils.serde.optDate,
          keyPrefix,
          key: "interval-end",
        },
      }),
    },
    fees: {
      percentage: signals.createSignal(/** @type {number | null} */ (0.25), {
        save: {
          ...utils.serde.optNumber,
          keyPrefix,
          key: "percentage",
        },
      }),
    },
  };

  parametersElement.append(
    utils.dom.createHeader("Save in Bitcoin").headerElement,
  );

  /**
   * @param {Object} param0
   * @param {ColorName} param0.color
   * @param {string} param0.type
   * @param {string} param0.text
   */
  function createColoredTypeHTML({ color, type, text }) {
    return `${createColoredSpan({ color, text: `${type}:` })} ${text}`;
  }

  /**
   * @param {Object} param0
   * @param {ColorName} param0.color
   * @param {string} param0.text
   */
  function createColoredSpan({ color, text }) {
    return `<span style="color: ${colors[
      color
    ]()}; font-weight: 500; text-transform: uppercase;
      font-size: var(--font-size-sm);">${text}</span>`;
  }

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Initial Amount",
      }),
      description:
        "The amount of dollars you have ready on the exchange on day one.",
      input: dom.createResetableInput(
        dom.createInputDollar({
          id: "simulation-dollars-initial",
          title: "Initial Dollar Amount",
          signal: settings.dollars.initial.amount,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Top Up Frequency",
      }),
      description:
        "The frequency at which you'll top up your account at the exchange.",
      input: dom.createResetableInput(
        utils.dom.createSelect({
          id: "top-up-frequency",
          list: frequencies.list,
          signal: settings.dollars.topUp.frenquency,
          deep: true,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Top Up Amount",
      }),
      description:
        "The recurrent amount of dollars you'll be transfering to said exchange.",
      input: dom.createResetableInput(
        dom.createInputDollar({
          id: "simulation-dollars-top-up-amount",
          title: "Top Up Dollar Amount",
          signal: settings.dollars.topUp.amount,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Initial Investment",
      }),
      description:
        "The amount, if available, of dollars that will be used to buy Bitcoin on day one.",
      input: dom.createResetableInput(
        dom.createInputDollar({
          id: "simulation-bitcoin-initial-investment",
          title: "Initial Swap Amount",
          signal: settings.bitcoin.investment.initial,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Investment Frequency",
      }),
      description: "The frequency at which you'll be buying Bitcoin.",
      input: dom.createResetableInput(
        utils.dom.createSelect({
          id: "investment-frequency",
          list: frequencies.list,
          signal: settings.bitcoin.investment.frequency,
          deep: true,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Recurrent Investment",
      }),
      description:
        "The recurrent amount, if available, of dollars that will be used to buy Bitcoin.",
      input: dom.createResetableInput(
        dom.createInputDollar({
          id: "simulation-bitcoin-recurrent-investment",
          title: "Bitcoin Recurrent Investment",
          signal: settings.bitcoin.investment.recurrent,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "sky",
        type: "Interval",
        text: "Start",
      }),
      description: "The first day of the simulation.",
      input: dom.createResetableInput(
        dom.createInputDate({
          id: "simulation-inverval-start",
          title: "First Simulation Date",
          signal: settings.interval.start,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "sky",
        type: "Interval",
        text: "End",
      }),
      description: "The last day of the simulation.",
      input: dom.createResetableInput(
        dom.createInputDate({
          id: "simulation-inverval-end",
          title: "Last Simulation Day",
          signal: settings.interval.end,
          signals,
        }),
      ),
    }),
  );

  parametersElement.append(
    utils.dom.createFieldElement({
      title: createColoredTypeHTML({
        color: "red",
        type: "Fees",
        text: "Exchange",
      }),
      description: "The amount of trading fees (in %) at the exchange.",
      input: dom.createResetableInput(
        dom.createInputNumberElement({
          id: "simulation-fees",
          title: "Exchange Fees",
          signal: settings.fees.percentage,
          min: 0,
          max: 50,
          step: 0.01,
          signals,
          placeholder: "Fees",
        }),
      ),
    }),
  );

  const p1 = window.document.createElement("p");
  resultsElement.append(p1);
  const p2 = window.document.createElement("p");
  resultsElement.append(p2);
  const p3 = window.document.createElement("p");
  resultsElement.append(p3);
  const p4 = window.document.createElement("p");
  resultsElement.append(p4);

  const owner = signals.getOwner();

  const totalInvestedAmountData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinValueData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const resultData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const dollarsLeftData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const totalValueData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const investmentData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const bitcoinAddedData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const averagePricePaidData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinPriceData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const buyCountData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const totalFeesPaidData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const daysCountData = signals.createSignal(/** @type {LineData[]} */ ([]), {
    equals: false,
  });
  const profitableDaysRatioData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );
  const unprofitableDaysRatioData = signals.createSignal(
    /** @type {LineData[]} */ ([]),
    {
      equals: false,
    },
  );

  const index = () => /** @type {DateIndex} */ (0);

  lightweightCharts.createChartElement({
    index,
    parent: resultsElement,
    signals,
    colors,
    id: `result`,
    fitContent: true,
    vecsResources,
    utils,
    elements,
    config: [
      {
        unit: "USD",
        blueprints: [
          {
            title: "Bitcoin Value",
            type: "Line",
            color: colors.amber,
            data: bitcoinValueData,
          },
          {
            title: "Dollars Converted",
            type: "Line",
            color: colors.green,
            data: totalInvestedAmountData,
          },
          {
            title: "Dollars Left",
            type: "Line",
            color: colors.emerald,
            data: dollarsLeftData,
            defaultActive: false,
          },
          {
            title: "Fees Paid",
            type: "Line",
            color: colors.rose,
            data: totalFeesPaidData,
            defaultActive: false,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    index,
    parent: resultsElement,
    signals,
    colors,
    id: `bitcoin`,
    fitContent: true,
    vecsResources,
    elements,
    utils,
    config: [
      {
        unit: "BTC",
        blueprints: [
          {
            title: "Bitcoin Stack",
            type: "Line",
            color: colors.orange,
            data: bitcoinData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    index,
    parent: resultsElement,
    signals,
    colors,
    id: `average-price`,
    fitContent: true,
    vecsResources,
    utils,
    elements,
    config: [
      {
        unit: "USD",
        blueprints: [
          {
            title: "Bitcoin Price",
            type: "Line",
            color: colors.default,
            data: bitcoinPriceData,
          },
          {
            title: "Average Price Paid",
            type: "Line",
            color: colors.lime,
            data: averagePricePaidData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    index,
    parent: resultsElement,
    signals,
    colors,
    vecsResources,
    id: `return-ratio`,
    fitContent: true,
    utils,
    elements,
    config: [
      {
        unit: "USD",
        blueprints: [
          {
            title: "Return Of Investment",
            type: "Baseline",
            data: resultData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    index,
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-profitability-ratios`,
    fitContent: true,
    vecsResources,
    utils,
    elements,
    config: [
      {
        unit: "percentage",
        blueprints: [
          {
            title: "Profitable Days Ratio",
            type: "Line",
            color: colors.green,
            data: profitableDaysRatioData,
          },
          {
            title: "Unprofitable Days Ratio",
            type: "Line",
            color: colors.red,
            data: unprofitableDaysRatioData,
          },
        ],
      },
    ],
  });

  vecsResources
    .getOrCreate(/** @satisfies {DateIndex} */ (0), "close")
    .fetch()
    .then((_closes) => {
      if (!_closes) return;
      const closes = /** @type {number[]} */ (_closes);

      signals.runWithOwner(owner, () => {
        signals.createEffect(
          () => ({
            initialDollarAmount: settings.dollars.initial.amount() || 0,
            topUpAmount: settings.dollars.topUp.amount() || 0,
            topUpFrequency: settings.dollars.topUp.frenquency(),
            initialSwap: settings.bitcoin.investment.initial() || 0,
            recurrentSwap: settings.bitcoin.investment.recurrent() || 0,
            swapFrequency: settings.bitcoin.investment.frequency(),
            start: settings.interval.start(),
            end: settings.interval.end(),
            fees: settings.fees.percentage(),
          }),
          ({
            initialDollarAmount,
            topUpAmount,
            topUpFrequency,
            initialSwap,
            recurrentSwap,
            swapFrequency,
            start,
            end,
            fees,
          }) => {
            if (!start || !end || start > end) return;

            const range = utils.date.getRange(start, end);

            totalInvestedAmountData().length = 0;
            bitcoinValueData().length = 0;
            bitcoinData().length = 0;
            resultData().length = 0;
            dollarsLeftData().length = 0;
            totalValueData().length = 0;
            investmentData().length = 0;
            bitcoinAddedData().length = 0;
            averagePricePaidData().length = 0;
            bitcoinPriceData().length = 0;
            buyCountData().length = 0;
            totalFeesPaidData().length = 0;
            daysCountData().length = 0;
            profitableDaysRatioData().length = 0;
            unprofitableDaysRatioData().length = 0;

            let bitcoin = 0;
            let sats = 0;
            let dollars = initialDollarAmount;
            let investedAmount = 0;
            let postFeesInvestedAmount = 0;
            let buyCount = 0;
            let averagePricePaid = 0;
            let bitcoinValue = 0;
            let roi = 0;
            let totalValue = 0;
            let totalFeesPaid = 0;
            let daysCount = range.length;
            let profitableDays = 0;
            let unprofitableDays = 0;
            let profitableDaysRatio = 0;
            let unprofitableDaysRatio = 0;
            let lastInvestDay = range[0];
            let dailyInvestment = 0;
            let bitcoinAdded = 0;
            let satsAdded = 0;
            let lastSatsAdded = 0;

            range.forEach((date, index) => {
              const time = date.valueOf() / 1000;

              if (topUpFrequency.isTriggerDay(date)) {
                dollars += topUpAmount;
              }

              const close = closes[utils.date.toDateIndex(date)];

              if (!close) return;

              dailyInvestment = 0;
              /** @param {number} value  */
              function invest(value) {
                value = Math.min(dollars, value);
                dailyInvestment += value;
                dollars -= value;
                buyCount += 1;
                lastInvestDay = date;
              }
              if (!index) {
                invest(initialSwap);
              }
              if (swapFrequency.isTriggerDay(date) && dollars > 0) {
                invest(recurrentSwap);
              }

              investedAmount += dailyInvestment;

              let dailyInvestmentPostFees =
                dailyInvestment * (1 - (fees || 0) / 100);

              totalFeesPaid += dailyInvestment - dailyInvestmentPostFees;

              bitcoinAdded = dailyInvestmentPostFees / close;
              bitcoin += bitcoinAdded;
              satsAdded = Math.floor(bitcoinAdded * 100_000_000);
              if (satsAdded > 0) {
                lastSatsAdded = satsAdded;
              }
              sats += satsAdded;

              postFeesInvestedAmount += dailyInvestmentPostFees;

              bitcoinValue = close * bitcoin;

              totalValue = dollars + bitcoinValue;

              averagePricePaid = postFeesInvestedAmount / bitcoin;

              roi = (bitcoinValue / postFeesInvestedAmount - 1) * 100;

              const daysCount = index + 1;
              profitableDaysRatio = profitableDays / daysCount;
              unprofitableDaysRatio = unprofitableDays / daysCount;

              if (roi >= 0) {
                profitableDays += 1;
              } else {
                unprofitableDays += 1;
              }

              bitcoinPriceData().push({
                time,
                value: close,
              });

              bitcoinData().push({
                time,
                value: bitcoin,
              });

              totalInvestedAmountData().push({
                time,
                value: investedAmount,
              });

              bitcoinValueData().push({
                time,
                value: bitcoinValue,
              });

              resultData().push({
                time,
                value: roi,
              });

              dollarsLeftData().push({
                time,
                value: dollars,
              });

              totalValueData().push({
                time,
                value: totalValue,
              });

              investmentData().push({
                time,
                value: dailyInvestment,
              });

              bitcoinAddedData().push({
                time,
                value: bitcoinAdded,
              });

              averagePricePaidData().push({
                time,
                value: averagePricePaid,
              });

              buyCountData().push({
                time,
                value: buyCount,
              });

              totalFeesPaidData().push({
                time,
                value: totalFeesPaid,
              });

              daysCountData().push({
                time,
                value: daysCount,
              });

              profitableDaysRatioData().push({
                time,
                value: profitableDaysRatio * 100,
              });

              unprofitableDaysRatioData().push({
                time,
                value: unprofitableDaysRatio * 100,
              });
            });

            const f = utils.locale.numberToUSFormat;
            /** @param {number} v */
            const fd = (v) => utils.formatters.dollars.format(v);
            /** @param {number} v */
            const fp = (v) => utils.formatters.percentage.format(v);
            /**
             * @param {ColorName} c
             * @param {string} t
             */
            const c = (c, t) => createColoredSpan({ color: c, text: t });

            const serInvestedAmount = c("green", fd(investedAmount));
            const serDaysCount = c("sky", f(daysCount));
            const serSats = c("orange", f(sats));
            const serBitcoin = c("orange", `~${f(bitcoin)}`);
            const serBitcoinValue = c("amber", fd(bitcoinValue));
            const serAveragePricePaid = c("lime", fd(averagePricePaid));
            const serRoi = c("yellow", fp(roi / 100));
            const serDollars = c("emerald", fd(dollars));
            const serTotalFeesPaid = c("rose", fd(totalFeesPaid));

            p1.innerHTML = `After exchanging ${serInvestedAmount} in the span of ${serDaysCount} days, you would have accumulated ${serSats} Satoshis (${serBitcoin} Bitcoin) worth today ${serBitcoinValue} at an average price of ${serAveragePricePaid} per Bitcoin with a return of investment of ${serRoi}, have ${serDollars} left and paid a total of ${serTotalFeesPaid} in fees.`;

            const dayDiff = Math.floor(
              utils.date.differenceBetween(new Date(), lastInvestDay),
            );
            const serDailyInvestment = c("emerald", fd(dailyInvestment));
            const setLastSatsAdded = c("orange", f(lastSatsAdded));
            p2.innerHTML = `You would've last bought ${c(
              "blue",
              dayDiff
                ? `${f(dayDiff)} ${dayDiff > 1 ? "days" : "day"} ago`
                : "today",
            )} and exchanged ${serDailyInvestment} for approximately ${setLastSatsAdded} Satoshis`;

            const serProfitableDaysRatio = c("green", fp(profitableDaysRatio));
            const serUnprofitableDaysRatio = c(
              "red",
              fp(unprofitableDaysRatio),
            );

            p3.innerHTML = `You would've been ${serProfitableDaysRatio} of the time profitable and ${serUnprofitableDaysRatio} of the time unprofitable.`;

            signals.createEffect(
              () => 0.073,
              (lowestAnnual4YReturn) => {
                const serLowestAnnual4YReturn = c(
                  "cyan",
                  `${fp(lowestAnnual4YReturn)}`,
                );

                const lowestAnnual4YReturnPercentage = 1 + lowestAnnual4YReturn;
                /**
                 * @param {number} power
                 */
                function bitcoinValueReturn(power) {
                  return (
                    bitcoinValue *
                    Math.pow(lowestAnnual4YReturnPercentage, power)
                  );
                }
                const bitcoinValueAfter4y = bitcoinValueReturn(4);
                const serBitcoinValueAfter4y = c(
                  "purple",
                  fd(bitcoinValueAfter4y),
                );
                const bitcoinValueAfter10y = bitcoinValueReturn(10);
                const serBitcoinValueAfter10y = c(
                  "fuchsia",
                  fd(bitcoinValueAfter10y),
                );
                const bitcoinValueAfter21y = bitcoinValueReturn(21);
                const serBitcoinValueAfter21y = c(
                  "pink",
                  fd(bitcoinValueAfter21y),
                );

                /** @param {number} v */
                p4.innerHTML = `The lowest annual return after 4 years has historically been ${serLowestAnnual4YReturn}.<br/>Using it as the baseline, your Bitcoin would be worth ${serBitcoinValueAfter4y} after 4 years, ${serBitcoinValueAfter10y} after 10 years and ${serBitcoinValueAfter21y} after 21 years.`;
              },
            );

            totalInvestedAmountData.set((a) => a);
            bitcoinValueData.set((a) => a);
            bitcoinData.set((a) => a);
            resultData.set((a) => a);
            dollarsLeftData.set((a) => a);
            totalValueData.set((a) => a);
            investmentData.set((a) => a);
            bitcoinAddedData.set((a) => a);
            averagePricePaidData.set((a) => a);
            bitcoinPriceData.set((a) => a);
            buyCountData.set((a) => a);
            totalFeesPaidData.set((a) => a);
            daysCountData.set((a) => a);
            profitableDaysRatioData.set((a) => a);
            unprofitableDaysRatioData.set((a) => a);
          },
        );
      });
    });
}
