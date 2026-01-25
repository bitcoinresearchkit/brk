/** Shared cohort chart section builders */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { baseline, line } from "../series.js";
import { satsBtcUsd } from "../shared.js";

/**
 * Create supply section for a single cohort
 * @param {PartialContext} ctx
 * @param {CohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSupplySeries(ctx, cohort) {
  const { colors } = ctx;
  const { tree } = cohort;

  return [
    ...satsBtcUsd(tree.supply.total, "Supply", colors.default),
    ...("supplyRelToCirculatingSupply" in tree.relative
      ? [
          line({
            metric: tree.relative.supplyRelToCirculatingSupply,
            name: "Supply",
            color: colors.default,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
    ...satsBtcUsd(tree.unrealized.supplyInProfit, "In Profit", colors.green),
    ...satsBtcUsd(tree.unrealized.supplyInLoss, "In Loss", colors.red),
    ...satsBtcUsd(tree.supply.halved, "half", colors.gray).map((s) => ({
      ...s,
      options: { lineStyle: 4 },
    })),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [
          line({
            metric: tree.relative.supplyInProfitRelToCirculatingSupply,
            name: "In Profit",
            color: colors.green,
            unit: Unit.pctSupply,
          }),
          line({
            metric: tree.relative.supplyInLossRelToCirculatingSupply,
            name: "In Loss",
            color: colors.red,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
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
 * Create supply total series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyTotalSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    ...satsBtcUsd(tree.supply.total, name, color),
    ...("supplyRelToCirculatingSupply" in tree.relative
      ? [
          line({
            metric: tree.relative.supplyRelToCirculatingSupply,
            name,
            color,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
  ]);
}

/**
 * Create supply in profit series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInProfitSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    ...satsBtcUsd(tree.unrealized.supplyInProfit, name, color),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [
          line({
            metric: tree.relative.supplyInProfitRelToCirculatingSupply,
            name,
            color,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
  ]);
}

/**
 * Create supply in loss series for grouped cohorts
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInLossSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    ...satsBtcUsd(tree.unrealized.supplyInLoss, name, color),
    ...("supplyInLossRelToCirculatingSupply" in tree.relative
      ? [
          line({
            metric: tree.relative.supplyInLossRelToCirculatingSupply,
            name,
            color,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
  ]);
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
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedPriceSeries(list) {
  return list.map(({ color, name, tree }) =>
    line({ metric: tree.realized.realizedPrice, name, color, unit: Unit.usd }),
  );
}

/**
 * Create realized price ratio series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedPriceRatioSeries(ctx, list) {
  return [
    ...list.map(({ name, tree }) =>
      baseline({
        metric: tree.realized.realizedPriceExtra.ratio,
        name,
        unit: Unit.ratio,
        base: 1,
      }),
    ),
    priceLine({ ctx, unit: Unit.ratio, number: 1 }),
  ];
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
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createCostBasisPercentilesSeries(colors, list, useGroupName) {
  return list.flatMap(({ name, tree }) => {
    const cb = tree.costBasis;
    const p = cb.percentiles;
    const n = (/** @type {number} */ pct) => (useGroupName ? `${name} p${pct}` : `p${pct}`);
    return [
      line({ metric: cb.max, name: n(100), color: colors.purple, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct95, name: n(95), color: colors.fuchsia, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct90, name: n(90), color: colors.pink, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct85, name: n(85), color: colors.pink, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct80, name: n(80), color: colors.rose, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct75, name: n(75), color: colors.red, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct70, name: n(70), color: colors.orange, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct65, name: n(65), color: colors.amber, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct60, name: n(60), color: colors.yellow, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct55, name: n(55), color: colors.yellow, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct50, name: n(50), color: colors.avocado, unit: Unit.usd }),
      line({ metric: p.pct45, name: n(45), color: colors.lime, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct40, name: n(40), color: colors.green, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct35, name: n(35), color: colors.emerald, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct30, name: n(30), color: colors.teal, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct25, name: n(25), color: colors.teal, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct20, name: n(20), color: colors.cyan, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct15, name: n(15), color: colors.sky, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct10, name: n(10), color: colors.blue, unit: Unit.usd, defaultActive: false }),
      line({ metric: p.pct05, name: n(5), color: colors.indigo, unit: Unit.usd, defaultActive: false }),
      line({ metric: cb.min, name: n(0), color: colors.violet, unit: Unit.usd, defaultActive: false }),
    ];
  });
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
    line({
      metric: tree.activity.satblocksDestroyed,
      name: "Satblocks",
      color,
      unit: Unit.satblocks,
    }),
    line({
      metric: tree.activity.satdaysDestroyed,
      name: "Satdays",
      color,
      unit: Unit.satdays,
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
 * Create satblocks destroyed series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSatblocksDestroyedSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.satblocksDestroyed,
      name,
      color,
      unit: Unit.satblocks,
    }),
  ]);
}

/**
 * Create satdays destroyed series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSatdaysDestroyedSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.satdaysDestroyed,
      name,
      color,
      unit: Unit.satdays,
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
  ];
}

/**
 * Create sent (sats) series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSentSatsSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.sent.sats.sum,
      name,
      color,
      unit: Unit.sats,
    }),
  ]);
}

/**
 * Create sent (bitcoin) series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSentBitcoinSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.sent.bitcoin.sum,
      name,
      color,
      unit: Unit.btc,
    }),
  ]);
}

/**
 * Create sent (dollars) series for grouped cohorts (comparison)
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSentDollarsSeries(list) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.activity.sent.dollars.sum,
      name,
      color,
      unit: Unit.usd,
    }),
  ]);
}
