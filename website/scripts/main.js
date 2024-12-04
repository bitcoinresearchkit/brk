// @ts-check

/**
 * @import { Option, ResourceDataset, TimeScale, TimeRange, Unit, Weighted, DatasetPath, OHLC, FetchedJSON, DatasetValue, FetchedResult, AnyDatasetPath, Color, DatasetCandlestickData, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, AnyPath, SimulationOption, Frequency, LastValues } from "./types/self"
 * @import {createChart as CreateClassicChart, createChartEx as CreateCustomChart, LineStyleOptions } from "../packages/lightweight-charts/v4.2.0/types";
 * @import { Marker,  CreatePaneParameters,  HoveredLegend, ChartPane, SplitSeries, SingleSeries, CreateSplitSeriesParameters, LineSeriesBlueprint, CandlestickSeriesBlueprint, BaselineSeriesBlueprint, CreateBaseSeriesParameters, BaseSeries, RemoveSeriesBlueprintFluff, SplitSeriesBlueprint, AnySeries, PriceSeriesType } from "../packages/lightweight-charts/types";
 * @import * as _ from "../packages/ufuzzy/v1.0.14/types"
 * @import { DeepPartial, ChartOptions, IChartApi, IHorzScaleBehavior, WhitespaceData, SingleValueData, ISeriesApi, Time, LineData, LogicalRange, SeriesMarker, CandlestickData, SeriesType, BaselineStyleOptions, SeriesOptionsCommon } from "../packages/lightweight-charts/v4.2.0/types"
 * @import { DatePath, HeightPath, LastPath } from "./types/paths";
 * @import { SignalOptions } from "../packages/solid-signals/2024-11-02/types/core/core"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup, Owner } from "../packages/solid-signals/2024-11-02/types/core/owner"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, Accessor, Setter, createMemo as CreateMemo, createRoot as CreateRoot, runWithOwner as RunWithOwner } from "../packages/solid-signals/2024-11-02/types/signals";
 * @import {Signal, Signals} from "../packages/solid-signals/types";
 */

function initPackages() {
  const imports = {
    async signals() {
      return import("../packages/solid-signals/wrapper.js").then((d) =>
        d.default.then((d) => d),
      );
    },
    async lightweightCharts() {
      return window.document.fonts.ready.then(() =>
        import("../packages/lightweight-charts/wrapper.js").then((d) =>
          d.default.then((d) => d),
        ),
      );
    },
    async leanQr() {
      return import("../packages/lean-qr/v2.3.4/script.js").then((d) => d);
    },
    async ufuzzy() {
      return import("../packages/ufuzzy/v1.0.14/script.js").then(
        ({ default: d }) => d,
      );
    },
  };

  /**
   * @typedef {ReturnType<typeof imports.signals>} SignalsPromise
   * @typedef {ReturnType<typeof imports.lightweightCharts>} LightweightChartsPromise
   * @typedef {ReturnType<typeof imports.leanQr>} LeanQrPromise
   * @typedef {ReturnType<typeof imports.ufuzzy>} uFuzzyPromise
   */

  /**
   * @template {keyof typeof imports} K
   * @param {K} key
   */
  function importPackage(key) {
    /** @type {ReturnType<typeof imports[K]> | null} */
    let packagePromise = null;

    return function () {
      if (!packagePromise) {
        // @ts-ignore
        packagePromise = imports[key]();
      }
      return /** @type {ReturnType<typeof imports[K]>} */ (packagePromise);
    };
  }

  return {
    signals: importPackage("signals"),
    lightweightCharts: importPackage("lightweightCharts"),
    leanQr: importPackage("leanQr"),
    ufuzzy: importPackage("ufuzzy"),
  };
}
const packages = initPackages();
/**
 * @typedef {Awaited<ReturnType<typeof packages.lightweightCharts>>} LightweightCharts
 * @typedef {ReturnType<LightweightCharts['createChart']>} Chart
 */

const options = import("./options.js");

