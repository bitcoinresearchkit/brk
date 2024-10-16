/**
 * @import {Options} from './options';
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
export function initChartsElement({
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
    signals.createEffect(() => {
      const ids = visibleDatasetIds();
      const datasets = Array.from(activeDatasets());

      if (ids.length === 0 || datasets.length === 0) return;

      signals.untrack(() => {
        console.log(ids, datasets);
        for (let i = 0; i < ids.length; i++) {
          const id = ids[i];
          for (let j = 0; j < datasets.length; j++) {
            datasets[j].fetch(id);
          }
        }
      });
    });
  }
  createFetchChunksOfVisibleDatasetsEffect();

  function resetChartListElement() {
    while (
      elements.chartList.lastElementChild?.classList.contains("chart-wrapper")
    ) {
      elements.chartList.lastElementChild?.remove();
    }
  }

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

      const field = utils.dom.createField({
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
              utils.dateFromTime(from),
              utils.dateFromTime(to),
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
    /** @type {{label: HTMLLabelElement, series: Series} | undefined} */ (
      undefined
    ),
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
    signals.createEffect(() => {
      div.hidden = disabled?.() ? true : false;
    });
    elements.legend.prepend(div);

    const { input, label, spanMain } = utils.dom.createComplexLabeledInput({
      inputId: `legend-${series.title}`,
      inputName: `selected-${series.title}${name}`,
      inputValue: "value",
      labelTitle: "Click to toggle",
      name: series.title,
      onClick: (event) => {
        event.preventDefault();
        event.stopPropagation();
        input.checked = !input.checked;
        series.active.set(input.checked);
      },
    });
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

    signals.createEffect(() => {
      input.checked = series.active();
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

      signals.createEffect(() => {
        const c = color();
        if (shouldHighlight()) {
          spanColor.style.backgroundColor = c;
        } else {
          spanColor.style.backgroundColor = `${c}${notHoveredLegendTransparency}`;
        }
      });
    });

    function createHoverEffect() {
      const initialColors = /** @type {Record<string, any>} */ ({});
      const darkenedColors = /** @type {Record<string, any>} */ ({});

      signals.createEffect(
        // @ts-ignore
        (previouslyHovered) => {
          const hovered = hoveredLegend();

          if (!hovered && !previouslyHovered) return hovered;

          const ids = visibleDatasetIds();

          for (let i = 0; i < ids.length; i++) {
            const chunkId = ids[i];
            const chunkIndex = utils.chunkIdToIndex(scale(), chunkId);
            const chunk = series.chunks[chunkIndex]?.();

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
                  darkenedColors[i][k] = `${v}${notHoveredLegendTransparency}`;
                } else if (k === "lastValueVisible" && v) {
                  initialColors[i][k] = true;
                  darkenedColors[i][k] = false;
                }
              });
            }

            if (shouldHighlight()) {
              chunk.applyOptions(initialColors[i]);
            } else {
              chunk.applyOptions(darkenedColors[i]);
            }
          }

          return hovered;
        },
        undefined,
      );
    }
    createHoverEffect();

    const anchor = window.document.createElement("a");
    anchor.href = series.dataset.url;
    anchor.innerHTML = `<svg viewBox="0 0 16 16"><path d="M8.75 2.75a.75.75 0 0 0-1.5 0v5.69L5.03 6.22a.75.75 0 0 0-1.06 1.06l3.5 3.5a.75.75 0 0 0 1.06 0l3.5-3.5a.75.75 0 0 0-1.06-1.06L8.75 8.44V2.75Z" /><path d="M3.5 9.75a.75.75 0 0 0-1.5 0v1.5A2.75 2.75 0 0 0 4.75 14h6.5A2.75 2.75 0 0 0 14 11.25v-1.5a.75.75 0 0 0-1.5 0v1.5c0 .69-.56 1.25-1.25 1.25h-6.5c-.69 0-1.25-.56-1.25-1.25v-1.5Z" /></svg>`;
    anchor.target = "_target";
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

    signals.createEffect(() => {
      if (disabled()) {
        return;
      }

      const a = active();

      if (a !== (defaultActive || true)) {
        utils.url.writeParam(id, a);
        utils.storage.write(storageId, a);
      } else {
        utils.url.removeParam(id);
        utils.storage.remove(storageId);
      }
    });

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

      signals.createEffect(() => {
        const values = json.vec();

        if (!values) return;

        if (seriesIndex > 0) {
          const previousSeriesChunk = chartSeries.at(seriesIndex - 1)?.chunks[
            index
          ];
          const isPreviousSeriesOnChart = previousSeriesChunk?.();
          if (!isPreviousSeriesOnChart) {
            return;
          }
        }

        signals.untrack(() => {
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
        });
      });

      signals.createEffect(() => {
        const _chunk = chunk();
        const currentVec = dataset.fetchedJSONs.at(index)?.vec();
        const nextVec = dataset.fetchedJSONs.at(index + 1)?.vec();

        if (_chunk && currentVec?.length && nextVec?.length) {
          _chunk.update(nextVec[0]);
        }
      });

      const isChunkLastVisible = signals.createMemo(() => {
        const last = lastVisibleDatasetIndex();
        return last !== undefined && last === index;
      });

      signals.createEffect(() => {
        chunk()?.applyOptions({
          lastValueVisible: series.visible() && isChunkLastVisible(),
        });
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

      signals.createEffect(() => {
        const visible = series.visible() && chunkVisible();
        chunk()?.applyOptions({
          visible,
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

    const dataset = datasets.getOrImport(s, datasetPath);

    // Don't trigger reactivity by design
    activeDatasets().add(dataset);

    const title = "Price";

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
      signals.createEffect(() => {
        const latest = webSockets.krakenCandle.latest();

        if (!latest) return;

        const index = utils.chunkIdToIndex(s, latest.year);

        const series = priceSeries.chunks.at(index)?.();

        series?.update(latest);
      });
    }
    createLiveCandleUpdateEffect();

    return priceSeries;
  }

  function resetLegendElement() {
    elements.legend.innerHTML = "";
  }

  function initTimeScaleElement() {
    function initScrollButtons() {
      const buttonBackward = utils.dom.getElementById("button-backward");
      const buttonBackwardIcon = utils.dom.getElementById(
        "button-backward-icon",
      );
      const buttonBackwardPauseIcon = utils.dom.getElementById(
        "button-backward-pause-icon",
      );
      const buttonForward = utils.dom.getElementById("button-forward");
      const buttonForwardIcon = utils.dom.getElementById("button-forward-icon");
      const buttonForwardPauseIcon = utils.dom.getElementById(
        "button-forward-pause-icon",
      );

      let interval = /** @type {number | undefined} */ (undefined);
      let direction = /** @type  {1 | -1 | 0} */ (0);

      const DELAY = 5;
      const MULTIPLIER = DELAY / 10000;

      function scrollChart() {
        if (direction <= 0) {
          buttonForwardIcon.removeAttribute("hidden");
          buttonForwardPauseIcon.setAttribute("hidden", "");
        }
        if (direction >= 0) {
          buttonBackwardIcon.removeAttribute("hidden");
          buttonBackwardPauseIcon.setAttribute("hidden", "");
        }
        if (direction === -1) {
          buttonBackwardIcon.setAttribute("hidden", "");
          buttonBackwardPauseIcon.removeAttribute("hidden");
        }
        if (direction === 1) {
          buttonForwardIcon.setAttribute("hidden", "");
          buttonForwardPauseIcon.removeAttribute("hidden");
        }

        if (!direction) {
          clearInterval(interval);
          return;
        }

        interval = setInterval(() => {
          const time = charts.at(-1)?.timeScale();

          if (!time) return;

          const range = time.getVisibleLogicalRange();

          if (!range) return;

          const speed = (range.to - range.from) * MULTIPLIER * direction;

          // @ts-ignore
          range.from += speed;
          // @ts-ignore
          range.to += speed;

          time.setVisibleLogicalRange(range);
        }, DELAY);
      }

      buttonBackward.addEventListener("click", () => {
        if (direction !== -1) {
          direction = -1;
        } else {
          direction = 0;
        }
        scrollChart();
      });

      buttonForward.addEventListener("click", () => {
        if (direction !== 1) {
          direction = 1;
        } else {
          direction = 0;
        }
        scrollChart();
      });
    }
    initScrollButtons();

    const GENESIS_DAY = "2009-01-03";

    /**
     * @param {HTMLButtonElement} button
     * @param {ChartOption} option
     */
    function setTimeScale(button, option) {
      const chart = charts.at(-1);
      if (!chart) return;
      const timeScale = chart.timeScale();

      const year = button.dataset.year;
      let days = button.dataset.days;
      let toHeight = button.dataset.to;

      switch (option.scale) {
        case "date": {
          let from = new Date();
          let to = new Date();
          to.setUTCHours(0, 0, 0, 0);

          if (!days && typeof button.dataset.yearToDate === "string") {
            days = String(
              Math.ceil(
                (to.getTime() -
                  new Date(`${to.getUTCFullYear()}-01-01`).getTime()) /
                  consts.ONE_DAY_IN_MS,
              ),
            );
          }

          if (year) {
            from = new Date(`${year}-01-01`);
            to = new Date(`${year}-12-31`);
          } else if (days) {
            from.setDate(from.getUTCDate() - Number(days));
          } else {
            from = new Date(GENESIS_DAY);
          }

          timeScale.setVisibleRange({
            from: /** @type {Time} */ (from.getTime() / 1000),
            to: /** @type {Time} */ (to.getTime() / 1000),
          });
          break;
        }
        case "height": {
          timeScale.setVisibleRange({
            from: /** @type {Time} */ (0),
            to: /** @type {Time} */ (Number(toHeight?.slice(0, -1)) * 1_000),
          });
          break;
        }
      }
    }

    /**
     * @param {HTMLElement} timeScaleButtons
     */
    function initGoToButtons(timeScaleButtons) {
      Array.from(timeScaleButtons.children).forEach((button) => {
        if (button.tagName !== "BUTTON") throw "Expect a button";
        button.addEventListener("click", () => {
          const option = options.selected();
          if (option.kind === "chart") {
            setTimeScale(/** @type {HTMLButtonElement} */ (button), option);
          }
        });
      });
    }
    initGoToButtons(elements.timeScaleDateButtons);
    initGoToButtons(elements.timeScaleHeightButtons);

    function createScaleButtonsToggleEffect() {
      signals.createEffect(() => {
        const scaleIsDate = scale() === "date";
        elements.timeScaleDateButtons.hidden = !scaleIsDate;
        elements.timeScaleHeightButtons.hidden = scaleIsDate;
      });
    }
    createScaleButtonsToggleEffect();
  }
  initTimeScaleElement();

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

    resetLegendElement();
    resetChartListElement();

    /** @type {Series[]} */
    const allSeries = [];

    charts = chartsBlueprints.map((seriesBlueprints, chartIndex) => {
      const { chartDiv, unitName, chartMode } = createChartDiv(
        elements.chartList,
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
                  const date = utils.dateFromTime(data.time);

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
        signals.createEffect(() => {
          visibleTimeRange();
          dark();
          signals.untrack(setMinMaxMarkersWhenIdle);
        });
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
          signals.createEffect(() => {
            priceCandlestickSeries.active.set(priceLineSeries.active());
          });

          signals.createEffect(() => {
            priceLineSeries.active.set(priceCandlestickSeries.active());
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
        const dataset = datasets.getOrImport(
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

        signals.createEffect(() => {
          series.active();
          signals.untrack(setMinMaxMarkersWhenIdle);
        });
      });

      const chartVisible = signals.createMemo(() =>
        chartSeries.some((series) => series.visible()),
      );

      function createChartVisibilityEffect() {
        signals.createEffect(() => {
          const chartWrapper = chartDiv.parentElement;
          if (!chartWrapper) throw "Should exist";
          chartWrapper.hidden = !chartVisible();
        });
      }
      createChartVisibilityEffect();

      function createTimeScaleVisibilityEffect() {
        signals.createEffect(() => {
          const visible = chartIndex === chartCount - 1 && chartVisible();

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

      signals.createEffect(() =>
        chart.priceScale("right").applyOptions({
          mode: chartMode() === "linear" ? 0 : 1,
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

  function createApplyChartOptionEffect() {
    signals.createEffect(() => {
      const option = selected();
      signals.createUntrackedRoot(() => {
        applyChartOption(option);
      });
    });
  }
  createApplyChartOptionEffect();
}
