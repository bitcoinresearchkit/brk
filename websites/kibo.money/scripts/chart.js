// @ts-check

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {LightweightCharts} args.lightweightCharts
 * @param {Accessor<ChartOption>} args.selected
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 * @param {WebSockets} args.webSockets
 * @param {Elements} args.elements
 * @param {VecsResources} args.vecsResources
 */
export function init({
  colors,
  elements,
  lightweightCharts,
  selected,
  signals,
  utils,
  webSockets,
  vecsResources,
}) {
  console.log("init chart state");

  elements.charts.append(utils.dom.createShadow("left"));
  elements.charts.append(utils.dom.createShadow("right"));

  const { headerElement, titleElement } = utils.dom.createHeader({});
  elements.charts.append(headerElement);
  signals.createEffect(selected, (option) => {
    titleElement.innerHTML = option.title;
  });

  const chartElement = lightweightCharts.createChartElement({
    parent: elements.charts,
    signals,
    colors,
    id: "chart",
    kind: "scrollable",
    utils,
  });

  const indexes = utils.dom.createHorizontalChoiceField({
    title: "Index",
    selected: "date",
    choices: [
      "Timestamp",
      "Date",
      "Week",
      "Difficulty Epoch",
      "Month",
      "Year",
      "Halving Epoch",
      "Decade",
    ],
    id: "index",
    signals,
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.append(indexes);

  elements.charts.append(fieldset);

  const vecs = signals.createSignal(
    /** @type {Set<VecResource<any>>} */ (new Set()),
    {
      equals: false,
    },
  );

  const index = /** @satisfies {Dateindex} */ (1);

  chartElement.createChart(index);

  const ohlc = vecsResources.getOrCreate(index, "ohlc");
  const date = vecsResources.getOrCreate(index, "date");
  date.fetch(-10_000);

  // function createFetchChunksOfVisibleDatasetsEffect() {
  //   signals.createEffect(
  //     () => ({
  //       ids: chart.visibleDatasetIds(),
  //       activeDatasets: activeDatasets(),
  //     }),
  //     ({ ids, activeDatasets }) => {
  //       const datasets = Array.from(activeDatasets);

  //       if (ids.length === 0 || datasets.length === 0) return;

  //       for (let i = 0; i < ids.length; i++) {
  //         const id = ids[i];
  //         for (let j = 0; j < datasets.length; j++) {
  //           datasets[j].fetch(id);
  //         }
  //       }
  //     },
  //   );
  // }
  // createFetchChunksOfVisibleDatasetsEffect();

  // /**
  //  * @param {ChartOption} option
  //  */
  // function applyChartOption(option) {
  //   chart.visibleTimeRange.set(chart.getInitialVisibleTimeRange());

  //   activeDatasets.set((s) => {
  //     s.clear();
  //     return s;
  //   });

  //   const chartsBlueprints = [option.top || [], option.bottom].flatMap(
  //     (list) => (list ? [list] : []),
  //   );

  //   chartsBlueprints.map((seriesBlueprints, paneIndex) => {
  //     const chartPane = chart.createPane({
  //       paneIndex,
  //       unit: paneIndex ? option.unit : "US Dollars",
  //     });

  //     if (!paneIndex) {
  //       /** @type {AnyDatasetPath} */
  //       const datasetPath = `${scale}-to-price`;

  //       const dataset = datasets.getOrCreate(scale, datasetPath);

  //       // Don't trigger reactivity by design
  //       activeDatasets().add(dataset);

  //       const priceSeries = chartPane.createSplitSeries({
  //         blueprint: {
  //           datasetPath,
  //           title: "BTC Price",
  //           type: "Candlestick",
  //         },
  //         dataset,
  //         id: option.id,
  //         index: -1,
  //       });

  //       signals.createEffect(webSockets.kraken1dCandle.latest, (latest) => {
  //         if (!latest) return;

  //         const index = utils.chunkIdToIndex(scale, latest.year);

  //         priceSeries.forEach((splitSeries) => {
  //           const series = splitSeries.chunks.at(index);
  //           if (series) {
  //             signals.createEffect(series, (series) => {
  //               series?.update(latest);
  //             });
  //           }
  //         });
  //       });
  //     }

  //     [...seriesBlueprints].reverse().forEach((blueprint, index) => {
  //       const dataset = datasets.getOrCreate(scale, blueprint.datasetPath);

  //       // Don't trigger reactivity by design
  //       activeDatasets().add(dataset);

  //       chartPane.createSplitSeries({
  //         index,
  //         blueprint,
  //         id: option.id,
  //         dataset,
  //       });
  //     });

  //     activeDatasets.set((s) => s);

  //     return chart;
  //   });
  // }

  // function createApplyChartOptionEffect() {
  //   signals.createEffect(selected, (option) => {
  //     chart.reset({ scale: option.scale, owner: signals.getOwner() });
  //     applyChartOption(option);
  //   });
  // }
  // createApplyChartOptionEffect();
}