const utils = {
  /**
   * @param {string} serialized
   * @returns {boolean}
   */
  isSerializedBooleanTrue(serialized) {
    return serialized === "true" || serialized === "1";
  },
  /**
   * @param {number} ms
   */
  sleep(ms) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
  },
  yield() {
    return this.sleep(0);
  },
  array: {
    /**
     * @param {number} start
     * @param {number} end
     */
    range(start, end) {
      const range = [];
      while (start <= end) {
        range.push(start);
        start += 1;
      }
      return range;
    },
  },
  dom: {
    /**
     * @param {string} id
     * @returns {HTMLElement}
     */
    getElementById(id) {
      const element = window.document.getElementById(id);
      if (!element) throw `Element with id = "${id}" should exist`;
      return element;
    },
    /**
     * @param {string} name
     */
    queryOrCreateMetaElement(name) {
      let meta = /** @type {HTMLMetaElement | null} */ (
        window.document.querySelector(`meta[name="${name}"]`)
      );

      if (!meta) {
        meta = window.document.createElement("meta");
        meta.name = name;
        elements.head.appendChild(meta);
      }
      return meta;
    },
    /**
     * @param {HTMLElement} element
     */
    isHidden(element) {
      return element.tagName !== "BODY" && !element.offsetParent;
    },
    /**
     *
     * @param {HTMLElement} element
     * @param {VoidFunction} callback
     */
    onFirstIntersection(element, callback) {
      const observer = new IntersectionObserver((entries) => {
        for (let i = 0; i < entries.length; i++) {
          if (entries[i].isIntersecting) {
            callback();
            observer.disconnect();
          }
        }
      });
      observer.observe(element);
    },
    /**
     * @param {string} name
     */
    createSpanName(name) {
      const spanName = window.document.createElement("span");
      spanName.classList.add("name");
      const [first, second, third] = name.split("-");
      spanName.innerHTML = first;

      if (second) {
        const smallRest = window.document.createElement("small");
        smallRest.innerHTML = `— ${second}`;
        spanName.append(smallRest);

        if (third) {
          throw "Shouldn't have more than one dash";
        }
      }

      return spanName;
    },
    /**
     * @param {Object} arg
     * @param {string} arg.href
     * @param {string} [arg.text]
     * @param {boolean} [arg.blank]
     * @param {VoidFunction} [arg.onClick]
     * @param {boolean} [arg.preventDefault]
     */
    createAnchorElement({ text, href, blank, onClick, preventDefault }) {
      const anchor = window.document.createElement("a");
      anchor.href = href;

      if (text) {
        anchor.innerText = text;
      }

      if (blank) {
        anchor.target = "_blank";
        anchor.rel = "noopener noreferrer";
      }

      if (onClick || preventDefault) {
        if (onClick) {
          anchor.addEventListener("click", (event) => {
            event.preventDefault();
            onClick();
          });
        }
      }

      return anchor;
    },
    /**
     * @param {Object} arg
     * @param {string} arg.text
     * @param {string} arg.title
     * @param {VoidFunction} arg.onClick
     */
    createButtonElement({ text, onClick, title }) {
      const button = window.document.createElement("button");

      button.innerHTML = text;

      button.title = title;

      button.addEventListener("click", onClick);

      return button;
    },
    /**
     * @param {Object} args
     * @param {string} args.inputName
     * @param {string} args.inputId
     * @param {string} args.inputValue
     * @param {boolean} [args.inputChecked=false]
     * @param {string} args.labelTitle
     * @param {'solo' | 'multi'} args.type
     * @param {(event: MouseEvent) => void} [args.onClick]
     */
    createLabeledInput({
      inputId,
      inputName,
      inputValue,
      inputChecked = false,
      labelTitle,
      onClick,
      type,
    }) {
      const label = window.document.createElement("label");

      inputId = inputId.toLowerCase();

      const input = window.document.createElement("input");
      if (type === "multi") {
        input.type = "radio";
        input.name = inputName;
      } else {
        input.type = "checkbox";
      }
      input.id = inputId;
      input.value = inputValue;
      input.checked = inputChecked;
      label.append(input);

      label.id = `${inputId}-label`;
      label.title = labelTitle;
      label.htmlFor = inputId;

      if (onClick) {
        label.addEventListener("click", onClick);
      }

      return {
        label,
        input,
      };
    },
    /**
     * @param {HTMLElement} parent
     * @param {HTMLElement} child
     * @param {number} index
     */
    insertElementAtIndex(parent, child, index) {
      if (!index) index = 0;
      if (index >= parent.children.length) {
        parent.appendChild(child);
      } else {
        parent.insertBefore(child, parent.children[index]);
      }
    },
    /**
     * @param {string} url
     * @param {boolean} [targetBlank]
     */
    open(url, targetBlank) {
      console.log(`open: ${url}`);
      const a = window.document.createElement("a");
      elements.body.append(a);
      a.href = url;

      if (targetBlank) {
        a.target = "_blank";
        a.rel = "noopener noreferrer";
      }

      a.click();
      a.remove();
    },
    /**
     * @param {string} text
     */
    createItalic(text) {
      const italic = window.document.createElement("i");
      italic.innerHTML = text;
      return italic;
    },
    /**
     * @param {string} href
     */
    importStyle(href) {
      const link = document.createElement("link");
      link.href = href;
      link.type = "text/css";
      link.rel = "stylesheet";
      link.media = "screen,print";
      elements.head.appendChild(link);
      return link;
    },
    /**
     * @param {string} href
     * @param {VoidFunction} callback
     */
    importStyleAndThen(href, callback) {
      this.importStyle(href).addEventListener("load", callback);
    },
    /**
     * @param {Object} args
     * @param {string | Accessor<string>} args.title
     * @param {string} args.id
     * @param {Readonly<string[]>} args.choices
     * @param {string} args.selected
     * @param {{createEffect: CreateEffect}} args.signals
     */
    createHorizontalChoiceField({ title, id, choices, selected, signals }) {
      const field = window.document.createElement("div");
      field.classList.add("field");

      const legend = window.document.createElement("legend");
      if (typeof title === "string") {
        legend.innerHTML = title;
      } else {
        signals.createEffect(title, (title) => {
          legend.innerHTML = title;
        });
      }
      field.append(legend);

      const hr = window.document.createElement("hr");
      field.append(hr);

      const div = window.document.createElement("div");
      field.append(div);

      choices.forEach((choice) => {
        const inputValue = choice.toLowerCase();
        const { label } = utils.dom.createLabeledInput({
          inputId: `${id}-${choice.toLowerCase()}`,
          inputName: id,
          inputValue,
          inputChecked: inputValue === selected,
          labelTitle: choice,
          type: "multi",
        });

        const text = window.document.createTextNode(choice);
        label.append(text);
        div.append(label);
      });

      return field;
    },
    createUlElement() {
      return window.document.createElement("ul");
    },
    createLiElement() {
      return window.document.createElement("li");
    },
    /**
     * @param {Object} args
     * @param {string} args.id
     * @param {string} args.title
     * @param {string} args.placeholder
     * @param {Signal<number | null>} args.signal
     * @param {number} args.min
     * @param {number} args.step
     * @param {number} [args.max]
     * @param {{createEffect: typeof CreateEffect}} args.signals
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
     * @param {{createEffect: typeof CreateEffect}} args.signals
     */
    createInputDollar({ id, title, signal, signals }) {
      return this.createInputNumberElement({
        id,
        placeholder: "US Dollars",
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
     * @param {{createEffect: typeof CreateEffect}} args.signals
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
          const date = signal();
          return date ? utils.date.toString(date) : "";
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
     * @param {string} [param0.title]
     * @param {string} [param0.description]
     */
    createHeader({ title, description }) {
      const headerElement = window.document.createElement("header");

      const div = window.document.createElement("div");
      headerElement.append(div);

      const h1 = window.document.createElement("h1");
      div.append(h1);
      h1.style.display = "flex";
      h1.style.flexDirection = "column";

      const titleElement = window.document.createElement("span");
      if (title) {
        titleElement.append(title);
      }
      h1.append(titleElement);
      titleElement.style.display = "block";

      const descriptionElement = window.document.createElement("small");
      if (description) {
        descriptionElement.append(description);
      }
      h1.append(descriptionElement);

      return {
        headerElement,
        titleElement,
        descriptionElement,
      };
    },
    /**
     * @param {Object} param0
     * @param {string} param0.name
     * @param {string} param0.value
     */
    createOption({ name, value }) {
      const option = window.document.createElement("option");
      option.value = value;
      option.innerText = name;
      return option;
    },
    /**
     * @template {{name: string; value: string}} T
     * @param {Object} param0
     * @param {string} param0.id
     * @param {(({name: string; value: string} & T) | {name: string; list: ({name: string; value: string} & T)[]})[]} param0.list
     * @param {Signal<T>} param0.signal
     */
    createSelect({ id, list, signal }) {
      const select = window.document.createElement("select");
      select.name = id;
      select.id = id;

      /** @type {Record<string, VoidFunction>} */
      const setters = {};

      list.forEach((anyOption, index) => {
        if ("list" in anyOption) {
          const { name, list } = anyOption;
          const optGroup = window.document.createElement("optgroup");
          optGroup.label = name;
          select.append(optGroup);
          list.forEach((option) => {
            optGroup.append(this.createOption(option));
            setters[option.value] = () => signal.set(option);
          });
        } else {
          select.append(this.createOption(anyOption));
          setters[anyOption.value] = () => signal.set(anyOption);
        }
        if (index !== list.length - 1) {
          select.append(window.document.createElement("hr"));
        }
      });

      select.addEventListener("change", () => {
        const callback = setters[select.value];
        // @ts-ignore
        if (callback) {
          callback();
        }
      });

      select.value = signal().value;

      return { select, signal };
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

      const button = this.createButtonElement({
        onClick: signal.reset,
        text: "Reset",
        title: "Reset field",
      });
      button.type = "reset";

      div.append(button);

      return div;
    },
    /**
     * @param {Object} args
     * @param {string} args.title
     * @param {string} args.description
     * @param {HTMLElement} args.input
     */
    createFieldElement({ title, description, input }) {
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

      label.htmlFor = forId;

      return div;
    },
    /**
     * @param {'left' | 'bottom' | 'top' | 'right'} position
     */
    createShadow(position) {
      const div = window.document.createElement("div");
      div.classList.add(`shadow-${position}`);
      return div;
    },
  },
  url: {
    chartParamsWhitelist: ["from", "to"],
    /**
     * @param {string} pathname
     */
    pushHistory(pathname) {
      const urlParams = new URLSearchParams(window.location.search);
      pathname ||= window.location.pathname;

      window.history.pushState(null, "", `${pathname}?${urlParams.toString()}`);
    },
    /**
     * @param {Object} args
     * @param {URLSearchParams} [args.urlParams]
     * @param {string} [args.pathname]
     */
    replaceHistory({ urlParams, pathname }) {
      urlParams ||= new URLSearchParams(window.location.search);
      pathname ||= window.location.pathname;

      window.history.replaceState(
        null,
        "",
        `${pathname}?${urlParams.toString()}`,
      );
    },
    /**
     * @param {Option} option
     */
    resetParams(option) {
      const urlParams = new URLSearchParams();

      if (option.kind === "chart") {
        [...new URLSearchParams(window.location.search).entries()]
          .filter(([key, _]) => this.chartParamsWhitelist.includes(key))
          .forEach(([key, value]) => {
            urlParams.set(key, value);
          });
      }

      this.replaceHistory({ urlParams, pathname: option.id });
    },
    /**
     * @param {string} key
     * @param {string | boolean | null | undefined} value
     */
    writeParam(key, value) {
      const urlParams = new URLSearchParams(window.location.search);

      if (value !== null && value !== undefined) {
        urlParams.set(key, String(value));
      } else {
        urlParams.delete(key);
      }

      this.replaceHistory({ urlParams });
    },
    /**
     * @param {string} key
     */
    removeParam(key) {
      this.writeParam(key, undefined);
    },
    /**
     *
     * @param {string} key
     * @returns {boolean | null}
     */
    readBoolParam(key) {
      const parameter = this.readParam(key);

      if (parameter) {
        return utils.isSerializedBooleanTrue(parameter);
      }

      return null;
    },
    /**
     *
     * @param {string} key
     * @returns {number | null}
     */
    readNumberParam(key) {
      const parameter = this.readParam(key);

      if (parameter) {
        return Number(parameter);
      }

      return null;
    },
    /**
     *
     * @param {string} key
     * @returns {string | null}
     */
    readParam(key) {
      const urlParams = new URLSearchParams(window.location.search);
      return urlParams.get(key);
    },
    pathnameToSelectedId() {
      return window.document.location.pathname.substring(1);
    },
  },
  locale: {
    /**
     * @param {number} value
     * @param {number} [digits]
     * @param {Intl.NumberFormatOptions} [options]
     */
    numberToUSFormat(value, digits, options) {
      return value.toLocaleString("en-us", {
        ...options,
        minimumFractionDigits: digits,
        maximumFractionDigits: digits,
      });
    },
    /** @param {number} value  */
    numberToShortUSFormat(value) {
      const absoluteValue = Math.abs(value);

      if (isNaN(value)) {
        return "";
      } else if (absoluteValue < 10) {
        return utils.locale.numberToUSFormat(value, 3);
      } else if (absoluteValue < 100) {
        return utils.locale.numberToUSFormat(value, 2);
      } else if (absoluteValue < 1_000) {
        return utils.locale.numberToUSFormat(value, 1);
      } else if (absoluteValue < 100_000) {
        return utils.locale.numberToUSFormat(value, 0);
      } else if (absoluteValue < 1_000_000) {
        return `${utils.locale.numberToUSFormat(value / 1_000, 1)}K`;
      } else if (absoluteValue >= 9_000_000_000_000_000) {
        return "Inf.";
      }

      const log = Math.floor(Math.log10(absoluteValue) - 6);

      const suffices = ["M", "B", "T", "Q"];
      const letterIndex = Math.floor(log / 3);
      const letter = suffices[letterIndex];

      const modulused = log % 3;

      if (modulused === 0) {
        return `${utils.locale.numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          3,
        )}${letter}`;
      } else if (modulused === 1) {
        return `${utils.locale.numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          2,
        )}${letter}`;
      } else {
        return `${utils.locale.numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          1,
        )}${letter}`;
      }
    },
  },
  storage: {
    /**
     * @param {string} key
     */
    readNumber(key) {
      const saved = this.read(key);
      if (saved) {
        return Number(saved);
      }
      return null;
    },
    /**
     * @param {string} key
     */
    readBool(key) {
      const saved = this.read(key);
      if (saved) {
        return utils.isSerializedBooleanTrue(saved);
      }
      return null;
    },
    /**
     * @param {string} key
     */
    read(key) {
      return localStorage.getItem(key);
    },
    /**
     * @param {string} key
     * @param {string | boolean | null | undefined} value
     */
    write(key, value) {
      value !== undefined && value !== null
        ? localStorage.setItem(key, String(value))
        : localStorage.removeItem(key);
    },
    /**
     * @param {string} key
     */
    remove(key) {
      this.write(key, undefined);
    },
  },
  serde: {
    number: {
      /**
       * @param {number} v
       */
      serialize(v) {
        return String(v);
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        return Number(v);
      },
    },
    date: {
      /**
       * @param {Date} v
       */
      serialize(v) {
        return utils.date.toString(v);
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        return new Date(v);
      },
    },
    boolean: {
      /**
       * @param {boolean} v
       */
      serialize(v) {
        return String(v);
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        if (v === "true") {
          return true;
        } else if (v === "false") {
          return false;
        } else {
          throw "deser bool err";
        }
      },
    },
  },
  formatters: {
    dollars: new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }),
    percentage: new Intl.NumberFormat("en-US", {
      style: "percent",
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }),
  },
  date: {
    ONE_DAY_IN_MS: 1000 * 60 * 60 * 24,
    todayUTC() {
      const today = new Date();
      return new Date(
        Date.UTC(
          today.getUTCFullYear(),
          today.getUTCMonth(),
          today.getUTCDate(),
          0,
          0,
          0,
        ),
      );
    },
    /**
     * @param {Date} date
     * @returns {string}
     */
    toString(date) {
      return date.toJSON().split("T")[0];
    },
    /**
     * @param {Time} time
     */
    fromTime(time) {
      return typeof time === "string"
        ? new Date(time)
        : // @ts-ignore
          new Date(time.year, time.month, time.day);
    },
    /**
     * @param {Date} start
     */
    getRangeUpToToday(start) {
      return this.getRange(start, new Date());
    },
    /**
     * @param {Date} start
     * @param {Date} end
     */
    getRange(start, end) {
      const dates = /** @type {Date[]} */ ([]);
      let currentDate = new Date(start);
      while (currentDate <= end) {
        dates.push(new Date(currentDate));
        currentDate.setUTCDate(currentDate.getUTCDate() + 1);
      }
      return dates;
    },
    /**
     * @param {Date} date1
     * @param {Date} date2
     * @returns
     */
    differenceBetween(date1, date2) {
      return Math.abs(date1.valueOf() - date2.valueOf()) / this.ONE_DAY_IN_MS;
    },
  },
  color: {
    /**
     *
     * @param {readonly [number, number, number, number, number, number, number, number, number]} A
     * @param {readonly [number, number, number]} B
     * @returns
     */
    multiplyMatrices(A, B) {
      return /** @type {const} */ ([
        A[0] * B[0] + A[1] * B[1] + A[2] * B[2],
        A[3] * B[0] + A[4] * B[1] + A[5] * B[2],
        A[6] * B[0] + A[7] * B[1] + A[8] * B[2],
      ]);
    },
    /**
     * @param {readonly [number, number, number]} param0
     */
    oklch2oklab([l, c, h]) {
      return /** @type {const} */ ([
        l,
        isNaN(h) ? 0 : c * Math.cos((h * Math.PI) / 180),
        isNaN(h) ? 0 : c * Math.sin((h * Math.PI) / 180),
      ]);
    },
    /**
     * @param {readonly [number, number, number]} rgb
     */
    srgbLinear2rgb(rgb) {
      return rgb.map((c) =>
        Math.abs(c) > 0.0031308
          ? (c < 0 ? -1 : 1) * (1.055 * Math.abs(c) ** (1 / 2.4) - 0.055)
          : 12.92 * c,
      );
    },
    /**
     * @param {readonly [number, number, number]} lab
     */
    oklab2xyz(lab) {
      const LMSg = this.multiplyMatrices(
        /** @type {const} */ ([
          1, 0.3963377773761749, 0.2158037573099136, 1, -0.1055613458156586,
          -0.0638541728258133, 1, -0.0894841775298119, -1.2914855480194092,
        ]),
        lab,
      );
      const LMS = /** @type {[number, number, number]} */ (
        LMSg.map((val) => val ** 3)
      );
      return this.multiplyMatrices(
        /** @type {const} */ ([
          1.2268798758459243, -0.5578149944602171, 0.2813910456659647,
          -0.0405757452148008, 1.112286803280317, -0.0717110580655164,
          -0.0763729366746601, -0.4214933324022432, 1.5869240198367816,
        ]),
        LMS,
      );
    },
    /**
     * @param {readonly [number, number, number]} xyz
     */
    xyz2rgbLinear(xyz) {
      return this.multiplyMatrices(
        [
          3.2409699419045226, -1.537383177570094, -0.4986107602930034,
          -0.9692436362808796, 1.8759675015077202, 0.04155505740717559,
          0.05563007969699366, -0.20397695888897652, 1.0569715142428786,
        ],
        xyz,
      );
    },
    /** @param {string} oklch */
    oklch2hex(oklch) {
      oklch = oklch.replace("oklch(", "");
      oklch = oklch.replace(")", "");
      const lch = oklch.split(" ").map((v, i) => {
        if (!i && v.includes("%")) {
          return Number(v.replace("%", "")) / 100;
        } else {
          return Number(v);
        }
      });
      const [r, g, b] = this.srgbLinear2rgb(
        this.xyz2rgbLinear(
          this.oklab2xyz(
            this.oklch2oklab(/** @type {[number, number, number]} */ (lch)),
          ),
        ),
      ).map((v) => {
        v = Math.max(Math.min(Math.round(v * 255), 255), 0);
        const s = v.toString(16);
        return s.length === 1 ? `0${s}` : s;
      });
      return `#${r}${g}${b}`;
    },
  },
  /**
   *
   * @template {(...args: any[]) => any} F
   * @param {F} callback
   * @param {number} [wait=250]
   */
  debounce(callback, wait = 250) {
    /** @type {number | undefined} */
    let timeoutId;
    /** @type {Parameters<F>} */
    let latestArgs;

    return (/** @type {Parameters<F>} */ ...args) => {
      latestArgs = args;

      if (!timeoutId) {
        timeoutId = window.setTimeout(async () => {
          await callback(...latestArgs);

          timeoutId = undefined;
        }, wait);
      }
    };
  },
  /**
   * @param {VoidFunction} callback
   * @param {number} [timeout = 1]
   */
  runWhenIdle(callback, timeout = 1) {
    if ("requestIdleCallback" in window) {
      requestIdleCallback(callback);
    } else {
      setTimeout(callback, timeout);
    }
  },
  /**
   * @param {Date} oldest
   * @param {Date} youngest
   * @returns {number}
   */
  getNumberOfDaysBetweenTwoDates(oldest, youngest) {
    return Math.round(
      Math.abs((youngest.getTime() - oldest.getTime()) / consts.ONE_DAY_IN_MS),
    );
  },
  /**
   * @param {TimeScale} scale
   * @param {number} id
   */
  chunkIdToIndex(scale, id) {
    return scale === "date"
      ? id - 2009
      : Math.floor(id / consts.HEIGHT_CHUNK_SIZE);
  },
  /**
   * @param {string} s
   */
  stringToId(s) {
    return s.replace(/\W/g, " ").trim().replace(/ +/g, "-").toLowerCase();
  },
};
/** @typedef {typeof utils} Utilities */

