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
import { createPriceRatioCharts, mapCohortsWithAll, priceRatioPercentilesTree } from "../shared.js";
import { baseline, price } from "../series.js";
import { Unit } from "../../utils/units.js";

/**
 * Create prices section for cohorts with full ratio patterns
 * (CohortAll, CohortFull, CohortLongTerm)
 * @param {{ cohort: CohortAll | CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createPricesSectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Prices",
    tree: [
      {
        name: "Compare",
        title: title("Prices"),
        top: [
          price({ series: tree.realized.price, name: "Realized", color: colors.realized }),
          price({ series: tree.realized.investor.price, name: "Investor", color: colors.investor }),
        ],
      },
      {
        name: "Realized",
        tree: createPriceRatioCharts({
          context: cohort.name,
          legend: "Realized",
          pricePattern: tree.realized.price,
          ratio: tree.realized.price,
          color,
          priceTitle: title("Realized Price"),
          titlePrefix: "Realized Price",
        }),
      },
      {
        name: "Investor",
        tree: priceRatioPercentilesTree({
          pattern: tree.realized.investor.price,
          title: title("Investor Price"),
          legend: "Investor",
          color,
        }),
      },
    ],
  };
}

/**
 * Create prices section for cohorts with basic ratio patterns only
 * (CohortWithAdjusted, CohortBasic, CohortAddr, CohortWithoutRelative)
 * @param {{ cohort: CohortWithAdjusted | CohortBasic | CohortAddr | CohortWithoutRelative | CohortAgeRange, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createPricesSectionBasic({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Prices",
    tree: [
      {
        name: "Realized",
        tree: [
          {
            name: "Price",
            title: title("Realized Price"),
            top: [price({ series: tree.realized.price, name: "Realized", color })],
          },
          {
            name: "MVRV",
            title: title("MVRV"),
            bottom: [
              baseline({
                series: tree.realized.mvrv,
                name: "MVRV",
                unit: Unit.ratio,
                base: 1,
              }),
            ],
          },
          {
            name: "Price Ratio",
            title: title("Realized Price Ratio"),
            bottom: [
              baseline({
                series: tree.realized.price.ratio,
                name: "Price Ratio",
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
 * @param {{ list: readonly CohortObject[], all: CohortAll, title: (name: string) => string }} args
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
              price({ series: tree.realized.price, name, color }),
            ),
          },
          {
            name: "Ratio",
            title: title("MVRV"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              baseline({
                series: tree.realized.mvrv,
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
