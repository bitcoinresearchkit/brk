/** Shared cohort chart section builders */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { baseline, dots, line, price } from "../series.js";
import { satsBtcUsd, createPriceRatioCharts, formatCohortTitle } from "../shared.js";

// ============================================================================
// Generic Price Helpers
// ============================================================================

/**
 * Create price folder (price + ratio + z-scores wrapped in folder)
 * For cohorts with full extended ratio metrics (ActivePriceRatioPattern)
 * @param {PartialContext} ctx
 * @param {{ name: string, cohortTitle?: string, priceMetric: ActivePricePattern, ratioPattern: AnyRatioPattern, color: Color }} args
 * @returns {PartialOptionsGroup}
 */
export function createPriceFolder(ctx, { name, cohortTitle, priceMetric, ratioPattern, color }) {
  const context = cohortTitle ? `${cohortTitle} ${name}` : name;
  return {
    name,
    tree: createPriceRatioCharts(ctx, {
      context,
      legend: name,
      pricePattern: priceMetric,
      ratio: ratioPattern,
      color,
    }),
  };
}

/**
 * Create basic price charts (price + ratio only, no z-scores) - flat array
 * For cohorts with basic ratio metrics (only .ratio field)
 * @template {AnyMetricPattern} R
 * @param {{ name: string, context: string, priceMetric: ActivePricePattern, ratioMetric: R, color: Color }} args
 * @returns {PartialOptionsTree}
 */
export function createBasicPriceCharts({ name, context, priceMetric, ratioMetric, color }) {
  return [
    {
      name: "Price",
      title: context,
      top: [price({ metric: priceMetric, name, color })],
    },
    {
      name: "Ratio",
      title: formatCohortTitle(context)("Ratio"),
      bottom: [
        baseline({
          metric: ratioMetric,
          name: "Ratio",
          color,
          unit: Unit.ratio,
          base: 1,
        }),
      ],
    },
  ];
}

/**
 * Create basic price folder (price + ratio wrapped in folder, no z-scores)
 * For cohorts with basic ratio metrics (only .ratio field)
 * @template {AnyMetricPattern} R
 * @param {{ name: string, cohortTitle?: string, priceMetric: ActivePricePattern, ratioMetric: R, color: Color }} args
 * @returns {PartialOptionsGroup}
 */
export function createBasicPriceFolder({ name, cohortTitle, priceMetric, ratioMetric, color }) {
  const context = cohortTitle ? `${cohortTitle} ${name}` : name;
  return {
    name,
    tree: createBasicPriceCharts({ name, context, priceMetric, ratioMetric, color }),
  };
}

/**
 * Create grouped price charts (price + ratio) - flat array, no z-scores
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {{ name: string, title: (metric: string) => string, list: readonly T[], getPrice: (tree: T['tree']) => ActivePricePattern, getRatio: (tree: T['tree']) => AnyMetricPattern }} args
 * @returns {PartialOptionsTree}
 */
export function createGroupedPriceCharts({ name, title, list, getPrice, getRatio }) {
  return [
    {
      name: "Price",
      title: title(name),
      top: list.map(({ color, name: cohortName, tree }) =>
        price({ metric: getPrice(tree), name: cohortName, color }),
      ),
    },
    {
      name: "Ratio",
      title: title(`${name} Ratio`),
      bottom: list.map(({ color, name: cohortName, tree }) =>
        baseline({ metric: getRatio(tree), name: cohortName, color, unit: Unit.ratio, base: 1 }),
      ),
    },
  ];
}