function initEnv() {
  const standalone =
    "standalone" in window.navigator && !!window.navigator.standalone;
  const userAgent = navigator.userAgent.toLowerCase();
  const isChrome = userAgent.includes("chrome");
  const safari = userAgent.includes("safari");
  const safariOnly = safari && !isChrome;
  const macOS = userAgent.includes("mac os");
  const iphone = userAgent.includes("iphone");
  const ipad = userAgent.includes("ipad");
  const ios = iphone || ipad;

  return {
    standalone,
    userAgent,
    isChrome,
    safari,
    safariOnly,
    macOS,
    iphone,
    ipad,
    ios,
    localhost: window.location.hostname === "localhost",
  };
}
const env = initEnv();
/** @typedef {typeof env} Env */

function createConstants() {
  const ONE_SECOND_IN_MS = 1_000;
  const FIVE_SECONDS_IN_MS = 5 * ONE_SECOND_IN_MS;
  const TEN_SECONDS_IN_MS = 2 * FIVE_SECONDS_IN_MS;
  const ONE_MINUTE_IN_MS = 6 * TEN_SECONDS_IN_MS;
  const FIVE_MINUTES_IN_MS = 5 * ONE_MINUTE_IN_MS;
  const TEN_MINUTES_IN_MS = 2 * FIVE_MINUTES_IN_MS;
  const ONE_HOUR_IN_MS = 6 * TEN_MINUTES_IN_MS;
  const ONE_DAY_IN_MS = 24 * ONE_HOUR_IN_MS;

  const HEIGHT_CHUNK_SIZE = 10_000;

  const MEDIUM_WIDTH = 768;

  return {
    ONE_SECOND_IN_MS,
    FIVE_SECONDS_IN_MS,
    TEN_SECONDS_IN_MS,
    ONE_MINUTE_IN_MS,
    FIVE_MINUTES_IN_MS,
    TEN_MINUTES_IN_MS,
    ONE_HOUR_IN_MS,
    ONE_DAY_IN_MS,

    HEIGHT_CHUNK_SIZE,

    MEDIUM_WIDTH,
  };
}
const consts = createConstants();
/** @typedef {typeof consts} Consts */

