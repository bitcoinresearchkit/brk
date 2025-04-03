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

  const { headerElement, headingElement } = utils.dom.createHeader({});
  elements.charts.append(headerElement);

  const chart = lightweightCharts.createChartElement({
    parent: elements.charts,
    signals,
    colors,
    id: "chart",
    utils,
    vecsResources,
  });

  const index_ = createIndexSelector({ elements, signals, utils });

  let firstRun = true;

  signals.createEffect(selected, (option) => {
    headingElement.innerHTML = option.title;
    signals.createEffect(index_, (index) => {
      chart.reset({ owner: signals.getOwner() });

      const TIMERANGE_LS_KEY = `chart-timerange-${index}`;

      const from = signals.createSignal(/** @type {number | null} */ (null), {
        save: {
          ...utils.serde.optNumber,
          keyPrefix: TIMERANGE_LS_KEY,
          key: "from",
          serializeParam: firstRun,
        },
      });
      const to = signals.createSignal(/** @type {number | null} */ (null), {
        save: {
          ...utils.serde.optNumber,
          keyPrefix: TIMERANGE_LS_KEY,
          key: "to",
          serializeParam: firstRun,
        },
      });

      chart.create({
        index,
        timeScaleSetCallback: (unknownTimeScaleCallback) => {
          const from_ = from();
          const to_ = to();
          if (from_ !== null && to_ !== null) {
            chart.inner()?.timeScale().setVisibleLogicalRange({
              from: from_,
              to: to_,
            });
          } else {
            unknownTimeScaleCallback();
          }
        },
      });

      const candles = chart.addCandlestickSeries({
        vecId: "ohlc",
        name: "Price",
        unit: "US Dollars",
      });
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
            chart.addLineSeries({
              vecId: blueprint.key,
              color: blueprint.color,
              name: blueprint.title,
              unit: option.unit,
              defaultActive: blueprint.defaultActive,
              paneIndex,
            });
          }
        });
      });

      chart
        .inner()
        ?.timeScale()
        .subscribeVisibleLogicalRangeChange(
          utils.debounce((t) => {
            from.set(t.from);
            to.set(t.to);
          }),
        );

      firstRun = false;
    });
  });
}

/**
 * @param {Object} args
 * @param {Elements} args.elements
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 */
function createIndexSelector({ elements, signals, utils }) {
  const indexChoices = /**@type {const} */ ([
    "timestamp",
    "date",
    "week",
    // "difficulty epoch",
    "month",
    "quarter",
    "year",
    // "halving epoch",
    "decade",
  ]);
  /** @typedef {(typeof indexChoices)[number]} SerializedIndex */
  const serializedIndex = /** @type {Signal<SerializedIndex>} */ (
    signals.createSignal("date", {
      save: {
        keyPrefix: "charts",
        key: "index",
        ...utils.serde.string,
      },
    })
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
    serializedIndex.set(value);
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.append(indexesField);
  elements.charts.append(fieldset);

  const index = signals.createMemo(
    /** @returns {Index} */ () => {
      switch (serializedIndex()) {
        case "timestamp":
          return /** @satisfies {Height} */ (0);
        case "date":
          return /** @satisfies {Dateindex} */ (1);
        case "week":
          return /** @satisfies {Weekindex} */ (2);
        case "month":
          return /** @satisfies {Monthindex} */ (4);
        case "quarter":
          return /** @satisfies {Quarterindex} */ (5);
        case "year":
          return /** @satisfies {Yearindex} */ (6);
        case "decade":
          return /** @satisfies {Decadeindex} */ (7);
      }
    },
  );

  return index;
}
