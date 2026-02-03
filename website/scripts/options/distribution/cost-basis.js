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
import { Unit } from "../../utils/units.js";
import { priceLines } from "../constants.js";
import { line, price } from "../series.js";

/**
 * @param {PercentilesPattern} p
 * @param {(name: string) => string} [n]
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createCorePercentileSeries(p, n = (x) => x) {
  return [
    price({
      metric: p.pct95,
      name: n("p95"),
      color: colors.pct._95,
      defaultActive: false,
    }),
    price({
      metric: p.pct90,
      name: n("p90"),
      color: colors.pct._90,
      defaultActive: false,
    }),
    price({
      metric: p.pct85,
      name: n("p85"),
      color: colors.pct._85,
      defaultActive: false,
    }),
    price({
      metric: p.pct80,
      name: n("p80"),
      color: colors.pct._80,
      defaultActive: false,
    }),
    price({ metric: p.pct75, name: n("p75"), color: colors.pct._75 }),
    price({
      metric: p.pct70,
      name: n("p70"),
      color: colors.pct._70,
      defaultActive: false,
    }),
    price({
      metric: p.pct65,
      name: n("p65"),
      color: colors.pct._65,
      defaultActive: false,
    }),
    price({
      metric: p.pct60,
      name: n("p60"),
      color: colors.pct._60,
      defaultActive: false,
    }),
    price({
      metric: p.pct55,
      name: n("p55"),
      color: colors.pct._55,
      defaultActive: false,
    }),
    price({ metric: p.pct50, name: n("p50"), color: colors.pct._50 }),
    price({
      metric: p.pct45,
      name: n("p45"),
      color: colors.pct._45,
      defaultActive: false,
    }),
    price({
      metric: p.pct40,
      name: n("p40"),
      color: colors.pct._40,
      defaultActive: false,
    }),
    price({
      metric: p.pct35,
      name: n("p35"),
      color: colors.pct._35,
      defaultActive: false,
    }),
    price({
      metric: p.pct30,
      name: n("p30"),
      color: colors.pct._30,
      defaultActive: false,
    }),
    price({ metric: p.pct25, name: n("p25"), color: colors.pct._25 }),
    price({
      metric: p.pct20,
      name: n("p20"),
      color: colors.pct._20,
      defaultActive: false,
    }),
    price({
      metric: p.pct15,
      name: n("p15"),
      color: colors.pct._15,
      defaultActive: false,
    }),
    price({
      metric: p.pct10,
      name: n("p10"),
      color: colors.pct._10,
      defaultActive: false,
    }),
    price({
      metric: p.pct05,
      name: n("p05"),
      color: colors.pct._05,
      defaultActive: false,
    }),
  ];
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
      color: colors.pct._100,
      defaultActive: false,
    }),
    price({
      metric: tree.costBasis.min,
      name: "Min",
      color: colors.pct._0,
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
      color: colors.pct._100,
      defaultActive: false,
    }),
    price({
      metric: p.pct75,
      name: "Q3 (p75)",
      color: colors.pct._75,
      defaultActive: false,
    }),
    price({ metric: p.pct50, name: "Median (p50)", color: colors.pct._50 }),
    price({
      metric: p.pct25,
      name: "Q1 (p25)",
      color: colors.pct._25,
      defaultActive: false,
    }),
    price({
      metric: tree.costBasis.min,
      name: "Min (p0)",
      color: colors.pct._0,
      defaultActive: false,
    }),
  ];
}

/**
 * @template {readonly { name: string, color: Color, tree: { realized: { realizedPrice: ActivePricePattern } } }[]} T
 * @param {T} list
 * @returns {FetchedPriceSeriesBlueprint[]}
 */
function createGroupedSummarySeries(list) {
  return list.map(({ name, color, tree }) =>
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
      color: colors.pct._100,
      defaultActive: false,
    }),
    ...createCorePercentileSeries(cb.percentiles),
    price({
      metric: cb.min,
      name: "p0",
      color: colors.pct._0,
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
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {{ list: T, title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCostBasisSection({ list, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createGroupedSummarySeries(list),
      },
    ],
  };
}

/**
 * @param {{ list: readonly (CohortAll | CohortFull | CohortWithPercentiles)[], title: (metric: string) => string }} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCostBasisSectionWithPercentiles({ list, title }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Summary",
        title: title("Cost Basis Summary"),
        top: createGroupedSummarySeries(list),
      },
      {
        name: "By Coin",
        tree: [
          {
            name: "Average",
            title: title("Realized Price Comparison"),
            top: list.map(({ name, color, tree }) =>
              price({ metric: tree.realized.realizedPrice, name, color }),
            ),
          },
          {
            name: "Median",
            title: title("Cost Basis Median (BTC-weighted)"),
            top: list.map(({ name, color, tree }) =>
              price({ metric: tree.costBasis.percentiles.pct50, name, color }),
            ),
          },
          {
            name: "Q3",
            title: title("Cost Basis Q3 (BTC-weighted)"),
            top: list.map(({ name, color, tree }) =>
              price({ metric: tree.costBasis.percentiles.pct75, name, color }),
            ),
          },
          {
            name: "Q1",
            title: title("Cost Basis Q1 (BTC-weighted)"),
            top: list.map(({ name, color, tree }) =>
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
            top: list.map(({ name, color, tree }) =>
              price({ metric: tree.realized.investorPrice, name, color }),
            ),
          },
          {
            name: "Median",
            title: title("Cost Basis Median (USD-weighted)"),
            top: list.map(({ name, color, tree }) =>
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
            top: list.map(({ name, color, tree }) =>
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
            top: list.map(({ name, color, tree }) =>
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
              ...list.map(({ name, color, tree }) =>
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
              ...list.map(({ name, color, tree }) =>
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
