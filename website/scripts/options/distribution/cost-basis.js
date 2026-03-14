/**
 * Cost Basis section builders
 *
 * Structure:
 * - Summary: Key stats (avg + median active, quartiles/extremes available)
 * - By Coin: BTC-weighted percentiles (IQR active: p25, p50, p75)
 * - By Capital: USD-weighted percentiles (IQR active: p25, p50, p75)
 * - Supply Density: Cost basis supply density percentage
 *
 * Only for cohorts WITH costBasis (All, STH, LTH)
 */

import { colors } from "../../utils/colors.js";
import { entries } from "../../utils/array.js";
import { Unit } from "../../utils/units.js";
import { priceLines } from "../constants.js";
import { price, percentRatio } from "../series.js";
import { mapCohortsWithAll, flatMapCohortsWithAll } from "../shared.js";

const ACTIVE_PCTS = new Set(["pct75", "pct50", "pct25"]);

/**
 * @param {PercentilesPattern} p
 * @param {(name: string) => string} [n]
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createCorePercentileSeries(p, n = (x) => x) {
  return entries(p)
    .reverse()
    .map(([key, metric], i, arr) =>
      price({
        metric,
        name: n(key.replace("pct", "p")),
        color: colors.at(i, arr.length),
        ...(ACTIVE_PCTS.has(key) ? {} : { defaultActive: false }),
      }),
    );
}

/**
 * @param {CohortAll | CohortFull | CohortLongTerm} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleSummarySeries(cohort) {
  const { color, tree } = cohort;
  const p = tree.costBasis.percentiles;
  return [
    price({ metric: tree.realized.price, name: "Average", color }),
    price({
      metric: tree.costBasis.max,
      name: "Max (p100)",
      color: colors.stat.max,
      defaultActive: false,
    }),
    price({
      metric: p.pct75,
      name: "Q3 (p75)",
      color: colors.stat.pct75,
      defaultActive: false,
    }),
    price({ metric: p.pct50, name: "Median (p50)", color: colors.stat.median }),
    price({
      metric: p.pct25,
      name: "Q1 (p25)",
      color: colors.stat.pct25,
      defaultActive: false,
    }),
    price({
      metric: tree.costBasis.min,
      name: "Min (p0)",
      color: colors.stat.min,
      defaultActive: false,
    }),
  ];
}

/**
 * @param {readonly (CohortAll | CohortFull | CohortLongTerm)[]} list
 * @param {CohortAll} all
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createGroupedSummarySeries(list, all) {
  return mapCohortsWithAll(list, all, ({ name, color, tree }) =>
    price({ metric: tree.realized.price, name, color }),
  );
}

/**
 * @param {CohortAll | CohortFull | CohortLongTerm} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleByCoinSeries(cohort) {
  const { color, tree } = cohort;
  const cb = tree.costBasis;
  return [
    price({ metric: tree.realized.price, name: "Average", color }),
    price({
      metric: cb.max,
      name: "p100",
      color: colors.stat.max,
      defaultActive: false,
    }),
    ...createCorePercentileSeries(cb.percentiles),
    price({
      metric: cb.min,
      name: "p0",
      color: colors.stat.min,
      defaultActive: false,
    }),
  ];
}

/**
 * @param {CohortAll | CohortFull | CohortLongTerm} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleByCapitalSeries(cohort) {
  const { color, tree } = cohort;
  return [
    price({ metric: tree.realized.investor.price, name: "Average", color }),
    ...createCorePercentileSeries(tree.costBasis.investedCapital),
  ];
}

/**
 * @param {CohortAll | CohortFull | CohortLongTerm} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleSupplyDensitySeries(cohort) {
  const { tree } = cohort;
  return [
    ...percentRatio({
      pattern: tree.costBasis.supplyDensity,
      name: "Supply Density",
      color: colors.bitcoin,
    }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.percentage }),
  ];
}

/**
 * @param {{ cohort: CohortAll | CohortFull | CohortLongTerm, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createCostBasisSectionWithPercentiles({ cohort, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createSingleSummarySeries(cohort),
      },
      {
        name: "By Coin",
        title: title("Cost Basis Distribution (BTC-weighted)"),
        top: createSingleByCoinSeries(cohort),
      },
      {
        name: "By Capital",
        title: title("Cost Basis Distribution (USD-weighted)"),
        top: createSingleByCapitalSeries(cohort),
      },
      {
        name: "Supply Density",
        title: title("Cost Basis Supply Density"),
        bottom: createSingleSupplyDensitySeries(cohort),
      },
    ],
  };
}

/**
 * @param {{ list: readonly (CohortAll | CohortFull | CohortLongTerm)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCostBasisSectionWithPercentiles({
  list,
  all,
  title,
}) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createGroupedSummarySeries(list, all),
      },
      {
        name: "By Coin",
        tree: [
          {
            name: "Average",
            title: title("Realized Price Comparison"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.realized.price, name, color }),
            ),
          },
          {
            name: "Median",
            title: title("Cost Basis Median (BTC-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.costBasis.percentiles.pct50, name, color }),
            ),
          },
          {
            name: "Q3",
            title: title("Cost Basis Q3 (BTC-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.costBasis.percentiles.pct75, name, color }),
            ),
          },
          {
            name: "Q1",
            title: title("Cost Basis Q1 (BTC-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.costBasis.percentiles.pct25, name, color }),
            ),
          },
        ],
      },
      {
        name: "By Capital",
        tree: [
          {
            name: "Average",
            title: title("Investor Price Comparison"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({ metric: tree.realized.investor.price, name, color }),
            ),
          },
          {
            name: "Median",
            title: title("Cost Basis Median (USD-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({
                metric: tree.costBasis.investedCapital.pct50,
                name,
                color,
              }),
            ),
          },
          {
            name: "Q3",
            title: title("Cost Basis Q3 (USD-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({
                metric: tree.costBasis.investedCapital.pct75,
                name,
                color,
              }),
            ),
          },
          {
            name: "Q1",
            title: title("Cost Basis Q1 (USD-weighted)"),
            top: mapCohortsWithAll(list, all, ({ name, color, tree }) =>
              price({
                metric: tree.costBasis.investedCapital.pct25,
                name,
                color,
              }),
            ),
          },
        ],
      },
      {
        name: "Supply Density",
        title: title("Cost Basis Supply Density"),
        bottom: flatMapCohortsWithAll(list, all, ({ name, color, tree }) =>
          percentRatio({
            pattern: tree.costBasis.supplyDensity,
            name,
            color,
          }),
        ),
      },
    ],
  };
}
