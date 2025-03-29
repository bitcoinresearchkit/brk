// @ts-check

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Elements} args.elements
 */
export function init({ colors, elements, lightweightCharts, signals, utils }) {
  /**
   * @import { ColorName } from './types/self';
   *
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
            ...utils.serde.number,
            keyPrefix,
            key: "initial-amount",
          },
        }),
      },
      topUp: {
        amount: signals.createSignal(/** @type {number | null} */ (150), {
          save: {
            ...utils.serde.number,
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
            ...utils.serde.number,
            keyPrefix,
            key: "initial-swap",
          },
        }),
        recurrent: signals.createSignal(/** @type {number | null} */ (5), {
          save: {
            ...utils.serde.number,
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
            ...utils.serde.date,
            keyPrefix,
            key: "interval-start",
          },
        },
      ),
      end: signals.createSignal(/** @type {Date | null} */ (new Date()), {
        save: {
          ...utils.serde.date,
          keyPrefix,
          key: "interval-end",
        },
      }),
    },
    fees: {
      percentage: signals.createSignal(/** @type {number | null} */ (0.25), {
        save: {
          ...utils.serde.number,
          keyPrefix,
          key: "percentage",
        },
      }),
    },
  };

  parametersElement.append(
    utils.dom.createHeader({
      title: "Save in Bitcoin",
    }).headerElement,
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
    return `<span style="color: ${colors[color]()}; font-weight: var(--font-weight-bold)">${text}</span>`;
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDollar({
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
      input: utils.dom.createResetableInput(
        utils.dom.createSelect({
          id: "top-up-frequency",
          list: frequencies.list,
          signal: settings.dollars.topUp.frenquency,
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDollar({
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDollar({
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
      input: utils.dom.createResetableInput(
        utils.dom.createSelect({
          id: "investment-frequency",
          list: frequencies.list,
          signal: settings.bitcoin.investment.frequency,
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDollar({
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDate({
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputDate({
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
      input: utils.dom.createResetableInput(
        utils.dom.createInputNumberElement({
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
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinValueData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const resultData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const dollarsLeftData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const totalValueData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const investmentData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinAddedData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const averagePricePaidData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const bitcoinPriceData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const buyCountData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const totalFeesPaidData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const daysCountData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const profitableDaysRatioData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );
  const unprofitableDaysRatioData = signals.createSignal(
    /** @type {LineData<Time>[]} */ ([]),
    {
      equals: false,
    },
  );

  lightweightCharts.createChartElement({
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-0`,
    kind: "static",
    scale: "date",
    utils,
    config: [
      {
        unit: "US Dollars",
        config: [
          {
            title: "Fees Paid",
            type: "Line",
            color: colors.rose,
            data: totalFeesPaidData,
          },
          {
            title: "Dollars Left",
            type: "Line",
            color: colors.offDollars,
            data: dollarsLeftData,
          },
          {
            title: "Dollars Converted",
            type: "Line",
            color: colors.dollars,
            data: totalInvestedAmountData,
          },
          {
            title: "Bitcoin Value",
            type: "Line",
            color: colors.amber,
            data: bitcoinValueData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-1`,
    scale: "date",
    kind: "static",
    utils,
    config: [
      {
        unit: "US Dollars",
        config: [
          {
            title: "Bitcoin Stack",
            type: "Line",
            color: colors.bitcoin,
            data: bitcoinData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-average-price`,
    scale: "date",
    kind: "static",
    utils,
    config: [
      {
        unit: "US Dollars",
        config: [
          {
            title: "Bitcoin Price",
            type: "Line",
            color: colors.default,
            data: bitcoinPriceData,
          },
          {
            title: "Average Price Paid",
            type: "Line",
            color: colors.lightDollars,
            data: averagePricePaidData,
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-return-ratio`,
    scale: "date",
    kind: "static",
    utils,
    config: [
      {
        unit: "US Dollars",
        config: [
          {
            title: "Return Of Investment",
            type: "Baseline",
            data: resultData,
            // TODO: Doesn't work for some reason
            // options: {
            //   baseLineColor: "#888",
            //   baseLineVisible: true,
            //   baseLineWidth: 1,
            //   baseValue: {
            //     price: 0,
            //     type: "price",
            //   },
            // },
          },
        ],
      },
    ],
  });

  lightweightCharts.createChartElement({
    parent: resultsElement,
    signals,
    colors,
    id: `simulation-profitability-ratios`,
    kind: "static",
    scale: "date",
    utils,
    owner,
    config: [
      {
        unit: "Percentage",
        config: [
          {
            title: "Unprofitable Days Ratio",
            type: "Line",
            color: colors.red,
            data: unprofitableDaysRatioData,
          },
          {
            title: "Profitable Days Ratio",
            type: "Line",
            color: colors.green,
            data: profitableDaysRatioData,
          },
        ],
      },
    ],
  });

  const closes = datasets.getOrCreate("date", "date-to-close");
  closes.fetchRange(2009, new Date().getUTCFullYear()).then(() => {
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
            const year = date.getUTCFullYear();
            const time = utils.date.toString(date);

            if (topUpFrequency.isTriggerDay(date)) {
              dollars += topUpAmount;
            }

            const close = closes.ranges
              .at(utils.chunkIdToIndex("date", year))
              ?.json()?.dataset.map[utils.date.toString(date)];

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

          const serInvestedAmount = c("dollars", fd(investedAmount));
          const serDaysCount = c("sky", f(daysCount));
          const serSats = c("orange", f(sats));
          const serBitcoin = c("orange", `~${f(bitcoin)}`);
          const serBitcoinValue = c("amber", fd(bitcoinValue));
          const serAveragePricePaid = c("lightDollars", fd(averagePricePaid));
          const serRoi = c("yellow", fp(roi / 100));
          const serDollars = c("offDollars", fd(dollars));
          const serTotalFeesPaid = c("rose", fd(totalFeesPaid));

          p1.innerHTML = `After exchanging ${serInvestedAmount} in the span of ${serDaysCount} days, you would have accumulated ${serSats} Satoshis (${serBitcoin} Bitcoin) worth today ${serBitcoinValue} at an average price of ${serAveragePricePaid} per Bitcoin with a return of investment of ${serRoi}, have ${serDollars} left and paid a total of ${serTotalFeesPaid} in fees.`;

          const dayDiff = Math.floor(
            utils.date.differenceBetween(new Date(), lastInvestDay),
          );
          const serDailyInvestment = c("offDollars", fd(dailyInvestment));
          const setLastSatsAdded = c("bitcoin", f(lastSatsAdded));
          p2.innerHTML = `You would've last bought ${c("blue", dayDiff ? `${f(dayDiff)} ${dayDiff > 1 ? "days" : "day"} ago` : "today")} and exchanged ${serDailyInvestment} for approximately ${setLastSatsAdded} Satoshis`;

          const serProfitableDaysRatio = c("green", fp(profitableDaysRatio));
          const serUnprofitableDaysRatio = c("red", fp(unprofitableDaysRatio));

          p3.innerHTML = `You would've been ${serProfitableDaysRatio} of the time profitable and ${serUnprofitableDaysRatio} of the time unprofitable.`;

          signals.createEffect(lastValues, (lastValues) => {
            const lowestAnnual4YReturn = 0.2368;
            // const lowestAnnual4YReturn = lastValues?.["price-4y-compound-return"] || 0
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
                bitcoinValue * Math.pow(lowestAnnual4YReturnPercentage, power)
              );
            }
            const bitcoinValueAfter4y = bitcoinValueReturn(4);
            const serBitcoinValueAfter4y = c("purple", fd(bitcoinValueAfter4y));
            const bitcoinValueAfter10y = bitcoinValueReturn(10);
            const serBitcoinValueAfter10y = c(
              "fuchsia",
              fd(bitcoinValueAfter10y),
            );
            const bitcoinValueAfter21y = bitcoinValueReturn(21);
            const serBitcoinValueAfter21y = c("pink", fd(bitcoinValueAfter21y));

            /** @param {number} v */
            p4.innerHTML = `The lowest annual return after 4 years has historically been ${serLowestAnnual4YReturn}.<br/>Using it as the baseline, your Bitcoin would be worth ${serBitcoinValueAfter4y} after 4 years, ${serBitcoinValueAfter10y} after 10 years and ${serBitcoinValueAfter21y} after 21 years.`;
          });

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
