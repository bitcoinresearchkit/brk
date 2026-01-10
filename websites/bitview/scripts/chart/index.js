import {
  createChart as _createChart,
  CandlestickSeries,
  HistogramSeries,
  LineSeries,
  BaselineSeries,
  // } from "../modules/lightweight-charts/5.0.9/dist/lightweight-charts.standalone.development.mjs";
} from "../modules/lightweight-charts/5.0.9/dist/lightweight-charts.standalone.production.mjs";

const createChart = /** @type {CreateChart} */ (_createChart);

import {
  createHorizontalChoiceField,
  createLabeledInput,
  createSpanName,
} from "../utils/dom.js";
import { createOklchToRGBA } from "./oklch.js";
import { throttle } from "../utils/timing.js";
import { serdeBool } from "../utils/serde.js";
import { stringToId } from "../utils/format.js";
import { style } from "../utils/elements.js";

/**
 * @typedef {Object} Valued
 * @property {number} value
 *
 * @typedef {Object} Indexed
 * @property {number} index
 *
 * @typedef {_ISeriesApi<LCSeriesType, number>} ISeries
 * @typedef {_ISeriesApi<'Candlestick', number>} CandlestickISeries
 * @typedef {_ISeriesApi<'Histogram', number>} HistogramISeries
 * @typedef {_ISeriesApi<'Line', number>} LineISeries
 * @typedef {_ISeriesApi<'Baseline', number>} BaselineISeries
 *
 * @typedef {_LineSeriesPartialOptions} LineSeriesPartialOptions
 * @typedef {_HistogramSeriesPartialOptions} HistogramSeriesPartialOptions
 * @typedef {_BaselineSeriesPartialOptions} BaselineSeriesPartialOptions
 * @typedef {_CandlestickSeriesPartialOptions} CandlestickSeriesPartialOptions
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
 * @typedef {_HistogramData<number>} HistogramData
 *
 * @typedef {function({ iseries: ISeries; unit: Unit; index: ChartableIndex }): void} SetDataCallback
 *
 * @typedef {Object} Legend
 * @property {HTMLLegendElement} element
 * @property {function({ series: Series, name: string, order: number, colors: Color[] }): void} addOrReplace
 * @property {function(number): void} removeFrom
 */

const oklchToRGBA = createOklchToRGBA();

const lineWidth = /** @type {any} */ (1.5);

/**
 * @param {Object} args
 * @param {string} args.id
 * @param {HTMLElement} args.parent
 * @param {Signals} args.signals
 * @param {Colors} args.colors
 * @param {Resources} args.resources
 * @param {BrkClient} args.brk
 * @param {Accessor<ChartableIndex>} args.index
 * @param {((unknownTimeScaleCallback: VoidFunction) => void)} [args.timeScaleSetCallback]
 * @param {true} [args.fitContent]
 * @param {{unit: Unit; blueprints: AnySeriesBlueprint[]}[]} [args.config]
 */
