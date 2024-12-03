// @ts-check

/**
 * @import { DeepPartial, ChartOptions, IChartApi, IHorzScaleBehavior, WhitespaceData, SingleValueData, ISeriesApi, Time, LogicalRange, SeriesType, BaselineStyleOptions, SeriesOptionsCommon, createChart as CreateClassicChart, createChartEx as CreateCustomChart } from "./v4.2.0/types"
 */

const ids = {
  from: "from",
  to: "to",
  chartRange: "chart-range",
  /**
   * @param {TimeScale} scale
   */
  visibleTimeRange(scale) {
    return `${ids.chartRange}-${scale}`;
  },
};

export default import("./v4.2.0/script.js").then((lightweightCharts) => {
  const createClassicChart = /** @type {CreateClassicChart} */ (
    lightweightCharts.createChart
  );
  const createCustomChart = /** @type {CreateCustomChart} */ (
    lightweightCharts.createChartEx
  );

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
      return tickMarks.reduce(this.getMarkWithGreaterWeight, tickMarks[0])
        .weight;
    }

    /**
     * @param {any} sortedTimePoints
     * @param {number} startIndex
     */
    fillWeightsForPoints(sortedTimePoints, startIndex) {
      for (let index = startIndex; index < sortedTimePoints.length; ++index) {
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
   * @param {Utilities} args.utils
   * @param {DeepPartial<ChartOptions>} [args.options]
   */
  function createLightweightChart({
    scale,
    element,
    signals,
    colors,
    utils,
    options: _options = {},
  }) {
    /** @satisfies {DeepPartial<ChartOptions>} */
    const options = {
      autoSize: true,
      layout: {
        fontFamily: "Satoshi Chart",
        fontSize: 13,
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
      localization: {
        priceFormatter: utils.locale.numberToShortUSFormat,
        locale: "en-us",
      },
      ..._options,
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

    signals.createEffect(
      () => ({
        defaultColor: colors.default(),
        offColor: colors.off(),
      }),
      ({ defaultColor, offColor }) => {
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
      },
    );

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

  function initWhitespace() {
    const whitespaceStartDate = new Date("1970-01-01");
    const whitespaceStartDateYear = whitespaceStartDate.getUTCFullYear();
    const whitespaceStartDateMonth = whitespaceStartDate.getUTCMonth();
    const whitespaceStartDateDate = whitespaceStartDate.getUTCDate();
    const whitespaceEndDate = new Date("2141-01-01");
    let whitespaceDateDataset =
      /** @type {(WhitespaceData | SingleValueData)[]} */ ([]);

    /**
     * @param {Object} param0
     * @param {Utilities} param0.utils
     */
    function initDateWhitespace({ utils }) {
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

        const time = utils.date.toString(date);

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
    let whitespaceHeightDataset = /** @type {WhitespaceData[]} */ ([]);

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
     * @param {Object} param0
     * @param {IChartApi} param0.chart
     * @param {TimeScale} param0.scale
     * @param {Utilities} param0.utils
     * @returns {ISeriesApi<'Line'>}
     */
    function setWhitespace({ chart, scale, utils }) {
      const whitespace = chart.addLineSeries();

      if (scale === "date") {
        if (!whitespaceDateDataset.length) {
          initDateWhitespace({ utils });
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
   * @typeof {Object} PaneParameters
   * @property {Unit} param.unit
   * @param {TimeScale} param.scale
   * @param {number} [param.chartIndex]
   * @param {true} [param.whitespace]
   * @param {DeepPartial<ChartOptions>} [param.options]
   */

  /**
   * @param {Object} param0
   * @param {string} param0.id
   * @param {HTMLElement} param0.parent
   * @param {Signals} param0.signals
   * @param {Colors} param0.colors
   * @param {TimeScale} param0.scale
   * @param {"static" | "moveable"} param0.kind
   * @param {Utilities} param0.utils
   * @param {Owner | null} [param0.owner]
   * @param {CreatePaneParameters[]} [param0.config]
   */
  function createChart({
    parent,
    signals,
    colors,
    id: chartId,
    kind,
    scale,
    config,
    utils,
    owner: _owner,
  }) {
    /** @type {SplitSeries[]} */
    const chartSplitSeries = [];

    let owner = _owner || signals.getOwner();

    const div = window.document.createElement("div");
    div.classList.add("chart");
    parent.append(div);

    const legendElement = window.document.createElement("legend");
    div.append(legendElement);

    /**
     * @returns {TimeRange}
     */
    function getInitialVisibleTimeRange() {
      const urlParams = new URLSearchParams(window.location.search);
      const urlFrom = urlParams.get(ids.from);
      const urlTo = urlParams.get(ids.to);

      if (urlFrom && urlTo) {
        if (scale === "date" && urlFrom.includes("-") && urlTo.includes("-")) {
          console.log({
            from: new Date(urlFrom).toJSON().split("T")[0],
            to: new Date(urlTo).toJSON().split("T")[0],
          });
          return {
            from: new Date(urlFrom).toJSON().split("T")[0],
            to: new Date(urlTo).toJSON().split("T")[0],
          };
        } else if (
          scale === "height" &&
          (!urlFrom.includes("-") || !urlTo.includes("-"))
        ) {
          console.log({
            from: Number(urlFrom),
            to: Number(urlTo),
          });
          return {
            from: Number(urlFrom),
            to: Number(urlTo),
          };
        }
      }

      function getSavedTimeRange() {
        return /** @type {TimeRange | null} */ (
          JSON.parse(
            localStorage.getItem(ids.visibleTimeRange(scale)) || "null",
          )
        );
      }

      const savedTimeRange = getSavedTimeRange();

      console.log(savedTimeRange);

      if (savedTimeRange) {
        return savedTimeRange;
      }

      function getDefaultTimeRange() {
        switch (scale) {
          case "date": {
            const defaultTo = new Date();
            const defaultFrom = new Date();
            defaultFrom.setDate(defaultFrom.getUTCDate() - 6 * 30);

            return {
              from: defaultFrom.toJSON().split("T")[0],
              to: defaultTo.toJSON().split("T")[0],
            };
          }
          case "height": {
            return {
              from: 850_000,
              to: 900_000,
            };
          }
        }
      }

      return getDefaultTimeRange();
    }

    const visibleTimeRange = signals.createSignal(getInitialVisibleTimeRange());

    const visibleDatasetIds = signals.createSignal(
      /** @type {number[]} */ ([]),
      {
        equals: false,
      },
    );

    const lastVisibleDatasetIndex = signals.createMemo(() => {
      const last = visibleDatasetIds().at(-1);
      return last !== undefined ? utils.chunkIdToIndex(scale, last) : undefined;
    });

    function updateVisibleDatasetIds() {
      /** @type {number[]} */
      let ids = [];

      const today = new Date();
      const { from: rawFrom, to: rawTo } = visibleTimeRange();

      if (typeof rawFrom === "string" && typeof rawTo === "string") {
        const from = new Date(rawFrom).getUTCFullYear();
        const to = new Date(rawTo).getUTCFullYear();

        ids = Array.from({ length: to - from + 1 }, (_, i) => i + from).filter(
          (year) => year >= 2009 && year <= today.getUTCFullYear(),
        );
      } else {
        const from = Math.floor(Number(rawFrom) / consts.HEIGHT_CHUNK_SIZE);
        const to = Math.floor(Number(rawTo) / consts.HEIGHT_CHUNK_SIZE);

        const length = to - from + 1;

        ids = Array.from(
          { length },
          (_, i) => (from + i) * consts.HEIGHT_CHUNK_SIZE,
        );
      }

      const old = visibleDatasetIds();

      if (
        old.length !== ids.length ||
        old.at(0) !== ids.at(0) ||
        old.at(-1) !== ids.at(-1)
      ) {
        console.log("range:", ids);

        visibleDatasetIds.set(ids);
      }
    }
    const debouncedUpdateVisibleDatasetIds = utils.debounce(
      updateVisibleDatasetIds,
      100,
    );

    function saveVisibleRange() {
      const range = visibleTimeRange();
      utils.url.writeParam(ids.from, String(range.from));
      utils.url.writeParam(ids.to, String(range.to));
      localStorage.setItem(ids.visibleTimeRange(scale), JSON.stringify(range));
    }
    const debouncedSaveVisibleRange = utils.debounce(saveVisibleRange, 250);

    const hoveredLegend = signals.createSignal(
      /** @type {HoveredLegend | undefined} */ (undefined),
    );
    const notHoveredLegendTransparency = "66";
    /**
     * @param {Object} args
     * @param {SingleSeries | SplitSeries} args.series
     * @param {string} [args.extraName]
     */
    function createLegend({ series, extraName }) {
      const div = window.document.createElement("div");

      if ("disabled" in series) {
        signals.createEffect(series.disabled, (disabled) => {
          div.hidden = disabled;
        });
      }

      legendElement.prepend(div);

      extraName ||= "Line";

      const { input, label } = utils.dom.createLabeledInput({
        inputId: utils.stringToId(`legend-${series.title}-${extraName}`),
        inputName: utils.stringToId(`selected-${series.title}-${extraName}`),
        inputValue: "value",
        labelTitle: "Click to toggle",
        onClick: () => {
          series.active.set(input.checked);
        },
        type: "solo",
      });

      input.checked = series.active();

      const spanMain = window.document.createElement("span");
      spanMain.classList.add("main");
      label.append(spanMain);

      const spanName = utils.dom.createSpanName(series.title);
      spanMain.append(spanName);

      div.append(label);
      label.addEventListener("mouseover", () => {
        const hovered = hoveredLegend();

        if (!hovered || hovered.label !== label) {
          hoveredLegend.set({ label, series });
        }
      });
      label.addEventListener("mouseleave", () => {
        hoveredLegend.set(undefined);
      });

      function shouldHighlight() {
        const hovered = hoveredLegend();
        return (
          !hovered ||
          (hovered.label === label && hovered.series.active()) ||
          (hovered.label !== label && !hovered.series.active())
        );
      }

      const spanColors = window.document.createElement("span");
      spanColors.classList.add("colors");
      spanMain.prepend(spanColors);
      const colors = Array.isArray(series.color)
        ? series.color
        : [series.color];
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
              spanColor.style.backgroundColor = `${color}${notHoveredLegendTransparency}`;
            }
          },
        );
      });

      function createHoverEffect() {
        const initialColors = /** @type {Record<string, any>} */ ({});
        const darkenedColors = /** @type {Record<string, any>} */ ({});

        /** @type {HoveredLegend | undefined} */
        let previouslyHovered = undefined;

        /**
         * @param {Object} param0
         * @param {HoveredLegend | undefined} param0.hovered
         * @param {ISeriesApi<SeriesType> | undefined} param0.series
         * @param {number} [param0.seriesIndex]
         */
        function applySeriesOption({ hovered, series, seriesIndex = 0 }) {
          if (!series) return;

          const i = seriesIndex;

          if (hovered) {
            const seriesOptions = series.options();
            if (!seriesOptions) return;

            initialColors[i] = {};
            darkenedColors[i] = {};

            Object.entries(seriesOptions).forEach(([k, v]) => {
              if (k.toLowerCase().includes("color") && v) {
                if (typeof v === "string" && !v.startsWith("#")) {
                  return;
                }

                v = /** @type {string} */ (v).substring(0, 7);
                initialColors[i][k] = v;
                darkenedColors[i][k] = `${v}${notHoveredLegendTransparency}`;
              } else if (k === "lastValueVisible" && v) {
                initialColors[i][k] = true;
                darkenedColors[i][k] = false;
              }
            });
          }

          signals.createEffect(shouldHighlight, (shouldHighlight) => {
            if (shouldHighlight) {
              series.applyOptions(initialColors[i]);
            } else {
              series.applyOptions(darkenedColors[i]);
            }
          });
        }

        signals.createEffect(
          () => ({
            hovered: hoveredLegend(),
            ids: visibleDatasetIds(),
          }),
          ({ hovered, ids }) => {
            if (!hovered && !previouslyHovered) return hovered;

            if ("chunks" in series) {
              for (let i = 0; i < ids.length; i++) {
                const chunkId = ids[i];
                const chunkIndex = utils.chunkIdToIndex(scale, chunkId);
                const chunk = series.chunks[chunkIndex];

                signals.createEffect(chunk, (chunk) => {
                  applySeriesOption({
                    hovered,
                    series: chunk,
                    seriesIndex: i,
                  });
                });
              }
            } else {
              applySeriesOption({
                series: series.iseries,
                hovered,
              });
            }

            previouslyHovered = hovered;
          },
        );
      }
      createHoverEffect();

      if ("dataset" in series) {
        const anchor = window.document.createElement("a");
        anchor.href = series.dataset.url;
        anchor.target = "_blank";
        anchor.rel = "noopener noreferrer";
        div.append(anchor);
      }
    }

    const panesElement = window.document.createElement("div");
    panesElement.classList.add("panes");
    div.append(panesElement);

    /** @type {ChartPane[]} */
    const panes = [];

    if (kind === "static") {
      new ResizeObserver(() => {
        panes.forEach((chart) => {
          chart.timeScale().fitContent();
        });
      }).observe(panesElement);
    }

    /**
     * @param {CreatePaneParameters} param
     */
    function createPane({ paneIndex, whitespace, unit, options, config }) {
      const chartWrapper = window.document.createElement("div");
      chartWrapper.classList.add("pane");
      panesElement.append(chartWrapper);

      const chartDiv = window.document.createElement("div");
      chartDiv.classList.add("lightweight-chart");
      chartWrapper.append(chartDiv);

      /** @type {SplitSeries[]} */
      const paneSplitSeries = [];

      options = { ...options };
      if (kind === "static") {
        options.handleScale = false;
        options.handleScroll = false;
      } else {
        options.crosshair = {
          ...options.crosshair,
          mode: 0,
        };
      }

      const _chart = createLightweightChart({
        scale,
        element: chartDiv,
        signals,
        colors,
        options,
        utils,
      });

      /**
       * @param {RemoveSeriesBlueprintFluff<BaselineSeriesBlueprint>} args
       */
      function createBaseLineSeries({ color, options, data }) {
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

        const series = _chart.addBaselineSeries(seriesOptions);

        signals.runWithOwner(owner, () => {
          signals.createEffect(computeColors, (computeColors) => {
            series.applyOptions(computeColors);
          });
        });

        if (data) {
          signals.runWithOwner(owner, () => {
            signals.createEffect(data, (data) => {
              series.setData(data);
            });
          });
        }

        return series;
      }

      /**
       * @param {RemoveSeriesBlueprintFluff<CandlestickSeriesBlueprint>} args
       */
      function createCandlestickSeries({ options, data }) {
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

        const series = _chart.addCandlestickSeries({
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
          signals.createEffect(computeColors, (computeColors) => {
            series.applyOptions(computeColors);
          });
        });

        if (data) {
          signals.runWithOwner(owner, () => {
            signals.createEffect(data, (data) => {
              series.setData(data);
            });
          });
        }

        return series;
      }

      /**
       * @param {RemoveSeriesBlueprintFluff<LineSeriesBlueprint>} args
       */
      function createLineSeries({ color, options, data }) {
        function computeColors() {
          return {
            color: color(),
          };
        }

        const series = _chart.addLineSeries({
          ...defaultSeriesOptions,
          ...options,
          ...computeColors(),
        });

        signals.runWithOwner(owner, () => {
          signals.createEffect(computeColors, (computeColors) => {
            series.applyOptions(computeColors);
          });
        });

        if (data) {
          signals.runWithOwner(owner, () => {
            signals.createEffect(data, (data) => {
              series.setData(data);
            });
          });
        }

        return series;
      }

      /**
       * @template {TimeScale} S
       * @param {CreateBaseSeriesParameters} args
       */
      function createBaseSeries({
        id,
        disabled: _disabled,
        title,
        color,
        defaultActive,
      }) {
        const keyPrefix = id;
        const paramKey = utils.stringToId(title);
        const storageKey = `${keyPrefix}-${paramKey}`;

        const active = signals.createSignal(
          utils.url.readBoolParam(paramKey) ??
            utils.storage.readBool(storageKey) ??
            defaultActive ??
            true,
        );

        const disabled = signals.createMemo(_disabled || (() => false));

        const visible = signals.createMemo(() => active() && !disabled());

        /** @satisfies {BaseSeries} */
        const series = {
          active,
          color: color || [colors.profit, colors.loss],
          disabled,
          id,
          title,
          visible,
        };

        signals.createEffect(
          () => ({ disabled: disabled(), active: active() }),
          ({ disabled, active }) => {
            if (disabled) {
              return;
            }

            if (active !== (defaultActive || true)) {
              utils.url.writeParam(paramKey, active);
              utils.storage.write(storageKey, active);
            } else {
              utils.url.removeParam(paramKey);
              utils.storage.remove(storageKey);
            }
          },
        );

        return series;
      }

      const chartPane = /** @type {ChartPane} */ (_chart);

      chartPane.createSingleSeries = function ({ blueprint, id }) {
        /** @type {ISeriesApi<SeriesType>} */
        let s;

        switch (blueprint.type) {
          case "Baseline": {
            s = createBaseLineSeries(blueprint);
            break;
          }
          case "Candlestick": {
            s = createCandlestickSeries(blueprint);
            break;
          }
          default:
          case "Line": {
            s = createLineSeries(blueprint);
            break;
          }
        }

        /** @satisfies {SingleSeries} */
        const series = {
          ...createBaseSeries({
            id,
            title: blueprint.title,
            color: blueprint.color,
            defaultActive: blueprint.defaultActive,
          }),
          iseries: s,
        };

        signals.createEffect(series.visible, (visible) => {
          series.iseries.applyOptions({
            visible,
          });
        });

        createLegend({ series });

        return series;
      };
      chartPane.createSplitSeries = function ({
        id,
        index: seriesIndex,
        disabled,
        setMinMaxMarkersWhenIdle,
        dataset,
        blueprint,
      }) {
        /** @satisfies {SplitSeries} */
        const series = {
          ...createBaseSeries({
            id,
            title: blueprint.title,
            color: blueprint.color,
            defaultActive: blueprint.defaultActive,
            disabled,
          }),
          dataset,
          chunks: new Array(dataset.fetchedJSONs.length),
        };

        paneSplitSeries.push(series);
        chartSplitSeries.unshift(series);

        dataset.fetchedJSONs.forEach((json, index) => {
          const chunk = signals.createSignal(
            /** @type {ISeriesApi<SeriesType> | undefined} */ (undefined),
          );

          series.chunks[index] = chunk;

          const isMyTurn = signals.createMemo(() => {
            if (seriesIndex <= 0) return true;

            const previousSeriesChunk = paneSplitSeries.at(seriesIndex - 1)
              ?.chunks[index];
            const isPreviousSeriesOnChart = previousSeriesChunk?.();

            return !!isPreviousSeriesOnChart;
          });

          signals.createEffect(
            () => ({ values: json.vec(), isMyTurn: isMyTurn() }),
            ({ values, isMyTurn }) => {
              if (!values || !isMyTurn) return;

              let s = chunk();

              if (!s) {
                switch (blueprint.type) {
                  case "Baseline": {
                    s = createBaseLineSeries({
                      color: blueprint.color,
                      options: blueprint.options,
                    });
                    break;
                  }
                  case "Candlestick": {
                    s = createCandlestickSeries({
                      options: blueprint.options,
                    });
                    break;
                  }
                  default:
                  case "Line": {
                    s = createLineSeries({
                      color: blueprint.color,
                      options: blueprint.options,
                    });
                    break;
                  }
                }

                chunk.set(s);
              }

              s.setData(values);

              setMinMaxMarkersWhenIdle();
            },
          );

          signals.createEffect(
            () => ({
              chunk: chunk(),
              currentVec: dataset.fetchedJSONs.at(index)?.vec(),
              nextVec: dataset.fetchedJSONs.at(index + 1)?.vec(),
            }),
            ({ chunk, currentVec, nextVec }) => {
              if (chunk && currentVec?.length && nextVec?.length) {
                chunk.update(nextVec[0]);
              }
            },
          );

          signals.createEffect(chunk, (chunk) => {
            const isChunkLastVisible = signals.createMemo(() => {
              const last = lastVisibleDatasetIndex();
              return last !== undefined && last === index;
            });

            signals.createEffect(
              () => ({
                visible: series.visible(),
                isChunkLastVisible: isChunkLastVisible(),
              }),
              ({ visible, isChunkLastVisible }) => {
                chunk?.applyOptions({
                  lastValueVisible: visible && isChunkLastVisible,
                });
              },
            );
          });

          const shouldChunkBeVisible = signals.createMemo(() => {
            if (visibleDatasetIds().length) {
              const start = utils.chunkIdToIndex(
                scale,
                /** @type {number} */ (visibleDatasetIds().at(0)),
              );
              const end = utils.chunkIdToIndex(
                scale,
                /** @type {number} */ (visibleDatasetIds().at(-1)),
              );

              if (index >= start && index <= end) {
                return true;
              }
            }

            return false;
          });

          let wasChunkVisible = false;
          const chunkVisible = signals.createMemo(() => {
            if (series.disabled()) {
              wasChunkVisible = false;
            } else {
              wasChunkVisible = wasChunkVisible || shouldChunkBeVisible();
            }
            return wasChunkVisible;
          });

          signals.createEffect(chunk, (chunk) => {
            if (!chunk) return;

            const visible = signals.createMemo(
              () => series.visible() && chunkVisible(),
            );

            signals.createEffect(visible, (visible) => {
              chunk.applyOptions({
                visible,
              });
            });
          });
        });

        createLegend({ series, extraName: blueprint.type });

        return series;
      };
      chartPane.hidden = () => {
        return chartWrapper.hidden;
      };
      chartPane.setHidden = (b) => {
        chartWrapper.hidden = b;
      };
      chartPane.splitSeries = paneSplitSeries;
      chartPane.setInitialVisibleTimeRange = () => {
        const range = visibleTimeRange();

        if (range) {
          chartPane.timeScale().setVisibleRange(/** @type {any} */ (range));

          // On small screen it doesn't it might not set it  in time
          setTimeout(() => {
            try {
              chartPane.timeScale().setVisibleRange(/** @type {any} */ (range));
            } catch {}
          }, 50);
        }
      };

      if (whitespace) {
        chartPane.whitespace = setWhitespace({ chart: _chart, scale, utils });
      }

      function createUnitAndModeElements() {
        const fieldset = window.document.createElement("fieldset");
        fieldset.dataset.size = "sm";
        chartWrapper.append(fieldset);

        const id = `chart-${chartId}-${paneIndex}-mode`;

        const chartModes = /** @type {const} */ (["Lin", "Log"]);
        const chartMode = signals.createSignal(
          /** @type {Lowercase<typeof chartModes[number]>} */ (
            localStorage.getItem(id) || "lin"
          ),
        );

        const field = utils.dom.createHorizontalChoiceField({
          choices: chartModes,
          selected: chartMode(),
          id,
          title: unit,
          signals,
        });
        fieldset.append(field);

        field.addEventListener("change", (event) => {
          // @ts-ignore
          const value = event.target.value;
          localStorage.setItem(id, value);
          chartMode.set(value);
        });

        signals.createEffect(chartMode, (chartMode) =>
          _chart.priceScale("right").applyOptions({
            mode: chartMode === "lin" ? 0 : 1,
          }),
        );
      }
      createUnitAndModeElements();

      switch (kind) {
        case "static": {
          config?.forEach((params) => {
            chartPane.createSingleSeries({
              id: utils.stringToId(params.title),
              blueprint: params,
            });
          });

          chartPane.timeScale().fitContent();

          break;
        }
        case "moveable": {
          chartPane.setInitialVisibleTimeRange();
          updateVisibleDatasetIds();

          if (!paneIndex) {
            setTimeout(() => {
              chartPane.timeScale().subscribeVisibleTimeRangeChange((range) => {
                if (!range) return;
                visibleTimeRange.set(range);
                debouncedUpdateVisibleDatasetIds();
                debouncedSaveVisibleRange();
              });
            });
          }

          break;
        }
      }

      panes.push(chartPane);

      return chartPane;
    }

    config?.forEach((params) => {
      createPane(params);
    });

    /**
     *
     * @param {Object} param0
     * @param {TimeScale} param0.scale
     * @param {Owner | null} param0.owner
     */
    function reset({ scale: _scale, owner: _owner }) {
      scale = _scale;
      owner = _owner;
      panes.forEach((pane) => pane.remove());
      panes.length = 0;
      legendElement.innerHTML = "";
      panesElement.innerHTML = "";
    }

    /**
     * @param {Object} args
     * @param {LogicalRange} [args.visibleLogicalRange]
     * @param {TimeRange} [args.visibleTimeRange]
     */
    function getTicksToWidthRatio({ visibleLogicalRange, visibleTimeRange }) {
      try {
        const chartPane = panes.find((pane) => !pane.hidden());
        if (!chartPane) return;
        const width = chartPane.chartElement().clientWidth;

        /** @type {number} */
        let ratio;

        if (visibleLogicalRange) {
          ratio = (visibleLogicalRange.to - visibleLogicalRange.from) / width;
        } else if (visibleTimeRange) {
          if (scale === "date") {
            const to = /** @type {Time} */ (visibleTimeRange.to);
            const from = /** @type {Time} */ (visibleTimeRange.from);

            ratio =
              utils.getNumberOfDaysBetweenTwoDates(
                utils.date.fromTime(from),
                utils.date.fromTime(to),
              ) / width;
          } else {
            const to = /** @type {number} */ (visibleTimeRange.to);
            const from = /** @type {number} */ (visibleTimeRange.from);

            ratio = (to - from) / width;
          }
        } else {
          throw Error();
        }

        return ratio;
      } catch {}
    }

    return {
      legendElement,
      panesElement,
      createPane,
      hoveredLegend,
      createLegend,
      panes,
      reset,
      visibleTimeRange,
      visibleDatasetIds,
      lastVisibleDatasetIndex,
      getInitialVisibleTimeRange,
      updateVisibleDatasetIds,
      debouncedUpdateVisibleDatasetIds,
      saveVisibleRange,
      getTicksToWidthRatio,
      debouncedSaveVisibleRange,
      splitSeries: chartSplitSeries,
    };
  }

  return {
    createChart,
  };
});
