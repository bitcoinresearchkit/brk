// @ts-check

/**
 * @import { OptionPath, PartialOption, PartialOptionsGroup, PartialOptionsTree, Option, OptionsGroup, Series, PriceSeriesType, ResourceDataset, TimeScale, SerializedHistory, TimeRange, Unit, Marker, Weighted, DatasetPath, OHLC, FetchedJSON, DatasetValue, FetchedResult, AnyDatasetPath, SeriesBlueprint, BaselineSpecificSeriesBlueprint, CandlestickSpecificSeriesBlueprint, LineSpecificSeriesBlueprint, SpecificSeriesBlueprintWithChart, Signal, Color, SettingsTheme, DatasetCandlestickData, FoldersFilter, PartialChartOption, ChartOption, AnyPartialOption, ProcessedOptionAddons, OptionsTree, AnyPath, SimulationOption } from "./types/self"
 * @import {createChart as CreateClassicChart, createChartEx as CreateCustomChart, LineStyleOptions} from "./packages/lightweight-charts/v4.2.0/types";
 * @import * as _ from "./packages/ufuzzy/v1.0.14/types"
 * @import { DeepPartial, ChartOptions, IChartApi, IHorzScaleBehavior, WhitespaceData, SingleValueData, ISeriesApi, Time, LogicalRange, SeriesMarker, CandlestickData, SeriesType, BaselineStyleOptions, SeriesOptionsCommon } from "./packages/lightweight-charts/v4.2.0/types"
 * @import { DatePath, HeightPath, LastPath } from "./types/paths";
 * @import { SignalOptions, untrack as Untrack } from "./packages/solid-signals/2024-04-17/types/core"
 * @import { getOwner as GetOwner, onCleanup as OnCleanup, Owner } from "./packages/solid-signals/2024-04-17/types/owner"
 * @import { createSignal as CreateSignal, createEffect as CreateEffect, Accessor, Setter, createMemo as CreateMemo, createRoot as CreateRoot, runWithOwner as RunWithOwner } from "./packages/solid-signals/2024-04-17/types/signals";
 */