/**
 * Create grouped price folder (price + ratio wrapped in folder)
 * @template {{ color: Color, name: string, tree: { realized: AnyRealizedPattern } }} T
 * @param {{ name: string, title: (metric: string) => string, list: readonly T[], getPrice: (tree: T['tree']) => ActivePricePattern, getRatio: (tree: T['tree']) => AnyMetricPattern }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedPriceFolder({ name, title, list, getPrice, getRatio }) {
  return {
    name,
    tree: createGroupedPriceCharts({ name, title, list, getPrice, getRatio }),
  };
}

/**
 * Create base supply series (without relative metrics)
 * @param {PartialContext} ctx
 * @param {CohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeriesBase(ctx, cohort) {
  const { colors } = ctx;
  const { tree } = cohort;

  return [
    ...satsBtcUsd({ pattern: tree.supply.total, name: "Supply", color: colors.default }),
    ...satsBtcUsd({ pattern: tree.supply._30dChange, name: "30d Change", color: colors.orange }),
    ...satsBtcUsd({ pattern: tree.unrealized.supplyInProfit, name: "In Profit", color: colors.green }),
    ...satsBtcUsd({ pattern: tree.unrealized.supplyInLoss, name: "In Loss", color: colors.red }),
    ...satsBtcUsd({ pattern: tree.supply.halved, name: "half", color: colors.gray }).map((s) => ({
      ...s,
      options: { lineStyle: 4 },
    })),
  ];
}

/**
 * Create supply relative to own supply metrics
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject | AddressCohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplyRelativeToOwnMetrics(ctx, cohort) {
  const { colors } = ctx;
  const { tree } = cohort;

  return [
    line({
      metric: tree.relative.supplyInProfitRelToOwnSupply,
      name: "In Profit",
      color: colors.green,
      unit: Unit.pctOwn,
    }),
    line({
      metric: tree.relative.supplyInLossRelToOwnSupply,
      name: "In Loss",
      color: colors.red,
      unit: Unit.pctOwn,
    }),
    priceLine({
      ctx,
      unit: Unit.pctOwn,
      number: 100,
      style: 0,
      color: colors.default,
    }),
    priceLine({ ctx, unit: Unit.pctOwn, number: 50 }),
  ];
}

/**
 * Create supply section for a single cohort (with relative metrics)
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject | AddressCohortObject} cohort
 * @param {Object} [options]
 * @param {AnyFetchedSeriesBlueprint[]} [options.supplyRelative] - Supply relative to circulating supply metrics
 * @param {AnyFetchedSeriesBlueprint[]} [options.pnlRelative] - Supply in profit/loss relative to circulating supply metrics
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSupplySeries(ctx, cohort, { supplyRelative = [], pnlRelative = [] } = {}) {
  return [
    ...createSingleSupplySeriesBase(ctx, cohort),
    ...supplyRelative,
    ...pnlRelative,
    ...createSingleSupplyRelativeToOwnMetrics(ctx, cohort),
  ];
}

/**
 * Create supply series for cohorts WITHOUT relative metrics
 * @param {PartialContext} ctx
 * @param {CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSupplySeriesWithoutRelative(ctx, cohort) {
  return createSingleSupplySeriesBase(ctx, cohort);
}

/**
 * Create supply total series for grouped cohorts
 * @template {readonly CohortObject[]} T
 * @param {T} list
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.relativeMetrics] - Generator for relative supply metrics
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyTotalSeries(list, { relativeMetrics } = {}) {
  return list.flatMap((cohort) => [
    ...satsBtcUsd({ pattern: cohort.tree.supply.total, name: cohort.name, color: cohort.color }),
    ...(relativeMetrics ? relativeMetrics(cohort) : []),
  ]);
}

/**
 * Create supply in profit series for grouped cohorts
 * @template {readonly CohortObject[]} T
 * @param {T} list
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.relativeMetrics] - Generator for relative supply metrics
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInProfitSeries(list, { relativeMetrics } = {}) {
  return list.flatMap((cohort) => [
    ...satsBtcUsd({ pattern: cohort.tree.unrealized.supplyInProfit, name: cohort.name, color: cohort.color }),
    ...(relativeMetrics ? relativeMetrics(cohort) : []),
  ]);
}

/**
 * Create supply in loss series for grouped cohorts
 * @template {readonly CohortObject[]} T
 * @param {T} list
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.relativeMetrics] - Generator for relative supply metrics
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInLossSeries(list, { relativeMetrics } = {}) {
  return list.flatMap((cohort) => [
    ...satsBtcUsd({ pattern: cohort.tree.unrealized.supplyInLoss, name: cohort.name, color: cohort.color }),
    ...(relativeMetrics ? relativeMetrics(cohort) : []),
  ]);
}

/**
 * Create supply section for grouped cohorts
 * @template {readonly (CohortObject | CohortWithoutRelative)[]} T
 * @param {T} list
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.supplyRelativeMetrics] - Generator for supply relative metrics
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.profitRelativeMetrics] - Generator for supply in profit relative metrics
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.lossRelativeMetrics] - Generator for supply in loss relative metrics
 * @returns {PartialOptionsGroup}
 */
