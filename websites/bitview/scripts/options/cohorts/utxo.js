/**
 * UTXO cohort folder builders
 * Creates option trees for UTXO-based cohorts (no addrCount)
 *
 * Cohort capabilities (based on brk client patterns):
 *
 * With adjustedSopr (RealizedPattern3/4):
 *   - all, term.short, maxAge.*
 *
 * Without adjustedSopr (RealizedPattern/2):
 *   - term.long, minAge.*, ageRange.*, epoch.*, all amount cohorts
 *
 * With cost basis percentiles (CostBasisPattern2):
 *   - all, term.*, ageRange.*
 *
 * Without percentiles (CostBasisPattern):
 *   - maxAge.*, minAge.*, epoch.*, all amount cohorts
 *
 * Folder builders:
 * - createCohortFolderFull: adjustedSopr + percentiles (all, term.short)
 * - createCohortFolderWithAdjusted: adjustedSopr only (maxAge.*)
 * - createCohortFolderWithPercentiles: percentiles only (term.long, ageRange.*)
 * - createCohortFolderBasic: neither (minAge.*, epoch.*, amount cohorts)
 */

import {
  createSingleSupplySeries,
  createGroupedSupplyTotalSeries,
  createGroupedSupplyInProfitSeries,
  createGroupedSupplyInLossSeries,
  createUtxoCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createCostBasisPercentilesSeries,
} from "./shared.js";
import { Unit } from "../../utils/units.js";

// ============================================================================
// Folder Builders (4 variants based on pattern capabilities)
// ============================================================================

