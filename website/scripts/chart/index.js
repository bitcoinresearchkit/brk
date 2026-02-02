import {
  createChart as untypedLcCreateChart,
  CandlestickSeries,
  HistogramSeries,
  LineSeries,
  BaselineSeries,
  // } from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.development.mjs";
} from "../modules/lightweight-charts/5.1.0/dist/lightweight-charts.standalone.production.mjs";
import { createLegend } from "./legend.js";
import { capture } from "./capture.js";
import { colors } from "./colors.js";
import { createRadios, createSelect } from "../utils/dom.js";
import { createPersistedValue } from "../utils/persisted.js";
import { onChange as onThemeChange } from "../utils/theme.js";
import { throttle, debounce } from "../utils/timing.js";
import { serdeBool, serdeChartableIndex } from "../utils/serde.js";
import { stringToId, numberToShortUSFormat } from "../utils/format.js";
import { style } from "../utils/elements.js";
import { Unit } from "../utils/units.js";

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
 * @property {number} paneIndex
 * @property {PersistedValue<boolean>} active
 * @property {(value: boolean) => void} setActive
 * @property {() => void} show
 * @property {() => void} hide
 * @property {(order: number) => void} setOrder
 * @property {() => void} highlight
 * @property {() => void} tame
 * @property {() => boolean} hasData
 * @property {() => void} [fetch]
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
 * @param {BrkClient} args.brk
 * @param {true} [args.fitContent]
 */
