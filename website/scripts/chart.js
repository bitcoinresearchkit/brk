/**
 * @import { HoveredLegend, PriceSeriesType, Series } from "./types/self"
 * @import { Options } from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Consts} args.consts
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.selected
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Options} args.options
 * @param {Datasets} args.datasets
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {Ids} args.ids
 * @param {Accessor<boolean>} args.dark
 */
export function init({
  colors,
  consts,
  dark,
  datasets,
  elements,
  ids,
  lightweightCharts,
  options,
  selected,
  signals,
  utils,
  webSockets,
}) {
  console.log("init chart state");

  /** @type {Array<(IChartApi & {whitespace: ISeriesApi<"Line">})>} */
  let charts = [];

  const scale = signals.createMemo(() => selected().scale);

  /**
   * @returns {TimeRange}
   */
  function getInitialVisibleTimeRange() {
    const urlParams = new URLSearchParams(window.location.search);

    const urlFrom = urlParams.get(ids.from);
    const urlTo = urlParams.get(ids.to);

    if (urlFrom && urlTo) {
      if (scale() === "date" && urlFrom.includes("-") && urlTo.includes("-")) {
        console.log({
          from: new Date(urlFrom).toJSON().split("T")[0],
          to: new Date(urlTo).toJSON().split("T")[0],
        });
        return {
          from: new Date(urlFrom).toJSON().split("T")[0],
          to: new Date(urlTo).toJSON().split("T")[0],
        };
      } else if (
        scale() === "height" &&
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
          localStorage.getItem(ids.visibleTimeRange(scale())) || "null",
        )
      );
    }

    const savedTimeRange = getSavedTimeRange();

    console.log(savedTimeRange);

    if (savedTimeRange) {
      return savedTimeRange;
    }

    function getDefaultTimeRange() {
      switch (scale()) {
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

  /**
   * @param {IChartApi} chart
   */
  function setInitialVisibleTimeRange(chart) {
    const range = visibleTimeRange();

    if (range) {
      chart.timeScale().setVisibleRange(/** @type {any} */ (range));

      // On small screen it doesn't it might not set it  in time
      setTimeout(() => {
        try {
          chart.timeScale().setVisibleRange(/** @type {any} */ (range));
        } catch {}
      }, 50);
    }
  }

  const activeDatasets = signals.createSignal(
    /** @type {Set<ResourceDataset<any, any>>} */ (new Set()),
    {
      equals: false,
    },
  );

  const visibleTimeRange = signals.createSignal(getInitialVisibleTimeRange());

  const visibleDatasetIds = signals.createSignal(/** @type {number[]} */ ([]), {
    equals: false,
  });

  const lastVisibleDatasetIndex = signals.createMemo(() => {
    const last = visibleDatasetIds().at(-1);
    return last !== undefined ? utils.chunkIdToIndex(scale(), last) : undefined;
  });

  const priceSeriesType = signals.createSignal(
    /** @type {PriceSeriesType} */ ("Candlestick"),
  );

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
  updateVisibleDatasetIds();
  const debouncedUpdateVisibleDatasetIds = utils.debounce(
    updateVisibleDatasetIds,
    100,
  );

  function saveVisibleRange() {
    const range = visibleTimeRange();
    utils.url.writeParam(ids.from, String(range.from));
    utils.url.writeParam(ids.to, String(range.to));
    localStorage.setItem(ids.visibleTimeRange(scale()), JSON.stringify(range));
  }
  const debouncedSaveVisibleRange = utils.debounce(saveVisibleRange, 250);

  function createFetchChunksOfVisibleDatasetsEffect() {
    signals.createEffect(
      () => ({ ids: visibleDatasetIds(), activeDatasets: activeDatasets() }),
      ({ ids, activeDatasets }) => {
        const datasets = Array.from(activeDatasets);

        if (ids.length === 0 || datasets.length === 0) return;

        for (let i = 0; i < ids.length; i++) {
          const id = ids[i];
          for (let j = 0; j < datasets.length; j++) {
            datasets[j].fetch(id);
          }
        }
      },
    );
  }
  createFetchChunksOfVisibleDatasetsEffect();

  /**
   * @param {HTMLElement} parent
   * @param {number} chartIndex
   */
  function createChartDiv(parent, chartIndex) {
    const chartWrapper = window.document.createElement("div");
    chartWrapper.classList.add("chart-wrapper");
    parent.append(chartWrapper);

    const chartDiv = window.document.createElement("div");
    chartDiv.classList.add("chart-div");
    chartWrapper.append(chartDiv);

    function createUnitAndModeElements() {
      const fieldset = window.document.createElement("fieldset");
      fieldset.dataset.size = "sm";
      chartWrapper.append(fieldset);

      const unitName = signals.createSignal("");

      const id = `chart-${chartIndex}-mode`;

      const chartModes = /** @type {const} */ (["Linear", "Log"]);
      const chartMode = signals.createSignal(
        /** @type {Lowercase<typeof chartModes[number]>} */ (
          localStorage.getItem(id) ||
            chartModes[chartIndex ? 0 : 1].toLowerCase()
        ),
      );

      const field = utils.dom.createHorizontalChoiceField({
        choices: chartModes,
        selected: chartMode(),
        id,
        title: unitName,
        signals,
      });
      fieldset.append(field);

      field.addEventListener("change", (event) => {
        // @ts-ignore
        const value = event.target.value;
        localStorage.setItem(id, value);
        chartMode.set(value);
      });

      return { unitName, chartMode };
    }
    const { unitName, chartMode } = createUnitAndModeElements();

    return { chartDiv, unitName, chartMode };
  }

  /**
   * @param {IChartApi} chart
   */
  function subscribeVisibleTimeRangeChange(chart) {
    chart.timeScale().subscribeVisibleTimeRangeChange((range) => {
      if (!range) return;

      visibleTimeRange.set(range);

      debouncedUpdateVisibleDatasetIds();

      debouncedSaveVisibleRange();
    });
  }

  /**
   * @param {Object} args
   * @param {IChartApi} args.chart
   * @param {LogicalRange} [args.visibleLogicalRange]
   * @param {TimeRange} [args.visibleTimeRange]
   */
  function updateVisiblePriceSeriesType({
    chart,
    visibleLogicalRange,
    visibleTimeRange,
  }) {
    try {
      const width = chart.timeScale().width();

      /** @type {number} */
      let ratio;

      if (visibleLogicalRange) {
        ratio = (visibleLogicalRange.to - visibleLogicalRange.from) / width;
      } else if (visibleTimeRange) {
        if (scale() === "date") {
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

      if (ratio <= 0.5) {
        priceSeriesType.set("Candlestick");
      } else {
        priceSeriesType.set("Line");
      }
    } catch {}
  }
  const debouncedUpdateVisiblePriceSeriesType = utils.debounce(
    updateVisiblePriceSeriesType,
    50,
  );

  const hoveredLegend = signals.createSignal(
    /** @type {HoveredLegend | undefined} */ (undefined),
  );
  const notHoveredLegendTransparency = "66";
  /**
   * @param {Object} args
   * @param {Series} args.series
   * @param {Accessor<boolean>} [args.disabled]
   * @param {string} [args.name]
   */
  function createLegend({ series, disabled, name }) {
    const div = window.document.createElement("div");

    if (disabled) {
      signals.createEffect(disabled, (disabled) => {
        div.hidden = disabled;
      });
    }

    elements.legend.prepend(div);

    const { input, label } = utils.dom.createLabeledInput({
      inputId: `legend-${series.title}`,
      inputName: `selected-${series.title}${name}`,
      inputValue: "value",
      labelTitle: "Click to toggle",
      onClick: (event) => {
        event.preventDefault();
        event.stopPropagation();
        input.checked = !input.checked;
        series.active.set(input.checked);
      },
    });

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

    signals.createEffect(series.active, (checked) => {
      input.checked = checked;
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
    const colors = Array.isArray(series.color) ? series.color : [series.color];
    colors.forEach((color) => {
      const spanColor = window.document.createElement("span");
      spanColors.append(spanColor);

      signals.createEffect(
        () => ({ color: color(), shouldHighlight: shouldHighlight() }),
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

      signals.createEffect(
        () => ({ hovered: hoveredLegend(), ids: visibleDatasetIds() }),
        ({ hovered, ids }) => {
          if (!hovered && !previouslyHovered) return hovered;

          for (let i = 0; i < ids.length; i++) {
            const chunkId = ids[i];
            const chunkIndex = utils.chunkIdToIndex(scale(), chunkId);
            const chunk = series.chunks[chunkIndex];

            signals.createEffect(chunk, (chunk) => {
              if (!chunk) return;

              if (hovered) {
                const seriesOptions = chunk.options();
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
                    darkenedColors[i][k] =
                      `${v}${notHoveredLegendTransparency}`;
                  } else if (k === "lastValueVisible" && v) {
                    initialColors[i][k] = true;
                    darkenedColors[i][k] = false;
                  }
                });
              }

              signals.createEffect(shouldHighlight, (shouldHighlight) => {
                if (shouldHighlight) {
                  chunk.applyOptions(initialColors[i]);
                } else {
                  chunk.applyOptions(darkenedColors[i]);
                }
              });
            });
          }

          previouslyHovered = hovered;
        },
      );
    }
    createHoverEffect();

    const anchor = window.document.createElement("a");
    anchor.href = series.dataset.url;
    anchor.target = "_blank";
    anchor.rel = "noopener noreferrer";
    div.append(anchor);
  }

  /**
   * @template {TimeScale} S
   * @param {Object} args
   * @param {ResourceDataset<S>} args.dataset
   * @param {SeriesBlueprint} args.seriesBlueprint
   * @param {Option} args.option
   * @param {IChartApi} args.chart
   * @param {number} args.index
   * @param {Series[]} args.chartSeries
   * @param {Accessor<number | undefined>} args.lastVisibleDatasetIndex
   * @param {VoidFunction} args.setMinMaxMarkersWhenIdle
   * @param {Accessor<boolean>} [args.disabled]
   */
  function createSeries({
    chart,
    option,
    index: seriesIndex,
    disabled: _disabled,
    lastVisibleDatasetIndex,
    setMinMaxMarkersWhenIdle,
    dataset,
    seriesBlueprint,
    chartSeries,
  }) {
    const {
      title,
      color,
      defaultActive,
      type,
      options: seriesOptions,
    } = seriesBlueprint;

    /** @type {Signal<ISeriesApi<SeriesType> | undefined>[]} */
    const chunks = new Array(dataset.fetchedJSONs.length);

    const id = ids.fromString(title);
    const storageId = options.optionAndSeriesToKey(option, seriesBlueprint);

    const active = signals.createSignal(
      utils.url.readBoolParam(id) ??
        utils.storage.readBool(storageId) ??
        defaultActive ??
        true,
    );

    const disabled = signals.createMemo(_disabled || (() => false));

    const visible = signals.createMemo(() => active() && !disabled());

    signals.createEffect(
      () => ({ disabled: disabled(), active: active() }),
      ({ disabled, active }) => {
        if (disabled) {
          return;
        }

        if (active !== (defaultActive || true)) {
          utils.url.writeParam(id, active);
          utils.storage.write(storageId, active);
        } else {
          utils.url.removeParam(id);
          utils.storage.remove(storageId);
        }
      },
    );

    /** @type {Series} */
    const series = {
      active,
      chunks,
      color: color || [colors.profit, colors.loss],
      dataset,
      disabled,
      id,
      title,
      visible,
    };

    chartSeries.push(series);

    const owner = signals.getOwner();

    dataset.fetchedJSONs.forEach((json, index) => {
      const chunk = signals.createSignal(
        /** @type {ISeriesApi<SeriesType> | undefined} */ (undefined),
      );

      chunks[index] = chunk;

      const isMyTurn = signals.createMemo(() => {
        if (seriesIndex <= 0) return true;

        const previousSeriesChunk = chartSeries.at(seriesIndex - 1)?.chunks[
          index
        ];
        const isPreviousSeriesOnChart = previousSeriesChunk?.();

        return !!isPreviousSeriesOnChart;
      });

      signals.createEffect(
        () => ({ values: json.vec(), isMyTurn: isMyTurn() }),
        ({ values, isMyTurn }) => {
          if (!values || !isMyTurn) return;

          let s = chunk();

          if (!s) {
            switch (type) {
              case "Baseline": {
                s = lightweightCharts.createBaseLineSeries({
                  chart,
                  color,
                  options: seriesOptions,
                  owner,
                  signals,
                  colors,
                });
                break;
              }
              case "Candlestick": {
                s = lightweightCharts.createCandlesticksSeries({
                  chart,
                  options: seriesOptions,
                  owner,
                  signals,
                  colors,
                });
                break;
              }
              // case "Histogram": {
              //   s = createHistogramSeries({
              //     chart,
              //     options,
              //   });
              //   break;
              // }
              default:
              case "Line": {
                s = lightweightCharts.createLineSeries({
                  chart,
                  color,
                  options: seriesOptions,
                  owner,
                  signals,
                  colors,
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
            scale(),
            /** @type {number} */ (visibleDatasetIds().at(0)),
          );
          const end = utils.chunkIdToIndex(
            scale(),
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

    createLegend({ series, disabled, name: type });

    return series;
  }

  /**
   * @param {Object} args
   * @param {PriceSeriesType} args.type
   * @param {VoidFunction} args.setMinMaxMarkersWhenIdle
   * @param {Option} args.option
   * @param {IChartApi} args.chart
   * @param {Series[]} args.chartSeries
   * @param {Accessor<number | undefined>} args.lastVisibleDatasetIndex
   */
  function createPriceSeries({
    type,
    setMinMaxMarkersWhenIdle,
    option,
    chart,
    chartSeries,
    lastVisibleDatasetIndex,
  }) {
    const s = scale();

    /** @type {AnyDatasetPath} */
    const datasetPath = `${s}-to-price`;

    const dataset = datasets.getOrCreate(s, datasetPath);

    // Don't trigger reactivity by design
    activeDatasets().add(dataset);

    const title = "BTC Price";

    /** @type {SeriesBlueprint} */
    let seriesBlueprint;

    if (type === "Candlestick") {
      seriesBlueprint = {
        datasetPath,
        title,
        type: "Candlestick",
      };
    } else {
      seriesBlueprint = {
        datasetPath,
        title,
        color: colors.default,
      };
    }

    const disabled = signals.createMemo(() => priceSeriesType() !== type);

    const priceSeries = createSeries({
      seriesBlueprint,
      dataset,
      option,
      index: -1,
      chart,
      chartSeries,
      lastVisibleDatasetIndex,
      disabled,
      setMinMaxMarkersWhenIdle,
    });

    function createLiveCandleUpdateEffect() {
      signals.createEffect(webSockets.kraken1dCandle.latest, (latest) => {
        if (!latest) return;

        const index = utils.chunkIdToIndex(s, latest.year);

        const series = priceSeries.chunks.at(index);

        if (series) {
          signals.createEffect(series, (series) => {
            series?.update(latest);
          });
        }
      });
    }
    createLiveCandleUpdateEffect();

    return priceSeries;
  }

  /**
   * @param {ChartOption} option
   */
  function applyChartOption(option) {
    const scale = option.scale;
    visibleTimeRange.set(getInitialVisibleTimeRange());

    activeDatasets.set((s) => {
      s.clear();
      return s;
    });

    const chartCount = 1 + (option.bottom?.length ? 1 : 0);
    const blueprintCount =
      1 + (option.top?.length || 0) + (option.bottom?.length || 0);
    const chartsBlueprints = [option.top || [], option.bottom].flatMap(
      (list) => (list ? [list] : []),
    );

    /** @type {Series[]} */
    const allSeries = [];

    charts = chartsBlueprints.map((seriesBlueprints, chartIndex) => {
      const { chartDiv, unitName, chartMode } = createChartDiv(
        elements.chartsChartList,
        chartIndex,
      );

      const chart = lightweightCharts.createChartWithWhitespace({
        scale,
        element: chartDiv,
        signals,
        colors,
      });

      setInitialVisibleTimeRange(chart);

      /** @type {Series[]} */
      const chartSeries = [];

      function setMinMaxMarkers() {
        try {
          const { from, to } = visibleTimeRange();

          const dateFrom = new Date(String(from));
          const dateTo = new Date(String(to));

          /** @type {Marker | undefined} */
          let max = undefined;
          /** @type {Marker | undefined} */
          let min = undefined;

          const ids = visibleDatasetIds();

          for (let i = 0; i < chartSeries.length; i++) {
            const { chunks, dataset } = chartSeries[i];

            for (let j = 0; j < ids.length; j++) {
              const id = ids[j];

              const chunkIndex = utils.chunkIdToIndex(scale, id);

              const chunk = chunks.at(chunkIndex)?.();

              if (!chunk || !chunk?.options().visible) continue;

              chunk.setMarkers([]);

              const isCandlestick = chunk.seriesType() === "Candlestick";

              const vec = dataset.fetchedJSONs.at(chunkIndex)?.vec();

              if (!vec) return;

              for (let k = 0; k < vec.length; k++) {
                const data = vec[k];

                let number;

                if (scale === "date") {
                  const date = utils.date.fromTime(data.time);

                  number = date.getTime();

                  if (date <= dateFrom || date >= dateTo) {
                    continue;
                  }
                } else {
                  const height = data.time;

                  number = /** @type {number} */ (height);

                  if (height <= from || height >= to) {
                    continue;
                  }
                }

                // @ts-ignore
                const high = isCandlestick ? data["high"] : data.value;
                // @ts-ignore
                const low = isCandlestick ? data["low"] : data.value;

                if (!max || high > max.value) {
                  max = {
                    weight: number,
                    time: data.time,
                    value: high,
                    seriesChunk: chunk,
                  };
                }
                if (!min || low < min.value) {
                  min = {
                    weight: number,
                    time: data.time,
                    value: low,
                    seriesChunk: chunk,
                  };
                }
              }
            }
          }

          /** @type {(SeriesMarker<Time> & Weighted) | undefined} */
          let minMarker;
          /** @type {(SeriesMarker<Time> & Weighted) | undefined} */
          let maxMarker;

          if (min) {
            minMarker = {
              weight: min.weight,
              time: min.time,
              color: colors.default(),
              position: "belowBar",
              shape: "arrowUp",
              size: 0,
              text: utils.locale.numberToShortUSFormat(min.value),
            };
          }

          if (max) {
            maxMarker = {
              weight: max.weight,
              time: max.time,
              color: colors.default(),
              position: "aboveBar",
              shape: "arrowDown",
              size: 0,
              text: utils.locale.numberToShortUSFormat(max.value),
            };
          }

          if (
            min &&
            max &&
            min.seriesChunk === max.seriesChunk &&
            minMarker &&
            maxMarker
          ) {
            min.seriesChunk.setMarkers(
              [minMarker, maxMarker].sort((a, b) => a.weight - b.weight),
            );
          } else {
            if (min && minMarker) {
              min.seriesChunk.setMarkers([minMarker]);
            }

            if (max && maxMarker) {
              max.seriesChunk.setMarkers([maxMarker]);
            }
          }
        } catch (e) {}
      }

      const setMinMaxMarkersWhenIdle = () =>
        utils.runWhenIdle(
          () => {
            setMinMaxMarkers();
          },
          blueprintCount * 10 + scale === "date" ? 50 : 100,
        );

      function createSetMinMaxMarkersWhenIdleEffect() {
        signals.createEffect(
          () => [visibleTimeRange(), dark()],
          setMinMaxMarkersWhenIdle,
        );
      }
      createSetMinMaxMarkersWhenIdleEffect();

      if (!chartIndex) {
        subscribeVisibleTimeRangeChange(chart);

        updateVisiblePriceSeriesType({
          chart,
          visibleTimeRange: visibleTimeRange(),
        });

        /** @param {PriceSeriesType} type */
        function _createPriceSeries(type) {
          return createPriceSeries({
            chart,
            chartSeries,
            lastVisibleDatasetIndex,
            option,
            setMinMaxMarkersWhenIdle,
            type,
          });
        }

        const priceCandlestickSeries = _createPriceSeries("Candlestick");
        const priceLineSeries = _createPriceSeries("Line");

        function createLinkPriceSeriesEffect() {
          signals.createEffect(priceLineSeries.active, (active) => {
            priceCandlestickSeries.active.set(active);
          });

          signals.createEffect(priceCandlestickSeries.active, (active) => {
            priceLineSeries.active.set(active);
          });
        }
        createLinkPriceSeriesEffect();

        /** @type {Unit} */
        const unit = "US Dollars";
        unitName.set(unit);
      } else {
        unitName.set(option.unit);
      }

      [...seriesBlueprints].reverse().forEach((seriesBlueprint, index) => {
        const dataset = datasets.getOrCreate(
          scale,
          seriesBlueprint.datasetPath,
        );

        // Don't trigger reactivity by design
        activeDatasets().add(dataset);

        createSeries({
          index,
          seriesBlueprint,
          chart,
          option,
          lastVisibleDatasetIndex,
          setMinMaxMarkersWhenIdle,
          chartSeries,
          dataset,
        });
      });

      setMinMaxMarkers();

      activeDatasets.set((s) => s);

      chartSeries.forEach((series) => {
        allSeries.unshift(series);

        signals.createEffect(series.active, () => {
          setMinMaxMarkersWhenIdle();
        });
      });

      const chartVisible = signals.createMemo(() =>
        chartSeries.some((series) => series.visible()),
      );

      function createChartVisibilityEffect() {
        signals.createEffect(chartVisible, (chartVisible) => {
          const chartWrapper = chartDiv.parentElement;
          if (!chartWrapper) throw "Should exist";
          chartWrapper.hidden = !chartVisible;
        });
      }
      createChartVisibilityEffect();

      function createTimeScaleVisibilityEffect() {
        signals.createEffect(chartVisible, (chartVisible) => {
          const visible = chartIndex === chartCount - 1 && chartVisible;

          chart.timeScale().applyOptions({
            visible,
          });

          if (chartIndex === 1) {
            charts[0].timeScale().applyOptions({
              visible: !visible,
            });
          }
        });
      }
      createTimeScaleVisibilityEffect();

      signals.createEffect(chartMode, (chartMode) =>
        chart.priceScale("right").applyOptions({
          mode: chartMode === "linear" ? 0 : 1,
        }),
      );

      chart.timeScale().subscribeVisibleLogicalRangeChange((logicalRange) => {
        if (!logicalRange) return;

        // Must be the chart with the visible timeScale
        if (chartIndex === chartCount - 1) {
          debouncedUpdateVisiblePriceSeriesType({
            chart,
            visibleLogicalRange: logicalRange,
          });
        }

        for (
          let otherChartIndex = 0;
          otherChartIndex <= chartCount - 1;
          otherChartIndex++
        ) {
          if (chartIndex !== otherChartIndex) {
            charts[otherChartIndex]
              .timeScale()
              .setVisibleLogicalRange(logicalRange);
          }
        }
      });

      chart.subscribeCrosshairMove(({ time, sourceEvent }) => {
        // Don't override crosshair position from scroll event
        if (time && !sourceEvent) return;

        for (
          let otherChartIndex = 0;
          otherChartIndex <= chartCount - 1;
          otherChartIndex++
        ) {
          const otherChart = charts[otherChartIndex];

          if (otherChart && chartIndex !== otherChartIndex) {
            if (time) {
              otherChart.setCrosshairPosition(NaN, time, otherChart.whitespace);
            } else {
              // No time when mouse goes outside the chart
              otherChart.clearCrosshairPosition();
            }
          }
        }
      });

      return chart;
    });
  }

  function resetLegendElement() {
    elements.legend.innerHTML = "";
  }

  function resetChartListElement() {
    while (
      elements.chartsChartList.lastElementChild?.classList.contains(
        "chart-wrapper",
      )
    ) {
      elements.chartsChartList.lastElementChild?.remove();
    }
  }

  function reset() {
    charts.forEach((chart) => chart.remove());
    charts = [];
    resetLegendElement();
    resetChartListElement();
  }

  function createApplyChartOptionEffect() {
    signals.createEffect(selected, (option) => {
      reset();
      applyChartOption(option);
    });
  }
  createApplyChartOptionEffect();
}