export function createGroupedSupplySection(list, title, { supplyRelativeMetrics, profitRelativeMetrics, lossRelativeMetrics } = {}) {
  return {
    name: "Supply",
    tree: [
      {
        name: "Total",
        title: title("Supply"),
        bottom: createGroupedSupplyTotalSeries(list, { relativeMetrics: supplyRelativeMetrics }),
      },
      {
        name: "30d Change",
        title: title("Supply 30d Change"),
        bottom: list.flatMap(({ color, name, tree }) =>
          satsBtcUsd({ pattern: tree.supply._30dChange, name, color }),
        ),
      },
      {
        name: "In Profit",
        title: title("Supply In Profit"),
        bottom: createGroupedSupplyInProfitSeries(list, { relativeMetrics: profitRelativeMetrics }),
      },
      {
        name: "In Loss",
        title: title("Supply In Loss"),
        bottom: createGroupedSupplyInLossSeries(list, { relativeMetrics: lossRelativeMetrics }),
      },
    ],
  };
}

// ============================================================================
// Circulating Supply Relative Metrics Generators
// ============================================================================

/**
 * Create supply relative to circulating supply series for single cohort
 * @param {PartialContext} ctx
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSupplyRelativeToCirculatingSeries(ctx, cohort) {
  return [
    line({
      metric: cohort.tree.relative.supplyRelToCirculatingSupply,
      name: "Supply",
      color: ctx.colors.default,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Create supply in profit/loss relative to circulating supply series for single cohort
 * @param {PartialContext} ctx
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSupplyPnlRelativeToCirculatingSeries(ctx, cohort) {
  return [
    line({
      metric: cohort.tree.relative.supplyInProfitRelToCirculatingSupply,
      name: "In Profit",
      color: ctx.colors.green,
      unit: Unit.pctSupply,
    }),
    line({
      metric: cohort.tree.relative.supplyInLossRelToCirculatingSupply,
      name: "In Loss",
      color: ctx.colors.red,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Create supply relative to circulating supply metrics generator for grouped cohorts
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyRelativeMetrics(cohort) {
  return [
    line({
      metric: cohort.tree.relative.supplyRelToCirculatingSupply,
      name: cohort.name,
      color: cohort.color,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Create supply in profit relative to circulating supply metrics generator for grouped cohorts
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInProfitRelativeMetrics(cohort) {
  return [
    line({
      metric: cohort.tree.relative.supplyInProfitRelToCirculatingSupply,
      name: cohort.name,
      color: cohort.color,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Create supply in loss relative to circulating supply metrics generator for grouped cohorts
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInLossRelativeMetrics(cohort) {
  return [
    line({
      metric: cohort.tree.relative.supplyInLossRelToCirculatingSupply,
      name: cohort.name,
      color: cohort.color,
      unit: Unit.pctSupply,
    }),
  ];
}

/**
 * Grouped supply relative generators object for cohorts with circulating supply relative
 * @type {{ supplyRelativeMetrics: typeof createGroupedSupplyRelativeMetrics, profitRelativeMetrics: typeof createGroupedSupplyInProfitRelativeMetrics, lossRelativeMetrics: typeof createGroupedSupplyInLossRelativeMetrics }}
 */
export const groupedSupplyRelativeGenerators = {
  supplyRelativeMetrics: createGroupedSupplyRelativeMetrics,
  profitRelativeMetrics: createGroupedSupplyInProfitRelativeMetrics,
  lossRelativeMetrics: createGroupedSupplyInLossRelativeMetrics,
};

/**
 * Create single cohort supply relative options for cohorts with circulating supply relative
 * @param {PartialContext} ctx
 * @param {CohortWithCirculatingSupplyRelative} cohort
 * @returns {{ supplyRelative: AnyFetchedSeriesBlueprint[], pnlRelative: AnyFetchedSeriesBlueprint[] }}
 */
export function createSingleSupplyRelativeOptions(ctx, cohort) {
  return {
    supplyRelative: createSupplyRelativeToCirculatingSeries(ctx, cohort),
    pnlRelative: createSupplyPnlRelativeToCirculatingSeries(ctx, cohort),
  };
}

