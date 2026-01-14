import {
  createChart as _createChart,
  CandlestickSeries,
  HistogramSeries,
  LineSeries,
  BaselineSeries,
  // } from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.development.mjs";
} from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.production.mjs";

const createChart = /** @type {CreateChart} */ (_createChart);

import {
  createChoiceField,
  createLabeledInput,
  createSpanName,
} from "../utils/dom.js";
import { createOklchToRGBA } from "./oklch.js";
import { throttle } from "../utils/timing.js";
import { serdeBool } from "../utils/serde.js";
import { stringToId } from "../utils/format.js";
import { style } from "../utils/elements.js";
import { resources } from "../resources.js";

/**
 * @typedef {Object} Valued
 * @property {number} value
 *
 * @typedef {Object} Indexed
 * @property {number} index
 *
 * @typedef {_ISeriesApi<LCSeriesType>} ISeries
 * @typedef {_ISeriesApi<'Candlestick'>} CandlestickISeries
 * @typedef {_ISeriesApi<'Histogram'>} HistogramISeries
 * @typedef {_ISeriesApi<'Line'>} LineISeries
 * @typedef {_ISeriesApi<'Baseline'>} BaselineISeries
 *
 * @typedef {_LineSeriesPartialOptions} LineSeriesPartialOptions
 * @typedef {_HistogramSeriesPartialOptions} HistogramSeriesPartialOptions
 * @typedef {_BaselineSeriesPartialOptions} BaselineSeriesPartialOptions
 * @typedef {_CandlestickSeriesPartialOptions} CandlestickSeriesPartialOptions
 */

/**
 * @template T
 * @typedef {Object} Series
 * @property {string} id
 * @property {Signal<boolean>} active
 * @property {Signal<boolean>} hasData
 * @property {Signal<string | null>} url
 * @property {() => Record<string, any>} getOptions
 * @property {(options: Record<string, any>) => void} applyOptions
 * @property {() => readonly T[]} getData
 * @property {(data: T) => void} update
 * @property {VoidFunction} remove
 */

/**
 * @typedef {Series<any>} AnySeries
 */

/**
 * @typedef {_SingleValueData} SingleValueData
 * @typedef {_CandlestickData} CandlestickData
 * @typedef {_LineData} LineData
 * @typedef {_BaselineData} BaselineData
 * @typedef {_HistogramData} HistogramData
 *
 * @typedef {Object} Legend
 * @property {HTMLLegendElement} element
 * @property {function({ series: AnySeries, name: string, order: number, colors: Color[] }): void} addOrReplace
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
 * @param {BrkClient} args.brk
 * @param {Accessor<ChartableIndex>} args.index
 * @param {((unknownTimeScaleCallback: VoidFunction) => void)} [args.timeScaleSetCallback]
 * @param {true} [args.fitContent]
 * @param {{unit: Unit; blueprints: AnySeriesBlueprint[]}[]} [args.config]
 */
