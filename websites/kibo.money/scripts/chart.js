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
 * @param {VecIdToIndexes} args.vecIdToIndexes
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
  vecIdToIndexes,
}) {
  console.log("init chart state");

  elements.charts.append(utils.dom.createShadow("left"));
  elements.charts.append(utils.dom.createShadow("right"));

  const { headerElement, titleElement } = utils.dom.createHeader({});
  elements.charts.append(headerElement);

  const chart = lightweightCharts.createChartElement({
    parent: elements.charts,
    signals,
    colors,
    id: "chart",
    kind: "scrollable",
    utils,
    vecsResources,
  });

  const index = createIndexSelector({ elements, signals, utils });

  // const vecs = signals.createSignal(
  //   /** @type {Set<VecResource>} */ (new Set()),
  //   {
  //     equals: false,
  //   },
  // );

  signals.createEffect(selected, (option) => {
    titleElement.innerHTML = option.title;
    signals.createEffect(index, (index) => {
      chart.reset({ owner: signals.getOwner() });

      chart.create(index);

      const candles = chart.addCandlestickSeries("ohlc");

      signals.createEffect(webSockets.kraken1dCandle.latest, (latest) => {
        if (!latest) return;
        const last = /** @type { CandlestickData | undefined} */ (
          candles.data().at(-1)
        );
        if (!last) return;
        candles?.update({ ...last, close: latest.close });
      });

      [
        { blueprints: option.top, paneIndex: 0 },
        { blueprints: option.bottom, paneIndex: 1 },
      ].forEach(({ blueprints, paneIndex }) => {
        blueprints?.forEach((blueprint) => {
          if (vecIdToIndexes[blueprint.key].includes(index)) {
            const series = chart.addLineSeries(blueprint.key, paneIndex);
            series.applyOptions({
              visible: blueprint.defaultActive !== false,
              color: blueprint.color?.(),
            });
          }
        });
      });
    });
  });

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

/**
 * @param {Object} args
 * @param {Elements} args.elements
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 */
function createIndexSelector({ elements, signals, utils }) {
  const indexLSKey = "chart-index";
  const indexChoices = /**@type {const} */ ([
    "timestamp",
    "date",
    "week",
    // "difficulty epoch",
    "month",
    "year",
    // "halving epoch",
    "decade",
  ]);
  /** @typedef {(typeof indexChoices)[number]} SerializedIndex */
  const serializedIndex = signals.createSignal(
    /** @type {SerializedIndex} */ (localStorage.getItem(indexLSKey) || "date"),
  );
  const indexesField = utils.dom.createHorizontalChoiceField({
    title: "Index",
    selected: serializedIndex(),
    choices: indexChoices,
    id: "index",
    signals,
  });
  indexesField.addEventListener("change", (event) => {
    // @ts-ignore
    const value = event.target.value;
    localStorage.setItem(indexLSKey, value);
    serializedIndex.set(value);
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.append(indexesField);
  elements.charts.append(fieldset);

  const index = signals.createMemo(
    /** @returns {Index} */ () => {
      switch (serializedIndex()) {
        case "timestamp":
          return /** @satisfies {Height} */ (2);
        case "date":
          return /** @satisfies {Dateindex} */ (1);
        case "week":
          return /** @satisfies {Weekindex} */ (13);
        case "month":
          return /** @satisfies {Monthindex} */ (14);
        case "year":
          return /** @satisfies {Yearindex} */ (15);
        case "decade":
          return /** @satisfies {Decadeindex} */ (16);
      }
    },
  );

  return index;
}
