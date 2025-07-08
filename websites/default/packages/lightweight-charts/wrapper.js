// @ts-check

/** @import { IChartApi, ISeriesApi as _ISeriesApi, SeriesDefinition, SingleValueData as _SingleValueData, CandlestickData as _CandlestickData, BaselineData as _BaselineData, SeriesType, IPaneApi, LineSeriesPartialOptions, BaselineSeriesPartialOptions, CandlestickSeriesPartialOptions, WhitespaceData, DeepPartial, ChartOptions, Time, LineData as _LineData } from './v5.0.8/types' */

/**
 * @typedef {[number, number, number, number]} OHLCTuple
 *
 * @typedef {Object} Valued
 * @property {number} value
 *
 * @typedef {Object} Indexed
 * @property {number} index
 *
 * @typedef {_ISeriesApi<SeriesType, number>} ISeries
 * @typedef {_ISeriesApi<'Candlestick', number>} CandlestickISeries
 * @typedef {_ISeriesApi<'Line', number>} LineISeries
 * @typedef {_ISeriesApi<'Baseline', number>} BaselineISeries
 *
 * @typedef {Object} Series
 * @property {ISeries} inner
 * @property {string} id
 * @property {Signal<boolean>} active
 * @property {Signal<boolean>} hasData
 * @property {Signal<string | null>} url
 * @property {VoidFunction} remove
 *
 * @typedef {_SingleValueData<number>} SingleValueData
 * @typedef {_CandlestickData<number>} CandlestickData
 * @typedef {_LineData<number>} LineData
 * @typedef {_BaselineData<number>} BaselineData
 *
 * @typedef {function({ iseries: ISeries; unit: Unit; index: Index }): void} SetDataCallback
 *
 * @typedef {Object} CreatePriceLine
 * @property {number} value
 *
 * @typedef {Object} CreatePriceLineOptions
 * @property {CreatePriceLine} createPriceLine
 *
 * @typedef {Partial<CreatePriceLineOptions>} PartialCreatePriceLineOptions
 *
 * @typedef {LineSeriesPartialOptions & PartialCreatePriceLineOptions} PartialLineStyleOptions
 * @typedef {CandlestickSeriesPartialOptions & PartialCreatePriceLineOptions} PartialCandlestickStyleOptions
 * @typedef {BaselineSeriesPartialOptions & PartialCreatePriceLineOptions} PartialBaselineStyleOptions
 */

import {
  createChart,
  CandlestickSeries,
  LineSeries,
  BaselineSeries,
} from "./v5.0.8/script.js";

const oklchToRGBA = createOklchToRGBA();

/**
 * @param {Object} args
 * @param {string} args.id
 * @param {HTMLElement} args.parent
 * @param {Signals} args.signals
 * @param {Colors} args.colors
 * @param {Utilities} args.utils
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 * @param {Accessor<Index>} args.index
 * @param {((unknownTimeScaleCallback: VoidFunction) => void)} [args.timeScaleSetCallback]
 * @param {true} [args.fitContent]
 * @param {{unit: Unit; blueprints: AnySeriesBlueprint[]}[]} [args.config]
 */
