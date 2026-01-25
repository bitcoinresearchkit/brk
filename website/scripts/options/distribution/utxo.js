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
import { createRatioChart, createZScoresFolder } from "../shared.js";
import { Unit } from "../../utils/units.js";
import { line, baseline } from "../series.js";
import { priceLine } from "../constants.js";

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
      createSingleUtxoCountChart(args, title),
      createSingleAddrCountChart(ctx, args, title),
      createSingleRealizedSectionFull(ctx, args, title),
      createSingleUnrealizedSectionAll(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      createSingleActivitySectionWithAdjusted(ctx, args, title),
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
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionWithAdjusted(ctx, list, title),
        createGroupedUnrealizedSectionFull(ctx, list, title),
        createGroupedCostBasisSectionWithPercentiles(ctx, list, title),
        createGroupedActivitySectionWithAdjusted(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionFull(ctx, args, title),
      createSingleUnrealizedSectionFull(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      createSingleActivitySectionWithAdjusted(ctx, args, title),
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
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionWithAdjusted(ctx, list, title),
        createGroupedUnrealizedSectionWithMarketCap(ctx, list, title),
        createGroupedCostBasisSection(list, title),
        createGroupedActivitySectionWithAdjusted(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionWithAdjusted(ctx, args, title),
      createSingleUnrealizedSectionWithMarketCap(ctx, args, title),
      createSingleCostBasisSection(args, title),
      createSingleActivitySectionWithAdjusted(ctx, args, title),
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
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionWithOwnCaps(ctx, list, title),
        createGroupedCostBasisSectionWithPercentiles(ctx, list, title),
        createGroupedActivitySectionBasic(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionWithPercentiles(ctx, args, title),
      createSingleUnrealizedSectionWithOwnCaps(ctx, args, title),
      createSingleCostBasisSectionWithPercentiles(ctx, args, title),
      createSingleActivitySectionBasic(ctx, args, title),
    ],
  };
}

/**
 * Basic folder WITH RelToMarketCap (minAge.*, geAmount.*, ltAmount.*, type.*)
 * @param {PartialContext} ctx
 * @param {CohortBasicWithMarketCap | CohortGroupBasicWithMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithMarketCap(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionWithMarketCapOnly(ctx, list, title),
        createGroupedCostBasisSection(list, title),
        createGroupedActivitySectionBasic(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(ctx, args, title),
      createSingleUnrealizedSectionWithMarketCapOnly(ctx, args, title),
      createSingleCostBasisSection(args, title),
      createSingleActivitySectionBasic(ctx, args, title),
    ],
  };
}

/**
 * Basic folder WITHOUT RelToMarketCap (epoch.*, amountRange.*, year.*)
 * @param {PartialContext} ctx
 * @param {CohortBasicWithoutMarketCap | CohortGroupBasicWithoutMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithoutMarketCap(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionBase(ctx, list, title),
        createGroupedCostBasisSection(list, title),
        createGroupedActivitySectionBasic(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(ctx, args, title),
      createSingleUnrealizedSectionBase(ctx, args, title),
      createSingleCostBasisSection(args, title),
      createSingleActivitySectionBasic(ctx, args, title),
    ],
  };
}

/**
 * @typedef {Object} CohortGroupAddress
 * @property {string} name
 * @property {string} title
 * @property {readonly CohortAddress[]} list
 */

/**
 * Address folder: like basic but with address count (addressable type cohorts)
 * Uses base unrealized section (no RelToMarketCap since it extends CohortBasicWithoutMarketCap)
 * @param {PartialContext} ctx
 * @param {CohortAddress | CohortGroupAddress} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAddress(ctx, args) {
  if ("list" in args) {
    const { list } = args;
    const title = args.title ? `by ${args.title}` : "";
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedAddrCountChart(ctx, list, title),
        createGroupedRealizedSectionBasic(ctx, list, title),
        createGroupedUnrealizedSectionBase(ctx, list, title),
        createGroupedCostBasisSection(list, title),
        createGroupedActivitySectionBasic(list, title),
      ],
    };
  }
  const title = args.title ? `of ${args.title}` : "";
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(ctx, args, title),
      createSingleUtxoCountChart(args, title),
      createSingleAddrCountChart(ctx, args, title),
      createSingleRealizedSectionBasic(ctx, args, title),
      createSingleUnrealizedSectionBase(ctx, args, title),
      createSingleCostBasisSection(args, title),
      createSingleActivitySectionBasic(ctx, args, title),
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
 * @param {readonly UtxoCohortObject[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSupplySection(list, title) {
  return {
    name: "supply",
    tree: [
      {
        name: "total",
        title: `Supply ${title}`,
        bottom: createGroupedSupplyTotalSeries(list),
      },
      {
        name: "in profit",
        title: `Supply In Profit ${title}`,
        bottom: createGroupedSupplyInProfitSeries(list),
      },
      {
        name: "in loss",
        title: `Supply In Loss ${title}`,
        bottom: createGroupedSupplyInLossSeries(list),
      },
    ],
  };
}

/**
 * Create UTXO count chart for single cohort
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleUtxoCountChart(cohort, title) {
  return {
    name: "utxo count",
    title: `UTXO Count ${title}`,
    bottom: createUtxoCountSeries([cohort], false),
  };
}

/**
 * Create UTXO count chart for grouped cohorts
 * @param {readonly UtxoCohortObject[]} list
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createGroupedUtxoCountChart(list, title) {
  return {
    name: "utxo count",
    title: `UTXO Count ${title}`,
    bottom: createUtxoCountSeries(list, true),
  };
}

/**
 * Create address count chart for single cohort with addrCount
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortAddress} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleAddrCountChart(ctx, cohort, title) {
  return {
    name: "address count",
    title: `Address Count ${title}`,
    bottom: [
      line({
        metric: cohort.addrCount,
        name: "Count",
        color: ctx.colors.orange,
        unit: Unit.count,
      }),
    ],
  };
}

/**
 * Create address count chart for grouped cohorts with addrCount
 * @param {PartialContext} _ctx
 * @param {readonly CohortAddress[]} list
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createGroupedAddrCountChart(_ctx, list, title) {
  return {
    name: "address count",
    title: `Address Count ${title}`,
    bottom: list.map(({ color, name, addrCount }) =>
      line({ metric: addrCount, name, color, unit: Unit.count }),
    ),
  };
}

/**
 * Create realized section for CohortAll/CohortFull (adjustedSopr + full ratio)
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionFull(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsWithRatio(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
        bottom: createSingleRealizedCapSeries(ctx, cohort),
      },
      ...createSingleRealizedPnlSection(ctx, cohort, title),
      createSingleSoprSectionWithAdjusted(ctx, cohort, title),
    ],
  };
}

/**
 * Create realized section for CohortWithAdjusted (adjustedSopr but partial ratio)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjusted} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionWithAdjusted(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsBasic(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
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
        top: createRealizedPriceSeries(list),
      },
      {
        name: "Ratio",
        title: `Realized Price Ratio ${title}`,
        bottom: createRealizedPriceRatioSeries(ctx, list),
      },
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
        bottom: createGroupedRealizedCapSeries(list),
      },
      ...createGroupedRealizedPnlSections(ctx, list, title),
      createGroupedSoprSectionWithAdjusted(ctx, list, title),
    ],
  };
}

/**
 * Create realized section for CohortWithPercentiles (no adjustedSopr but full ratio)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionWithPercentiles(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsWithRatio(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
        bottom: createSingleRealizedCapSeries(ctx, cohort),
      },
      ...createSingleRealizedPnlSection(ctx, cohort, title),
      createSingleSoprSectionBasic(ctx, cohort, title),
    ],
  };
}

/**
 * Create realized section for CohortBasic (no adjustedSopr, partial ratio)
 * @param {PartialContext} ctx
 * @param {CohortBasic | CohortAddress} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionBasic(ctx, cohort, title) {
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsBasic(ctx, cohort, title),
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
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
 * @param {readonly (CohortWithPercentiles | CohortBasic | CohortAddress)[]} list
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
        top: createRealizedPriceSeries(list),
      },
      {
        name: "Ratio",
        title: `Realized Price Ratio ${title}`,
        bottom: createRealizedPriceRatioSeries(ctx, list),
      },
      {
        name: "capitalization",
        title: `Realized Cap ${title}`,
        bottom: createGroupedRealizedCapSeries(list),
      },
      ...createGroupedRealizedPnlSections(ctx, list, title),
      createGroupedSoprSectionBasic(ctx, list, title),
    ],
  };
}

/**
 * Create realized price chart for single cohort
 * @param {UtxoCohortObject} cohort
 * @param {string} title
 * @returns {PartialChartOption}
 */
function createSingleRealizedPriceChart(cohort, title) {
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
 * Create realized price and ratio charts for cohorts with full ActivePriceRatioPattern
 * (CohortAll, CohortFull, CohortWithPercentiles have RealizedPattern2/3 which has ActivePriceRatioPattern)
 * @param {PartialContext} ctx
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @param {string} title
 * @returns {PartialOptionsTree}
 */
function createSingleRealizedPriceChartsWithRatio(ctx, cohort, title) {
  const { tree, color } = cohort;
  const ratio = /** @type {ActivePriceRatioPattern} */ (
    tree.realized.realizedPriceExtra
  );
  return [
    createSingleRealizedPriceChart(cohort, title),
    createRatioChart(ctx, {
      title,
      price: tree.realized.realizedPrice,
      ratio,
      color,
      name: "MVRV",
    }),
    createZScoresFolder(ctx, {
      title: `Realized Price ${title}`,
      legend: "price",
      price: tree.realized.realizedPrice,
      ratio,
      color,
    }),
  ];
}

/**
 * Create realized price and basic ratio charts for cohorts with RealizedPriceExtraPattern
 * (CohortWithAdjusted, CohortBasic have RealizedPattern/4 which has RealizedPriceExtraPattern)
 * @param {PartialContext} ctx
 * @param {CohortWithAdjusted | CohortBasic | CohortAddress} cohort
 * @param {string} title
 * @returns {PartialChartOption[]}
 */
function createSingleRealizedPriceChartsBasic(ctx, cohort, title) {
  const { tree, color } = cohort;
  return [
    createSingleRealizedPriceChart(cohort, title),
    {
      name: "ratio",
      title: `Realized Price Ratio ${title}`,
      bottom: [
        baseline({
          metric: tree.realized.realizedPriceExtra.ratio,
          name: "Ratio",
          color,
          unit: Unit.ratio,
        }),
        priceLine({ ctx, unit: Unit.ratio, number: 1 }),
      ],
    },
  ];
}

/**
 * Create realized cap series for single cohort
 * @param {PartialContext} ctx
 * @param {UtxoCohortObject} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRealizedCapSeries(ctx, cohort) {
  const { color, tree } = cohort;

  return [
    line({
      metric: tree.realized.realizedCap,
      name: "Capitalization",
      color,
      unit: Unit.usd,
    }),
    line({
      metric: tree.realized.realizedValue,
      name: "Value",
      color,
      unit: Unit.usd,
      defaultActive: false,
    }),
    baseline({
      metric: tree.realized.realizedCap30dDelta,
      name: "30d change",
      unit: Unit.usd,
      defaultActive: false,
    }),
    priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
    ...("realizedCapRelToOwnMarketCap" in tree.realized
      ? [
          baseline({
            metric: tree.realized.realizedCapRelToOwnMarketCap,
            name: "ratio",
            unit: Unit.pctOwnMcap,
            options: { baseValue: { price: 100 } },
          }),
          priceLine({
            ctx,
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
 * @param {readonly UtxoCohortObject[]} list
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createGroupedRealizedCapSeries(list) {
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
  const { colors, fromBlockCountWithUnit, fromBitcoinPatternWithUnit } = ctx;
  const { tree } = cohort;

  return [
    {
      name: "pnl",
      title: `Realized P&L ${title}`,
      bottom: [
        ...fromBlockCountWithUnit(
          tree.realized.realizedProfit,
          Unit.usd,
          "Profit",
          colors.green,
        ),
        ...fromBlockCountWithUnit(
          tree.realized.realizedLoss,
          Unit.usd,
          "Loss",
          colors.red,
        ),
        ...fromBitcoinPatternWithUnit(
          tree.realized.negRealizedLoss,
          Unit.usd,
          "Negative Loss",
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
          metric: tree.realized.realizedProfitRelToRealizedCap.cumulative,
          name: "Profit Cumulative",
          color: colors.green,
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        baseline({
          metric: tree.realized.realizedLossRelToRealizedCap.sum,
          name: "Loss",
          color: colors.red,
          unit: Unit.pctRcap,
        }),
        baseline({
          metric: tree.realized.realizedLossRelToRealizedCap.cumulative,
          name: "Loss Cumulative",
          color: colors.red,
          unit: Unit.pctRcap,
          defaultActive: false,
        }),
        priceLine({ ctx, unit: Unit.pctRcap }),
        priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
      ],
    },
    {
      name: "Net pnl",
      title: `Net Realized P&L ${title}`,
      bottom: [
        ...fromBlockCountWithUnit(
          tree.realized.netRealizedPnl,
          Unit.usd,
          "Net",
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
          metric: tree.realized.netRealizedPnlRelToRealizedCap.cumulative,
          name: "Net Cumulative",
          unit: Unit.pctRcap,
          defaultActive: false,
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
        priceLine({ ctx, unit: Unit.pctMcap }),
        priceLine({ ctx, unit: Unit.pctRcap }),
        priceLine({ ctx, unit: Unit.usd }),
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
        priceLine({ ctx, unit: Unit.usd }),
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
        priceLine({ ctx, unit: Unit.usd }),
      ],
    },
    {
      name: "Total pnl",
      title: `Total Realized P&L ${title}`,
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
      title: `Net Realized P&L ${title}`,
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
        priceLine({ ctx, unit: Unit.usd }),
        priceLine({ ctx, unit: Unit.pctRcap }),
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
          title: `Cumulative Net Realized P&L ${title}`,
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              baseline({
                metric: tree.realized.netRealizedPnl.cumulative,
                name,
                color,
                unit: Unit.usd,
              }),
            ]),
            priceLine({ ctx, unit: Unit.usd }),
          ],
        },
        {
          name: "Net pnl 30d change",
          title: `Net Realized P&L 30d Change ${title}`,
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
            priceLine({ ctx, unit: Unit.usd }),
            priceLine({ ctx, unit: Unit.pctMcap }),
            priceLine({ ctx, unit: Unit.pctRcap }),
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
  const { colors } = ctx;
  const { tree } = cohort;

  return {
    name: "Normal",
    title: `SOPR ${title}`,
    bottom: [
      baseline({
        metric: tree.realized.sopr,
        name: "SOPR",
        unit: Unit.ratio,
        base: 1,
      }),
      baseline({
        metric: tree.realized.sopr7dEma,
        name: "7d EMA",
        color: [colors.lime, colors.rose],
        unit: Unit.ratio,
        defaultActive: false,
        base: 1,
      }),
      baseline({
        metric: tree.realized.sopr30dEma,
        name: "30d EMA",
        color: [colors.avocado, colors.pink],
        unit: Unit.ratio,
        defaultActive: false,
        base: 1,
      }),
      priceLine({ ctx, number: 1, unit: Unit.ratio }),
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
  const { colors } = ctx;
  const { tree } = cohort;

  return {
    name: "Adjusted",
    title: `aSOPR ${title}`,
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
      priceLine({ ctx, number: 1, unit: Unit.ratio }),
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
  return {
    name: "Normal",
    title: `SOPR ${title}`,
    bottom: [
      ...list.flatMap(({ color, name, tree }) => [
        baseline({
          metric: tree.realized.sopr,
          name,
          color,
          unit: Unit.ratio,
          base: 1,
        }),
        baseline({
          metric: tree.realized.sopr7dEma,
          name: `${name} 7d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          base: 1,
        }),
        baseline({
          metric: tree.realized.sopr30dEma,
          name: `${name} 30d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          base: 1,
        }),
      ]),
      priceLine({ ctx, number: 1, unit: Unit.ratio }),
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
  return {
    name: "Adjusted",
    title: `aSOPR ${title}`,
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
      priceLine({ ctx, number: 1, unit: Unit.ratio }),
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
 * @param {CohortWithPercentiles | CohortBasic | CohortAddress} cohort
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
 * @param {readonly (CohortWithPercentiles | CohortBasic | CohortAddress)[]} list
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
  const { colors } = ctx;
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
  const { colors } = ctx;
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
    priceLine({ ctx, unit: Unit.pctOwnMcap, number: 100 }),
    priceLine({ ctx, unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnPnl} rel
 */
function createUnrealizedPnlRelToOwnPnlMetrics(ctx, rel) {
  const { colors } = ctx;
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
    priceLine({ ctx, unit: Unit.pctOwnPnl, number: 100 }),
    priceLine({ ctx, unit: Unit.pctOwnPnl }),
  ];
}

/**
 * @param {RelativeWithMarketCap} rel
 */
function createNetUnrealizedPnlRelToMarketCapMetrics(rel) {
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
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net",
      unit: Unit.pctOwnMcap,
    }),
    priceLine({ ctx, unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {PartialContext} ctx
 * @param {RelativeWithOwnPnl} rel
 */
function createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, rel) {
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net",
      unit: Unit.pctOwnPnl,
    }),
    priceLine({ ctx, unit: Unit.pctOwnPnl }),
  ];
}

