// @ts-check

/**
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, Unit, AnySeriesBlueprint } from "./options"
 * @import {Valued,  SingleValueData, CandlestickData, ChartData, OHLCTuple} from "../packages/lightweight-charts/wrapper"
 * @import * as _ from "../packages/ufuzzy/v1.0.14/types"
 * @import { createChart as CreateClassicChart, LineStyleOptions, DeepPartial, ChartOptions, IChartApi, IHorzScaleBehavior, WhitespaceData, ISeriesApi, Time, LineData, LogicalRange, SeriesType, BaselineStyleOptions, SeriesOptionsCommon, BaselineData, CandlestickStyleOptions } from "../packages/lightweight-charts/v5.0.5-treeshaked/types"
 * @import { SignalOptions } from "../packages/solid-signals/v0.2.4-treeshaked/types/core/core"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup, Owner } from "../packages/solid-signals/v0.2.4-treeshaked/types/core/owner"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, Accessor, Setter, createMemo as CreateMemo } from "../packages/solid-signals/v0.2.4-treeshaked/types/signals";
 * @import {Signal, Signals} from "../packages/solid-signals/types";
 * @import {Addressindex, Dateindex, Decadeindex, Difficultyepoch, Index, Halvingepoch, Height, Monthindex, P2PK33index, P2PK65index, P2PKHindex, P2SHindex, P2TRindex, P2WPKHindex, P2WSHindex, Txindex, Txinindex, Txoutindex, VecId, Weekindex, Yearindex, VecIdToIndexes, Quarterindex} from "./vecid-to-indexes"
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
/**
 * @typedef {ReturnType<typeof initPackages>} Packages
 * @typedef {Awaited<ReturnType<Packages["lightweightCharts"]>>} LightweightCharts
 * @typedef {ReturnType<LightweightCharts['createChartElement']>} Chart
 */