export function createChartElement({
  parent,
  signals,
  colors,
  id: chartId,
  index,
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

  const visibleBarsCount = signals.createSignal(0);
  ichart.timeScale().subscribeVisibleLogicalRangeChange((range) => {
    if (range) {
      visibleBarsCount.set(range.to - range.from);
    }
  });

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
        const { field, selected } = createChoiceField({
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
   * @param {string} args.name
   * @param {Unit} args.unit
   * @param {number} args.order
   * @param {Color[]} args.colors
   * @param {LCSeriesType} args.seriesType
   * @param {AnyMetricPattern} [args.metric]
   * @param {Accessor<WhitespaceData[]>} [args.data]
   * @param {number} args.paneIndex
   * @param {boolean} [args.defaultActive]
   * @param {(ctx: { active: Signal<boolean> }) => void} args.setup
   * @param {() => readonly any[]} args.getData
   * @param {(data: any[]) => void} args.setData
   * @param {(data: any) => void} args.update
   * @param {() => Record<string, any>} args.getOptions
   * @param {(options: Record<string, any>) => void} args.applyOptions
   * @param {() => void} args.onRemove
   */
  function addSeries({
    metric,
    name,
    unit,
    order,
    seriesType,
    paneIndex,
    defaultActive,
    colors,
    data,
    setup,
    getData,
    setData,
    update,
    getOptions,
    applyOptions,
    onRemove,
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

      setup({ active });

      const hasData = signals.createSignal(false);
      let lastTime = -Infinity;

      /** @type {MetricResource<unknown> | undefined} */
      let _valuesResource;

      /** @type {AnySeries} */
      const series = {
        active,
        hasData,
        id,
        url: signals.createSignal(/** @type {string | null} */ (null)),
        getOptions,
        applyOptions,
        getData,
        update,
        remove() {
          dispose();
          onRemove();
          if (_valuesResource) {
            activeResources.delete(_valuesResource);
          }
        },
      };

      if (metric) {
        signals.createEffect(index, (index) => {
          // Get timestamp metric from tree based on index type
          // timestampMonotonic has height only, timestamp has date-based indexes
          /** @type {AnyMetricPattern} */
          const timeMetric =
            index === "height"
              ? brk.metrics.blocks.time.timestampMonotonic
              : brk.metrics.blocks.time.timestamp;
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
                    const time = /** @type {Time} */ (indexes[i]);
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

                  if (!hasData()) {
                    setData(data);
                    hasData.set(true);
                    lastTime = /** @type {number} */ (data.at(-1)?.time) ?? -Infinity;

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
                    for (let i = 0; i < data.length; i++) {
                      const time = /** @type {number} */ (data[i].time);
                      if (time >= lastTime) {
                        update(data[i]);
                        lastTime = time;
                      }
                    }
                  }
                },
              );
            } else {
              activeResources.delete(valuesResource);
            }
          });
        });
      } else if (data) {
        signals.createEffect(data, (data) => {
          setData(data);
          hasData.set(true);

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
     * @param {[Color, Color]} [args.colors] - [upColor, downColor] for legend
     * @param {boolean} [args.defaultActive]
     * @param {boolean} [args.inverse]
     * @param {CandlestickSeriesPartialOptions} [args.options]
     */
    addCandlestickSeries({
      metric,
      name,
      unit,
      order,
      paneIndex = 0,
      colors: customColors,
      defaultActive,
      data,
      inverse,
      options,
    }) {
      const defaultGreen = inverse ? colors.red : colors.green;
      const defaultRed = inverse ? colors.green : colors.red;
      const upColor = customColors?.[0] ?? defaultGreen;
      const downColor = customColors?.[1] ?? defaultRed;

      /** @type {CandlestickISeries} */
      const candlestickISeries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Candlestick'>} */ (CandlestickSeries),
          {
            upColor: upColor(),
            downColor: downColor(),
            wickUpColor: upColor(),
            wickDownColor: downColor(),
            borderVisible: false,
            visible: defaultActive !== false,
            ...options,
          },
          paneIndex,
        )
      );

      /** @type {LineISeries} */
      const lineISeries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (LineSeries),
          {
            color: colors.default(),
            lineWidth,
            visible: false,
            priceLineVisible: false,
          },
          paneIndex,
        )
      );

      let showLine = false;

      return addSeries({
        colors: [upColor, downColor],
        name,
        order,
        paneIndex,
        seriesType: "Candlestick",
        unit,
        data,
        defaultActive,
        metric,
        setup: ({ active }) => {
          candlestickISeries.setSeriesOrder(order);
          lineISeries.setSeriesOrder(order);
          signals.createEffect(
            () => ({ count: visibleBarsCount(), active: active() }),
            ({ count, active }) => {
              showLine = count > 500;
              candlestickISeries.applyOptions({ visible: active && !showLine });
              lineISeries.applyOptions({ visible: active && showLine });
            },
          );
        },
        setData: (data) => {
          candlestickISeries.setData(data);
          const lineData = data.map((d) => ({ time: d.time, value: d.close }));
          lineISeries.setData(lineData);
        },
        update: (data) => {
          candlestickISeries.update(data);
          lineISeries.update({ time: data.time, value: data.close });
        },
        getData: () => candlestickISeries.data(),
        getOptions: () =>
          showLine ? lineISeries.options() : candlestickISeries.options(),
        applyOptions: (options) =>
          showLine
            ? lineISeries.applyOptions(options)
            : candlestickISeries.applyOptions(options),
        onRemove: () => {
          ichart.removeSeries(candlestickISeries);
          ichart.removeSeries(lineISeries);
        },
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
        name,
        order,
        paneIndex,
        seriesType: "Bar",
        unit,
        data,
        defaultActive,
        metric,
        setup: ({ active }) => {
          iseries.setSeriesOrder(order);
          signals.createEffect(active, (active) =>
            iseries.applyOptions({ visible: active }),
          );
        },
        setData: (data) => {
          if (isDualColor) {
            iseries.setData(
              data.map((d) => ({
                ...d,
                color:
                  "value" in d && d.value >= 0
                    ? positiveColor()
                    : negativeColor(),
              })),
            );
          } else {
            iseries.setData(data);
          }
        },
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
        getOptions: () => iseries.options(),
        applyOptions: (options) => iseries.applyOptions(options),
        onRemove: () => ichart.removeSeries(iseries),
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
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {LineSeriesPartialOptions} [args.options]
     */
    addLineSeries({
      metric,
      name,
      unit,
      order,
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
            ...options,
          },
          paneIndex,
        )
      );

      return addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
        unit,
        data,
        defaultActive,
        metric,
        setup: ({ active }) => {
          iseries.setSeriesOrder(order);
          signals.createEffect(active, (active) =>
            iseries.applyOptions({ visible: active }),
          );
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
        getOptions: () => iseries.options(),
        applyOptions: (options) => iseries.applyOptions(options),
        onRemove: () => ichart.removeSeries(iseries),
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
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {LineSeriesPartialOptions} [args.options]
     */
    addDotsSeries({
      metric,
      name,
      unit,
      order,
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
            visible: defaultActive !== false,
            priceLineVisible: false,
            color: color(),
            lineVisible: false,
            pointMarkersVisible: true,
            pointMarkersRadius: 1,
            ...options,
          },
          paneIndex,
        )
      );

      return addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
        unit,
        data,
        defaultActive,
        metric,
        setup: ({ active }) => {
          iseries.setSeriesOrder(order);
          signals.createEffect(active, (active) =>
            iseries.applyOptions({ visible: active }),
          );
          signals.createEffect(visibleBarsCount, (count) => {
            const radius = count > 1000 ? 1 : count > 200 ? 1.5 : 2;
            iseries.applyOptions({ pointMarkersRadius: radius });
          });
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
        getOptions: () => iseries.options(),
        applyOptions: (options) => iseries.applyOptions(options),
        onRemove: () => ichart.removeSeries(iseries),
      });
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {Accessor<BaselineData[]>} [args.data]
     * @param {AnyMetricPattern} [args.metric]
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
        name,
        order,
        paneIndex,
        seriesType: "Baseline",
        unit,
        data,
        defaultActive,
        metric,
        setup: ({ active }) => {
          iseries.setSeriesOrder(order);
          signals.createEffect(active, (active) =>
            iseries.applyOptions({ visible: active }),
          );
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
        getOptions: () => iseries.options(),
        applyOptions: (options) => iseries.applyOptions(options),
        onRemove: () => ichart.removeSeries(iseries),
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
      } else if (blueprint.type === "Dots") {
        chart.addDotsSeries({
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

  const hovered = signals.createSignal(/** @type {AnySeries | null} */ (null));

  /** @type {HTMLElement[]} */
  const legends = [];

  return {
    element,
    /**
     * @param {Object} args
     * @param {AnySeries} args.series
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

      const seriesOptions = series.getOptions();
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
          series.applyOptions(initialColors);
        } else {
          series.applyOptions(darkenedColors);
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
