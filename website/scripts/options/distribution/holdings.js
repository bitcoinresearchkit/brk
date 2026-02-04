/**
 * Holdings section builders
 *
 * Structure (Option C - optimized for UX):
 * - Supply: Total BTC held (flat, one click)
 * - UTXO Count: Number of UTXOs (flat, one click)
 * - Address Count: Number of addresses (when available, flat)
 * - 30d Changes/: Folder for change metrics
 *   - Supply: 30d supply change
 * - Relative: % of circulating supply (when available)
 *
 * Rationale: Most-used metrics (Supply, UTXO Count) are immediately accessible.
 * 30d changes are grouped together for consistency and cleaner navigation.
 */

import { Unit } from "../../utils/units.js";
import { line, baseline } from "../series.js";
import { satsBtcUsd, satsBtcUsdBaseline } from "../shared.js";
import { colors } from "../../utils/colors.js";
import { priceLines } from "../constants.js";

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeries(cohort) {
  const { tree } = cohort;
  return [
    ...satsBtcUsd({
      pattern: tree.supply.total,
      name: "Total",
      color: colors.default,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInProfit,
      name: "In Profit",
      color: colors.profit,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInLoss,
      name: "In Loss",
      color: colors.loss,
    }),
    // Halved supply (sparse line)
    ...satsBtcUsd({
      pattern: tree.supply.halved,
      name: "Halved",
      color: colors.gray,
      style: 4,
    }),
  ];
}