/**
 * Create UTXO count series
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createUtxoCountSeries(list, useGroupName) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.outputs.utxoCount,
      name: useGroupName ? name : "Count",
      color,
      unit: Unit.count,
    }),
  ]);
}

/**
 * Create address count series (for address cohorts only)
 * @param {PartialContext} ctx
 * @param {readonly AddressCohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createAddressCountSeries(ctx, list, useGroupName) {
  const { colors } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.addrCount,
      name: useGroupName ? name : "Count",
      color: useGroupName ? color : colors.orange,
      unit: Unit.count,
    }),
  ]);
}

/**
 * Create realized price series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
export function createRealizedPriceSeries(list) {
  return list.map(({ color, name, tree }) =>
    price({ metric: tree.realized.realizedPrice, name, color }),
  );
}

/**
 * Create realized price ratio series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedPriceRatioSeries(list) {
  return list.map(({ name, tree }) =>
    baseline({
      metric: tree.realized.realizedPriceExtra.ratio,
      name,
      unit: Unit.ratio,
      base: 1,
    }),
  );
}

/**
 * Create realized capitalization series
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedCapSeries(list, useGroupName) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.realized.realizedCap,
      name: useGroupName ? name : "Capitalization",
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create cost basis percentile series (only for cohorts with CostBasisPattern2)
 * Includes min (p0) and max (p100) with full rainbow coloring
 * @param {Colors} colors
 * @param {readonly CohortWithCostBasisPercentiles[]} list
 * @param {boolean} useGroupName
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
export function createCostBasisPercentilesSeries(colors, list, useGroupName) {
  return list.flatMap(({ name, tree }) => {
    const cb = tree.costBasis;
    const p = cb.percentiles;
    const n = (/** @type {number} */ pct) => (useGroupName ? `${name} p${pct}` : `p${pct}`);
    return [
      price({ metric: cb.max, name: n(100), color: colors.purple, defaultActive: false }),
      price({ metric: p.pct95, name: n(95), color: colors.fuchsia, defaultActive: false }),
      price({ metric: p.pct90, name: n(90), color: colors.pink, defaultActive: false }),
      price({ metric: p.pct85, name: n(85), color: colors.pink, defaultActive: false }),
      price({ metric: p.pct80, name: n(80), color: colors.rose, defaultActive: false }),
      price({ metric: p.pct75, name: n(75), color: colors.red, defaultActive: false }),
      price({ metric: p.pct70, name: n(70), color: colors.orange, defaultActive: false }),
      price({ metric: p.pct65, name: n(65), color: colors.amber, defaultActive: false }),
      price({ metric: p.pct60, name: n(60), color: colors.yellow, defaultActive: false }),
      price({ metric: p.pct55, name: n(55), color: colors.yellow, defaultActive: false }),
      price({ metric: p.pct50, name: n(50), color: colors.avocado }),
      price({ metric: p.pct45, name: n(45), color: colors.lime, defaultActive: false }),
      price({ metric: p.pct40, name: n(40), color: colors.green, defaultActive: false }),
      price({ metric: p.pct35, name: n(35), color: colors.emerald, defaultActive: false }),
      price({ metric: p.pct30, name: n(30), color: colors.teal, defaultActive: false }),
      price({ metric: p.pct25, name: n(25), color: colors.teal, defaultActive: false }),
      price({ metric: p.pct20, name: n(20), color: colors.cyan, defaultActive: false }),
      price({ metric: p.pct15, name: n(15), color: colors.sky, defaultActive: false }),
      price({ metric: p.pct10, name: n(10), color: colors.blue, defaultActive: false }),
      price({ metric: p.pct05, name: n(5), color: colors.indigo, defaultActive: false }),
      price({ metric: cb.min, name: n(0), color: colors.violet, defaultActive: false }),
    ];
  });
}

