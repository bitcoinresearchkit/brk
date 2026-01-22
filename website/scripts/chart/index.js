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
import { colors } from "./colors.js";

const lcCreateChart = /** @type {CreateLCChart} */ (untypedLcCreateChart);
import { createChoiceField } from "../utils/dom.js";
import { createPersistedValue } from "../utils/persisted.js";
import { onChange as onThemeChange } from "../utils/theme.js";
import { throttle, debounce } from "../utils/timing.js";
import { serdeBool, serdeChartableIndex } from "../utils/serde.js";
import { stringToId, numberToShortUSFormat } from "../utils/format.js";
import { style } from "../utils/elements.js";

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
 * @param {HTMLElement} [args.captureElement]
 * @param {{unit: Unit; blueprints: AnyFetchedSeriesBlueprint[]}[]} [args.config]
 */
export function createChart({
  parent,
  id: chartId,
  brk,
  fitContent,
  captureElement,
  config,
}) {
  // Chart owns its index state
  /** @type {Set<(index: ChartableIndex) => void>} */
  const onIndexChange = new Set();

  const index = () => serdeChartableIndex.deserialize(indexName.value);

  const indexName = createPersistedValue({
    defaultValue: /** @type {ChartableIndexName} */ ("date"),
    storageKey: "chart-index",
    urlKey: "i",
    serialize: (v) => v,
    deserialize: (s) => /** @type {ChartableIndexName} */ (s),
    onChange: () => {
      // Reset URL range so getRange() falls back to per-index saved range
      range.set(null);
      onIndexChange.forEach((cb) => cb(index()));
    },
  });

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
  const getRange = () => range.value ?? ranges.value[indexName.value] ?? null;

  /** @param {Range} value */
  const setRange = (value) => {
    ranges.set({ ...ranges.value, [indexName.value]: value });
    range.set(value);
  };

  const div = window.document.createElement("div");
  div.classList.add("chart");
  parent.append(div);

  // Registry for shared active states (same name = linked across panes)
  /** @type {Map<string, PersistedValue<boolean>>} */
  const sharedActiveStates = new Map();

  // Registry for linked series (same key = linked across panes)
  /** @type {Map<string, Set<AnySeries>>} */
  const seriesByKey = new Map();

  // Track series by their home pane for pane collapse management
  /** @type {Map<number, Map<AnySeries, ISeries[]>>} */
  const seriesByHomePane = new Map();


  /**
   * Register series with its home pane for collapse management
   * @param {number} paneIndex
   * @param {AnySeries} series
   * @param {ISeries[]} iseries
   */
  function registerSeriesPane(paneIndex, series, iseries) {
    let paneMap = seriesByHomePane.get(paneIndex);
    if (!paneMap) {
      paneMap = new Map();
      seriesByHomePane.set(paneIndex, paneMap);
    }
    paneMap.set(series, iseries);

    // Create fieldsets when pane becomes active (first series or after restore)
    if (!panesWithFieldsets.has(paneIndex)) {
      setTimeout(() => createPaneFieldsets(paneIndex), paneIndex ? 50 : 0);
    }
  }

  /**
   * Check if pane should collapse (all series hidden) and move series accordingly
   * @param {number} homePane
   */
  function updatePaneVisibility(homePane) {
    const paneMap = seriesByHomePane.get(homePane);
    if (!paneMap || paneMap.size === 0) return;

    const allHidden = [...paneMap.keys()].every((s) => !s.active.value);

    if (homePane === 0) {
      // For pane 0: manage pane 1 series based on visibility of both panes
      const pane1Map = seriesByHomePane.get(1);
      if (!pane1Map || pane1Map.size === 0) return;

      const pane1AllHidden = [...pane1Map.keys()].every((s) => !s.active.value);

      // Determine what fieldsets should show on physical pane 0
      // and whether pane 1 should exist
      if (allHidden && !pane1AllHidden) {
        // Pane 0 hidden, pane 1 visible: show pane 1 content/fieldsets on pane 0
        for (const iseries of pane1Map.values()) {
          for (const is of iseries) {
            if (is.getPane().paneIndex() !== 0) {
              is.moveToPane(0);
            }
          }
        }
        panesWithFieldsets.delete(0);
        panesWithFieldsets.delete(1);
        setTimeout(() => createPaneFieldsets(1, 0), 50);
      } else if (!allHidden && !pane1AllHidden) {
        // Both visible: pane 0 on pane 0, pane 1 on pane 1
        for (const iseries of pane1Map.values()) {
          for (const is of iseries) {
            if (is.getPane().paneIndex() === 0) {
              is.moveToPane(1);
            }
          }
        }
        panesWithFieldsets.delete(0);
        panesWithFieldsets.delete(1);
        setTimeout(() => {
          createPaneFieldsets(0);
          createPaneFieldsets(1);
        }, 50);
      } else if (!allHidden && pane1AllHidden) {
        // Pane 0 visible, pane 1 hidden: show pane 0 fieldsets, pane 1 collapsed
        for (const iseries of pane1Map.values()) {
          for (const is of iseries) {
            if (is.getPane().paneIndex() !== 0) {
              is.moveToPane(0);
            }
          }
        }
        panesWithFieldsets.delete(0);
        panesWithFieldsets.delete(1);
        setTimeout(() => createPaneFieldsets(0), 50);
      }
      // If both hidden: leave as-is, show pane 0 fieldsets (already there)
    } else {
      // For pane 1: move series to pane 0 when hidden, back when visible
      const pane0Map = seriesByHomePane.get(0);
      const pane0AllHidden = pane0Map ? [...pane0Map.keys()].every((s) => !s.active.value) : true;

      if (allHidden) {
        // Pane 1 hidden: move to pane 0
        for (const iseries of paneMap.values()) {
          for (const is of iseries) {
            if (is.getPane().paneIndex() !== 0) {
              is.moveToPane(0);
            }
          }
        }
        panesWithFieldsets.delete(homePane);
        // Update pane 0 fieldsets based on what's visible
        if (pane0AllHidden) {
          // Both hidden: keep pane 0 fieldsets
        } else {
          // Pane 0 visible: show pane 0 fieldsets
          panesWithFieldsets.delete(0);
          setTimeout(() => createPaneFieldsets(0), 50);
        }
      } else {
        // Pane 1 visible: move back to pane 1 if pane 0 is also visible
        if (!pane0AllHidden) {
          for (const iseries of paneMap.values()) {
            for (const is of iseries) {
              if (is.getPane().paneIndex() === 0) {
                is.moveToPane(homePane);
              }
            }
          }
          panesWithFieldsets.delete(0);
          panesWithFieldsets.delete(homePane);
          setTimeout(() => {
            createPaneFieldsets(0);
            createPaneFieldsets(homePane);
          }, 50);
        } else {
          // Pane 0 hidden, pane 1 visible: show pane 1 fieldsets on pane 0
          panesWithFieldsets.delete(0);
          panesWithFieldsets.delete(homePane);
          setTimeout(() => createPaneFieldsets(homePane, 0), 50);
        }
      }
    }
  }

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

  /** @typedef {(visibleBarsCount: number) => void} ZoomChangeCallback */

  const initialRange = getRange();
  if (initialRange) {
    ichart.timeScale().setVisibleLogicalRange(initialRange);
  }

  let visibleBarsCount = initialRange
    ? initialRange.to - initialRange.from
    : Infinity;

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

  /** @param {ChartableIndex} index */
  function applyIndexSettings(index) {
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
  }
  applyIndexSettings(index());
  onIndexChange.add(applyIndexSettings);

  // Periodic refresh of active series data
  setInterval(() => {
    seriesByKey.forEach((set) => {
      set.forEach((s) => {
        if (s.active.value) s.fetch?.();
      });
    });
  }, 30_000);

  if (fitContent) {
    new ResizeObserver(() => ichart.timeScale().fitContent()).observe(chartDiv);
  }

  /**
   * @typedef {Object} FieldsetConfig
   * @property {string} id
   * @property {"nw" | "ne" | "se" | "sw"} position
   * @property {(pane: IPaneApi<Time>) => HTMLElement} createChild
   */

  /** @type {Map<number, Map<string, FieldsetConfig>>} */
  const paneFieldsetConfigs = new Map();

  /** @type {Set<number>} */
  const panesWithFieldsets = new Set();

  /**
   * Create all fieldsets for a logical pane on a physical pane
   * @param {number} configPaneIndex - which pane's config to use
   * @param {number} [targetPaneIndex] - which physical pane to create on (defaults to configPaneIndex)
   */
  function createPaneFieldsets(configPaneIndex, targetPaneIndex = configPaneIndex) {
    const pane = ichart.panes().at(targetPaneIndex);
    if (!pane) return;

    const parent = pane.getHTMLElement()?.children?.item(1)?.firstChild;
    if (!parent) return;

    const configs = paneFieldsetConfigs.get(configPaneIndex);
    if (!configs) return;

    for (const { id, position, createChild } of configs.values()) {
      // Remove existing at same position
      Array.from(parent.childNodes)
        .filter((el) => /** @type {HTMLElement} */ (el).dataset?.position === position)
        .forEach((el) => el.remove());

      const fieldset = window.document.createElement("fieldset");
      fieldset.dataset.size = "xs";
      fieldset.dataset.position = position;
      fieldset.id = `${id}-${configPaneIndex}`;
      parent.appendChild(fieldset);
      fieldset.append(createChild(pane));
    }

    panesWithFieldsets.add(configPaneIndex);
  }

  /**
   * Register a fieldset config for a pane (created when pane becomes active)
   * @param {Object} args
   * @param {string} args.id
   * @param {number} args.paneIndex
   * @param {"nw" | "ne" | "se" | "sw"} args.position
   * @param {(pane: IPaneApi<Time>) => HTMLElement} args.createChild
   */
  function addFieldsetIfNeeded({ paneIndex, id, position, createChild }) {
    let configs = paneFieldsetConfigs.get(paneIndex);
    if (!configs) {
      configs = new Map();
      paneFieldsetConfigs.set(paneIndex, configs);
    }
    configs.set(id, { id, position, createChild });
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
        /** @type {"lin" | "log"} */
        const defaultValue =
          unit.id === "usd" && seriesType !== "Baseline" ? "log" : "lin";

        const persisted = createPersistedValue({
          defaultValue,
          storageKey: `${id}-scale-${paneIndex}`,
          urlKey: paneIndex === 0 ? "price_scale" : "unit_scale",
          serialize: (v) => v,
          deserialize: (s) => /** @type {"lin" | "log"} */ (s),
        });

        /** @param {"lin" | "log"} value */
        const applyScale = (value) => {
          try {
            pane.priceScale("right").applyOptions({
              mode: value === "lin" ? 0 : 1,
            });
          } catch {}
        };

        // Apply initial value
        applyScale(persisted.value);

        const field = createChoiceField({
          choices: /** @type {const} */ (["lin", "log"]),
          id: stringToId(`${id} ${paneIndex} ${unit}`),
          initialValue: persisted.value,
          onChange(value) {
            persisted.set(value);
            applyScale(value);
          },
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
   * @param {AnyMetricPattern} args.metric
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
   * @param {() => void} [args.onDataLoaded]
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
    setOrder,
    show,
    hide,
    highlight,
    tame,
    getData,
    setData,
    update,
    onRemove,
    onDataLoaded,
  }) {
    const key = stringToId(name);
    const id = `${key}-${paneIndex}`;

    // Reuse existing state if same name (links legends across panes)
    const existingActive = sharedActiveStates.get(key);
    const active =
      existingActive ??
      createPersistedValue({
        defaultValue: defaultActive ?? true,
        storageKey: id,
        urlKey: key,
        ...serdeBool,
      });
    if (!existingActive) sharedActiveStates.set(key, active);

    setOrder(-order);

    active.value ? show() : hide();

    let hasData = false;
    let lastTime = -Infinity;

    /** @type {VoidFunction | null} */
    let _fetch = null;

    /** @type {AnySeries} */
    const series = {
      active,
      setActive(value) {
        const wasActive = active.value;
        active.set(value);
        seriesByKey.get(key)?.forEach((s) => {
          value ? s.show() : s.hide();
        });
        document.querySelectorAll(`[data-series="${id}"]`).forEach((el) => {
          if (el instanceof HTMLInputElement && el.type === "checkbox") {
            el.checked = value;
          }
        });
        if (value && !wasActive) _fetch?.();
        updatePaneVisibility(paneIndex);
      },
      setOrder,
      show,
      hide,
      highlight,
      tame,
      hasData: () => hasData,
      fetch: () => _fetch?.(),
      key,
      id,
      paneIndex,
      url: null,
      getData,
      update,
      remove() {
        onRemove();
        seriesByKey.get(key)?.delete(series);
      },
    };

    // Register series for cross-pane linking
    let keySet = seriesByKey.get(key);
    if (!keySet) {
      keySet = new Set();
      seriesByKey.set(key, keySet);
    }
    keySet.add(series);

    /** @param {ChartableIndex} idx */
    function setupIndexEffect(idx) {
      // Reset data state for new index
      hasData = false;
      lastTime = -Infinity;
      _fetch = null;

      // Get timestamp metric from tree based on index type
      const timeMetric =
        idx === "height"
          ? brk.metrics.blocks.time.timestampMonotonic
          : brk.metrics.blocks.time.timestamp;
      const valuesMetric = /** @type {AnyMetricPattern} */ (metric);
      const _timeEndpoint = timeMetric.get(idx);
      if (!_timeEndpoint) throw "Expect time endpoint";
      const timeEndpoint = _timeEndpoint;
      const valuesEndpoint = valuesMetric.by[idx];
      // Gracefully skip - series may be about to be removed by option change
      if (!timeEndpoint || !valuesEndpoint) return;

      series.url = `${
        brk.baseUrl.endsWith("/") ? brk.baseUrl.slice(0, -1) : brk.baseUrl
      }${valuesEndpoint.path}`;

      (paneIndex ? legendBottom : legendTop).addOrReplace({
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
            idx === "quarterindex" ||
            idx === "semesterindex" ||
            idx === "yearindex" ||
            idx === "decadeindex"
          ) {
            ichart
              .timeScale()
              .setVisibleLogicalRange({ from: -1, to: data.length });
          }
          // Delay until chart has applied the range
          requestAnimationFrame(() => onDataLoaded?.());
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
          timeEndpoint.slice(-10000).fetch(),
          valuesEndpoint?.slice(-10000).fetch(),
        ]);
        if (timeResult?.data?.length && valuesResult?.data?.length) {
          processData(timeResult.data, valuesResult.data);
        }
      }

      _fetch = fetchAndProcess;

      // Initial fetch if active
      if (active.value) {
        fetchAndProcess();
      }
    }

    setupIndexEffect(index());
    // Series don't subscribe to onIndexChange - panes recreates them on index change
    // onIndexChange.add(setupIndexEffect);
    // _cleanup = () => onIndexChange.delete(setupIndexEffect);

    addPriceScaleSelectorIfNeeded({
      paneIndex,
      seriesType,
      unit,
    });

    return series;
  }

  const chart = {
    index,
    indexName,
    onIndexChange,

    legendTop,
    legendBottom,

    addFieldsetIfNeeded,

    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
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
      let showLine = visibleBarsCount > 500;
      let dataLoaded = false;

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
        if (!dataLoaded) return; // Ignore zoom changes until data is ready
        const newShowLine = count > 500;
        if (newShowLine === showLine) return;
        showLine = newShowLine;
        update();
      }
      onZoomChange.add(handleZoom);
      const removeSeriesThemeListener = onThemeChange(update);

      const series = addSeries({
        colors: [upColor, downColor],
        name,
        order,
        paneIndex,
        seriesType: "Candlestick",
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
          seriesByHomePane.get(paneIndex)?.delete(series);
        },
        onDataLoaded: () => {
          dataLoaded = true;
          update();
        },
      });

      registerSeriesPane(paneIndex, series, [candlestickISeries, lineISeries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
     * @param {Color | [Color, Color]} [args.color] - Single color or [positive, negative] colors
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

      const series = addSeries({
        colors: isDualColor ? [positiveColor, negativeColor] : [positiveColor],
        name,
        order,
        paneIndex,
        seriesType: "Bar",
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
          seriesByHomePane.get(paneIndex)?.delete(series);
        },
      });

      registerSeriesPane(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
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
      const removeSeriesThemeListener = onThemeChange(update);

      const series = addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
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
          seriesByHomePane.get(paneIndex)?.delete(series);
        },
      });

      registerSeriesPane(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
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
      const removeSeriesThemeListener = onThemeChange(update);

      const series = addSeries({
        colors: [color],
        name,
        order,
        paneIndex,
        seriesType: "Line",
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
          seriesByHomePane.get(paneIndex)?.delete(series);
        },
      });

      registerSeriesPane(paneIndex, series, [iseries]);

      return series;
    },
    /**
     * @param {Object} args
     * @param {string} args.name
     * @param {Unit} args.unit
     * @param {number} args.order
     * @param {AnyMetricPattern} args.metric
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

      const series = addSeries({
        colors: [topColor, bottomColor],
        name,
        order,
        paneIndex,
        seriesType: "Baseline",
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
          seriesByHomePane.get(paneIndex)?.delete(series);
        },
      });

      registerSeriesPane(paneIndex, series, [iseries]);

      return series;
    },

    destroy() {
      removeThemeListener();
      ichart.remove();
    },
  };

  config?.forEach(({ unit, blueprints }, paneIndex) => {
    blueprints.forEach((blueprint, order) => {
      if (blueprint.type === "Candlestick") {
        chart.addCandlestickSeries({
          metric: blueprint.metric,
          name: blueprint.title,
          unit,
          colors: blueprint.colors,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          options: blueprint.options,
          order,
        });
      } else if (blueprint.type === "Baseline") {
        chart.addBaselineSeries({
          metric: blueprint.metric,
          name: blueprint.title,
          unit,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          options: blueprint.options,
          order,
        });
      } else if (blueprint.type === "Histogram") {
        chart.addHistogramSeries({
          metric: blueprint.metric,
          name: blueprint.title,
          unit,
          color: blueprint.color,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          options: blueprint.options,
          order,
        });
      } else if (blueprint.type === "Dots") {
        chart.addDotsSeries({
          metric: blueprint.metric,
          name: blueprint.title,
          unit,
          color: blueprint.color,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          options: blueprint.options,
          order,
        });
      } else {
        chart.addLineSeries({
          metric: blueprint.metric,
          name: blueprint.title,
          unit,
          defaultActive: blueprint.defaultActive,
          paneIndex,
          color: blueprint.color,
          options: blueprint.options,
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
