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
import { ROLLING_WINDOWS, line, baseline, mapWindows, sumsTree, rollingPercentRatioTree, percentRatio, percentRatioBaseline } from "../series.js";
import { createRatioChart, mapCohortsWithAll, flatMapCohortsWithAll } from "../shared.js";

/**
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRealizedCapSeries(cohort) {
  const { color, tree } = cohort;
  return [
    line({
      series: tree.realized.cap.usd,
      name: "Realized Cap",
      color,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Create valuation section for cohorts with full ratio patterns
 * (CohortAll, CohortFull, CohortWithPercentiles)
 * @param {{ cohort: CohortAll | CohortFull | CohortLongTerm, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSectionFull({ cohort, title }) {
  const { tree, color } = cohort;
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        tree: [
          {
            name: "USD",
            title: title("Realized Cap"),
            bottom: createSingleRealizedCapSeries(cohort),
          },
          {
            name: "% of Own Mcap",
            title: title("Realized Cap (% of Own Mcap)"),
            bottom: percentRatioBaseline({ pattern: tree.realized.cap.toOwnMcap, name: "Rel. to Own M.Cap", color }),
          },
        ],
      },
      {
        name: "Change",
        tree: [
          { ...sumsTree({ windows: mapWindows(tree.realized.cap.delta.absolute, (c) => c.usd), title: title("Realized Cap Change"), unit: Unit.usd, series: baseline }), name: "Absolute" },
          { ...rollingPercentRatioTree({ windows: tree.realized.cap.delta.rate, title: title("Realized Cap Rate") }), name: "Rate" },
        ],
      },
      createRatioChart({
        title,
        pricePattern: tree.realized.price,
        ratio: tree.realized.price,
        color,
        name: "MVRV",
      }),
    ],
  };
}

/**
 * Create valuation section for cohorts with basic ratio patterns
 * (CohortWithAdjusted, CohortBasic, CohortAddr, CohortWithoutRelative)
 * @param {{ cohort: CohortWithAdjusted | CohortBasic | CohortAddr | CohortWithoutRelative, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createValuationSection({ cohort, title }) {
  const { tree } = cohort;
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort),
      },
      {
        name: "Change",
        tree: [
          { ...sumsTree({ windows: mapWindows(tree.realized.cap.delta.absolute, (c) => c.usd), title: title("Realized Cap Change"), unit: Unit.usd, series: baseline }), name: "Absolute" },
          { ...rollingPercentRatioTree({ windows: tree.realized.cap.delta.rate, title: title("Realized Cap Rate") }), name: "Rate" },
        ],
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
    ],
  };
}

/**
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedValuationSection({ list, all, title }) {
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        title: title("Realized Cap"),
        bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
          line({
            series: tree.realized.cap.usd,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Change",
        tree: [
          {
            name: "Absolute",
            tree: ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Realized Cap Change (${w.name})`),
              bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                baseline({ series: tree.realized.cap.delta.absolute[w.key].usd, name, color, unit: Unit.usd }),
              ),
            })),
          },
          {
            name: "Rate",
            tree: ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Realized Cap Rate (${w.name})`),
              bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.realized.cap.delta.rate[w.key], name, color }),
              ),
            })),
          },
        ],
      },
      {
        name: "MVRV",
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
  };
}

/**
 * @param {{ list: readonly (CohortAll | CohortFull | CohortLongTerm)[], all: CohortAll, title: (name: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedValuationSectionWithOwnMarketCap({
  list,
  all,
  title,
}) {
  return {
    name: "Valuation",
    tree: [
      {
        name: "Realized Cap",
        tree: [
          {
            name: "USD",
            title: title("Realized Cap"),
            bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              line({ series: tree.realized.cap.usd, name, color, unit: Unit.usd }),
            ),
          },
          {
            name: "% of Own Mcap",
            title: title("Realized Cap (% of Own Mcap)"),
            bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
              percentRatio({ pattern: tree.realized.cap.toOwnMcap, name, color }),
            ),
          },
        ],
      },
      {
        name: "Change",
        tree: [
          {
            name: "Absolute",
            tree: ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Realized Cap Change (${w.name})`),
              bottom: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                baseline({ series: tree.realized.cap.delta.absolute[w.key].usd, name, color, unit: Unit.usd }),
              ),
            })),
          },
          {
            name: "Rate",
            tree: ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: title(`Realized Cap Rate (${w.name})`),
              bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
                percentRatio({ pattern: tree.realized.cap.delta.rate[w.key], name, color }),
              ),
            })),
          },
        ],
      },
      {
        name: "MVRV",
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
  };
}