function initPackages() {
  async function importSignals() {
    return import("./packages/solid-signals/2024-04-17/script.js").then(
      (_signals) => {
        const signals = {
          createSolidSignal: /** @type {CreateSignal} */ (
            _signals.createSignal
          ),
          createEffect: /** @type {CreateEffect} */ (_signals.createEffect),
          createMemo: /** @type {CreateMemo} */ (_signals.createMemo),
          createRoot: /** @type {CreateRoot} */ (_signals.createRoot),
          untrack: /** @type {Untrack} */ (_signals.untrack),
          getOwner: /** @type {GetOwner} */ (_signals.getOwner),
          runWithOwner: /** @type {RunWithOwner} */ (_signals.runWithOwner),
          onCleanup: /** @type {OnCleanup} */ (_signals.onCleanup),
          flushSync: _signals.flushSync,
          /**
           * @template T
           * @param {T} initialValue
           * @param {SignalOptions<T>} [options]
           * @returns {Signal<T>}
           */
          createSignal(initialValue, options) {
            const [get, set] = this.createSolidSignal(initialValue, options);
            // @ts-ignore
            get.set = set;
            // @ts-ignore
            return get;
          },
          /**
           * @param {(dispose: VoidFunction) => void} callback
           */
          createUntrackedRoot: (callback) =>
            signals.untrack(() => {
              signals.createRoot(callback);
            }),
        };

        return signals;
      },
    );
  }

  /** @typedef {Awaited<ReturnType<typeof importSignals>>} Signals */

  const imports = {
    signals: importSignals,
    async lightweightCharts() {
      return window.document.fonts.ready.then(() =>
        import("./packages/lightweight-charts/v4.2.0/script.js").then(
          ({
            createChart: createClassicChart,
            createChartEx: createCustomChart,
          }) => {
            /**
             * @class
             * @implements {IHorzScaleBehavior<number>}
             */
            class HorzScaleBehaviorHeight {
              options() {
                return /** @type {any} */ (undefined);
              }
              setOptions() {}
              preprocessData() {}
              updateFormatter() {}

              createConverterToInternalObj() {
                /** @type {(p: any) => any} */
                return (price) => price;
              }

              /** @param {any} item  */
              key(item) {
                return item;
              }

              /** @param {any} item  */
              cacheKey(item) {
                return item;
              }

              /** @param {any} item  */
              convertHorzItemToInternal(item) {
                return item;
              }

              /** @param {any} item  */
              formatHorzItem(item) {
                return item;
              }

              /** @param {any} tickMark  */
              formatTickmark(tickMark) {
                return tickMark.time.toLocaleString("en-us");
              }

              /** @param {any} tickMarks  */
              maxTickMarkWeight(tickMarks) {
                return tickMarks.reduce(
                  this.getMarkWithGreaterWeight,
                  tickMarks[0],
                ).weight;
              }

              /**
               * @param {any} sortedTimePoints
               * @param {number} startIndex
               */
              fillWeightsForPoints(sortedTimePoints, startIndex) {
                for (
                  let index = startIndex;
                  index < sortedTimePoints.length;
                  ++index
                ) {
                  sortedTimePoints[index].timeWeight = this.computeHeightWeight(
                    sortedTimePoints[index].time,
                  );
                }
              }

              /**
               * @param {any} a
               * @param {any} b
               */
              getMarkWithGreaterWeight(a, b) {
                return a.weight > b.weight ? a : b;
              }

              /** @param {number} value  */
              computeHeightWeight(value) {
                // if (value === Math.ceil(value / 1000000) * 1000000) {
                //   return 12;
                // }
                if (value === Math.ceil(value / 100000) * 100000) {
                  return 11;
                }
                if (value === Math.ceil(value / 10000) * 10000) {
                  return 10;
                }
                if (value === Math.ceil(value / 1000) * 1000) {
                  return 9;
                }
                if (value === Math.ceil(value / 100) * 100) {
                  return 8;
                }
                if (value === Math.ceil(value / 50) * 50) {
                  return 7;
                }
                if (value === Math.ceil(value / 25) * 25) {
                  return 6;
                }
                if (value === Math.ceil(value / 10) * 10) {
                  return 5;
                }
                if (value === Math.ceil(value / 5) * 5) {
                  return 4;
                }
                if (value === Math.ceil(value)) {
                  return 3;
                }
                if (value * 2 === Math.ceil(value * 2)) {
                  return 1;
                }

                return 0;
              }
            }

            /**
             * @param {Object} args
             * @param {TimeScale} args.scale
             * @param {HTMLElement} args.element
             * @param {Signals} args.signals
             * @param {Colors} args.colors
             */
            function createChart({ scale, element, signals, colors }) {
              /** @satisfies {DeepPartial<ChartOptions>} */
              const options = {
                autoSize: true,
                layout: {
                  fontFamily: "Satoshi Chart",
                  // fontSize: 13,
                  background: { color: "transparent" },
                  attributionLogo: false,
                },
                grid: {
                  vertLines: { visible: false },
                  horzLines: { visible: false },
                },
                timeScale: {
                  minBarSpacing: 0.05,
                  shiftVisibleRangeOnNewBar: false,
                  allowShiftVisibleRangeOnWhitespaceReplacement: false,
                },
                handleScale: {
                  axisDoubleClickReset: {
                    time: false,
                  },
                },
                crosshair: {
                  mode: 0,
                },
                localization: {
                  priceFormatter: utils.locale.numberToShortUSFormat,
                  locale: "en-us",
                  ...(scale === "date"
                    ? {
                        // dateFormat: "EEEE, dd MMM 'yy",
                      }
                    : {}),
                },
              };

              /** @type {IChartApi} */
              let chart;

              if (scale === "date") {
                chart = createClassicChart(element, options);
              } else {
                const horzScaleBehavior = new HorzScaleBehaviorHeight();
                // @ts-ignore
                chart = createCustomChart(element, horzScaleBehavior, options);
              }

              chart.priceScale("right").applyOptions({
                scaleMargins: {
                  top: 0.075,
                  bottom: 0.05,
                },
                minimumWidth: 78,
              });

              signals.createEffect(() => {
                const { default: _defaultColor, off: _offColor } = colors;

                const defaultColor = _defaultColor();
                const offColor = _offColor();

                chart.applyOptions({
                  layout: {
                    textColor: offColor,
                  },
                  rightPriceScale: {
                    borderVisible: false,
                  },
                  timeScale: {
                    borderVisible: false,
                  },
                  crosshair: {
                    horzLine: {
                      color: defaultColor,
                      labelBackgroundColor: defaultColor,
                    },
                    vertLine: {
                      color: defaultColor,
                      labelBackgroundColor: defaultColor,
                    },
                  },
                });
              });

              return chart;
            }

            /**
             * @type {DeepPartial<SeriesOptionsCommon>}
             */
            const defaultSeriesOptions = {
              // @ts-ignore
              lineWidth: 1.5,
              priceLineVisible: false,
              baseLineVisible: false,
              baseLineColor: "",
            };

            /**
             * @param {SpecificSeriesBlueprintWithChart<BaselineSpecificSeriesBlueprint> & {colors: Colors, signals: Signals}} args
             */
            function createBaseLineSeries({
              chart,
              color,
              options,
              owner,
              colors,
              signals,
            }) {
              const topLineColor = color || colors.profit;
              const bottomLineColor = color || colors.loss;

              function computeColors() {
                return {
                  topLineColor: topLineColor(),
                  bottomLineColor: bottomLineColor(),
                };
              }

              const transparent = "transparent";

              /** @type {DeepPartial<BaselineStyleOptions & SeriesOptionsCommon>} */
              const seriesOptions = {
                priceScaleId: "right",
                ...defaultSeriesOptions,
                ...options,
                topFillColor1: transparent,
                topFillColor2: transparent,
                bottomFillColor1: transparent,
                bottomFillColor2: transparent,
                ...computeColors(),
              };

              const series = chart.addBaselineSeries(seriesOptions);

              signals.runWithOwner(owner, () => {
                signals.createEffect(() => {
                  series.applyOptions(computeColors());
                });
              });

              return series;
            }

            /**
             * @param {SpecificSeriesBlueprintWithChart<CandlestickSpecificSeriesBlueprint> & {colors: Colors, signals: Signals}} args
             */
            function createCandlesticksSeries({
              chart,
              options,
              owner,
              signals,
              colors,
            }) {
              function computeColors() {
                const upColor = colors.profit();
                const downColor = colors.loss();

                return {
                  upColor,
                  wickUpColor: upColor,
                  downColor,
                  wickDownColor: downColor,
                };
              }

              const candlestickSeries = chart.addCandlestickSeries({
                baseLineVisible: false,
                borderVisible: false,
                priceLineVisible: false,
                baseLineColor: "",
                borderColor: "",
                borderDownColor: "",
                borderUpColor: "",
                ...options,
                ...computeColors(),
              });

              signals.runWithOwner(owner, () => {
                signals.createEffect(() => {
                  candlestickSeries.applyOptions(computeColors());
                });
              });

              return candlestickSeries;
            }

            /**
             * @param {SpecificSeriesBlueprintWithChart<LineSpecificSeriesBlueprint> & {colors: Colors, signals: Signals}} args
             */
            function createLineSeries({
              chart,
              color,
              options,
              owner,
              signals,
              colors,
            }) {
              function computeColors() {
                return {
                  color: color(),
                };
              }

              const series = chart.addLineSeries({
                ...defaultSeriesOptions,
                ...options,
                ...computeColors(),
              });

              signals.runWithOwner(owner, () => {
                signals.createEffect(() => {
                  series.applyOptions(computeColors());
                });
              });

              return series;
            }

            function initWhitespace() {
              const whitespaceStartDate = new Date("1970-01-01");
              const whitespaceStartDateYear =
                whitespaceStartDate.getUTCFullYear();
              const whitespaceStartDateMonth =
                whitespaceStartDate.getUTCMonth();
              const whitespaceStartDateDate = whitespaceStartDate.getUTCDate();
              const whitespaceEndDate = new Date("2141-01-01");
              let whitespaceDateDataset =
                /** @type {(WhitespaceData | SingleValueData)[]} */ ([]);

              function initDateWhitespace() {
                whitespaceDateDataset = new Array(
                  utils.getNumberOfDaysBetweenTwoDates(
                    whitespaceStartDate,
                    whitespaceEndDate,
                  ),
                );
                // Hack to be able to scroll freely
                // Setting them all to NaN is much slower
                for (let i = 0; i < whitespaceDateDataset.length; i++) {
                  const date = new Date(
                    whitespaceStartDateYear,
                    whitespaceStartDateMonth,
                    whitespaceStartDateDate + i,
                  );

                  const time = utils.dateToString(date);

                  if (i === whitespaceDateDataset.length - 1) {
                    whitespaceDateDataset[i] = {
                      time,
                      value: NaN,
                    };
                  } else {
                    whitespaceDateDataset[i] = {
                      time,
                    };
                  }
                }
              }

              const heightStart = -50_000;
              let whitespaceHeightDataset =
                /** @type {WhitespaceData[]} */ ([]);

              function initHeightWhitespace() {
                whitespaceHeightDataset = new Array(
                  (new Date().getUTCFullYear() - 2009 + 1) * 60_000,
                );
                for (let i = 0; i < whitespaceHeightDataset.length; i++) {
                  const height = heightStart + i;

                  whitespaceHeightDataset[i] = {
                    time: /** @type {Time} */ (height),
                  };
                }
              }

              /**
               * @param {IChartApi} chart
               * @param {TimeScale} scale
               * @returns {ISeriesApi<'Line'>}
               */
              function setWhitespace(chart, scale) {
                const whitespace = chart.addLineSeries();

                if (scale === "date") {
                  if (!whitespaceDateDataset.length) {
                    initDateWhitespace();
                  }

                  whitespace.setData(whitespaceDateDataset);
                } else {
                  if (!whitespaceHeightDataset.length) {
                    initHeightWhitespace();
                  }

                  whitespace.setData(whitespaceHeightDataset);

                  const time = whitespaceHeightDataset.length;
                  whitespace.update({
                    time: /** @type {Time} */ (time),
                    value: NaN,
                  });
                }

                return whitespace;
              }

              return { setWhitespace };
            }
            const { setWhitespace } = initWhitespace();

            /**
             *
             * @param {Parameters<typeof createChart>[0]} args
             */
            function createChartWithWhitespace({
              element,
              scale,
              colors,
              signals,
            }) {
              const chart =
                /** @type {IChartApi & {whitespace: ISeriesApi<"Line">}} */ (
                  createChart({
                    colors,
                    element,
                    scale,
                    signals,
                  })
                );
              chart.whitespace = setWhitespace(chart, scale);
              return chart;
            }

            return {
              createChart,
              createChartWithWhitespace,
              createBaseLineSeries,
              createCandlesticksSeries,
              createLineSeries,
            };
          },
        ),
      );
    },
    async leanQr() {
      return import("./packages/lean-qr/v2.3.4/script.js").then((d) => d);
    },
    async ufuzzy() {
      return import("./packages/ufuzzy/v1.0.14/script.js").then(
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
      let p = null;
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
 * @typedef {Awaited<ReturnType<typeof packages.signals>>} Signals
 * @typedef {Awaited<ReturnType<typeof packages.lightweightCharts>>} LightweightCharts
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
     * @template T
     * @param {T[]} array
     */
    getRandomIndex(array) {
      return Math.floor(Math.random() * array.length);
    },
    /**
     * @template T
     * @param {T[]} array
     */
    getRandomElement(array) {
      return array[this.getRandomIndex(array)];
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
     * @param {Object} args
     * @param {string} args.inputName
     * @param {string} args.inputId
     * @param {string} args.inputValue
     * @param {boolean} [args.inputChecked=false]
     * @param {string} args.labelTitle
     * @param {(event: MouseEvent) => void} [args.onClick]
     */
    createLabeledInput({
      inputId,
      inputName,
      inputValue,
      inputChecked = false,
      labelTitle,
      onClick,
    }) {
      const label = window.document.createElement("label");

      const input = window.document.createElement("input");
      input.type = "radio";
      input.name = inputName;
      input.id = inputId;
      input.value = inputValue;
      input.checked = inputChecked;
      label.append(input);

      label.id = `${inputId}-label`;
      // @ts-ignore
      label.for = inputId;
      label.title = labelTitle;

      if (onClick) {
        label.addEventListener("click", onClick);
      }

      return {
        label,
        input,
      };
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {string} args.inputName
     * @param {string} args.inputId
     * @param {string} args.inputValue
     * @param {string} args.labelTitle
     * @param {string} [args.href]
     * @param {(event: MouseEvent) => void} args.onClick
     */
    createComplexLabeledInput({
      inputId,
      inputName,
      inputValue,
      labelTitle,
      name,
      onClick,
      href,
    }) {
      const { label, input } = this.createLabeledInput({
        inputId,
        inputName,
        inputValue,
        labelTitle,
        onClick,
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = this.createSpanName(name);

      if (href) {
        const anchor = window.document.createElement("a");
        anchor.href = href;
        anchor.append(spanName);
        spanMain.append(anchor);

        if (href.includes(".")) {
          anchor.target = "_target";
          anchor.rel = "noopener noreferrer";

          anchor.addEventListener("click", (event) => {
            event.stopPropagation();
            // event.preventDefault();
          });
        } else {
          anchor.addEventListener("click", (event) => {
            // event.stopPropagation();
            event.preventDefault();
          });
        }
      } else {
        spanMain.append(spanName);
      }

      return {
        label,
        input,
        spanMain,
        spanName,
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
    createField({ title, id, choices, selected, signals }) {
      const field = window.document.createElement("div");
      field.classList.add("field");

      const legend = window.document.createElement("legend");
      if (typeof title === "string") {
        legend.innerHTML = title;
      } else {
        signals.createEffect(() => {
          legend.innerHTML = title();
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
        });

        const text = window.document.createTextNode(choice);
        label.append(text);
        div.append(label);
      });

      return field;
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
     * @param {string | boolean | undefined} value
     */
    writeParam(key, value) {
      const urlParams = new URLSearchParams(window.location.search);

      if (value !== undefined) {
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
      const urlParams = new URLSearchParams(window.location.search);

      const parameter = urlParams.get(key);

      if (parameter) {
        return utils.isSerializedBooleanTrue(parameter);
      }

      return null;
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

      // value = absoluteValue;

      if (isNaN(value)) {
        return "";
        // } else if (value === 0) {
        //   return "0";
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
      } else if (absoluteValue >= 1_000_000_000_000_000_000) {
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
    readBool(key) {
      const saved = localStorage.getItem(key);
      if (saved) {
        return utils.isSerializedBooleanTrue(saved);
      }
      return null;
    },
    /**
     * @param {string} key
     * @param {string | boolean | undefined} value
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
   * @param {Date} date
   * @returns {string}
   */
  dateToString(date) {
    return date.toJSON().split("T")[0];
  },
  /**
   * @param {Time} time
   */
  dateFromTime(time) {
    return typeof time === "string"
      ? new Date(time)
      : // @ts-ignore
        new Date(time.year, time.month, time.day);
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
  const FIVE_SECOND_IN_MS = 5 * ONE_SECOND_IN_MS;
  const TEN_SECOND_IN_MS = 2 * FIVE_SECOND_IN_MS;
  const ONE_MINUTE_IN_MS = 6 * TEN_SECOND_IN_MS;
  const FIVE_MINUTES_IN_MS = 5 * ONE_MINUTE_IN_MS;
  const TEN_MINUTES_IN_MS = 2 * FIVE_MINUTES_IN_MS;
  const ONE_HOUR_IN_MS = 6 * TEN_MINUTES_IN_MS;
  const ONE_DAY_IN_MS = 24 * ONE_HOUR_IN_MS;

  const HEIGHT_CHUNK_SIZE = 10_000;

  const MEDIUM_WIDTH = 768;

  return {
    ONE_SECOND_IN_MS,
    FIVE_SECOND_IN_MS,
    TEN_SECOND_IN_MS,
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
  selectedFrameSelectorLabel: `selected-frame-selector-label`,
  foldersFilter: "folders-filter",
  chartRange: "chart-range",
  from: "from",
  to: "to",
  checkedFrameSelectorLabel: "checked-frame-selector-label",
  /**
   * @param {TimeScale} scale
   */
  visibleTimeRange(scale) {
    return `${ids.chartRange}-${scale}`;
  },
  /**
   * @param {string} s
   */
  fromString(s) {
    return s.replace(/\W/g, " ").trim().replace(/ +/g, "-").toLowerCase();
  },
});
/** @typedef {typeof ids} Ids */

const elements = {
  head: window.document.getElementsByTagName("head")[0],
  body: window.document.body,
  main: utils.dom.getElementById("main"),
  aside: utils.dom.getElementById("aside"),
  selectedLabel: utils.dom.getElementById(ids.selectedFrameSelectorLabel),
  foldersLabel: utils.dom.getElementById(`folders-frame-selector-label`),
  searchLabel: utils.dom.getElementById(`search-frame-selector-label`),
  searchFrame: utils.dom.getElementById("search-frame"),
  historyFrame: utils.dom.getElementById("history-frame"),
  settingsFrame: utils.dom.getElementById("settings-frame"),
  foldersFrame: utils.dom.getElementById("folders-frame"),
  selectedFrame: utils.dom.getElementById("selected-frame"),
  historyList: utils.dom.getElementById("history-list"),
  searchInput: /** @type {HTMLInputElement} */ (
    utils.dom.getElementById("search-input")
  ),
  searchSmall: utils.dom.getElementById("search-small"),
  searchResults: utils.dom.getElementById("search-results"),
  selectedTitle: utils.dom.getElementById("selected-title"),
  selectedDescription: utils.dom.getElementById("selected-description"),
  selectors: utils.dom.getElementById("frame-selectors"),
  foldersFilterAllCount: utils.dom.getElementById("folders-filter-all-count"),
  foldersFilterFavoritesCount: utils.dom.getElementById(
    "folders-filter-favorites-count",
  ),
  foldersFilterNewCount: utils.dom.getElementById("folders-filter-new-count"),
  chartList: utils.dom.getElementById("chart-list"),
  legend: utils.dom.getElementById("legend"),
  style: getComputedStyle(window.document.documentElement),
  buttonFavorite: utils.dom.getElementById("button-favorite"),
  timeScaleDateButtons: utils.dom.getElementById("timescale-date-buttons"),
  timeScaleHeightButtons: utils.dom.getElementById("timescale-height-buttons"),
  selectedHeader: utils.dom.getElementById("selected-header"),
  selectedHr: utils.dom.getElementById("selected-hr"),
  home: utils.dom.getElementById("home"),
  charts: utils.dom.getElementById("charts"),
  simulation: utils.dom.getElementById("simulation"),
};
/** @typedef {typeof elements} Elements */

const savedSelectedId = localStorage.getItem(ids.selectedId);
const isFirstTime = !savedSelectedId;
const urlSelected = utils.url.pathnameToSelectedId();

function initFrameSelectors() {
  let selectedFrameLabel = localStorage.getItem(ids.checkedFrameSelectorLabel);

  const selectors = elements.selectors;

  const children = Array.from(selectors.children);

  /** @type {HTMLElement | undefined} */
  let focusedSection = undefined;

  for (let i = 0; i < children.length; i++) {
    const element = children[i];

    switch (element.tagName) {
      case "LABEL": {
        element.addEventListener("click", () => {
          const id = element.id;

          selectedFrameLabel = id;
          localStorage.setItem(ids.checkedFrameSelectorLabel, id);

          const sectionId = element.id.split("-").splice(0, 2).join("-"); // Remove -selector

          const section = window.document.getElementById(sectionId);

          if (!section) {
            console.log(sectionId, section);
            throw "Section should exist";
          }

          if (section === focusedSection) {
            return;
          }

          section.hidden = false;
          if (focusedSection) {
            focusedSection.hidden = true;
          }
          focusedSection = section;
        });
        break;
      }
    }
  }

  if (selectedFrameLabel && (!urlSelected || urlSelected === savedSelectedId)) {
    const frameLabel = window.document.getElementById(selectedFrameLabel);
    if (!frameLabel) throw "Frame should exist";
    frameLabel.click();
  } else {
    elements.selectedLabel.click();
  }

  // When going from mobile view to desktop view, if selected frame was open, go to the folders frame
  new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (
        !entries[i].isIntersecting &&
        entries[i].target === elements.selectedLabel &&
        selectedFrameLabel === ids.selectedFrameSelectorLabel
      ) {
        elements.foldersLabel.click();
      }
    }
  }).observe(elements.selectedLabel);

  function setSelectedFrameParent() {
    const { clientWidth } = window.document.documentElement;
    if (clientWidth >= consts.MEDIUM_WIDTH) {
      elements.aside.append(elements.selectedFrame);
    } else {
      elements.main.append(elements.selectedFrame);
    }
  }
  setSelectedFrameParent();
  window.addEventListener("resize", setSelectedFrameParent);
}
initFrameSelectors();

function createKeyDownEventListener() {
  window.document.addEventListener("keydown", (event) => {
    switch (event.key) {
      case "Escape": {
        event.stopPropagation();
        event.preventDefault();
        elements.foldersLabel.click();
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
  function lightRed() {
    const tailwindRed300 = "#fca5a5";
    const tailwindRed800 = "#991b1b";
    return dark() ? tailwindRed300 : tailwindRed800;
  }
  function red() {
    return "#e63636"; // 550
  }
  function darkRed() {
    const tailwindRed900 = "#7f1d1d";
    const tailwindRed100 = "#fee2e2";
    return dark() ? tailwindRed900 : tailwindRed100;
  }
  function orange() {
    return elements.style.getPropertyValue("--orange"); // 550
  }
  function darkOrange() {
    const tailwindOrange900 = "#7c2d12";
    const tailwindOrange100 = "#ffedd5";
    return dark() ? tailwindOrange900 : tailwindOrange100;
  }
  function amber() {
    return "#e78a05"; // 550
  }
  function yellow() {
    return "#db9e03"; // 550
  }
  function lime() {
    return "#74b713"; // 550
  }
  function green() {
    return "#1cb454";
  }
  function darkGreen() {
    const tailwindGreen900 = "#14532d";
    const tailwindGreen100 = "#dcfce7";
    return dark() ? tailwindGreen900 : tailwindGreen100;
  }
  function emerald() {
    return "#0ba775";
  }
  function darkEmerald() {
    const tailwindEmerald900 = "#064e3b";
    const tailwindEmerald100 = "#d1fae5";
    return dark() ? tailwindEmerald900 : tailwindEmerald100;
  }
  function teal() {
    return "#10a697"; // 550
  }
  function cyan() {
    return "#06a3c3"; // 550
  }
  function sky() {
    return "#0794d8"; // 550
  }
  function blue() {
    return "#2f73f1"; // 550
  }
  function indigo() {
    return "#5957eb";
  }
  function violet() {
    return "#834cf2";
  }
  function purple() {
    return "#9d45f0";
  }
  function fuchsia() {
    return "#cc37e1";
  }
  function pink() {
    return "#e53882";
  }
  function rose() {
    return "#ea3053";
  }
  function darkRose() {
    const tailwindRose900 = "#881337";
    const tailwindRose100 = "#ffe4e6";
    return dark() ? tailwindRose900 : tailwindRose100;
  }

  function off() {
    const _ = dark();
    return elements.style.getPropertyValue("--off-color");
  }

  function textColor() {
    const _ = dark();
    return elements.style.getPropertyValue("--color");
  }

  return {
    default: textColor,
    off,
    lightBitcoin: yellow,
    bitcoin: orange,
    darkBitcoin: darkOrange,
    lightDollars: lime,
    dollars: emerald,
    darkDollars: darkEmerald,

    yellow,
    orange,
    red,

    _1d: lightRed,
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
    darkLoss: darkRed,
    profit: green,
    darkProfit: darkGreen,
    thermoCap: green,
    investorCap: rose,
    realizedCap: orange,
    darkLiveliness: darkRose,
    liveliness: rose,
    vaultedness: green,
    activityToVaultednessRatio: violet,
    up_to_1d: lightRed,
    up_to_1w: red,
    up_to_1m: orange,
    up_to_2m: orange,
    up_to_3m: orange,
    up_to_4m: orange,
    up_to_5m: orange,
    up_to_6m: orange,
    up_to_1y: orange,
    up_to_2y: orange,
    up_to_3y: orange,
    up_to_4y: orange,
    up_to_5y: orange,
    up_to_7y: orange,
    up_to_10y: orange,
    up_to_15y: orange,
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
    momentum: [green, yellow, red],
    momentumGreen: green,
    momentumYellow: yellow,
    momentumRed: red,
    probability0_1p: red,
    probability0_5p: orange,
    probability1p: yellow,
    year_2009: yellow,
    year_2010: yellow,
    year_2011: yellow,
    year_2012: yellow,
    year_2013: yellow,
    year_2014: yellow,
    year_2015: yellow,
    year_2016: yellow,
    year_2017: yellow,
    year_2018: yellow,
    year_2019: yellow,
    year_2020: yellow,
    year_2021: yellow,
    year_2022: yellow,
    year_2023: yellow,
    year_2024: yellow,
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
  function getOrImport(scale, path) {
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
    getOrImport,
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
   * @returns
   */
  function krakenCandleWebSocketCreator(callback) {
    const ws = new WebSocket("wss://ws.kraken.com");

    ws.addEventListener("open", () => {
      ws.send(
        JSON.stringify({
          event: "subscribe",
          pair: ["XBT/USD"],
          subscription: {
            name: "ohlc",
            interval: 1440,
          },
        }),
      );
    });

    ws.addEventListener("message", (message) => {
      const result = JSON.parse(message.data);

      if (!Array.isArray(result)) return;

      const [timestamp, _, open, high, low, close, __, volume] = result[1];

      const date = new Date(Number(timestamp) * 1000);

      const dateStr = utils.dateToString(date);

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

  const krakenCandle = createWebsocket(krakenCandleWebSocketCreator);

  krakenCandle.open();

  function createDocumentTitleEffect() {
    signals.createEffect(() => {
      const latest = krakenCandle.latest();

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
    krakenCandle,
  };
}
/** @typedef {ReturnType<typeof initWebSockets>} WebSockets */

packages.signals().then((signals) =>
  options.then(({ initOptions }) => {
    const dark = signals.createSignal(true);

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
      setInterval(fetchLastHeight, consts.TEN_SECOND_IN_MS, {});

      return lastHeight;
    }
    const lastHeight = createLastHeightResource();

    const lastValues = signals.createSignal(
      /** @type {Record<LastPath, number> | null} */ (null),
    );
    function createFetchLastValuesWhenNeededEffect() {
      let previousHeight = -1;
      signals.createEffect(() => {
        if (previousHeight !== lastHeight()) {
          fetch("/api/last").then((response) => {
            response.json().then((json) => {
              if (typeof json === "object") {
                lastValues.set(json);
                previousHeight = lastHeight();
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
    });

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

          signals.createEffect(() => {
            const option = options.selected();

            signals.untrack(() => {
              if (previousElement) {
                previousElement.hidden = true;
                utils.url.resetParams(option);
                utils.url.pushHistory(option.id);
              } else {
                utils.url.replaceHistory({ pathname: option.id });
              }

              const hideTop = option.kind === "home" || option.kind === "pdf";
              elements.selectedHeader.hidden = hideTop;
              elements.selectedHr.hidden = hideTop;

              elements.selectedTitle.innerHTML = option.title;
              elements.selectedDescription.innerHTML = option.serializedPath;

              /** @type {HTMLElement} */
              let element;

              switch (option.kind) {
                case "home": {
                  element = elements.home;
                  break;
                }
                case "chart": {
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
                              consts,
                              dark,
                              datasets,
                              elements,
                              ids,
                              lightweightCharts,
                              options,
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
                              consts,
                              dark,
                              datasets,
                              elements,
                              ids,
                              lightweightCharts,
                              options,
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
                  firstSimulationOption = false;

                  break;
                }
                default: {
                  return;
                }
              }

              element.hidden = false;
              previousElement = element;
            });
          });
        }
        createApplyOptionEffect();

        function initFavoriteButton() {
          elements.buttonFavorite.addEventListener("click", () => {
            const option = options.selected();

            option.isFavorite.set((f) => {
              const newState = !f;

              const localStorageKey = options.optionToFavoriteKey(option);
              if (newState) {
                localStorage.setItem(localStorageKey, "1");
              } else {
                localStorage.removeItem(localStorageKey);
              }

              return newState;
            });
          });

          signals.createEffect(() => {
            if (options.selected().isFavorite()) {
              elements.buttonFavorite.dataset.highlight = "";
            } else {
              delete elements.buttonFavorite.dataset.highlight;
            }
          });
        }
        initFavoriteButton();

        function initShareButton() {
          const shareDiv = utils.dom.getElementById("share-div");
          const shareContentDiv = utils.dom.getElementById("share-content-div");

          shareDiv.addEventListener("click", () => {
            shareDiv.hidden = true;
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

            utils.dom
              .getElementById("button-share")
              .addEventListener("click", () => {
                const href = window.location.href;
                anchor.href = href;
                anchor.innerHTML = href;

                const qrcode = generate(
                  /** @type {any} */ (window.document.location.href),
                )?.toDataURL({
                  // @ts-ignore
                  padX: 0,
                  padY: 0,
                });
                imgQrcode.src = qrcode || "";

                shareDiv.hidden = false;
              });
          });
        }
        initShareButton();
      }

      function createMobileSwitchEffect() {
        let firstRun = true;
        signals.createEffect(() => {
          options.selected();

          if (!firstRun && !utils.dom.isHidden(elements.selectedLabel)) {
            elements.selectedLabel.click();
          }
          firstRun = false;
        });
      }
      createMobileSwitchEffect();

      utils.dom.onFirstIntersection(elements.selectedFrame, initSelectedFrame);
    }
    initSelected();

    function initFolders() {
      function initTreeElement() {
        options.treeElement.set(() => {
          const treeElement = window.document.createElement("div");
          treeElement.classList.add("tree");
          elements.foldersFrame.append(treeElement);
          return treeElement;
        });
      }

      function createCountersDomUpdateEffect() {
        elements.foldersFilterAllCount.innerHTML =
          options.list.length.toLocaleString();

        signals.createEffect(() => {
          elements.foldersFilterFavoritesCount.innerHTML = options.counters
            .favorites()
            .toLocaleString();
        });

        signals.createEffect(() => {
          elements.foldersFilterNewCount.innerHTML = options.counters
            .new()
            .toLocaleString();
        });
      }

      function initFilters() {
        const filterAllInput = /** @type {HTMLInputElement} */ (
          utils.dom.getElementById("folders-filter-all")
        );
        const filterFavoritesInput = /** @type {HTMLInputElement} */ (
          utils.dom.getElementById("folders-filter-favorites")
        );
        const filterNewInput = /** @type {HTMLInputElement} */ (
          utils.dom.getElementById("folders-filter-new")
        );

        filterAllInput.addEventListener("change", () => {
          options.filter.set("all");
        });
        filterFavoritesInput.addEventListener("change", () => {
          options.filter.set("favorites");
        });
        filterNewInput.addEventListener("change", () => {
          options.filter.set("new");
        });

        signals.createEffect(() => {
          const f = options.filter();
          localStorage.setItem(ids.foldersFilter, f);
          switch (f) {
            case "all": {
              filterAllInput.checked = true;
              break;
            }
            case "favorites": {
              filterFavoritesInput.checked = true;
              break;
            }
            case "new": {
              filterNewInput.checked = true;
              break;
            }
          }
        });
      }

      function initCloseAllButton() {
        utils.dom
          .getElementById("button-close-all-folders")
          .addEventListener("click", () => {
            options.details.forEach((details) => (details.open = false));
          });
      }

      function initScrollToSelectedButton() {
        utils.dom
          .getElementById("scroll-go-to-selected")
          .addEventListener("click", () => {
            scrollToSelected();
          });
      }

      async function scrollToSelected() {
        options.filter.set("all");

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

        utils.dom
          .getElementById(`${selectedId}-folders-selector`)
          .scrollIntoView({
            behavior: "instant",
            block: "center",
          });
      }

      utils.dom.onFirstIntersection(elements.foldersFrame, () => {
        console.log("folders: init");
        initTreeElement();
        createCountersDomUpdateEffect();
        initFilters();
        initCloseAllButton();
        initScrollToSelectedButton();
        if (isFirstTime) {
          scrollToSelected();
        }
      });
    }
    initFolders();

    function initSearch() {
      function initNoInputButton() {
        utils.dom
          .getElementById("search-no-input-text-button")
          .addEventListener("click", () => {
            options.selected.set(utils.array.getRandomElement(options.list));
          });
      }

      /**
       * @param {string} [value = '']
       */
      function setInputValue(value = "") {
        elements.searchInput.focus();
        elements.searchInput.value = value;
        elements.searchInput.dispatchEvent(new Event("input"));
      }

      function initResetSearchButton() {
        const resetSearchButton = utils.dom.getElementById("reset-search");
        resetSearchButton.addEventListener("click", () => {
          setInputValue();
        });
      }

      function initSearchFrame() {
        console.log("search: init");
        initNoInputButton();
        initResetSearchButton();

        const localStorageSearchKey = "search";

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

              if (info && order) {
                for (let i = minIndex; i <= maxIndex; i++) {
                  let infoIdx = order[i];

                  const [title, path] = ufuzzy
                    .highlight(
                      haystack[info.idx[infoIdx]],
                      info.ranges[infoIdx],
                    )
                    .split("\t");

                  list[i % 100] = {
                    option: options.list[info.idx[infoIdx]],
                    path,
                    title,
                  };
                }
              } else {
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

              utils.storage.write(localStorageSearchKey, needle);

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

                const label = options.createOptionLabeledInput({
                  option,
                  frame: "search",
                  name: title,
                  top: path,
                });

                li.append(label);
              });
            });
          }

          if (elements.searchInput.value) {
            inputEvent();
          }

          elements.searchInput.addEventListener("input", inputEvent);
        });

        setInputValue(localStorage.getItem(localStorageSearchKey) || "");
      }
      utils.dom.onFirstIntersection(elements.searchFrame, initSearchFrame);
    }
    initSearch();

    function initHistory() {
      const LOCAL_STORAGE_HISTORY_KEY = "history";
      const MAX_HISTORY_LENGTH = 1_000;

      const history = /** @type {SerializedHistory} */ (
        JSON.parse(localStorage.getItem(LOCAL_STORAGE_HISTORY_KEY) || "[]")
      ).flatMap(([optionId, timestamp]) => {
        const option = options.list.find((option) => option.id === optionId);
        return option ? [{ option, date: new Date(timestamp) }] : [];
      });

      /** @param {Date} date  */
      function dateToTestedString(date) {
        return date.toLocaleString().split(",")[0];
      }

      function createUnshiftHistoryEffect() {
        signals.createEffect(() => {
          const option = options.selected();

          const head = history.at(0);
          if (
            head &&
            head.option === option &&
            dateToTestedString(new Date()) === dateToTestedString(head.date)
          ) {
            return;
          }

          history.unshift({
            date: new Date(),
            option,
          });

          utils.runWhenIdle(() => {
            /** @type {SerializedHistory} */
            const serializedHistory = history.map(({ option, date }) => [
              option.id,
              date.getTime(),
            ]);

            if (serializedHistory.length > MAX_HISTORY_LENGTH) {
              serializedHistory.length = MAX_HISTORY_LENGTH;
            }

            const jsonHistory = JSON.stringify(serializedHistory);

            localStorage.setItem(LOCAL_STORAGE_HISTORY_KEY, jsonHistory);
          });
        });
      }
      createUnshiftHistoryEffect();

      function initHistoryFrame() {
        console.log("history: init");

        const owner = signals.getOwner();

        /** @param {Date} date  */
        function dateToDisplayedString(date) {
          const formattedDate = dateToTestedString(date);

          const now = new Date();
          if (dateToTestedString(now) === formattedDate) {
            return "Today";
          }

          now.setUTCDate(now.getUTCDate() - 1);

          if (dateToTestedString(now) === formattedDate) {
            return "Yesterday";
          }

          return date.toLocaleDateString(undefined, {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
          });
        }

        const grouped = history.reduce((grouped, { option, date }) => {
          grouped[dateToTestedString(date)] ||= [];
          grouped[dateToTestedString(date)].push({ option, date });
          return grouped;
        }, /** @type {Record<string, {option: Option, date: Date}[]>} */ ({}));

        /** @type {[string|undefined, string|undefined]} */
        const firstTwo = [undefined, undefined];

        function initHistoryListInDom() {
          Object.entries(grouped).forEach(([key, tuples], index) => {
            if (index < 2) {
              firstTwo[index] = key;
            }

            const heading = window.document.createElement("h4");
            heading.id = key;
            heading.innerHTML = dateToDisplayedString(tuples[0].date);
            elements.historyList.append(heading);

            tuples.forEach(({ option, date }) => {
              elements.historyList.append(
                options.createOptionLabeledInput({
                  option,
                  frame: "history",
                  name: option.title,
                  id: date.valueOf().toString(),
                  top: date.toLocaleTimeString(),
                  owner,
                }),
              );
            });
          });
        }
        initHistoryListInDom();

        function createUpdateHistoryEffect() {
          signals.createEffect(() => {
            const option = options.selected();
            const date = new Date();
            const testedString = dateToTestedString(date);

            const label = options.createOptionLabeledInput({
              option,
              frame: "history",
              name: option.title,
              id: date.valueOf().toString(),
              top: date.toLocaleTimeString(),
              owner,
            });

            const li = window.document.createElement("li");
            li.append(label);

            if (testedString === firstTwo[0]) {
              if (options.selected() === grouped[testedString].at(0)?.option) {
                return;
              }

              grouped[testedString].unshift({ option, date });
              utils.dom.getElementById(testedString).after(li);
            } else {
              const [first, second] = firstTwo;
              /** @param {string | undefined} id  */
              function updateHeading(id) {
                if (!id) return;
                utils.dom.getElementById(id).innerHTML = dateToDisplayedString(
                  grouped[id][0].date,
                );
              }

              updateHeading(first);
              updateHeading(second);

              const heading = window.document.createElement("h4");
              heading.innerHTML = dateToDisplayedString(date);
              heading.id = testedString;

              elements.historyList.prepend(li);
              elements.historyList.prepend(heading);

              grouped[testedString] = [{ option, date }];

              firstTwo[1] = firstTwo[0];
              firstTwo[0] = testedString;
            }
          });
        }
        createUpdateHistoryEffect();
      }
      utils.dom.onFirstIntersection(elements.historyFrame, initHistoryFrame);
    }
    initHistory();

    function initSettings() {
      function initSettingsFrame() {
        console.log("settings: init");

        function initTheme() {
          const inputLight = /** @type {HTMLInputElement} */ (
            utils.dom.getElementById("settings-theme-light-input")
          );
          const inputDark = /** @type {HTMLInputElement} */ (
            utils.dom.getElementById("settings-theme-dark-input")
          );
          const inputSystem = /** @type {HTMLInputElement} */ (
            utils.dom.getElementById("settings-theme-system-input")
          );

          const settingsThemeLocalStorageKey = "settings-theme";

          let savedTheme = /** @type {SettingsTheme} */ (
            localStorage.getItem(settingsThemeLocalStorageKey)
          );

          switch (savedTheme) {
            case "dark": {
              inputDark.checked = true;
              break;
            }
            case "light": {
              inputLight.checked = true;
              break;
            }
            default:
            case "system": {
              inputSystem.checked = true;
              savedTheme = "system";
              break;
            }
          }

          const theme = signals.createSignal(savedTheme);

          const preferredColorSchemeMatchMedia = window.matchMedia(
            "(prefers-color-scheme: dark)",
          );

          /**
           * @param {boolean} shouldBeDark
           */
          function updateTheme(shouldBeDark) {
            dark.set(shouldBeDark);

            if (shouldBeDark) {
              window.document.documentElement.dataset.theme = "dark";
            } else {
              delete window.document.documentElement.dataset.theme;
            }

            const backgroundColor = getComputedStyle(
              window.document.documentElement,
            ).getPropertyValue("--background-color");
            const meta = utils.dom.queryOrCreateMetaElement("theme-color");
            meta.content = backgroundColor;
          }

          function createUpdateDataThemeEffect() {
            signals.createEffect(() => {
              localStorage.setItem(settingsThemeLocalStorageKey, theme());
              updateTheme(
                theme() === "dark" ||
                  (theme() === "system" &&
                    preferredColorSchemeMatchMedia.matches),
              );
            });
          }
          createUpdateDataThemeEffect();

          preferredColorSchemeMatchMedia.addEventListener("change", (media) => {
            if (theme() === "system") {
              updateTheme(media.matches);
            }
          });

          utils.dom
            .getElementById("settings-theme-field")
            .addEventListener("change", (event) => {
              const newTheme = /** @type {SettingsTheme | string} */ (
                // @ts-ignore
                event.target?.value
              );
              switch (newTheme) {
                case "dark":
                case "light":
                case "system": {
                  theme.set(newTheme);
                  break;
                }
                default: {
                  throw "Bad theme";
                }
              }
            });
        }
        initTheme();

        function initLeaderboard() {
          const leaderboard = utils.dom.getElementById("leaderboard");

          const donations = [
            {
              name: "_Checkɱate",
              // url: "https://xcancel.com/_Checkmatey_",
              url: "https://primal.net/p/npub1qh5sal68c8swet6ut0w5evjmj6vnw29x3k967h7atn45unzjyeyq6ceh9r",
              amount: 500_000,
            },
            {
              name: "avvi |",
              url: "https://primal.net/p/npub1md2q6fexrtmd5hx9gw2p5640vg662sjlpxyz3tdmu4j4g8hhkm6scn6hx3",
              amount: 5_000,
            },
            {
              name: "mutatrum",
              url: "https://primal.net/p/npub1hklphk7fkfdgmzwclkhshcdqmnvr0wkfdy04j7yjjqa9lhvxuflsa23u2k",
              amount: 5_000,
            },
            {
              name: "Gunnar",
              url: "https://primal.net/p/npub1rx9wg2d5lhah45xst3580sajcld44m0ll9u5dqhu2t74p6xwufaqwghtd4",
              amount: 1_000,
            },
            {
              name: "Blokchain Boog",
              url: "https://xcancel.com/BlokchainB",
              amount: 1_500 + 1590,
            },
            {
              name: "Josh",
              url: "https://primal.net/p/npub1pc57ls4rad5kvsp733suhzl2d4u9y7h4upt952a2pucnalc59teq33dmza",
              amount: 1_000,
            },
            {
              name: "Alp",
              url: "https://primal.net/p/npub175nul9cvufswwsnpy99lvyhg7ad9nkccxhkhusznxfkr7e0zxthql9g6w0",
              amount: 1_000,
            },
            {
              name: "Ulysses",
              url: "https://primal.net/p/npub1n7n3dssm90hfsfjtamwh2grpzwjlvd2yffae9pqgg99583lxdypsnn9gtv",
              amount: 1_000,
            },
            {
              name: "btcschellingpt",
              url: "https://primal.net/p/npub1nvfgglea9zlcs58tcqlc6j26rt50ngkgdk7699wfq4txrx37aqcsz4e7zd",
              amount: 1_000 + 1_000,
            },
            {
              name: "Coinatra",
              url: "https://primal.net/p/npub1eut9kcejweegwp9waq3a4g03pvprdzkzvjjvl8fvj2a2wlx030eswzfna8",
              amount: 1_000,
            },
            {
              name: "Printer Go Brrrr",
              url: "https://primal.net/p/npub1l5pxvjzhw77h86tu0sml2gxg8jpwxch7fsj6d05n7vuqpq75v34syk4q0n",
              amount: 1_000,
            },
            {
              name: "b81776c32d7b",
              url: "https://primal.net/p/npub1hqthdsed0wpg57sqsc5mtyqxxgrh3s7493ja5h49v23v2nhhds4qk4w0kz",
              amount: 17_509,
            },
            {
              name: "DerGigi",
              url: "https://primal.net/p/npub1dergggklka99wwrs92yz8wdjs952h2ux2ha2ed598ngwu9w7a6fsh9xzpc",
              amount: 6001,
            },
            {
              name: "Adarnit",
              url: "https://primal.net/p/npub17armdveqy42uhuuuwjc5m2dgjkz7t7epgvwpuccqw8jusm8m0g4sn86n3s",
              amount: 17_726,
            },
            {
              name: "Auburn Citadel",
              url: "https://primal.net/p/npub1730y5k2s9u82w9snx3hl37r8gpsrmqetc2y3xyx9h65yfpf28rtq0y635y",
              amount: 17_471,
            },
            {
              name: "anon",
              amount: 210_000,
            },
            {
              name: "Daniel ∞/21M",
              url: "https://twitter.com/DanielAngelovBG",
              amount: 21_000,
            },
            {
              name: "Ivo",
              url: "https://primal.net/p/npub1mnwjn40hr042rsmzu64rsnwsw07uegg4tjkv620c94p6e797wkvq3qeujc",
              amount: 5_000,
            },
            {
              name: "lassdas",
              url: "https://primal.net/p/npub1gmhctt2hmjqz8ay2x8h5f8fl3h4fpfcezwqneal3usu3u65qca4s8094ea",
              amount: 210_000,
            },
            {
              name: "anon",
              amount: 21_000,
            },
            {
              name: "xplbzx",
              url: "https://primal.net/p/npub1e0f808a350rxrhppu4zylzljt3arfpvrrpqdg6ft78xy6u49kq5slf0g92",
              amount: 12_110,
            },
            {
              name: "SoundMoney=Prosperity4ALL",
              url: "https://xcancel.com/SoundmoneyP",
              amount: 420_000,
            },
            {
              name: "Johan",
              url: "https://primal.net/p/npub1a4sd4cprrucfkvkfq9zs99ur4xe7lxw3uhhgvuzx6nqxhnpa2yyqlsa26u",
              amount: 500_000,
            },
            {
              name: "highperfocused",
              url: "https://primal.net/p/npub1fq8vrf63vsrqjrwqgtwlvauqauc0yme6se8g8dqhcpf6tfs3equqntmzut",
              amount: 4620,
            },
            {
              name: "ClearMined",
              url: "https://primal.net/p/npub1dj8zwktp3eyktfhs5mjlw8v0v2838xlquxr7ddsanayhcw98fcks8ddrq9",
              amount: 300_000,
            },
          ];

          donations.sort((a, b) =>
            b.amount !== a.amount
              ? b.amount - a.amount
              : a.name.localeCompare(b.name),
          );

          donations.slice(0, 21).forEach(({ name, url, amount }) => {
            const li = window.document.createElement("li");
            leaderboard.append(li);

            const a = window.document.createElement("a");
            a.href = url || "";
            a.target = "_blank";
            a.rel = "noopener noreferrer";
            a.innerHTML = name;
            li.append(a);

            li.append(" — ");

            const small = window.document.createElement("small");
            small.classList.add("sats");
            small.innerHTML = `${amount.toLocaleString("en-us")} sats`;
            li.append(small);
          });
        }
        initLeaderboard();

        function initInstallInstructions() {
          if (
            !env.standalone &&
            env.safariOnly &&
            (env.macOS || env.ipad || env.iphone)
          ) {
            const installInstructionsElement = utils.dom.getElementById(
              "settings-install-instructions",
            );
            installInstructionsElement.hidden = false;

            const hr = window.document.createElement("hr");
            installInstructionsElement.before(hr);

            const heading = window.document.createElement("h4");
            heading.innerHTML = "Install";
            installInstructionsElement.append(heading);

            const p = window.document.createElement("p");
            installInstructionsElement.append(p);

            if (env.macOS) {
              p.innerHTML = `This app can be installed by clicking on the <strong>File</strong> tab on the menu bar and then on <strong>Add to dock</strong>.`;
            } else {
              p.innerHTML = `This app can be installed by tapping on the <strong>Share</strong> button tab of Safari and then on <strong>Add to Home Screen</strong>.`;
            }
          }
        }
        initInstallInstructions();

        function initMobileNav() {
          const anchorApi = /** @type {HTMLAnchorElement} */ (
            utils.dom.getElementById("anchor-api").cloneNode(true)
          );

          const anchorGit = /** @type {HTMLAnchorElement} */ (
            utils.dom.getElementById("anchor-git").cloneNode(true)
          );

          const anchorNostr = /** @type {HTMLAnchorElement} */ (
            utils.dom.getElementById("anchor-nostr").cloneNode(true)
          );

          const anchorGeyser = /** @type {HTMLAnchorElement} */ (
            utils.dom.getElementById("anchor-geyser").cloneNode(true)
          );

          if (!anchorApi || !anchorGit || !anchorNostr || !anchorGeyser)
            throw "Anchors should exist by now";

          anchorApi.id = "";
          anchorGit.id = "";
          anchorNostr.id = "";
          anchorGeyser.id = "";

          const nav = utils.dom.getElementById("settings-nav");

          nav.append(anchorApi);
          nav.append(anchorGit);
          nav.append(anchorNostr);
          nav.append(anchorGeyser);
        }
        initMobileNav();
      }
      utils.dom.onFirstIntersection(elements.settingsFrame, initSettingsFrame);
    }
    initSettings();

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
