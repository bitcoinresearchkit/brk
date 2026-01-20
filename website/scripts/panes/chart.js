import { createShadow, createChoiceField, createHeader } from "../utils/dom.js";
import { chartElement } from "../utils/elements.js";
import { serdeChartableIndex } from "../utils/serde.js";
import { Unit } from "../utils/units.js";
import signals from "../signals.js";
import { createChart } from "../chart/index.js";
import { createChartState } from "../chart/state.js";
import { webSockets } from "../utils/ws.js";
import { debounce } from "../utils/timing.js";

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

  const chart = createChart({
    parent: chartElement,
    signals,
    colors,
    id: "charts",
    brk,
    index,
    initialVisibleBarsCount: from !== null && to !== null ? to - from : null,
    captureElement: chartElement,
    timeScaleSetCallback: (unknownTimeScaleCallback) => {
      const { from, to } = state.range();
      if (from !== null && to !== null) {
        chart.setVisibleLogicalRange({ from, to });
      } else {
        unknownTimeScaleCallback();
      }
    },
  });

  // Sync chart → state.range on user pan/zoom
  // Debounce to avoid rapid URL updates while panning
  const debouncedSetRange = debounce(
    (/** @type {{ from: number, to: number }} */ range) =>
      state.setRange(range),
    500,
  );
  chart.onVisibleLogicalRangeChange((t) => {
    if (!t || t.from >= t.to) return;
    debouncedSetRange({ from: t.from, to: t.to });
  });

  chartElement.append(fieldset);

  const unitChoices = /** @type {const} */ ([Unit.usd, Unit.sats]);
  /** @type {Signal<Unit>} */
  const topUnit = signals.createPersistedSignal({
    defaultValue: /** @type {Unit} */ (Unit.usd),
    storageKey: `${keyPrefix}-price`,
    urlKey: "price",
    serialize: (u) => u.id,
    deserialize: (s) =>
      /** @type {Unit} */ (unitChoices.find((u) => u.id === s) ?? Unit.usd),
  });
  const topUnitField = createChoiceField({
    defaultValue: Unit.usd,
    choices: unitChoices,
    toKey: (u) => u.id,
    toLabel: (u) => u.name,
    selected: topUnit,
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

    /** @type {Signal<Unit> | undefined} */
    let bottomUnit;

    if (bottomUnits.length) {
      // Storage key based on unit group (sorted unit IDs) so each group remembers its selection
      const unitGroupKey = bottomUnits
        .map((u) => u.id)
        .sort()
        .join("-");
      bottomUnit = signals.createPersistedSignal({
        defaultValue: bottomUnits[0],
        storageKey: `${keyPrefix}-unit-${unitGroupKey}`,
        urlKey: "unit",
        serialize: (u) => u.id,
        deserialize: (s) =>
          bottomUnits.find((u) => u.id === s) ?? bottomUnits[0],
      });
      const field = createChoiceField({
        defaultValue: bottomUnits[0],
        choices: bottomUnits,
        toKey: (u) => u.id,
        toLabel: (u) => u.name,
        selected: bottomUnit,
        signals,
        sorted: true,
        type: "select",
      });
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

    /**
     * @param {Object} args
     * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} args.blueprints
     * @param {number} args.paneIndex
     * @param {Unit} args.unit
     * @param {IndexName} args.idx
     * @param {AnySeries[]} args.seriesList
     * @param {number} args.orderStart
     * @param {Legend} args.legend
     */
    function createSeriesFromBlueprints({
      blueprints,
      paneIndex,
      unit,
      idx,
      seriesList,
      orderStart,
      legend,
    }) {
      legend.removeFrom(orderStart);
      seriesList.splice(orderStart).forEach((series) => series.remove());

      blueprints.get(unit)?.forEach((blueprint, order) => {
        order += orderStart;
        const options = blueprint.options;
        const indexes = Object.keys(blueprint.metric.by);

        if (indexes.includes(idx)) {
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
    }

    // Price series + top pane blueprints: combined effect on index + topUnit
    signals.createScopedEffect(
      () => ({ idx: index(), unit: topUnit() }),
      ({ idx, unit }) => {
        // Create price series
        /** @type {AnySeries | undefined} */
        let series;
        switch (unit) {
          case Unit.usd: {
            series = chart.addCandlestickSeries({
              metric: brk.metrics.price.usd.ohlc,
              name: "Price",
              unit,
              order: 0,
            });
            break;
          }
          case Unit.sats: {
            series = chart.addCandlestickSeries({
              metric: brk.metrics.price.sats.ohlc,
              name: "Price",
              unit,
              inverse: true,
              order: 0,
            });
            break;
          }
        }
        if (!series) throw Error("Unreachable");

        seriesListTop[0]?.remove();
        seriesListTop[0] = series;

        // Live price update effect
        signals.createEffect(
          () => ({
            latest: webSockets.kraken1dCandle.latest(),
            hasData: series.hasData(),
          }),
          ({ latest, hasData }) => {
            if (!series || !latest || !hasData) return;
            printLatest({ series, unit, index: idx });
          },
        );

        // Top pane blueprint series
        createSeriesFromBlueprints({
          blueprints: option.top,
          paneIndex: 0,
          unit,
          idx,
          seriesList: seriesListTop,
          orderStart: 1,
          legend: chart.legendTop,
        });
      },
    );

    // Bottom pane blueprints: combined effect on index + bottomUnit
    if (bottomUnit) {
      signals.createScopedEffect(
        () => ({ idx: index(), unit: bottomUnit() }),
        ({ idx, unit }) => {
          createSeriesFromBlueprints({
            blueprints: option.bottom,
            paneIndex: 1,
            unit,
            idx,
            seriesList: seriesListBottom,
            orderStart: 0,
            legend: chart.legendBottom,
          });
        },
      );
    }
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

  /** @type {ChartableIndexName} */
  const defaultIndex = "date";
  const field = createChoiceField({
    defaultValue: defaultIndex,
    selected: state.index,
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

  // Convert short name to internal name
  const index = signals.createMemo(() =>
    serdeChartableIndex.deserialize(state.index()),
  );

  return { fieldset, index };
}