/**
 * Base unrealized metrics (always present)
 * @param {PartialContext} ctx
 * @param {{ unrealized: { totalUnrealizedPnl: AnyMetricPattern, unrealizedProfit: AnyMetricPattern, unrealizedLoss: AnyMetricPattern, negUnrealizedLoss: AnyMetricPattern } }} tree
 */
function createUnrealizedPnlBaseMetrics(ctx, tree) {
  const { colors } = ctx;
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
 * @param {{ unrealized: { netUnrealizedPnl: AnyMetricPattern } }} tree
 */
function createNetUnrealizedPnlBaseMetric(tree) {
  return baseline({
    metric: tree.unrealized.netUnrealizedPnl,
    name: "Net",
    unit: Unit.usd,
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
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd }),
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
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
          priceLine({ ctx, unit: Unit.pctMcap, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
          ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
        ],
      },
      {
        name: "nupl",
        title: `NUPL ${title}`,
        bottom: [
          baseline({
            metric: tree.relative.nupl,
            name: "NUPL",
            unit: Unit.ratio,
          }),
          priceLine({ ctx, unit: Unit.ratio }),
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
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
          priceLine({ ctx, unit: Unit.pctMcap, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
        ],
      },
      ...("nupl" in tree.relative
        ? [
            {
              name: "nupl",
              title: `NUPL ${title}`,
              bottom: [
                baseline({
                  metric: tree.relative.nupl,
                  name: "NUPL",
                  unit: Unit.ratio,
                }),
                priceLine({ ctx, unit: Unit.ratio }),
              ],
            },
          ]
        : []),
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
  const { tree } = cohort;
  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(ctx, tree.relative),
          ...createNetUnrealizedPnlRelToOwnPnlMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd }),
        ],
      },
      ...("nupl" in tree.relative
        ? [
            {
              name: "nupl",
              title: `NUPL ${title}`,
              bottom: [
                baseline({
                  metric: tree.relative.nupl,
                  name: "NUPL",
                  unit: Unit.ratio,
                }),
                priceLine({ ctx, unit: Unit.ratio }),
              ],
            },
          ]
        : []),
    ],
  };
}