/**
 * All folder: for the special "All" cohort (adjustedSopr + percentiles but no RelToMarketCap)
 * @param {PartialContext} ctx
 * @param {CohortAll} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAll(ctx, args) {
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(ctx, args, title),
      createSingleRealizedSectionWithAdjusted(ctx, args, title),
      createSingleUnrealizedSectionAll(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      ...createSingleActivitySectionWithAdjusted(ctx, args, title),
    ],
  };
}

/**
 * Full folder: adjustedSopr + percentiles + RelToMarketCap (term.short only)
 * @param {PartialContext} ctx
 * @param {CohortFull | CohortGroupFull} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderFull(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(ctx, list, title),
        createGroupedUtxoCountChart(ctx, list, title),
        createGroupedRealizedSectionWithAdjusted(ctx, list, title),
        createGroupedUnrealizedSectionFull(ctx, list, title),
        createGroupedCostBasisSectionWithPercentiles(ctx, list, title),
        ...createGroupedActivitySectionWithAdjusted(ctx, list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(ctx, args, title),
      createSingleRealizedSectionWithAdjusted(ctx, args, title),
      createSingleUnrealizedSectionFull(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      ...createSingleActivitySectionWithAdjusted(ctx, args, title),
    ],
  };
}

/**
 * Adjusted folder: adjustedSopr only, no percentiles (maxAge.*)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjusted | CohortGroupWithAdjusted} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithAdjusted(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(ctx, list, title),
        createGroupedUtxoCountChart(ctx, list, title),
        createGroupedRealizedSectionWithAdjusted(ctx, list, title),
        createGroupedUnrealizedSectionWithMarketCap(ctx, list, title),
        createGroupedCostBasisSection(ctx, list, title),
        ...createGroupedActivitySectionWithAdjusted(ctx, list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(ctx, args, title),
      createSingleRealizedSectionWithAdjusted(ctx, args, title),
      createSingleUnrealizedSectionWithMarketCap(ctx, args, title),
      createSingleCostBasisSection(ctx, args, title),
      ...createSingleActivitySectionWithAdjusted(ctx, args, title),
    ],
  };
}

/**
 * Percentiles folder: percentiles only, no adjustedSopr (term.long, ageRange.*)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles | CohortGroupWithPercentiles} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithPercentiles(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(ctx, list, title),
        createGroupedUtxoCountChart(ctx, list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionWithOwnCaps(ctx, list, title),
        createGroupedCostBasisSectionWithPercentiles(ctx, list, title),
        ...createGroupedActivitySectionBasic(ctx, list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(ctx, args, title),
      createSingleRealizedSectionBasic(ctx, args, title),
      createSingleUnrealizedSectionWithOwnCaps(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      ...createSingleActivitySectionBasic(ctx, args, title),
    ],
  };
}

/**
 * Basic folder: no adjustedSopr, no percentiles (minAge.*, epoch.*, amount cohorts)
 * @param {PartialContext} ctx
 * @param {CohortBasic | CohortGroupBasic} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasic(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(ctx, list, title),
        createGroupedUtxoCountChart(ctx, list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionBase(ctx, list, title),
        createGroupedCostBasisSection(ctx, list, title),
        ...createGroupedActivitySectionBasic(ctx, list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(ctx, args, title),
      createSingleRealizedSectionBasic(ctx, args, title),
      createSingleUnrealizedSectionBase(ctx, args, title),
      createSingleCostBasisSection(ctx, args, title),
      ...createSingleActivitySectionBasic(ctx, args, title),
    ],
  };
}

/**
 * Create supply chart for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleSupplyChart(ctx, cohort, title) {
  return {
    name: "supply",
    title: `Supply ${title}`,
    bottom: createSingleSupplySeries(ctx, cohort),
  };
}

/**
 * Create supply section for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSupplySection(ctx, list, title) {
  return {
    name: "supply",
    tree: [
      {
        name: "total",
        title: `Supply ${title}`,
        bottom: createGroupedSupplyTotalSeries(ctx, list),
      },
      {
        name: "in profit",
        title: `Supply In Profit ${title}`,
        bottom: createGroupedSupplyInProfitSeries(ctx, list),
      },
      {
        name: "in loss",
        title: `Supply In Loss ${title}`,
        bottom: createGroupedSupplyInLossSeries(ctx, list),
      },
    ],
  };
}

/**
 * Create UTXO count chart for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleUtxoCountChart(ctx, cohort, title) {
  return {
    name: "utxo count",
    title: `UTXO Count ${title}`,
    bottom: createUtxoCountSeries(ctx, [cohort], false),
  };
}

/**
 * Create UTXO count chart for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createGroupedUtxoCountChart(ctx, list, title) {
  return {
    name: "utxo count",
    title: `UTXO Count ${title}`,
    bottom: createUtxoCountSeries(ctx, list, true),
  };
}

/**
 * Create realized section with adjusted SOPR (for cohorts with RealizedPattern3/4)
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionWithAdjusted(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      createSingleRealizedPriceChart(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Capitalization ${title}`,
        bottom: createSingleRealizedCapSeries(ctx, cohort),
      },
      ...createSingleRealizedPnlSection(ctx, cohort, title),
      createSingleSoprSectionWithAdjusted(ctx, cohort, title),
    ],
  };
}

/**
 * Create realized section with adjusted SOPR for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedRealizedSectionWithAdjusted(ctx, list, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "Price",
        title: `Realized Price ${title}`,
        top: createRealizedPriceSeries(ctx, list),
      },
      {
        name: "Ratio",
        title: `Realized Price Ratio ${title}`,
        bottom: createRealizedPriceRatioSeries(ctx, list),
      },
      {
        name: "capitalization",
        title: `Realized Capitalization ${title}`,
        bottom: createGroupedRealizedCapSeries(ctx, list),
      },
      ...createGroupedRealizedPnlSections(ctx, list, title),
      createGroupedSoprSectionWithAdjusted(ctx, list, title),
    ],
  };
}

/**
 * Create realized section without adjusted SOPR (for cohorts with RealizedPattern/2)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles | CohortBasic} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionBasic(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      createSingleRealizedPriceChart(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Capitalization ${title}`,
        bottom: createSingleRealizedCapSeries(ctx, cohort),
      },
      ...createSingleRealizedPnlSection(ctx, cohort, title),
      createSingleSoprSectionBasic(ctx, cohort, title),
    ],
  };
}

/**
 * Create realized section without adjusted SOPR for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly (CohortWithPercentiles | CohortBasic)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedRealizedSectionBasic(ctx, list, title) {
  return {
    name: "Realized",
    tree: [
      {
        name: "Price",
        title: `Realized Price ${title}`,
        top: createRealizedPriceSeries(ctx, list),
      },
      {
        name: "Ratio",
        title: `Realized Price Ratio ${title}`,
        bottom: createRealizedPriceRatioSeries(ctx, list),
      },
      {
        name: "capitalization",
        title: `Realized Capitalization ${title}`,
        bottom: createGroupedRealizedCapSeries(ctx, list),
      },
      ...createGroupedRealizedPnlSections(ctx, list, title),
      createGroupedSoprSectionBasic(ctx, list, title),
    ],
  };
}

/**
 * Create realized price chart for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleRealizedPriceChart(ctx, cohort, title) {
  const { line } = ctx;
  const { tree, color } = cohort;

  return {
    name: "price",
    title: `Realized Price ${title}`,
    top: [
      line({
        metric: tree.realized.realizedPrice,
        name: "realized",
        color,
        unit: Unit.usd,
      }),
    ],
  };
}

/**
 * Create realized cap series for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRealizedCapSeries(ctx, cohort) {
  const { colors, line, baseline, createPriceLine } = ctx;
  const { color, tree } = cohort;

  return [
    line({
      metric: tree.realized.realizedCap,
      name: "Capitalization",
      color,
      unit: Unit.usd,
    }),
    baseline({
      metric: tree.realized.realizedCap30dDelta,
      name: "30d change",
      unit: Unit.usd,
      defaultActive: false,
    }),
    createPriceLine({ unit: Unit.usd, defaultActive: false }),
    ...("realizedCapRelToOwnMarketCap" in tree.realized
      ? [
          baseline({
            metric: tree.realized.realizedCapRelToOwnMarketCap,
            name: "ratio",
            unit: Unit.pctOwnMcap,
            options: { baseValue: { price: 100 } },
            color: [colors.red, colors.green],
          }),
          createPriceLine({
            unit: Unit.pctOwnMcap,
            defaultActive: true,
            number: 100,
          }),
        ]
      : []),
  ];
}

/**
 * Create realized cap series for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createGroupedRealizedCapSeries(ctx, list) {
  const { line } = ctx;

  return list.map(({ color, name, tree }) =>
    line({
      metric: tree.realized.realizedCap,
      name,
      color,
      unit: Unit.usd,
    }),
  );
}

/**
 * Create realized PnL section for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createSingleRealizedPnlSection(ctx, cohort, title) {
  const {
    colors,
    line,
    baseline,
    createPriceLine,
    fromBlockCountWithUnit,
    fromBitcoinPatternWithUnit,
  } = ctx;
  const { tree } = cohort;

  return [
    {
      name: "pnl",
      title: `Realized Profit And Loss ${title}`,
      bottom: [
        ...fromBlockCountWithUnit(
          tree.realized.realizedProfit,
          "Profit",
          Unit.usd,
          colors.green,
        ),
        ...fromBlockCountWithUnit(
          tree.realized.realizedLoss,
          "Loss",
          Unit.usd,
          colors.red,
        ),
        ...fromBitcoinPatternWithUnit(
          tree.realized.negRealizedLoss,
          "Negative Loss",
          Unit.usd,
          colors.red,
        ),
        ...("realizedProfitToLossRatio" in tree.realized
          ? [
              line({
                metric: tree.realized.realizedProfitToLossRatio,
                name: "Profit / Loss",
                color: colors.yellow,
                unit: Unit.ratio,
              }),
            ]
          : []),
        line({
          metric: tree.realized.totalRealizedPnl,
          name: "Total",
          color: colors.default,
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: tree.realized.realizedProfitRelToRealizedCap.sum,
          name: "Profit",
          color: colors.green,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: tree.realized.realizedLossRelToRealizedCap.sum,
          name: "Loss",
          color: colors.red,
          unit: Unit.pctRcap,
        }),
        createPriceLine({ unit: Unit.pctRcap }),
        createPriceLine({ unit: Unit.usd, defaultActive: false }),
      ],
    },
    {
      name: "Net pnl",
      title: `Net Realized Profit And Loss ${title}`,
      bottom: [
        ...fromBlockCountWithUnit(
          tree.realized.netRealizedPnl,
          "Net",
          Unit.usd,
        ),
        baseline({
          metric: tree.realized.netRealizedPnlCumulative30dDelta,
          name: "Cumulative 30d change",
          unit: Unit.usd,
          defaultActive: false,
        }),
        baseline({
          metric: tree.realized.netRealizedPnlRelToRealizedCap.sum,
          name: "Net",
          unit: Unit.pctRcap,
        }),
        baseline({
          metric:
            tree.realized.netRealizedPnlCumulative30dDeltaRelToRealizedCap,
          name: "Cumulative 30d change",
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: tree.realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
          name: "Cumulative 30d change",
          unit: Unit.pctMcap,
        }),
        createPriceLine({ unit: Unit.pctMcap }),
        createPriceLine({ unit: Unit.pctRcap }),
        createPriceLine({ unit: Unit.usd }),
      ],
    },
  ];
}

/**
 * Create realized PnL sections for grouped cohorts
 * @param {PartialContext} ctx
 * @param {readonly UtxoCohortObject[]} list
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedRealizedPnlSections(ctx, list, title) {
  const { line, baseline, createPriceLine } = ctx;

  return [
    {
      name: "profit",
      title: `Realized Profit ${title}`,
      bottom: [
        ...list.flatMap(({ color, name, tree }) => [
          line({
            metric: tree.realized.realizedProfit.sum,
            name,
            color,
            unit: Unit.usd,
          }),
          baseline({
            metric: tree.realized.realizedProfitRelToRealizedCap.sum,
            name,
            color,
            unit: Unit.pctRcap,
          }),
        ]),
        createPriceLine({ unit: Unit.usd }),
      ],
    },
    {
      name: "loss",
      title: `Realized Loss ${title}`,
      bottom: [
        ...list.flatMap(({ color, name, tree }) => [
          line({
            metric: tree.realized.realizedLoss.sum,
            name,
            color,
            unit: Unit.usd,
          }),
          baseline({
            metric: tree.realized.realizedLossRelToRealizedCap.sum,
            name,
            color,
            unit: Unit.pctRcap,
          }),
        ]),
        createPriceLine({ unit: Unit.usd }),
      ],
    },
    {
      name: "Total pnl",
      title: `Total Realized Profit And Loss ${title}`,
      bottom: [
        ...list.flatMap(({ color, name, tree }) => [
          line({
            metric: tree.realized.totalRealizedPnl,
            name,
            color,
            unit: Unit.usd,
          }),
          ...("realizedProfitToLossRatio" in tree.realized
            ? [
                line({
                  metric: tree.realized.realizedProfitToLossRatio,
                  name,
                  color,
                  unit: Unit.ratio,
                }),
              ]
            : []),
        ]),
      ],
    },
    {
      name: "Net pnl",
      title: `Net Realized Profit And Loss ${title}`,
      bottom: [
        ...list.flatMap(({ color, name, tree }) => [
          baseline({
            metric: tree.realized.netRealizedPnl.sum,
            name,
            color,
            unit: Unit.usd,
          }),
          baseline({
            metric: tree.realized.netRealizedPnlRelToRealizedCap.sum,
            name,
            color,
            unit: Unit.pctRcap,
          }),
        ]),
        createPriceLine({ unit: Unit.usd }),
        createPriceLine({ unit: Unit.pctRcap }),
      ],
    },
    {
      name: "cumulative",
      tree: [
        {
          name: "profit",
          title: `Cumulative Realized Profit ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.realizedProfit.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "loss",
          title: `Cumulative Realized Loss ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.realizedLoss.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "Net pnl",
          title: `Cumulative Net Realized Profit And Loss ${title}`,
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              baseline({
                metric: tree.realized.netRealizedPnl.cumulative,
                name,
                color,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ]),
            createPriceLine({ unit: Unit.usd }),
          ],
        },
        {
          name: "Net pnl 30d change",
          title: `Cumulative Net Realized Profit And Loss 30 Day Change ${title}`,
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              baseline({
                metric: tree.realized.netRealizedPnlCumulative30dDelta,
                name,
                color,
                unit: Unit.usd,
              }),
              baseline({
                metric:
                  tree.realized
                    .netRealizedPnlCumulative30dDeltaRelToRealizedCap,
                name,
                color,
                unit: Unit.pctRcap,
              }),
              baseline({
                metric:
                  tree.realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
                name,
                color,
                unit: Unit.pctMcap,
              }),
            ]),
            createPriceLine({ unit: Unit.usd }),
            createPriceLine({ unit: Unit.pctMcap }),
            createPriceLine({ unit: Unit.pctRcap }),
          ],
        },
      ],
    },
  ];
}

// ============================================================================
// SOPR Chart Builders (Composable)
// ============================================================================

/**
 * @typedef {Object} CohortWithBaseSopr
 * @property {string} name
 * @property {Color} color
 * @property {{ realized: { sopr: any, sopr7dEma: any, sopr30dEma: any } }} tree
 */