/**
 * Create invested capital percentile series (only for cohorts with CostBasisPattern2)
 * Shows invested capital at each percentile level
 * @param {Colors} colors
 * @param {readonly CohortWithCostBasisPercentiles[]} list
 * @param {boolean} useGroupName
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
export function createInvestedCapitalPercentilesSeries(colors, list, useGroupName) {
  return list.flatMap(({ name, tree }) => {
    const ic = tree.costBasis.investedCapital;
    const n = (/** @type {number} */ pct) => (useGroupName ? `${name} p${pct}` : `p${pct}`);
    return [
      price({ metric: ic.pct95, name: n(95), color: colors.fuchsia, defaultActive: false }),
      price({ metric: ic.pct90, name: n(90), color: colors.pink, defaultActive: false }),
      price({ metric: ic.pct85, name: n(85), color: colors.pink, defaultActive: false }),
      price({ metric: ic.pct80, name: n(80), color: colors.rose, defaultActive: false }),
      price({ metric: ic.pct75, name: n(75), color: colors.red, defaultActive: false }),
      price({ metric: ic.pct70, name: n(70), color: colors.orange, defaultActive: false }),
      price({ metric: ic.pct65, name: n(65), color: colors.amber, defaultActive: false }),
      price({ metric: ic.pct60, name: n(60), color: colors.yellow, defaultActive: false }),
      price({ metric: ic.pct55, name: n(55), color: colors.yellow, defaultActive: false }),
      price({ metric: ic.pct50, name: n(50), color: colors.avocado }),
      price({ metric: ic.pct45, name: n(45), color: colors.lime, defaultActive: false }),
      price({ metric: ic.pct40, name: n(40), color: colors.green, defaultActive: false }),
      price({ metric: ic.pct35, name: n(35), color: colors.emerald, defaultActive: false }),
      price({ metric: ic.pct30, name: n(30), color: colors.teal, defaultActive: false }),
      price({ metric: ic.pct25, name: n(25), color: colors.teal, defaultActive: false }),
      price({ metric: ic.pct20, name: n(20), color: colors.cyan, defaultActive: false }),
      price({ metric: ic.pct15, name: n(15), color: colors.sky, defaultActive: false }),
      price({ metric: ic.pct10, name: n(10), color: colors.blue, defaultActive: false }),
      price({ metric: ic.pct05, name: n(5), color: colors.indigo, defaultActive: false }),
    ];
  });
}

/**
 * Create spot percentile series (shows current percentile of price relative to cost basis/invested capital)
 * @param {Colors} colors
 * @param {readonly CohortWithCostBasisPercentiles[]} list
 * @param {boolean} useGroupName
 * @returns {FetchedBaselineSeriesBlueprint[]}
 */
export function createSpotPercentileSeries(colors, list, useGroupName) {
  return list.flatMap(({ name, color, tree }) => [
    baseline({
      metric: tree.costBasis.spotCostBasisPercentile,
      name: useGroupName ? `${name} Cost Basis` : "Cost Basis",
      color: useGroupName ? color : colors.default,
      unit: Unit.ratio,
    }),
    baseline({
      metric: tree.costBasis.spotInvestedCapitalPercentile,
      name: useGroupName ? `${name} Invested Capital` : "Invested Capital",
      color: useGroupName ? color : colors.orange,
      unit: Unit.ratio,
      defaultActive: false,
    }),
  ]);
}

// ============================================================================
// Activity Section Helpers
// ============================================================================