/**
 * Unrealized section WITH RelToMarketCap metrics (for CohortBasicWithMarketCap)
 * Used by: minAge.*, geAmount.*, ltAmount.*
 * @param {PartialContext} ctx
 * @param {CohortBasicWithMarketCap} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionWithMarketCapOnly(ctx, cohort, title) {
  const { tree } = cohort;

  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          ...createUnrealizedPnlRelToMarketCapMetrics(ctx, tree.relative),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
          priceLine({ ctx, unit: Unit.pctMcap, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
        ],
      },
      ...("nupl" in tree.relative
        ? [
            {
              name: "nupl",
              title: `NUPL ${title}`,
              bottom: [
                baseline({
                  metric: tree.relative.nupl,
                  name: "NUPL",
                  unit: Unit.ratio,
                }),
                priceLine({ ctx, unit: Unit.ratio }),
              ],
            },
          ]
        : []),
    ],
  };
}

/**
 * Unrealized section with only base metrics (no RelToMarketCap)
 * Used by: epoch.*, amountRange.*, year.*
 * @param {PartialContext} ctx
 * @param {CohortBasicWithoutMarketCap} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleUnrealizedSectionBase(ctx, cohort, title) {
  const { tree } = cohort;

  return {
    name: "Unrealized",
    tree: [
      {
        name: "pnl",
        title: `Unrealized P&L ${title}`,
        bottom: [
          ...createUnrealizedPnlBaseMetrics(ctx, tree),
          priceLine({ ctx, unit: Unit.usd, defaultActive: false }),
        ],
      },
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          createNetUnrealizedPnlBaseMetric(tree),
          priceLine({ ctx, unit: Unit.usd }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized base charts (profit, loss, total pnl)
 * @param {readonly { color: Color, name: string, tree: { unrealized: PatternAll["unrealized"] } }[]} list
 * @param {string} title
 */
