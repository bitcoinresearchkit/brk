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
  createSingleSupplySeriesWithoutRelative,
  createGroupedSupplySection,
  createUtxoCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createCostBasisPercentilesSeries,
  createInvestedCapitalPercentilesSeries,
  createSpotPercentileSeries,
  groupedSupplyRelativeGenerators,
  createSingleSupplyRelativeOptions,
  createSingleSellSideRiskSeries,
  createGroupedSellSideRiskSeries,
  createSingleValueCreatedDestroyedSeries,
  createSingleValueFlowBreakdownSeries,
  createSingleCapitulationProfitFlowSeries,
  createSingleSoprSeries,
  createSingleCoinsDestroyedSeries,
  createInvestorPriceFolderFull,
  createInvestorPriceFolderBasic,
  createGroupedInvestorPriceFolder,
  createSingleRealizedAthRegretSeries,
  createGroupedRealizedAthRegretSeries,
  createSingleSentimentSeries,
  createGroupedNetSentimentSeries,
  createGroupedGreedIndexSeries,
  createGroupedPainIndexSeries,
} from "./shared.js";
import {
  createPriceRatioCharts,
  formatCohortTitle,
  satsBtcUsd,
} from "../shared.js";
import { Unit } from "../../utils/units.js";
import { line, baseline, price } from "../series.js";
import { priceLine } from "../constants.js";
import { colors } from "../../utils/colors.js";

// ============================================================================
// Folder Builders (4 variants based on pattern capabilities)
// ============================================================================

/**
 * All folder: for the special "All" cohort (adjustedSopr + percentiles + RelToMarketCap)
 * @param {CohortAll} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAll(args) {
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(args, title),
      createSingleUtxoCountChart(args, title),
      createSingleAddrCountChart(args, title),
      createSingleRealizedSectionFull(args, title),
      createSingleUnrealizedSectionAll(args, title),
      createSingleCostBasisSectionWithPercentiles(args, title),
      createSingleActivitySectionWithAdjusted(args, title),
    ],
  };
}

/**
 * Full folder: adjustedSopr + percentiles + RelToMarketCap (term.short only)
 * @param {CohortFull | CohortGroupFull} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderFull(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(
          list,
          title,
          groupedSupplyRelativeGenerators,
        ),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionWithAdjusted(list, title, {
          ratioMetrics: createGroupedRealizedPnlRatioMetrics,
        }),
        createGroupedUnrealizedSectionFull(list, title),
        createGroupedCostBasisSectionWithPercentiles(list, title),
        createGroupedActivitySectionWithAdjusted(list, title),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(
        args,
        title,
        createSingleSupplyRelativeOptions(args),
      ),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionFull(args, title),
      createSingleUnrealizedSectionFull(args, title),
      createSingleCostBasisSectionWithPercentiles(args, title),
      createSingleActivitySectionWithAdjusted(args, title),
    ],
  };
}

/**
 * Adjusted folder: adjustedSopr only, no percentiles (maxAge.*)
 * @param {CohortWithAdjusted | CohortGroupWithAdjusted} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithAdjusted(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(
          list,
          title,
          groupedSupplyRelativeGenerators,
        ),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionWithAdjusted(list, title),
        createGroupedUnrealizedSectionWithMarketCap(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySectionWithAdjusted(list, title),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(
        args,
        title,
        createSingleSupplyRelativeOptions(args),
      ),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionWithAdjusted(args, title),
      createSingleUnrealizedSectionWithMarketCap(args, title),
      createCostBasisSection({ cohort: args, title }),
      createSingleActivitySectionWithAdjusted(args, title),
    ],
  };
}

/**
 * Folder for cohorts with nupl + percentiles (term.short, term.long)
 * @param {CohortWithNuplPercentiles | CohortGroupWithNuplPercentiles} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithNupl(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(
          list,
          title,
          groupedSupplyRelativeGenerators,
        ),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title, {
          ratioMetrics: createGroupedRealizedPnlRatioMetrics,
        }),
        createGroupedUnrealizedSectionWithNupl({ list, title }),
        createGroupedCostBasisSectionWithPercentiles(list, title),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(
        args,
        title,
        createSingleSupplyRelativeOptions(args),
      ),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionWithPercentiles(args, title),
      createSingleUnrealizedSectionWithNupl({ cohort: args, title }),
      createSingleCostBasisSectionWithPercentiles(args, title),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Age range folder: ageRange.* (no nupl via RelativePattern2)
 * @param {CohortAgeRange | CohortGroupAgeRange} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAgeRange(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title, {
          ratioMetrics: createGroupedRealizedPnlRatioMetrics,
        }),
        createGroupedUnrealizedSectionAgeRange(list, title),
        createGroupedCostBasisSectionWithPercentiles(list, title),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionWithPercentiles(args, title),
      createSingleUnrealizedSectionAgeRange(args, title),
      createSingleCostBasisSectionWithPercentiles(args, title),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * MinAge folder - has peakRegret in unrealized (minAge.*)
 * @param {CohortMinAge | CohortGroupMinAge} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderMinAge(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(
          list,
          title,
          groupedSupplyRelativeGenerators,
        ),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title),
        createGroupedUnrealizedSectionMinAge(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(
        args,
        title,
        createSingleSupplyRelativeOptions(args),
      ),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(args, title),
      createSingleUnrealizedSectionMinAge(args, title),
      createCostBasisSection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Basic folder WITH RelToMarketCap (geAmount.*, ltAmount.*)
 * @param {CohortBasicWithMarketCap | CohortGroupBasicWithMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithMarketCap(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(
          list,
          title,
          groupedSupplyRelativeGenerators,
        ),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title),
        createGroupedUnrealizedSectionWithMarketCapOnly(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(
        args,
        title,
        createSingleSupplyRelativeOptions(args),
      ),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(args, title),
      createSingleUnrealizedSectionWithMarketCapOnly(args, title),
      createCostBasisSection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Basic folder WITHOUT RelToMarketCap (epoch.*, amountRange.*, year.*)
 * @param {CohortBasicWithoutMarketCap | CohortGroupBasicWithoutMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithoutMarketCap(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title),
        createGroupedUnrealizedSectionBase(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(args, title),
      createSingleUnrealizedSectionBase(args, title),
      createCostBasisSection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Address folder: like basic but with address count (addressable type cohorts)
 * Uses base unrealized section (no RelToMarketCap since it extends CohortBasicWithoutMarketCap)
 * @param {CohortAddress | CohortGroupAddress} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAddress(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedAddrCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title),
        createGroupedUnrealizedSectionBase(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChart(args, title),
      createSingleUtxoCountChart(args, title),
      createSingleAddrCountChart(args, title),
      createSingleRealizedSectionBasic(args, title),
      createSingleUnrealizedSectionBase(args, title),
      createCostBasisSection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Create supply chart for cohorts WITHOUT relative metrics
 * @param {CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleSupplyChartWithoutRelative(cohort, title) {
  return {
    name: "Supply",
    title: title("Supply"),
    bottom: createSingleSupplySeriesWithoutRelative(cohort),
  };
}

/**
 * Folder for cohorts WITHOUT relative section (edge case types: empty, p2ms, unknown)
 * @param {CohortWithoutRelative | CohortGroupWithoutRelative} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithoutRelative(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedSupplySection(list, title),
        createGroupedUtxoCountChart(list, title),
        createGroupedRealizedSectionBasic(list, title),
        createGroupedUnrealizedSectionWithoutRelative(list, title),
        createGroupedCostBasisSection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createSingleSupplyChartWithoutRelative(args, title),
      createSingleUtxoCountChart(args, title),
      createSingleRealizedSectionBasic(args, title),
      createSingleUnrealizedSectionWithoutRelative(args, title),
      createCostBasisSection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Create supply chart for single cohort
 * @param {UtxoCohortObject} cohort
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {AnyFetchedSeriesBlueprint[]} [options.supplyRelative] - Supply relative to circulating supply
 * @param {AnyFetchedSeriesBlueprint[]} [options.pnlRelative] - Supply in profit/loss relative to circulating supply
 * @returns {PartialChartOption}
 */
