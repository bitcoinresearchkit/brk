/**
 * Prices section builders
 *
 * Structure (single cohort):
 * - Compare: Both prices on one chart
 * - Realized: Price + Ratio (MVRV) + Z-Scores (for full cohorts)
 * - Investor: Price + Ratio + Z-Scores (for full cohorts)
 *
 * Structure (grouped cohorts):
 * - Realized: Price + Ratio comparison across cohorts
 * - Investor: Price + Ratio comparison across cohorts
 *
 * For cohorts WITHOUT full ratio patterns: basic Price/Ratio charts only (no Z-Scores)
 */

import { colors } from "../../utils/colors.js";
import { createPriceRatioCharts, mapCohortsWithAll } from "../shared.js";
import { baseline, price } from "../series.js";
import { Unit } from "../../utils/units.js";

/**
 * @param {{ realized: { realizedPrice: ActivePricePattern, investorPrice: ActivePricePattern } }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createCompareChart(tree, title) {
  return {
    name: "Compare",
    title: title("Prices"),
    top: [
      price({ metric: tree.realized.realizedPrice, name: "Realized", color: colors.realized }),
      price({ metric: tree.realized.investorPrice, name: "Investor", color: colors.investor }),
    ],
  };
}

/**
 * Create prices section for cohorts with full ActivePriceRatioPattern
 * (CohortAll, CohortFull, CohortWithPercentiles)
 * @param {{ cohort: CohortAll | CohortFull | CohortWithPercentiles, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createPricesSectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Prices",
    tree: [
      createCompareChart(tree, title),
      {
        name: "Realized",
        tree: createPriceRatioCharts({
          context: cohort.name,
          legend: "Realized",
          pricePattern: tree.realized.realizedPrice,
          ratio: tree.realized.realizedPriceExtra,
          color,
          priceTitle: title("Realized Price"),
          titlePrefix: "Realized Price",
        }),
      },
      {
        name: "Investor",
        tree: createPriceRatioCharts({
          context: cohort.name,
          legend: "Investor",
          pricePattern: tree.realized.investorPrice,
          ratio: tree.realized.investorPriceExtra,
          color,
          priceTitle: title("Investor Price"),
          titlePrefix: "Investor Price",
        }),
      },
    ],
  };
}

/**
 * Create prices section for cohorts with basic ratio patterns only
 * (CohortWithAdjusted, CohortBasic, CohortAddress, CohortWithoutRelative)
 * @param {{ cohort: CohortWithAdjusted | CohortBasic | CohortAddress | CohortWithoutRelative, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createPricesSectionBasic({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Prices",
    tree: [
      createCompareChart(tree, title),
      {
        name: "Realized",
        tree: [
          {
            name: "Price",
            title: title("Realized Price"),
            top: [
              price({
                metric: tree.realized.realizedPrice,
                name: "Realized",
                color,
              }),
            ],
          },
          {
            name: "Ratio",
            title: title("Realized Price Ratio"),
            bottom: [
              baseline({
                metric: tree.realized.realizedPriceExtra.ratio,
                name: "Ratio",
                unit: Unit.ratio,
                base: 1,
              }),
            ],
          },
        ],
      },
      {
        name: "Investor",
        tree: [
          {
            name: "Price",
            title: title("Investor Price"),
            top: [
              price({
                metric: tree.realized.investorPrice,
                name: "Investor",
                color,
              }),
            ],
          },
          {
            name: "Ratio",
            title: title("Investor Price Ratio"),
            bottom: [
              baseline({
                metric: tree.realized.investorPriceExtra.ratio,
                name: "Ratio",
                unit: Unit.ratio,
                base: 1,
              }),
            ],
          },
        ],
      },
    ],
  };
}

/**
 * Create prices section for grouped cohorts
 * @param {{ list: readonly CohortObject[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedPricesSection({ list, all, title }) {
  return {
    name: "Prices",
    tree: [
      {
        name: "Realized",
        tree: [
          {
            name: "Price",
            title: title("Realized Price"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.realized.realizedPrice, name, color }),
            ),
          },
          {
            name: "Ratio",
            title: title("Realized Price Ratio"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
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
      },
      {
        name: "Investor",
        tree: [
          {
            name: "Price",
            title: title("Investor Price"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.realized.investorPrice, name, color }),
            ),
          },
          {
            name: "Ratio",
            title: title("Investor Price Ratio"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                metric: tree.realized.investorPriceExtra.ratio,
                name,
                color,
                unit: Unit.ratio,
                base: 1,
              }),
            ),
          },
        ],
      },
    ],
  };
}