function createGroupedUnrealizedBaseCharts(list, title) {
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
      title: `Unrealized Total P&L ${title}`,
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
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.usd,
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
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
          priceLine({ ctx, unit: Unit.pctOwnMcap }),
          priceLine({ ctx, unit: Unit.pctOwnPnl }),
        ],
      },
      {
        name: "nupl",
        title: `NUPL ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.relative.nupl,
              name,
              color,
              unit: Unit.ratio,
            }),
          ]),
          priceLine({ ctx, unit: Unit.ratio }),
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
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.usd,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToMarketCap,
              name,
              color,
              unit: Unit.pctMcap,
            }),
          ]),
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
        ],
      },
      {
        name: "nupl",
        title: `NUPL ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.relative.nupl,
              name,
              color,
              unit: Unit.ratio,
            }),
          ]),
          priceLine({ ctx, unit: Unit.ratio }),
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
  const cohortsWithNupl = list.filter(({ tree }) => "nupl" in tree.relative);
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.usd,
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
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctOwnMcap }),
          priceLine({ ctx, unit: Unit.pctOwnPnl }),
        ],
      },
      ...(cohortsWithNupl.length > 0
        ? [
            {
              name: "nupl",
              title: `NUPL ${title}`,
              bottom: [
                ...cohortsWithNupl.flatMap(({ color, name, tree }) => [
                  baseline({
                    metric: /** @type {{ nupl: AnyMetricPattern }} */ (
                      tree.relative
                    ).nupl,
                    name,
                    color,
                    unit: Unit.ratio,
                  }),
                ]),
                priceLine({ ctx, unit: Unit.ratio }),
              ],
            },
          ]
        : []),
    ],
  };
}