function createSingleSupplyChart(cohort, title, options = {}) {
  return {
    name: "Supply",
    title: title("Supply"),
    bottom: createSingleSupplySeries(cohort, options),
  };
}

/**
 * Create UTXO count chart for single cohort
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleUtxoCountChart(cohort, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: createUtxoCountSeries([cohort], false),
  };
}

/**
 * Create UTXO count chart for grouped cohorts
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedUtxoCountChart(list, title) {
  return {
    name: "UTXO Count",
    title: title("UTXO Count"),
    bottom: createUtxoCountSeries(list, true),
  };
}

/**
 * Create address count chart for single cohort with addrCount
 * @param {CohortAll | CohortAddress} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleAddrCountChart(cohort, title) {
  return {
    name: "Address Count",
    title: title("Address Count"),
    bottom: [
      line({
        metric: cohort.addrCount,
        name: "Count",
        color: colors.orange,
        unit: Unit.count,
      }),
    ],
  };
}

/**
 * Create address count chart for grouped cohorts with addrCount
 * @param {readonly CohortAddress[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedAddrCountChart(list, title) {
  return {
    name: "Address Count",
    title: title("Address Count"),
    bottom: list.map(({ color, name, addrCount }) =>
      line({ metric: addrCount, name, color, unit: Unit.count }),
    ),
  };
}

/**
 * Create realized section for CohortAll/CohortFull (adjustedSopr + full ratio)
 * @param {CohortAll | CohortFull} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionFull(cohort, title) {
  const { tree, color } = cohort;
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsWithRatio(cohort, title),
      createInvestorPriceFolderFull(cohort, cohort.name),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort, {
          extra: createRealizedCapRatioSeries(tree),
        }),
      },
      ...createSingleRealizedPnlSection(cohort, title, {
        extra: createRealizedPnlRatioSeries(colors, tree),
      }),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createSingleRealizedAthRegretSeries(tree, color),
      },
      createSingleSoprSectionWithAdjusted(cohort, title),
    ],
  };
}

/**
 * Create realized section for CohortWithAdjusted (adjustedSopr but partial ratio)
 * @param {CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionWithAdjusted(cohort, title) {
  const { tree, color } = cohort;
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsBasic(cohort, title),
      createInvestorPriceFolderBasic(cohort, cohort.name),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort),
      },
      ...createSingleRealizedPnlSection(cohort, title),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createSingleRealizedAthRegretSeries(tree, color),
      },
      createSingleSoprSectionWithAdjusted(cohort, title),
    ],
  };
}

/**
 * Create realized section with adjusted SOPR for grouped cohorts
 * @template {readonly (CohortFull | CohortWithAdjusted)[]} T
 * @param {T} list
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.ratioMetrics] - Generator for ratio metrics per cohort
 * @returns {PartialOptionsGroup}
 */
function createGroupedRealizedSectionWithAdjusted(
  list,
  title,
  { ratioMetrics } = {},
) {
  return {
    name: "Realized",
    tree: [
      {
        name: "Price",
        title: title("Realized Price"),
        top: createRealizedPriceSeries(list),
      },
      {
        name: "Ratio",
        title: title("Realized Price Ratio"),
        bottom: createRealizedPriceRatioSeries(list),
      },
      createGroupedInvestorPriceFolder(list, title),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createGroupedRealizedCapSeries(list),
      },
      ...createGroupedRealizedPnlSections(list, title, { ratioMetrics }),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createGroupedRealizedAthRegretSeries(list),
      },
      createGroupedSoprSectionWithAdjusted(list, title),
    ],
  };
}

/**
 * Create realized section for CohortWithPercentiles (no adjustedSopr but full ratio)
 * @param {CohortWithPercentiles} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionWithPercentiles(cohort, title) {
  const { tree, color } = cohort;
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsWithRatio(cohort, title),
      createInvestorPriceFolderFull(cohort, cohort.name),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort, {
          extra: createRealizedCapRatioSeries(tree),
        }),
      },
      ...createSingleRealizedPnlSection(cohort, title, {
        extra: createRealizedPnlRatioSeries(colors, tree),
      }),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createSingleRealizedAthRegretSeries(tree, color),
      },
      createSingleSoprSectionBasic(cohort, title),
    ],
  };
}

/**
 * Create realized section for CohortBasic (no adjustedSopr, partial ratio)
 * @param {CohortBasic | CohortAddress | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleRealizedSectionBasic(cohort, title) {
  const { tree, color } = cohort;
  return {
    name: "Realized",
    tree: [
      ...createSingleRealizedPriceChartsBasic(cohort, title),
      createInvestorPriceFolderBasic(cohort, cohort.name),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort),
      },
      ...createSingleRealizedPnlSection(cohort, title),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createSingleRealizedAthRegretSeries(tree, color),
      },
      createSingleSoprSectionBasic(cohort, title),
    ],
  };
}

/**
 * Create realized section without adjusted SOPR for grouped cohorts
 * @template {readonly (CohortWithPercentiles | CohortBasic | CohortAddress | CohortWithoutRelative)[]} T
 * @param {T} list
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.ratioMetrics] - Generator for ratio metrics per cohort
 * @returns {PartialOptionsGroup}
 */
function createGroupedRealizedSectionBasic(list, title, { ratioMetrics } = {}) {
  return {
    name: "Realized",
    tree: [
      {
        name: "Price",
        title: title("Realized Price"),
        top: createRealizedPriceSeries(list),
      },
      {
        name: "Ratio",
        title: title("Realized Price Ratio"),
        bottom: createRealizedPriceRatioSeries(list),
      },
      createGroupedInvestorPriceFolder(list, title),
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createGroupedRealizedCapSeries(list),
      },
      ...createGroupedRealizedPnlSections(list, title, { ratioMetrics }),
      {
        name: "Peak Regret",
        title: title("Realized Peak Regret"),
        bottom: createGroupedRealizedAthRegretSeries(list),
      },
      createGroupedSoprSectionBasic(list, title),
    ],
  };
}

/**
 * Create realized price chart for single cohort
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleRealizedPriceChart(cohort, title) {
  const { tree, color } = cohort;
  return {
    name: "Price",
    title: title("Realized Price"),
    top: [
      price({
        metric: tree.realized.realizedPrice,
        name: "Realized",
        color,
      }),
    ],
  };
}

/**
 * Create realized price and ratio charts for cohorts with full ActivePriceRatioPattern
 * (CohortAll, CohortFull, CohortWithPercentiles have RealizedPattern2/3 which has ActivePriceRatioPattern)
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createSingleRealizedPriceChartsWithRatio(cohort, title) {
  const { tree, color } = cohort;
  const ratio = /** @type {ActivePriceRatioPattern} */ (
    tree.realized.realizedPriceExtra
  );
  return createPriceRatioCharts({
    context: cohort.name,
    legend: "Realized",
    pricePattern: tree.realized.realizedPrice,
    ratio,
    color,
    ratioName: "MVRV",
    priceTitle: title("Realized Price"),
    zScoresSuffix: "MVRV",
  });
}

