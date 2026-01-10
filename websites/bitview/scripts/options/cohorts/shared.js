/** Shared cohort chart section builders */

import { Unit } from "../../utils/units.js";

/**
 * Create supply section for a single cohort
 * @param {PartialContext} ctx
 * @param {CohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSupplySeries(ctx, cohort) {
  const { colors, s, createPriceLine } = ctx;
  const { tree } = cohort;

  return [
    s({
      metric: tree.supply.supply.sats,
      name: "Supply",
      color: colors.default,
      unit: Unit.sats,
    }),
    s({
      metric: tree.supply.supply.bitcoin,
      name: "Supply",
      color: colors.default,
      unit: Unit.btc,
    }),
    s({
      metric: tree.supply.supply.dollars,
      name: "Supply",
      color: colors.default,
      unit: Unit.usd,
    }),
    ...("supplyRelToCirculatingSupply" in tree.relative
      ? [
          s({
            metric: tree.relative.supplyRelToCirculatingSupply,
            name: "Supply",
            color: colors.default,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
    s({
      metric: tree.unrealized.supplyInProfit.sats,
      name: "In Profit",
      color: colors.green,
      unit: Unit.sats,
    }),
    s({
      metric: tree.unrealized.supplyInProfit.bitcoin,
      name: "In Profit",
      color: colors.green,
      unit: Unit.btc,
    }),
    s({
      metric: tree.unrealized.supplyInProfit.dollars,
      name: "In Profit",
      color: colors.green,
      unit: Unit.usd,
    }),
    s({
      metric: tree.unrealized.supplyInLoss.sats,
      name: "In Loss",
      color: colors.red,
      unit: Unit.sats,
    }),
    s({
      metric: tree.unrealized.supplyInLoss.bitcoin,
      name: "In Loss",
      color: colors.red,
      unit: Unit.btc,
    }),
    s({
      metric: tree.unrealized.supplyInLoss.dollars,
      name: "In Loss",
      color: colors.red,
      unit: Unit.usd,
    }),
    s({
      metric: tree.supply.supplyHalf.sats,
      name: "half",
      color: colors.gray,
      unit: Unit.sats,
      options: { lineStyle: 4 },
    }),
    s({
      metric: tree.supply.supplyHalf.bitcoin,
      name: "half",
      color: colors.gray,
      unit: Unit.btc,
      options: { lineStyle: 4 },
    }),
    s({
      metric: tree.supply.supplyHalf.dollars,
      name: "half",
      color: colors.gray,
      unit: Unit.usd,
      options: { lineStyle: 4 },
    }),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [
          s({
            metric: tree.relative.supplyInProfitRelToCirculatingSupply,
            name: "In Profit",
            color: colors.green,
            unit: Unit.pctSupply,
          }),
          s({
            metric: tree.relative.supplyInLossRelToCirculatingSupply,
            name: "In Loss",
            color: colors.red,
            unit: Unit.pctSupply,
          }),
        ]
      : []),
    s({
      metric: tree.relative.supplyInProfitRelToOwnSupply,
      name: "In Profit",
      color: colors.green,
      unit: Unit.pctOwn,
    }),
    s({
      metric: tree.relative.supplyInLossRelToOwnSupply,
      name: "In Loss",
      color: colors.red,
      unit: Unit.pctOwn,
    }),
    createPriceLine({
      unit: Unit.pctOwn,
      number: 100,
      lineStyle: 0,
      color: colors.default,
    }),
    createPriceLine({ unit: Unit.pctOwn, number: 50 }),
  ];
}

/**
 * Create supply total series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyTotalSeries(ctx, list) {
  const { s, brk } = ctx;
  const constant100 = brk.tree.constants.constant100;

  return list.flatMap(({ color, name, tree }) => [
    s({ metric: tree.supply.supply.sats, name, color, unit: Unit.sats }),
    s({ metric: tree.supply.supply.bitcoin, name, color, unit: Unit.btc }),
    s({ metric: tree.supply.supply.dollars, name, color, unit: Unit.usd }),
    "supplyRelToCirculatingSupply" in tree.relative
      ? s({
          metric: tree.relative.supplyRelToCirculatingSupply,
          name,
          color,
          unit: Unit.pctSupply,
        })
      : s({ metric: constant100, name, color, unit: Unit.pctSupply }),
  ]);
}

/**
 * Create supply in profit series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInProfitSeries(ctx, list) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.unrealized.supplyInProfit.sats,
      name,
      color,
      unit: Unit.sats,
    }),
    s({
      metric: tree.unrealized.supplyInProfit.bitcoin,
      name,
      color,
      unit: Unit.btc,
    }),
    s({
      metric: tree.unrealized.supplyInProfit.dollars,
      name,
      color,
      unit: Unit.usd,
    }),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [
          s({
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
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyInLossSeries(ctx, list) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.unrealized.supplyInLoss.sats,
      name,
      color,
      unit: Unit.sats,
    }),
    s({
      metric: tree.unrealized.supplyInLoss.bitcoin,
      name,
      color,
      unit: Unit.btc,
    }),
    s({
      metric: tree.unrealized.supplyInLoss.dollars,
      name,
      color,
      unit: Unit.usd,
    }),
    ...("supplyInLossRelToCirculatingSupply" in tree.relative
      ? [
          s({
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
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createUtxoCountSeries(ctx, list, useGroupName) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.supply.utxoCount,
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
  const { s, colors } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.addrCount,
      name: useGroupName ? name : "Count",
      color: useGroupName ? color : colors.orange,
      unit: Unit.count,
    }),
  ]);
}

/**
 * Create realized price series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedPriceSeries(ctx, list) {
  const { s } = ctx;

  return list.map(({ color, name, tree }) =>
    s({ metric: tree.realized.realizedPrice, name, color, unit: Unit.usd }),
  );
}

/**
 * Create realized price ratio series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedPriceRatioSeries(ctx, list) {
  const { s, createPriceLine } = ctx;

  return [
    ...list.map(({ color, name, tree }) =>
      s({
        metric: tree.realized.realizedPriceExtra.ratio,
        name,
        color,
        unit: Unit.ratio,
      }),
    ),
    createPriceLine({ unit: Unit.ratio, number: 1 }),
  ];
}

/**
 * Create realized capitalization series
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createRealizedCapSeries(ctx, list, useGroupName) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.realized.realizedCap,
      name: useGroupName ? name : "Capitalization",
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create cost basis min/max series (available on all cohorts)
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createCostBasisMinMaxSeries(ctx, list, useGroupName) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({
      metric: tree.costBasis.minCostBasis,
      name: useGroupName ? `${name} min` : "Min",
      color,
      unit: Unit.usd,
    }),
    s({
      metric: tree.costBasis.maxCostBasis,
      name: useGroupName ? `${name} max` : "Max",
      color,
      unit: Unit.usd,
    }),
  ]);
}

/**
 * Create cost basis percentile series (only for cohorts with CostBasisPattern2)
 * @param {PartialContext} ctx
 * @param {readonly CohortWithCostBasisPercentiles[]} list
 * @param {boolean} useGroupName
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createCostBasisPercentilesSeries(ctx, list, useGroupName) {
  const { s } = ctx;

  return list.flatMap(({ color, name, tree }) => {
    const percentiles = tree.costBasis.percentiles;
    return [
      s({
        metric: percentiles.costBasisPct10,
        name: useGroupName ? `${name} p10` : "p10",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      s({
        metric: percentiles.costBasisPct25,
        name: useGroupName ? `${name} p25` : "p25",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      s({
        metric: percentiles.costBasisPct50,
        name: useGroupName ? `${name} p50` : "p50",
        color,
        unit: Unit.usd,
      }),
      s({
        metric: percentiles.costBasisPct75,
        name: useGroupName ? `${name} p75` : "p75",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
      s({
        metric: percentiles.costBasisPct90,
        name: useGroupName ? `${name} p90` : "p90",
        color,
        unit: Unit.usd,
        defaultActive: false,
      }),
    ];
  });
}