/**
 * Grouped unrealized section WITH RelToMarketCap (for CohortBasicWithMarketCap)
 * Used by: minAge.*, geAmount.*, ltAmount.*
 * @param {PartialContext} ctx
 * @param {readonly CohortBasicWithMarketCap[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionWithMarketCapOnly(ctx, list, title) {
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.usd,
            }),
            baseline({
              metric: tree.relative.netUnrealizedPnlRelToMarketCap,
              name,
              color,
              unit: Unit.pctMcap,
            }),
          ]),
          priceLine({ ctx, unit: Unit.usd }),
          priceLine({ ctx, unit: Unit.pctMcap }),
        ],
      },
      {
        name: "nupl",
        title: `NUPL ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.relative.nupl,
              name,
              color,
              unit: Unit.ratio,
            }),
          ]),
          priceLine({ ctx, unit: Unit.ratio }),
        ],
      },
    ],
  };
}

/**
 * Grouped unrealized section without RelToMarketCap (for CohortBasicWithoutMarketCap)
 * @param {PartialContext} ctx
 * @param {readonly CohortBasicWithoutMarketCap[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionBase(ctx, list, title) {
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net pnl",
        title: `Net Unrealized P&L ${title}`,
        bottom: [
          ...list.flatMap(({ color, name, tree }) => [
            baseline({
              metric: tree.unrealized.netUnrealizedPnl,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
          priceLine({ ctx, unit: Unit.usd }),
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
  const { colors } = ctx;
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
        top: createCostBasisPercentilesSeries(colors, [cohort], false),
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
  const { colors } = ctx;
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
      {
        name: "percentiles",
        title: `Cost Basis Percentiles ${title}`,
        top: createCostBasisPercentilesSeries(colors, list, true),
      },
    ],
  };
}

/**
 * Create cost basis section for single cohort (no percentiles)
 * @param {CohortWithAdjusted | CohortBasic | CohortAddress} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleCostBasisSection(cohort, title) {
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
 * @param {readonly (CohortWithAdjusted | CohortBasic | CohortAddress)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedCostBasisSection(list, title) {
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
 * @returns {PartialOptionsGroup}
 */
