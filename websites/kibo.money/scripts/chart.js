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
    id: "charts",
    utils,
    vecsResources,
  });

  const index = createIndexSelector({ elements, signals, utils });

  let firstRun = true;

  signals.createEffect(selected, (option) => {
    headingElement.innerHTML = option.title;
    signals.createEffect(index, (index) => {
      const { field: topUnitField, selected: topUnit } =
        utils.dom.createHorizontalChoiceField({
          defaultValue: "USD",
          keyPrefix: "charts",
          key: "unit-0",
          choices: /** @type {const} */ ([
            /** @satisfies {Unit} */ ("USD"),
            /** @satisfies {Unit} */ ("Sats"),
          ]),
          signals,
        });

      signals.createEffect(topUnit, (topUnit) => {
        const { field: seriesTypeField, selected: topSeriesType } =
          utils.dom.createHorizontalChoiceField({
            defaultValue: "Candles",
            keyPrefix: "charts",
            key: "seriestype-0",
            choices: /** @type {const} */ (["Candles", "Line"]),
            signals,
          });

        signals.createEffect(topSeriesType, (topSeriesType) => {
          const bottomUnits = /** @type {readonly Unit[]} */ (
            Object.keys(option.bottom)
          );
          const { field: bottomUnitField, selected: bottomUnit } =
            utils.dom.createHorizontalChoiceField({
              defaultValue: bottomUnits.at(0) || "",
              keyPrefix: "charts",
              key: "unit-1",
              choices: bottomUnits,
              signals,
            });

          signals.createEffect(bottomUnit, (bottomUnit) => {
            chart.reset({ owner: signals.getOwner() });

            chart.addFieldsetIfNeeded({
              id: "charts-unit-0",
              paneIndex: 0,
              position: "nw",
              createChild() {
                return topUnitField;
              },
            });

            if (bottomUnits.length) {
              chart.addFieldsetIfNeeded({
                id: "charts-unit-1",
                paneIndex: 1,
                position: "nw",
                createChild() {
                  return bottomUnitField;
                },
              });
            }

            chart.addFieldsetIfNeeded({
              id: "charts-seriestype-0",
              paneIndex: 0,
              position: "ne",
              createChild() {
                return seriesTypeField;
              },
            });

            const TIMERANGE_LS_KEY = `chart-timerange-${index}`;

            const from = signals.createSignal(
              /** @type {number | null} */ (null),
              {
                save: {
                  ...utils.serde.optNumber,
                  keyPrefix: TIMERANGE_LS_KEY,
                  key: "from",
                  serializeParam: firstRun,
                },
              },
            );
            const to = signals.createSignal(
              /** @type {number | null} */ (null),
              {
                save: {
                  ...utils.serde.optNumber,
                  keyPrefix: TIMERANGE_LS_KEY,
                  key: "to",
                  serializeParam: firstRun,
                },
              },
            );

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

            switch (topUnit) {
              case "USD": {
                switch (topSeriesType) {
                  case "Candles": {
                    const candles = chart.addCandlestickSeries({
                      vecId: "ohlc",
                      name: "Price",
                      unit: topUnit,
                    });
                    break;
                  }
                  case "Line": {
                    const line = chart.addLineSeries({
                      vecId: "close",
                      name: "Price",
                      unit: topUnit,
                      color: colors.default,
                      options: {
                        priceLineVisible: true,
                      },
                    });
                  }
                }
                // signals.createEffect(webSockets.kraken1dCandle.latest, (latest) => {
                //   if (!latest) return;
                //   const last = /** @type { CandlestickData | undefined} */ (
                //     candles.data().at(-1)
                //   );
                //   if (!last) return;
                //   candles?.update({ ...last, close: latest.close });
                // });
                break;
              }
              case "Sats": {
                switch (topSeriesType) {
                  case "Candles": {
                    const candles = chart.addCandlestickSeries({
                      vecId: "ohlc-in-sats",
                      name: "Price",
                      unit: topUnit,
                      inverse: true,
                    });
                    break;
                  }
                  case "Line": {
                    const line = chart.addLineSeries({
                      vecId: "close-in-sats",
                      name: "Price",
                      unit: topUnit,
                      color: colors.default,
                      options: {
                        priceLineVisible: true,
                      },
                    });
                  }
                }
                break;
              }
            }

            [
              { blueprints: option.top, paneIndex: 0 },
              { blueprints: option.bottom, paneIndex: 1 },
            ].forEach(({ blueprints, paneIndex }) => {
              const unit = paneIndex ? bottomUnit : topUnit;
              console.log({ unit });
              blueprints[unit]?.forEach((blueprint) => {
                const indexes = /** @type {readonly number[]} */ (
                  vecIdToIndexes[blueprint.key]
                );
                if (indexes.includes(index)) {
                  chart.addLineSeries({
                    vecId: blueprint.key,
                    color: blueprint.color,
                    name: blueprint.title,
                    unit,
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
      });
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
  const { field, selected } = utils.dom.createHorizontalChoiceField({
    title: "Index",
    defaultValue: "date",
    keyPrefix: "charts",
    key: "index",
    choices: /**@type {const} */ ([
      "timestamp",
      "date",
      "week",
      // "difficulty epoch",
      "month",
      "quarter",
      "year",
      // "halving epoch",
      "decade",
    ]),
    id: "index",
    signals,
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.append(field);
  fieldset.dataset.size = "sm";
  elements.charts.append(fieldset);

  const index = signals.createMemo(
    /** @returns {ChartableIndex} */ () => {
      switch (selected()) {
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
