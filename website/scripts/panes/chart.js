import { createHeader } from "../utils/dom.js";
import { chartElement } from "../utils/elements.js";
import { serdeChartableIndex } from "../utils/serde.js";
import { Unit } from "../utils/units.js";
import { createChart } from "../chart/index.js";
import { colors } from "../utils/colors.js";
import { latestPrice, onPrice } from "../utils/price.js";
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
  const { headerElement, headingElement } = createHeader();
  chartElement.append(headerElement);

  const chart = createChart({
    parent: chartElement,
    brk,
  });

  const setChoices = chart.setIndexChoices;

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
      metric: brk.metrics.prices.ohlc.usd.day1,
    };
    result.set(Unit.usd, [usdPrice, ...(optionTop.get(Unit.usd) ?? [])]);

    // Sats price + option blueprints
    /** @type {FetchedCandlestickSeriesBlueprint} */
    const satsPrice = {
      type: "Candlestick",
      title: "Price",
      metric: brk.metrics.prices.ohlc.sats.day1,
      colors: /** @type {const} */ ([colors.bi.p1[1], colors.bi.p1[0]]),
    };
    result.set(Unit.sats, [satsPrice, ...(optionTop.get(Unit.sats) ?? [])]);

    return result;
  }

  function updatePriceWithLatest() {
    const latest = latestPrice();
    if (latest === null) return;

    const priceSeries = chart.panes[0].series[0];
    const unit = chart.panes[0].unit;
    if (!priceSeries?.hasData() || !unit) return;

    const last = /** @type {CandlestickData | undefined} */ (
      priceSeries.getData().at(-1)
    );
    if (!last) return;

    // Convert to sats if needed
    const close =
      unit === Unit.sats
        ? Math.floor(ONE_BTC_IN_SATS / latest)
        : latest;

    priceSeries.update({ ...last, close });
  }

  // Set up the setOption function
  _setOption = (opt) => {
    headingElement.innerHTML = opt.title;

    // Set blueprints first so storageId is correct before any index change
    chart.setBlueprints({
      name: opt.title,
      top: buildTopBlueprints(opt.top()),
      bottom: opt.bottom(),
      onDataLoaded: updatePriceWithLatest,
    });

    // Update index choices (may trigger rebuild if index changes)
    setChoices(computeChoices(opt));
  };

  // Live price update listener
  onPrice(updatePriceWithLatest);
}

const ALL_CHOICES = /** @satisfies {ChartableIndexName[]} */ ([
  "timestamp",
  "date",
  "week",
  "month",
  "month3",
  "month6",
  "year",
  "year10",
]);

/**
 * @param {ChartOption} opt
 * @returns {ChartableIndexName[]}
 */
function computeChoices(opt) {
  if (!opt.top().size && !opt.bottom().size) {
    return [...ALL_CHOICES];
  }
  const rawIndexes = new Set(
    [Array.from(opt.top().values()), Array.from(opt.bottom().values())]
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

