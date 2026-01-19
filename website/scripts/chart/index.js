import {
  createChart as _createChart,
  CandlestickSeries,
  HistogramSeries,
  LineSeries,
  BaselineSeries,
  // } from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.development.mjs";
} from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.production.mjs";
import { createMinMaxMarkers } from "./markers.js";
import { createLegend } from "./legend.js";

const createChart = /** @type {CreateChart} */ (_createChart);
import { createChoiceField } from "../utils/dom.js";
import { createOklchToRGBA } from "./oklch.js";
import { throttle } from "../utils/timing.js";
import { serdeBool } from "../utils/serde.js";
import { stringToId, numberToShortUSFormat } from "../utils/format.js";
import { style } from "../utils/elements.js";
import { resources } from "../resources.js";

/**
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
 * @property {() => ISeries} inner
 * @property {number} paneIndex
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
 * @param {number | null} [args.initialVisibleBarsCount]
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
  initialVisibleBarsCount,
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
        enableConflation: true,
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

  /** @param {{ from: number, to: number }} range */
  const setVisibleLogicalRange = (range) => {
    // Defer to next frame to ensure chart has rendered
    requestAnimationFrame(() => {
      ichart.timeScale().setVisibleLogicalRange(range);
    });
  };

  const seriesList = signals.createSignal(/** @type {Set<AnySeries>} */ (new Set()), { equals: false });
  const seriesCount = signals.createMemo(() => seriesList().size);
  const markers = createMinMaxMarkers({
    chart: ichart,
    seriesList,
    colors,
    formatValue: numberToShortUSFormat,
  });

  const visibleBarsCount = signals.createSignal(
    initialVisibleBarsCount ?? Infinity,
  );
  /** @type {() => 0 | 1 | 2 | 3} 0: <=200, 1: <=500, 2: <=1000, 3: >1000 */
  const visibleBarsCountBucket = signals.createMemo(() => {
    const count = visibleBarsCount();
    return count > 1000 ? 3 : count > 500 ? 2 : count > 200 ? 1 : 0;
  });
  const shouldShowLine = signals.createMemo(
    () => visibleBarsCountBucket() >= 2,
  );
  const shouldUpdateMarkers = signals.createMemo(
    () => visibleBarsCount() * seriesCount() <= 5000,
  );

  signals.createEffect(shouldUpdateMarkers, (should) => {
    if (should) markers.update();
    else markers.clear();
  });

  ichart.timeScale().subscribeVisibleLogicalRangeChange(
    throttle((range) => {
      if (range) {
        visibleBarsCount.set(range.to - range.from);
        if (shouldUpdateMarkers()) markers.update();
      }
    }, 100),
  );

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
   * @param {() => ISeries} args.inner
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
    inner,
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
        inner,
        paneIndex,
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
          seriesList().delete(series);
          seriesList.set(seriesList());
        },
      };

      seriesList().add(series);
      seriesList.set(seriesList());

      if (metric) {
        signals.createScopedEffect(index, (index) => {
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

          signals.createScopedEffect(active, (active) => {
            if (active) {
              timeResource.fetch();
              valuesResource.fetch();
              activeResources.add(valuesResource);

              const timeRange = timeResource.range();
              const valuesRange = valuesResource.range();
              const valuesCacheKey = signals.createMemo(() => {
                const res = valuesRange.response();
                if (!res?.data?.length) return null;
                if (!timeRange.response()?.data?.length) return null;
                return `${res.version}|${res.stamp}|${res.total}|${res.start}|${res.end}`;
              });
              signals.createEffect(valuesCacheKey, (cacheKey) => {
                if (!cacheKey) return;
                const _indexes = timeRange.response()?.data;
                const values = valuesRange.response()?.data;
                if (!_indexes?.length || !values?.length) return;

                const indexes = /** @type {number[]} */ (_indexes);
                const length = Math.min(indexes.length, values.length);

                // Find start index for processing
                let startIdx = 0;
                if (hasData()) {
                  // Binary search to find first index where time >= lastTime
                  let lo = 0;
                  let hi = length;
                  while (lo < hi) {
                    const mid = (lo + hi) >>> 1;
                    if (indexes[mid] < lastTime) {
                      lo = mid + 1;
                    } else {
                      hi = mid;
                    }
                  }
                  startIdx = lo;
                  if (startIdx >= length) return; // No new data
                }

                /**
                 * @param {number} i
                 * @param {(number | null | [number, number, number, number])[]} vals
                 * @returns {LineData | CandlestickData}
                 */
                function buildDataPoint(i, vals) {
                  const time = /** @type {Time} */ (indexes[i]);
                  const v = vals[i];
                  if (v === null) {
                    return { time, value: NaN };
                  } else if (typeof v === "number") {
                    return { time, value: v };
                  } else {
                    if (!Array.isArray(v) || v.length !== 4)
                      throw new Error(`Expected OHLC tuple, got: ${v}`);
                    const [open, high, low, close] = v;
                    return { time, open, high, low, close };
                  }
                }

                if (!hasData()) {
                  // Initial load: build full array
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
                    const offsetedI = i - timeOffset;
                    const point = buildDataPoint(i, values);
                    if (sameTime && "open" in point) {
                      const prev = /** @type {CandlestickData} */ (
                        data[offsetedI]
                      );
                      point.open = prev.open;
                      point.high = Math.max(prev.high, point.high);
                      point.low = Math.min(prev.low, point.low);
                    }
                    data[offsetedI] = point;
                    prevTime = time;
                  }

                  data.length -= timeOffset;

                  setData(data);
                  hasData.set(true);
                  if (shouldUpdateMarkers()) markers.scheduleUpdate();
                  lastTime =
                    /** @type {number} */ (data.at(-1)?.time) ?? -Infinity;

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
                      setVisibleLogicalRange({ from: -1, to: data.length });
                    }
                  });
                } else {
                  // Incremental update: only process new data points
                  for (let i = startIdx; i < length; i++) {
                    const point = buildDataPoint(i, values);
                    update(point);
                    lastTime = /** @type {number} */ (point.time);
                  }
                }
              });
            } else {
              activeResources.delete(valuesResource);
            }
          });
        });
      } else if (data) {
        signals.createEffect(data, (data) => {
          setData(data);
          hasData.set(true);
          if (shouldUpdateMarkers()) markers.scheduleUpdate();

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
    legendTop,
    legendBottom,

    addFieldsetIfNeeded,

    setVisibleLogicalRange,

    /**
     * @param {(range: { from: number, to: number } | null) => void} callback
     * @param {number} [wait=500]
     */
    onVisibleLogicalRangeChange(callback, wait = 500) {
      ichart
        .timeScale()
        .subscribeVisibleLogicalRangeChange(throttle(callback, wait));
    },

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
      let showLine = shouldShowLine();

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
            visible: false,
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

      const series = addSeries({
        inner: () => (showLine ? lineISeries : candlestickISeries),
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
            () => ({
              shouldShow: shouldShowLine(),
              active: active(),
              barsCount: visibleBarsCount(),
            }),
            ({ shouldShow, active, barsCount }) => {
              if (barsCount === Infinity) return;
              const wasLine = showLine;
              showLine = shouldShow;
              candlestickISeries.applyOptions({ visible: active && !showLine });
              lineISeries.applyOptions({
                visible: active && showLine,
                priceLineVisible: active && showLine,
              });
              if (wasLine !== showLine && shouldUpdateMarkers())
                markers.scheduleUpdate();
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
      return series;
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

      const series = addSeries({
        inner: () => iseries,
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
      return series;
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

      const series = addSeries({
        inner: () => iseries,
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
      return series;
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

      const series = addSeries({
        inner: () => iseries,
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
          signals.createEffect(visibleBarsCountBucket, (bucket) => {
            const radius = bucket === 3 ? 1 : bucket >= 1 ? 1.5 : 2;
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
      return series;
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

      const series = addSeries({
        inner: () => iseries,
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
      return series;
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
 * @typedef {typeof createChartElement} CreateChartElement
 * @typedef {ReturnType<createChartElement>} Chart
 */
