/**
 * Capitalization section builders
 */

import { Unit } from "../../utils/units.js";
import { colors } from "../../utils/colors.js";
import { ROLLING_WINDOWS, line, baseline, mapWindows, sumsTreeBaseline, rollingPercentRatioTree, percentRatio, percentRatioBaseline } from "../series.js";
import { ratioBottomSeries, mapCohortsWithAll, flatMapCohortsWithAll } from "../shared.js";

// ============================================================================
// Shared building blocks
// ============================================================================

/**
 * Single cohort: Change + Growth Rate items (flat)
 * @param {UtxoCohortObject["tree"]} tree
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function singleDeltaItems(tree, title) {
  return [
    { ...sumsTreeBaseline({ windows: mapWindows(tree.realized.cap.delta.absolute, (c) => c.usd), title, metric: "Realized Cap Change", unit: Unit.usd }), name: "Change" },
    { ...rollingPercentRatioTree({ windows: tree.realized.cap.delta.rate, title, metric: "Realized Cap Growth Rate" }), name: "Growth Rate" },
  ];
}

/**
 * Grouped: Change + Growth Rate + MVRV items (flat)
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {CohortAll} all
 * @param {(name: string) => string} title
 * @returns {PartialOptionsTree}
 */
function groupedDeltaAndMvrv(list, all, title) {
  return [
    {
      name: "MVRV",
      title: title("MVRV"),
      bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
        baseline({ series: tree.realized.mvrv, name, color, unit: Unit.ratio, base: 1 }),
      ),
    },
    {
      name: "Change",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`${w.title} Realized Cap Change`),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          baseline({ series: tree.realized.cap.delta.absolute[w.key].usd, name, color, unit: Unit.usd }),
        ),
      })),
    },
    {
      name: "Growth Rate",
      tree: ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: title(`${w.title} Realized Cap Growth Rate`),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
          percentRatioBaseline({ pattern: tree.realized.cap.delta.rate[w.key], name, color }),
        ),
      })),
    },
  ];
}

// ============================================================================
// Single Cohort Sections
// ============================================================================

/**
 * Full capitalization (has invested capital, own market cap ratio, full MVRV)
 * @param {{ cohort: CohortAll | CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Capitalization",
    tree: [
      { name: "Total", title: title("Realized Cap"), bottom: [line({ series: tree.realized.cap.usd, name: "Realized Cap", color, unit: Unit.usd })] },
      {
        name: "Profitability",
        title: title("Invested Capital"),
        bottom: [
          line({ series: tree.realized.cap.usd, name: "Total", color: colors.default, unit: Unit.usd }),
          line({ series: tree.unrealized.investedCapital.inProfit.usd, name: "In Profit", color: colors.profit, unit: Unit.usd }),
          line({ series: tree.unrealized.investedCapital.inLoss.usd, name: "In Loss", color: colors.loss, unit: Unit.usd }),
        ],
      },
      { name: "MVRV", title: title("MVRV"), bottom: ratioBottomSeries(tree.realized.price) },
      ...singleDeltaItems(tree, title),
      { name: "% of Own Market Cap", title: title("Realized Cap (% of Own Market Cap)"), bottom: percentRatioBaseline({ pattern: tree.realized.cap.toOwnMcap, name: "Rel. to Own Market Cap", color }) },
    ],
  };
}

/**
 * Basic capitalization (no invested capital, simple MVRV)
 * @param {{ cohort: CohortWithAdjusted | CohortBasic | CohortAddr | CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSection({ cohort, title }) {
  const { tree } = cohort;
  return {
    name: "Capitalization",
    tree: [
      { name: "Total", title: title("Realized Cap"), bottom: [line({ series: tree.realized.cap.usd, name: "Realized Cap", color: cohort.color, unit: Unit.usd })] },
      ...singleDeltaItems(tree, title),
      { name: "MVRV", title: title("MVRV"), bottom: [baseline({ series: tree.realized.mvrv, name: "MVRV", unit: Unit.ratio, base: 1 })] },
    ],
  };
}

// ============================================================================
// Grouped Cohort Sections
// ============================================================================

/**
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedValuationSection({ list, all, title }) {
  return {
    name: "Capitalization",
    tree: [
      {
        name: "Total",
        title: title("Realized Cap"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.realized.cap.usd, name, color, unit: Unit.usd }),
        ),
      },
      ...groupedDeltaAndMvrv(list, all, title),
    ],
  };
}

/**
 * @param {{ list: readonly (CohortAll | CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedValuationSectionWithOwnMarketCap({ list, all, title }) {
  return {
    name: "Capitalization",
    tree: [
      {
        name: "Total",
        title: title("Realized Cap"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.realized.cap.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "In Profit",
        title: title("Invested Capital In Profit"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.unrealized.investedCapital.inProfit.usd, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "In Loss",
        title: title("Invested Capital In Loss"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({ series: tree.unrealized.investedCapital.inLoss.usd, name, color, unit: Unit.usd }),
        ),
      },
      ...groupedDeltaAndMvrv(list, all, title),
      {
        name: "% of Own Market Cap",
        title: title("Realized Cap (% of Own Market Cap)"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
          percentRatio({ pattern: tree.realized.cap.toOwnMcap, name, color }),
        ),
      },
    ],
  };
}