/**
 * Create coins destroyed series (coinblocks, coindays, satblocks, satdays) for single cohort
 * All metrics on one chart
 * @param {CohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleCoinsDestroyedSeries(cohort) {
  const { tree, color } = cohort;
  return [
    line({
      metric: tree.activity.coinblocksDestroyed.sum,
      name: "Coinblocks",
      color,
      unit: Unit.coinblocks,
    }),
    line({
      metric: tree.activity.coinblocksDestroyed.cumulative,
      name: "Coinblocks Cumulative",
      color,
      unit: Unit.coinblocks,
      defaultActive: false,
    }),
    line({
      metric: tree.activity.coindaysDestroyed.sum,
      name: "Coindays",
      color,
      unit: Unit.coindays,
    }),
    line({
      metric: tree.activity.coindaysDestroyed.cumulative,
      name: "Coindays Cumulative",
      color,
      unit: Unit.coindays,
      defaultActive: false,
    }),
  ];
}

/**
 * Create coinblocks destroyed series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedCoinblocksDestroyedSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.coinblocksDestroyed.sum,
      name,
      color,
      unit: Unit.coinblocks,
    }),
  ]);
}

/**
 * Create coindays destroyed series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedCoindaysDestroyedSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.coindaysDestroyed.sum,
      name,
      color,
      unit: Unit.coindays,
    }),
  ]);
}

/**
 * Create sent series (sats, btc, usd) for single cohort - all on one chart
 * @param {CohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSentSeries(cohort) {
  const { tree, color } = cohort;
  return [
    line({
      metric: tree.activity.sent.sats.sum,
      name: "Sent",
      color,
      unit: Unit.sats,
    }),
    line({
      metric: tree.activity.sent.sats.cumulative,
      name: "Cumulative",
      color,
      unit: Unit.sats,
      defaultActive: false,
    }),
    line({
      metric: tree.activity.sent.bitcoin.sum,
      name: "Sent",
      color,
      unit: Unit.btc,
    }),
    line({
      metric: tree.activity.sent.bitcoin.cumulative,
      name: "Cumulative",
      color,
      unit: Unit.btc,
      defaultActive: false,
    }),
    line({
      metric: tree.activity.sent.dollars.sum,
      name: "Sent",
      color,
      unit: Unit.usd,
    }),
    line({
      metric: tree.activity.sent.dollars.cumulative,
      name: "Cumulative",
      color,
      unit: Unit.usd,
      defaultActive: false,
    }),
    ...satsBtcUsd({ pattern: tree.activity.sent14dEma, name: "14d EMA" }),
  ];
}

// ============================================================================
// Sell Side Risk Ratio Helpers
// ============================================================================

/**
 * Create sell side risk ratio series for single cohort
 * @param {Colors} colors
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSellSideRiskSeries(colors, tree) {
  return [
    dots({
      metric: tree.realized.sellSideRiskRatio,
      name: "Raw",
      color: colors.orange,
      unit: Unit.ratio,
    }),
    line({
      metric: tree.realized.sellSideRiskRatio7dEma,
      name: "7d EMA",
      color: colors.red,
      unit: Unit.ratio,
    }),
    line({
      metric: tree.realized.sellSideRiskRatio30dEma,
      name: "30d EMA",
      color: colors.pink,
      unit: Unit.ratio,
    }),
  ];
}

/**
 * Create sell side risk ratio series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSellSideRiskSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.realized.sellSideRiskRatio,
      name,
      color,
      unit: Unit.ratio,
    }),
  ]);
}

// ============================================================================
// Value Created & Destroyed Helpers
// ============================================================================

/**
 * Create value created & destroyed series for single cohort
 * @param {Colors} colors
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleValueCreatedDestroyedSeries(colors, tree) {
  return [
    line({
      metric: tree.realized.valueCreated,
      name: "Created",
      color: colors.emerald,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.valueDestroyed,
      name: "Destroyed",
      color: colors.red,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create profit/loss value breakdown series for single cohort
 * Shows profit value created/destroyed and loss value created/destroyed
 * @param {Colors} colors
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleValueFlowBreakdownSeries(colors, tree) {
  return [
    line({
      metric: tree.realized.profitValueCreated,
      name: "Profit Created",
      color: colors.green,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.profitValueDestroyed,
      name: "Profit Destroyed",
      color: colors.lime,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.realized.lossValueCreated,
      name: "Loss Created",
      color: colors.orange,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.realized.lossValueDestroyed,
      name: "Loss Destroyed",
      color: colors.red,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create capitulation & profit flow series for single cohort
 * @param {Colors} colors
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleCapitulationProfitFlowSeries(colors, tree) {
  return [
    line({
      metric: tree.realized.profitFlow,
      name: "Profit Flow",
      color: colors.green,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.capitulationFlow,
      name: "Capitulation Flow",
      color: colors.red,
      unit: Unit.usd,
    }),
  ];
}

// ============================================================================
// SOPR Helpers
// ============================================================================

/**
 * Create base SOPR series for single cohort (all cohorts have base SOPR)
 * @param {Colors} colors
 * @param {{ realized: AnyRealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSoprSeries(colors, tree) {
  return [
    baseline({
      metric: tree.realized.sopr,
      name: "SOPR",
      unit: Unit.ratio,
      base: 1,
    }),
    baseline({
      metric: tree.realized.sopr7dEma,
      name: "7d EMA",
      color: [colors.lime, colors.rose],
      unit: Unit.ratio,
      defaultActive: false,
      base: 1,
    }),
    baseline({
      metric: tree.realized.sopr30dEma,
      name: "30d EMA",
      color: [colors.avocado, colors.pink],
      unit: Unit.ratio,
      defaultActive: false,
      base: 1,
    }),
  ];
}

// ============================================================================
// Investor Price Helpers
// ============================================================================

/**
 * Create investor price series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {Color} color
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
export function createSingleInvestorPriceSeries(tree, color) {
  return [
    price({
      metric: tree.realized.investorPrice,
      name: "Investor",
      color,
    }),
  ];
}

/**
 * Create investor price ratio series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleInvestorPriceRatioSeries(tree, color) {
  return [
    baseline({
      metric: tree.realized.investorPriceExtra.ratio,
      name: "Investor Ratio",
      color,
      unit: Unit.ratio,
      base: 1,
    }),
  ];
}

/**
 * Create investor price series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
export function createInvestorPriceSeries(list) {
  return list.map(({ color, name, tree }) =>
    price({ metric: tree.realized.investorPrice, name, color }),
  );
}

/**
 * Create investor price ratio series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createInvestorPriceRatioSeries(list) {
  return list.map(({ name, tree }) =>
    baseline({
      metric: tree.realized.investorPriceExtra.ratio,
      name,
      unit: Unit.ratio,
      base: 1,
    }),
  );
}

/**
 * Create investor price folder for extended cohorts (with full Z-scores)
 * For cohorts with ActivePriceRatioPattern (all, term.*, ageRange.* UTXO cohorts)
 * @param {PartialContext} ctx
 * @param {{ tree: { realized: RealizedWithExtras }, color: Color }} cohort
 * @param {string} [cohortTitle] - Cohort title (e.g., "STH")
 * @returns {PartialOptionsGroup}
 */