/**
 * @typedef {Object} CohortWithAdjustedSopr
 * @property {string} name
 * @property {Color} color
 * @property {{ realized: { adjustedSopr: any, adjustedSopr7dEma: any, adjustedSopr30dEma: any } }} tree
 */

/**
 * Create single base SOPR chart
 * @param {PartialContext} ctx
 * @param {CohortWithBaseSopr} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleBaseSoprChart(ctx, cohort, title) {
  const { colors, baseline, createPriceLine } = ctx;
  const { tree } = cohort;

  return {
    name: "Normal",
    title: `Spent Output Profit Ratio ${title}`,
    bottom: [
      baseline({
        metric: tree.realized.sopr,
        name: "SOPR",
        unit: Unit.ratio,
        options: { baseValue: { price: 1 } },
      }),
      baseline({
        metric: tree.realized.sopr7dEma,
        name: "7d EMA",
        color: [colors.lime, colors.rose],
        unit: Unit.ratio,
        defaultActive: false,
        options: { baseValue: { price: 1 } },
      }),
      baseline({
        metric: tree.realized.sopr30dEma,
        name: "30d EMA",
        color: [colors.avocado, colors.pink],
        unit: Unit.ratio,
        defaultActive: false,
        options: { baseValue: { price: 1 } },
      }),
      createPriceLine({ number: 1, unit: Unit.ratio }),
    ],
  };
}

/**
 * Create single adjusted SOPR chart (only for age cohorts)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjustedSopr} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleAdjustedSoprChart(ctx, cohort, title) {
  const { colors, baseline, createPriceLine } = ctx;
  const { tree } = cohort;

  return {
    name: "Adjusted",
    title: `Adjusted Spent Output Profit Ratio ${title}`,
    bottom: [
      baseline({
        metric: tree.realized.adjustedSopr,
        name: "Adjusted",
        color: [colors.yellow, colors.fuchsia],
        unit: Unit.ratio,
        options: { baseValue: { price: 1 } },
      }),
      baseline({
        metric: tree.realized.adjustedSopr7dEma,
        name: "Adj. 7d EMA",
        color: [colors.amber, colors.purple],
        unit: Unit.ratio,
        defaultActive: false,
        options: { baseValue: { price: 1 } },
      }),
      baseline({
        metric: tree.realized.adjustedSopr30dEma,
        name: "Adj. 30d EMA",
        color: [colors.orange, colors.violet],
        unit: Unit.ratio,
        defaultActive: false,
        options: { baseValue: { price: 1 } },
      }),
      createPriceLine({ number: 1, unit: Unit.ratio }),
    ],
  };
}

/**
 * Create grouped base SOPR chart
 * @param {PartialContext} ctx
 * @param {readonly CohortWithBaseSopr[]} list
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createGroupedBaseSoprChart(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;

  return {
    name: "Normal",
    title: `Spent Output Profit Ratio ${title}`,
    bottom: [
      ...list.flatMap(({ color, name, tree }) => [
        baseline({
          metric: tree.realized.sopr,
          name,
          color,
          unit: Unit.ratio,
          options: { baseValue: { price: 1 } },
        }),
        baseline({
          metric: tree.realized.sopr7dEma,
          name: `${name} 7d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          options: { baseValue: { price: 1 } },
        }),
        baseline({
          metric: tree.realized.sopr30dEma,
          name: `${name} 30d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          options: { baseValue: { price: 1 } },
        }),
      ]),
      createPriceLine({ number: 1, unit: Unit.ratio }),
    ],
  };
}

/**
 * Create grouped adjusted SOPR chart (only for age cohorts)
 * @param {PartialContext} ctx
 * @param {readonly CohortWithAdjustedSopr[]} list
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createGroupedAdjustedSoprChart(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;

  return {
    name: "Adjusted",
    title: `Adjusted Spent Output Profit Ratio ${title}`,
    bottom: [
      ...list.flatMap(({ color, name, tree }) => [
        baseline({
          metric: tree.realized.adjustedSopr,
          name,
          color,
          unit: Unit.ratio,
          options: { baseValue: { price: 1 } },
        }),
        baseline({
          metric: tree.realized.adjustedSopr7dEma,
          name: `${name} 7d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          options: { baseValue: { price: 1 } },
        }),
        baseline({
          metric: tree.realized.adjustedSopr30dEma,
          name: `${name} 30d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          options: { baseValue: { price: 1 } },
        }),
      ]),
      createPriceLine({ number: 1, unit: Unit.ratio }),
    ],
  };
}

// ============================================================================
// SOPR Section Composers
// ============================================================================

/**
 * Create SOPR section with adjusted SOPR (for cohorts with RealizedPattern3/4)
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleSoprSectionWithAdjusted(ctx, cohort, title) {
  return {
    name: "sopr",
    tree: [
      createSingleBaseSoprChart(ctx, cohort, title),
      createSingleAdjustedSoprChart(ctx, cohort, title),
    ],
  };
}

/**
 * Create grouped SOPR section with adjusted SOPR
 * @param {PartialContext} ctx
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSoprSectionWithAdjusted(ctx, list, title) {
  return {
    name: "sopr",
    tree: [
      createGroupedBaseSoprChart(ctx, list, title),
      createGroupedAdjustedSoprChart(ctx, list, title),
    ],
  };
}

/**
 * Create SOPR section without adjusted SOPR (for cohorts with RealizedPattern/2)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles | CohortBasic} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleSoprSectionBasic(ctx, cohort, title) {
  return {
    name: "sopr",
    tree: [createSingleBaseSoprChart(ctx, cohort, title)],
  };
}

/**
 * Create grouped SOPR section without adjusted SOPR
 * @param {PartialContext} ctx
 * @param {readonly (CohortWithPercentiles | CohortBasic)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSoprSectionBasic(ctx, list, title) {
  return {
    name: "sopr",
    tree: [createGroupedBaseSoprChart(ctx, list, title)],
  };
}

// ============================================================================
// Unrealized Section Helpers (by relative pattern capability)
// ============================================================================

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithMarketCap} rel
 */
