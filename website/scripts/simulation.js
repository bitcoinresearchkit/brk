// @ts-check

/**
 * @import { Options } from './options';
 * @import { ColorName, Frequencies, Frequency } from './types/self';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Datasets} args.datasets
 * @param {Elements} args.elements
 * @param {Signal<LastValues>} args.lastValues
 */
export function init({
  colors,
  datasets,
  elements,
  lightweightCharts,
  signals,
  utils,
  lastValues,
}) {
  const simulationElement = elements.simulation;

  const parametersElement = window.document.createElement("div");
  simulationElement.append(parametersElement);
  const resultsElement = window.document.createElement("div");
  simulationElement.append(resultsElement);

  const frequencies = computeFrequencies();

  const storagePrefix = "save-in-bitcoin";
  const settings = {
    dollars: {
      initial: {
        amount: signals.createSignal(/** @type {number | null} */ (1000), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-initial-amount`,
            param: "initial-amount",
          },
        }),
      },
      topUp: {
        amount: signals.createSignal(/** @type {number | null} */ (150), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-top-up-amount`,
            param: "top-up-amount",
          },
        }),
        frenquency: signals.createSignal(
          /** @type {Frequency} */ (frequencies.list[3].list[0]),
          {
            save: {
              ...frequencies.serde,
              id: `${storagePrefix}-top-up-freq`,
              param: "top-up-freq",
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
            id: `${storagePrefix}-initial-swap`,
            param: "initial-swap",
          },
        }),
        recurrent: signals.createSignal(/** @type {number | null} */ (5), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-recurrent-swap`,
            param: "recurrent-swap",
          },
        }),
        frequency: signals.createSignal(
          /** @type {Frequency} */ (frequencies.list[0]),
          {
            save: {
              ...frequencies.serde,
              id: `${storagePrefix}-swap-freq`,
              param: "swap-freq",
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
            id: `${storagePrefix}-interval-start`,
            param: "interval-start",
          },
        },
      ),
      end: signals.createSignal(/** @type {Date | null} */ (new Date()), {
        save: {
          ...utils.serde.date,
          id: `${storagePrefix}-interval-end`,
          param: "interval-end",
        },
      }),
    },
    fees: {
      percentage: signals.createSignal(/** @type {number | null} */ (0.25), {
        save: {
          ...utils.serde.number,
          id: `${storagePrefix}-percentage`,
          param: "percentage",
        },
      }),
    },
  };

  parametersElement.append(
    utils.dom.createHeader({
      title: "Save in Bitcoin",
      description: "What if you bought Bitcoin in the past ?",
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
    createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Initial Amount",
      }),
      description:
        "The amount of dollars you have ready on the exchange on day one.",
      input: createResetableInput({
        ...utils.dom.createInputDollar({
          id: "simulation-dollars-initial",
          title: "Initial Dollar Amount",
          signal: settings.dollars.initial.amount,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Top Up Frequency",
      }),
      description:
        "The frequency at which you'll top up your account at the exchange.",
      input: createResetableInput({
        ...utils.dom.createSelect({
          id: "top-up-frequency",
          list: frequencies.list,
          signal: settings.dollars.topUp.frenquency,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Top Up Amount",
      }),
      description:
        "The recurrent amount of dollars you'll be transfering to said exchange.",
      input: createResetableInput({
        ...utils.dom.createInputDollar({
          id: "simulation-dollars-top-up-amount",
          title: "Top Up Dollar Amount",
          signal: settings.dollars.topUp.amount,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Initial Investment",
      }),
      description:
        "The amount, if available, of dollars that will be used to buy Bitcoin on day one.",
      input: createResetableInput({
        ...utils.dom.createInputDollar({
          id: "simulation-bitcoin-initial-investment",
          title: "Initial Swap Amount",
          signal: settings.bitcoin.investment.initial,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Investment Frequency",
      }),
      description: "The frequency at which you'll be buying Bitcoin.",
      input: createResetableInput({
        ...utils.dom.createSelect({
          id: "investment-frequency",
          list: frequencies.list,
          signal: settings.bitcoin.investment.frequency,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Bitcoin",
        text: "Recurrent Investment",
      }),
      description:
        "The recurrent amount, if available, of dollars that will be used to buy Bitcoin.",
      input: createResetableInput({
        ...utils.dom.createInputDollar({
          id: "simulation-bitcoin-recurrent-investment",
          title: "Bitcoin Recurrent Investment",
          signal: settings.bitcoin.investment.recurrent,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "sky",
        type: "Interval",
        text: "Start",
      }),
      description: "The first day of the simulation.",
      input: createResetableInput({
        ...utils.dom.createInputDate({
          id: "simulation-inverval-start",
          title: "First Simulation Date",
          signal: settings.interval.start,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "sky",
        type: "Interval",
        text: "End",
      }),
      description: "The last day of the simulation.",
      input: createResetableInput({
        ...utils.dom.createInputDate({
          id: "simulation-inverval-end",
          title: "Last Simulation Day",
          signal: settings.interval.end,
          signals,
        }),
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "red",
        type: "Fees",
        text: "Exchange",
      }),
      description: "The amount of trading fees (in %) at the exchange.",
      input: createResetableInput({
        ...utils.dom.createInputNumberElement({
          id: "simulation-fees",
          title: "Exchange Fees",
          signal: settings.fees.percentage,
          min: 0,
          max: 50,
          step: 0.01,
          signals,
          placeholder: "Fees",
        }),
        utils,
      }),
    }),
  );

  const p1 = window.document.createElement("p");
  const p2 = window.document.createElement("p");
  const p3 = window.document.createElement("p");
  const p4 = window.document.createElement("p");

  const owner = signals.getOwner();

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
          resultsElement.innerHTML = "";
          resultsElement.append(p1);
          resultsElement.append(p2);
          resultsElement.append(p3);
          resultsElement.append(p4);

          if (!start || !end || start > end) return;

          const range = utils.date.getRange(start, end);

          /** @type {LineData<Time>[]} */
          const totalInvestedAmountData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinValueData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinData = [];
          /** @type {LineData<Time>[]} */
          const resultData = [];
          /** @type {LineData<Time>[]} */
          const dollarsLeftData = [];
          /** @type {LineData<Time>[]} */
          const totalValueData = [];
          /** @type {LineData<Time>[]} */
          const investmentData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinAddedData = [];
          /** @type {LineData<Time>[]} */
          const averagePricePaidData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinPriceData = [];
          /** @type {LineData<Time>[]} */
          const buyCountData = [];
          /** @type {LineData<Time>[]} */
          const totalFeesPaidData = [];
          /** @type {LineData<Time>[]} */
          const daysCountData = [];
          /** @type {LineData<Time>[]} */
          const profitableDaysRatioData = [];
          /** @type {LineData<Time>[]} */
          const unprofitableDaysRatioData = [];

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

            const close = closes.fetchedJSONs
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

            bitcoinPriceData.push({
              time,
              value: close,
            });

            bitcoinData.push({
              time,
              value: bitcoin,
            });

            totalInvestedAmountData.push({
              time,
              value: investedAmount,
            });

            bitcoinValueData.push({
              time,
              value: bitcoinValue,
            });

            resultData.push({
              time,
              value: roi,
            });

            dollarsLeftData.push({
              time,
              value: dollars,
            });

            totalValueData.push({
              time,
              value: totalValue,
            });

            investmentData.push({
              time,
              value: dailyInvestment,
            });

            bitcoinAddedData.push({
              time,
              value: bitcoinAdded,
            });

            averagePricePaidData.push({
              time,
              value: averagePricePaid,
            });

            buyCountData.push({
              time,
              value: buyCount,
            });

            totalFeesPaidData.push({
              time,
              value: totalFeesPaid,
            });

            daysCountData.push({
              time,
              value: daysCount,
            });

            profitableDaysRatioData.push({
              time,
              value: profitableDaysRatio * 100,
            });

            unprofitableDaysRatioData.push({
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

          lightweightCharts.createChart({
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
                    title: "Bitcoin Value",
                    kind: "line",
                    color: colors.amber,
                    owner,
                    data: bitcoinValueData,
                  },
                  {
                    title: "Dollars Left",
                    kind: "line",
                    color: colors.offDollars,
                    owner,
                    data: dollarsLeftData,
                  },
                  {
                    title: "Dollars Converted",
                    kind: "line",
                    color: colors.dollars,
                    owner,
                    data: totalInvestedAmountData,
                  },
                  {
                    title: "Fees Paid",
                    kind: "line",
                    color: colors.rose,
                    owner,
                    data: totalFeesPaidData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
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
                    kind: "line",
                    color: colors.bitcoin,
                    owner,
                    data: bitcoinData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
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
                    kind: "line",
                    owner,
                    color: colors.default,
                    data: bitcoinPriceData,
                  },
                  {
                    title: "Average Price Paid",
                    kind: "line",
                    owner,
                    color: colors.lightDollars,
                    data: averagePricePaidData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
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
                    kind: "baseline",
                    owner,
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

          lightweightCharts.createChart({
            parent: resultsElement,
            signals,
            colors,
            id: `simulation-profitability-ratios`,
            kind: "static",
            scale: "date",
            utils,
            config: [
              {
                unit: "Percentage",
                config: [
                  {
                    title: "Unprofitable Days Ratio",
                    kind: "line",
                    owner,
                    color: colors.red,
                    data: unprofitableDaysRatioData,
                  },
                  {
                    title: "Profitable Days Ratio",
                    kind: "line",
                    owner,
                    color: colors.green,
                    data: profitableDaysRatioData,
                  },
                ],
              },
            ],
          });
        },
      );
    });
  });
}

/**
 * @param {Object} args
 * @param {HTMLInputElement} args.input
 * @param {string} args.name
 */
function createInputField({ name, input }) {
  const div = window.document.createElement("div");

  const label = window.document.createElement("label");
  div.append(label);
  // @ts-ignore
  label.for = input.id;
  label.innerHTML = name;

  div.append(input);

  return div;
}

/**
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.description
 * @param {HTMLElement} args.input
 */
function createFieldElement({ title, description, input }) {
  const div = window.document.createElement("div");

  const label = window.document.createElement("label");
  div.append(label);

  const titleElement = window.document.createElement("span");
  titleElement.innerHTML = title;
  label.append(titleElement);

  const descriptionElement = window.document.createElement("small");
  descriptionElement.innerHTML = description;
  label.append(descriptionElement);

  div.append(input);

  const forId = input.id || input.firstElementChild?.id;

  if (!forId) {
    console.log(input);
    throw `Input should've an ID`;
  }

  // @ts-ignore
  label.for = forId;

  return div;
}

/**
 * @param {Object} param0
 * @param {Signal<any>} param0.signal
 * @param {HTMLInputElement} [param0.input]
 * @param {HTMLSelectElement} [param0.select]
 * @param {Utilities} param0.utils
 */
function createResetableInput({ input, select, signal, utils }) {
  const div = window.document.createElement("div");

  const element = input || select;
  if (!element) throw "createResetableField element missing";
  div.append(element);

  const button = utils.dom.createButtonElement({
    onClick: signal.reset,
    text: "Reset",
    title: "Reset field",
  });
  button.type = "reset";

  div.append(button);

  return div;
}

/** @param {number} day  */
function getOrdinalDay(day) {
  const rest = (day % 30) % 20;

  return `${day}${
    rest === 1 ? "st" : rest === 2 ? "nd" : rest === 3 ? "rd" : "th"
  }`;
}

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