const ids = /** @type {const} */ ({
  selectedId: `selected-id`,
  asideSelectorLabel: `aside-selector-label`,
  checkedFrameSelectorLabel: "checked-frame-selector-label",
});
/** @typedef {typeof ids} Ids */

const elements = {
  head: window.document.getElementsByTagName("head")[0],
  body: window.document.body,
  main: utils.dom.getElementById("main"),
  aside: utils.dom.getElementById("aside"),
  asideLabel: utils.dom.getElementById(ids.asideSelectorLabel),
  navLabel: utils.dom.getElementById(`nav-selector-label`),
  searchLabel: utils.dom.getElementById(`search-selector-label`),
  search: utils.dom.getElementById("search"),
  nav: utils.dom.getElementById("nav"),
  navHeader: utils.dom.getElementById("nav-header"),
  searchInput: /** @type {HTMLInputElement} */ (
    utils.dom.getElementById("search-input")
  ),
  searchSmall: utils.dom.getElementById("search-small"),
  searchResults: utils.dom.getElementById("search-results"),
  selectors: utils.dom.getElementById("frame-selectors"),
  style: getComputedStyle(window.document.documentElement),
  charts: utils.dom.getElementById("charts"),
  simulation: utils.dom.getElementById("simulation"),
  livePrice: utils.dom.getElementById("live-price"),
  moscowTime: utils.dom.getElementById("moscow-time"),
};
/** @typedef {typeof elements} Elements */

const urlSelected = utils.url.pathnameToSelectedId();

function initFrameSelectors() {
  const children = Array.from(elements.selectors.children);

  /** @type {HTMLElement | undefined} */
  let focusedFrame = undefined;

  for (let i = 0; i < children.length; i++) {
    const element = children[i];

    switch (element.tagName) {
      case "LABEL": {
        element.addEventListener("click", () => {
          const inputId = element.getAttribute("for");

          if (!inputId) {
            console.log(element, element.getAttribute("for"));
            throw "Input id in label not found";
          }

          const input = window.document.getElementById(inputId);

          if (!input || !("value" in input)) {
            throw "Not input or no value";
          }

          const frame = window.document.getElementById(
            /** @type {string} */ (input.value),
          );

          if (!frame) {
            console.log(input.value);
            throw "Frame element doesn't exist";
          }

          if (frame === focusedFrame) {
            return;
          }

          frame.hidden = false;
          if (focusedFrame) {
            focusedFrame.hidden = true;
          }
          focusedFrame = frame;
        });
        break;
      }
    }
  }

  elements.asideLabel.click();

  // When going from mobile view to desktop view, if selected frame was open, go to the nav frame
  new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (
        !entries[i].isIntersecting &&
        entries[i].target === elements.asideLabel &&
        focusedFrame == elements.aside
      ) {
        elements.navLabel.click();
      }
    }
  }).observe(elements.asideLabel);

  function setAsideParent() {
    const { clientWidth } = window.document.documentElement;
    if (clientWidth >= consts.MEDIUM_WIDTH) {
      elements.body.append(elements.aside);
    } else {
      elements.main.append(elements.aside);
    }
  }

  setAsideParent();

  window.addEventListener("resize", setAsideParent);
}
initFrameSelectors();

function createKeyDownEventListener() {
  window.document.addEventListener("keydown", (event) => {
    switch (event.key) {
      case "Escape": {
        event.stopPropagation();
        event.preventDefault();
        elements.navLabel.click();
        break;
      }
      case "/": {
        if (window.document.activeElement === elements.searchInput) {
          return;
        }

        event.stopPropagation();
        event.preventDefault();
        elements.searchLabel.click();
        elements.searchInput.focus();
        break;
      }
    }
  });
}
createKeyDownEventListener();

