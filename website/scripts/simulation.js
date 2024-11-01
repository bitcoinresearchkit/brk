/**
 * @import {Options} from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Consts} args.consts
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.selected
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

  const storagePrefix = "save-in-bitcoin";
  const settings = {
    initial: signals.createSignal(/** @type {number | null} */ (1000), {
      save: {
        ...utils.serde.number,
        id: `${storagePrefix}-initial-amount`,
        param: "initial-amount",
      },
    }),
    later: signals.createSignal(/** @type {number | null} */ (0), {
      save: {
        ...utils.serde.number,
        id: `${storagePrefix}-later-amount`,
        param: "later-amount",
      },
    }),
    recurrent: {
      amount: signals.createSignal(/** @type {number | null} */ (100), {
        save: {
          ...utils.serde.number,
          id: `${storagePrefix}-recurrent-amount`,
          param: "recurrent-amount",
        },
      }),
    },
  };

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
        signal: settings.initial,
      }),
    }),
  );

  initialGroup.append(
    createInputField({
      name: "Converted over time",
      input: createInputDollar({
        id: "simulation-dollars-later",
        title: "Dollars to spread over time",
        signal: settings.later,
      }),
    }),
  );

  parametersElement.append(createHrElement());

  const recurrentGroup = createParameterGroup({
    title: "Recurrent",
    description:
      "The recurrent amount of dollars you're willing to eventually save in Bitcoin.",
  });
  parametersElement.append(recurrentGroup);

  recurrentGroup.append(
    createInputField({
      name: "Amount",
      input: createInputDollar({
        id: "simulation-dollars-recurrent",
        title: "Recurrent dollar amount",
        signal: settings.recurrent.amount,
      }),
    }),
  );

  const frequencyUL = appendUl({ parent: recurrentGroup });

  [{ name: "Daily" }, { name: "Weekly" }, { name: "Monthly" }].forEach(
    ({ name }) => {
      const li = appendLi({ name, parent: frequencyUL });
    },
  );

  const frequencyChoiceUL = appendUl({ parent: recurrentGroup });

  [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
  ].forEach((name) => {
    const li = appendLi({ name, parent: frequencyChoiceUL });
  });

  parametersElement.append(createHrElement());

  const today = signals.createSignal(utils.date.todayUTC());
  setInterval(() => {
    today.set(utils.date.todayUTC());
  }, consts.FIVE_SECONDS_IN_MS);

  const intervalGroup = createParameterGroup({
    title: "Interval",
    description: "wkfpweokf",
  });
  parametersElement.append(intervalGroup);

  /**
   * @param {Object} args
   * @param {HTMLElement} args.parent
   */
  function appendDiv({ parent }) {
    const div = window.document.createElement("div");
    parent.append(div);
    return div;
  }

  function createInputDateField() {
    const div = appendDiv({ parent: intervalGroup });

    appendInputDate({
      id: "",
      title: "",
      value: "2021-04-15",
      parent: div,
      signals,
      today,
    });

    appendButton({
      onClick: () => {},
      text: "Reset",
      title: "",
      parent: div,
    });

    return div;
  }

  createInputDateField();
  createInputDateField();

  parametersElement.append(createHrElement());

  const feesGroup = createParameterGroup({
    title: "Fees",
    description:
      "The amount of fees (in %) from where you'll be exchanging your currency",
  });
  parametersElement.append(feesGroup);

  createInputNumber({
    id: "",
    title: "",
    value: 0.25,
    parent: feesGroup,
    min: 0,
    max: 10,
  });

  parametersElement.append(createHrElement());

  const strategyGroup = createParameterGroup({
    title: "Strategy",
    description: "The strategy used to convert your fiat into Bitcoin",
  });
  parametersElement.append(strategyGroup);

  const ulStrategies = appendUl({ parent: strategyGroup });

  ["All in", "Weighted Local", "Weighted Cycle"].forEach((strategy) => {
    appendLi({
      name: strategy,
      parent: ulStrategies,
    });
  });

  //
  // On the side
  // Value in Bitcoin
  // Value in Dollars + total converted
  //
  // Value min estimated value in 4 years
  //
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

function createHrElement() {
  return window.document.createElement("hr");
}

/**
 *@param {Object} args
 *@param {string} args.id
 *@param {string} args.title
 *@param {number} args.value
 *@param {HTMLElement} args.parent
 *@param {number} args.min
 *@param {number} args.max
 */
function createInputNumber({ id, title, value, parent, min, max }) {
  const input = window.document.createElement("input");
  input.id = id;
  input.title = title;
  input.type = "number";
  input.value = String(value);
  input.min = String(min);
  input.max = String(max);
  parent.append(input);
  return input;
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
 * @param {Object} args
 * @param {string} args.id
 * @param {string} args.title
 * @param {Signal<number | null>} args.signal
 */
function createInputRangePercentage({ id, title, signal }) {
  const input = window.document.createElement("input");
  input.id = id;
  input.title = title;
  input.type = "range";
  input.min = "0";
  input.max = "100";

  const value = signal();
  input.value = value !== null ? String(value) : "";

  input.addEventListener("input", () => {
    const value = input.value;
    signal.set(value ? Number(value) : null);
  });

  return input;
}

/**
 * @param {Object} args
 * @param {HTMLElement} args.parent
 */
function appendUl({ parent }) {
  const ul = window.document.createElement("ul");
  parent.append(ul);
  return ul;
}

/**
 * @param {Object} args
 * @param {string} args.name
 * @param {HTMLUListElement} args.parent
 */
function appendLi({ name, parent }) {
  const li = window.document.createElement("li");
  li.innerHTML = name;
  parent.append(li);
  return li;
}

/**
 *@param {Object} args
 *@param {string} args.id
 *@param {string} args.title
 *@param {string} args.value
 *@param {HTMLElement} args.parent
 *@param {Accessor<Date>} args.today
 *@param {Signals} args.signals
 */
function appendInputDate({ id, title, value, parent, today, signals }) {
  const input = window.document.createElement("input");
  input.id = id;
  input.title = title;
  input.type = "date";
  input.value = value;

  input.min = "2011-01-01";

  signals.createEffect(today, (today) => {
    input.max = today.toJSON().split("T")[0];
  });

  parent.append(input);
  return input;
}

/**
 *@param {Object} args
 *@param {string} args.title
 *@param {string} args.text
 *@param {HTMLElement} args.parent
 *@param {VoidFunction} args.onClick
 */
function appendButton({ title, text, onClick, parent }) {
  const button = window.document.createElement("button");
  button.title = title;
  button.innerHTML = text;
  button.addEventListener("click", onClick);
  parent.append(button);
  return button;
}
