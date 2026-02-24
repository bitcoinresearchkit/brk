/**
 * Cost Basis section builders
 *
 * Structure:
 * - Summary: Key stats (avg + median active, quartiles/extremes available)
 * - By Coin: BTC-weighted percentiles (IQR active: p25, p50, p75)
 * - By Capital: USD-weighted percentiles (IQR active: p25, p50, p75)
 * - Price Position: Spot percentile (both perspectives active)
 *
 * For cohorts WITHOUT percentiles: Summary only
 */

import { colors } from "../../utils/colors.js";
import { entries } from "../../utils/array.js";
import { Unit } from "../../utils/units.js";
import { priceLines } from "../constants.js";
import { line, price } from "../series.js";
import { mapCohortsWithAll } from "../shared.js";

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
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleSummarySeriesBasic(cohort) {
  const { color, tree } = cohort;
  return [
    price({ metric: tree.realized.realizedPrice, name: "Average", color }),
    price({
      metric: tree.costBasis.max,
      name: "Max",
      color: colors.stat.max,
      defaultActive: false,
    }),
    price({
      metric: tree.costBasis.min,
      name: "Min",
      color: colors.stat.min,
      defaultActive: false,
    }),
  ];
}

/**
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleSummarySeriesWithPercentiles(cohort) {
  const { color, tree } = cohort;
  const p = tree.costBasis.percentiles;
  return [
    price({ metric: tree.realized.realizedPrice, name: "Average", color }),
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
 * @param {readonly CohortObject[]} list
 * @param {CohortAll} all
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createGroupedSummarySeries(list, all) {
  return mapCohortsWithAll(list, all, ({ name, color, tree }) =>
    price({ metric: tree.realized.realizedPrice, name, color }),
  );
}

/**
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleByCoinSeries(cohort) {
  const { color, tree } = cohort;
  const cb = tree.costBasis;
  return [
    price({ metric: tree.realized.realizedPrice, name: "Average", color }),
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
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createSingleByCapitalSeries(cohort) {
  const { color, tree } = cohort;
  return [
    price({ metric: tree.realized.investorPrice, name: "Average", color }),
    ...createCorePercentileSeries(tree.costBasis.investedCapital),
  ];
}

/**
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSinglePricePositionSeries(cohort) {
  const { tree } = cohort;
  return [
    line({
      metric: tree.costBasis.spotCostBasisPercentile,
      name: "By Coin",
      color: colors.bitcoin,
      unit: Unit.percentage,
    }),
    line({
      metric: tree.costBasis.spotInvestedCapitalPercentile,
      name: "By Capital",
      color: colors.usd,
      unit: Unit.percentage,
    }),
    ...priceLines({ numbers: [100, 50, 0], unit: Unit.percentage }),
  ];
}

/**
 * @param {{ cohort: UtxoCohortObject | CohortWithoutRelative, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createCostBasisSection({ cohort, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createSingleSummarySeriesBasic(cohort),
      },
    ],
  };
}

/**
 * @param {{ cohort: CohortAll | CohortFull | CohortWithPercentiles, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createCostBasisSectionWithPercentiles({ cohort, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createSingleSummarySeriesWithPercentiles(cohort),
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
        name: "Price Position",
        title: title("Current Price Position"),
        bottom: createSinglePricePositionSeries(cohort),
      },
    ],
  };
}

/**
 * @param {{ list: readonly (UtxoCohortObject | CohortWithoutRelative)[], all: CohortAll, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCostBasisSection({ list, all, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createGroupedSummarySeries(list, all),
      },
    ],
  };
}

/**
 * @param {{ list: readonly (CohortAll | CohortFull | CohortWithPercentiles)[], all: CohortAll, title: (metric: string) => string }} args
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
              price({ metric: tree.realized.realizedPrice, name, color }),
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
              price({ metric: tree.realized.investorPrice, name, color }),
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
        name: "Price Position",
        tree: [
          {
            name: "By Coin",
            title: title("Price Position (BTC-weighted)"),
            bottom: [
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.costBasis.spotCostBasisPercentile,
                  name,
                  color,
                  unit: Unit.percentage,
                }),
              ),
              ...priceLines({ numbers: [100, 50, 0], unit: Unit.percentage }),
            ],
          },
          {
            name: "By Capital",
            title: title("Price Position (USD-weighted)"),
            bottom: [
              ...mapCohortsWithAll(list, all, ({ name, color, tree }) =>
                line({
                  metric: tree.costBasis.spotInvestedCapitalPercentile,
                  name,
                  color,
                  unit: Unit.percentage,
                }),
              ),
              ...priceLines({ numbers: [100, 50, 0], unit: Unit.percentage }),
            ],
          },
        ],
      },
    ],
  };
}