function createSingleActivitySectionWithAdjusted(ctx, cohort, title) {
  const { colors, fromBlockCountWithUnit, fromBitcoinPatternWithUnit } = ctx;
  const { tree, color } = cohort;

  return {
    name: "Activity",
    tree: [
      {
        name: "Sent",
        title: `Sent ${title}`,
        bottom: [
          ...fromBlockCountWithUnit(
            tree.activity.sent.sats,
            Unit.sats,
            undefined,
            color,
          ),
          ...fromBitcoinPatternWithUnit(
            tree.activity.sent.bitcoin,
            Unit.btc,
            undefined,
            color,
          ),
          ...fromBlockCountWithUnit(
            tree.activity.sent.dollars,
            Unit.usd,
            undefined,
            color,
          ),
        ],
      },
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
        title: `Value Created & Destroyed ${title}`,
        bottom: [
          line({
            metric: tree.realized.valueCreated,
            name: "Created",
            color: colors.emerald,
            unit: Unit.usd,
          }),
          line({
            metric: tree.realized.adjustedValueCreated,
            name: "Adjusted Created",
            color: colors.lime,
            unit: Unit.usd,
          }),
          line({
            metric: tree.realized.valueDestroyed,
            name: "Destroyed",
            color: colors.red,
            unit: Unit.usd,
          }),
          line({
            metric: tree.realized.adjustedValueDestroyed,
            name: "Adjusted Destroyed",
            color: colors.pink,
            unit: Unit.usd,
          }),
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
          line({
            metric: tree.activity.satblocksDestroyed,
            name: "Satblocks",
            color,
            unit: Unit.satblocks,
          }),
          line({
            metric: tree.activity.satdaysDestroyed,
            name: "Satdays",
            color,
            unit: Unit.satdays,
          }),
        ],
      },
    ],
  };
}

