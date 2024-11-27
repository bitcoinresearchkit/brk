/**
 * @import {Options} from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Consts} args.consts
 * @param {LightweightCharts} args.lightweightCharts
 * @param {SimulationOption} args.selected
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Options} args.options
 * @param {Datasets} args.datasets
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {Ids} args.ids
 * @param {Accessor<boolean>} args.dark
 */
export function init({
  colors,
  consts,
  dark,
  datasets,
  elements,
  ids,
  lightweightCharts,
  options,
  selected,
  signals,
  utils,
  webSockets,
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
        amount: signals.createSignal(/** @type {number | null} */ (10), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-top-up-amount`,
            param: "top-up-amount",
          },
        }),
        frenquency: signals.createSignal(
          /** @type {Frequency} */ (frequencies.list[0]),
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
    swap: {
      amount: {
        initial: signals.createSignal(/** @type {number | null} */ (1000), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-initial-swap`,
            param: "initial-swap",
          },
        }),
        recurrent: signals.createSignal(/** @type {number | null} */ (10), {
          save: {
            ...utils.serde.number,
            id: `${storagePrefix}-recurrent-swap`,
            param: "recurrent-swap",
          },
        }),
      },
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

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "green",
        type: "Dollars",
        text: "Initial Amount",
      }),
      description: "The amount of dollars you have ready to swap on day one.",
      input: createInputDollar({
        id: "simulation-dollars-initial",
        title: "Initial Dollar Amount",
        signal: settings.dollars.initial.amount,
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
        "The recurrent amount of dollars you'll be putting aside to swap.",
      input: createInputDollar({
        id: "simulation-dollars-later",
        title: "Top Up Dollar Amount",
        signal: settings.dollars.topUp.amount,
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
        "The frequency at which you'll be putting aside the preceding amount of dollars.",
      input: utils.dom.createSelect({
        id: "top-up-frequency",
        list: frequencies.list,
        signal: settings.dollars.topUp.frenquency,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Swap",
        text: "Initial Amount",
      }),
      description:
        "The maximum initial amount of dollars you'll exchange on day one.",
      input: createInputDollar({
        id: "simulation-dollars-later",
        title: "Initial Swap Amount",
        signal: settings.swap.amount.initial,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Swap",
        text: "Recurrent Amount",
      }),
      description:
        "The maximum recurrent amount of dollars you'll be exchanging.",
      input: createInputDollar({
        id: "simulation-dollars-later",
        title: "Recurrent Swap Amount",
        signal: settings.swap.amount.recurrent,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "orange",
        type: "Swap",
        text: "Frequency",
      }),
      description:
        "The frequency at which you'll be exchanging the preceding amount.",
      input: utils.dom.createSelect({
        id: "top-up-frequency",
        list: frequencies.list,
        signal: settings.swap.frequency,
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
      input: createInputDateField({
        signal: settings.interval.start,
        signals,
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
      input: createInputDateField({
        signal: settings.interval.end,
        signals,
        utils,
      }),
    }),
  );

  parametersElement.append(
    createFieldElement({
      title: createColoredTypeHTML({
        color: "red",
        type: "Fees",
        text: "Percentage",
      }),
      description:
        "The amount of fees (in %) from where you'll be exchanging your dollars.",
      input: utils.dom.createInputNumberElement({
        id: "",
        title: "",
        signal: settings.fees.percentage,
        min: 0,
        max: 50,
        step: 0.01,
        signals,
      }),
    }),
  );

  const firstParagraph = window.document.createElement("p");

  const secondParagraph = window.document.createElement("p");

  const owner = signals.getOwner();

  const closes = datasets.getOrCreate("date", "date-to-close");
  closes.fetchRange(2009, new Date().getUTCFullYear()).then(() => {
    signals.runWithOwner(owner, () => {
      signals.createEffect(
        () => ({
          initialDollarAmount: settings.dollars.initial.amount() || 0,
          topUpAmount: settings.dollars.topUp.amount() || 0,
          topUpFrequency: settings.dollars.topUp.frenquency(),
          initialSwap: settings.swap.amount.initial() || 0,
          recurrentSwap: settings.swap.amount.recurrent() || 0,
          swapFrequency: settings.swap.frequency(),
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
          console.log({ start, end });

          resultsElement.innerHTML = "";
          resultsElement.append(firstParagraph);
          resultsElement.append(secondParagraph);

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

          let bitcoin = 0;
          let dollars = initialDollarAmount;
          let investedAmount = 0;
          let buyCount = 0;
          let averagePricePaid = 0;
          let bitcoinValue = 0;
          let roi = 0;
          let totalValue = 0;
          let totalFeesPaid = 0;
          let daysCount = range.length;
          let profitableDays = 0;
          let unprofitableDays = 0;

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

            let dailyInvestment = 0;
            /** @param {number} value  */
            function invest(value) {
              value = Math.min(dollars, value);
              dailyInvestment += value;
              dollars -= value;
              buyCount += 1;
            }
            if (!index) {
              invest(initialSwap);
            }
            if (swapFrequency.isTriggerDay(date) && dollars > 0) {
              invest(recurrentSwap);
            }

            let dailyInvestmentPostFees =
              dailyInvestment * (1 - (fees || 0) / 100);

            totalFeesPaid += dailyInvestment - dailyInvestmentPostFees;

            const bitcoinAdded = dailyInvestmentPostFees / close;
            bitcoin += bitcoinAdded;

            investedAmount += dailyInvestmentPostFees;

            bitcoinValue = close * bitcoin;

            totalValue = dollars + bitcoinValue;

            averagePricePaid = investedAmount / bitcoin;

            roi = (bitcoinValue / investedAmount - 1) * 100;

            const daysCount = index + 1;

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
              value: profitableDays / daysCount,
            });
          });

          const f = utils.locale.numberToUSFormat;
          /**
           * @param {string} c
           * @param {string} t
           */
          const c = (c, t) => createColoredSpan({ color: c, text: t });

          firstParagraph.innerHTML = `After exchanging ${c("dollar", `$${f(investedAmount)}`)} in the span of ${c("sky", f(daysCount))} days, you would've accumulated ${c("orange", f(bitcoin))} Bitcoin worth ${c("dollar", `$${f(bitcoinValue)}`)} at an average price of ${c("dollar", `$${f(averagePricePaid)}`)} per Bitcoin with a return of investment of ${c("yellow", `${f(roi)}%`)}.`;

          secondParagraph.innerHTML = `Work in progress`;

          lightweightCharts.createChart({
            parent: resultsElement,
            signals,
            colors,
            id: `simulation-0`,
            kind: "static",
            config: [
              {
                unit: "US Dollars",
                scale: "date",
                config: [
                  {
                    kind: "line",
                    color: colors.dollars,
                    owner,
                    data: totalInvestedAmountData,
                  },
                  {
                    kind: "line",
                    color: colors.bitcoin,
                    owner,
                    data: bitcoinValueData,
                  },
                  {
                    kind: "line",
                    color: colors.lime,
                    owner,
                    data: dollarsLeftData,
                  },
                  {
                    kind: "line",
                    color: colors.default,
                    owner,
                    data: totalValueData,
                  },
                  {
                    kind: "line",
                    color: colors.yellow,
                    owner,
                    data: investmentData,
                  },
                  {
                    kind: "line",
                    color: colors.red,
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
            kind: "static",
            config: [
              {
                unit: "US Dollars",
                scale: "date",
                config: [
                  {
                    kind: "line",
                    color: colors.bitcoin,
                    owner,
                    data: bitcoinData,
                  },
                  {
                    kind: "line",
                    color: colors.lightBitcoin,
                    owner,
                    data: bitcoinAddedData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
            parent: resultsElement,
            signals,
            colors,
            id: `simulation-2`,
            kind: "static",
            config: [
              {
                unit: "US Dollars",
                scale: "date",
                config: [
                  {
                    kind: "baseline",
                    owner,
                    data: resultData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
            parent: resultsElement,
            signals,
            colors,
            id: `simulation-3`,
            kind: "static",
            config: [
              {
                unit: "US Dollars",
                scale: "date",
                config: [
                  {
                    kind: "line",
                    owner,
                    color: colors.bitcoin,
                    data: bitcoinPriceData,
                  },
                  {
                    kind: "line",
                    owner,
                    color: colors.default,
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
            id: `simulation-4`,
            kind: "static",
            config: [
              {
                unit: "US Dollars",
                scale: "date",
                config: [
                  {
                    kind: "line",
                    owner,
                    color: colors.blue,
                    data: buyCountData,
                  },
                  {
                    kind: "line",
                    owner,
                    color: colors.sky,
                    data: daysCountData,
                  },
                ],
              },
            ],
          });

          lightweightCharts.createChart({
            parent: resultsElement,
            signals,
            colors,
            id: `simulation-5`,
            kind: "static",
            config: [
              {
                unit: "Percentage",
                scale: "date",
                config: [
                  {
                    kind: "line",
                    owner,
                    color: colors.pink,
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

  return div;
}

/**
 * @param {Object} args
 * @param {string} args.id
 * @param {string} args.title
 * @param {Signal<number | null>} args.signal
 */
function createInputDollar({ id, title, signal }) {
  const input = window.document.createElement("input");
  input.id = id;
  input.type = "number";
  input.placeholder = "US Dollars";
  input.min = "0";
  input.title = title;

  const value = signal();
  input.value = value !== null ? String(value) : "";

  input.addEventListener("input", () => {
    const value = input.value;
    signal.set(value ? Number(value) : null);
  });

  return input;
}

/**
 *
 * @param {Object} arg
 * @param {Signal<Date | null>} arg.signal
 * @param {Utilities} arg.utils
 * @param {Signals} arg.signals
 */
function createInputDateField({ signal, signals, utils }) {
  const div = window.document.createElement("div");

  div.append(
    utils.dom.createInputDate({
      id: "",
      title: "",
      signal,
      signals,
    }),
  );

  const button = utils.dom.createButtonElement({
    onClick: signal.reset,
    text: "Reset",
    title: "Reset field",
  });

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

/**
 * @param {Object} param0
 * @param {string} param0.color
 * @param {string} param0.type
 * @param {string} param0.text
 */
function createColoredTypeHTML({ color, type, text }) {
  return `${createColoredSpan({ color, text: `${type}:` })} ${text}`;
}

/**
 * @param {Object} param0
 * @param {string} param0.color
 * @param {string} param0.text
 */
function createColoredSpan({ color, text }) {
  return `<span style="color: var(--${color}); font-weight: var(--font-weight-bold)">${text}</span>`;
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

  /** @satisfies {((Frequency | {name: string; list: Frequency[]})[])} */
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
