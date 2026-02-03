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

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplySeries(cohort) {
  const { color, tree } = cohort;
  return [...satsBtcUsd({ pattern: tree.supply.total, name: "Supply", color })];
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingle30dChangeSeries(cohort) {
  return satsBtcUsdBaseline({ pattern: cohort.tree.supply._30dChange, name: "30d Change" });
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
 * @param {CohortFull | CohortWithAdjusted | CohortBasicWithMarketCap | CohortMinAge} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRelativeSeries(cohort) {
  const { color, tree } = cohort;
  return [
    line({
      metric: tree.relative.supplyRelToCirculatingSupply,
      name: "% of Circulating",
      color,
      unit: Unit.pctSupply,
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
      {
        name: "Relative",
        title: title("Relative to Circulating Supply"),
        bottom: createSingleRelativeSeries(cohort),
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
        bottom: createSingleSupplySeries(cohort),
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
        bottom: createSingleSupplySeries(cohort),
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
        title: title("Supply"),
        bottom: list.flatMap(({ name, color, tree }) =>
          satsBtcUsd({ pattern: tree.supply.total, name, color }),
        ),
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
              satsBtcUsdBaseline({ pattern: tree.supply._30dChange, name, color }),
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
        title: title("Supply"),
        bottom: list.flatMap(({ name, color, tree }) =>
          satsBtcUsd({ pattern: tree.supply.total, name, color }),
        ),
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
              satsBtcUsdBaseline({ pattern: tree.supply._30dChange, name, color }),
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
        title: title("Supply"),
        bottom: list.flatMap(({ name, color, tree }) =>
          satsBtcUsd({ pattern: tree.supply.total, name, color }),
        ),
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
              satsBtcUsdBaseline({ pattern: tree.supply._30dChange, name, color }),
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
      {
        name: "Relative",
        title: title("Relative to Circulating Supply"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.relative.supplyRelToCirculatingSupply,
            name,
            color,
            unit: Unit.pctSupply,
          }),
        ),
      },
    ],
  };
}