export function createChart({ parent, id: chartId, brk, fitContent }) {
  const baseUrl = brk.baseUrl.replace(/\/$/, "");

  /** @type {string} */
  let storageId = "";

  /** @param {ChartableIndex} idx */
  const getTimeEndpoint = (idx) =>
    idx === "height"
      ? brk.metrics.blocks.time.timestampMonotonic.by[idx]
      : brk.metrics.blocks.time.timestamp.by[idx];

  const index = {
    /** @type {Set<(index: ChartableIndex) => void>} */
    onChange: new Set(),

    get() {
      return serdeChartableIndex.deserialize(index.name.value);
    },

    name: createPersistedValue({
      defaultValue: /** @type {ChartableIndexName} */ ("date"),
      storageKey: "chart-index",
      urlKey: "i",
      serialize: (v) => v,
      deserialize: (s) => /** @type {ChartableIndexName} */ (s),
      onChange: () => {
        range.set(null);
        index.onChange.forEach((cb) => cb(index.get()));
      },
    }),
  };

  // Range state: localStorage stores all ranges per-index, URL stores current range only
  /** @typedef {{ from: number, to: number }} Range */
  const ranges = createPersistedValue({
    defaultValue: /** @type {Record<string, Range>} */ ({}),
    storageKey: "chart-ranges",
    serialize: JSON.stringify,
    deserialize: JSON.parse,
  });

  const range = createPersistedValue({
    defaultValue: /** @type {Range | null} */ (null),
    urlKey: "r",
    serialize: (v) => (v ? `${v.from.toFixed(2)}_${v.to.toFixed(2)}` : ""),
    deserialize: (s) => {
      if (!s) return null;
      const [from, to] = s.split("_").map(Number);
      return !isNaN(from) && !isNaN(to) ? { from, to } : null;
    },
  });

  /** @returns {Range | null} */
  const getRange = () => range.value ?? ranges.value[index.name.value] ?? null;

  /** @param {Range} value */
  const setRange = (value) => {
    ranges.set({ ...ranges.value, [index.name.value]: value });
    range.set(value);
  };

  const legends = {
    top: createLegend(),
    bottom: createLegend(),
  };

  const elements = {
    root: document.createElement("div"),
    chart: document.createElement("div"),

    setup() {
      elements.root.classList.add("chart");
      elements.chart.classList.add("lightweight-chart");
      parent.append(elements.root);
      elements.root.append(legends.top.element);
      elements.root.append(elements.chart);
      elements.root.append(legends.bottom.element);
    },
  };
  elements.setup();

  const ichart = /** @type {CreateLCChart} */ (untypedLcCreateChart)(
    elements.chart,
    /** @satisfies {DeepPartial<ChartOptions>} */ ({
      autoSize: true,
      layout: {
        // fontSize: 14,
        fontFamily: style.fontFamily,
        background: { color: "transparent" },
        attributionLogo: false,
        panes: {
          enableResize: false,
        },
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

  /** @typedef {(visibleBarsCount: number) => void} ZoomChangeCallback */

  const initialRange = getRange();
  if (initialRange) {
    ichart.timeScale().setVisibleLogicalRange(initialRange);
  }

  let visibleBarsCount = initialRange
    ? initialRange.to - initialRange.from
    : Infinity;

  /** @param {number} count */
  const getDotsRadius = (count) =>
    count > 1000 ? 1 : count > 200 ? 1.5 : count > 100 ? 2 : 3;

  /** @type {Set<ZoomChangeCallback>} */
  const onZoomChange = new Set();

  ichart.timeScale().subscribeVisibleLogicalRangeChange(
    throttle((range) => {
      if (!range) return;
      const count = range.to - range.from;
      if (count === visibleBarsCount) return;
      visibleBarsCount = count;
      onZoomChange.forEach((cb) => cb(count));
    }, 100),
  );

  // Debounced range persistence
  ichart.timeScale().subscribeVisibleLogicalRangeChange(
    debounce((range) => {
      if (range && range.from < range.to) {
        setRange({ from: range.from, to: range.to });
      }
    }, 100),
  );

  function applyColors() {
    const defaultColor = colors.default();
    const offColor = colors.gray();
    const borderColor = colors.border();
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
  }
  applyColors();
  const removeThemeListener = onThemeChange(applyColors);

  /** @type {Partial<Record<ChartableIndex, number>>} */
  const minBarSpacingByIndex = {
    monthindex: 1,
    quarterindex: 2,
    semesterindex: 3,
    yearindex: 6,
    decadeindex: 60,
  };

  /** @param {ChartableIndex} index */
  function applyIndexSettings(index) {
    const minBarSpacing = minBarSpacingByIndex[index] ?? 0.5;

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
  }
  applyIndexSettings(index.get());
  index.onChange.add(applyIndexSettings);

  // Periodic refresh of active series data
  const refreshInterval = setInterval(() => serieses.refreshAll(), 30_000);

  if (fitContent) {
    new ResizeObserver(() => ichart.timeScale().fitContent()).observe(
      elements.chart,
    );
  }

  const fieldsets = {
    /** @type {Map<number, Map<string, { id: string, position: string, createChild: (pane: IPaneApi<Time>) => HTMLElement }>>} */
    configs: new Map(),

    /**
     * @param {number} configPaneIndex
     * @param {number} [targetPaneIndex]
     */
    createForPane(configPaneIndex, targetPaneIndex = configPaneIndex) {
      const pane = ichart.panes().at(targetPaneIndex);
      if (!pane) return;

      const parent = pane.getHTMLElement()?.children?.item(1)?.firstChild;
      if (!parent) return;

      const configs = this.configs.get(configPaneIndex);
      if (!configs) return;

      for (const { id, position, createChild } of configs.values()) {
        /** @type {Element} */ (parent)
          .querySelectorAll(`[data-position="${position}"]`)
          .forEach((el) => el.remove());

        const fieldset = document.createElement("fieldset");
        fieldset.dataset.size = "xs";
        fieldset.dataset.position = position;
        fieldset.id = `${id}-${configPaneIndex}`;
        parent.appendChild(fieldset);
        fieldset.append(createChild(pane));
      }
    },

    /**
     * @param {Object} args
     * @param {string} args.id
     * @param {number} args.paneIndex
     * @param {"nw" | "ne" | "se" | "sw"} args.position
     * @param {(pane: IPaneApi<Time>) => HTMLElement} args.createChild
     */
    addIfNeeded({ paneIndex, id, position, createChild }) {
      let configs = this.configs.get(paneIndex);
      if (!configs) {
        configs = new Map();
        this.configs.set(paneIndex, configs);
      }
      configs.set(id, { id, position, createChild });
    },
  };

  const panes = {
    /** @type {Map<number, Map<AnySeries, ISeries[]>>} */
    seriesByHome: new Map(),
    pendingVisibilityCheck: false,

    /** @param {number} homePane */
    isAllHidden(homePane) {
      const map = this.seriesByHome.get(homePane);
      return !map || [...map.keys()].every((s) => !s.active.value);
    },

    /**
     * @param {number} homePane
     * @param {number} targetPane
     */
    moveTo(homePane, targetPane) {
      const map = this.seriesByHome.get(homePane);
      if (!map) return;
      for (const iseries of map.values()) {
        for (const is of iseries) {
          if (is.getPane().paneIndex() !== targetPane) {
            is.moveToPane(targetPane);
          }
        }
      }
    },

    /**
     * @param {number} paneIndex
     * @param {VoidFunction} callback
     * @param {number} [retries]
     */
    whenReady(paneIndex, callback, retries = 10) {
      const pane = ichart.panes().at(paneIndex);
      const parent = pane?.getHTMLElement()?.children?.item(1)?.firstChild;
      if (parent) {
        callback();
      } else if (retries > 0) {
        requestAnimationFrame(() =>
          this.whenReady(paneIndex, callback, retries - 1),
        );
      }
    },

    updateVisibility() {
      const pane0Hidden = this.isAllHidden(0);
      const pane1Hidden = this.isAllHidden(1);
      const bothVisible = !pane0Hidden && !pane1Hidden;

      this.moveTo(1, bothVisible ? 1 : 0);

      if (bothVisible) {
        this.whenReady(1, () => {
          fieldsets.createForPane(0);
          fieldsets.createForPane(1);
        });
      } else {
        this.whenReady(0, () => {
          if (pane0Hidden && !pane1Hidden) {
            fieldsets.createForPane(1, 0);
          } else {
            fieldsets.createForPane(0);
          }
        });
      }
    },

    /**
     * @param {number} paneIndex
     * @param {AnySeries} series
     * @param {ISeries[]} iseries
     */
    register(paneIndex, series, iseries) {
      let paneMap = this.seriesByHome.get(paneIndex);
      if (!paneMap) {
        paneMap = new Map();
        this.seriesByHome.set(paneIndex, paneMap);
      }
      paneMap.set(series, iseries);

      if (!this.pendingVisibilityCheck) {
        this.pendingVisibilityCheck = true;
        requestAnimationFrame(() => {
          this.pendingVisibilityCheck = false;
          this.updateVisibility();
        });
      }
    },
  };

  const serieses = {
    /** @type {Set<AnySeries>} */
    all: new Set(),

    refreshAll() {
      serieses.all.forEach((s) => {
        if (s.active.value) s.fetch?.();
      });
    },

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {Color[]} args.colors
     * @param {AnyMetricPattern} args.metric
     * @param {number} args.paneIndex
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
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
    create({
      metric,
      name,
      order,
      paneIndex,
      unit,
      key: customKey,
      defaultActive,
      colors,
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
      const key = customKey ?? stringToId(name);
      const id = `${unit.id}-${key}`;

      const active = createPersistedValue({
        defaultValue: defaultActive ?? true,
        storageKey: `${storageId}-p${paneIndex}-${key}`,
        urlKey: `${paneIndex === 0 ? "t" : "b"}-${key}`,
        ...serdeBool,
      });

      setOrder(-order);

      active.value ? show() : hide();

      let hasData = false;
      let lastTime = -Infinity;
      /** @type {string | null} */
      let lastStamp = null;

      /** @type {VoidFunction | null} */
      let _fetch = null;

      /** @type {AnySeries} */
      const series = {
        active,
        setActive(value) {
          const wasActive = active.value;
          active.set(value);
          value ? show() : hide();
          if (value && !wasActive) {
            _fetch?.();
          }
          panes.updateVisibility();
        },
        setOrder,
        show,
        hide,
        highlight,
        tame,
        hasData: () => hasData,
        fetch: () => _fetch?.(),
        id,
        paneIndex,
        url: null,
        getData,
        update,
        remove() {
          onRemove();
          serieses.all.delete(series);
          panes.seriesByHome.get(paneIndex)?.delete(series);
        },
      };

      serieses.all.add(series);

      /** @param {ChartableIndex} idx */
      function setupIndexEffect(idx) {
        // Reset data state for new index
        hasData = false;
        lastTime = -Infinity;
        _fetch = null;

        const _valuesEndpoint = metric.by[idx];
        // Gracefully skip - series may be about to be removed by option change
        if (!_valuesEndpoint) return;
        const valuesEndpoint = _valuesEndpoint;

        series.url = `${baseUrl}${valuesEndpoint.path}`;

        (paneIndex ? legends.bottom : legends.top).addOrReplace({
          series,
          name,
          colors,
          order,
        });

        /**
         * @param {number[]} indexes
         * @param {(number | null | [number, number, number, number])[]} values
         */
        function processData(indexes, values) {
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
           * @returns {LineData | CandlestickData}
           */
          function buildDataPoint(i) {
            const time = /** @type {Time} */ (indexes[i]);
            const v = values[i];
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
              const point = buildDataPoint(i);
              if (sameTime && "open" in point) {
                const prev = /** @type {CandlestickData} */ (data[offsetedI]);
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
            lastTime = /** @type {number} */ (data.at(-1)?.time) ?? -Infinity;

            // Restore saved range or use defaults
            const savedRange = getRange();
            if (savedRange) {
              ichart.timeScale().setVisibleLogicalRange({
                from: savedRange.from,
                to: savedRange.to,
              });
            } else if (fitContent) {
              ichart.timeScale().fitContent();
            } else if (
              (minBarSpacingByIndex[idx] ?? 0) >=
              /** @type {number} */ (minBarSpacingByIndex.quarterindex)
            ) {
              ichart
                .timeScale()
                .setVisibleLogicalRange({ from: -1, to: data.length });
            }
            // Delay until chart has applied the range
            requestAnimationFrame(() => blueprints.onDataLoaded?.());
          } else {
            // Incremental update: only process new data points
            for (let i = startIdx; i < length; i++) {
              const point = buildDataPoint(i);
              update(point);
              lastTime = /** @type {number} */ (point.time);
            }
          }
        }

        async function fetchAndProcess() {
          const [timeResult, valuesResult] = await Promise.all([
            getTimeEndpoint(idx).slice(-10000).fetch(),
            valuesEndpoint.slice(-10000).fetch(),
          ]);
          if (valuesResult.stamp === lastStamp) {
            return;
          }
          lastStamp = valuesResult.stamp;
          if (timeResult.data.length && valuesResult.data.length) {
            processData(timeResult.data, valuesResult.data);
          }
        }

        _fetch = fetchAndProcess;

        // Initial fetch if active
        if (active.value) {
          fetchAndProcess();
        }
      }

      setupIndexEffect(index.get());
      // Series don't subscribe to index.onChange - panes recreates them on index change
      // index.onChange.add(setupIndexEffect);
      // _cleanup = () => index.onChange.delete(setupIndexEffect);

      return series;
    },

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
     * @param {number} [args.paneIndex]
     * @param {[Color, Color]} [args.colors] - [upColor, downColor] for legend
     * @param {boolean} [args.defaultActive]
     * @param {CandlestickSeriesPartialOptions} [args.options]
     */
    addCandlestick({
      metric,
      name,
      key,
      order,
      unit,
      paneIndex = 0,
      colors: customColors,
      defaultActive,
      options,
    }) {
      const upColor = customColors?.[0] ?? colors.green;
      const downColor = customColors?.[1] ?? colors.red;

      /** @type {CandlestickISeries} */
      const candlestickISeries = /** @type {any} */ (
        ichart.addSeries(
          /** @type {SeriesDefinition<'Candlestick'>} */ (CandlestickSeries),
          {
            visible: false,
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
            visible: false,
            lineWidth,
            priceLineVisible: true,
          },
          paneIndex,
        )
      );

      let active = defaultActive !== false;
      let highlighted = true;

      /** @param {number} barCount */
      const shouldShowLine = (barCount) => barCount > 500;
      let showLine = shouldShowLine(visibleBarsCount);

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

      /** @type {ZoomChangeCallback} */
      function handleZoom(count) {
        if (!series.hasData()) return; // Ignore zoom changes until data is ready
        const newShowLine = shouldShowLine(count);
        if (newShowLine === showLine) return;
        showLine = newShowLine;
        update();
      }
      onZoomChange.add(handleZoom);
      const removeSeriesThemeListener = onThemeChange(update);

      const series = serieses.create({
        colors: [upColor, downColor],
        name,
        key,
        order,
        paneIndex,
        unit,
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
          requestAnimationFrame(() => {
            const range = ichart.timeScale().getVisibleLogicalRange();
            if (range) {
              showLine = shouldShowLine(range.to - range.from);
            }
            update();
          });
        },
        update: (data) => {
          candlestickISeries.update(data);
          lineISeries.update({ time: data.time, value: data.close });
        },
        getData: () => candlestickISeries.data(),
        onRemove: () => {
          onZoomChange.delete(handleZoom);
          removeSeriesThemeListener();
          ichart.removeSeries(candlestickISeries);
          ichart.removeSeries(lineISeries);
        },
      });

      panes.register(paneIndex, series, [candlestickISeries, lineISeries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
     * @param {Color | [Color, Color]} [args.color] - Single color or [positive, negative] colors
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {HistogramSeriesPartialOptions} [args.options]
     */
    addHistogram({
      metric,
      name,
      key,
      color = [colors.green, colors.red],
      order,
      unit,
      paneIndex = 0,
      defaultActive,
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
      const removeSeriesThemeListener = onThemeChange(update);

      const series = serieses.create({
        colors: isDualColor ? [positiveColor, negativeColor] : [positiveColor],
        name,
        key,
        order,
        paneIndex,
        unit,
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
        onRemove: () => {
          removeSeriesThemeListener();
          ichart.removeSeries(iseries);
        },
      });

      panes.register(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
     * @param {Color} args.color
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {LineSeriesPartialOptions} [args.options]
     */
    addLine({
      metric,
      name,
      key,
      order,
      color,
      unit,
      paneIndex = 0,
      defaultActive,
      options,
    }) {
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
      const showLastValue = options?.lastValueVisible !== false;

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: showLastValue && highlighted,
          color: color.highlight(highlighted),
        });
      }
      update();
      const removeSeriesThemeListener = onThemeChange(update);

      const series = serieses.create({
        colors: [color],
        name,
        key,
        order,
        paneIndex,
        unit,
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
          removeSeriesThemeListener();
          ichart.removeSeries(iseries);
        },
      });

      panes.register(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
     * @param {Color} args.color
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {LineSeriesPartialOptions} [args.options]
     */
    addDots({
      metric,
      name,
      key,
      order,
      color,
      unit,
      paneIndex = 0,
      defaultActive,
      options,
    }) {
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
      let radius = getDotsRadius(visibleBarsCount);

      function update() {
        iseries.applyOptions({
          visible: active,
          lastValueVisible: highlighted,
          color: color.highlight(highlighted),
          pointMarkersRadius: radius,
        });
      }
      update();

      /** @type {ZoomChangeCallback} */
      function handleZoom(count) {
        const newRadius = getDotsRadius(count);
        if (newRadius === radius) return;
        radius = newRadius;
        iseries.applyOptions({ pointMarkersRadius: radius });
      }
      onZoomChange.add(handleZoom);
      const removeSeriesThemeListener = onThemeChange(update);

      const series = serieses.create({
        colors: [color],
        name,
        key,
        order,
        paneIndex,
        unit,
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
          removeSeriesThemeListener();
          ichart.removeSeries(iseries);
        },
      });

      panes.register(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Unit} args.unit
     * @param {string} [args.key] - Optional key for persistence (derived from name if not provided)
     * @param {number} [args.paneIndex]
     * @param {boolean} [args.defaultActive]
     * @param {Color} [args.topColor]
     * @param {Color} [args.bottomColor]
     * @param {BaselineSeriesPartialOptions} [args.options]
     */
    addBaseline({
      metric,
      name,
      key,
      order,
      unit,
      paneIndex: _paneIndex,
      defaultActive,
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
      const removeSeriesThemeListener = onThemeChange(update);

      const series = serieses.create({
        colors: [topColor, bottomColor],
        name,
        key,
        order,
        paneIndex,
        unit,
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
          removeSeriesThemeListener();
          ichart.removeSeries(iseries);
        },
      });

      panes.register(paneIndex, series, [iseries]);

      return series;
    },
  };

  /**
   * @param {number} paneIndex
   */
  function applyScaleForUnit(paneIndex) {
    const id = `${storageId}-scale`;
    const defaultValue = paneIndex === 0 ? "log" : "lin";

    const persisted = createPersistedValue({
      defaultValue: /** @type {"lin" | "log"} */ (defaultValue),
      storageKey: `${storageId}-p${paneIndex}-scale`,
      urlKey: paneIndex === 0 ? "price_scale" : "unit_scale",
      serialize: (v) => v,
      deserialize: (s) => /** @type {"lin" | "log"} */ (s),
    });

    /** @param {IPaneApi<Time>} pane @param {"lin" | "log"} value */
    const applyScale = (pane, value) => {
      try {
        pane.priceScale("right").applyOptions({
          mode: value === "lin" ? 0 : 1,
        });
      } catch {}
    };

    fieldsets.addIfNeeded({
      id,
      paneIndex,
      position: "sw",
      createChild(pane) {
        applyScale(pane, persisted.value);
        return createRadios({
          choices: /** @type {const} */ (["lin", "log"]),
          id: stringToId(`${id} ${paneIndex}`),
          initialValue: persisted.value,
          onChange(value) {
            persisted.set(value);
            applyScale(pane, value);
          },
        });
      },
    });
  }

  const blueprints = {
    /** @type {{ map: Map<Unit, AnyFetchedSeriesBlueprint[]>, series: AnySeries[], unit: Unit | null, legend: Legend }[]} */
    panes: [
      { map: new Map(), series: [], unit: null, legend: legends.top },
      { map: new Map(), series: [], unit: null, legend: legends.bottom },
    ],

    /** @type {VoidFunction | undefined} */
    onDataLoaded: undefined,

    /** @param {number} paneIndex */
    rebuildPane(paneIndex) {
      const pane = this.panes[paneIndex];
      const { map, series, unit, legend } = pane;

      if (!unit) {
        series.forEach((s) => s.remove());
        pane.series = [];
        legend.removeFrom(0);
        return;
      }

      const idx = index.get();
      legend.removeFrom(0);

      // Store old series to remove AFTER adding new ones
      // This prevents pane collapse which loses scale settings
      const oldSeries = [...series];
      pane.series = [];

      map.get(unit)?.forEach((blueprint, order) => {
        const options = blueprint.options;
        const indexes = Object.keys(blueprint.metric.by);

        const defaultColor = unit === Unit.usd ? colors.green : colors.orange;

        if (indexes.includes(idx)) {
          switch (blueprint.type) {
            case "Baseline": {
              pane.series.push(
                serieses.addBaseline({
                  metric: blueprint.metric,
                  name: blueprint.title,
                  key: blueprint.key,
                  defaultActive: blueprint.defaultActive,
                  paneIndex,
                  unit,
                  topColor: blueprint.colors?.[0] ?? blueprint.color,
                  bottomColor: blueprint.colors?.[1] ?? blueprint.color,
                  options,
                  order,
                }),
              );
              break;
            }
            case "Histogram": {
              pane.series.push(
                serieses.addHistogram({
                  metric: blueprint.metric,
                  name: blueprint.title,
                  key: blueprint.key,
                  color: blueprint.color,
                  defaultActive: blueprint.defaultActive,
                  paneIndex,
                  unit,
                  options,
                  order,
                }),
              );
              break;
            }
            case "Candlestick": {
              pane.series.push(
                serieses.addCandlestick({
                  metric: blueprint.metric,
                  name: blueprint.title,
                  key: blueprint.key,
                  colors: blueprint.colors,
                  defaultActive: blueprint.defaultActive,
                  paneIndex,
                  unit,
                  options,
                  order,
                }),
              );
              break;
            }
            case "Dots": {
              pane.series.push(
                serieses.addDots({
                  metric: blueprint.metric,
                  color: blueprint.color ?? defaultColor,
                  name: blueprint.title,
                  key: blueprint.key,
                  defaultActive: blueprint.defaultActive,
                  paneIndex,
                  unit,
                  options,
                  order,
                }),
              );
              break;
            }
            case "Line":
            case undefined:
              pane.series.push(
                serieses.addLine({
                  metric: blueprint.metric,
                  color: blueprint.color ?? defaultColor,
                  name: blueprint.title,
                  key: blueprint.key,
                  defaultActive: blueprint.defaultActive,
                  paneIndex,
                  unit,
                  options,
                  order,
                }),
              );
          }
        }
      });

      // Remove old series AFTER adding new ones to prevent pane collapse
      oldSeries.forEach((s) => s.remove());

      // Store scale config - it will be applied when createForPane runs after updateVisibility
      applyScaleForUnit(paneIndex);
    },

    rebuild() {
      this.rebuildPane(0);
      this.rebuildPane(1);
    },
  };

  // Rebuild when index changes
  index.onChange.add(() => blueprints.rebuild());

  const chart = {
    index,

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} args.top
     * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} args.bottom
     * @param {VoidFunction} [args.onDataLoaded]
     */
    setBlueprints({ name, top, bottom, onDataLoaded }) {
      storageId = stringToId(name);
      blueprints.panes[0].map = top;
      blueprints.panes[1].map = bottom;
      blueprints.onDataLoaded = onDataLoaded;

      // Set up unit selectors for each pane
      [top, bottom].forEach((map, paneIndex) => {
        const units = Array.from(map.keys());
        if (!units.length) {
          blueprints.panes[paneIndex].unit = null;
          return;
        }

        const defaultUnit = units[0];
        const persistedUnit = createPersistedValue({
          defaultValue: /** @type {string} */ (defaultUnit.id),
          storageKey: `${storageId}-p${paneIndex}-unit`,
          urlKey: paneIndex === 0 ? "u0" : "u1",
          serialize: (v) => v,
          deserialize: (s) => s,
        });

        // Find unit matching persisted value, or use default
        const initialUnit =
          units.find((u) => u.id === persistedUnit.value) ?? defaultUnit;
        blueprints.panes[paneIndex].unit = initialUnit;

        fieldsets.addIfNeeded({
          id: `${chartId}-unit`,
          paneIndex,
          position: "nw",
          createChild() {
            return createSelect({
              choices: units,
              id: `pane-${paneIndex}-unit`,
              initialValue: blueprints.panes[paneIndex].unit ?? defaultUnit,
              toKey: (u) => u.id,
              toLabel: (u) => u.name,
              sorted: true,
              onChange(unit) {
                persistedUnit.set(unit.id);
                blueprints.panes[paneIndex].unit = unit;
                blueprints.rebuildPane(paneIndex);
              },
            });
          },
        });
      });

      blueprints.rebuild();

      return blueprints;
    },

    destroy() {
      removeThemeListener();
      clearInterval(refreshInterval);
      ichart.remove();
    },
  };

  const captureButton = document.createElement("button");
  captureButton.className = "capture";
  captureButton.innerText = "capture";
  captureButton.title = "Capture chart as image";
  captureButton.addEventListener("click", () => {
    capture({
      screenshot: ichart.takeScreenshot(),
      chartWidth: elements.chart.clientWidth,
      parent,
      legends,
    });
  });
  elements.chart.append(captureButton);

  return chart;
}

/**
 * @typedef {typeof createChart} CreateChart
 * @typedef {ReturnType<createChart>} Chart
 */