/**
 * Create activity section without adjusted values (for cohorts with RealizedPattern/2)
 * @param {PartialContext} ctx
 * @param {CohortWithPercentiles | CohortBasic | CohortAddress} cohort
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleActivitySectionBasic(ctx, cohort, title) {
  const { colors, fromBlockCountWithUnit, fromBitcoinPatternWithUnit } = ctx;
  const { tree, color } = cohort;

  return {
    name: "Activity",
    tree: [
      {
        name: "Sent",
        title: `Sent ${title}`,
        bottom: [
          ...fromBlockCountWithUnit(
            tree.activity.sent.sats,
            Unit.sats,
            undefined,
            color,
          ),
          ...fromBitcoinPatternWithUnit(
            tree.activity.sent.bitcoin,
            Unit.btc,
            undefined,
            color,
          ),
          ...fromBlockCountWithUnit(
            tree.activity.sent.dollars,
            Unit.usd,
            undefined,
            color,
          ),
        ],
      },
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
        title: `Value Created & Destroyed ${title}`,
        bottom: [
          line({
            metric: tree.realized.valueCreated,
            name: "Created",
            color: colors.emerald,
            unit: Unit.usd,
          }),
          line({
            metric: tree.realized.valueDestroyed,
            name: "Destroyed",
            color: colors.red,
            unit: Unit.usd,
          }),
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
          line({
            metric: tree.activity.satblocksDestroyed,
            name: "Satblocks",
            color,
            unit: Unit.satblocks,
          }),
          line({
            metric: tree.activity.satdaysDestroyed,
            name: "Satdays",
            color,
            unit: Unit.satdays,
          }),
        ],
      },
    ],
  };
}

/**
 * Create activity section for grouped cohorts with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedActivitySectionWithAdjusted(list, title) {
  return {
    name: "Activity",
    tree: [
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
            title: `Coins Destroyed ${title}`,
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
    ],
  };
}

/**
 * Create activity section for grouped cohorts without adjusted values (for cohorts with RealizedPattern/2)
 * @param {readonly (CohortWithPercentiles | CohortBasic | CohortAddress)[]} list
 * @param {string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedActivitySectionBasic(list, title) {
  return {
    name: "Activity",
    tree: [
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
            title: `Coins Destroyed ${title}`,
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
    ],
  };
}
