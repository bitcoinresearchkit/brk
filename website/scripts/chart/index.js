import {
  createChart as untypedLcCreateChart,
  CandlestickSeries,
  HistogramSeries,
  LineSeries,
  BaselineSeries,
  // } from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.development.mjs";
} from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.production.mjs";
import { createLegend } from "./legend.js";
import { capture, canCapture } from "./capture.js";

const lcCreateChart = /** @type {CreateLCChart} */ (untypedLcCreateChart);
import { createChoiceField } from "../utils/dom.js";
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
 * @property {string} key
 * @property {string} id
 * @property {number} paneIndex
 * @property {Signal<boolean>} active
 * @property {(value: boolean) => void} setActive
 * @property {() => void} show
 * @property {() => void} hide
 * @property {(order: number) => void} setOrder
 * @property {() => void} highlight
 * @property {() => void} tame
 * @property {() => boolean} hasData
 * @property {string | null} url
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
 * @param {HTMLElement} [args.captureElement]
 * @param {{unit: Unit; blueprints: AnySeriesBlueprint[]}[]} [args.config]
 */
export function createChart({
  parent,
  signals,
  colors,
  id: chartId,
  index,
  brk,
  timeScaleSetCallback,
  initialVisibleBarsCount,
  fitContent,
  captureElement,
  config,
}) {
  const div = window.document.createElement("div");
  div.classList.add("chart");
  parent.append(div);

  // Registry for shared legend signals (same name = linked across panes)
  /** @type {Map<string, Signal<boolean>>} */
  const sharedActiveSignals = new Map();

  // Registry for linked series (same key = linked across panes)
  /** @type {Map<string, Set<AnySeries>>} */
  const seriesByKey = new Map();

  const legendTop = createLegend();
  div.append(legendTop.element);

  const chartDiv = window.document.createElement("div");
  chartDiv.classList.add("lightweight-chart");
  div.append(chartDiv);

  const legendBottom = createLegend();
  div.append(legendBottom.element);

  const ichart = lcCreateChart(
    chartDiv,
    /** @satisfies {DeepPartial<ChartOptions>} */ ({
      autoSize: true,
      layout: {
        fontFamily: style.fontFamily,
        background: { color: "transparent" },
        attributionLogo: false,
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
        // conflationThresholdFactor: 8,
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
        mode: 0,
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

  /** @typedef {(visibleBarsCount: number) => void} ZoomChangeCallback */

  let visibleBarsCount = initialVisibleBarsCount ?? Infinity;
  /** @type {Set<ZoomChangeCallback>} */
  const onZoomChange = new Set();

  /** @param {{ from: number, to: number } | null} range */
  function updateVisibleBarsCount(range) {
    if (!range) return;
    const count = range.to - range.from;
    if (count === visibleBarsCount) return;
    visibleBarsCount = count;
    onZoomChange.forEach((cb) => cb(count));
  }

  ichart
    .timeScale()
    .subscribeVisibleLogicalRangeChange(throttle(updateVisibleBarsCount, 100));

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
    }, 10_000),
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
        const defaultValue =
          unit.id === "usd" && seriesType !== "Baseline" ? "log" : "lin";
        const selected = signals.createPersistedSignal({
          defaultValue,
          storageKey: `${id}-scale-${paneIndex}`,
          urlKey: paneIndex === 0 ? "price_scale" : "unit_scale",
          serialize: (v) => v,
          deserialize: (s) => /** @type {"lin" | "log"} */ (s),
        });
        const field = createChoiceField({
          choices: /** @type {const} */ (["lin", "log"]),
          id: stringToId(`${id} ${paneIndex} ${unit}`),
          defaultValue,
          selected,
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
   * @param {(order: number) => void} args.setOrder
   * @param {() => void} args.show
   * @param {() => void} args.hide
   * @param {() => void} args.highlight
   * @param {() => void} args.tame
   * @param {() => readonly any[]} args.getData
   * @param {(data: any[]) => void} args.setData
   * @param {(data: any) => void} args.update
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
    setOrder,
    show,
    hide,
    highlight,
    tame,
    getData,
    setData,
    update,
    onRemove,
  }) {
    return signals.createRoot((dispose) => {
      const key = stringToId(name);
      const id = `${key}-${paneIndex}`;

      // Reuse existing signal if same name (links legends across panes)
      let active = sharedActiveSignals.get(key);
      if (!active) {
        active = signals.createPersistedSignal({
          defaultValue: defaultActive ?? true,
          storageKey: id,
          urlKey: key,
          ...serdeBool,
        });
        sharedActiveSignals.set(key, active);
      }

      setOrder(-order);

      active() ? show() : hide();

      let hasData = false;
      let lastTime = -Infinity;

      /** @type {MetricResource<unknown> | undefined} */
      let _valuesResource;

      /** @type {AnySeries} */
      const series = {
        active,
        setActive(value) {
          active.set(value);
          seriesByKey.get(key)?.forEach((s) => {
            value ? s.show() : s.hide();
          });
          document.querySelectorAll(`[data-series="${key}"]`).forEach((el) => {
            if (el instanceof HTMLInputElement && el.type === "checkbox") {
              el.checked = value;
            }
          });
        },
        setOrder,
        show,
        hide,
        highlight,
        tame,
        hasData: () => hasData,
        key,
        id,
        paneIndex,
        url: null,
        getData,
        update,
        remove() {
          dispose();
          onRemove();
          seriesByKey.get(key)?.delete(series);
          if (_valuesResource) {
            activeResources.delete(_valuesResource);
          }
        },
      };

      // Register series for cross-pane linking
      let keySet = seriesByKey.get(key);
      if (!keySet) {
        keySet = new Set();
        seriesByKey.set(key, keySet);
      }
      keySet.add(series);

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

          series.url = `${
            brk.baseUrl.endsWith("/") ? brk.baseUrl.slice(0, -1) : brk.baseUrl
          }${valuesResource.path}`;

          (paneIndex ? legendBottom : legendTop).addOrReplace({
            series,
            name,
            colors,
            order,
          });

          // Create memo outside active check (cheap, just checks data existence)
          const timeRange = timeResource.range();
          const valuesRange = valuesResource.range();
          const valuesCacheKey = signals.createMemo(() => {
            const res = valuesRange.response();
            if (!res?.data?.length) return null;
            if (!timeRange.response()?.data?.length) return null;
            return `${res.version}|${res.stamp}|${res.total}|${res.start}|${res.end}`;
          });

          // Combined effect for active + data processing (flat, uses prev comparison)
          signals.createEffect(
            () => ({ isActive: active(), cacheKey: valuesCacheKey() }),
            (curr, prev) => {
              const becameActive = curr.isActive && (!prev || !prev.isActive);
              const becameInactive = !curr.isActive && prev?.isActive;

              if (becameInactive) {
                activeResources.delete(valuesResource);
                return;
              }

              if (!curr.isActive) return;

              if (becameActive) {
                timeResource.fetch();
                valuesResource.fetch();
                activeResources.add(valuesResource);
              }

              // Process data only if cacheKey changed
              if (!curr.cacheKey || curr.cacheKey === prev?.cacheKey) return;

              const _indexes = timeRange.response()?.data;
              const values = valuesRange.response()?.data;
              if (!_indexes?.length || !values?.length) return;

              const indexes = /** @type {number[]} */ (_indexes);
              const length = Math.min(indexes.length, values.length);

              // Find start index for processing
              let startIdx = 0;
              if (hasData) {
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

              if (!hasData) {
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
                hasData = true;
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
            },
          );
        });
      } else {
        (paneIndex ? legendBottom : legendTop).addOrReplace({
          series,
          name,
          colors,
          order,
        });

        if (data) {
          signals.createEffect(data, (data) => {
            setData(data);
            hasData = true;
            if (fitContent) {
              ichart.timeScale().fitContent();
            }
          });
        }
      }

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
            priceLineVisible: true,
          },
          paneIndex,
        )
      );

      let active = defaultActive !== false;
      let highlighted = true;
      let showLine = visibleBarsCount > 500;

      function update() {
        candlestickISeries.applyOptions({
          visible: active && !showLine,
          lastValueVisible: highlighted,
          upColor: upColor.highlight(highlighted),
          downColor: downColor.highlight(highlighted),
          wickUpColor: upColor.highlight(highlighted),
          wickDownColor: downColor.highlight(highlighted),
        });
        lineISeries.applyOptions({
          visible: active && showLine,
          lastValueVisible: highlighted,
          color: colors.default.highlight(highlighted),
        });
      }
      update();

      /** @type {ZoomChangeCallback} */
      function handleZoom(count) {
        const newShowLine = count > 500;
        showLine = newShowLine;
        update();
      }
      onZoomChange.add(handleZoom);

      const series = addSeries({
        colors: [upColor, downColor],
        name,
        order,
        paneIndex,
        seriesType: "Candlestick",
        unit,
        data,
        defaultActive,
        metric,
        setOrder(order) {
          candlestickISeries.setSeriesOrder(order);
          lineISeries.setSeriesOrder(order);
        },
        show() {
          if (active) return;
          active = true;
          update();
        },
        hide() {
          if (!active) return;
          active = false;
          update();
        },
        highlight() {
          if (highlighted) return;
          highlighted = true;
          update();
        },
        tame() {
          if (!highlighted) return;
          highlighted = false;
          update();
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
        onRemove: () => {
          onZoomChange.delete(handleZoom);
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
            priceLineVisible: false,
            ...options,
          },
          paneIndex,
        )
      );

      let active = defaultActive !== false;
      let highlighted = true;

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: highlighted,
          color: positiveColor.highlight(highlighted),
        });
      }
      update();

      const series = addSeries({
        colors: isDualColor ? [positiveColor, negativeColor] : [positiveColor],
        name,
        order,
        paneIndex,
        seriesType: "Bar",
        unit,
        data,
        defaultActive,
        metric,
        setOrder: (order) => iseries.setSeriesOrder(order),
        show() {
          if (active) return;
          active = true;
          update();
        },
        hide() {
          if (!active) return;
          active = false;
          update();
        },
        highlight() {
          if (highlighted) return;
          highlighted = true;
          update();
        },
        tame() {
          if (!highlighted) return;
          highlighted = false;
          update();
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
      color: _color,
      paneIndex = 0,
      defaultActive,
      data,
      options,
    }) {
      const color =
        _color ?? (unit.id === "usd" ? colors.green : colors.orange);

      /** @type {LineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (LineSeries),
          {
            lineWidth,
            priceLineVisible: false,
            ...options,
          },
          paneIndex,
        )
      );

      let active = defaultActive !== false;
      let highlighted = true;

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: highlighted,
          color: color.highlight(highlighted),
        });
      }
      update();

      const series = addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
        unit,
        data,
        defaultActive,
        metric,
        setOrder: (order) => iseries.setSeriesOrder(order),
        show() {
          if (active) return;
          active = true;
          update();
        },
        hide() {
          if (!active) return;
          active = false;
          update();
        },
        highlight() {
          if (highlighted) return;
          highlighted = true;
          update();
        },
        tame() {
          if (!highlighted) return;
          highlighted = false;
          update();
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
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
      color: _color,
      paneIndex = 0,
      defaultActive,
      data,
      options,
    }) {
      const color =
        _color ?? (unit.id === "usd" ? colors.green : colors.orange);

      /** @type {LineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Line'>} */ (LineSeries),
          {
            priceLineVisible: false,
            lineVisible: false,
            pointMarkersVisible: true,
            pointMarkersRadius: 1,
            ...options,
          },
          paneIndex,
        )
      );

      let active = defaultActive !== false;
      let highlighted = true;
      let radius =
        visibleBarsCount > 1000 ? 1 : visibleBarsCount > 200 ? 1.5 : 2;

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: highlighted,
          color: color.highlight(highlighted),
        });
      }
      update();

      /** @type {ZoomChangeCallback} */
      function handleZoom(count) {
        const newRadius = count > 1000 ? 1 : count > 200 ? 1.5 : 2;
        if (newRadius === radius) return;
        radius = newRadius;
        iseries.applyOptions({ pointMarkersRadius: radius });
      }
      onZoomChange.add(handleZoom);

      const series = addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
        unit,
        data,
        defaultActive,
        metric,
        setOrder: (order) => iseries.setSeriesOrder(order),
        show() {
          if (active) return;
          active = true;
          update();
        },
        hide() {
          if (!active) return;
          active = false;
          update();
        },
        highlight() {
          if (highlighted) return;
          highlighted = true;
          update();
        },
        tame() {
          if (!highlighted) return;
          highlighted = false;
          update();
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
        onRemove: () => {
          onZoomChange.delete(handleZoom);
          ichart.removeSeries(iseries);
        },
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
     * @param {Color} [args.topColor]
     * @param {Color} [args.bottomColor]
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
      topColor = colors.green,
      bottomColor = colors.red,
      options,
    }) {
      const paneIndex = _paneIndex ?? 0;

      /** @type {BaselineISeries} */
      const iseries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Baseline'>} */ (BaselineSeries),
          {
            lineWidth,
            baseValue: {
              price: options?.baseValue?.price ?? 0,
            },
            ...options,
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

      let active = defaultActive !== false;
      let highlighted = true;

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: highlighted,
          topLineColor: topColor.highlight(highlighted),
          bottomLineColor: bottomColor.highlight(highlighted),
        });
      }
      update();

      const series = addSeries({
        colors: [topColor, bottomColor],
        name,
        order,
        paneIndex,
        seriesType: "Baseline",
        unit,
        data,
        defaultActive,
        metric,
        setOrder: (order) => iseries.setSeriesOrder(order),
        show() {
          if (active) return;
          active = true;
          update();
        },
        hide() {
          if (!active) return;
          active = false;
          update();
        },
        highlight() {
          if (highlighted) return;
          highlighted = true;
          update();
        },
        tame() {
          if (!highlighted) return;
          highlighted = false;
          update();
        },
        setData: (data) => iseries.setData(data),
        update: (data) => iseries.update(data),
        getData: () => iseries.data(),
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

  if (captureElement && canCapture) {
    const domain = window.document.createElement("p");
    domain.innerText = window.location.host;
    domain.id = "domain";

    addFieldsetIfNeeded({
      id: "capture",
      paneIndex: 0,
      position: "ne",
      createChild() {
        const button = window.document.createElement("button");
        button.id = "capture";
        button.innerText = "capture";
        button.title = "Capture chart as image";
        button.addEventListener("click", async () => {
          captureElement.dataset.screenshot = "true";
          captureElement.append(domain);
          try {
            await capture({ element: captureElement, name: chartId });
          } catch {}
          captureElement.removeChild(domain);
          captureElement.dataset.screenshot = "false";
        });
        return button;
      },
    });
  }

  return chart;
}

/**
 * @typedef {typeof createChart} CreateChart
 * @typedef {ReturnType<createChart>} Chart
 */
