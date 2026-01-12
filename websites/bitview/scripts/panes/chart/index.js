import {
  createShadow,
  createHorizontalChoiceField,
  createHeader,
} from "../../utils/dom.js";
import { chartElement } from "../../utils/elements.js";
import { ios, canShare } from "../../utils/env.js";
import { serdeChartableIndex, serdeOptNumber } from "../../utils/serde.js";
import { throttle } from "../../utils/timing.js";
import { Unit } from "../../utils/units.js";
import signals from "../../signals.js";
import { createChartElement } from "../../chart/index.js";
import { webSockets } from "../../utils/ws.js";

const keyPrefix = "chart";
const ONE_BTC_IN_SATS = 100_000_000;
const AUTO = "auto";
const LINE = "line";
const CANDLE = "candle";

/**
 * @typedef {"timestamp" | "date" | "week" | "month" | "quarter" | "semester" | "year" | "decade" } ChartableIndexName
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Accessor<ChartOption>} args.option
 * @param {BrkClient} args.brk
 */
export function init({
  colors,
  option,
  brk,
}) {
  chartElement.append(createShadow("left"));
  chartElement.append(createShadow("right"));

  const { headerElement, headingElement } = createHeader();
  chartElement.append(headerElement);

  const { index, fieldset } = createIndexSelector(option);

  const TIMERANGE_LS_KEY = signals.createMemo(
    () => `chart-timerange-${index()}`,
  );

  let firstRun = true;

  const from = signals.createSignal(/** @type {number | null} */ (null), {
    save: {
      ...serdeOptNumber,
      keyPrefix: TIMERANGE_LS_KEY,
      key: "from",
      serializeParam: firstRun,
    },
  });
  const to = signals.createSignal(/** @type {number | null} */ (null), {
    save: {
      ...serdeOptNumber,
      keyPrefix: TIMERANGE_LS_KEY,
      key: "to",
      serializeParam: firstRun,
    },
  });

  const chart = createChartElement({
    parent: chartElement,
    signals,
    colors,
    id: "charts",
    brk,
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

  if (!(ios && !canShare)) {
    const chartBottomRightCanvas = Array.from(
      chart.inner.chartElement().getElementsByTagName("tr"),
    ).at(-1)?.lastChild?.firstChild?.firstChild;
    if (chartBottomRightCanvas) {
      const domain = window.document.createElement("p");
      domain.innerText = `${window.location.host}`;
      domain.id = "domain";
      const screenshotButton = window.document.createElement("button");
      screenshotButton.id = "screenshot";
      const camera = "[ ◉¯]";
      screenshotButton.innerHTML = camera;
      screenshotButton.title = "Screenshot";
      chartBottomRightCanvas.replaceWith(screenshotButton);
      screenshotButton.addEventListener("click", () => {
        import("./screenshot").then(async ({ screenshot }) => {
          chartElement.dataset.screenshot = "true";
          chartElement.append(domain);
          seriesTypeField.hidden = true;
          try {
            await screenshot({
              element: chartElement,
              name: option().path.join("-"),
              title: option().title,
            });
          } catch {}
          chartElement.removeChild(domain);
          seriesTypeField.hidden = false;
          chartElement.dataset.screenshot = "false";
        });
      });
    }
  }

  chart.inner.timeScale().subscribeVisibleLogicalRangeChange(
    throttle((t) => {
      if (!t) return;
      from.set(t.from);
      to.set(t.to);
    }, 250),
  );

  chartElement.append(fieldset);

  const { field: seriesTypeField, selected: topSeriesType_ } =
    createHorizontalChoiceField({
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
    createHorizontalChoiceField({
      defaultValue: Unit.usd,
      keyPrefix,
      key: "unit-0",
      choices: [Unit.usd, Unit.sats],
      toKey: (u) => u.id,
      toLabel: (u) => u.name,
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
   * @param {ISeries} params.iseries
   * @param {Unit} params.unit
   * @param {IndexName} params.index
   */
  function printLatest({ iseries, unit, index }) {
    const _latest = webSockets.kraken1dCandle.latest();

    if (!_latest) return;

    const latest = { ..._latest };

    if (unit === Unit.sats) {
      latest.open = Math.floor(ONE_BTC_IN_SATS / latest.open);
      latest.high = Math.floor(ONE_BTC_IN_SATS / latest.high);
      latest.low = Math.floor(ONE_BTC_IN_SATS / latest.low);
      latest.close = Math.floor(ONE_BTC_IN_SATS / latest.close);
    }

    const last_ = iseries.data().at(-1);
    if (!last_) return;
    const last = { ...last_ };

    if ("close" in last) {
      last.close = latest.close;
    }
    if ("value" in last) {
      last.value = latest.close;
    }
    const date = new Date(latest.time * 1000);

    switch (index) {
      case "height":
      case "difficultyepoch":
      case "halvingepoch": {
        if ("close" in last) {
          last.low = Math.min(last.low, latest.close);
          last.high = Math.max(last.high, latest.close);
        }
        iseries.update(last);
        break;
      }
      default: {
        if (index === "weekindex") {
          date.setUTCDate(date.getUTCDate() - ((date.getUTCDay() + 6) % 7));
        } else if (index === "monthindex") {
          date.setUTCDate(1);
        } else if (index === "quarterindex") {
          const month = date.getUTCMonth();
          date.setUTCMonth(month - (month % 3), 1);
        } else if (index === "semesterindex") {
          const month = date.getUTCMonth();
          date.setUTCMonth(month - (month % 6), 1);
        } else if (index === "yearindex") {
          date.setUTCMonth(0, 1);
        } else if (index === "decadeindex") {
          date.setUTCFullYear(
            Math.floor(date.getUTCFullYear() / 10) * 10,
            0,
            1,
          );
        } else if (index !== "dateindex") {
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
          last.time = time;
          iseries.update(last);
        }
      }
    }
  }

  signals.createEffect(option, (option) => {
    headingElement.innerHTML = option.title;

    const bottomUnits = Array.from(option.bottom.keys());

    /** @type {{ field: HTMLDivElement, selected: Accessor<Unit> } | undefined} */
    let bottomUnitSelector;

    if (bottomUnits.length) {
      bottomUnitSelector = createHorizontalChoiceField({
        defaultValue: bottomUnits[0],
        keyPrefix,
        key: "unit-1",
        choices: bottomUnits,
        toKey: (u) => u.id,
        toLabel: (u) => u.name,
        signals,
        sorted: true,
      });

      const field = bottomUnitSelector.field;
      chart.addFieldsetIfNeeded({
        id: "charts-unit-1",
        paneIndex: 1,
        position: "nw",
        createChild() {
          return field;
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

          console.log({ topUnit, topSeriesType });

          switch (topUnit) {
            case Unit.usd: {
              switch (topSeriesType) {
                case null:
                case CANDLE: {
                  series = chart.addCandlestickSeries({
                    metric: brk.metrics.price.usd.ohlc,
                    name: "Price",
                    unit: topUnit,
                    setDataCallback: printLatest,
                    order: 0,
                  });
                  break;
                }
                case LINE: {
                  series = chart.addLineSeries({
                    metric: brk.metrics.price.usd.split.close,
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    setDataCallback: printLatest,
                    options: {
                      priceLineVisible: true,
                      lastValueVisible: true,
                    },
                    order: 0,
                  });
                }
              }
              break;
            }
            case Unit.sats: {
              switch (topSeriesType) {
                case null:
                case CANDLE: {
                  series = chart.addCandlestickSeries({
                    metric: brk.metrics.price.sats.ohlc,
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
                    metric: brk.metrics.price.sats.split.close,
                    name: "Price",
                    unit: topUnit,
                    color: colors.default,
                    setDataCallback: printLatest,
                    options: {
                      priceLineVisible: true,
                      lastValueVisible: true,
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

          signals.createEffect(
            () => ({
              latest: webSockets.kraken1dCandle.latest(),
              hasData: series.hasData(),
            }),
            ({ latest, hasData }) => {
              if (!series || !latest || !hasData) return;
              printLatest({ iseries: series.inner, unit: topUnit, index });
            },
          );
        },
      );

      /**
       * @param {Object} args
       * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} args.blueprints
       * @param {number} args.paneIndex
       * @param {Accessor<Unit>} args.unit
       * @param {Series[]} args.seriesList
       * @param {number} args.orderStart
       * @param {Legend} args.legend
       */
      function processPane({
        blueprints,
        paneIndex,
        unit,
        seriesList,
        orderStart,
        legend,
      }) {
        signals.createEffect(unit, (unit) => {
          legend.removeFrom(orderStart);

          seriesList.splice(orderStart).forEach((series) => {
            series.remove();
          });

          blueprints.get(unit)?.forEach((blueprint, order) => {
            order += orderStart;

            const options = blueprint.options;

            // Tree-first: metric is now an accessor with .by property
            const indexes = Object.keys(blueprint.metric.by);

            if (indexes.includes(index)) {
              switch (blueprint.type) {
                case "Baseline": {
                  seriesList.push(
                    chart.addBaselineSeries({
                      metric: blueprint.metric,
                      name: blueprint.title,
                      unit,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options: {
                        ...options,
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
                case "Histogram": {
                  seriesList.push(
                    chart.addHistogramSeries({
                      metric: blueprint.metric,
                      name: blueprint.title,
                      unit,
                      color: blueprint.color,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options,
                      order,
                    }),
                  );
                  break;
                }
                case "Candlestick": {
                  throw Error("TODO");
                }
                case "Line":
                case undefined:
                  seriesList.push(
                    chart.addLineSeries({
                      metric: blueprint.metric,
                      color: blueprint.color,
                      name: blueprint.title,
                      unit,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options,
                      order,
                    }),
                  );
              }
            }
          });
        });
      }

      processPane({
        blueprints: option.top,
        paneIndex: 0,
        unit: topUnit,
        seriesList: seriesListTop,
        orderStart: 1,
        legend: chart.legendTop,
      });

      if (bottomUnitSelector) {
        processPane({
          blueprints: option.bottom,
          paneIndex: 1,
          unit: bottomUnitSelector.selected,
          seriesList: seriesListBottom,
          orderStart: 0,
          legend: chart.legendBottom,
        });
      }

      firstRun = false;
    });
  });
}

/**
 * @param {Accessor<ChartOption>} option
 */
function createIndexSelector(option) {
  const choices_ = /** @satisfies {ChartableIndexName[]} */ ([
    "timestamp",
    "date",
    "week",
    "month",
    "quarter",
    "semester",
    "year",
    "decade",
  ]);

  /** @type {Accessor<typeof choices_>} */
  const choices = signals.createMemo(() => {
    const o = option();

    if (!o.top.size && !o.bottom.size) {
      return [...choices_];
    }
    const rawIndexes = new Set(
      [Array.from(o.top.values()), Array.from(o.bottom.values())]
        .flat(2)
        .filter((blueprint) => {
          const path = Object.values(blueprint.metric.by)[0]?.path ?? "";
          return !path.includes("constant_");
        })
        .flatMap((blueprint) => blueprint.metric.indexes()),
    );

    const serializedIndexes = [...rawIndexes].flatMap((index) => {
      const c = serdeChartableIndex.serialize(index);
      return c ? [c] : [];
    });

    return /** @type {any} */ (
      choices_.filter((choice) => serializedIndexes.includes(choice))
    );
  });

  /** @type {ChartableIndexName} */
  const defaultIndex = "date";
  const { field, selected } = createHorizontalChoiceField({
    defaultValue: defaultIndex,
    keyPrefix,
    key: "index",
    choices,
    id: "index",
    signals,
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.id = "interval";

  const screenshotSpan = window.document.createElement("span");
  screenshotSpan.innerText = "interval:";
  fieldset.append(screenshotSpan);

  fieldset.append(field);
  fieldset.dataset.size = "sm";

  const index = signals.createMemo(() =>
    serdeChartableIndex.deserialize(selected()),
  );

  return { fieldset, index };
}