function createChartElement({
  parent,
  signals,
  colors,
  utils,
  elements,
  id: chartId,
  index,
  vecsResources,
  timeScaleSetCallback,
  fitContent,
  config,
}) {
  const div = window.document.createElement("div");
  div.classList.add("chart");
  parent.append(div);

  const legendTop = createLegend({
    utils,
    signals,
  });
  div.append(legendTop.element);

  const chartDiv = window.document.createElement("div");
  chartDiv.classList.add("lightweight-chart");
  div.append(chartDiv);

  const legendBottom = createLegend({
    utils,
    signals,
  });
  div.append(legendBottom.element);

  /** @type {IChartApi} */
  const ichart = createChart(
    chartDiv,
    /** @satisfies {DeepPartial<ChartOptions>} */ ({
      autoSize: true,
      layout: {
        fontFamily: elements.style.fontFamily,
        background: { color: "transparent" },
        attributionLogo: false,
        colorSpace: "display-p3",
        colorParsers: [oklchToRGBA],
      },
      grid: {
        vertLines: { visible: false },
        horzLines: { visible: false },
      },
      rightPriceScale: {
        borderVisible: false,
      },
      timeScale: {
        borderVisible: false,
        ...(fitContent
          ? {
              minBarSpacing: 0.001,
            }
          : {}),
      },
      localization: {
        priceFormatter: numberToShortUSFormat,
        locale: "en-us",
      },
      crosshair: {
        mode: 3,
      },
      ...(fitContent
        ? {
            handleScale: false,
            handleScroll: false,
          }
        : {}),
      // ..._options,
    }),
  );

  ichart.priceScale("right").applyOptions({
    minimumWidth: 80,
  });

  ichart.panes().at(0)?.setStretchFactor(1);

  signals.createEffect(
    () => ({
      defaultColor: colors.default(),
      offColor: colors.gray(),
      borderColor: colors.border(),
    }),
    ({ defaultColor, offColor, borderColor }) => {
      ichart.applyOptions({
        layout: {
          textColor: offColor,
          panes: {
            separatorColor: borderColor,
          },
        },
        crosshair: {
          horzLine: {
            color: offColor,
            labelBackgroundColor: defaultColor,
          },
          vertLine: {
            color: offColor,
            labelBackgroundColor: defaultColor,
          },
        },
      });
    },
  );

  signals.createEffect(index, (index) => {
    const minBarSpacing =
      index === /** @satisfies {MonthIndex} */ (7)
        ? 1
        : index === /** @satisfies {QuarterIndex} */ (19)
          ? 2
          : index === /** @satisfies {YearIndex} */ (23)
            ? 6
            : index === /** @satisfies {DecadeIndex} */ (1)
              ? 60
              : 0.5;

    ichart.applyOptions({
      timeScale: {
        timeVisible:
          index === /** @satisfies {Height} */ (5) ||
          index === /** @satisfies {DifficultyEpoch} */ (2) ||
          index === /** @satisfies {HalvingEpoch} */ (4),
        ...(!fitContent
          ? {
              minBarSpacing,
            }
          : {}),
      },
    });
  });

  const activeResources = /** @type {Set<VecResource>} */ (new Set());
  ichart.subscribeCrosshairMove(
    utils.debounce(() => {
      activeResources.forEach((v) => {
        v.fetch();
      });
    }),
  );

  if (fitContent) {
    new ResizeObserver(() => ichart.timeScale().fitContent()).observe(chartDiv);
  }

  /**
   * @param {Object} args
   * @param {string} args.id
   * @param {number} args.paneIndex
   * @param {"nw" | "ne" | "se" | "sw"} args.position
   * @param {number} [args.timeout]
   * @param {(pane: IPaneApi<Time>) => HTMLElement} args.createChild
   */
  function addFieldsetIfNeeded({ paneIndex, id, position, createChild }) {
    const owner = signals.getOwner();

    setTimeout(
      () =>
        signals.runWithOwner(owner, () => {
          const parent = ichart
            ?.panes()
            .at(paneIndex)
            ?.getHTMLElement()
            ?.children?.item(1)?.firstChild;

          if (!parent) throw Error("Parent should exist");

          const children = Array.from(parent.childNodes).filter(
            (element) =>
              /** @type {HTMLElement} */ (element).dataset.position ===
              position,
          );

          if (children.length === 1) {
            children[0].remove();
          } else if (children.length > 1) {
            throw Error("Untraceable");
          }

          const fieldset = window.document.createElement("fieldset");
          fieldset.dataset.size = "xs";
          fieldset.dataset.position = position;
          fieldset.id = `${id}-${paneIndex}`;
          const pane = ichart.panes().at(paneIndex);
          if (!pane) throw Error("Expect pane");
          pane
            .getHTMLElement()
            ?.children?.item(1)
            ?.firstChild?.appendChild(fieldset);

          fieldset.append(createChild(pane));
        }),
      paneIndex ? 50 : 0,
    );
  }

  /**
   * @param {Object} args
   * @param {Unit} args.unit
   * @param {SeriesType} args.seriesType
   * @param {number} args.paneIndex
   */
  function addPriceScaleSelectorIfNeeded({ unit, paneIndex, seriesType }) {
    const id = `${chartId}-scale`;

    addFieldsetIfNeeded({
      id,
      paneIndex,
      position: "sw",
      createChild(pane) {
        const { field, selected } = utils.dom.createHorizontalChoiceField({
          choices: /** @type {const} */ (["lin", "log"]),
          id: utils.stringToId(`${id} ${paneIndex} ${unit}`),
          defaultValue:
            unit === "USD" && seriesType !== "Baseline" ? "log" : "lin",
          key: `${id}-price-scale-${paneIndex}`,
          signals,
        });

        signals.createEffect(selected, (selected) => {
          try {
            pane.priceScale("right").applyOptions({
              mode: selected === "lin" ? 0 : 1,
            });
          } catch {}
        });

        return field;
      },
    });
  }

  /**
   * @param {Object} args
   * @param {ISeries} args.iseries
   * @param {string} args.name
   * @param {Unit} args.unit
   * @param {number} args.order
   * @param {Color[]} args.colors
   * @param {SeriesType} args.seriesType
   * @param {VecId} [args.vecId]
   * @param {SetDataCallback} [args.setDataCallback]
   * @param {Accessor<WhitespaceData<number>[]>} [args.data]
   * @param {number} args.paneIndex
   * @param {boolean} [args.defaultActive]
   */
  function addSeries({
    iseries,
    vecId,
    name,
    unit,
    order,
    seriesType,
    setDataCallback,
    paneIndex,
    defaultActive,
    colors,
    data,
  }) {
    return signals.createRoot((dispose) => {
      const id = `${utils.stringToId(name)}-${paneIndex}`;

      const active = signals.createSignal(defaultActive ?? true, {
        save: {
          keyPrefix: "",
          key: id,
          ...utils.serde.boolean,
        },
      });

      const hasData = signals.createSignal(false);

      signals.createEffect(active, (active) =>
        // Or remove ?
        iseries.applyOptions({
          visible: active,
        }),
      );

      iseries.setSeriesOrder(order);

      /** @type {VecResource | undefined} */
      let _valuesResource;

      /** @type {Series} */
      const series = {
        inner: iseries,
        active,
        hasData,
        id,
        url: signals.createSignal(/** @type {string | null} */ (null)),
        remove() {
          dispose();
          // @ts-ignore
          chart.inner.removeSeries(iseries);
          if (_valuesResource) {
            activeResources.delete(_valuesResource);
          }
        },
      };

      if (vecId) {
        signals.createEffect(index, (index) => {
          const timeResource = vecsResources.getOrCreate(
            index,
            index === /** @satisfies {Height} */ (5)
              ? "timestamp_fixed"
              : "timestamp",
          );
          timeResource.fetch();

          const valuesResource = vecsResources.getOrCreate(index, vecId);
          _valuesResource = valuesResource;

          series.url.set(() => valuesResource.url);

          signals.createEffect(active, (active) => {
            if (active) {
              valuesResource.fetch();
              activeResources.add(valuesResource);

              const fetchedKey = vecsResources.defaultFetchedKey;
              signals.createEffect(
                () => ({
                  _indexes: timeResource.fetched().get(fetchedKey)?.vec(),
                  values: valuesResource.fetched().get(fetchedKey)?.vec(),
                }),
                ({ _indexes, values }) => {
                  if (!_indexes?.length || !values?.length) return;

                  const consoleTimeLabel = `${vecId}-time`;
                  console.time(consoleTimeLabel);

                  const indexes = /** @type {number[]} */ (_indexes);

                  let length = Math.min(indexes.length, values.length);

                  // TODO: Don't create new Array if data already present, update instead
                  const data = /** @type {LineData[] | CandlestickData[]} */ (
                    Array.from({ length })
                  );

                  let prevTime = null;
                  let timeOffset = 0;

                  for (let i = 0; i < length; i++) {
                    const time = indexes[i];
                    const sameTime = prevTime === time;
                    if (sameTime) {
                      timeOffset += 1;
                    }
                    const v = values[i];
                    const offsetedI = i - timeOffset;
                    if (v === null) {
                      data[offsetedI] = {
                        time,
                        value: NaN,
                      };
                    } else if (typeof v === "number") {
                      data[offsetedI] = {
                        time,
                        value: v,
                      };
                    } else {
                      if (sameTime) {
                        console.log(data[offsetedI]);
                      }
                      let [open, high, low, close] = v;
                      data[offsetedI] = {
                        time,
                        // @ts-ignore
                        open: sameTime ? data[offsetedI].open : open,
                        high: sameTime
                          ? // @ts-ignore
                            Math.max(data[offsetedI].high, high)
                          : high,
                        low: sameTime
                          ? // @ts-ignore
                            Math.min(data[offsetedI].low, low)
                          : low,
                        close,
                      };
                    }
                    prevTime = time;
                  }

                  data.length -= timeOffset;

                  hasData.set(true);

                  const seriesData = series.inner.data();
                  if (!seriesData.length) {
                    iseries.setData(data);

                    if (fitContent) {
                      ichart.timeScale().fitContent();
                    }

                    timeScaleSetCallback?.(() => {
                      if (
                        index === /** @satisfies {QuarterIndex} */ (19) ||
                        index === /** @satisfies {YearIndex} */ (23) ||
                        index === /** @satisfies {DecadeIndex} */ (1)
                      ) {
                        ichart.timeScale().setVisibleLogicalRange({
                          from: -1,
                          to: data.length,
                        });
                      }
                    });
                  } else {
                    const last = seriesData.at(-1);
                    if (!last) throw Error("Unreachable");
                    for (let i = 0; i < data.length; i++) {
                      if (data[i].time >= last.time) {
                        iseries.update(data[i]);
                      }
                    }
                  }

                  setDataCallback?.({
                    iseries,
                    index,
                    unit,
                  });

                  console.timeEnd(consoleTimeLabel);
                },
              );
            } else {
              activeResources.delete(valuesResource);
            }
          });
        });
      } else if (data) {
        signals.createEffect(data, (data) => {
          iseries.setData(data);
          hasData.set(true);
          setDataCallback?.({
            iseries,
            index: index(),
            unit,
          });

          if (fitContent) {
            ichart.timeScale().fitContent();
          }
        });
      }

      (paneIndex ? legendBottom : legendTop).addOrReplace({
        series,
        name,
        colors,
        order,
      });

      addPriceScaleSelectorIfNeeded({
        paneIndex,
        seriesType,
        unit,
      });

      return series;
    });
  }

  const chart = {
    inner: ichart,
    legendTop,
    legendBottom,

    addFieldsetIfNeeded,

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {VecId} [args.vecId]
     * @param {Accessor<CandlestickData[]>} [args.data]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {boolean} [args.inverse]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {PartialCandlestickStyleOptions} [args.options]
     */
    addCandlestickSeries({
      vecId,
      name,
      unit,
      order,
      paneIndex = 0,
      defaultActive,
      setDataCallback,
      data,
      inverse,
    }) {
      const green = inverse ? colors.red : colors.green;
      const red = inverse ? colors.green : colors.red;

      /** @type {CandlestickISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Candlestick'>} */ (CandlestickSeries),
          {
            upColor: green(),
            downColor: red(),
            wickUpColor: green(),
            wickDownColor: red(),
            borderVisible: false,
            visible: defaultActive !== false,
          },
          paneIndex,
        )
      );

      return addSeries({
        colors: [green, red],
        iseries,
        name,
        order,
        paneIndex,
        seriesType: "Candlestick",
        unit,
        data,
        setDataCallback,
        defaultActive,
        vecId,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Accessor<LineData[]>} [args.data]
     * @param {VecId} [args.vecId]
     * @param {Color} [args.color]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {PartialLineStyleOptions} [args.options]
     */
    addLineSeries({
      vecId,
      name,
      unit,
      order,
      setDataCallback,
      color,
      paneIndex = 0,
      defaultActive,
      data,
      options,
    }) {
      color ||= unit === "USD" ? colors.green : colors.orange;

      /** @type {LineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (LineSeries),
          {
            lineWidth: /** @type {any} */ (1.5),
            visible: defaultActive !== false,
            priceLineVisible: false,
            color: color(),
            ...options,
          },
          paneIndex,
        )
      );

      const priceLineOptions = options?.createPriceLine;
      if (priceLineOptions) {
        createPriceLine(iseries, priceLineOptions, colors);
      }

      return addSeries({
        colors: [color],
        iseries,
        name,
        order,
        paneIndex,
        seriesType: "Line",
        unit,
        setDataCallback,
        data,
        defaultActive,
        vecId,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Accessor<BaselineData[]>} [args.data]
     * @param {VecId} [args.vecId]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {PartialBaselineStyleOptions} [args.options]
     */
    addBaselineSeries({
      vecId,
      name,
      unit,
      order,
      paneIndex: _paneIndex,
      setDataCallback,
      defaultActive,
      data,
      options,
    }) {
      const paneIndex = _paneIndex ?? 0;

      /** @type {BaselineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Baseline'>} */ (BaselineSeries),
          {
            lineWidth: /** @type {any} */ (1.5),
            visible: defaultActive !== false,
            baseValue: {
              price: options?.createPriceLine?.value ?? 0,
            },
            ...options,
            topLineColor: options?.topLineColor ?? colors.green(),
            bottomLineColor: options?.bottomLineColor ?? colors.red(),
            priceLineVisible: false,
            bottomFillColor1: "transparent",
            bottomFillColor2: "transparent",
            topFillColor1: "transparent",
            topFillColor2: "transparent",
            lineVisible: true,
          },
          paneIndex,
        )
      );

      const priceLineOptions = options?.createPriceLine;
      if (priceLineOptions) {
        createPriceLine(iseries, priceLineOptions, colors);
      }

      return addSeries({
        colors: [
          () => options?.topLineColor ?? colors.green(),
          () => options?.bottomLineColor ?? colors.red(),
        ],
        iseries,
        name,
        order,
        paneIndex,
        seriesType: "Baseline",
        setDataCallback,
        unit,
        data,
        defaultActive,
        vecId,
      });
    },
  };

  config?.forEach(({ unit, blueprints }, paneIndex) => {
    blueprints.forEach((blueprint, order) => {
      if (blueprint.type === "Candlestick") {
        chart.addCandlestickSeries({
          name: blueprint.title,
          unit,
          data: blueprint.data,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          order,
        });
      } else if (blueprint.type === "Baseline") {
        chart.addBaselineSeries({
          name: blueprint.title,
          unit,
          data: blueprint.data,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          order,
        });
      } else {
        chart.addLineSeries({
          name: blueprint.title,
          unit,
          data: blueprint.data,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          color: blueprint.color,
          order,
        });
      }
    });
  });

  return chart;
}

