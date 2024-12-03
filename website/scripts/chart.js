// @ts-check

/**
 * @import { PriceSeriesType } from '../packages/lightweight-charts/types';
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
   * @param {Option} args.option
   * @param {ChartPane} args.chartPane
   */
  function createPriceSeries({ type, option, chartPane }) {
    const s = scale();

    /** @type {AnyDatasetPath} */
    const datasetPath = `${s}-to-price`;

    const dataset = datasets.getOrCreate(s, datasetPath);

    // Don't trigger reactivity by design
    activeDatasets().add(dataset);

    const title = "BTC Price";

    /** @type {SplitSeriesBlueprint} */
    let blueprint;

    if (type === "Candlestick") {
      blueprint = {
        datasetPath,
        title,
        type: "Candlestick",
      };
    } else {
      blueprint = {
        datasetPath,
        title,
        color: colors.default,
      };
    }

    const disabled = signals.createMemo(() => priceSeriesType() !== type);

    const priceSeries = chartPane.createSplitSeries({
      blueprint,
      dataset,
      id: option.id,
      index: -1,
      disabled,
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

    const chartsBlueprints = [option.top || [], option.bottom].flatMap(
      (list) => (list ? [list] : []),
    );

    chartsBlueprints.map((seriesBlueprints, paneIndex) => {
      const chartPane = chart.createPane({
        paneIndex,
        unit: paneIndex ? option.unit : "US Dollars",
        whitespace: true,
      });

      if (!paneIndex) {
        updateVisiblePriceSeriesType({
          visibleTimeRange: chart.visibleTimeRange(),
        });

        chartPane
          .timeScale()
          .subscribeVisibleLogicalRangeChange((logicalRange) => {
            if (!logicalRange) return;

            // Must be the chart with the visible timeScale
            debouncedUpdateVisiblePriceSeriesType({
              visibleLogicalRange: logicalRange,
            });
          });

        /** @param {PriceSeriesType} type */
        function _createPriceSeries(type) {
          return createPriceSeries({
            chartPane,
            option,
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

      [...seriesBlueprints].reverse().forEach((blueprint, index) => {
        const dataset = datasets.getOrCreate(scale, blueprint.datasetPath);

        // Don't trigger reactivity by design
        activeDatasets().add(dataset);

        chartPane.createSplitSeries({
          index,
          blueprint,
          id: option.id,
          dataset,
        });
      });

      activeDatasets.set((s) => s);

      return chart;
    });
  }

  function createApplyChartOptionEffect() {
    signals.createEffect(selected, (option) => {
      chart.reset({ scale: option.scale, owner: signals.getOwner() });
      applyChartOption(option);
    });
  }
  createApplyChartOptionEffect();
}