/**
 * Create realized price and basic ratio charts for cohorts with RealizedPriceExtraPattern
 * (CohortWithAdjusted, CohortBasic have RealizedPattern/4 which has RealizedPriceExtraPattern)
 * @param {CohortWithAdjusted | CohortBasic | CohortAddress | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption[]}
 */
function createSingleRealizedPriceChartsBasic(cohort, title) {
  const { tree, color } = cohort;
  return [
    createSingleRealizedPriceChart(cohort, title),
    {
      name: "Ratio",
      title: title("Realized Price Ratio"),
      bottom: [
        baseline({
          metric: tree.realized.realizedPriceExtra.ratio,
          name: "Ratio",
          color,
          unit: Unit.ratio,
        }),
      ],
    },
  ];
}

/**
 * Create realized cap series for single cohort
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {Object} [options]
 * @param {AnyFetchedSeriesBlueprint[]} [options.extra] - Additional series (e.g., ratio for cohorts with RealizedWithCapRatio)
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createSingleRealizedCapSeries(cohort, { extra = [] } = {}) {
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
      name: "30d Change",
      unit: Unit.usd,
      defaultActive: false,
    }),
    ...extra,
  ];
}

/**
 * Create realized cap ratio series (for cohorts with RealizedPattern2 or RealizedPattern3)
 * @param {{ realized: RealizedWithExtras }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedCapRatioSeries(tree) {
  return [
    baseline({
      metric: tree.realized.realizedCapRelToOwnMarketCap,
      name: "Ratio",
      unit: Unit.pctOwnMcap,
      options: { baseValue: { price: 100 } },
    }),
    priceLine({
      unit: Unit.pctOwnMcap,
      defaultActive: true,
      number: 100,
    }),
  ];
}

/**
 * Create realized cap series for grouped cohorts
 * @param {readonly (UtxoCohortObject | CohortWithoutRelative)[]} list
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
 * Create realized PnL ratio series (for cohorts with RealizedPattern2 or RealizedPattern3)
 * @param {Colors} colors
 * @param {{ realized: RealizedWithExtras }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedPnlRatioSeries(colors, tree) {
  return [
    line({
      metric: tree.realized.realizedProfitToLossRatio,
      name: "P/L Ratio",
      color: colors.yellow,
      unit: Unit.ratio,
    }),
  ];
}

/**
 * Create realized PnL ratio metrics generator for grouped cohorts with RealizedWithExtras
 * @param {CohortWithRealizedExtras} cohort
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createGroupedRealizedPnlRatioMetrics(cohort) {
  return [
    line({
      metric: cohort.tree.realized.realizedProfitToLossRatio,
      name: cohort.name,
      color: cohort.color,
      unit: Unit.ratio,
    }),
  ];
}

/**
 * Create realized PnL section for single cohort
 * @param {UtxoCohortObject | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {AnyFetchedSeriesBlueprint[]} [options.extra] - Extra series (e.g., pnl ratio for cohorts with RealizedWithPnlRatio)
 * @returns {PartialOptionsTree}
 */