/**
 * @param {Object} args
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 */
function createLegend({ signals, utils }) {
  const element = window.document.createElement("legend");

  const hovered = signals.createSignal(/** @type {Series | null} */ (null));

  /** @type {HTMLElement[]} */
  const legends = [];

  return {
    element,
    /**
     * @param {Object} args
     * @param {Series} args.series
     * @param {string} args.name
     * @param {number} args.order
     * @param {Color[]} args.colors
     */
    addOrReplace({ series, name, colors, order }) {
      const div = window.document.createElement("div");

      const prev = legends[order];
      if (prev) {
        prev.replaceWith(div);
      } else {
        const elementAtOrder = Array.from(element.children).at(order);
        if (elementAtOrder) {
          elementAtOrder.before(div);
        } else {
          element.append(div);
        }
      }
      legends[order] = div;

      const { input, label } = utils.dom.createLabeledInput({
        inputId: utils.stringToId(`legend-${series.id}`),
        inputName: utils.stringToId(`selected-${series.id}`),
        inputValue: "value",
        labelTitle: "Click to toggle",
        inputChecked: series.active(),
        onClick: () => {
          series.active.set(input.checked);
        },
        type: "checkbox",
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = utils.dom.createSpanName(name);
      spanMain.append(spanName);

      div.append(label);
      label.addEventListener("mouseover", () => {
        const h = hovered();
        if (!h || h !== series) {
          hovered.set(series);
        }
      });
      label.addEventListener("mouseleave", () => {
        hovered.set(null);
      });

      function shouldHighlight() {
        const h = hovered();
        return !h || h === series;
      }

      /**
       * @param {string} color
       */
      function tameColor(color) {
        return `${color.slice(0, -1)} / 50%)`;
      }

      const spanColors = window.document.createElement("span");
      spanColors.classList.add("colors");
      spanMain.prepend(spanColors);
      colors.forEach((color) => {
        const spanColor = window.document.createElement("span");
        spanColors.append(spanColor);

        signals.createEffect(
          () => ({
            color: color(),
            shouldHighlight: shouldHighlight(),
          }),
          ({ color, shouldHighlight }) => {
            if (shouldHighlight) {
              spanColor.style.backgroundColor = color;
            } else {
              spanColor.style.backgroundColor = tameColor(color);
            }
          },
        );
      });

      const initialColors = /** @type {Record<string, any>} */ ({});
      const darkenedColors = /** @type {Record<string, any>} */ ({});

      const seriesOptions = series.inner.options();
      if (!seriesOptions) return;

      Object.entries(seriesOptions).forEach(([k, v]) => {
        if (k.toLowerCase().includes("color") && typeof v === "string") {
          if (!v.startsWith("oklch")) return;
          initialColors[k] = v;
          darkenedColors[k] = tameColor(v);
        } else if (k === "lastValueVisible" && v) {
          initialColors[k] = true;
          darkenedColors[k] = false;
        }
      });

      signals.createEffect(shouldHighlight, (shouldHighlight) => {
        if (shouldHighlight) {
          series.inner.applyOptions(initialColors);
        } else {
          series.inner.applyOptions(darkenedColors);
        }
      });

      const anchor = window.document.createElement("a");

      signals.createEffect(series.url, (url) => {
        if (url) {
          anchor.href = url;
          anchor.target = "_blank";
          anchor.rel = "noopener noreferrer";
          anchor.title = "Click to view data";
          div.append(anchor);
        }
      });
    },
    /**
     * @param {number} start
     */
    removeFrom(start) {
      // disposeFrom(start);
      legends.splice(start).forEach((child) => child.remove());
    },
  };
}

/**
 * @param {ISeries} series
 * @param {DeepPartial<CreatePriceLine>} options
 * @param {Colors} colors
 */
function createPriceLine(series, options, colors) {
  series.createPriceLine({
    price: options.value || 0,
    color: colors.gray(),
    axisLabelVisible: false,
    lineWidth: 1,
    lineStyle: 4,
  });
}

/**
 * @param {number} value
 * @param {0 | 2} [digits]
 */
function numberToShortUSFormat(value, digits) {
  const absoluteValue = Math.abs(value);

  if (isNaN(value)) {
    return "";
  } else if (absoluteValue < 10) {
    return numberToUSFormat(value, Math.min(3, digits || 10));
  } else if (absoluteValue < 1_000) {
    return numberToUSFormat(value, Math.min(2, digits || 10));
  } else if (absoluteValue < 10_000) {
    return numberToUSFormat(value, Math.min(1, digits || 10));
  } else if (absoluteValue < 1_000_000) {
    return numberToUSFormat(value, 0);
  } else if (absoluteValue >= 900_000_000_000_000_000) {
    return "Inf.";
  }

  const log = Math.floor(Math.log10(absoluteValue) - 6);

  const suffices = ["M", "B", "T", "P", "E"];
  const letterIndex = Math.floor(log / 3);
  const letter = suffices[letterIndex];

  const modulused = log % 3;

  // return `${numberToUSFormat(
  //   value / (1_000_000 * 1_000 ** letterIndex),
  //   3,
  // )}${letter}`;

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
}

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

function createOklchToRGBA() {
  {
    /**
     *
     * @param {readonly [number, number, number, number, number, number, number, number, number]} A
     * @param {readonly [number, number, number]} B
     * @returns
     */
    function multiplyMatrices(A, B) {
      return /** @type {const} */ ([
        A[0] * B[0] + A[1] * B[1] + A[2] * B[2],
        A[3] * B[0] + A[4] * B[1] + A[5] * B[2],
        A[6] * B[0] + A[7] * B[1] + A[8] * B[2],
      ]);
    }
    /**
     * @param {readonly [number, number, number]} param0
     */
    function oklch2oklab([l, c, h]) {
      return /** @type {const} */ ([
        l,
        isNaN(h) ? 0 : c * Math.cos((h * Math.PI) / 180),
        isNaN(h) ? 0 : c * Math.sin((h * Math.PI) / 180),
      ]);
    }
    /**
     * @param {readonly [number, number, number]} rgb
     */
    function srgbLinear2rgb(rgb) {
      return rgb.map((c) =>
        Math.abs(c) > 0.0031308
          ? (c < 0 ? -1 : 1) * (1.055 * Math.abs(c) ** (1 / 2.4) - 0.055)
          : 12.92 * c,
      );
    }
    /**
     * @param {readonly [number, number, number]} lab
     */
    function oklab2xyz(lab) {
      const LMSg = multiplyMatrices(
        /** @type {const} */ ([
          1, 0.3963377773761749, 0.2158037573099136, 1, -0.1055613458156586,
          -0.0638541728258133, 1, -0.0894841775298119, -1.2914855480194092,
        ]),
        lab,
      );
      const LMS = /** @type {[number, number, number]} */ (
        LMSg.map((val) => val ** 3)
      );
      return multiplyMatrices(
        /** @type {const} */ ([
          1.2268798758459243, -0.5578149944602171, 0.2813910456659647,
          -0.0405757452148008, 1.112286803280317, -0.0717110580655164,
          -0.0763729366746601, -0.4214933324022432, 1.5869240198367816,
        ]),
        LMS,
      );
    }
    /**
     * @param {readonly [number, number, number]} xyz
     */
    function xyz2rgbLinear(xyz) {
      return multiplyMatrices(
        [
          3.2409699419045226, -1.537383177570094, -0.4986107602930034,
          -0.9692436362808796, 1.8759675015077202, 0.04155505740717559,
          0.05563007969699366, -0.20397695888897652, 1.0569715142428786,
        ],
        xyz,
      );
    }

    /** @param {string} oklch */
    return function (oklch) {
      oklch = oklch.replace("oklch(", "");
      oklch = oklch.replace(")", "");
      let splitOklch = oklch.split(" / ");
      let alpha = 1;
      if (splitOklch.length === 2) {
        alpha = Number(splitOklch.pop()?.replace("%", "")) / 100;
      }
      splitOklch = oklch.split(" ");
      const lch = splitOklch.map((v, i) => {
        if (!i && v.includes("%")) {
          return Number(v.replace("%", "")) / 100;
        } else {
          return Number(v);
        }
      });
      const rgb = srgbLinear2rgb(
        xyz2rgbLinear(
          oklab2xyz(oklch2oklab(/** @type {[number, number, number]} */ (lch))),
        ),
      ).map((v) => {
        return Math.max(Math.min(Math.round(v * 255), 255), 0);
      });
      return [...rgb, alpha];
    };
  }
}

export default { createChartElement };
