import {
  createShadow,
  createChoiceField,
  createHeader,
} from "../../utils/dom.js";
import { chartElement } from "../../utils/elements.js";
import { ios, canShare } from "../../utils/env.js";
import { serdeChartableIndex } from "../../utils/serde.js";
import { Unit } from "../../utils/units.js";
import signals from "../../signals.js";
import { createChartElement } from "../../chart/index.js";
import { createChartState } from "../../chart/state.js";
import { webSockets } from "../../utils/ws.js";
import { screenshot } from "./screenshot.js";

const keyPrefix = "chart";
const ONE_BTC_IN_SATS = 100_000_000;

/**
 * @typedef {"timestamp" | "date" | "week" | "month" | "quarter" | "semester" | "year" | "decade" } ChartableIndexName
 */

/**
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {Accessor<ChartOption>} args.option
 * @param {BrkClient} args.brk
 */
export function init({ colors, option, brk }) {
  chartElement.append(createShadow("left"));
  chartElement.append(createShadow("right"));

  const { headerElement, headingElement } = createHeader();
  chartElement.append(headerElement);

  const state = createChartState(signals);
  const { fieldset, index } = createIndexSelector(option, state);

  const { from, to } = state.range();

  const chart = createChartElement({
    parent: chartElement,
    signals,
    colors,
    id: "charts",
    brk,
    index,
    initialVisibleBarsCount:
      from !== null && to !== null ? to - from : null,
    timeScaleSetCallback: (unknownTimeScaleCallback) => {
      const { from, to } = state.range();
      if (from !== null && to !== null) {
        chart.setVisibleLogicalRange({ from, to });
      } else {
        unknownTimeScaleCallback();
      }
    },
  });

  if (!(ios && !canShare)) {
    const domain = window.document.createElement("p");
    domain.innerText = `${window.location.host}`;
    domain.id = "domain";

    chart.addFieldsetIfNeeded({
      id: "capture",
      paneIndex: 0,
      position: "ne",
      createChild() {
        const button = window.document.createElement("button");
        button.id = "capture";
        button.innerText = "capture";
        button.title = "Capture chart as image";
        button.addEventListener("click", async () => {
          chartElement.dataset.screenshot = "true";
          chartElement.append(domain);
          try {
            await screenshot({
              element: chartElement,
              name: option().path.join("-"),
              title: option().title,
            });
          } catch {}
          chartElement.removeChild(domain);
          chartElement.dataset.screenshot = "false";
        });
        return button;
      },
    });
  }

  chart.onVisibleLogicalRangeChange((t) => {
    if (!t) return;
    state.setRange({ from: t.from, to: t.to });
  });

  chartElement.append(fieldset);

  const { field: topUnitField, selected: topUnit } = createChoiceField({
    defaultValue: Unit.usd,
    keyPrefix,
    key: "unit-0",
    choices: [Unit.usd, Unit.sats],
    toKey: (u) => u.id,
    toLabel: (u) => u.name,
    signals,
    sorted: true,
    type: "select",
  });

  chart.addFieldsetIfNeeded({
    id: "charts-unit-0",
    paneIndex: 0,
    position: "nw",
    createChild() {
      return topUnitField;
    },
  });

  const seriesListTop = /** @type {AnySeries[]} */ ([]);
  const seriesListBottom = /** @type {AnySeries[]} */ ([]);

  /**
   * @param {Object} params
   * @param {AnySeries} params.series
   * @param {Unit} params.unit
   * @param {IndexName} params.index
   */
  function printLatest({ series, unit, index }) {
    const _latest = webSockets.kraken1dCandle.latest();

    if (!_latest) return;

    const latest = { ..._latest };

    if (unit === Unit.sats) {
      latest.open = Math.floor(ONE_BTC_IN_SATS / latest.open);
      latest.high = Math.floor(ONE_BTC_IN_SATS / latest.high);
      latest.low = Math.floor(ONE_BTC_IN_SATS / latest.low);
      latest.close = Math.floor(ONE_BTC_IN_SATS / latest.close);
    }

    const last_ = series.getData().at(-1);
    if (!last_) return;
    const last = { ...last_ };

    if ("close" in last) {
      last.close = latest.close;
    }
    if ("value" in last) {
      last.value = latest.close;
    }
    const date = new Date(/** @type {number} */ (latest.time) * 1000);

    switch (index) {
      case "height":
      case "difficultyepoch":
      case "halvingepoch": {
        if ("close" in last) {
          last.low = Math.min(last.low, latest.close);
          last.high = Math.max(last.high, latest.close);
        }
        series.update(last);
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
          series.update(last);
        } else {
          last.time = time;
          series.update(last);
        }
      }
    }
  }

  signals.createScopedEffect(option, (option) => {
    headingElement.innerHTML = option.title;

    const bottomUnits = Array.from(option.bottom.keys());

    /** @type {{ field: HTMLDivElement, selected: Accessor<Unit> } | undefined} */
    let bottomUnitSelector;

    if (bottomUnits.length) {
      bottomUnitSelector = createChoiceField({
        defaultValue: bottomUnits[0],
        keyPrefix,
        key: "unit-1",
        choices: bottomUnits,
        toKey: (u) => u.id,
        toLabel: (u) => u.name,
        signals,
        sorted: true,
        type: "select",
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
    } else {
      // Clean up bottom pane when new option has no bottom series
      seriesListBottom.forEach((series) => series.remove());
      seriesListBottom.length = 0;
      chart.legendBottom.removeFrom(0);
    }

    signals.createScopedEffect(index, (index) => {
      signals.createScopedEffect(topUnit, (topUnit) => {
        /** @type {AnySeries | undefined} */
        let series;

        switch (topUnit) {
          case Unit.usd: {
            series = chart.addCandlestickSeries({
              metric: brk.metrics.price.usd.ohlc,
              name: "Price",
              unit: topUnit,
              order: 0,
            });
            break;
          }
          case Unit.sats: {
            series = chart.addCandlestickSeries({
              metric: brk.metrics.price.sats.ohlc,
              name: "Price",
              unit: topUnit,
              inverse: true,
              order: 0,
            });
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
            printLatest({ series, unit: topUnit, index });
          },
        );
      });

      /**
       * @param {Object} args
       * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} args.blueprints
       * @param {number} args.paneIndex
       * @param {Accessor<Unit>} args.unit
       * @param {AnySeries[]} args.seriesList
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
        signals.createScopedEffect(unit, (unit) => {
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
                  seriesList.push(
                    chart.addCandlestickSeries({
                      metric: blueprint.metric,
                      name: blueprint.title,
                      unit,
                      colors: blueprint.colors,
                      defaultActive: blueprint.defaultActive,
                      paneIndex,
                      options,
                      order,
                    }),
                  );
                  break;
                }
                case "Dots": {
                  seriesList.push(
                    chart.addDotsSeries({
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
                  break;
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

    });
  });
}

/**
 * @param {Accessor<ChartOption>} option
 * @param {ReturnType<typeof createChartState>} state
 */
function createIndexSelector(option, state) {
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

  // Create UI that syncs with state.index
  const fieldset = window.document.createElement("fieldset");
  fieldset.id = "interval";

  const screenshotSpan = window.document.createElement("span");
  screenshotSpan.innerText = "interval:";
  fieldset.append(screenshotSpan);

  const select = window.document.createElement("select");
  select.id = "index";
  fieldset.append(select);
  fieldset.dataset.size = "sm";

  // Populate and update options when choices change
  signals.createEffect(choices, (choices) => {
    const currentValue = state.index();
    select.innerHTML = "";
    choices.forEach((choice) => {
      const option = window.document.createElement("option");
      option.value = choice;
      option.textContent = choice;
      option.selected = choice === currentValue;
      select.append(option);
    });
  });

  // Sync select value with state
  signals.createEffect(state.index, (value) => {
    select.value = value;
  });

  select.addEventListener("change", () => {
    state.index.set(/** @type {ChartableIndexName} */ (select.value));
  });

  // Convert short name to internal name
  const index = signals.createMemo(() =>
    serdeChartableIndex.deserialize(state.index()),
  );

  return { fieldset, index };
}
