// @ts-check

const keyPrefix = "chart";
const ONE_BTC_IN_SATS = 100_000_000;
const AUTO = "auto";
const LINE = "line";
const CANDLE = "candle";

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
    () => `chart-timerange-${index()}`
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
    })
  );

  elements.charts.append(fieldset);

  const { field: seriesTypeField, selected: topSeriesType_ } =
    utils.dom.createHorizontalChoiceField({
      defaultValue: CANDLE,
      keyPrefix,
      key: "seriestype-0",
      choices: /** @type {const} */ ([AUTO, CANDLE, LINE]),
      signals,
    });

  const topSeriesType = signals.createMemo(() => {
    const topSeriesType = topSeriesType_();
    if (topSeriesType === AUTO) {
      const t = to();
      const f = from();
      if (!t || !f) return null;
      const diff = t - f;
      if (diff / chart.inner.paneSize().width <= 0.5) {
        return CANDLE;
      } else {
        return LINE;
      }
    } else {
      return topSeriesType;
    }
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

  /**
   * @param {Object} params
   * @param {ISeriesApi<any, number>} params.iseries
   * @param {Unit} params.unit
   * @param {Index} params.index
   */
  function printLatest({ iseries, unit, index }) {
    const _latest = webSockets.kraken1dCandle.latest();

    if (!_latest) return;

    const latest = { ..._latest };

    if (unit === "Sats") {
      latest.open = Math.floor(ONE_BTC_IN_SATS / latest.open);
      latest.high = Math.floor(ONE_BTC_IN_SATS / latest.high);
      latest.low = Math.floor(ONE_BTC_IN_SATS / latest.low);
      latest.close = Math.floor(ONE_BTC_IN_SATS / latest.close);
      latest.value = Math.floor(ONE_BTC_IN_SATS / latest.value);
    }

    const last_ = iseries.data().at(-1);
    if (!last_) return;
    const last = { ...last_ };

    if ("close" in last) {
      last.close = latest.close;
    }
    if ("value" in last) {
      last.value = latest.value;
    }
    const date = new Date(latest.time * 1000);

    switch (index) {
      case /** @satisfies {Height} */ (5): {
        if ("close" in last) {
          last.low = Math.min(last.low, latest.close);
          last.high = Math.max(last.high, latest.close);
        }
        iseries.update(last);
        break;
      }
      case /** @satisfies {DateIndex} */ (0): {
        iseries.update(latest);
        break;
      }
      default: {
        if (index === /** @satisfies {WeekIndex} */ (22)) {
          date.setUTCDate(date.getUTCDate() - ((date.getUTCDay() + 6) % 7));
        } else if (index === /** @satisfies {MonthIndex} */ (7)) {
          date.setUTCDate(1);
        } else if (index === /** @satisfies {QuarterIndex} */ (19)) {
          const month = date.getUTCMonth();
          date.setUTCMonth(month - (month % 3), 1);
        } else if (index === /** @satisfies {YearIndex} */ (23)) {
          date.setUTCMonth(0, 1);
        } else if (index === /** @satisfies {DecadeIndex} */ (1)) {
          date.setUTCFullYear(
            Math.floor(date.getUTCFullYear() / 10) * 10,
            0,
            1
          );
        } else {
          throw Error("Unsupported");
        }

        const time = date.valueOf() / 1000;

        if (time === last.time) {
          if ("close" in last) {
            last.low = Math.min(last.low, latest.low);
            last.high = Math.max(last.high, latest.high);
          }
          iseries.update(last);
        } else {
          latest.time = time;
          iseries.update(latest);
        }
      }
    }
  }

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
        () => ({
          topUnit: topUnit(),
          topSeriesType: topSeriesType(),
        }),
        ({ topUnit, topSeriesType }) => {
          /** @type {Series | undefined} */
          let series;

          switch (topUnit) {
            case "USD": {
              switch (topSeriesType) {
                case CANDLE: {
                  series = chart.addCandlestickSeries({
                    vecId: "ohlc",
                    name: "Price",
                    unit: topUnit,
                    setDataCallback: printLatest,
                    order: 0,
                  });

                  break;
                }
                case LINE: {
                  series = chart.addLineSeries({
                    vecId: "close",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    setDataCallback: printLatest,
                    options: {
                      priceLineVisible: true,
                    },
                    order: 0,
                  });
                }
              }
              break;
            }
            case "Sats": {
              switch (topSeriesType) {
                case CANDLE: {
                  series = chart.addCandlestickSeries({
                    vecId: "ohlc-in-sats",
                    name: "Price",
                    unit: topUnit,
                    inverse: true,
                    setDataCallback: printLatest,
                    order: 0,
                  });
                  break;
                }
                case LINE: {
                  series = chart.addLineSeries({
                    vecId: "close-in-sats",
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    setDataCallback: printLatest,
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

          if (!series) throw Error("Unreachable");

          seriesListTop[0]?.remove();
          seriesListTop[0] = series;

          // setDataCallback insimport("./options").tead of hasData
          signals.createEffect(
            () => ({
              latest: webSockets.kraken1dCandle.latest(),
              hasData: series.hasData(),
            }),
            ({ latest, hasData }) => {
              if (!series || !latest || !hasData) return;
              printLatest({ iseries: series.inner, unit: topUnit, index });
            }
          );
        }
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
                      })
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
                      })
                    );
                }
              }
            });
          });
        }
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
    }
  );

  return { fieldset, index };
}
