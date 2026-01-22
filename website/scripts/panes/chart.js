import {
  createShadow,
  createChoiceField,
  createHeader,
} from "../utils/dom.js";
import { chartElement } from "../utils/elements.js";
import { serdeChartableIndex } from "../utils/serde.js";
import { Unit } from "../utils/units.js";
import signals from "../signals.js";
import { createChart } from "../chart/index.js";
import { colors } from "../chart/colors.js";
import { webSockets } from "../utils/ws.js";

const ONE_BTC_IN_SATS = 100_000_000;

/**
 * @param {Object} args
 * @param {Accessor<ChartOption>} args.option
 * @param {BrkClient} args.brk
 */
export function init({ option, brk }) {
  chartElement.append(createShadow("left"));
  chartElement.append(createShadow("right"));

  const { headerElement, headingElement } = createHeader();
  chartElement.append(headerElement);

  const chart = createChart({
    parent: chartElement,
    id: "charts",
    brk,
    captureElement: chartElement,
  });

  // Create index selector using chart's index state
  const fieldset = createIndexSelector(option, chart);
  chartElement.append(fieldset);

  /**
   * Build top blueprints with price series prepended for each unit
   * @param {Map<Unit, AnyFetchedSeriesBlueprint[]>} optionTop
   * @returns {Map<Unit, AnyFetchedSeriesBlueprint[]>}
   */
  function buildTopBlueprints(optionTop) {
    /** @type {Map<Unit, AnyFetchedSeriesBlueprint[]>} */
    const result = new Map();

    // USD price + option blueprints
    /** @type {FetchedCandlestickSeriesBlueprint} */
    const usdPrice = {
      type: "Candlestick",
      title: "Price",
      metric: brk.metrics.price.usd.ohlc,
    };
    result.set(Unit.usd, [usdPrice, ...(optionTop.get(Unit.usd) ?? [])]);

    // Sats price + option blueprints
    /** @type {FetchedCandlestickSeriesBlueprint} */
    const satsPrice = {
      type: "Candlestick",
      title: "Price",
      metric: brk.metrics.price.sats.ohlc,
      colors: [colors.red, colors.green],
    };
    result.set(Unit.sats, [satsPrice, ...(optionTop.get(Unit.sats) ?? [])]);

    return result;
  }

  /** @type {ReturnType<typeof chart.setBlueprints> | null} */
  let blueprints = null;

  function updatePriceWithLatest() {
    const latest = webSockets.kraken1dCandle.latest();
    if (!latest || !blueprints) return;

    const priceSeries = blueprints.panes[0].series[0];
    const unit = blueprints.panes[0].unit;
    if (!priceSeries?.hasData() || !unit) return;

    const last = /** @type {CandlestickData | undefined} */ (
      priceSeries.getData().at(-1)
    );
    if (!last) return;

    // Convert to sats if needed
    const close =
      unit === Unit.sats
        ? Math.floor(ONE_BTC_IN_SATS / latest.close)
        : latest.close;

    priceSeries.update({ ...last, close });
  }

  // When option changes, update heading and rebuild blueprints
  signals.createEffect(option, (opt) => {
    headingElement.innerHTML = opt.title;

    blueprints = chart.setBlueprints({
      top: buildTopBlueprints(opt.top),
      bottom: opt.bottom,
      onDataLoaded: updatePriceWithLatest,
    });
  });

  // Live price update listener
  signals.createEffect(
    () => webSockets.kraken1dCandle.latest(),
    updatePriceWithLatest,
  );
}

/**
 * @param {Accessor<ChartOption>} option
 * @param {Chart} chart
 */
function createIndexSelector(option, chart) {
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

    return /** @type {any} */ (
      choices_.filter((choice) =>
        rawIndexes.has(serdeChartableIndex.deserialize(choice)),
      )
    );
  });

  const fieldset = window.document.createElement("fieldset");
  fieldset.id = "interval";
  fieldset.dataset.size = "sm";

  const screenshotSpan = window.document.createElement("span");
  screenshotSpan.innerText = "interval:";
  fieldset.append(screenshotSpan);

  // Track user's preferred index (only updated on explicit selection)
  let preferredIndex = chart.index.name.value;

  /** @type {HTMLElement | null} */
  let field = null;
  signals.createEffect(choices, (newChoices) => {
    if (field) field.remove();

    // Use preferred index if available, otherwise fall back to first choice
    let currentValue = newChoices.includes(preferredIndex)
      ? preferredIndex
      : (newChoices[0] ?? "date");

    if (currentValue !== chart.index.name.value) {
      chart.index.name.set(currentValue);
    }

    field = createChoiceField({
      initialValue: currentValue,
      onChange: (v) => {
        preferredIndex = v; // User explicitly selected, update preference
        chart.index.name.set(v);
      },
      choices: newChoices,
      id: "index",
    });
    fieldset.append(field);
  });

  return fieldset;
}