/**
 * @param {Accessor<boolean>} dark
 */
function createColors(dark) {
  /**
   * @param {string} color
   */
  function getColor(color) {
    return utils.color.oklch2hex(elements.style.getPropertyValue(`--${color}`));
  }
  function red() {
    return getColor("red");
  }
  function orange() {
    return getColor("orange");
  }
  function amber() {
    return getColor("amber");
  }
  function yellow() {
    return getColor("yellow");
  }
  function avocado() {
    return getColor("avocado");
  }
  function lime() {
    return getColor("lime");
  }
  function green() {
    return getColor("green");
  }
  function emerald() {
    return getColor("emerald");
  }
  function teal() {
    return getColor("teal");
  }
  function cyan() {
    return getColor("cyan");
  }
  function sky() {
    return getColor("sky");
  }
  function blue() {
    return getColor("blue");
  }
  function indigo() {
    return getColor("indigo");
  }
  function violet() {
    return getColor("violet");
  }
  function purple() {
    return getColor("purple");
  }
  function fuchsia() {
    return getColor("fuchsia");
  }
  function pink() {
    return getColor("pink");
  }
  function rose() {
    return getColor("rose");
  }

  /**
   * @param {string} property
   */
  function getLightDarkValue(property) {
    const value = elements.style.getPropertyValue(property);
    const [light, _dark] = value.slice(11, -1).split(", ");
    return dark() ? _dark : light;
  }

  function off() {
    return getLightDarkValue("--off-color");
  }

  function textColor() {
    return getLightDarkValue("--color");
  }

  return {
    default: textColor,
    off,
    lightBitcoin: yellow,
    bitcoin: orange,
    offBitcoin: red,
    lightDollars: lime,
    dollars: green,
    offDollars: emerald,

    yellow,
    lime,
    orange,
    red,
    sky,
    blue,
    rose,
    green,
    amber,
    avocado,
    cyan,
    violet,
    purple,
    fuchsia,
    pink,

    _1d: pink,
    _1w: red,
    _8d: orange,
    _13d: amber,
    _21d: yellow,
    _1m: lime,
    _34d: green,
    _55d: emerald,
    _89d: teal,
    _144d: cyan,
    _6m: sky,
    _1y: blue,
    _2y: indigo,
    _200w: violet,
    _4y: purple,
    _10y: fuchsia,

    p2pk: lime,
    p2pkh: violet,
    p2sh: emerald,
    p2wpkh: cyan,
    p2wsh: pink,
    p2tr: blue,
    crab: red,
    fish: lime,
    humpback: violet,
    plankton: emerald,
    shark: cyan,
    shrimp: pink,
    whale: blue,
    megalodon: purple,
    realizedPrice: orange,
    oneMonthHolders: cyan,
    threeMonthsHolders: lime,
    sth: yellow,
    sixMonthsHolder: red,
    oneYearHolders: pink,
    twoYearsHolders: purple,
    lth: fuchsia,
    balancedPrice: yellow,
    cointimePrice: yellow,
    trueMarketMeanPrice: blue,
    vaultedPrice: green,
    cvdd: lime,
    terminalPrice: red,
    loss: red,
    profit: green,
    thermoCap: green,
    investorCap: rose,
    realizedCap: orange,
    offLiveliness: red,
    liveliness: rose,
    vaultedness: green,
    activityToVaultednessRatio: violet,
    up_to_1d: pink,
    up_to_1w: red,
    up_to_1m: orange,
    up_to_2m: amber,
    up_to_3m: yellow,
    up_to_4m: lime,
    up_to_5m: green,
    up_to_6m: teal,
    up_to_1y: sky,
    up_to_2y: indigo,
    up_to_3y: violet,
    up_to_4y: purple,
    up_to_5y: red,
    up_to_7y: orange,
    up_to_10y: amber,
    up_to_15y: yellow,
    from_10y_to_15y: purple,
    from_7y_to_10y: violet,
    from_5y_to_7y: indigo,
    from_3y_to_5y: sky,
    from_2y_to_3y: teal,
    from_1y_to_2y: green,
    from_6m_to_1y: lime,
    from_3m_to_6m: yellow,
    from_1m_to_3m: amber,
    from_1w_to_1m: orange,
    from_1d_to_1w: red,
    from_1y: green,
    from_2y: teal,
    from_4y: indigo,
    from_10y: violet,
    from_15y: fuchsia,
    coinblocksCreated: purple,
    coinblocksDestroyed: red,
    coinblocksStored: green,
    momentumGreen: green,
    momentumYellow: yellow,
    momentumRed: red,
    probability0_1p: red,
    probability0_5p: orange,
    probability1p: yellow,
    epoch_1: red,
    epoch_2: orange,
    epoch_3: yellow,
    epoch_4: green,
    epoch_5: blue,
    highly_liquid: red,
    liquid: lime,
    illiquid: cyan,
  };
}
/**
 * @typedef {ReturnType<typeof createColors>} Colors
 */

/**
 * @param {Signals} signals
 */