function createUtils() {
  /**
   * @param {string} serialized
   * @returns {boolean}
   */
  function isSerializedBooleanTrue(serialized) {
    return serialized === "true" || serialized === "1";
  }

  /**
   * @param {number} ms
   */
  function sleep(ms) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
  }

  function next() {
    return sleep(0);
  }

  const array = {
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
  };

  const dom = {
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
     * @param {Elements} elements
     */
    queryOrCreateMetaElement(name, elements) {
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
     * @param {'radio' | 'checkbox'} args.type
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
      if (type === "radio") {
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
     * @param {Elements} elements
     * @param {boolean} [targetBlank]
     */
    open(url, elements, targetBlank) {
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
     * @param {string} href
     */
    importStyle(href) {
      const link = document.createElement("link");
      link.href = href;
      link.type = "text/css";
      link.rel = "stylesheet";
      link.media = "screen,print";
      const head = window.document.getElementsByTagName("head")[0];
      head.appendChild(link);
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
        const { label } = this.createLabeledInput({
          inputId: `${id}-${choice.toLowerCase()}`,
          inputName: id,
          inputValue,
          inputChecked: inputValue === selected,
          labelTitle: choice,
          type: "radio",
        });

        const text = window.document.createTextNode(choice);
        label.append(text);
        div.append(label);
      });

      return field;
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
      const max = date.toString(maxDate);
      input.min = min;
      input.max = max;

      let stateValue = /** @type {string | null} */ (null);

      signals.createEffect(
        () => {
          const dateSignal = signal();
          return dateSignal ? date.toString(dateSignal) : "";
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
     * @param {Object} args
     * @param {1 | 2 | 3} [args.level]
     * @param {string} [args.title]
     */
    createHeader({ title, level = 1 }) {
      const headerElement = window.document.createElement("header");

      const headingElement = window.document.createElement(`h${level}`);
      headingElement.innerHTML = title || "";
      headerElement.append(headingElement);
      headingElement.style.display = "block";

      return {
        headerElement,
        headingElement,
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
            setters[option.value] = () => signal.set(() => option);
          });
        } else {
          select.append(this.createOption(anyOption));
          setters[anyOption.value] = () => signal.set(() => anyOption);
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
  };

  const url = {
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
        return isSerializedBooleanTrue(parameter);
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
  };

  /**
   * @param {number} value
   * @param {number} [digits]
   * @param {Intl.NumberFormatOptions} [options]
   */
  function numberToUSFormat(value, digits, options) {
    return value.toLocaleString("en-us", {
      ...options,
      minimumFractionDigits: digits,
      maximumFractionDigits: digits,
    });
  }

  const locale = {
    numberToUSFormat,
    /** @param {number} value  */
    numberToShortUSFormat(value) {
      const absoluteValue = Math.abs(value);

      if (isNaN(value)) {
        return "";
      } else if (absoluteValue < 10) {
        return numberToUSFormat(value, 3);
      } else if (absoluteValue < 100) {
        return numberToUSFormat(value, 2);
      } else if (absoluteValue < 1_000) {
        return numberToUSFormat(value, 1);
      } else if (absoluteValue < 100_000) {
        return numberToUSFormat(value, 0);
      } else if (absoluteValue < 1_000_000) {
        return `${numberToUSFormat(value / 1_000, 1)}K`;
      } else if (absoluteValue >= 9_000_000_000_000_000) {
        return "Inf.";
      }

      const log = Math.floor(Math.log10(absoluteValue) - 6);

      const suffices = ["M", "B", "T", "Q"];
      const letterIndex = Math.floor(log / 3);
      const letter = suffices[letterIndex];

      const modulused = log % 3;

      if (modulused === 0) {
        return `${numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          3,
        )}${letter}`;
      } else if (modulused === 1) {
        return `${numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          2,
        )}${letter}`;
      } else {
        return `${numberToUSFormat(
          value / (1_000_000 * 1_000 ** letterIndex),
          1,
        )}${letter}`;
      }
    },
  };

  const storage = {
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
        return isSerializedBooleanTrue(saved);
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
  };

  const serde = {
    string: {
      /**
       * @param {string} v
       */
      serialize(v) {
        return v;
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        return v;
      },
    },
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
    optNumber: {
      /**
       * @param {number | null} v
       */
      serialize(v) {
        return v !== null ? String(v) : "";
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        return v ? Number(v) : null;
      },
    },
    optDate: {
      /**
       * @param {Date |  null} date
       */
      serialize(date) {
        return date !== null ? date.toString() : "";
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
  };

  const formatters = {
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
  };

  const date = {
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
     */
    toString(date) {
      return date.toJSON().split("T")[0];
    },
    /**
     * @param {Date} date
     */
    toDateIndex(date) {
      if (
        date.getUTCFullYear() === 2009 &&
        date.getUTCMonth() === 0 &&
        date.getUTCDate() === 3
      )
        return 0;
      return this.differenceBetween(date, new Date("2009-01-09"));
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
     */
    differenceBetween(date1, date2) {
      return Math.abs(date1.valueOf() - date2.valueOf()) / this.ONE_DAY_IN_MS;
    },
  };

  /**
   *
   * @template {(...args: any[]) => any} F
   * @param {F} callback
   * @param {number} [wait=250]
   */
  function debounce(callback, wait = 250) {
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
  }

  /**
   * @param {VoidFunction} callback
   * @param {number} [timeout = 1]
   */
  function runWhenIdle(callback, timeout = 1) {
    if ("requestIdleCallback" in window) {
      requestIdleCallback(callback);
    } else {
      setTimeout(callback, timeout);
    }
  }

  /**
   * @param {Date} oldest
   * @param {Date} youngest
   * @returns {number}
   */
  function getNumberOfDaysBetweenTwoDates(oldest, youngest) {
    return Math.round(
      Math.abs((youngest.getTime() - oldest.getTime()) / date.ONE_DAY_IN_MS),
    );
  }

  /**
   * @param {string} s
   */
  function stringToId(s) {
    return s.replace(/\W/g, " ").trim().replace(/ +/g, "-").toLowerCase();
  }

  const api = (() => {
    /**
     * @template T
     * @param {(value: T) => void} callback
     * @param {string} path
     */
    async function fetchApi(callback, path) {
      const url = `/api${path}`;

      /** @type {T | null} */
      let cachedJson = null;

      /** @type {Cache | undefined} */
      let cache;
      try {
        cache = await caches.open("api");
        const cachedResponse = await cache.match(url);
        if (cachedResponse) {
          console.log(`cache: ${url}`);
          const json = /** @type {T} */ await cachedResponse.json();
          cachedJson = json;
          callback(json);
        }
      } catch {}

      if (navigator.onLine) {
        // TODO: rerun after 10s instead of returning (due to some kind of error)

        /** @type {Response | undefined} */
        let fetchedResponse;
        try {
          fetchedResponse = await fetch(url, {
            signal: AbortSignal.timeout(5000),
          });
          if (!fetchedResponse.ok) {
            throw Error;
          }
        } catch {
          return cachedJson;
        }

        const clonedResponse = fetchedResponse.clone();

        let fetchedJson = /** @type {T | null} */ (null);
        try {
          fetchedJson = /** @type {T} */ (await fetchedResponse.json());
        } catch (_) {
          return cachedJson;
        }

        if (!fetchedJson) return cachedJson;

        console.log(`fetch: ${url}`);

        if (Array.isArray(cachedJson) && Array.isArray(fetchedJson)) {
          const previousLength = cachedJson?.length || 0;
          const newLength = fetchedJson.length;

          if (!newLength) {
            return cachedJson;
          }

          if (previousLength && previousLength === newLength) {
            const previousLastValue = Object.values(cachedJson || []).at(-1);
            const newLastValue = Object.values(fetchedJson).at(-1);
            if (
              JSON.stringify(previousLastValue) === JSON.stringify(newLastValue)
            ) {
              return cachedJson;
            }
          }
        }

        callback(fetchedJson);

        runWhenIdle(async function () {
          try {
            await cache?.put(url, clonedResponse);
          } catch (_) {}
        });

        return fetchedJson;
      } else {
        return cachedJson;
      }
    }

    /**
     * @param {Index} index
     */
    function vecIndexToString(index) {
      switch (index) {
        case /** @satisfies {Addressindex} */ (9):
          return "Addressindex";
        case /** @satisfies {Dateindex} */ (1):
          return "Dateindex";
        case /** @satisfies {Height} */ (0):
          return "Height";
        case /** @satisfies {P2PK33index} */ (10):
          return "P2PK33index";
        case /** @satisfies {P2PK65index} */ (11):
          return "P2PK65index";
        case /** @satisfies {P2PKHindex} */ (12):
          return "P2PKHindex";
        case /** @satisfies {P2SHindex} */ (13):
          return "P2SHindex";
        case /** @satisfies {P2TRindex} */ (14):
          return "P2TRindex";
        case /** @satisfies {P2WPKHindex} */ (15):
          return "P2WPKHindex";
        case /** @satisfies {P2WSHindex} */ (16):
          return "P2WSHindex";
        case /** @satisfies {Txindex} */ (17):
          return "Txindex";
        case /** @satisfies {Txinindex} */ (18):
          return "Txinindex";
        case /** @satisfies {Txoutindex} */ (19):
          return "Txoutindex";
        case /** @satisfies {Weekindex} */ (2):
          return "Weekindex";
        case /** @satisfies {Monthindex} */ (4):
          return "Monthindex";
        case /** @satisfies {Quarterindex} */ (5):
          return "Quarterindex";
        case /** @satisfies {Yearindex} */ (6):
          return "Yearindex";
        case /** @satisfies {Decadeindex} */ (7):
          return "Decadeindex";
        case /** @satisfies {Difficultyepoch} */ (3):
          return "Difficultyepoch";
        case /** @satisfies {Halvingepoch} */ (8):
          return "Halvingepoch";
      }
    }

    /**
     * @param {Index} index
     * @param {VecId} vecId
     * @param {number} [from]
     * @param {number} [to]
     */
    function genPath(index, vecId, from, to) {
      let path = `/query?index=${vecIndexToString(index)}&values=${vecId}`;
      if (from !== undefined) {
        path += `&from=${from}`;
      }
      if (to !== undefined) {
        path += `&to=${to}`;
      }
      return path;
    }

    return {
      /**
       * @param {Index} index
       * @param {VecId} vecId
       */
      genUrl(index, vecId) {
        return `/api${genPath(index, vecId)}`;
      },
      /**
       * @template {number | OHLCTuple} [T=number]
       * @param {(v: T[]) => void} callback
       * @param {Index} index
       * @param {VecId} vecId
       * @param {number} [from]
       * @param {number} [to]
       */
      fetchVec(callback, index, vecId, from, to) {
        return fetchApi(callback, genPath(index, vecId, from, to));
      },
      /**
       * @template {number | OHLCTuple} [T=number]
       * @param {(v: T) => void} callback
       * @param {Index} index
       * @param {VecId} vecId
       */
      fetchLast(callback, index, vecId) {
        return fetchApi(callback, genPath(index, vecId, -1));
      },
    };
  })();

  return {
    api,
    isSerializedBooleanTrue,
    sleep,
    next,
    array,
    dom,
    url,
    locale,
    storage,
    serde,
    formatters,
    date,
    debounce,
    runWhenIdle,
    getNumberOfDaysBetweenTwoDates,
    stringToId,
  };
}
/** @typedef {ReturnType<typeof createUtils>} Utilities */

/**
 * @param {Signals} signals
 * @param {Utilities} utils
 */
function createVecsResources(signals, utils) {
  const owner = signals.getOwner();

  /**
   * @template {number | OHLCTuple} [T=number]
   * @param {Index} index
   * @param {VecId} id
   */
  function createVecResource(index, id) {
    return signals.runWithOwner(owner, () => {
      /** @typedef {T extends number ? SingleValueData : CandlestickData} Value */

      const fetched = signals.createSignal(/** @type {T[] | null} */ (null));
      let loading = false;
      let at = /** @type {Date | null} */ (null);

      return {
        url: utils.api.genUrl(index, id),
        fetched,
        async fetch() {
          if (loading) return fetched();
          if (at) {
            const diff = new Date().getTime() - at.getTime();
            const ONE_MINUTE_IN_MS = 60_000;
            if (diff < ONE_MINUTE_IN_MS) return fetched();
          }
          loading = true;
          const res = /** @type {T[] | null} */ (
            await utils.api.fetchVec(
              (values) => {
                fetched.set(/** @type {T[]} */ (values));
              },
              index,
              id,
              -10_000,
            )
          );
          at = new Date();
          loading = false;
          return res;
        },
      };
    });
  }

  /** @type {Map<string, NonNullable<ReturnType<typeof createVecResource>>>} */
  const map = new Map();

  const vecs = {
    /**
     * @template {number | OHLCTuple} [T=number]
     * @param {Index} index
     * @param {VecId} id
     */
    getOrCreate(index, id) {
      const key = `${index},${id}`;
      const found = map.get(key);
      if (found) {
        return found;
      }

      const vec = createVecResource(index, id);
      if (!vec) throw Error("vec is undefined");
      map.set(key, /** @type {any} */ (vec));
      return vec;
    },
  };

  return vecs;
}
/** @typedef {ReturnType<typeof createVecsResources>} VecsResources */
/** @typedef {ReturnType<VecsResources["getOrCreate"]>} VecResource */

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
/** @typedef {ReturnType<typeof initEnv>} Env */

function getElements() {
  /**
   * @param {string} id
   */
  function getElementById(id) {
    const element = window.document.getElementById(id);
    if (!element) throw `Element with id = "${id}" should exist`;
    return element;
  }

  return {
    head: window.document.getElementsByTagName("head")[0],
    body: window.document.body,
    main: getElementById("main"),
    aside: getElementById("aside"),
    asideLabel: getElementById("aside-selector-label"),
    navLabel: getElementById(`nav-selector-label`),
    searchLabel: getElementById(`search-selector-label`),
    search: getElementById("search"),
    nav: getElementById("nav"),
    searchInput: /** @type {HTMLInputElement} */ (
      getElementById("search-input")
    ),
    searchResults: getElementById("search-results"),
    selectors: getElementById("frame-selectors"),
    style: getComputedStyle(window.document.documentElement),
    charts: getElementById("charts"),
    simulation: getElementById("simulation"),
  };
}
/** @typedef {ReturnType<typeof getElements>} Elements */

/**
 * @param {Accessor<boolean>} dark
 * @param {Elements} elements
 */
function createColors(dark, elements) {
  /**
   * @param {string} color
   */
  function getColor(color) {
    return elements.style.getPropertyValue(`--${color}`);
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
  function off() {
    return getColor("gray");
  }

  /**
   * @param {string} property
   */
  function getLightDarkValue(property) {
    const value = elements.style.getPropertyValue(property);
    const [light, _dark] = value.slice(11, -1).split(", ");
    return dark() ? _dark : light;
  }

  function textColor() {
    return getLightDarkValue("--color");
  }
  function borderColor() {
    return getLightDarkValue("--border-color");
  }

  return {
    default: textColor,
    off,
    border: borderColor,
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
 * @typedef {Colors["orange"]} Color
 * @typedef {keyof Colors} ColorName
 */

/**
 * @param {Signals} signals
 * @param {Utilities} utils
 */
function initWebSockets(signals, utils) {
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
   * @param {(candle: CandlestickData) => void} callback
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

      /** @type {CandlestickData} */
      const candle = {
        index: -1,
        time: dateStr,
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

  kraken1dCandle.open();

  function createDocumentTitleEffect() {
    signals.createEffect(kraken1dCandle.latest, (latest) => {
      if (latest) {
        const close = latest.close;
        console.log("close:", close);

        window.document.title = `${latest.close.toLocaleString(
          "en-us",
        )} | kibo.money`;
      }
    });
  }
  createDocumentTitleEffect();

  return {
    kraken1dCandle,
  };
}
/** @typedef {ReturnType<typeof initWebSockets>} WebSockets */

function main() {
  const optionsPromise = import("./options.js");
  const vecidToIndexesPromise = import("./vecid-to-indexes.js");
  const packages = initPackages();
  const env = initEnv();
  const utils = createUtils();
  const elements = getElements();

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
      const { aside, body, main } = elements;
      const MEDIUM_WIDTH = 768;
      if (clientWidth >= MEDIUM_WIDTH) {
        aside.parentElement !== body && body.append(aside);
      } else {
        aside.parentElement !== main && main.append(aside);
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

  packages.signals().then((signals) =>
    vecidToIndexesPromise.then(({ createVecIdToIndexes }) =>
      optionsPromise.then(async ({ initOptions }) => {
        const vecIdToIndexes = createVecIdToIndexes();

        function initDark() {
          const preferredColorSchemeMatchMedia = window.matchMedia(
            "(prefers-color-scheme: dark)",
          );
          const dark = signals.createSignal(
            preferredColorSchemeMatchMedia.matches,
          );
          preferredColorSchemeMatchMedia.addEventListener(
            "change",
            ({ matches }) => {
              dark.set(matches);
            },
          );
          return dark;
        }
        const dark = initDark();

        const qrcode = signals.createSignal(
          /** @type {string | null} */ (null),
        );

        function createLastHeightResource() {
          const lastHeight = signals.createSignal(0);
          function fetchLastHeight() {
            utils.api.fetchLast(
              (h) => {
                lastHeight.set(h);
              },
              /** @satisfies {Height} */ (0),
              "height",
            );
          }
          fetchLastHeight();
          setInterval(fetchLastHeight, 10_000);
          return lastHeight;
        }
        const lastHeight = createLastHeightResource();

        const webSockets = initWebSockets(signals, utils);

        const vecsResources = createVecsResources(signals, utils);

        const colors = createColors(dark, elements);

        const options = initOptions({
          colors,
          env,
          signals,
          utils,
          webSockets,
          qrcode,
        });

        // const urlSelected = utils.url.pathnameToSelectedId();
        // function createWindowPopStateEvent() {
        //   window.addEventListener("popstate", (event) => {
        //     const urlSelected = utils.url.pathnameToSelectedId();
        //     const option = options.list.find((option) => urlSelected === option.id);
        //     if (option) {
        //       options.selected.set(option);
        //     }
        //   });
        // }
        // createWindowPopStateEvent();

        function initSelected() {
          function initSelectedFrame() {
            console.log("selected: init");

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
              let firstTimeLoadingChart = true;
              let firstTimeLoadingSim = true;

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
                  case "chart": {
                    console.log("chart", option);

                    element = elements.charts;

                    lastChartOption.set(option);

                    if (firstTimeLoadingChart) {
                      const lightweightCharts = packages.lightweightCharts();
                      const chartScript = import("./chart.js");
                      utils.dom.importStyleAndThen("/styles/chart.css", () =>
                        chartScript.then(({ init: initChartsElement }) =>
                          lightweightCharts.then((lightweightCharts) =>
                            signals.runWithOwner(owner, () =>
                              initChartsElement({
                                colors,
                                elements,
                                lightweightCharts,
                                selected: /** @type {any} */ (lastChartOption),
                                signals,
                                utils,
                                webSockets,
                                vecsResources,
                                vecIdToIndexes,
                              }),
                            ),
                          ),
                        ),
                      );
                    }
                    firstTimeLoadingChart = false;

                    break;
                  }
                  case "simulation": {
                    element = elements.simulation;

                    lastSimulationOption.set(option);

                    if (firstTimeLoadingSim) {
                      const lightweightCharts = packages.lightweightCharts();
                      const simulationScript = import("./simulation.js");

                      utils.dom.importStyleAndThen(
                        "/styles/simulation.css",
                        () =>
                          simulationScript.then(({ init }) =>
                            lightweightCharts.then((lightweightCharts) =>
                              signals.runWithOwner(owner, () =>
                                init({
                                  colors,
                                  elements,
                                  lightweightCharts,
                                  signals,
                                  utils,
                                  vecsResources,
                                }),
                              ),
                            ),
                          ),
                      );
                    }
                    firstTimeLoadingSim = false;

                    break;
                  }
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

        function initFolders() {
          function initTreeElement() {
            options.treeElement.set(() => {
              const treeElement = window.document.createElement("div");
              treeElement.classList.add("tree");
              elements.nav.append(treeElement);
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
                const id = path[i];
                const details = /** @type {HTMLDetailsElement} */ (
                  utils.dom.getElementById(id)
                );
                details.open = true;
                i++;
              } catch {
                await utils.next();
              }
            }

            await utils.next();

            utils.dom
              .getElementById(`${selectedId}-nav-selector`)
              .scrollIntoView({
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

            const haystack = options.list.map((option) => option.title);

            const RESULTS_PER_PAGE = 100;

            packages.ufuzzy().then((ufuzzy) => {
              /**
               * @param {uFuzzy.SearchResult} searchResult
               * @param {number} pageIndex
               */
              function computeResultPage(searchResult, pageIndex) {
                /** @type {{ option: Option, title: string }[]} */
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

                    const title = haystack[index];

                    list[i % 100] = {
                      option: options.list[index],
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
                  const needle = /** @type {string} */ (
                    elements.searchInput.value
                  );

                  dispose?.();

                  dispose = _dispose;

                  elements.searchResults.scrollTo({
                    top: 0,
                  });

                  if (!needle) {
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

                  elements.searchResults.innerHTML = "";

                  const list = computeResultPage(result, 0);

                  list.forEach(({ option, title }) => {
                    const li = window.document.createElement("li");
                    elements.searchResults.appendChild(li);

                    const element = options.createOptionElement({
                      option,
                      frame: "search",
                      name: title,
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
    ),
  );
}
main();
