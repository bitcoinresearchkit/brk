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

  const getDefaultIntervalStart = () => new Date("2021-04-15");
  const getDefaultIntervalEnd = () => new Date();

  const storagePrefix = "save-in-bitcoin";
  const settings = {
    initial: {
      firstDay: signals.createSignal(/** @type {number | null} */ (1000), {
        save: {
          ...utils.serde.number,
          id: `${storagePrefix}-initial-amount`,
          param: "initial-amount",
        },
      }),
      overTime: signals.createSignal(/** @type {number | null} */ (0), {
        save: {
          ...utils.serde.number,
          id: `${storagePrefix}-later-amount`,
          param: "later-amount",
        },
      }),
    },
    recurrent: {
      amount: signals.createSignal(/** @type {number | null} */ (100), {
        save: {
          ...utils.serde.number,
          id: `${storagePrefix}-recurrent-amount`,
          param: "recurrent-amount",
        },
      }),
    },
    interval: {
      start: signals.createSignal(
        /** @type {Date | null} */ (getDefaultIntervalStart()),
        {
          save: {
            ...utils.serde.date,
            id: `${storagePrefix}-interval-start`,
            param: "interval-start",
          },
        },
      ),
      end: signals.createSignal(
        /** @type {Date | null} */ (getDefaultIntervalEnd()),
        {
          save: {
            ...utils.serde.date,
            id: `${storagePrefix}-interval-end`,
            param: "interval-end",
          },
        },
      ),
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

  const { headerElement } = utils.dom.createHeader({
    title: selected.title,
    description: selected.serializedPath,
  });
  parametersElement.append(headerElement);

  const initialGroup = createParameterGroup({
    title: "Initial",
    description:
      "The initial amount of dollars you're willing to eventually save in Bitcoin.",
  });
  parametersElement.append(initialGroup);

  initialGroup.append(
    createInputField({
      name: "Directly converted",
      input: createInputDollar({
        id: "simulation-dollars-initial",
        title: "Initial amount of dollars converted",
        signal: settings.initial.firstDay,
      }),
    }),
  );

  initialGroup.append(
    createInputField({
      name: "Converted over time",
      input: createInputDollar({
        id: "simulation-dollars-later",
        title: "Dollars to spread over time",
        signal: settings.initial.overTime,
      }),
    }),
  );

  const topUpGroup = createParameterGroup({
    title: "Top Up",
    description:
      "The topUp amount of dollars you're willing to eventually save in Bitcoin.",
  });
  parametersElement.append(topUpGroup);

  const recurrentGroup = createParameterGroup({
    title: "Recurrent",
    description:
      "The recurrent amount of dollars you're willing to eventually save in Bitcoin.",
  });
  parametersElement.append(recurrentGroup);

  recurrentGroup.append(
    createInputField({
      name: "Maximum Amount",
      input: createInputDollar({
        id: "simulation-dollars-recurrent",
        title: "Recurrent dollar amount",
        signal: settings.recurrent.amount,
      }),
    }),
  );

  const frequencyUL = utils.dom.createUlElement();
  recurrentGroup.append(frequencyUL);

  [
    { name: "Daily" },
    {
      name: "Weekly",
      sub: [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
      ],
    },
    {
      name: "Monthly",
      sub: [
        "The 1st",
        "The 2nd",
        "The 3rd",
        "The 4th",
        "The 5th",
        "The 6th",
        "The 7th",
        "The 8th",
        "The 9th",
        "The 10th",
        "The 11th",
        "The 12th",
        "The 13th",
        "The 14th",
        "The 15th",
        "The 16th",
        "The 17th",
        "The 18th",
        "The 19th",
        "The 20th",
        "The 21st",
        "The 22nd",
        "The 23rd",
        "The 24th",
        "The 25th",
        "The 26th",
        "The 27th",
        "The 28th",
      ],
    },
  ].forEach(({ name, sub }, index) => {
    const li = utils.dom.createLiElement();
    const { label, input } = utils.dom.createLabeledInput({
      inputId: `frequency-${name}`,
      inputName: "frequency",
      inputValue: name.toLowerCase(),
      labelTitle: name,
      inputChecked: !index,
      onClick: () => {},
    });
    label.append(name);
    li.append(label);
    if (sub) {
      const parentName = name;
      const ul = utils.dom.createUlElement();
      li.append(ul);
      sub.forEach((name) => {
        const li = utils.dom.createLiElement();
        const { label, input } = utils.dom.createLabeledInput({
          inputId: `frequency-${parentName}-${name}`,
          inputName: `frequency-${parentName}`,
          inputValue: name.toLowerCase(),
          labelTitle: name,
          inputChecked: !index,
          onClick: () => {},
        });
        label.append(name);
        li.append(label);
        ul.append(li);
      });
    }
    frequencyUL.append(li);
  });

  const frequencyChoiceUL = utils.dom.createUlElement();
  recurrentGroup.append(frequencyChoiceUL);

  const intervalGroup = createParameterGroup({
    title: "Interval",
    description: "wkfpweokf",
  });
  parametersElement.append(intervalGroup);

  console.log("weofpwklfpwkofwepokf");

  createInputDateField({
    signal: settings.interval.start,
    getDefault: getDefaultIntervalStart,
    parent: intervalGroup,
    signals,
    utils,
  });
  createInputDateField({
    signal: settings.interval.end,
    getDefault: getDefaultIntervalEnd,
    parent: intervalGroup,
    signals,
    utils,
  });

  const feesGroup = createParameterGroup({
    title: "Fees",
    description:
      "The amount of fees (in %) from where you'll be exchanging your currency",
  });
  parametersElement.append(feesGroup);

  const input = utils.dom.createInputNumberElement({
    id: "",
    title: "",
    signal: settings.fees.percentage,
    min: 0,
    max: 50,
    step: 0.01,
    signals,
  });
  feesGroup.append(input);

  // parametersElement.append(utils.dom.createHrElement());

  // const strategyGroup = createParameterGroup({
  //   title: "Strategy",
  //   description: "The strategy used to convert your fiat into Bitcoin",
  // });
  // parametersElement.append(strategyGroup);

  // const ulStrategies = utils.dom.createUlElement();
  // strategyGroup.append(ulStrategies);

  // ["All in", "Weighted Local", "Weighted Cycle"].forEach((strategy) => {
  //   const li = utils.dom.createLiElement();
  //   li.append(strategy);
  //   ulStrategies.append(li);
  // });

  const parent = window.document.createElement("div");
  parent.classList.add("chart-list");
  resultsElement.append(parent);

  signals.createEffect(settings.interval.start, (start) => {
    console.log("start", start);
  });

  signals.createEffect(settings.interval.end, (end) => {
    console.log("end", end);
  });

  const owner = signals.getOwner();

  const closes = datasets.getOrCreate("date", "date-to-close");
  closes.fetchRange(2009, new Date().getUTCFullYear()).then(() => {
    signals.runWithOwner(owner, () => {
      signals.createEffect(
        () => ({
          initialAmount: settings.initial.firstDay() || 0,
          recurrentAmount: settings.recurrent.amount() || 0,
          dollarsLeft: settings.initial.overTime() || 0,
          start: settings.interval.start(),
          end: settings.interval.end(),
          fees: settings.fees.percentage(),
        }),
        // ({ initialAmount, recurrentAmount, dollarsLeft, start, end }) => {
        //   console.log({
        //     start,
        //     end,
        //   });
        // },
        ({ initialAmount, recurrentAmount, dollarsLeft, start, end, fees }) => {
          console.log({ start, end });
          parent.innerHTML = "";

          if (!start || !end || start > end) return;

          const range = utils.date.getRange(start, end);

          let investedAmount = 0;

          /** @type {LineData<Time>[]} */
          const investedData = [];
          /** @type {LineData<Time>[]} */
          const returnData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinData = [];
          /** @type {LineData<Time>[]} */
          const resultData = [];
          /** @type {LineData<Time>[]} */
          const dollarsData = [];
          /** @type {LineData<Time>[]} */
          const totalData = [];
          /** @type {LineData<Time>[]} */
          const investmentData = [];
          /** @type {LineData<Time>[]} */
          const bitcoinAddedData = [];

          let bitcoin = 0;

          let feesPaid = 0;

          range.forEach((date, index) => {
            const year = date.getUTCFullYear();
            const time = utils.date.toString(date);

            const close = closes.fetchedJSONs
              .at(utils.chunkIdToIndex("date", year))
              ?.json()?.dataset.map[utils.date.toString(date)];

            if (!close) return;

            let investmentPreFees =
              (!index ? initialAmount : 0) + recurrentAmount;

            if (dollarsLeft > 0) {
              if (dollarsLeft >= recurrentAmount) {
                investmentPreFees += recurrentAmount;
                dollarsLeft -= recurrentAmount;
              } else {
                investmentPreFees += dollarsLeft;
                dollarsLeft = 0;
              }
            }

            let investment = investmentPreFees * (1 - (fees || 0) / 100);
            feesPaid += investmentPreFees - investment;

            const bitcoinAdded = investment / close;
            bitcoin += bitcoinAdded;

            investedAmount += investment;

            const _return = close * bitcoin;

            bitcoinData.push({
              time,
              value: bitcoin,
            });

            investedData.push({
              time,
              value: investedAmount,
            });

            returnData.push({
              time,
              value: _return,
            });

            resultData.push({
              time,
              value: (_return / investedAmount - 1) * 100,
            });

            dollarsData.push({
              time,
              value: dollarsLeft,
            });

            totalData.push({
              time,
              value: dollarsLeft + _return,
            });

            investmentData.push({
              time,
              value: investment,
            });

            bitcoinAddedData.push({
              time,
              value: bitcoinAdded,
            });
          });

          (() => {
            const chartWrapper = window.document.createElement("div");
            chartWrapper.classList.add("chart-wrapper");
            parent.append(chartWrapper);

            const chartDiv = window.document.createElement("div");
            chartDiv.classList.add("chart-div");
            chartWrapper.append(chartDiv);

            const chart = lightweightCharts.createChart({
              scale: "date",
              element: chartDiv,
              signals,
              colors,
              options: {
                handleScale: false,
                handleScroll: false,
              },
            });

            const line = chart.addLineSeries();

            line.setData(investedData);

            const line2 = chart.addLineSeries();

            line2.setData(returnData);

            const line3 = chart.addLineSeries();

            line3.setData(dollarsData);

            const line4 = chart.addLineSeries();

            line4.setData(totalData);

            const line5 = chart.addLineSeries();

            line5.setData(investmentData);

            chart.timeScale().fitContent();
          })();

          (() => {
            const chartWrapper = window.document.createElement("div");
            chartWrapper.classList.add("chart-wrapper");
            parent.append(chartWrapper);

            const chartDiv = window.document.createElement("div");
            chartDiv.classList.add("chart-div");
            chartWrapper.append(chartDiv);

            const chart = lightweightCharts.createChart({
              scale: "date",
              element: chartDiv,
              signals,
              colors,
              options: {
                handleScale: false,
                handleScroll: false,
              },
            });

            const line = chart.addLineSeries();

            line.setData(bitcoinData);

            const line2 = chart.addLineSeries();

            line2.setData(bitcoinAddedData);

            chart.timeScale().fitContent();
          })();

          (() => {
            const chartWrapper = window.document.createElement("div");
            chartWrapper.classList.add("chart-wrapper");
            parent.append(chartWrapper);

            const chartDiv = window.document.createElement("div");
            chartDiv.classList.add("chart-div");
            chartWrapper.append(chartDiv);

            const chart = lightweightCharts.createChart({
              scale: "date",
              element: chartDiv,
              signals,
              colors,
              options: {
                handleScale: false,
                handleScroll: false,
              },
            });

            const line = chart.addLineSeries();

            line.setData(resultData);

            chart.timeScale().fitContent();
          })();
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
 */
function createParameterGroup({ title, description }) {
  const div = window.document.createElement("div");

  const wrapper = window.document.createElement("div");
  div.append(wrapper);

  const titleElement = window.document.createElement("h4");
  titleElement.innerHTML = title;
  wrapper.append(titleElement);

  const descriptionElement = window.document.createElement("small");
  descriptionElement.innerHTML = description;
  wrapper.append(descriptionElement);

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
 * @param {() => Date | null} arg.getDefault
 * @param {HTMLElement} arg.parent
 * @param {Utilities} arg.utils
 * @param {Signals} arg.signals
 */
function createInputDateField({ signal, getDefault, parent, signals, utils }) {
  const div = window.document.createElement("div");
  parent.append(div);

  div.append(
    utils.dom.createInputDate({
      id: "",
      title: "",
      signal,
      signals,
    }),
  );

  const button = utils.dom.createButtonElement({
    onClick: () => {
      signal.set(getDefault());
    },
    text: "Reset",
    title: "Reset field",
  });

  div.append(button);

  return div;
}