function createUnrealizedPnlRelToMarketCapMetrics(ctx, rel) {
  const { colors, line } = ctx;
  return [
    line({
      metric: rel.unrealizedProfitRelToMarketCap,
      name: "Profit",
      color: colors.green,
      unit: Unit.pctMcap,
    }),
    line({
      metric: rel.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.red,
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToMarketCap,
      name: "Negative Loss",
      color: colors.red,
      unit: Unit.pctMcap,
    }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnMarketCap} rel
 */
function createUnrealizedPnlRelToOwnMarketCapMetrics(ctx, rel) {
  const { colors, line, createPriceLine } = ctx;
  return [
    line({
      metric: rel.unrealizedProfitRelToOwnMarketCap,
      name: "Profit",
      color: colors.green,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: rel.unrealizedLossRelToOwnMarketCap,
      name: "Loss",
      color: colors.red,
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToOwnMarketCap,
      name: "Negative Loss",
      color: colors.red,
      unit: Unit.pctOwnMcap,
    }),
    createPriceLine({ unit: Unit.pctOwnMcap, number: 100 }),
    createPriceLine({ unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnPnl} rel
 */
function createUnrealizedPnlRelToOwnPnlMetrics(ctx, rel) {
  const { colors, line, createPriceLine } = ctx;
  return [
    line({
      metric: rel.unrealizedProfitRelToOwnTotalUnrealizedPnl,
      name: "Profit",
      color: colors.green,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.red,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.red,
      unit: Unit.pctOwnPnl,
    }),
    createPriceLine({ unit: Unit.pctOwnPnl, number: 100 }),
    createPriceLine({ unit: Unit.pctOwnPnl }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithMarketCap} rel
 */
function createNetUnrealizedPnlRelToMarketCapMetrics(ctx, rel) {
  const { baseline } = ctx;
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToMarketCap,
      name: "Net",
      unit: Unit.pctMcap,
    }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnMarketCap} rel
 */
function createNetUnrealizedPnlRelToOwnMarketCapMetrics(ctx, rel) {
  const { baseline, createPriceLine } = ctx;
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net",
      unit: Unit.pctOwnMcap,
    }),
    createPriceLine({ unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnPnl} rel
 */
function createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, rel) {
  const { baseline, createPriceLine } = ctx;
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net",
      unit: Unit.pctOwnPnl,
    }),
    createPriceLine({ unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Base unrealized metrics (always present)
 * @param {PartialContext} ctx
 * @param {{ unrealized: { totalUnrealizedPnl: AnyMetricPattern, unrealizedProfit: AnyMetricPattern, unrealizedLoss: AnyMetricPattern, negUnrealizedLoss: AnyMetricPattern } }} tree
 */
function createUnrealizedPnlBaseMetrics(ctx, tree) {
  const { colors, line } = ctx;
  return [
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.green,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.red,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.red,
      unit: Unit.usd,
    }),
  ];
}

/**
 * Base net unrealized metric (always present)
 * @param {PartialContext} ctx
 * @param {{ unrealized: { netUnrealizedPnl: AnyMetricPattern } }} tree
 */
function createNetUnrealizedPnlBaseMetric(ctx, tree) {
  const { baseline } = ctx;
  return baseline({
    metric: tree.unrealized.netUnrealizedPnl,
    name: "Net",
    unit: Unit.ratio,
    options: { baseValue: { price: 0 } },
  });
}

// ============================================================================
// Unrealized Section Variants (by cohort capability)
// ============================================================================

/**
 * Unrealized section for All cohort (only RelToOwnPnl)
 * @param {PartialContext} ctx
 * @param {CohortAll} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionAll(ctx, cohort, title) {
  const { createPriceLine } = ctx;
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized Profit And Loss ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(ctx, tree),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Unrealized section for Full cohort (all capabilities: MarketCap + OwnMarketCap + OwnPnl)
 * @param {PartialContext} ctx
 * @param {CohortFull} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionFull(ctx, cohort, title) {
  const { createPriceLine } = ctx;
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized Profit And Loss ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
          createPriceLine({ unit: Unit.pctMcap, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(ctx, tree),
          ...createNetUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd }),
          createPriceLine({ unit: Unit.pctMcap }),
        ],
      },
    ],
  };
}

/**
 * Unrealized section for WithAdjusted cohort (only MarketCap)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjusted} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionWithMarketCap(ctx, cohort, title) {
  const { createPriceLine } = ctx;
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized Profit And Loss ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
          createPriceLine({ unit: Unit.pctMcap, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(ctx, tree),
          ...createNetUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd }),
          createPriceLine({ unit: Unit.pctMcap }),
        ],
      },
    ],
  };
}

/**
 * Unrealized section for patterns with OwnMarketCap + OwnPnl (RelativePattern2, RelativePattern5)
 * Used by: LongTerm, AgeRange
 * @param {PartialContext} ctx
 * @param {{ tree: { unrealized: PatternAll["unrealized"], relative: RelativeWithOwnMarketCap } }} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionWithOwnCaps(ctx, cohort, title) {
  const { createPriceLine } = ctx;
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized Profit And Loss ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(ctx, tree),
          ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          createPriceLine({ unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Unrealized section with only base metrics (no relative unrealized)
 * Used by: Epoch cohorts (RelativePattern4)
 * @param {PartialContext} ctx
 * @param {{ tree: { unrealized: PatternAll["unrealized"] } }} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionBase(ctx, cohort, title) {
  const { createPriceLine } = ctx;
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized Profit And Loss ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          createPriceLine({ unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(ctx, tree),
          createPriceLine({ unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized base charts (profit, loss, total pnl)
 * @param {PartialContext} ctx
 * @param {readonly { color: Color, name: string, tree: { unrealized: PatternAll["unrealized"] } }[]} list
 * @param {string} title
 */
function createGroupedUnrealizedBaseCharts(ctx, list, title) {
  const { line } = ctx;
  return [
    {
      name: "profit",
      title: `Unrealized Profit ${title}`,
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.unrealized.unrealizedProfit,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "loss",
      title: `Unrealized Loss ${title}`,
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.unrealized.unrealizedLoss,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "total pnl",
      title: `Unrealized Total Profit And Loss ${title}`,
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.unrealized.totalUnrealizedPnl,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
  ];
}

/**
 * Grouped unrealized section for Full cohorts (all relative capabilities)
 * @param {PartialContext} ctx
 * @param {readonly CohortFull[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionFull(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(ctx, list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.ratio,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToMarketCap,
              name,
              color,
              unit: Unit.pctMcap,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
              name,
              color,
              unit: Unit.pctOwnMcap,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
              name,
              color,
              unit: Unit.pctOwnPnl,
            }),
          ]),
          createPriceLine({ unit: Unit.usd }),
          createPriceLine({ unit: Unit.pctMcap }),
          createPriceLine({ unit: Unit.pctOwnMcap }),
          createPriceLine({ unit: Unit.pctOwnPnl }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized section for WithAdjusted cohorts (only MarketCap)
 * @param {PartialContext} ctx
 * @param {readonly CohortWithAdjusted[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionWithMarketCap(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(ctx, list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.ratio,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToMarketCap,
              name,
              color,
              unit: Unit.pctMcap,
            }),
          ]),
          createPriceLine({ unit: Unit.usd }),
          createPriceLine({ unit: Unit.pctMcap }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized section for WithPercentiles cohorts (OwnMarketCap + OwnPnl)
 * @param {PartialContext} ctx
 * @param {readonly CohortWithPercentiles[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionWithOwnCaps(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(ctx, list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.ratio,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToOwnMarketCap,
              name,
              color,
              unit: Unit.pctOwnMcap,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
              name,
              color,
              unit: Unit.pctOwnPnl,
            }),
          ]),
          createPriceLine({ unit: Unit.usd }),
          createPriceLine({ unit: Unit.pctOwnMcap }),
          createPriceLine({ unit: Unit.pctOwnPnl }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized section for Basic cohorts (base only)
 * @param {PartialContext} ctx
 * @param {readonly CohortBasic[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionBase(ctx, list, title) {
  const { baseline, createPriceLine } = ctx;
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(ctx, list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized Profit And Loss ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.ratio,
            }),
          ]),
          createPriceLine({ unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Create cost basis section for single cohort WITH percentiles
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleCostBasisSectionWithPercentiles(ctx, cohort, title) {
  const { line } = ctx;
  const { color, tree } = cohort;

  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Average",
        title: `Cost Basis ${title}`,
        top: [
          line({
            metric: tree.realized.realizedPrice,
            name: "Average",
            color,
            unit: Unit.usd,
          }),
          line({
            metric: tree.costBasis.min,
            name: "Min",
            color,
            unit: Unit.usd,
            defaultActive: false,
          }),
          line({
            metric: tree.costBasis.max,
            name: "Max",
            color,
            unit: Unit.usd,
          }),
        ],
      },
      {
        name: "percentiles",
        title: `Cost Basis Percentiles ${title}`,
        top: createCostBasisPercentilesSeries(ctx, [cohort], false),
      },
    ],
  };
}

/**
 * Create cost basis section for grouped cohorts WITH percentiles
 * @param {PartialContext} ctx
 * @param {readonly (CohortFull | CohortWithPercentiles)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedCostBasisSectionWithPercentiles(ctx, list, title) {
  const { line } = ctx;

  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Average",
        title: `Average Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({
            metric: tree.realized.realizedPrice,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Min",
        title: `Min Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({ metric: tree.costBasis.min, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Max",
        title: `Max Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({ metric: tree.costBasis.max, name, color, unit: Unit.usd }),
        ),
      },
    ],
  };
}

/**
 * Create cost basis section for single cohort (no percentiles)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjusted | CohortBasic} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleCostBasisSection(ctx, cohort, title) {
  const { line } = ctx;
  const { color, tree } = cohort;

  return {
    name: "Cost Basis",
    tree: [
      {
        name: "cost basis",
        title: `Cost Basis ${title}`,
        top: [
          line({
            metric: tree.realized.realizedPrice,
            name: "Average",
            color,
            unit: Unit.usd,
          }),
          line({
            metric: tree.costBasis.min,
            name: "Min",
            color,
            unit: Unit.usd,
            defaultActive: false,
          }),
          line({
            metric: tree.costBasis.max,
            name: "Max",
            color,
            unit: Unit.usd,
          }),
        ],
      },
    ],
  };
}

/**
 * Create cost basis section for grouped cohorts (no percentiles)
 * @param {PartialContext} ctx
 * @param {readonly (CohortWithAdjusted | CohortBasic)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedCostBasisSection(ctx, list, title) {
  const { line } = ctx;

  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Average",
        title: `Average Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({
            metric: tree.realized.realizedPrice,
            name,
            color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Min",
        title: `Min Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({ metric: tree.costBasis.min, name, color, unit: Unit.usd }),
        ),
      },
      {
        name: "Max",
        title: `Max Cost Basis ${title}`,
        top: list.map(({ color, name, tree }) =>
          line({ metric: tree.costBasis.max, name, color, unit: Unit.usd }),
        ),
      },
    ],
  };
}

/**
 * Create activity section with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createSingleActivitySectionWithAdjusted(ctx, cohort, title) {
  const { colors, line } = ctx;
  const { tree, color } = cohort;

  return [
    {
      name: "Sell Side Risk",
      title: `Sell Side Risk Ratio ${title}`,
      bottom: [
        line({
          metric: tree.realized.sellSideRiskRatio,
          name: "Raw",
          color: colors.orange,
          unit: Unit.ratio,
        }),
        line({
          metric: tree.realized.sellSideRiskRatio7dEma,
          name: "7d EMA",
          color: colors.red,
          unit: Unit.ratio,
          defaultActive: false,
        }),
        line({
          metric: tree.realized.sellSideRiskRatio30dEma,
          name: "30d EMA",
          color: colors.rose,
          unit: Unit.ratio,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "value",
      tree: [
        {
          name: "created",
          title: `Value Created ${title}`,
          bottom: [
            line({
              metric: tree.realized.valueCreated,
              name: "Normal",
              color: colors.emerald,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.adjustedValueCreated,
              name: "Adjusted",
              color: colors.lime,
              unit: Unit.usd,
            }),
          ],
        },
        {
          name: "destroyed",
          title: `Value Destroyed ${title}`,
          bottom: [
            line({
              metric: tree.realized.valueDestroyed,
              name: "Normal",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.adjustedValueDestroyed,
              name: "Adjusted",
              color: colors.pink,
              unit: Unit.usd,
            }),
          ],
        },
      ],
    },
    {
      name: "Coins Destroyed",
      title: `Coins Destroyed ${title}`,
      bottom: [
        line({
          metric: tree.activity.coinblocksDestroyed.sum,
          name: "Coinblocks",
          color,
          unit: Unit.coinblocks,
        }),
        line({
          metric: tree.activity.coinblocksDestroyed.cumulative,
          name: "Cumulative",
          color,
          unit: Unit.coinblocks,
          defaultActive: false,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.sum,
          name: "Coindays",
          color,
          unit: Unit.coindays,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.cumulative,
          name: "Cumulative",
          color,
          unit: Unit.coindays,
          defaultActive: false,
        }),
      ],
    },
  ];
}

/**
 * Create activity section without adjusted values (for cohorts with RealizedPattern/2)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles | CohortBasic} cohort
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createSingleActivitySectionBasic(ctx, cohort, title) {
  const { colors, line } = ctx;
  const { tree, color } = cohort;

  return [
    {
      name: "Sell Side Risk",
      title: `Sell Side Risk Ratio ${title}`,
      bottom: [
        line({
          metric: tree.realized.sellSideRiskRatio,
          name: "Raw",
          color: colors.orange,
          unit: Unit.ratio,
        }),
        line({
          metric: tree.realized.sellSideRiskRatio7dEma,
          name: "7d EMA",
          color: colors.red,
          unit: Unit.ratio,
          defaultActive: false,
        }),
        line({
          metric: tree.realized.sellSideRiskRatio30dEma,
          name: "30d EMA",
          color: colors.rose,
          unit: Unit.ratio,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "value",
      tree: [
        {
          name: "created",
          title: `Value Created ${title}`,
          bottom: [
            line({
              metric: tree.realized.valueCreated,
              name: "Value Created",
              color: colors.emerald,
              unit: Unit.usd,
            }),
          ],
        },
        {
          name: "destroyed",
          title: `Value Destroyed ${title}`,
          bottom: [
            line({
              metric: tree.realized.valueDestroyed,
              name: "Value Destroyed",
              color: colors.red,
              unit: Unit.usd,
            }),
          ],
        },
      ],
    },
    {
      name: "Coins Destroyed",
      title: `Coins Destroyed ${title}`,
      bottom: [
        line({
          metric: tree.activity.coinblocksDestroyed.sum,
          name: "Coinblocks",
          color,
          unit: Unit.coinblocks,
        }),
        line({
          metric: tree.activity.coinblocksDestroyed.cumulative,
          name: "Cumulative",
          color,
          unit: Unit.coinblocks,
          defaultActive: false,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.sum,
          name: "Coindays",
          color,
          unit: Unit.coindays,
        }),
        line({
          metric: tree.activity.coindaysDestroyed.cumulative,
          name: "Cumulative",
          color,
          unit: Unit.coindays,
          defaultActive: false,
        }),
      ],
    },
  ];
}

/**
 * Create activity section for grouped cohorts with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {PartialContext} ctx
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedActivitySectionWithAdjusted(ctx, list, title) {
  const { line } = ctx;

  return [
    {
      name: "Sell Side Risk",
      title: `Sell Side Risk Ratio ${title}`,
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.sellSideRiskRatio,
          name,
          color,
          unit: Unit.ratio,
        }),
      ]),
    },
    {
      name: "value",
      tree: [
        {
          name: "created",
          tree: [
            {
              name: "Normal",
              title: `Value Created ${title}`,
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.realized.valueCreated,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ]),
            },
            {
              name: "Adjusted",
              title: `Adjusted Value Created ${title}`,
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.realized.adjustedValueCreated,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ]),
            },
          ],
        },
        {
          name: "destroyed",
          tree: [
            {
              name: "Normal",
              title: `Value Destroyed ${title}`,
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.realized.valueDestroyed,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ]),
            },
            {
              name: "Adjusted",
              title: `Adjusted Value Destroyed ${title}`,
              bottom: list.flatMap(({ color, name, tree }) => [
                line({
                  metric: tree.realized.adjustedValueDestroyed,
                  name,
                  color,
                  unit: Unit.usd,
                }),
              ]),
            },
          ],
        },
      ],
    },
    {
      name: "Coins Destroyed",
      tree: [
        {
          name: "Sum",
          title: `Sum of Coins Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.activity.coinblocksDestroyed.sum,
              name,
              color,
              unit: Unit.coinblocks,
            }),
            line({
              metric: tree.activity.coindaysDestroyed.sum,
              name,
              color,
              unit: Unit.coindays,
            }),
          ]),
        },
        {
          name: "Cumulative",
          title: `Cumulative Coins Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.activity.coinblocksDestroyed.cumulative,
              name,
              color,
              unit: Unit.coinblocks,
            }),
            line({
              metric: tree.activity.coindaysDestroyed.cumulative,
              name,
              color,
              unit: Unit.coindays,
            }),
          ]),
        },
      ],
    },
  ];
}

/**
 * Create activity section for grouped cohorts without adjusted values (for cohorts with RealizedPattern/2)
 * @param {PartialContext} ctx
 * @param {readonly (CohortWithPercentiles | CohortBasic)[]} list
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedActivitySectionBasic(ctx, list, title) {
  const { line } = ctx;

  return [
    {
      name: "Sell Side Risk",
      title: `Sell Side Risk Ratio ${title}`,
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.sellSideRiskRatio,
          name,
          color,
          unit: Unit.ratio,
        }),
      ]),
    },
    {
      name: "value",
      tree: [
        {
          name: "created",
          title: `Value Created ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.valueCreated,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "destroyed",
          title: `Value Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.valueDestroyed,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
      ],
    },
    {
      name: "Coins Destroyed",
      tree: [
        {
          name: "Sum",
          title: `Sum of Coins Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.activity.coinblocksDestroyed.sum,
              name,
              color,
              unit: Unit.coinblocks,
            }),
            line({
              metric: tree.activity.coindaysDestroyed.sum,
              name,
              color,
              unit: Unit.coindays,
            }),
          ]),
        },
        {
          name: "Cumulative",
          title: `Cumulative Coins Destroyed ${title}`,
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.activity.coinblocksDestroyed.cumulative,
              name,
              color,
              unit: Unit.coinblocks,
            }),
            line({
              metric: tree.activity.coindaysDestroyed.cumulative,
              name,
              color,
              unit: Unit.coindays,
            }),
          ]),
        },
      ],
    },
  ];
}
