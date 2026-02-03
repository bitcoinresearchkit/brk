import { createShadow, createRadios, createHeader } from "../utils/dom.js";
import { chartElement } from "../utils/elements.js";
import { serdeChartableIndex } from "../utils/serde.js";
import { Unit } from "../utils/units.js";
import { createChart } from "../chart/index.js";
import { colors } from "../utils/colors.js";
import { webSockets } from "../utils/ws.js";
import { brk } from "../client.js";

const ONE_BTC_IN_SATS = 100_000_000;

/** @type {((opt: ChartOption) => void) | null} */
let _setOption = null;

/**
 * @param {ChartOption} opt
 */
export function setOption(opt) {
  if (!_setOption) throw new Error("Chart not initialized");
  _setOption(opt);
}

export function init() {
  chartElement.append(createShadow("left"));
  chartElement.append(createShadow("right"));

  const { headerElement, headingElement } = createHeader();
  chartElement.append(headerElement);

  const chart = createChart({
    parent: chartElement,
    id: "charts",
    brk,
  });

  // Create index selector
  const { fieldset, setChoices } = createIndexSelector(chart);
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
      colors: colors.bi.profitLoss,
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

  // Set up the setOption function
  _setOption = (opt) => {
    headingElement.innerHTML = opt.title;

    // Update index choices based on option
    setChoices(computeChoices(opt));

    blueprints = chart.setBlueprints({
      name: opt.title,
      top: buildTopBlueprints(opt.top),
      bottom: opt.bottom,
      onDataLoaded: updatePriceWithLatest,
    });
  };

  // Live price update listener
  webSockets.kraken1dCandle.onLatest(updatePriceWithLatest);
}

const ALL_CHOICES = /** @satisfies {ChartableIndexName[]} */ ([
  "timestamp",
  "date",
  "week",
  "month",
  "quarter",
  "semester",
  "year",
  "decade",
]);

/**
 * @param {ChartOption} opt
 * @returns {ChartableIndexName[]}
 */
function computeChoices(opt) {
  if (!opt.top.size && !opt.bottom.size) {
    return [...ALL_CHOICES];
  }
  const rawIndexes = new Set(
    [Array.from(opt.top.values()), Array.from(opt.bottom.values())]
      .flat(2)
      .filter((blueprint) => {
        const path = Object.values(blueprint.metric.by)[0]?.path ?? "";
        return !path.includes("constant_");
      })
      .flatMap((blueprint) => blueprint.metric.indexes()),
  );

  return ALL_CHOICES.filter((choice) =>
    rawIndexes.has(serdeChartableIndex.deserialize(choice)),
  );
}

/**
 * @param {Chart} chart
 */
function createIndexSelector(chart) {
  const fieldset = window.document.createElement("fieldset");
  fieldset.id = "interval";
  fieldset.dataset.size = "sm";

  // Track user's preferred index (only updated on explicit selection)
  let preferredIndex = chart.index.name.value;

  /** @type {HTMLElement | null} */
  let field = null;

  /**
   * @param {ChartableIndexName[]} newChoices
   */
  function setChoices(newChoices) {
    if (field) field.remove();

    // Use preferred index if available, otherwise fall back to first choice
    let currentValue = newChoices.includes(preferredIndex)
      ? preferredIndex
      : (newChoices[0] ?? "date");

    if (currentValue !== chart.index.name.value) {
      chart.index.name.set(currentValue);
    }

    field = createRadios({
      initialValue: currentValue,
      onChange: (v) => {
        preferredIndex = v; // User explicitly selected, update preference
        chart.index.name.set(v);
      },
      choices: newChoices,
      id: "index",
    });
    fieldset.append(field);
  }

  return { fieldset, setChoices };
}
