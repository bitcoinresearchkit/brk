// @ts-check

/**
 * @import { ChartPane, HoveredLegend, PriceSeriesType, SplitSeries } from "./types/self"
 * @import { Options } from './options';
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.selected
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Datasets} args.datasets
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {Accessor<boolean>} args.dark
 */
export function init({
  colors,
  dark,
  datasets,
  elements,
  lightweightCharts,
  selected,
  signals,
  utils,
  webSockets,
}) {
  console.log("init chart state");

  const scale = signals.createMemo(() => selected().scale);

  elements.charts.append(utils.dom.createShadow("left"));
  elements.charts.append(utils.dom.createShadow("right"));

  const { headerElement, titleElement, descriptionElement } =
    utils.dom.createHeader({});
  elements.charts.append(headerElement);
  signals.createEffect(selected, (option) => {
    titleElement.innerHTML = option.title;
    descriptionElement.innerHTML = option.serializedPath;
  });

  const chart = lightweightCharts.createChart({
    parent: elements.charts,
    signals,
    colors,
    id: "chart",
    scale: scale(),
    kind: "moveable",
    utils,
  });

  const activeDatasets = signals.createSignal(
    /** @type {Set<ResourceDataset<any, any>>} */ (new Set()),
    {
      equals: false,
    },
  );

  const priceSeriesType = signals.createSignal(
    /** @type {PriceSeriesType} */ ("Candlestick"),
  );

  function createFetchChunksOfVisibleDatasetsEffect() {
    signals.createEffect(
      () => ({
        ids: chart.visibleDatasetIds(),
        activeDatasets: activeDatasets(),
      }),
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
   * @param {Parameters<Chart['getTicksToWidthRatio']>[0]} args
   */
  function updateVisiblePriceSeriesType(args) {
    const ratio = chart.getTicksToWidthRatio(args);
    if (ratio) {
      if (ratio <= 0.5) {
        priceSeriesType.set("Candlestick");
      } else {
        priceSeriesType.set("Line");
      }
    }
  }
  const debouncedUpdateVisiblePriceSeriesType = utils.debounce(
    updateVisiblePriceSeriesType,
    50,
  );

  /**
   * @param {Object} args
   * @param {PriceSeriesType} args.type
   * @param {VoidFunction} args.setMinMaxMarkersWhenIdle
   * @param {Option} args.option
   * @param {ChartPane} args.chartPane
   * @param {SplitSeries[]} args.chartSeries
   */
  function createPriceSeries({
    type,
    setMinMaxMarkersWhenIdle,
    option,
    chartPane,
    chartSeries: splitSeries,
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

    const priceSeries = chartPane.createSplitSeries({
      seriesBlueprint,
      dataset,
      option,
      index: -1,
      splitSeries,
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
    chart.visibleTimeRange.set(chart.getInitialVisibleTimeRange());

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

    /** @type {SplitSeries[]} */
    const allSeries = [];

    chartsBlueprints.map((seriesBlueprints, paneIndex) => {
      const chartPane = chart.createPane({
        paneIndex,
        unit: paneIndex ? option.unit : "US Dollars",
        whitespace: true,
      });

      /** @type {SplitSeries[]} */
      const splitSeries = [];

      function setMinMaxMarkers() {
        try {
          const { from, to } = chart.visibleTimeRange();

          const dateFrom = new Date(String(from));
          const dateTo = new Date(String(to));

          /** @type {Marker | undefined} */
          let max = undefined;
          /** @type {Marker | undefined} */
          let min = undefined;

          const ids = chart.visibleDatasetIds();

          for (let i = 0; i < splitSeries.length; i++) {
            const { chunks, dataset } = splitSeries[i];

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
          () => [chart.visibleTimeRange(), dark()],
          setMinMaxMarkersWhenIdle,
        );
      }
      createSetMinMaxMarkersWhenIdleEffect();

      if (!paneIndex) {
        updateVisiblePriceSeriesType({
          visibleTimeRange: chart.visibleTimeRange(),
        });

        /** @param {PriceSeriesType} type */
        function _createPriceSeries(type) {
          return createPriceSeries({
            chartPane,
            chartSeries: splitSeries,
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
      }

      [...seriesBlueprints].reverse().forEach((seriesBlueprint, index) => {
        const dataset = datasets.getOrCreate(
          scale,
          seriesBlueprint.datasetPath,
        );

        // Don't trigger reactivity by design
        activeDatasets().add(dataset);

        chartPane.createSplitSeries({
          index,
          seriesBlueprint,
          option,
          setMinMaxMarkersWhenIdle,
          splitSeries,
          dataset,
        });
      });

      setMinMaxMarkers();

      activeDatasets.set((s) => s);

      splitSeries.forEach((series) => {
        allSeries.unshift(series);

        signals.createEffect(series.active, () => {
          setMinMaxMarkersWhenIdle();
        });
      });

      const chartVisible = signals.createMemo(() =>
        splitSeries.some((series) => series.visible()),
      );

      function createChartVisibilityEffect() {
        signals.createEffect(chartVisible, (chartVisible) => {
          chartPane.setHidden(!chartVisible);
        });
      }
      createChartVisibilityEffect();

      function createTimeScaleVisibilityEffect() {
        signals.createEffect(chartVisible, (chartVisible) => {
          const visible = paneIndex === chartCount - 1 && chartVisible;

          chartPane.timeScale().applyOptions({
            visible,
          });

          if (paneIndex === 1) {
            chart.panes[0].timeScale().applyOptions({
              visible: !visible,
            });
          }
        });
      }
      createTimeScaleVisibilityEffect();

      chartPane
        .timeScale()
        .subscribeVisibleLogicalRangeChange((logicalRange) => {
          if (!logicalRange) return;

          // Must be the chart with the visible timeScale
          if (paneIndex === chartCount - 1) {
            debouncedUpdateVisiblePriceSeriesType({
              visibleLogicalRange: logicalRange,
            });
          }

          for (
            let otherChartIndex = 0;
            otherChartIndex <= chartCount - 1;
            otherChartIndex++
          ) {
            if (paneIndex !== otherChartIndex) {
              chart.panes[otherChartIndex]
                .timeScale()
                .setVisibleLogicalRange(logicalRange);
            }
          }
        });

      chartPane.subscribeCrosshairMove(({ time, sourceEvent }) => {
        // Don't override crosshair position from scroll event
        if (time && !sourceEvent) return;

        for (
          let otherChartIndex = 0;
          otherChartIndex <= chartCount - 1;
          otherChartIndex++
        ) {
          const otherChart = chart.panes[otherChartIndex];

          if (otherChart && paneIndex !== otherChartIndex) {
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
    signals.createEffect(selected, (option) => {
      chart.reset({ scale: option.scale });
      applyChartOption(option);
    });
  }
  createApplyChartOptionEffect();
}
