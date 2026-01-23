/** Shared cohort chart section builders */

import { Unit } from "../../utils/units.js";
import { priceLine } from "../constants.js";
import { line } from "../series.js";
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
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyTotalSeries(ctx, list) {
  const { brk } = ctx;
  const constant100 = brk.metrics.constants.constant100;

  return list.flatMap(({ color, name, tree }) => [
    ...satsBtcUsd(tree.supply.total, name, color),
    line({
      metric:
        "supplyRelToCirculatingSupply" in tree.relative
          ? tree.relative.supplyRelToCirculatingSupply
          : constant100,
      name,
      color,
      unit: Unit.pctSupply,
    }),
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
    ...list.map(({ color, name, tree }) =>
      line({
        metric: tree.realized.realizedPriceExtra.ratio,
        name,
        color,
        unit: Unit.ratio,
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
 * Create cost basis min/max series (available on all cohorts)
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createCostBasisMinMaxSeries(list, useGroupName) {
  return list.flatMap(({ color, name, tree }) => [
    line({
      metric: tree.costBasis.min,
      name: useGroupName ? `${name} min` : "Min",
      color,
      unit: Unit.usd,
    }),
    line({
      metric: tree.costBasis.max,
      name: useGroupName ? `${name} max` : "Max",
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create cost basis percentile series (only for cohorts with CostBasisPattern2)
 * @param {readonly CohortWithCostBasisPercentiles[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createCostBasisPercentilesSeries(list, useGroupName) {
  return list.flatMap(({ color, name, tree }) => {
    const percentiles = tree.costBasis.percentiles;
    return [
      line({
        metric: percentiles.pct10,
        name: useGroupName ? `${name} p10` : "p10",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      line({
        metric: percentiles.pct25,
        name: useGroupName ? `${name} p25` : "p25",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      line({
        metric: percentiles.pct50,
        name: useGroupName ? `${name} p50` : "p50",
        color,
        unit: Unit.usd,
      }),
      line({
        metric: percentiles.pct75,
        name: useGroupName ? `${name} p75` : "p75",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      line({
        metric: percentiles.pct90,
        name: useGroupName ? `${name} p90` : "p90",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
    ];
  });
}