function createDatasets(signals) {
  /** @type {Map<DatePath, ResourceDataset<"date">>} */
  const date = new Map();
  /** @type {Map<HeightPath, ResourceDataset<"height">>} */
  const height = new Map();

  const URL = "/api";
  const BACKUP_URL = "https://backup.kibo.money/api";

  const datasetsOwner = signals.getOwner();

  /**
   * @template {TimeScale} S
   * @template {number | OHLC} [T=number]
   * @param {S} scale
   * @param {string} path
   */
  function createResourceDataset(scale, path) {
    return /** @type {ResourceDataset<S, T>} */ (
      signals.runWithOwner(datasetsOwner, () => {
        /** @typedef {DatasetValue<T extends number ? SingleValueData : CandlestickData>} Value */

        const baseURL = `${URL}/${path}`;

        const backupURL = `${BACKUP_URL}/${path}`;

        const fetchedJSONs = new Array(
          (new Date().getFullYear() -
            new Date("2009-01-01").getFullYear() +
            2) *
            (scale === "date" ? 1 : 6),
        )
          .fill(null)
          .map(() => {
            const json = signals.createSignal(
              /** @type {FetchedJSON<S, T> | null} */ (null),
            );

            /** @type {FetchedResult<S, T>} */
            const fetchedResult = {
              at: null,
              json,
              loading: false,
              vec: signals.createMemo(() => {
                const map = json()?.dataset.map;

                if (!map) {
                  return null;
                }

                const chunkId = json()?.chunk.id;

                if (chunkId === undefined) {
                  throw `ChunkId ${chunkId} is undefined`;
                }

                if (Array.isArray(map)) {
                  const values = new Array(map.length);

                  for (let i = 0; i < map.length; i++) {
                    const value = map[i];

                    values[i] = /** @type {Value} */ ({
                      time: /** @type {Time} */ (chunkId + i),
                      ...(typeof value !== "number" && value !== null
                        ? {
                            .../** @type {OHLC} */ (value),
                            value: value.close,
                          }
                        : {
                            value:
                              value === null
                                ? NaN
                                : /** @type {number} */ (value),
                          }),
                    });
                  }

                  return values;
                } else {
                  return Object.entries(map).map(
                    ([date, value]) =>
                      /** @type {Value} */ ({
                        time: date,
                        ...(typeof value !== "number" && value !== null
                          ? {
                              .../** @type {OHLC} */ (value),
                              value: value.close,
                            }
                          : {
                              value:
                                value === null
                                  ? NaN
                                  : /** @type {number} */ (value),
                            }),
                      }),
                  );
                }
              }),
            };

            return fetchedResult;
          });

        /**
         * @param {number} id
         */
        async function _fetch(id) {
          const index = utils.chunkIdToIndex(scale, id);

          if (
            index < 0 ||
            (scale === "date" && id > new Date().getUTCFullYear()) ||
            (scale === "height" &&
              id > 165 * 365 * (new Date().getUTCFullYear() - 2009))
          ) {
            return;
          }

          const fetched = fetchedJSONs.at(index);

          if (scale === "height" && index > 0) {
            const length = fetchedJSONs.at(index - 1)?.vec()?.length;

            if (length !== undefined && length < consts.HEIGHT_CHUNK_SIZE) {
              return;
            }
          }

          if (!fetched || fetched.loading) {
            return;
          } else if (fetched.at) {
            const diff = new Date().getTime() - fetched.at.getTime();

            if (
              diff < consts.ONE_MINUTE_IN_MS ||
              (index < fetchedJSONs.findLastIndex((json) => json.at) &&
                diff < consts.ONE_HOUR_IN_MS)
            ) {
              return;
            }
          }

          fetched.loading = true;

          /** @type {Cache | undefined} */
          let cache;

          const urlWithQuery = `${baseURL}?chunk=${id}`;
          const backupUrlWithQuery = `${backupURL}?chunk=${id}`;

          if (!fetched.json()) {
            try {
              cache = await caches.open("resources");

              const cachedResponse = await cache.match(urlWithQuery);

              if (cachedResponse) {
                /** @type {FetchedJSON<S, T> | null} */
                const json = await convertResponseToJSON(cachedResponse);

                if (json) {
                  console.log(`cache: ${path}?chunk=${id}`);

                  fetched.json.set(() => json);
                }
              }
            } catch {}
          }

          if (!navigator.onLine) {
            fetched.loading = false;
            return;
          }

          /** @type {Response | undefined} */
          let fetchedResponse;

          /** @type {RequestInit} */
          const fetchConfig = {
            signal: AbortSignal.timeout(5000),
          };

          try {
            fetchedResponse = await fetch(urlWithQuery, fetchConfig);

            if (!fetchedResponse.ok) {
              throw Error;
            }
          } catch {
            try {
              fetchedResponse = await fetch(backupUrlWithQuery, fetchConfig);
            } catch {
              fetched.loading = false;
              return;
            }

            if (!fetchedResponse || !fetchedResponse.ok) {
              fetched.loading = false;
              return;
            }
          }

          const clonedResponse = fetchedResponse.clone();

          /** @type {FetchedJSON<S, T> | null} */
          const json = await convertResponseToJSON(fetchedResponse);

          if (!json) {
            fetched.loading = false;
            return;
          }

          console.log(`fetch: ${path}?chunk=${id}`);

          const previousMap = fetched.json()?.dataset;
          const newMap = json.dataset.map;

          const previousLength = Object.keys(previousMap || []).length;
          const newLength = Object.keys(newMap).length;

          if (!newLength) {
            fetched.loading = false;
            return;
          }

          if (previousLength && previousLength === newLength) {
            const previousLastValue = Object.values(previousMap || []).at(-1);
            const newLastValue = Object.values(newMap).at(-1);

            if (newLastValue === null && previousLastValue === null) {
              fetched.at = new Date();
              fetched.loading = false;
              return;
            } else if (typeof newLastValue === "number") {
              if (previousLastValue === newLastValue) {
                fetched.at = new Date();
                fetched.loading = false;
                return;
              }
            } else {
              const previousLastOHLC = /** @type {OHLC} */ (previousLastValue);
              const newLastOHLC = /** @type {OHLC} */ (newLastValue);

              if (
                previousLastOHLC.open === newLastOHLC.open &&
                previousLastOHLC.high === newLastOHLC.high &&
                previousLastOHLC.low === newLastOHLC.low &&
                previousLastOHLC.close === newLastOHLC.close
              ) {
                fetched.loading = false;
                fetched.at = new Date();
                return;
              }
            }
          }

          fetched.json.set(() => json);

          utils.runWhenIdle(async function () {
            try {
              await cache?.put(urlWithQuery, clonedResponse);
            } catch (_) {}
          });

          fetched.at = new Date();
          fetched.loading = false;
        }

        /** @type {ResourceDataset<S, T>} */
        const resource = {
          scale,
          url: baseURL,
          fetch: _fetch,
          fetchRange(start, end) {
            const promises = /** @type {Promise<void>[]} */ ([]);
            switch (scale) {
              case "date": {
                utils.array.range(start, end).forEach((year) => {
                  promises.push(this.fetch(year));
                });
                break;
              }
              default: {
                throw "Unsupported";
              }
            }
            return Promise.all(promises);
          },
          fetchedJSONs,
          // drop() {
          //   dispose();
          //   fetchedJSONs.forEach((fetched) => {
          //     fetched.at = null;
          //     fetched.json.set(null);
          //   });
          // },
        };

        return resource;
      })
    );
  }

  /**
   * @template {TimeScale} S
   * @template {number | OHLC} T
   * @param {Response} response
   */
  async function convertResponseToJSON(response) {
    try {
      return /** @type {FetchedJSON<S, T>} */ (await response.json());
    } catch (_) {
      return null;
    }
  }

  /**
   * @template {TimeScale} S
   * @param {S} scale
   * @param {DatasetPath<S>} path
   * @returns {ResourceDataset<S>}
   */
  function getOrCreate(scale, path) {
    if (scale === "date") {
      const found = date.get(/** @type {DatePath} */ (path));
      if (found) return /** @type {ResourceDataset<S>} */ (found);
    } else {
      const found = height.get(/** @type {HeightPath} */ (path));
      if (found) return /** @type {ResourceDataset<S>} */ (found);
    }

    /** @type {ResourceDataset<S, any>} */
    let dataset;

    if (path === `/${scale}-to-price`) {
      /** @type {ResourceDataset<S, OHLC>} */
      dataset = createResourceDataset(scale, path);
    } else {
      /** @type {ResourceDataset<S, number>} */
      dataset = createResourceDataset(scale, path);
    }

    if (scale === "date") {
      date.set(/** @type {DatePath} */ (path), /** @type {any} */ (dataset));
    } else {
      height.set(
        /** @type {HeightPath} */ (path),
        /** @type {any} */ (dataset),
      );
    }

    return dataset;
  }

  return {
    getOrCreate,
  };
}
/** @typedef {ReturnType<typeof createDatasets>} Datasets */

/**
 * @param {Signals} signals
 */
function initWebSockets(signals) {
  /**
   * @template T
   * @param {(callback: (value: T) => void) => WebSocket} creator
   */
  function createWebsocket(creator) {
    let ws = /** @type {WebSocket | null} */ (null);

    const live = signals.createSignal(false);
    const latest = signals.createSignal(/** @type {T | null} */ (null));

    function reinitWebSocket() {
      if (!ws || ws.readyState === ws.CLOSED) {
        console.log("ws: reinit");
        resource.open();
      }
    }

    function reinitWebSocketIfDocumentNotHidden() {
      !window.document.hidden && reinitWebSocket();
    }

    const resource = {
      live,
      latest,
      open() {
        ws = creator((value) => latest.set(() => value));

        ws.addEventListener("open", () => {
          console.log("ws: open");
          live.set(true);
        });

        ws.addEventListener("close", () => {
          console.log("ws: close");
          live.set(false);
        });

        window.document.addEventListener(
          "visibilitychange",
          reinitWebSocketIfDocumentNotHidden,
        );

        window.document.addEventListener("online", reinitWebSocket);
      },
      close() {
        ws?.close();
        window.document.removeEventListener(
          "visibilitychange",
          reinitWebSocketIfDocumentNotHidden,
        );
        window.document.removeEventListener("online", reinitWebSocket);
        live.set(false);
        ws = null;
      },
    };

    return resource;
  }

  /**
   * @param {(candle: DatasetCandlestickData) => void} callback
   * @param {number} interval
   */
  function krakenCandleWebSocketCreator(callback, interval) {
    const ws = new WebSocket("wss://ws.kraken.com/v2");

    ws.addEventListener("open", () => {
      ws.send(
        JSON.stringify({
          method: "subscribe",
          params: {
            channel: "ohlc",
            symbol: ["BTC/USD"],
            interval: 1440,
          },
        }),
      );
    });

    ws.addEventListener("message", (message) => {
      const result = JSON.parse(message.data);

      if (result.channel !== "ohlc") return;

      const { interval_begin, open, high, low, close } = result.data.at(-1);

      const date = new Date(interval_begin);

      const dateStr = utils.date.toString(date);

      /** @type {DatasetCandlestickData} */
      const candle = {
        time: dateStr,
        year: date.getUTCFullYear(),
        open: Number(open),
        high: Number(high),
        low: Number(low),
        close: Number(close),
        value: Number(close),
      };

      candle && callback({ ...candle });
    });

    return ws;
  }

  const kraken1dCandle = createWebsocket((callback) =>
    krakenCandleWebSocketCreator(callback, 1440),
  );
  const kraken5mnCandle = createWebsocket((callback) =>
    krakenCandleWebSocketCreator(callback, 5),
  );

  kraken1dCandle.open();
  kraken5mnCandle.open();

  function createDocumentTitleEffect() {
    signals.createEffect(kraken5mnCandle.latest, (latest) => {
      if (latest) {
        const close = latest.close;
        console.log("close:", close);

        window.document.title = `${latest.close.toLocaleString(
          "en-us",
        )} | kibō`;
      }
    });
  }
  createDocumentTitleEffect();

  return {
    kraken1dCandle,
    // kraken5mnCandle,
  };
}
/** @typedef {ReturnType<typeof initWebSockets>} WebSockets */

