// @ts-check

/**
 * @import { Option, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, SimulationOption, AnySeriesBlueprint, SeriesType } from "./options"
 * @import { Valued,  SingleValueData, CandlestickData, OHLCTuple, Series, ISeries, LineData, BaselineData, PartialLineStyleOptions, PartialBaselineStyleOptions, PartialCandlestickStyleOptions } from "../packages/lightweight-charts/wrapper"
 * @import * as _ from "../packages/ufuzzy/v1.0.18/types"
 * @import { Signal, Signals, Accessor } from "../packages/solid-signals/wrapper";
 * @import { DateIndex, DecadeIndex, DifficultyEpoch, Index, HalvingEpoch, Height, MonthIndex, P2PK33Index, P2PK65Index, P2PKHIndex, P2SHIndex, P2MSIndex, P2AIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, TxIndex, InputIndex, OutputIndex, VecId, WeekIndex, YearIndex, VecIdToIndexes, QuarterIndex, EmptyOutputIndex, OpReturnIndex, UnknownOutputIndex } from "./vecid-to-indexes"
 */

/**
 * @typedef {"" |
 *   "BTC" |
 *   "Cents" |
 *   "coinblocks" |
 *   "coindays" |
 *   "satblocks" |
 *   "satdays" |
 *   "Count" |
 *   "Date" |
 *   "Difficulty" |
 *   "ExaHash / Second" |
 *   "Gigabytes" |
 *   "Hash" |
 *   "Index" |
 *   "mb" |
 *   "percentage" |
 *   "Ratio" |
 *   "Sats" |
 *   "Seconds" |
 *   "Timestamp" |
 *   "tx" |
 *   "Type" |
 *   "USD / (PetaHash / Second)" |
 *   "USD" |
 *   "Version" |
 *   "WU" |
 *   "Bool" |
 *   "Days" |
 *   "%mcap" |
 *   "%rcap" |
 *   "%self" |
 *   "%all" |
 *   "Years" |
 *   "Locktime" |
 *   "sat/vB" |
 *   "constant" |
 *   "cagr" |
 *   "vB" |
 *   "performance" |
 *   "sd" |
 *   "Epoch" |
 *   "Height" |
 *   "Type" |
 *   "zscore" |
 *   "Bytes"
 * } Unit
 */

const localhost = window.location.hostname === "localhost";

