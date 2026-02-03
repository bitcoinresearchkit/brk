/**
 * Valuation section builders
 *
 * Structure:
 * - Realized Cap: Total value at cost basis (USD)
 * - 30d Change: Recent realized cap changes
 * - MVRV: Market Value to Realized Value ratio
 *
 * For cohorts WITH full ratio patterns: MVRV uses createRatioChart (price + percentiles)
 * For cohorts WITHOUT full ratio patterns: MVRV is simple baseline
 */

import { Unit } from "../../utils/units.js";
import { line, baseline } from "../series.js";
import { createRatioChart } from "../shared.js";

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRealizedCapSeries(cohort) {
  const { color, tree } = cohort;
  return [
    line({
      metric: tree.realized.realizedCap,
      name: "Realized Cap",
      color,
      unit: Unit.usd,
    }),
  ];
}

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingle30dChangeSeries(cohort) {
  return [
    baseline({
      metric: cohort.tree.realized.realizedCap30dDelta,
      name: "30d Change",
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create valuation section for cohorts with full ratio patterns
 * (CohortAll, CohortFull, CohortWithPercentiles)
 * @param {{ cohort: CohortAll | CohortFull | CohortWithPercentiles, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort),
      },
      {
        name: "30d Change",
        title: title("Realized Cap 30d Change"),
        bottom: createSingle30dChangeSeries(cohort),
      },
      createRatioChart({
        title,
        pricePattern: tree.realized.realizedPrice,
        ratio: tree.realized.realizedPriceExtra,
        color,
        name: "MVRV",
      }),
    ],
  };
}

/**
 * Create valuation section for cohorts with basic ratio patterns
 * (CohortWithAdjusted, CohortBasic, CohortAddress, CohortWithoutRelative)
 * @param {{ cohort: CohortWithAdjusted | CohortBasic | CohortAddress | CohortWithoutRelative, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSection({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort),
      },
      {
        name: "30d Change",
        title: title("Realized Cap 30d Change"),
        bottom: createSingle30dChangeSeries(cohort),
      },
      {
        name: "MVRV",
        title: title("MVRV"),
        bottom: [
          baseline({
            metric: tree.realized.realizedPriceExtra.ratio,
            name: "MVRV",
            color,
            unit: Unit.ratio,
            base: 1,
          }),
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
export function createGroupedValuationSection({ list, title }) {
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        title: title("Realized Cap"),
        bottom: list.map(({ name, color, tree }) =>
          line({
            metric: tree.realized.realizedCap,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "30d Change",
        title: title("Realized Cap 30d Change"),
        bottom: list.map(({ name, color, tree }) =>
          baseline({
            metric: tree.realized.realizedCap30dDelta,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "MVRV",
        title: title("MVRV"),
        bottom: list.map(({ name, color, tree }) =>
          baseline({
            metric: tree.realized.realizedPriceExtra.ratio,
            name,
            color,
            unit: Unit.ratio,
            base: 1,
          }),
        ),
      },
    ],
  };
}
