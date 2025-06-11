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

  const { headerElement, headingElement } = utils.dom.createHeader();
  elements.charts.append(headerElement);

  const { index, fieldset } = createIndexSelector({ signals, utils });

  const TIMERANGE_LS_KEY = signals.createMemo(
    () => `chart-timerange-${index()}`,
  );

  let firstRun = true;

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

  const chart = lightweightCharts.createChartElement({
    owner: signals.getOwner(),
    parent: elements.charts,
    signals,
    colors,
    id: "charts",
    utils,
    vecsResources,
    elements,
    index,
    timeScaleSetCallback: (unknownTimeScaleCallback) => {
      // TODO: Although it mostly works in practice, need to make it more robust, there is no guarantee that this runs in order and wait for `from` and `to` to update when `index` and thus `TIMERANGE_LS_KEY` is updated
      // Need to have the right values before the update

      const from_ = from();
      const to_ = to();
      if (from_ !== null && to_ !== null) {
        chart.inner.timeScale().setVisibleLogicalRange({
          from: from_,
          to: to_,
        });
      } else {
        unknownTimeScaleCallback();
      }
    },
  });

  chart.inner.timeScale().subscribeVisibleLogicalRangeChange(
    utils.debounce((t) => {
      if (t) {
        from.set(t.from);
        to.set(t.to);
      }
    }),
  );

  elements.charts.append(fieldset);

  const { field: seriesTypeField, selected: topSeriesType } =
    utils.dom.createHorizontalChoiceField({
      defaultValue: "Line",
      keyPrefix,
      key: "seriestype-0",
      choices: /** @type {const} */ (["Candles", "Line"]),
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

  chart.addFieldsetIfNeeded({
    id: "charts-unit-0",
    paneIndex: 0,
    position: "nw",
    createChild() {
      return topUnitField;
    },
  });

  const seriesListTop = /** @type {Series[]} */ ([]);
  const seriesListBottom = /** @type {Series[]} */ ([]);

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
      signals.createEffect(
        () => [topUnit(), topSeriesType()],
        ([topUnit, topSeriesType]) => {
          switch (topUnit) {
            case "USD": {
              switch (topSeriesType) {
                case "Candles": {
                  const series = chart.addCandlestickSeries({
                    vecId: "ohlc",
                    name: "Price",
                    unit: topUnit,
                    order: 0,
                  });
                  seriesListTop[0]?.remove();
                  seriesListTop[0] = series;
                  break;
                }
                case "Line": {
                  const series = chart.addLineSeries({
                    vecId: "close",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    options: {
                      priceLineVisible: true,
                    },
                    order: 0,
                  });
                  seriesListTop[0]?.remove();
                  seriesListTop[0] = series;
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
                  const series = chart.addCandlestickSeries({
                    vecId: "ohlc-in-sats",
                    name: "Price",
                    unit: topUnit,
                    inverse: true,
                    order: 0,
                  });
                  seriesListTop[0]?.remove();
                  seriesListTop[0] = series;
                  break;
                }
                case "Line": {
                  const series = chart.addLineSeries({
                    vecId: "close-in-sats",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    options: {
                      priceLineVisible: true,
                    },
                    order: 0,
                  });
                  seriesListTop[0]?.remove();
                  seriesListTop[0] = series;
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
          seriesList: seriesListTop,
          orderStart: 1,
          legend: chart.legendTop,
        },
        {
          blueprints: option.bottom,
          paneIndex: 1,
          unit: bottomUnit,
          seriesList: seriesListBottom,
          orderStart: 0,
          legend: chart.legendBottom,
        },
      ].forEach(
        ({
          blueprints,
          paneIndex,
          unit,
          seriesList: seriesList,
          orderStart,
          legend,
        }) => {
          signals.createEffect(unit, (unit) => {
            legend.removeFrom(orderStart);

            seriesList.splice(orderStart).forEach((series) => {
              series.remove();
            });

            blueprints[unit]?.forEach((blueprint, order) => {
              order += orderStart;

              const indexes = /** @type {readonly number[]} */ (
                vecIdToIndexes[blueprint.key]
              );

              if (indexes.includes(index)) {
                switch (blueprint.type) {
                  case "Baseline": {
                    seriesList.push(
                      chart.addBaselineSeries({
                        vecId: blueprint.key,
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
                  }
                  default:
                    seriesList.push(
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
        },
      );

      firstRun = false;
    });
  });
}

/**
 * @param {Object} args
 * @param {Signals} args.signals
 * @param {Utilities} args.utils
 */
function createIndexSelector({ signals, utils }) {
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

  return { fieldset, index };
}