function initPackages() {
  const imports = {
    async signals() {
      return import("../packages/solid-signals/wrapper.js").then(
        (d) => d.default
      );
    },
    async lightweightCharts() {
      return window.document.fonts.ready.then(() =>
        import("../packages/lightweight-charts/wrapper.js").then(
          (d) => d.default
        )
      );
    },
    async leanQr() {
      return import("../packages/lean-qr/v2.5.0/script.js").then((d) => d);
    },
    async ufuzzy() {
      return import("../packages/ufuzzy/v1.0.18/script.js").then(
        ({ default: d }) => d
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
     * @param {string | HTMLElement} arg.inside
     * @param {string} arg.title
     * @param {(event: MouseEvent) => void} arg.onClick
     */
    createButtonElement({ inside: text, onClick, title }) {
      const button = window.document.createElement("button");

      button.append(text);

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
     * @template {Readonly<string[]>} T
     * @param {Object} args
     * @param {T[number]} args.defaultValue
     * @param {string} [args.id]
     * @param {T | Accessor<T>} args.choices
     * @param {string} [args.keyPrefix]
     * @param {string} args.key
     * @param {boolean} [args.sorted]
     * @param {Signals} args.signals
     */
    createHorizontalChoiceField({
      id,
      choices: unsortedChoices,
      defaultValue,
      keyPrefix,
      key,
      signals,
      sorted,
    }) {
      const choices = signals.createMemo(() => {
        /** @type {T} */
        let c;
        if (typeof unsortedChoices === "function") {
          c = unsortedChoices();
        } else {
          c = unsortedChoices;
        }

        return sorted
          ? /** @type {T} */ (
              /** @type {any} */ (c.toSorted((a, b) => a.localeCompare(b)))
            )
          : c;
      });

      /** @type {Signal<T[number]>} */
      const selected = signals.createSignal(defaultValue, {
        save: {
          ...serde.string,
          keyPrefix: keyPrefix ?? "",
          key,
          saveDefaultValue: true,
        },
      });

      const field = window.document.createElement("div");
      field.classList.add("field");

      const div = window.document.createElement("div");
      field.append(div);

      signals.createEffect(choices, (choices) => {
        const s = selected();
        if (!choices.includes(s)) {
          if (choices.includes(defaultValue)) {
            selected.set(() => defaultValue);
          } else if (choices.length) {
            selected.set(() => choices[0]);
          }
        }

        div.innerHTML = "";

        choices.forEach((choice) => {
          const inputValue = choice;
          const { label } = this.createLabeledInput({
            inputId: `${id ?? key}-${choice.toLowerCase()}`,
            inputName: id ?? key,
            inputValue,
            inputChecked: inputValue === selected(),
            labelTitle: choice,
            type: "radio",
          });

          const text = window.document.createTextNode(choice);
          label.append(text);
          div.append(label);
        });
      });

      field.addEventListener("change", (event) => {
        // @ts-ignore
        const value = event.target.value;
        selected.set(value);
      });

      return { field, selected };
    },
    /**
     * @param {string} [title]
     * @param {1 | 2 | 3} [level]
     */
    createHeader(title = "", level = 1) {
      const headerElement = window.document.createElement("header");

      const headingElement = window.document.createElement(`h${level}`);
      headingElement.innerHTML = title;
      headerElement.append(headingElement);
      headingElement.style.display = "block";

      return {
        headerElement,
        headingElement,
      };
    },
    /**
     * @template {string} Name
     * @template {string} Value
     * @template {Value | {name: Name; value: Value}} T
     * @param {T} arg
     */
    createOption(arg) {
      const option = window.document.createElement("option");
      if (typeof arg === "object") {
        option.value = arg.value;
        option.innerText = arg.name;
      } else {
        option.value = arg;
        option.innerText = arg;
      }
      return option;
    },
    /**
     * @template {string} Name
     * @template {string} Value
     * @template {Value | {name: Name; value: Value}} T
     * @param {Object} args
     * @param {string} [args.id]
     * @param {boolean} [args.deep]
     * @param {readonly ((T) | {name: string; list: T[]})[]} args.list
     * @param {Signal<T>} args.signal
     */
    createSelect({ id, list, signal, deep = false }) {
      const select = window.document.createElement("select");

      if (id) {
        select.name = id;
        select.id = id;
      }

      /** @type {Record<string, VoidFunction>} */
      const setters = {};

      list.forEach((anyOption, index) => {
        if (typeof anyOption === "object" && "list" in anyOption) {
          const { name, list } = anyOption;
          const optGroup = window.document.createElement("optgroup");
          optGroup.label = name;
          select.append(optGroup);
          list.forEach((option) => {
            optGroup.append(this.createOption(option));
            const key = /** @type {string} */ (
              typeof option === "object" ? option.value : option
            );
            setters[key] = () => signal.set(() => option);
          });
        } else {
          select.append(this.createOption(anyOption));
          const key = /** @type {string} */ (
            typeof anyOption === "object" ? anyOption.value : anyOption
          );
          setters[key] = () => signal.set(() => anyOption);
        }
        if (deep && index !== list.length - 1) {
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

      const initialSignal = signal();
      const initialValue =
        typeof initialSignal === "object" ? initialSignal.value : initialSignal;
      select.value = String(initialValue);

      return { select, signal };
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

      try {
        window.history.pushState(
          null,
          "",
          `${pathname}?${urlParams.toString()}`
        );
      } catch (_) {}
    },
    /**
     * @param {Object} args
     * @param {URLSearchParams} [args.urlParams]
     * @param {string} [args.pathname]
     */
    replaceHistory({ urlParams, pathname }) {
      urlParams ||= new URLSearchParams(window.location.search);
      pathname ||= window.location.pathname;

      try {
        window.history.replaceState(
          null,
          "",
          `${pathname}?${urlParams.toString()}`
        );
      } catch (_) {}
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

  const thoroughUnitCheck = localhost;

  /**
   * @param {VecId} id
   */
  function vecidToUnit(id) {
    /** @type {Unit | undefined} */
    let unit;

    if (
      (!unit || thoroughUnitCheck) &&
      (id.includes("in-sats") ||
        (id.endsWith("supply") &&
          !(id.endsWith("circulating-supply") || id.endsWith("-own-supply"))) ||
        id.endsWith("supply-even") ||
        id.endsWith("supply-in-profit") ||
        id.endsWith("supply-in-loss") ||
        id.endsWith("stack") ||
        (id.endsWith("value") && !id.includes("realized")) ||
        ((id.includes("coinbase") ||
          id.includes("fee") ||
          id.includes("subsidy") ||
          id.includes("rewards")) &&
          !(
            id.startsWith("is-") ||
            id.includes("in-btc") ||
            id.includes("in-usd")
          )))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Sats";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("in-btc")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "BTC";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "high" ||
        id === "ohlc" ||
        id === "low" ||
        id === "close" ||
        id === "open" ||
        id === "marketcap" ||
        id.includes("in-usd") ||
        id.startsWith("price") ||
        id.endsWith("price-paid") ||
        id.endsWith("price") ||
        id.endsWith("value-created") ||
        id.endsWith("value-destroyed") ||
        (id.includes("realized") &&
          !id.includes("ratio") &&
          !id.includes("relative-to")) ||
        (id.endsWith("sma") && !id.includes("ratio")) ||
        id === "ath")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "USD";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("cents")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Cents";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id.endsWith("ratio") ||
        (id.includes("ratio") && id.endsWith("sma")) ||
        id.endsWith("1sd") ||
        id.endsWith("2sd") ||
        id.endsWith("3sd") ||
        id.endsWith("p0-1") ||
        id.endsWith("p0-5") ||
        id.endsWith("p1") ||
        id.endsWith("p99") ||
        id.endsWith("p99-5") ||
        id.endsWith("p99-9"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Ratio";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "drawdown" || id.endsWith("oscillator"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "percentage";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id.endsWith("count") ||
        id.includes("-count-") ||
        id.startsWith("block-count") ||
        id.includes("tx-v"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Count";
    }
    if ((!unit || thoroughUnitCheck) && id.startsWith("is-")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Bool";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("type")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Type";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "interval" || id.startsWith("block-interval"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Seconds";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("returns")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "performance";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("zscore")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "zscore";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("locktime")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Locktime";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("cagr")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "cagr";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("version")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Version";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "txid" || (id.endsWith("bytes") && !id.endsWith("vbytes")))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Bytes";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("standard-deviation")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "sd";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id.endsWith("-size") || id.endsWith("-size-sum"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "mb";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id.endsWith("vsize") ||
        id.endsWith("vbytes") ||
        id.endsWith("-vbytes-sum"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "vB";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("weight")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "WU";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("index")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Index";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "date" || id === "date-fixed")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Date";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "timestamp" || id === "timestamp-fixed")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Timestamp";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("coinblocks")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "coinblocks";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("coindays")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "coindays";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("satblocks")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "satblocks";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("satdays")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "satdays";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("height")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Height";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("relative-to-market-cap")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "%mcap";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      id.endsWith("relative-to-realized-cap")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "%rcap";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      id.endsWith("relative-to-circulating-supply")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "%all";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("relative-to-own-supply")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "%self";
    }
    if ((!unit || thoroughUnitCheck) && id.endsWith("epoch")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Epoch";
    }
    if ((!unit || thoroughUnitCheck) && id === "difficulty") {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Difficulty";
    }
    if ((!unit || thoroughUnitCheck) && id === "blockhash") {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Hash";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id.includes("days-between") || id.includes("days-since"))
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Days";
    }
    if ((!unit || thoroughUnitCheck) && id.includes("years-between")) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "Years";
    }
    if (
      (!unit || thoroughUnitCheck) &&
      (id === "0" || id === "1" || id === "50" || id === "100")
    ) {
      if (unit) throw Error(`Unit "${unit}" already assigned "${id}"`);
      unit = "constant";
    }

    if (!unit) {
      console.log();
      throw Error(`Unit not set for "${id}"`);
    }
    return /** @type {Unit} */ (unit);
  }

  const locale = {
    numberToUSFormat,
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
      try {
        return localStorage.getItem(key);
      } catch (_) {
        return null;
      }
    },
    /**
     * @param {string} key
     * @param {string | boolean | null | undefined} value
     */
    write(key, value) {
      try {
        value !== undefined && value !== null
          ? localStorage.setItem(key, String(value))
          : localStorage.removeItem(key);
      } catch (_) {}
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
    vecIds: {
      /**
       * @param {VecId[]} v
       */
      serialize(v) {
        return v.join(",");
      },
      /**
       * @param {string} v
       */
      deserialize(v) {
        return /** @type {VecId[]} */ (v.split(","));
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
    index: {
      /**
       * @param {Index} v
       */
      serialize(v) {
        switch (v) {
          case /** @satisfies {DateIndex} */ (0):
            return "dateindex";
          case /** @satisfies {DecadeIndex} */ (1):
            return "decadeindex";
          case /** @satisfies {DifficultyEpoch} */ (2):
            return "difficultyepoch";
          case /** @satisfies {EmptyOutputIndex} */ (3):
            return "emptyoutputindex";
          case /** @satisfies {HalvingEpoch} */ (4):
            return "halvingepoch";
          case /** @satisfies {Height} */ (5):
            return "height";
          case /** @satisfies {InputIndex} */ (6):
            return "inputindex";
          case /** @satisfies {MonthIndex} */ (7):
            return "monthindex";
          case /** @satisfies {OpReturnIndex} */ (8):
            return "opreturnindex";
          case /** @satisfies {OutputIndex} */ (9):
            return "outputindex";
          case /** @satisfies {P2AIndex} */ (10):
            return "p2aindex";
          case /** @satisfies {P2MSIndex} */ (11):
            return "p2msindex";
          case /** @satisfies {P2PK33Index} */ (12):
            return "p2pk33index";
          case /** @satisfies {P2PK65Index} */ (13):
            return "p2pk65index";
          case /** @satisfies {P2PKHIndex} */ (14):
            return "p2pkhindex";
          case /** @satisfies {P2SHIndex} */ (15):
            return "p2shindex";
          case /** @satisfies {P2TRIndex} */ (16):
            return "p2trindex";
          case /** @satisfies {P2WPKHIndex} */ (17):
            return "p2wpkhindex";
          case /** @satisfies {P2WSHIndex} */ (18):
            return "p2wshindex";
          case /** @satisfies {QuarterIndex} */ (19):
            return "quarterindex";
          case /** @satisfies {TxIndex} */ (20):
            return "txindex";
          case /** @satisfies {UnknownOutputIndex} */ (21):
            return "unknownoutputindex";
          case /** @satisfies {WeekIndex} */ (22):
            return "weekindex";
          case /** @satisfies {YearIndex} */ (23):
            return "yearindex";
        }
      },
    },
    chartableIndex: {
      /**
       * @param {number} v
       */
      serialize(v) {
        switch (v) {
          case /** @satisfies {DateIndex} */ (0):
            return "date";
          case /** @satisfies {DecadeIndex} */ (1):
            return "decade";
          // case /** @satisfies {DifficultyEpoch} */ (2):
          //   return "difficulty";
          // case /** @satisfies {HalvingEpoch} */ (4):
          //   return "halving";
          case /** @satisfies {Height} */ (5):
            return "timestamp";
          case /** @satisfies {MonthIndex} */ (7):
            return "month";
          case /** @satisfies {QuarterIndex} */ (19):
            return "quarter";
          case /** @satisfies {WeekIndex} */ (22):
            return "week";
          case /** @satisfies {YearIndex} */ (23):
            return "year";
          default:
            return null;
        }
      },
      /**
       * @param {string} v
       * @returns {Index}
       */
      deserialize(v) {
        switch (v) {
          case "timestamp":
            return /** @satisfies {Height} */ (5);
          case "date":
            return /** @satisfies {DateIndex} */ (0);
          case "week":
            return /** @satisfies {WeekIndex} */ (22);
          case "month":
            return /** @satisfies {MonthIndex} */ (7);
          case "quarter":
            return /** @satisfies {QuarterIndex} */ (19);
          case "year":
            return /** @satisfies {YearIndex} */ (23);
          case "decade":
            return /** @satisfies {DecadeIndex} */ (1);
          default:
            throw Error("todo");
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
          0
        )
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
      Math.abs((youngest.getTime() - oldest.getTime()) / date.ONE_DAY_IN_MS)
    );
  }

  /**
   * @param {string} s
   */
  function stringToId(s) {
    return (
      s
        // .replace(/\W/g, " ")
        .trim()
        .replace(/ +/g, "-")
        .toLowerCase()
    );
  }

  const api = (() => {
    const CACHE_NAME = "api";

    /**
     * @template T
     * @param {(value: T) => void} callback
     * @param {string} path
     * @param {boolean} [mustBeArray]
     */
    async function fetchApi(callback, path, mustBeArray) {
      const url = `/api${path}`;

      /** @type {T | null} */
      let cachedJson = null;

      /** @type {Cache | undefined} */
      let cache;
      try {
        cache = await caches.open(CACHE_NAME);
        const cachedResponse = await cache.match(url);
        if (cachedResponse) {
          console.debug(`cache: ${url}`);
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
          const f = await fetchedResponse.json();
          fetchedJson = /** @type {T} */ (
            mustBeArray && !Array.isArray(f) ? [f] : f
          );
        } catch (_) {
          return cachedJson;
        }

        if (!fetchedJson) return cachedJson;

        console.debug(`fetch: ${url}`);

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
     * @param {VecId} vecId
     * @param {number} [from]
     * @param {number} [to]
     */
    function genPath(index, vecId, from, to) {
      let path = `/query?index=${serde.index.serialize(index)}&values=${vecId}`;
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
       * @param {number} from
       */
      genUrl(index, vecId, from) {
        return `/api${genPath(index, vecId, from)}`;
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
        return fetchApi(callback, genPath(index, vecId, from, to), true);
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
    vecidToUnit,
  };
}
/** @typedef {ReturnType<typeof createUtils>} Utilities */

/**
 * @param {Signals} signals
 * @param {Utilities} utils
 */
function createVecsResources(signals, utils) {
  const owner = signals.getOwner();

  const defaultFrom = -10_000;
  const defaultTo = undefined;

  /**
   * Defaults
   * - from: -10_000
   * - to: undefined
   *
   * @param {Object} [args]
   * @param {number} [args.from]
   * @param {number} [args.to]
   */
  function genFetchedKey(args) {
    return `${args?.from}-${args?.to}`;
  }

  const defaultFetchedKey = genFetchedKey({ from: defaultFrom, to: defaultTo });

  /**
   * @template {number | OHLCTuple} [T=number]
   * @param {Index} index
   * @param {VecId} id
   */
  function createVecResource(index, id) {
    return signals.runWithOwner(owner, () => {
      /** @typedef {T extends number ? SingleValueData : CandlestickData} Value */

      const fetchedRecord = signals.createSignal(
        /** @type {Map<string, {loading: boolean, at: Date | null, vec: Signal<T[] | null>}>} */ (
          new Map()
        )
      );

      return {
        url: utils.api.genUrl(index, id, defaultFrom),
        fetched: fetchedRecord,
        /**
         * Defaults
         * - from: -10_000
         * - to: undefined
         *
         * @param {Object} [args]
         * @param {number} [args.from]
         * @param {number} [args.to]
         */
        async fetch(args) {
          const from = args?.from ?? defaultFrom;
          const to = args?.to ?? defaultTo;
          const fetchedKey = genFetchedKey({ from, to });
          if (!fetchedRecord().has(fetchedKey)) {
            fetchedRecord.set((map) => {
              map.set(fetchedKey, {
                loading: false,
                at: null,
                vec: signals.createSignal(/** @type {T[] | null} */ (null)),
              });
              return map;
            });
          }
          const fetched = fetchedRecord().get(fetchedKey);
          if (!fetched) throw Error("Unreachable");
          if (fetched.loading) return fetched.vec();
          if (fetched.at) {
            const diff = new Date().getTime() - fetched.at.getTime();
            const ONE_MINUTE_IN_MS = 60_000;
            if (diff < ONE_MINUTE_IN_MS) return fetched.vec();
          }
          fetched.loading = true;
          const res = /** @type {T[] | null} */ (
            await utils.api.fetchVec(
              (values) => {
                if (values.length || !fetched.vec()) {
                  fetched.vec.set(values);
                }
              },
              index,
              id,
              from,
              to
            )
          );
          fetched.at = new Date();
          fetched.loading = false;
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
    genFetchedKey,
    defaultFetchedKey,
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
    localhost,
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
    table: getElementById("table"),
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
  function gray() {
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
    gray,
    border: borderColor,

    red,
    orange,
    amber,
    yellow,
    avocado,
    lime,
    green,
    emerald,
    teal,
    cyan,
    sky,
    blue,
    indigo,
    violet,
    purple,
    fuchsia,
    pink,
    rose,
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
          reinitWebSocketIfDocumentNotHidden
        );

        window.document.addEventListener("online", reinitWebSocket);
      },
      close() {
        ws?.close();
        window.document.removeEventListener(
          "visibilitychange",
          reinitWebSocketIfDocumentNotHidden
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
   */
  function krakenCandleWebSocketCreator(callback) {
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
        })
      );
    });

    ws.addEventListener("message", (message) => {
      const result = JSON.parse(message.data);

      if (result.channel !== "ohlc") return;

      const { interval_begin, open, high, low, close } = result.data.at(-1);

      /** @type {CandlestickData} */
      const candle = {
        // index: -1,
        time: new Date(interval_begin).valueOf() / 1000,
        open: Number(open),
        high: Number(high),
        low: Number(low),
        close: Number(close),
      };

      candle && callback({ ...candle });
    });

    return ws;
  }

  /** @type {ReturnType<typeof createWebsocket<CandlestickData>>} */
  const kraken1dCandle = createWebsocket((callback) =>
    krakenCandleWebSocketCreator(callback)
  );

  kraken1dCandle.open();

  function createDocumentTitleEffect() {
    signals.createEffect(kraken1dCandle.latest, (latest) => {
      if (latest) {
        const close = latest.close;
        console.log("close:", close);

        window.document.title = `${latest.close.toLocaleString("en-us")} | ${
          window.location.host
        }`;
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
              /** @type {string} */ (input.value)
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
    vecidToIndexesPromise.then(({ createVecIdToIndexes, VERSION }) =>
      optionsPromise.then(async ({ initOptions }) =>
        signals.createRoot(() => {
          const owner = signals.getOwner();

          console.log(`VERSION = ${VERSION}`);

          const vecIdToIndexes = createVecIdToIndexes();

          if (env.localhost) {
            Object.keys(vecIdToIndexes).forEach((id) => {
              utils.vecidToUnit(/** @type {VecId} */ (id));
            });
          }

          function initDark() {
            const preferredColorSchemeMatchMedia = window.matchMedia(
              "(prefers-color-scheme: dark)"
            );
            const dark = signals.createSignal(
              preferredColorSchemeMatchMedia.matches
            );
            preferredColorSchemeMatchMedia.addEventListener(
              "change",
              ({ matches }) => {
                dark.set(matches);
              }
            );
            return dark;
          }
          const dark = initDark();

          const qrcode = signals.createSignal(
            /** @type {string | null} */ (null)
          );

          function createLastHeightResource() {
            const lastHeight = signals.createSignal(0);
            function fetchLastHeight() {
              utils.api.fetchLast(
                (h) => {
                  lastHeight.set(h);
                },
                /** @satisfies {Height} */ (5),
                "height"
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
            qrcode,
          });

          // window.addEventListener("popstate", (_) => {
          //   const urlSelected = utils.url.pathnameToSelectedId();
          //   const option = options.list.find(
          //     (option) => urlSelected === option.id,
          //   );
          //   if (option) {
          //     options.selected.set(option);
          //   }
          // });

          function initSelected() {
            let firstRun = true;
            function initSelectedFrame() {
              if (!firstRun) throw Error("Unreachable");
              firstRun = false;

              console.log("selected: init");

              const owner = signals.getOwner();

              const chartOption = signals.createSignal(
                /** @type {ChartOption | null} */ (null)
              );
              const simOption = signals.createSignal(
                /** @type {SimulationOption | null} */ (null)
              );

              let previousElement = /** @type {HTMLElement | undefined} */ (
                undefined
              );
              let firstTimeLoadingChart = true;
              let firstTimeLoadingTable = true;
              let firstTimeLoadingSimulation = true;

              signals.createEffect(options.selected, (option) => {
                console.log(utils.url.pathnameToSelectedId(), option.id);
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
                    element = elements.charts;

                    chartOption.set(option);

                    if (firstTimeLoadingChart) {
                      const lightweightCharts = packages.lightweightCharts();
                      import("./chart.js").then(({ init: initChartsElement }) =>
                        lightweightCharts.then((lightweightCharts) =>
                          signals.runWithOwner(owner, () =>
                            initChartsElement({
                              colors,
                              elements,
                              lightweightCharts,
                              option: /** @type {Accessor<ChartOption>} */ (
                                chartOption
                              ),
                              signals,
                              utils,
                              webSockets,
                              vecsResources,
                              vecIdToIndexes,
                            })
                          )
                        )
                      );
                    }
                    firstTimeLoadingChart = false;

                    break;
                  }
                  case "table": {
                    element = elements.table;

                    if (firstTimeLoadingTable) {
                      import("./table.js").then(({ init }) =>
                        signals.runWithOwner(owner, () =>
                          init({
                            colors,
                            elements,
                            signals,
                            utils,
                            vecsResources,
                            option,
                            vecIdToIndexes,
                          })
                        )
                      );
                    }
                    firstTimeLoadingTable = false;

                    break;
                  }
                  case "simulation": {
                    element = elements.simulation;

                    simOption.set(option);

                    if (firstTimeLoadingSimulation) {
                      const lightweightCharts = packages.lightweightCharts();
                      import("./simulation.js").then(({ init }) =>
                        lightweightCharts.then((lightweightCharts) =>
                          signals.runWithOwner(owner, () =>
                            init({
                              colors,
                              elements,
                              lightweightCharts,
                              signals,
                              utils,
                              vecsResources,
                            })
                          )
                        )
                      );
                    }
                    firstTimeLoadingSimulation = false;

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

            utils.dom.onFirstIntersection(elements.aside, () =>
              signals.runWithOwner(owner, initSelectedFrame)
            );
          }
          initSelected();

          function initFolders() {
            // async function scrollToSelected() {
            //   if (!options.selected()) throw "Selected should be set by now";
            //   const selectedId = options.selected().id;

            //   const path = options.selected().path;

            //   let i = 0;
            //   while (i !== path.length) {
            //     try {
            //       const id = path[i];
            //       const details = /** @type {HTMLDetailsElement} */ (
            //         utils.dom.getElementById(id)
            //       );
            //       details.open = true;
            //       i++;
            //     } catch {
            //       await utils.next();
            //     }
            //   }

            //   await utils.next();
            //   await utils.next();

            //   utils.dom
            //     .getElementById(`${selectedId}-nav-selector`)
            //     .scrollIntoView({
            //       behavior: "instant",
            //       block: "center",
            //     });
            // }

            utils.dom.onFirstIntersection(elements.nav, () => {
              options.treeElement.set(() => {
                const treeElement = window.document.createElement("div");
                treeElement.classList.add("tree");
                elements.nav.append(treeElement);
                return treeElement;
              });

              // setTimeout(scrollToSelected, 10);
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

                  let [indexes, info, order] = searchResult || [
                    null,
                    null,
                    null,
                  ];

                  const minIndex = pageIndex * RESULTS_PER_PAGE;

                  if (indexes?.length) {
                    const maxIndex = Math.min(
                      (order || indexes).length - 1,
                      minIndex + RESULTS_PER_PAGE - 1
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
                      infoThresh
                    );

                    if (!result?.[0]?.length || !result?.[1]) {
                      result = fuzzyMultiInsert?.search(
                        haystack,
                        needle,
                        outOfOrder,
                        infoThresh
                      );
                    }

                    if (!result?.[0]?.length || !result?.[1]) {
                      result = fuzzySingleError?.search(
                        haystack,
                        needle,
                        outOfOrder,
                        infoThresh
                      );
                    }

                    if (!result?.[0]?.length || !result?.[1]) {
                      result = fuzzySingleErrorFuzzier?.search(
                        haystack,
                        needle,
                        outOfOrder,
                        infoThresh
                      );
                    }

                    if (!result?.[0]?.length || !result?.[1]) {
                      result = fuzzyMultiInsertFuzzier?.search(
                        haystack,
                        needle,
                        undefined,
                        infoThresh
                      );
                    }

                    if (!result?.[0]?.length || !result?.[1]) {
                      result = fuzzyMultiInsertFuzzier?.search(
                        haystack,
                        needle,
                        outOfOrder,
                        infoThresh
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
            const shareContentDiv =
              utils.dom.getElementById("share-content-div");

            shareDiv.addEventListener("click", () => {
              qrcode.set(null);
            });

            shareContentDiv.addEventListener("click", (event) => {
              event.stopPropagation();
              event.preventDefault();
            });

            packages.leanQr().then(({ generate }) =>
              signals.runWithOwner(owner, () => {
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
              })
            );
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
              // TODO: Check if should be a signal ??
              try {
                if (typeof width === "number") {
                  elements.main.style.width = `${width}px`;
                  utils.storage.write(barWidthLocalStorageKey, String(width));
                } else {
                  elements.main.style.width = elements.style.getPropertyValue(
                    "--default-main-width"
                  );
                  utils.storage.remove(barWidthLocalStorageKey);
                }
              } catch (_) {}
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
        })
      )
    )
  );
}
main();