/**
 * Supply series for CohortAll (has % of Own Supply but not % of Circulating)
 * @param {CohortAll} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeriesAll(cohort) {
  const { tree } = cohort;
  return [
    ...satsBtcUsd({
      pattern: tree.supply.total,
      name: "Total",
      color: colors.default,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInProfit,
      name: "In Profit",
      color: colors.profit,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInLoss,
      name: "In Loss",
      color: colors.loss,
    }),
    // Halved supply (sparse line)
    ...satsBtcUsd({
      pattern: tree.supply.halved,
      name: "Halved",
      color: colors.gray,
      style: 4,
    }),
    // % of Own Supply
    line({
      metric: tree.relative.supplyInProfitRelToOwnSupply,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctOwn,
    }),
    line({
      metric: tree.relative.supplyInLossRelToOwnSupply,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwn,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwn,
    }),
  ];
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingle30dChangeSeries(cohort) {
  return satsBtcUsdBaseline({
    pattern: cohort.tree.supply._30dChange,
    name: "30d Change",
  });
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleUtxoCountSeries(cohort) {
  const { color, tree } = cohort;
  return [
    line({
      metric: tree.outputs.utxoCount,
      name: "UTXO Count",
      color,
      unit: Unit.count,
    }),
  ];
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleUtxoCount30dChangeSeries(cohort) {
  return [
    baseline({
      metric: cohort.tree.outputs.utxoCount30dChange,
      name: "30d Change",
      unit: Unit.count,
    }),
  ];
}

/**
 * @param {CohortAll | CohortAddress} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleAddrCount30dChangeSeries(cohort) {
  return [
    baseline({
      metric: cohort.addrCount._30dChange,
      name: "30d Change",
      unit: Unit.count,
    }),
  ];
}

/**
 * Create supply series with % of Circulating (for cohorts with relative data)
 * @param {CohortFull | CohortWithAdjusted | CohortBasicWithMarketCap | CohortMinAge} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeriesWithRelative(cohort) {
  const { tree } = cohort;
  return [
    ...satsBtcUsd({
      pattern: tree.supply.total,
      name: "Total",
      color: colors.default,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInProfit,
      name: "In Profit",
      color: colors.profit,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInLoss,
      name: "In Loss",
      color: colors.loss,
    }),
    // Halved supply (sparse line)
    ...satsBtcUsd({
      pattern: tree.supply.halved,
      name: "Halved",
      color: colors.gray,
      style: 4,
    }),
    // % of Circulating Supply
    line({
      metric: tree.relative.supplyRelToCirculatingSupply,
      name: "Total",
      color: colors.default,
      unit: Unit.pctSupply,
    }),
    line({
      metric: tree.relative.supplyInProfitRelToCirculatingSupply,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctSupply,
    }),
    line({
      metric: tree.relative.supplyInLossRelToCirculatingSupply,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctSupply,
    }),
    // % of Own Supply
    line({
      metric: tree.relative.supplyInProfitRelToOwnSupply,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctOwn,
    }),
    line({
      metric: tree.relative.supplyInLossRelToOwnSupply,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwn,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwn,
    }),
  ];
}

/**
 * Supply series with % of Own Supply only (for cohorts without % of Circulating)
 * @param {CohortAgeRange | CohortBasicWithoutMarketCap} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeriesWithOwnSupply(cohort) {
  const { tree } = cohort;
  return [
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInProfit,
      name: "In Profit",
      color: colors.profit,
    }),
    ...satsBtcUsd({
      pattern: tree.unrealized.supplyInLoss,
      name: "In Loss",
      color: colors.loss,
    }),
    ...satsBtcUsd({
      pattern: tree.supply.total,
      name: "Total",
      color: colors.default,
    }),
    // Halved supply (sparse line)
    ...satsBtcUsd({
      pattern: tree.supply.halved,
      name: "Halved",
      color: colors.gray,
      style: 4,
    }),
    // % of Own Supply
    line({
      metric: tree.relative.supplyInProfitRelToOwnSupply,
      name: "In Profit",
      color: colors.profit,
      unit: Unit.pctOwn,
    }),
    line({
      metric: tree.relative.supplyInLossRelToOwnSupply,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwn,
    }),
    ...priceLines({
      numbers: [100, 50, 0],
      unit: Unit.pctOwn,
    }),
  ];
}

/**
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSection({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeries(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * Holdings section with % of Own Supply only (for cohorts without % of Circulating)
 * @param {{ cohort: CohortAgeRange | CohortBasicWithoutMarketCap, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithOwnSupply({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeriesWithOwnSupply(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * @param {{ cohort: CohortFull | CohortWithAdjusted | CohortBasicWithMarketCap | CohortMinAge, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionWithRelative({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeriesWithRelative(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * @param {{ cohort: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAll({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeriesAll(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: [
          line({
            metric: cohort.addrCount.count,
            name: "Address Count",
            color: cohort.color,
            unit: Unit.count,
          }),
        ],
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: createSingleAddrCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * @param {{ cohort: CohortAddress, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAddress({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeriesWithOwnSupply(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: [
          line({
            metric: cohort.addrCount.count,
            name: "Address Count",
            color: cohort.color,
            unit: Unit.count,
          }),
        ],
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: createSingleAddrCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * Holdings section for address amount cohorts (has relative supply + address count)
 * @param {{ cohort: AddressCohortObject, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createHoldingsSectionAddressAmount({ cohort, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        title: title("Supply"),
        bottom: createSingleSupplySeriesWithRelative(cohort),
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: createSingleUtxoCountSeries(cohort),
      },
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: [
          line({
            metric: cohort.addrCount.count,
            name: "Address Count",
            color: cohort.color,
            unit: Unit.count,
          }),
        ],
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: createSingle30dChangeSeries(cohort),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: createSingleUtxoCount30dChangeSeries(cohort),
          },
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: createSingleAddrCount30dChangeSeries(cohort),
          },
        ],
      },
    ],
  };
}

/**
 * @param {{ list: readonly CohortAddress[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionAddress({ list, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInProfit,
                  name,
                  color,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInLoss,
                  name,
                  color,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
        ],
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.outputs.utxoCount,
            name,
            color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: list.map(({ name, color, addrCount }) =>
          line({ metric: addrCount.count, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsdBaseline({
                pattern: tree.supply._30dChange,
                name,
                color,
              }),
            ),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: list.map(({ name, color, tree }) =>
              baseline({
                metric: tree.outputs.utxoCount30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: list.map(({ name, color, addrCount }) =>
              baseline({
                metric: addrCount._30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped holdings section for address amount cohorts (has relative supply + address count)
 * @param {{ list: readonly AddressCohortObject[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionAddressAmount({ list, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({ pattern: tree.supply.total, name, color }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInProfit,
                  name,
                  color,
                }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInLoss,
                  name,
                  color,
                }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
        ],
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.outputs.utxoCount,
            name,
            color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "Address Count",
        title: title("Address Count"),
        bottom: list.map(({ name, color, addrCount }) =>
          line({ metric: addrCount.count, name, color, unit: Unit.count }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsdBaseline({
                pattern: tree.supply._30dChange,
                name,
                color,
              }),
            ),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: list.map(({ name, color, tree }) =>
              baseline({
                metric: tree.outputs.utxoCount30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
          {
            name: "Address Count",
            title: title("Address Count 30d Change"),
            bottom: list.map(({ name, color, addrCount }) =>
              baseline({
                metric: addrCount._30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {{ list: T, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSection({ list, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.unrealized.supplyInProfit,
                name,
                color,
              }),
            ),
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsd({
                pattern: tree.unrealized.supplyInLoss,
                name,
                color,
              }),
            ),
          },
        ],
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.outputs.utxoCount,
            name,
            color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsdBaseline({
                pattern: tree.supply._30dChange,
                name,
                color,
              }),
            ),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: list.map(({ name, color, tree }) =>
              baseline({
                metric: tree.outputs.utxoCount30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * Grouped holdings section with % of Own Supply only (for cohorts without % of Circulating)
 * @param {{ list: readonly (CohortAgeRange | CohortBasicWithoutMarketCap)[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionWithOwnSupply({ list, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInProfit,
                  name,
                  color,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInLoss,
                  name,
                  color,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
          {
            name: "Total",
            title: title("Supply"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsd({ pattern: tree.supply.total, name, color }),
            ),
          },
        ],
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.outputs.utxoCount,
            name,
            color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsdBaseline({
                pattern: tree.supply._30dChange,
                name,
                color,
              }),
            ),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: list.map(({ name, color, tree }) =>
              baseline({
                metric: tree.outputs.utxoCount30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}

/**
 * @param {{ list: readonly (CohortFull | CohortWithAdjusted | CohortBasicWithMarketCap | CohortMinAge)[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedHoldingsSectionWithRelative({ list, title }) {
  return {
    name: "Holdings",
    tree: [
      {
        name: "Supply",
        tree: [
          {
            name: "Total",
            title: title("Supply"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({ pattern: tree.supply.total, name, color }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
            ],
          },
          {
            name: "In Profit",
            title: title("Supply In Profit"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInProfit,
                  name,
                  color,
                }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInProfitRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
          {
            name: "In Loss",
            title: title("Supply In Loss"),
            bottom: [
              ...list.flatMap(({ name, color, tree }) =>
                satsBtcUsd({
                  pattern: tree.unrealized.supplyInLoss,
                  name,
                  color,
                }),
              ),
              // % of Circulating
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToCirculatingSupply,
                  name,
                  color,
                  unit: Unit.pctSupply,
                }),
              ),
              // % of Own Supply
              ...list.map(({ name, color, tree }) =>
                line({
                  metric: tree.relative.supplyInLossRelToOwnSupply,
                  name,
                  color,
                  unit: Unit.pctOwn,
                }),
              ),
              ...priceLines({
                numbers: [100, 50, 0],
                unit: Unit.pctOwn,
              }),
            ],
          },
        ],
      },
      {
        name: "UTXO Count",
        title: title("UTXO Count"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.outputs.utxoCount,
            name,
            color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "30d Changes",
        tree: [
          {
            name: "Supply",
            title: title("Supply 30d Change"),
            bottom: list.flatMap(({ name, color, tree }) =>
              satsBtcUsdBaseline({
                pattern: tree.supply._30dChange,
                name,
                color,
              }),
            ),
          },
          {
            name: "UTXO Count",
            title: title("UTXO Count 30d Change"),
            bottom: list.map(({ name, color, tree }) =>
              baseline({
                metric: tree.outputs.utxoCount30dChange,
                name,
                unit: Unit.count,
                color,
              }),
            ),
          },
        ],
      },
    ],
  };
}