packages.signals().then((signals) =>
  options.then(({ initOptions }) => {
    function initDark() {
      const preferredColorSchemeMatchMedia = window.matchMedia(
        "(prefers-color-scheme: dark)",
      );
      const dark = signals.createSignal(preferredColorSchemeMatchMedia.matches);
      preferredColorSchemeMatchMedia.addEventListener(
        "change",
        ({ matches }) => {
          dark.set(matches);
        },
      );
      return dark;
    }
    const dark = initDark();

    const qrcode = signals.createSignal(/** @type {string | null} */ (null));

    function createLastHeightResource() {
      const lastHeight = signals.createSignal(0);

      function fetchLastHeight() {
        fetch("/api/last-height").then((response) => {
          response.json().then((json) => {
            if (typeof json === "number") {
              lastHeight.set(json);
            }
          });
        });
      }
      fetchLastHeight();
      setInterval(fetchLastHeight, consts.TEN_SECONDS_IN_MS, {});

      return lastHeight;
    }
    const lastHeight = createLastHeightResource();

    const lastValues = signals.createSignal(/** @type {LastValues} */ (null));

    function createFetchLastValuesWhenNeededEffect() {
      let previousHeight = -1;
      signals.createEffect(lastHeight, (lastHeight) => {
        if (previousHeight !== lastHeight) {
          fetch("/api/last").then((response) => {
            response.json().then((json) => {
              if (typeof json === "object") {
                lastValues.set(json);
                previousHeight = lastHeight;
              }
            });
          });
        }
      });
    }
    createFetchLastValuesWhenNeededEffect();

    const webSockets = initWebSockets(signals);

    const colors = createColors(dark);

    const options = initOptions({
      colors,
      env,
      ids,
      lastValues,
      signals,
      utils,
      webSockets,
      qrcode,
    });

    function createWindowPopStateEvent() {
      window.addEventListener("popstate", (event) => {
        const urlSelected = utils.url.pathnameToSelectedId();
        const option = options.list.find((option) => urlSelected === option.id);
        if (option) {
          options.selected.set(option);
        }
      });
    }
    // createWindowPopStateEvent();

    function initSelected() {
      function initSelectedFrame() {
        console.log("selected: init");

        const datasets = createDatasets(signals);

        function createApplyOptionEffect() {
          const lastChartOption = signals.createSignal(
            /** @type {ChartOption | null} */ (null),
          );
          const lastSimulationOption = signals.createSignal(
            /** @type {SimulationOption | null} */ (null),
          );

          const owner = signals.getOwner();

          let previousElement = /** @type {HTMLElement | undefined} */ (
            undefined
          );
          let firstChartOption = true;
          let firstSimulationOption = true;
          let firstLivePriceOption = true;
          let firstMoscowTimeOption = true;

          signals.createEffect(options.selected, (option) => {
            if (previousElement) {
              previousElement.hidden = true;
              utils.url.resetParams(option);
              utils.url.pushHistory(option.id);
            } else {
              utils.url.replaceHistory({ pathname: option.id });
            }

            /** @type {HTMLElement} */
            let element;

            switch (option.kind) {
              // case "home": {
              //   element = elements.home;
              //   break;
              // }
              case "chart": {
                console.log("chart", option);

                element = elements.charts;

                lastChartOption.set(option);

                if (firstChartOption) {
                  const lightweightCharts = packages.lightweightCharts();
                  const chartScript = import("./chart.js");
                  utils.dom.importStyleAndThen("/styles/chart.css", () =>
                    chartScript.then(({ init: initChartsElement }) =>
                      lightweightCharts.then((lightweightCharts) =>
                        signals.runWithOwner(owner, () =>
                          initChartsElement({
                            colors,
                            datasets,
                            elements,
                            lightweightCharts,
                            selected: /** @type {any} */ (lastChartOption),
                            signals,
                            utils,
                            webSockets,
                          }),
                        ),
                      ),
                    ),
                  );
                }
                firstChartOption = false;

                break;
              }
              case "simulation": {
                element = elements.simulation;

                lastSimulationOption.set(option);

                if (firstSimulationOption) {
                  const lightweightCharts = packages.lightweightCharts();
                  const simulationScript = import("./simulation.js");

                  utils.dom.importStyleAndThen("/styles/simulation.css", () =>
                    simulationScript.then(({ init }) =>
                      lightweightCharts.then((lightweightCharts) =>
                        signals.runWithOwner(owner, () =>
                          init({
                            colors,
                            datasets,
                            elements,
                            lightweightCharts,
                            signals,
                            utils,
                            lastValues,
                          }),
                        ),
                      ),
                    ),
                  );
                }
                firstSimulationOption = false;

                break;
              }
              case "live-price": {
                console.log("live-price");

                element = elements.livePrice;

                if (firstLivePriceOption) {
                  const lightweightCharts = packages.lightweightCharts();
                  const script = import("./live-price.js");

                  utils.dom.importStyleAndThen("/styles/live-price.css", () =>
                    script.then(({ init }) =>
                      lightweightCharts.then((lightweightCharts) =>
                        signals.runWithOwner(owner, () =>
                          init({
                            colors,
                            consts,
                            dark,
                            datasets,
                            elements,
                            ids,
                            lightweightCharts,
                            options,
                            signals,
                            utils,
                            webSockets,
                          }),
                        ),
                      ),
                    ),
                  );
                }
                firstLivePriceOption = false;

                break;
              }
              case "moscow-time": {
                console.log("moscow-time");

                element = elements.moscowTime;

                if (firstLivePriceOption) {
                  const lightweightCharts = packages.lightweightCharts();
                  const script = import("./moscow-time.js");

                  utils.dom.importStyleAndThen("/styles/moscow-time.css", () =>
                    script.then(({ init }) =>
                      signals.runWithOwner(owner, () =>
                        init({
                          colors,
                          consts,
                          dark,
                          datasets,
                          elements,
                          ids,
                          options,
                          signals,
                          utils,
                          webSockets,
                        }),
                      ),
                    ),
                  );
                }
                firstLivePriceOption = false;

                break;
              }
              case "converter":
              case "home":
              case "pdf":
              case "url": {
                return;
              }
            }

            element.hidden = false;
            previousElement = element;
          });
        }
        createApplyOptionEffect();
      }

      function createMobileSwitchEffect() {
        let firstRun = true;
        signals.createEffect(options.selected, () => {
          if (!firstRun && !utils.dom.isHidden(elements.asideLabel)) {
            elements.asideLabel.click();
          }
          firstRun = false;
        });
      }
      createMobileSwitchEffect();

      utils.dom.onFirstIntersection(elements.aside, initSelectedFrame);
    }
    initSelected();

    function initShare() {
      const shareDiv = utils.dom.getElementById("share-div");
      const shareContentDiv = utils.dom.getElementById("share-content-div");

      shareDiv.addEventListener("click", () => {
        qrcode.set(null);
      });

      shareContentDiv.addEventListener("click", (event) => {
        event.stopPropagation();
        event.preventDefault();
      });

      packages.leanQr().then(({ generate }) => {
        const imgQrcode = /** @type {HTMLImageElement} */ (
          utils.dom.getElementById("share-img")
        );

        const anchor = /** @type {HTMLAnchorElement} */ (
          utils.dom.getElementById("share-anchor")
        );

        signals.createEffect(qrcode, (qrcode) => {
          if (!qrcode) {
            shareDiv.hidden = true;
            return;
          }

          const href = qrcode;
          anchor.href = href;
          anchor.innerText =
            (href.startsWith("http")
              ? href.split("//").at(-1)
              : href.split(":").at(-1)) || "";

          imgQrcode.src =
            generate(/** @type {any} */ (href))?.toDataURL({
              // @ts-ignore
              padX: 0,
              padY: 0,
            }) || "";

          shareDiv.hidden = false;
        });
      });
    }
    initShare();

    function initFolders() {
      function initTreeElement() {
        options.treeElement.set(() => {
          const treeElement = window.document.createElement("div");
          treeElement.classList.add("tree");
          elements.navHeader.after(treeElement);
          return treeElement;
        });
      }

      async function scrollToSelected() {
        if (!options.selected()) throw "Selected should be set by now";
        const selectedId = options.selected().id;

        const path = options.selected().path;

        let i = 0;
        while (i !== path.length) {
          try {
            const id = path[i].id;
            const details = /** @type {HTMLDetailsElement} */ (
              utils.dom.getElementById(id)
            );
            details.open = true;
            i++;
          } catch {
            await utils.yield();
          }
        }

        await utils.yield();

        utils.dom.getElementById(`${selectedId}-nav-selector`).scrollIntoView({
          behavior: "instant",
          block: "center",
        });
      }

      utils.dom.onFirstIntersection(elements.nav, () => {
        console.log("nav: init");
        initTreeElement();
        scrollToSelected();
      });
    }
    initFolders();

    function initSearch() {
      function initSearchFrame() {
        console.log("search: init");

        const haystack = options.list.map(
          (option) => `${option.title}\t${option.serializedPath}`,
        );

        const searchSmallOgInnerHTML = elements.searchSmall.innerHTML;

        const RESULTS_PER_PAGE = 100;

        packages.ufuzzy().then((ufuzzy) => {
          /**
           * @param {uFuzzy.SearchResult} searchResult
           * @param {number} pageIndex
           */
          function computeResultPage(searchResult, pageIndex) {
            /** @type {{ option: Option, path: string, title: string }[]} */
            let list = [];

            let [indexes, info, order] = searchResult || [null, null, null];

            const minIndex = pageIndex * RESULTS_PER_PAGE;

            if (indexes?.length) {
              const maxIndex = Math.min(
                (order || indexes).length - 1,
                minIndex + RESULTS_PER_PAGE - 1,
              );

              list = Array(maxIndex - minIndex + 1);

              for (let i = minIndex; i <= maxIndex; i++) {
                let index = indexes[i];

                const [title, path] = haystack[index].split("\t");

                list[i % 100] = {
                  option: options.list[index],
                  path,
                  title,
                };
              }
            }

            return list;
          }

          /** @type {uFuzzy.Options} */
          const config = {
            intraIns: Infinity,
            intraChars: `[a-z\d' ]`,
          };

          const fuzzyMultiInsert = /** @type {uFuzzy} */ (
            ufuzzy({
              intraIns: 1,
            })
          );
          const fuzzyMultiInsertFuzzier = /** @type {uFuzzy} */ (
            ufuzzy(config)
          );
          const fuzzySingleError = /** @type {uFuzzy} */ (
            ufuzzy({
              intraMode: 1,
              ...config,
            })
          );
          const fuzzySingleErrorFuzzier = /** @type {uFuzzy} */ (
            ufuzzy({
              intraMode: 1,
              ...config,
            })
          );

          /** @type {VoidFunction | undefined} */
          let dispose;

          function inputEvent() {
            signals.createRoot((_dispose) => {
              const needle = /** @type {string} */ (elements.searchInput.value);

              dispose?.();

              dispose = _dispose;

              elements.searchResults.scrollTo({
                top: 0,
              });

              if (!needle) {
                elements.searchSmall.innerHTML = searchSmallOgInnerHTML;
                elements.searchResults.innerHTML = "";
                return;
              }

              const outOfOrder = 5;
              const infoThresh = 5_000;

              let result = fuzzyMultiInsert?.search(
                haystack,
                needle,
                undefined,
                infoThresh,
              );

              if (!result?.[0]?.length || !result?.[1]) {
                result = fuzzyMultiInsert?.search(
                  haystack,
                  needle,
                  outOfOrder,
                  infoThresh,
                );
              }

              if (!result?.[0]?.length || !result?.[1]) {
                result = fuzzySingleError?.search(
                  haystack,
                  needle,
                  outOfOrder,
                  infoThresh,
                );
              }

              if (!result?.[0]?.length || !result?.[1]) {
                result = fuzzySingleErrorFuzzier?.search(
                  haystack,
                  needle,
                  outOfOrder,
                  infoThresh,
                );
              }

              if (!result?.[0]?.length || !result?.[1]) {
                result = fuzzyMultiInsertFuzzier?.search(
                  haystack,
                  needle,
                  undefined,
                  infoThresh,
                );
              }

              if (!result?.[0]?.length || !result?.[1]) {
                result = fuzzyMultiInsertFuzzier?.search(
                  haystack,
                  needle,
                  outOfOrder,
                  infoThresh,
                );
              }

              elements.searchSmall.innerHTML = `Found <strong>${
                result?.[0]?.length || 0
              }</strong> result(s)`;
              elements.searchResults.innerHTML = "";

              const list = computeResultPage(result, 0);

              list.forEach(({ option, path, title }) => {
                const li = window.document.createElement("li");
                elements.searchResults.appendChild(li);

                const element = options.createOptionElement({
                  option,
                  frame: "search",
                  name: title,
                  top: path,
                  qrcode,
                });

                if (element) {
                  li.append(element);
                }
              });
            });
          }

          if (elements.searchInput.value) {
            inputEvent();
          }

          elements.searchInput.addEventListener("input", inputEvent);
        });
      }
      utils.dom.onFirstIntersection(elements.search, initSearchFrame);
    }
    initSearch();

    function initDesktopResizeBar() {
      const resizeBar = utils.dom.getElementById("resize-bar");
      let resize = false;
      let startingWidth = 0;
      let startingClientX = 0;

      const barWidthLocalStorageKey = "bar-width";

      /**
       * @param {number | null} width
       */
      function setBarWidth(width) {
        if (typeof width === "number") {
          elements.main.style.width = `${width}px`;
          localStorage.setItem(barWidthLocalStorageKey, String(width));
        } else {
          elements.main.style.width = elements.style.getPropertyValue(
            "--default-main-width",
          );
          localStorage.removeItem(barWidthLocalStorageKey);
        }
      }

      /**
       * @param {MouseEvent} event
       */
      function mouseMoveEvent(event) {
        if (resize) {
          setBarWidth(startingWidth + (event.clientX - startingClientX));
        }
      }

      resizeBar.addEventListener("mousedown", (event) => {
        startingClientX = event.clientX;
        startingWidth = elements.main.clientWidth;
        resize = true;
        window.document.documentElement.dataset.resize = "";
        window.addEventListener("mousemove", mouseMoveEvent);
      });

      resizeBar.addEventListener("dblclick", () => {
        setBarWidth(null);
      });

      const setResizeFalse = () => {
        resize = false;
        delete window.document.documentElement.dataset.resize;
        window.removeEventListener("mousemove", mouseMoveEvent);
      };
      window.addEventListener("mouseup", setResizeFalse);
      window.addEventListener("mouseleave", setResizeFalse);
    }
    initDesktopResizeBar();
  }),
);