function createChartElement({
  parent,
  signals,
  colors,
  id: chartId,
  index,
  resources,
  brk,
  timeScaleSetCallback,
  fitContent,
  config,
}) {
  const div = window.document.createElement("div");
  div.classList.add("chart");
  parent.append(div);

  const legendTop = createLegend(signals);
  div.append(legendTop.element);

  const chartDiv = window.document.createElement("div");
  chartDiv.classList.add("lightweight-chart");
  div.append(chartDiv);

  const legendBottom = createLegend(signals);
  div.append(legendBottom.element);

  const ichart = createChart(
    chartDiv,
    /** @satisfies {DeepPartial<ChartOptions>} */ ({
      autoSize: true,
      layout: {
        fontFamily: style.fontFamily,
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
  // Takes a bit more space sometimes but it's better UX than having the scale being resized on option change
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
      index === "monthindex"
        ? 1
        : index === "quarterindex"
          ? 2
          : index === "semesterindex"
            ? 3
            : index === "yearindex"
              ? 6
              : index === "decadeindex"
                ? 60
                : 0.5;

    ichart.applyOptions({
      timeScale: {
        timeVisible: index === "height",
        ...(!fitContent
          ? {
              minBarSpacing,
            }
          : {}),
      },
    });
  });

  const activeResources = /** @type {Set<MetricResource<unknown>>} */ (
    new Set()
  );
  ichart.subscribeCrosshairMove(
    throttle(() => {
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
   * @param {LCSeriesType} args.seriesType
   * @param {number} args.paneIndex
   */
  function addPriceScaleSelectorIfNeeded({ unit, paneIndex, seriesType }) {
    const id = `${chartId}-scale`;

    addFieldsetIfNeeded({
      id,
      paneIndex,
      position: "sw",
      createChild(pane) {
        const { field, selected } = createHorizontalChoiceField({
          choices: /** @type {const} */ (["lin", "log"]),
          id: stringToId(`${id} ${paneIndex} ${unit}`),
          defaultValue:
            unit.id === "usd" && seriesType !== "Baseline" ? "log" : "lin",
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
   * @param {LCSeriesType} args.seriesType
   * @param {AnyMetricPattern} [args.metric]
   * @param {SetDataCallback} [args.setDataCallback]
   * @param {Accessor<WhitespaceData<number>[]>} [args.data]
   * @param {number} args.paneIndex
   * @param {boolean} [args.defaultActive]
   */
  function addSeries({
    iseries,
    metric,
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
      const id = `${stringToId(name)}-${paneIndex}`;

      const active = signals.createSignal(defaultActive ?? true, {
        save: {
          keyPrefix: "",
          key: id,
          ...serdeBool,
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

      /** @type {MetricResource<unknown> | undefined} */
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

      if (metric) {
        signals.createEffect(index, (index) => {
          // Get timestamp metric from tree based on index type
          // timestampFixed has height only, timestamp has date-based indexes
          /** @type {AnyMetricPattern} */
          const timeMetric =
            index === "height"
              ? brk.tree.blocks.time.timestampFixed
              : brk.tree.blocks.time.timestamp;
          /** @type {AnyMetricPattern} */
          const valuesMetric = metric;
          const timeNode = timeMetric.by[index];
          const valuesNode = valuesMetric.by[index];
          if (!timeNode || !valuesNode)
            throw new Error(`Missing node for index: ${index}`);

          const timeResource = resources.useMetricEndpoint(timeNode);
          const valuesResource = resources.useMetricEndpoint(valuesNode);
          _valuesResource = valuesResource;

          series.url.set(() => {
            const base = brk.baseUrl.endsWith("/")
              ? brk.baseUrl.slice(0, -1)
              : brk.baseUrl;
            return `${base}${valuesResource.path}`;
          });

          signals.createEffect(active, (active) => {
            if (active) {
              timeResource.fetch();
              valuesResource.fetch();
              activeResources.add(valuesResource);

              const timeRange = timeResource.range();
              const valuesRange = valuesResource.range();
              signals.createEffect(
                () => ({
                  _indexes: timeRange.response()?.data,
                  values: valuesRange.response()?.data,
                }),
                ({ _indexes, values }) => {
                  if (!_indexes?.length || !values?.length) return;

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
                      // if (sameTime) {
                      //   console.log(data[offsetedI]);
                      // }
                      if (!Array.isArray(v) || v.length !== 4)
                        throw new Error(`Expected OHLC tuple, got: ${v}`);
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
                        index === "quarterindex" ||
                        index === "semesterindex" ||
                        index === "yearindex" ||
                        index === "decadeindex"
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
     * @param {AnyMetricPattern} [args.metric]
     * @param {Accessor<CandlestickData[]>} [args.data]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {boolean} [args.inverse]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {CandlestickSeriesPartialOptions} [args.options]
     */
    addCandlestickSeries({
      metric,
      name,
      unit,
      order,
      paneIndex = 0,
      defaultActive,
      setDataCallback,
      data,
      inverse,
      options,
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
            ...options,
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
        metric,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Color | [Color, Color]} [args.color] - Single color or [positive, negative] colors
     * @param {AnyMetricPattern} [args.metric]
     * @param {Accessor<HistogramData[]>} [args.data]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {HistogramSeriesPartialOptions} [args.options]
     */
    addHistogramSeries({
      metric,
      name,
      unit,
      color = [colors.green, colors.red],
      order,
      paneIndex = 0,
      defaultActive,
      setDataCallback,
      data,
      options,
    }) {
      const isDualColor = Array.isArray(color);
      const positiveColor = isDualColor ? color[0] : color;
      const negativeColor = isDualColor ? color[1] : color;

      /** @type {HistogramISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Histogram'>} */ (HistogramSeries),
          {
            color: positiveColor(),
            visible: defaultActive !== false,
            priceLineVisible: false,
            ...options,
          },
          paneIndex,
        )
      );

      return addSeries({
        colors: isDualColor ? [positiveColor, negativeColor] : [positiveColor],
        iseries,
        name,
        order,
        paneIndex,
        seriesType: "Bar",
        unit,
        data,
        setDataCallback: isDualColor
          ? (args) => {
              iseries.setData(
                iseries.data().map((d) => ({
                  ...d,
                  color:
                    "value" in d && d.value >= 0
                      ? positiveColor()
                      : negativeColor(),
                })),
              );
              setDataCallback?.(args);
            }
          : setDataCallback,
        defaultActive,
        metric,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Accessor<LineData[]>} [args.data]
     * @param {AnyMetricPattern} [args.metric]
     * @param {Color} [args.color]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {LineSeriesPartialOptions} [args.options]
     */
    addLineSeries({
      metric,
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
      color ||= unit.id === "usd" ? colors.green : colors.orange;

      /** @type {LineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (LineSeries),
          {
            lineWidth,
            visible: defaultActive !== false,
            priceLineVisible: false,
            color: color(),
            // lineVisible: false,
            // pointMarkersVisible: true,
            // pointMarkersRadius: 1,
            ...options,
          },
          paneIndex,
        )
      );

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
        metric,
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Accessor<BaselineData[]>} [args.data]
     * @param {AnyMetricPattern} [args.metric]
     * @param {SetDataCallback} [args.setDataCallback]
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {BaselineSeriesPartialOptions} [args.options]
     */
    addBaselineSeries({
      metric,
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
            lineWidth,
            visible: defaultActive !== false,
            baseValue: {
              price: options?.baseValue?.price ?? 0,
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
        metric,
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
      } else if (blueprint.type === "Histogram") {
        chart.addHistogramSeries({
          name: blueprint.title,
          unit,
          color: blueprint.color,
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
 * @param {Signals} signals
 */
function createLegend(signals) {
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

      const { input, label } = createLabeledInput({
        inputId: stringToId(`legend-${series.id}`),
        inputName: stringToId(`selected-${series.id}`),
        inputValue: "value",
        title: "Click to toggle",
        inputChecked: series.active(),
        onClick: () => {
          series.active.set(input.checked);
        },
        type: "checkbox",
      });

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = createSpanName(name);
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
  } else if (absoluteValue >= 1_000_000_000_000_000_000_000) {
    return "Inf.";
  }

  const log = Math.floor(Math.log10(absoluteValue) - 6);

  const suffices = ["M", "B", "T", "P", "E", "Z"];
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

/**
 * @typedef {typeof createChartElement} CreateChartElement
 * @typedef {ReturnType<createChartElement>} Chart
 */

export default { createChartElement };
