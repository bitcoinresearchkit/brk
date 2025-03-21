// @ts-check

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.selected
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {Datasets} args.datasets
 * @param {Constants} args.consts
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 */
export function init({
  colors,
  datasets,
  elements,
  lightweightCharts,
  selected,
  signals,
  utils,
  consts,
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
    consts,
    utils,
  });

  const activeDatasets = signals.createSignal(
    /** @type {Set<ResourceDataset<any, any>>} */ (new Set()),
    {
      equals: false,
    },
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
      });

      if (!paneIndex) {
        /** @type {AnyDatasetPath} */
        const datasetPath = `${scale}-to-price`;

        const dataset = datasets.getOrCreate(scale, datasetPath);

        // Don't trigger reactivity by design
        activeDatasets().add(dataset);

        const priceSeries = chartPane.createSplitSeries({
          blueprint: {
            datasetPath,
            title: "BTC Price",
            type: "Candlestick",
          },
          dataset,
          id: option.id,
          index: -1,
        });

        signals.createEffect(webSockets.kraken1dCandle.latest, (latest) => {
          if (!latest) return;

          const index = utils.chunkIdToIndex(scale, latest.year);

          priceSeries.forEach((splitSeries) => {
            const series = splitSeries.chunks.at(index);
            if (series) {
              signals.createEffect(series, (series) => {
                series?.update(latest);
              });
            }
          });
        });
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