export function createInvestorPriceFolderFull(ctx, cohort, cohortTitle) {
  const { tree, color } = cohort;
  return createPriceFolder(ctx, {
    name: "Investor Price",
    cohortTitle,
    priceMetric: tree.realized.investorPrice,
    ratioPattern: tree.realized.investorPriceExtra,
    color,
  });
}

/**
 * Create investor price folder for basic cohorts (price + ratio only)
 * For cohorts with InvestorPriceExtraPattern (only .ratio field)
 * @param {{ tree: { realized: AnyRealizedPattern }, color: Color }} cohort
 * @param {string} [cohortTitle] - Cohort title (e.g., "STH")
 * @returns {PartialOptionsGroup}
 */
export function createInvestorPriceFolderBasic(cohort, cohortTitle) {
  const { tree, color } = cohort;
  return createBasicPriceFolder({
    name: "Investor Price",
    cohortTitle,
    priceMetric: tree.realized.investorPrice,
    ratioMetric: tree.realized.investorPriceExtra.ratio,
    color,
  });
}

/**
 * Create investor price folder for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
export function createGroupedInvestorPriceFolder(list, title) {
  return createGroupedPriceFolder({
    name: "Investor Price",
    title,
    list,
    getPrice: (tree) => tree.realized.investorPrice,
    getRatio: (tree) => tree.realized.investorPriceExtra.ratio,
  });
}

// ============================================================================
// Peak Regret Helpers
// ============================================================================

/**
 * Create realized peak regret series for single cohort
 * @param {{ realized: AnyRealizedPattern }} tree
 * @param {Color} color
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleRealizedAthRegretSeries(tree, color) {
  return [
    line({
      metric: tree.realized.peakRegret.sum,
      name: "Peak Regret",
      color,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.peakRegret.cumulative,
      name: "Cumulative",
      color,
      unit: Unit.usd,
      defaultActive: false,
    }),
    baseline({
      metric: tree.realized.peakRegretRelToRealizedCap,
      name: "Rel. to Realized Cap",
      color,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Create realized ATH regret series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedRealizedAthRegretSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.realized.peakRegret.sum,
      name,
      color,
      unit: Unit.usd,
    }),
    baseline({
      metric: tree.realized.peakRegretRelToRealizedCap,
      name,
      color,
      unit: Unit.pctRcap,
    }),
  ]);
}

// ============================================================================
// Sentiment Helpers (greedIndex, painIndex, netSentiment)
// ============================================================================

/**
 * Create sentiment series for single cohort
 * @param {Colors} colors
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSentimentSeries(colors, tree) {
  return [
    baseline({
      metric: tree.unrealized.netSentiment,
      name: "Net Sentiment",
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.greedIndex,
      name: "Greed Index",
      color: colors.green,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.painIndex,
      name: "Pain Index",
      color: colors.red,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create net sentiment series for grouped cohorts
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedNetSentimentSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    baseline({
      metric: tree.unrealized.netSentiment,
      name,
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create greed index series for grouped cohorts
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedGreedIndexSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.unrealized.greedIndex,
      name,
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create pain index series for grouped cohorts
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedPainIndexSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.unrealized.painIndex,
      name,
      color,
      unit: Unit.usd,
    }),
  ]);
}
