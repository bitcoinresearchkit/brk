/** Shared cohort chart section builders */

/**
 * Create supply section for a single cohort
 * @param {PartialContext} ctx
 * @param {CohortObject} cohort
 * @param {string} title
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createSingleSupplySeries(ctx, cohort, title) {
  const { colors, s, createPriceLine } = ctx;
  const { tree, color, name } = cohort;

  return [
    s({ metric: tree.supply.supply.sats, name: "Supply", color: colors.default }),
    s({ metric: tree.supply.supply.bitcoin, name: "Supply", color: colors.default }),
    s({ metric: tree.supply.supply.dollars, name: "Supply", color: colors.default }),
    ...("supplyRelToCirculatingSupply" in tree.relative
      ? [s({ metric: tree.relative.supplyRelToCirculatingSupply, name: "Supply", color: colors.default })]
      : []),
    s({ metric: tree.unrealized.supplyInProfit.sats, name: "In Profit", color: colors.green }),
    s({ metric: tree.unrealized.supplyInProfit.bitcoin, name: "In Profit", color: colors.green }),
    s({ metric: tree.unrealized.supplyInProfit.dollars, name: "In Profit", color: colors.green }),
    s({ metric: tree.unrealized.supplyInLoss.sats, name: "In Loss", color: colors.red }),
    s({ metric: tree.unrealized.supplyInLoss.bitcoin, name: "In Loss", color: colors.red }),
    s({ metric: tree.unrealized.supplyInLoss.dollars, name: "In Loss", color: colors.red }),
    s({ metric: tree.supply.supplyHalf.sats, name: "half", color: colors.gray, options: { lineStyle: 4 } }),
    s({ metric: tree.supply.supplyHalf.bitcoin, name: "half", color: colors.gray, options: { lineStyle: 4 } }),
    s({ metric: tree.supply.supplyHalf.dollars, name: "half", color: colors.gray, options: { lineStyle: 4 } }),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [
          s({ metric: tree.relative.supplyInProfitRelToCirculatingSupply, name: "In Profit", color: colors.green }),
          s({ metric: tree.relative.supplyInLossRelToCirculatingSupply, name: "In Loss", color: colors.red }),
        ]
      : []),
    s({ metric: tree.relative.supplyInProfitRelToOwnSupply, name: "In Profit", color: colors.green }),
    s({ metric: tree.relative.supplyInLossRelToOwnSupply, name: "In Loss", color: colors.red }),
    createPriceLine({ unit: "%self", number: 100, lineStyle: 0, color: colors.default }),
    createPriceLine({ unit: "%self", number: 50 }),
  ];
}

/**
 * Create supply total series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly CohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
export function createGroupedSupplyTotalSeries(ctx, list) {
  const { s, constant100 } = ctx;

  return list.flatMap(({ color, name, tree }) => [
    s({ metric: tree.supply.supply.sats, name, color }),
    s({ metric: tree.supply.supply.bitcoin, name, color }),
    s({ metric: tree.supply.supply.dollars, name, color }),
    "supplyRelToCirculatingSupply" in tree.relative
      ? s({ metric: tree.relative.supplyRelToCirculatingSupply, name, color })
      : s({ unit: "%all", metric: constant100, name, color }),
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
    s({ metric: tree.unrealized.supplyInProfit.sats, name, color }),
    s({ metric: tree.unrealized.supplyInProfit.bitcoin, name, color }),
    s({ metric: tree.unrealized.supplyInProfit.dollars, name, color }),
    ...("supplyInProfitRelToCirculatingSupply" in tree.relative
      ? [s({ metric: tree.relative.supplyInProfitRelToCirculatingSupply, name, color })]
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
    s({ metric: tree.unrealized.supplyInLoss.sats, name, color }),
    s({ metric: tree.unrealized.supplyInLoss.bitcoin, name, color }),
    s({ metric: tree.unrealized.supplyInLoss.dollars, name, color }),
    ...("supplyInLossRelToCirculatingSupply" in tree.relative
      ? [s({ metric: tree.relative.supplyInLossRelToCirculatingSupply, name, color })]
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
    s({ metric: tree.supply.utxoCount, name: useGroupName ? name : "Count", color }),
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
    s({ metric: tree.realized.realizedPrice, name, color }),
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
      s({ metric: tree.realized.realizedPriceExtra.ratio, name, color }),
    ),
    createPriceLine({ unit: "ratio", number: 1 }),
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
    s({ metric: tree.realized.realizedCap, name: useGroupName ? name : "Capitalization", color }),
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
    s({ metric: tree.costBasis.minCostBasis, name: useGroupName ? `${name} min` : "Min", color }),
    s({ metric: tree.costBasis.maxCostBasis, name: useGroupName ? `${name} max` : "Max", color }),
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
  const { s, colors } = ctx;

  return list.flatMap(({ color, name, tree }) => {
    const percentiles = tree.costBasis.percentiles;
    return [
      s({ metric: percentiles.pct10, name: useGroupName ? `${name} p10` : "p10", color, defaultActive: false }),
      s({ metric: percentiles.pct25, name: useGroupName ? `${name} p25` : "p25", color, defaultActive: false }),
      s({ metric: percentiles.pct50, name: useGroupName ? `${name} p50` : "p50", color }),
      s({ metric: percentiles.pct75, name: useGroupName ? `${name} p75` : "p75", color, defaultActive: false }),
      s({ metric: percentiles.pct90, name: useGroupName ? `${name} p90` : "p90", color, defaultActive: false }),
    ];
  });
}