function createSingleRealizedPnlSection(cohort, title, { extra = [] } = {}) {
  const { tree } = cohort;

  return [
    {
      name: "P&L",
      tree: [
        {
          name: "Sum",
          title: title("Realized P&L"),
          bottom: [
            line({
              metric: tree.realized.realizedProfit.sum,
              name: "Profit",
              color: colors.green,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedProfit7dEma,
              name: "Profit 7d EMA",
              color: colors.green,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss.sum,
              name: "Loss",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss7dEma,
              name: "Loss 7d EMA",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.negRealizedLoss.sum,
              name: "Negative Loss",
              color: colors.red,
              unit: Unit.usd,
              defaultActive: false,
            }),
            ...extra,
            line({
              metric: tree.realized.totalRealizedPnl,
              name: "Total",
              color: colors.default,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Realized P&L (Total)"),
          bottom: [
            line({
              metric: tree.realized.realizedProfit.cumulative,
              name: "Profit",
              color: colors.green,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss.cumulative,
              name: "Loss",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.negRealizedLoss.cumulative,
              name: "Negative Loss",
              color: colors.red,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
      ],
    },
    {
      name: "P&L Relative",
      title: title("Realized P&L"),
      bottom: [
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
      ],
    },
    {
      name: "Net P&L",
      tree: [
        {
          name: "Sum",
          title: title("Net Realized P&L"),
          bottom: [
            baseline({
              metric: tree.realized.netRealizedPnl.sum,
              name: "Net",
              unit: Unit.usd,
            }),
            baseline({
              metric: tree.realized.netRealizedPnl7dEma,
              name: "Net 7d EMA",
              unit: Unit.usd,
            }),
            baseline({
              metric: tree.realized.netRealizedPnlRelToRealizedCap.sum,
              name: "Net",
              unit: Unit.pctRcap,
            }),
            baseline({
              metric:
                tree.realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
              name: "30d Change",
              unit: Unit.pctMcap,
            }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Net Realized P&L (Total)"),
          bottom: [
            baseline({
              metric: tree.realized.netRealizedPnl.cumulative,
              name: "Net",
              unit: Unit.usd,
            }),
            baseline({
              metric: tree.realized.netRealizedPnlCumulative30dDelta,
              name: "30d Change",
              unit: Unit.usd,
              defaultActive: false,
            }),
            baseline({
              metric: tree.realized.netRealizedPnlRelToRealizedCap.cumulative,
              name: "Net",
              unit: Unit.pctRcap,
            }),
            baseline({
              metric:
                tree.realized.netRealizedPnlCumulative30dDeltaRelToRealizedCap,
              name: "30d Change",
              unit: Unit.pctRcap,
              defaultActive: false,
            }),
          ],
        },
      ],
    },
    {
      name: "Sent In P/L",
      tree: [
        {
          name: "In Profit",
          title: title("Sent In Profit"),
          bottom: [
            line({
              metric: tree.realized.sentInProfit.bitcoin.sum,
              name: "Sum",
              color: colors.green,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.green,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.sum,
              name: "Sum",
              color: colors.green,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.cumulative,
              name: "Cumulative",
              color: colors.green,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.sum,
              name: "Sum",
              color: colors.green,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.cumulative,
              name: "Cumulative",
              color: colors.green,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "In Loss",
          title: title("Sent In Loss"),
          bottom: [
            line({
              metric: tree.realized.sentInLoss.bitcoin.sum,
              name: "Sum",
              color: colors.red,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.red,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.sum,
              name: "Sum",
              color: colors.red,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.cumulative,
              name: "Cumulative",
              color: colors.red,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.sum,
              name: "Sum",
              color: colors.red,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.cumulative,
              name: "Cumulative",
              color: colors.red,
              unit: Unit.usd,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "In Profit 14d EMA",
          title: title("Sent In Profit 14d EMA"),
          bottom: satsBtcUsd({
            pattern: tree.realized.sentInProfit14dEma,
            name: "14d EMA",
            color: colors.green,
          }),
        },
        {
          name: "In Loss 14d EMA",
          title: title("Sent In Loss 14d EMA"),
          bottom: satsBtcUsd({
            pattern: tree.realized.sentInLoss14dEma,
            name: "14d EMA",
            color: colors.red,
          }),
        },
      ],
    },
  ];
}

/**
 * Create realized PnL sections for grouped cohorts
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {T} list
 * @param {(metric: string) => string} title
 * @param {Object} [options]
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [options.ratioMetrics] - Generator for ratio metrics per cohort
 * @returns {PartialOptionsTree}
 */
function createGroupedRealizedPnlSections(list, title, { ratioMetrics } = {}) {
  const pnlConfigs = /** @type {const} */ ([
    {
      name: "Profit",
      sum: "realizedProfit",
      ema: "realizedProfit7dEma",
      rel: "realizedProfitRelToRealizedCap",
      isNet: false,
    },
    {
      name: "Loss",
      sum: "realizedLoss",
      ema: "realizedLoss7dEma",
      rel: "realizedLossRelToRealizedCap",
      isNet: false,
    },
    {
      name: "Net P&L",
      sum: "netRealizedPnl",
      ema: "netRealizedPnl7dEma",
      rel: "netRealizedPnlRelToRealizedCap",
      isNet: true,
    },
  ]);

  return [
    ...pnlConfigs.map(({ name, sum, ema, rel, isNet }) => ({
      name,
      tree: [
        {
          name: "Sum",
          title: title(`Realized ${name}`),
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              (isNet ? baseline : line)({
                metric: tree.realized[sum].sum,
                name,
                color,
                unit: Unit.usd,
              }),
              baseline({
                metric: tree.realized[rel].sum,
                name,
                color,
                unit: Unit.pctRcap,
              }),
            ]),
          ],
        },
        {
          name: "7d EMA",
          title: title(`Realized ${name} 7d EMA`),
          bottom: [
            ...list.map(({ color, name, tree }) =>
              (isNet ? baseline : line)({
                metric: tree.realized[ema],
                name,
                color,
                unit: Unit.usd,
              }),
            ),
          ],
        },
      ],
    })),
    {
      name: "Total P&L",
      title: title("Total Realized P&L"),
      bottom: list.flatMap((cohort) => [
        line({
          metric: cohort.tree.realized.totalRealizedPnl,
          name: cohort.name,
          color: cohort.color,
          unit: Unit.usd,
        }),
        ...(ratioMetrics ? ratioMetrics(cohort) : []),
      ]),
    },
    {
      name: "Cumulative",
      tree: [
        {
          name: "Profit",
          title: title("Cumulative Realized Profit"),
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
          name: "Loss",
          title: title("Cumulative Realized Loss"),
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
          name: "Net P&L",
          title: title("Cumulative Net Realized P&L"),
          bottom: [
            ...list.flatMap(({ color, name, tree }) => [
              baseline({
                metric: tree.realized.netRealizedPnl.cumulative,
                name,
                color,
                unit: Unit.usd,
              }),
            ]),
          ],
        },
        {
          name: "Net P&L 30d Change",
          title: title("Net Realized P&L 30d Change"),
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
          ],
        },
      ],
    },
    {
      name: "Sent In P/L",
      tree: [
        {
          name: "In Profit",
          title: title("Sent In Profit"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInProfit.bitcoin.sum,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.sum,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.sum,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Profit Cumulative",
          title: title("Sent In Profit Cumulative"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInProfit.bitcoin.cumulative,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.cumulative,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Loss",
          title: title("Sent In Loss"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInLoss.bitcoin.sum,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.sum,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.sum,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Loss Cumulative",
          title: title("Sent In Loss Cumulative"),
          bottom: list.flatMap(({ color, name, tree }) => [
            line({
              metric: tree.realized.sentInLoss.bitcoin.cumulative,
              name,
              color,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.cumulative,
              name,
              color,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.cumulative,
              name,
              color,
              unit: Unit.usd,
            }),
          ]),
        },
        {
          name: "In Profit 14d EMA",
          title: title("Sent In Profit 14d EMA"),
          bottom: list.flatMap(({ color, name, tree }) =>
            satsBtcUsd({
              pattern: tree.realized.sentInProfit14dEma,
              name,
              color,
            }),
          ),
        },
        {
          name: "In Loss 14d EMA",
          title: title("Sent In Loss 14d EMA"),
          bottom: list.flatMap(({ color, name, tree }) =>
            satsBtcUsd({
              pattern: tree.realized.sentInLoss14dEma,
              name,
              color,
            }),
          ),
        },
      ],
    },
  ];
}

// ============================================================================
// SOPR Chart Builders (Composable)
// ============================================================================

/**
 * Create single base SOPR chart (all UTXO cohorts have base SOPR)
 * @param {CohortAll | CohortFull | CohortWithAdjusted | CohortLongTerm | CohortAgeRange | CohortBasicWithMarketCap | CohortBasicWithoutMarketCap | CohortAddress | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleBaseSoprChart(cohort, title) {
  return {
    name: "Normal",
    title: title("SOPR"),
    bottom: createSingleSoprSeries(colors, cohort.tree),
  };
}

/**
 * Create single adjusted SOPR chart (cohorts with RealizedPattern3/4)
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleAdjustedSoprChart(cohort, title) {
  const { tree } = cohort;

  return {
    name: "Adjusted",
    title: title("aSOPR"),
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
    ],
  };
}

/**
 * Create grouped base SOPR chart (all UTXO cohorts have base SOPR)
 * @param {readonly (CohortAll | CohortFull | CohortWithAdjusted | CohortLongTerm | CohortAgeRange | CohortBasicWithMarketCap | CohortBasicWithoutMarketCap | CohortAddress | CohortWithoutRelative)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedBaseSoprChart(list, title) {
  return {
    name: "Normal",
    title: title("SOPR"),
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
    ],
  };
}

/**
 * Create grouped adjusted SOPR chart (cohorts with RealizedPattern3/4)
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedAdjustedSoprChart(list, title) {
  return {
    name: "Adjusted",
    title: title("aSOPR"),
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
    ],
  };
}

// ============================================================================
// SOPR Section Composers
// ============================================================================

/**
 * Create SOPR section with adjusted SOPR (for cohorts with RealizedPattern3/4)
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleSoprSectionWithAdjusted(cohort, title) {
  return {
    name: "SOPR",
    tree: [
      createSingleBaseSoprChart(cohort, title),
      createSingleAdjustedSoprChart(cohort, title),
    ],
  };
}

/**
 * Create grouped SOPR section with adjusted SOPR
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSoprSectionWithAdjusted(list, title) {
  return {
    name: "SOPR",
    tree: [
      createGroupedBaseSoprChart(list, title),
      createGroupedAdjustedSoprChart(list, title),
    ],
  };
}

/**
 * Create SOPR section without adjusted SOPR (for cohorts with RealizedPattern/2)
 * @param {CohortWithPercentiles | CohortBasic | CohortAddress | CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleSoprSectionBasic(cohort, title) {
  return {
    name: "SOPR",
    tree: [createSingleBaseSoprChart(cohort, title)],
  };
}

/**
 * Create grouped SOPR section without adjusted SOPR
 * @param {readonly (CohortWithPercentiles | CohortBasic | CohortAddress | CohortWithoutRelative)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedSoprSectionBasic(list, title) {
  return {
    name: "SOPR",
    tree: [createGroupedBaseSoprChart(list, title)],
  };
}

// ============================================================================
// Unrealized Section Helpers (by relative pattern capability)
// ============================================================================

/**
 * @param {RelativeWithMarketCap} rel
 */
function createUnrealizedPnlRelToMarketCapMetrics(rel) {
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
      defaultActive: false,
    }),
  ];
}

/**
 * @param {RelativeWithOwnMarketCap} rel
 */
function createUnrealizedPnlRelToOwnMarketCapMetrics(rel) {
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
      defaultActive: false,
    }),
    priceLine({ unit: Unit.pctOwnMcap, number: 100 }),
    priceLine({ unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {RelativeWithOwnPnl} rel
 */
function createUnrealizedPnlRelToOwnPnlMetrics(rel) {
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
      defaultActive: false,
    }),
    priceLine({ unit: Unit.pctOwnPnl, number: 100 }),
    priceLine({ unit: Unit.pctOwnPnl }),
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
 * @param {RelativeWithOwnMarketCap} rel
 */
function createNetUnrealizedPnlRelToOwnMarketCapMetrics(rel) {
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnMarketCap,
      name: "Net",
      unit: Unit.pctOwnMcap,
    }),
    priceLine({ unit: Unit.pctOwnMcap }),
  ];
}

/**
 * @param {RelativeWithOwnPnl} rel
 */
function createNetUnrealizedPnlRelToOwnPnlMetrics(rel) {
  return [
    baseline({
      metric: rel.netUnrealizedPnlRelToOwnTotalUnrealizedPnl,
      name: "Net",
      unit: Unit.pctOwnPnl,
    }),
  ];
}

/**
 * Create invested capital relative metrics (% of realized cap)
 * @param {RelativeWithInvestedCapitalPct} rel
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createInvestedCapitalRelMetrics(rel) {
  return [
    baseline({
      metric: rel.investedCapitalInProfitPct,
      name: "In Profit",
      color: colors.green,
      unit: Unit.pctRcap,
    }),
    baseline({
      metric: rel.investedCapitalInLossPct,
      name: "In Loss",
      color: colors.red,
      unit: Unit.pctRcap,
    }),
  ];
}

/**
 * Base unrealized metrics (always present)
 * @param {{ unrealized: UnrealizedPattern }} tree
 */
function createUnrealizedPnlBaseMetrics(tree) {
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
      defaultActive: false,
    }),
  ];
}

/**
 * Base net unrealized metric (always present)
 * @param {{ unrealized: UnrealizedPattern }} tree
 */
function createNetUnrealizedPnlBaseMetric(tree) {
  return baseline({
    metric: tree.unrealized.netUnrealizedPnl,
    name: "Net",
    unit: Unit.usd,
  });
}

// ============================================================================
// Unrealized Chart Builders (composable charts)
// ============================================================================

/**
 * Create NUPL chart for single cohort
 * @param {RelativeWithNupl} rel
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createNuplChart(rel, title) {
  return {
    name: "NUPL",
    title: title("NUPL"),
    bottom: [
      baseline({
        metric: rel.nupl,
        name: "NUPL",
        unit: Unit.ratio,
      }),
    ],
  };
}

/**
 * Create peak regret chart (basic - just absolute value)
 * @param {{ unrealized: UnrealizedFullPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createPeakRegretChart(tree, title) {
  return {
    name: "Peak Regret",
    title: title("Unrealized Peak Regret"),
    bottom: [
      line({
        metric: tree.unrealized.peakRegret,
        name: "Peak Regret",
        color: colors.orange,
        unit: Unit.usd,
      }),
    ],
  };
}

/**
 * Create peak regret chart with RelToMarketCap metric
 * @param {{ unrealized: UnrealizedFullPattern, relative: RelativeWithMarketCap }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createPeakRegretChartWithMarketCap(tree, title) {
  return {
    name: "Peak Regret",
    title: title("Unrealized Peak Regret"),
    bottom: [
      line({
        metric: tree.unrealized.peakRegret,
        name: "Peak Regret",
        color: colors.orange,
        unit: Unit.usd,
      }),
      baseline({
        metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
        name: "Peak Regret",
        color: colors.orange,
        unit: Unit.pctMcap,
      }),
    ],
  };
}

/**
 * Create invested capital absolute chart
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleInvestedCapitalAbsoluteChart(tree, title) {
  return {
    name: "Absolute",
    title: title("Invested Capital In Profit & Loss"),
    bottom: [
      line({
        metric: tree.unrealized.investedCapitalInProfit,
        name: "In Profit",
        color: colors.green,
        unit: Unit.usd,
      }),
      line({
        metric: tree.unrealized.investedCapitalInLoss,
        name: "In Loss",
        color: colors.red,
        unit: Unit.usd,
      }),
    ],
  };
}

/**
 * Create invested capital relative chart
 * @param {RelativeWithInvestedCapitalPct} rel
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createSingleInvestedCapitalRelativeChart(rel, title) {
  return {
    name: "Relative",
    title: title("Invested Capital In Profit & Loss %"),
    bottom: [...createInvestedCapitalRelMetrics(rel)],
  };
}

/**
 * Create invested capital folder for cohorts WITHOUT relative metrics
 * @param {{ unrealized: UnrealizedPattern }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleInvestedCapitalFolder(tree, title) {
  return {
    name: "Invested Capital",
    tree: [createSingleInvestedCapitalAbsoluteChart(tree, title)],
  };
}

/**
 * Create invested capital folder for cohorts WITH relative metrics
 * @param {{ unrealized: UnrealizedPattern, relative: RelativeWithInvestedCapitalPct }} tree
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleInvestedCapitalFolderFull(tree, title) {
  return {
    name: "Invested Capital",
    tree: [
      createSingleInvestedCapitalAbsoluteChart(tree, title),
      createSingleInvestedCapitalRelativeChart(tree.relative, title),
    ],
  };
}

/**
 * Create NUPL chart for grouped cohorts
 * @param {readonly { name: string, color: Color, tree: { relative: RelativeWithNupl } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedNuplChart(list, title) {
  return {
    name: "NUPL",
    title: title("NUPL"),
    bottom: [
      ...list.map(({ color, name, tree }) =>
        baseline({
          metric: tree.relative.nupl,
          name,
          color,
          unit: Unit.ratio,
        }),
      ),
    ],
  };
}

/**
 * Create grouped peak regret chart (basic - no RelToMarketCap)
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedFullPattern } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedPeakRegretChartBasic(list, title) {
  return {
    name: "Peak Regret",
    title: title("Unrealized Peak Regret"),
    bottom: list.flatMap(({ color, name, tree }) => [
      line({
        metric: tree.unrealized.peakRegret,
        name,
        color,
        unit: Unit.usd,
      }),
    ]),
  };
}

/**
 * Create grouped peak regret chart with RelToMarketCap metric
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedFullPattern, relative: RelativeWithMarketCap } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption}
 */
function createGroupedPeakRegretChart(list, title) {
  return {
    name: "Peak Regret",
    title: title("Unrealized Peak Regret"),
    bottom: list.flatMap(({ color, name, tree }) => [
      line({
        metric: tree.unrealized.peakRegret,
        name,
        color,
        unit: Unit.usd,
      }),
      baseline({
        metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
        name,
        color,
        unit: Unit.pctMcap,
      }),
    ]),
  };
}

// ============================================================================
// Unrealized Section Builder (generic, type-safe composition)
// ============================================================================

/**
 * Generic single unrealized section builder - callers pass typed metrics
 * @param {Object} args
 * @param {{ unrealized: UnrealizedPattern }} args.tree
 * @param {(metric: string) => string} args.title
 * @param {AnyFetchedSeriesBlueprint[]} [args.pnl] - Extra pnl metrics
 * @param {AnyFetchedSeriesBlueprint[]} [args.netPnl] - Extra net pnl metrics
 * @param {PartialOptionsGroup} args.investedCapitalFolder - Invested capital folder (use createSingleInvestedCapitalFolder or createSingleInvestedCapitalFolderFull)
 * @param {PartialChartOption[]} [args.charts] - Extra charts (e.g., nupl)
 * @returns {PartialOptionsGroup}
 */
function createUnrealizedSection({
  tree,
  title,
  pnl = [],
  netPnl = [],
  investedCapitalFolder,
  charts = [],
}) {
  return {
    name: "Unrealized",
    tree: [
      {
        name: "P&L",
        title: title("Unrealized P&L"),
        bottom: [...createUnrealizedPnlBaseMetrics(tree), ...pnl],
      },
      {
        name: "Net P&L",
        title: title("Net Unrealized P&L"),
        bottom: [createNetUnrealizedPnlBaseMetric(tree), ...netPnl],
      },
      investedCapitalFolder,
      {
        name: "Sentiment",
        title: title("Market Sentiment"),
        bottom: createSingleSentimentSeries(colors, tree),
      },
      ...charts,
    ],
  };
}

/**
 * Create grouped invested capital absolute charts (In Profit, In Loss)
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption[]}
 */
function createGroupedInvestedCapitalAbsoluteCharts(list, title) {
  return [
    {
      name: "In Profit",
      title: title("Invested Capital In Profit"),
      bottom: list.map(({ color, name, tree }) =>
        line({
          metric: tree.unrealized.investedCapitalInProfit,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
    {
      name: "In Loss",
      title: title("Invested Capital In Loss"),
      bottom: list.map(({ color, name, tree }) =>
        line({
          metric: tree.unrealized.investedCapitalInLoss,
          name,
          color,
          unit: Unit.usd,
        }),
      ),
    },
  ];
}

/**
 * Create grouped invested capital relative charts (In Profit %, In Loss %)
 * @param {readonly { color: Color, name: string, tree: { relative: RelativeWithInvestedCapitalPct } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialChartOption[]}
 */
function createGroupedInvestedCapitalRelativeCharts(list, title) {
  return [
    {
      name: "In Profit %",
      title: title("Invested Capital In Profit %"),
      bottom: list.map(({ color, name, tree }) =>
        baseline({
          metric: tree.relative.investedCapitalInProfitPct,
          name,
          color,
          unit: Unit.pctRcap,
        }),
      ),
    },
    {
      name: "In Loss %",
      title: title("Invested Capital In Loss %"),
      bottom: list.map(({ color, name, tree }) =>
        baseline({
          metric: tree.relative.investedCapitalInLossPct,
          name,
          color,
          unit: Unit.pctRcap,
        }),
      ),
    },
  ];
}

/**
 * Generic grouped unrealized section builder - callers pass typed metric generators
 * @template {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern, relative: RelativeWithInvestedCapitalPct } }[]} T
 * @param {Object} args
 * @param {T} args.list
 * @param {(metric: string) => string} args.title
 * @param {(cohort: T[number]) => AnyFetchedSeriesBlueprint[]} [args.netPnlMetrics] - Generator for extra net pnl metrics per cohort
 * @param {PartialChartOption[]} [args.charts] - Extra charts
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSection({
  list,
  title,
  netPnlMetrics,
  charts = [],
}) {
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net P&L",
        title: title("Net Unrealized P&L"),
        bottom: [
          ...list.flatMap((cohort) => [
            baseline({
              metric: cohort.tree.unrealized.netUnrealizedPnl,
              name: cohort.name,
              color: cohort.color,
              unit: Unit.usd,
            }),
            ...(netPnlMetrics ? netPnlMetrics(cohort) : []),
          ]),
        ],
      },
      {
        name: "Invested Capital",
        tree: [
          ...createGroupedInvestedCapitalAbsoluteCharts(list, title),
          ...createGroupedInvestedCapitalRelativeCharts(list, title),
        ],
      },
      {
        name: "Sentiment",
        tree: [
          {
            name: "Net",
            title: title("Net Sentiment"),
            bottom: createGroupedNetSentimentSeries(list),
          },
          {
            name: "Greed",
            title: title("Greed Index"),
            bottom: createGroupedGreedIndexSeries(list),
          },
          {
            name: "Pain",
            title: title("Pain Index"),
            bottom: createGroupedPainIndexSeries(list),
          },
        ],
      },
      ...charts,
    ],
  };
}

/**
 * Grouped unrealized section for cohorts WITHOUT relative (edge case types: empty, p2ms, unknown)
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedUnrealizedSectionWithoutRelative(list, title) {
  return {
    name: "Unrealized",
    tree: [
      ...createGroupedUnrealizedBaseCharts(list, title),
      {
        name: "Net P&L",
        title: title("Net Unrealized P&L"),
        bottom: list.map((cohort) =>
          baseline({
            metric: cohort.tree.unrealized.netUnrealizedPnl,
            name: cohort.name,
            color: cohort.color,
            unit: Unit.usd,
          }),
        ),
      },
      {
        name: "Invested Capital",
        tree: createGroupedInvestedCapitalAbsoluteCharts(list, title),
      },
      {
        name: "Sentiment",
        tree: [
          {
            name: "Net",
            title: title("Net Sentiment"),
            bottom: createGroupedNetSentimentSeries(list),
          },
          {
            name: "Greed",
            title: title("Greed Index"),
            bottom: createGroupedGreedIndexSeries(list),
          },
          {
            name: "Pain",
            title: title("Pain Index"),
            bottom: createGroupedPainIndexSeries(list),
          },
        ],
      },
    ],
  };
}

// ============================================================================
// Unrealized Section Variants (by cohort capability)
// ============================================================================

/**
 * Unrealized section for All cohort (only RelToOwnPnl)
 * @param {CohortAll} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionAll(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [
      ...createUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    netPnl: [
      ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [
      createNuplChart(tree.relative, title),
      createPeakRegretChartWithMarketCap(tree, title),
    ],
  });
}

/**
 * Unrealized section for Full cohort (all capabilities: MarketCap + OwnMarketCap + OwnPnl)
 * @param {CohortFull} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionFull(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [
      ...createUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    netPnl: [
      ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [
      createNuplChart(tree.relative, title),
      createPeakRegretChartWithMarketCap(tree, title),
    ],
  });
}

/**
 * Unrealized section for WithAdjusted cohort (MarketCap + nupl)
 * @param {CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionWithMarketCap(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [...createUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    netPnl: [...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [
      createNuplChart(tree.relative, title),
      createPeakRegretChartWithMarketCap(tree, title),
    ],
  });
}

/**
 * Unrealized section WITH RelToMarketCap metrics (for CohortBasicWithMarketCap)
 * @param {CohortBasicWithMarketCap} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionWithMarketCapOnly(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [...createUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    netPnl: [...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [createNuplChart(tree.relative, title)],
  });
}

/**
 * Unrealized section for minAge cohorts (has peakRegret)
 * @param {CohortMinAge} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionMinAge(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [...createUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    netPnl: [...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative)],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [
      createNuplChart(tree.relative, title),
      createPeakRegretChartWithMarketCap(tree, title),
    ],
  });
}

/**
 * Unrealized section with only base metrics (no RelToMarketCap)
 * @param {CohortBasicWithoutMarketCap} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionBase(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    investedCapitalFolder: createSingleInvestedCapitalFolder(tree, title),
  });
}

/**
 * Unrealized section for cohorts WITHOUT relative (edge case types: empty, p2ms, unknown)
 * @param {CohortWithoutRelative} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionWithoutRelative(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    investedCapitalFolder: createSingleInvestedCapitalFolder(tree, title),
  });
}

/**
 * Grouped unrealized base charts (profit, loss, total pnl)
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedPattern } }[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedBaseCharts(list, title) {
  return [
    {
      name: "Profit",
      title: title("Unrealized Profit"),
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
      name: "Loss",
      title: title("Unrealized Loss"),
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
      name: "Total P&L",
      title: title("Unrealized Total P&L"),
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
 * @param {readonly CohortFull[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionFull(list, title) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
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
    ],
    charts: [
      createGroupedNuplChart(list, title),
      createGroupedPeakRegretChart(list, title),
    ],
  });
}

/**
 * Grouped unrealized section for WithAdjusted cohorts (MarketCap + nupl)
 * @param {readonly CohortWithAdjusted[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionWithMarketCap(list, title) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
      baseline({
        metric: tree.relative.netUnrealizedPnlRelToMarketCap,
        name,
        color,
        unit: Unit.pctMcap,
      }),
    ],
    charts: [
      createGroupedNuplChart(list, title),
      createGroupedPeakRegretChart(list, title),
    ],
  });
}

/**
 * Grouped unrealized section WITH RelToMarketCap (for CohortBasicWithMarketCap)
 * @param {readonly CohortBasicWithMarketCap[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionWithMarketCapOnly(list, title) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
      baseline({
        metric: tree.relative.netUnrealizedPnlRelToMarketCap,
        name,
        color,
        unit: Unit.pctMcap,
      }),
    ],
    charts: [createGroupedNuplChart(list, title)],
  });
}

/**
 * Grouped unrealized section for minAge cohorts (has peakRegret)
 * @param {readonly CohortMinAge[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionMinAge(list, title) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
      baseline({
        metric: tree.relative.netUnrealizedPnlRelToMarketCap,
        name,
        color,
        unit: Unit.pctMcap,
      }),
    ],
    charts: [
      createGroupedNuplChart(list, title),
      createGroupedPeakRegretChart(list, title),
    ],
  });
}

/**
 * Grouped unrealized section without RelToMarketCap (for CohortBasicWithoutMarketCap)
 * @param {readonly CohortBasicWithoutMarketCap[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionBase(list, title) {
  return createGroupedUnrealizedSection({ list, title });
}

/**
 * Unrealized section for cohorts with nupl (OwnMarketCap + OwnPnl + nupl)
 * @param {Object} args
 * @param {CohortWithNuplPercentiles} args.cohort
 * @param {(metric: string) => string} args.title
 */
function createSingleUnrealizedSectionWithNupl({ cohort, title }) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [
      ...createUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    netPnl: [
      ...createNetUnrealizedPnlRelToMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [
      createNuplChart(tree.relative, title),
      createPeakRegretChartWithMarketCap(tree, title),
    ],
  });
}

/**
 * Grouped unrealized section for cohorts with nupl (OwnMarketCap + OwnPnl + nupl)
 * @param {Object} args
 * @param {readonly CohortWithNuplPercentiles[]} args.list
 * @param {(metric: string) => string} args.title
 */
function createGroupedUnrealizedSectionWithNupl({ list, title }) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
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
    ],
    charts: [
      createGroupedNuplChart(list, title),
      createGroupedPeakRegretChart(list, title),
    ],
  });
}

/**
 * Unrealized section for AgeRange cohort (no nupl via RelativePattern2)
 * @param {CohortAgeRange} cohort
 * @param {(metric: string) => string} title
 */
function createSingleUnrealizedSectionAgeRange(cohort, title) {
  const { tree } = cohort;
  return createUnrealizedSection({
    tree,
    title,
    pnl: [
      ...createUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    netPnl: [
      ...createNetUnrealizedPnlRelToOwnMarketCapMetrics(tree.relative),
      ...createNetUnrealizedPnlRelToOwnPnlMetrics(tree.relative),
    ],
    investedCapitalFolder: createSingleInvestedCapitalFolderFull(tree, title),
    charts: [createPeakRegretChart(tree, title)],
  });
}

/**
 * Grouped unrealized section for AgeRange cohorts (no nupl via RelativePattern2)
 * @param {readonly CohortAgeRange[]} list
 * @param {(metric: string) => string} title
 */
function createGroupedUnrealizedSectionAgeRange(list, title) {
  return createGroupedUnrealizedSection({
    list,
    title,
    netPnlMetrics: ({ color, name, tree }) => [
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
    ],
    charts: [createGroupedPeakRegretChartBasic(list, title)],
  });
}

// ============================================================================
// Cost Basis Section Builders (generic, type-safe composition)
// ============================================================================

/**
 * Generic single cost basis section builder - callers pass optional percentiles
 * @param {Object} args
 * @param {UtxoCohortObject | CohortWithoutRelative} args.cohort
 * @param {(metric: string) => string} args.title
 * @param {PartialChartOption[]} [args.charts] - Extra charts (e.g., percentiles)
 * @returns {PartialOptionsGroup}
 */
function createCostBasisSection({ cohort, title, charts = [] }) {
  const { color, tree } = cohort;
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Average",
        title: title("Cost Basis"),
        top: [
          price({
            metric: tree.realized.realizedPrice,
            name: "Average",
            color,
          }),
          price({
            metric: tree.costBasis.max,
            name: "Max",
            color: colors.green,
            defaultActive: false,
          }),
          price({
            metric: tree.costBasis.min,
            name: "Min",
            color: colors.red,
            defaultActive: false,
          }),
        ],
      },
      {
        name: "Max",
        title: title("Max Cost Basis"),
        top: [
          price({
            metric: tree.costBasis.max,
            name: "Max",
            color: colors.green,
          }),
        ],
      },
      {
        name: "Min",
        title: title("Min Cost Basis"),
        top: [
          price({
            metric: tree.costBasis.min,
            name: "Min",
            color: colors.red,
          }),
        ],
      },
      ...charts,
    ],
  };
}

/**
 * Generic grouped cost basis section builder - callers pass optional percentiles
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {Object} args
 * @param {T} args.list
 * @param {(metric: string) => string} args.title
 * @param {PartialChartOption[]} [args.charts] - Extra charts (e.g., percentiles)
 * @returns {PartialOptionsGroup}
 */
function createGroupedCostBasisSection({ list, title, charts = [] }) {
  return {
    name: "Cost Basis",
    tree: [
      {
        name: "Average",
        title: title("Average Cost Basis"),
        top: list.map(({ color, name, tree }) =>
          price({
            metric: tree.realized.realizedPrice,
            name,
            color,
          }),
        ),
      },
      {
        name: "Max",
        title: title("Max Cost Basis"),
        top: list.map(({ color, name, tree }) =>
          price({ metric: tree.costBasis.max, name, color }),
        ),
      },
      {
        name: "Min",
        title: title("Min Cost Basis"),
        top: list.map(({ color, name, tree }) =>
          price({ metric: tree.costBasis.min, name, color }),
        ),
      },
      ...charts,
    ],
  };
}

// ============================================================================
// Cost Basis Section Variants (by cohort capability)
// ============================================================================

/**
 * Create cost basis section for single cohort WITH percentiles
 * @param {CohortAll | CohortFull | CohortWithPercentiles} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleCostBasisSectionWithPercentiles(cohort, title) {
  return createCostBasisSection({
    cohort,
    title,
    charts: [
      {
        name: "Percentiles",
        title: title("Cost Basis Percentiles"),
        top: createCostBasisPercentilesSeries(colors, [cohort], false),
      },
      {
        name: "Invested Capital Percentiles",
        title: title("Invested Capital Percentiles"),
        top: createInvestedCapitalPercentilesSeries(colors, [cohort], false),
      },
      {
        name: "Spot Percentile",
        title: title("Spot Percentile"),
        bottom: createSpotPercentileSeries(colors, [cohort], false),
      },
    ],
  });
}

/**
 * Create cost basis section for grouped cohorts WITH percentiles
 * @param {readonly (CohortFull | CohortWithPercentiles)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedCostBasisSectionWithPercentiles(list, title) {
  return createGroupedCostBasisSection({
    list,
    title,
    charts: [
      {
        name: "Percentiles",
        title: title("Cost Basis Percentiles"),
        top: createCostBasisPercentilesSeries(colors, list, true),
      },
      {
        name: "Invested Capital Percentiles",
        title: title("Invested Capital Percentiles"),
        top: createInvestedCapitalPercentilesSeries(colors, list, true),
      },
      {
        name: "Spot Percentile",
        title: title("Spot Percentile"),
        bottom: createSpotPercentileSeries(colors, list, true),
      },
    ],
  });
}

// ============================================================================
// Activity Section Builders (generic, type-safe composition)
// ============================================================================

/**
 * Generic single activity section builder - callers pass optional extra value metrics
 * @param {Object} args
 * @param {UtxoCohortObject | CohortWithoutRelative} args.cohort
 * @param {(metric: string) => string} args.title
 * @param {AnyFetchedSeriesBlueprint[]} [args.valueMetrics] - Extra value metrics (e.g., adjusted)
 * @returns {PartialOptionsGroup}
 */
function createActivitySection({ cohort, title, valueMetrics = [] }) {
  const { tree, color } = cohort;

  return {
    name: "Activity",
    tree: [
      {
        name: "Sent",
        tree: [
          {
            name: "Sum",
            title: title("Sent"),
            bottom: [
              line({
                metric: tree.activity.sent.sats.sum,
                name: "sum",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.bitcoin.sum,
                name: "sum",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.dollars.sum,
                name: "sum",
                color,
                unit: Unit.usd,
              }),
              line({
                metric: tree.activity.sent14dEma.sats,
                name: "14d EMA",
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent14dEma.bitcoin,
                name: "14d EMA",
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent14dEma.dollars,
                name: "14d EMA",
                unit: Unit.usd,
              }),
            ],
          },
          {
            name: "Cumulative",
            title: title("Sent (Total)"),
            bottom: [
              line({
                metric: tree.activity.sent.sats.cumulative,
                name: "all-time",
                color,
                unit: Unit.sats,
              }),
              line({
                metric: tree.activity.sent.bitcoin.cumulative,
                name: "all-time",
                color,
                unit: Unit.btc,
              }),
              line({
                metric: tree.activity.sent.dollars.cumulative,
                name: "all-time",
                color,
                unit: Unit.usd,
              }),
            ],
          },
        ],
      },
      {
        name: "Sell Side Risk",
        title: title("Sell Side Risk Ratio"),
        bottom: createSingleSellSideRiskSeries(colors, tree),
      },
      {
        name: "Value",
        tree: [
          {
            name: "Created & Destroyed",
            title: title("Value Created & Destroyed"),
            bottom: [
              ...createSingleValueCreatedDestroyedSeries(colors, tree),
              ...valueMetrics,
            ],
          },
          {
            name: "Breakdown",
            title: title("Value Flow Breakdown"),
            bottom: createSingleValueFlowBreakdownSeries(colors, tree),
          },
          {
            name: "Flow",
            title: title("Capitulation & Profit Flow"),
            bottom: createSingleCapitulationProfitFlowSeries(colors, tree),
          },
        ],
      },
      {
        name: "Coins Destroyed",
        title: title("Coins Destroyed"),
        bottom: createSingleCoinsDestroyedSeries(cohort),
      },
    ],
  };
}

/**
 * Create grouped value flow charts (profit/loss created/destroyed, profit/capitulation flow)
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {T} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsTree}
 */
function createGroupedValueFlowCharts(list, title) {
  return [
    {
      name: "Profit Created",
      title: title("Profit Value Created"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.profitValueCreated,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "Profit Destroyed",
      title: title("Profit Value Destroyed"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.profitValueDestroyed,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "Loss Created",
      title: title("Loss Value Created"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.lossValueCreated,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "Loss Destroyed",
      title: title("Loss Value Destroyed"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.lossValueDestroyed,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "Profit Flow",
      title: title("Profit Flow"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.profitFlow,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
    {
      name: "Capitulation Flow",
      title: title("Capitulation Flow"),
      bottom: list.flatMap(({ color, name, tree }) => [
        line({
          metric: tree.realized.capitulationFlow,
          name,
          color,
          unit: Unit.usd,
        }),
      ]),
    },
  ];
}

/**
 * Generic grouped activity section builder - callers pass optional value tree
 * @template {readonly (UtxoCohortObject | CohortWithoutRelative)[]} T
 * @param {Object} args
 * @param {T} args.list
 * @param {(metric: string) => string} args.title
 * @param {PartialOptionsTree} [args.valueTree] - Optional value tree (defaults to basic created/destroyed)
 * @returns {PartialOptionsGroup}
 */
function createGroupedActivitySection({ list, title, valueTree }) {
  return {
    name: "Activity",
    tree: [
      {
        name: "Sent",
        tree: [
          {
            name: "Sum",
            title: title("Sent"),
            bottom: list.flatMap(({ color, name, tree }) =>
              satsBtcUsd({
                pattern: {
                  sats: tree.activity.sent.sats.sum,
                  bitcoin: tree.activity.sent.bitcoin.sum,
                  dollars: tree.activity.sent.dollars.sum,
                },
                name,
                color,
              }),
            ),
          },
          {
            name: "14d EMA",
            title: title("Sent 14d EMA"),
            bottom: list.flatMap(({ color, name, tree }) =>
              satsBtcUsd({ pattern: tree.activity.sent14dEma, name, color }),
            ),
          },
        ],
      },
      {
        name: "Sell Side Risk",
        title: title("Sell Side Risk Ratio"),
        bottom: createGroupedSellSideRiskSeries(list),
      },
      {
        name: "Value",
        tree: valueTree ?? [
          {
            name: "Created",
            title: title("Value Created"),
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
            name: "Destroyed",
            title: title("Value Destroyed"),
            bottom: list.flatMap(({ color, name, tree }) => [
              line({
                metric: tree.realized.valueDestroyed,
                name,
                color,
                unit: Unit.usd,
              }),
            ]),
          },
          ...createGroupedValueFlowCharts(list, title),
        ],
      },
      {
        name: "Coins Destroyed",
        tree: [
          {
            name: "Sum",
            title: title("Coins Destroyed"),
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
            title: title("Cumulative Coins Destroyed"),
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

// ============================================================================
// Activity Section Variants (by cohort capability)
// ============================================================================

/**
 * Create activity section with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {CohortAll | CohortFull | CohortWithAdjusted} cohort
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createSingleActivitySectionWithAdjusted(cohort, title) {
  const { tree } = cohort;
  return createActivitySection({
    cohort,
    title,
    valueMetrics: [
      line({
        metric: tree.realized.adjustedValueCreated,
        name: "Adjusted Created",
        color: colors.lime,
        unit: Unit.usd,
      }),
      line({
        metric: tree.realized.adjustedValueDestroyed,
        name: "Adjusted Destroyed",
        color: colors.pink,
        unit: Unit.usd,
      }),
    ],
  });
}

/**
 * Create activity section for grouped cohorts with adjusted values (for cohorts with RealizedPattern3/4)
 * @param {readonly (CohortFull | CohortWithAdjusted)[]} list
 * @param {(metric: string) => string} title
 * @returns {PartialOptionsGroup}
 */
function createGroupedActivitySectionWithAdjusted(list, title) {
  return createGroupedActivitySection({
    list,
    title,
    valueTree: [
      {
        name: "Created",
        tree: [
          {
            name: "Normal",
            title: title("Value Created"),
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
            title: title("Adjusted Value Created"),
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
        name: "Destroyed",
        tree: [
          {
            name: "Normal",
            title: title("Value Destroyed"),
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
            title: title("Adjusted Value Destroyed"),
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
      ...createGroupedValueFlowCharts(list, title),
    ],
  });
}
