// @ts-check

const keyPrefix = "chart";

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
    elements,
  });

  const index = createIndexSelector({ elements, signals, utils });

  let firstRun = true;

  const { field: seriesTypeField, selected: topSeriesType } =
    utils.dom.createHorizontalChoiceField({
      defaultValue: "Line",
      keyPrefix,
      key: "seriestype-0",
      choices: /** @type {const} */ (["Auto", "Candles", "Line"]),
      signals,
    });

  const { field: topUnitField, selected: topUnit } =
    utils.dom.createHorizontalChoiceField({
      defaultValue: "USD",
      keyPrefix,
      key: "unit-0",
      choices: /** @type {const} */ ([
        /** @satisfies {Unit} */ ("USD"),
        /** @satisfies {Unit} */ ("Sats"),
      ]),
      signals,
      sorted: true,
    });

  signals.createEffect(selected, (option) => {
    headingElement.innerHTML = option.title;

    const bottomUnits = /** @type {readonly Unit[]} */ (
      Object.keys(option.bottom)
    );
    const { field: bottomUnitField, selected: bottomUnit } =
      utils.dom.createHorizontalChoiceField({
        defaultValue: bottomUnits.at(0) || "",
        keyPrefix,
        key: "unit-1",
        choices: bottomUnits,
        signals,
        sorted: true,
      });

    // signals.createEffect(bottomUnit, (bottomUnit) => {
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

    signals.createEffect(index, (index) => {
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

      /** @type {ISeriesApi<any> | null} */
      let prevPriceSeries = null;
      signals.createEffect(
        () => [topUnit(), topSeriesType()],
        ([topUnit, topSeriesType]) => {
          if (prevPriceSeries) {
            chart.inner()?.removeSeries(prevPriceSeries);
          }

          switch (topUnit) {
            case "USD": {
              switch (topSeriesType) {
                case "Candles": {
                  prevPriceSeries = chart.addCandlestickSeries({
                    vecId: "ohlc",
                    name: "Price",
                    unit: topUnit,
                    order: 0,
                  });
                  break;
                }
                case "Line": {
                  prevPriceSeries = chart.addLineSeries({
                    vecId: "close",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    options: {
                      priceLineVisible: true,
                    },
                    order: 0,
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
                  prevPriceSeries = chart.addCandlestickSeries({
                    vecId: "ohlc-in-sats",
                    name: "Price",
                    unit: topUnit,
                    inverse: true,
                    order: 0,
                  });
                  break;
                }
                case "Line": {
                  prevPriceSeries = chart.addLineSeries({
                    vecId: "close-in-sats",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    options: {
                      priceLineVisible: true,
                    },
                    order: 0,
                  });
                }
              }
              break;
            }
          }
        },
      );

      [
        {
          blueprints: option.top,
          paneIndex: 0,
          unit: topUnit,
          prevSeriesList: /** @type {ISeriesApi<any>[]} */ ([]),
        },
        {
          blueprints: option.bottom,
          paneIndex: 1,
          unit: bottomUnit,
          prevSeriesList: /** @type {ISeriesApi<any>[]} */ ([]),
        },
      ].forEach(({ blueprints, paneIndex, unit, prevSeriesList }) => {
        signals.createEffect(unit, (unit) => {
          prevSeriesList.splice(0).forEach((series) => {
            chart.inner()?.removeSeries(series);
          });

          blueprints[unit]?.forEach((blueprint, order) => {
            order++;

            const indexes = /** @type {readonly number[]} */ (
              vecIdToIndexes[blueprint.key]
            );

            if (indexes.includes(index)) {
              switch (blueprint.type) {
                case "Baseline": {
                  prevSeriesList.push(
                    chart.addBaselineSeries({
                      vecId: blueprint.key,
                      // color: blueprint.color,
                      name: blueprint.title,
                      unit,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options: {
                        ...blueprint.options,
                        topLineColor:
                          blueprint.color?.() ?? blueprint.colors?.[0](),
                        bottomLineColor:
                          blueprint.color?.() ?? blueprint.colors?.[1](),
                      },
                      order,
                    }),
                  );
                  break;
                }
                case "Candlestick": {
                  throw Error("TODO");
                  break;
                }
                default:
                  prevSeriesList.push(
                    chart.addLineSeries({
                      vecId: blueprint.key,
                      color: blueprint.color,
                      name: blueprint.title,
                      unit,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options: blueprint.options,
                      order,
                    }),
                  );
              }
            }
          });
        });

        chart
          .inner()
          ?.timeScale()
          .subscribeVisibleLogicalRangeChange(
            utils.debounce((t) => {
              if (t) {
                from.set(t.from);
                to.set(t.to);
              }
            }),
          );

        firstRun = false;
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
    defaultValue: "date",
    keyPrefix,
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
          return /** @satisfies {Height} */ (5);
        case "date":
          return /** @satisfies {DateIndex} */ (0);
        case "week":
          return /** @satisfies {WeekIndex} */ (22);
        case "month":
          return /** @satisfies {MonthIndex} */ (7);
        case "quarter":
          return /** @satisfies {QuarterIndex} */ (19);
        case "year":
          return /** @satisfies {YearIndex} */ (23);
        case "decade":
          return /** @satisfies {DecadeIndex} */ (1);
      }
    },
  );

  return index;
}
