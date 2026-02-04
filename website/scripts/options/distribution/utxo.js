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
  createSingleSoprSeries,
  createSingleRealizedAthRegretSeries,
  createGroupedRealizedAthRegretSeries,
  createSingleSentimentSeries,
  createGroupedNetSentimentSeries,
  createGroupedGreedIndexSeries,
  createGroupedPainIndexSeries,
} from "./shared.js";
import { formatCohortTitle, satsBtcUsd } from "../shared.js";
import {
  createCostBasisSection,
  createCostBasisSectionWithPercentiles,
  createGroupedCostBasisSection,
  createGroupedCostBasisSectionWithPercentiles,
} from "./cost-basis.js";
import {
  createHoldingsSection,
  createHoldingsSectionAll,
  createHoldingsSectionAddress,
  createHoldingsSectionWithRelative,
  createGroupedHoldingsSection,
  createGroupedHoldingsSectionAddress,
  createGroupedHoldingsSectionWithRelative,
} from "./holdings.js";
import {
  createPricesSectionFull,
  createPricesSectionBasic,
  createGroupedPricesSection,
} from "./prices.js";
import {
  createValuationSection,
  createValuationSectionFull,
  createGroupedValuationSection,
} from "./valuation.js";
import {
  createActivitySection,
  createActivitySectionWithAdjusted,
  createGroupedActivitySection,
  createGroupedActivitySectionWithAdjusted,
} from "./activity.js";
import {
  createProfitabilitySection,
  createProfitabilitySectionWithNupl,
  createProfitabilitySectionAll,
  createProfitabilitySectionWithPeakRegret,
  createGroupedProfitabilitySection,
  createGroupedProfitabilitySectionWithNupl,
} from "./profitability.js";
import { Unit } from "../../utils/units.js";
import { line, baseline } from "../series.js";
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
      createHoldingsSectionAll({ cohort: args, title }),
      createValuationSectionFull({ cohort: args, title }),
      createPricesSectionFull({ cohort: args, title }),
      createCostBasisSectionWithPercentiles({ cohort: args, title }),
      createProfitabilitySectionAll({ cohort: args, title }),
      createActivitySectionWithAdjusted({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSectionWithPercentiles({ list, title }),
        createGroupedProfitabilitySectionWithNupl({ list, title }),
        createGroupedActivitySectionWithAdjusted({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort: args, title }),
      createValuationSectionFull({ cohort: args, title }),
      createPricesSectionFull({ cohort: args, title }),
      createCostBasisSectionWithPercentiles({ cohort: args, title }),
      createProfitabilitySectionWithNupl({ cohort: args, title }),
      createActivitySectionWithAdjusted({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySectionWithNupl({ list, title }),
        createGroupedActivitySectionWithAdjusted({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySectionWithNupl({ cohort: args, title }),
      createActivitySectionWithAdjusted({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSectionWithPercentiles({ list, title }),
        createGroupedProfitabilitySectionWithNupl({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort: args, title }),
      createValuationSectionFull({ cohort: args, title }),
      createPricesSectionFull({ cohort: args, title }),
      createCostBasisSectionWithPercentiles({ cohort: args, title }),
      createProfitabilitySectionWithNupl({ cohort: args, title }),
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
        createGroupedHoldingsSection({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSectionWithPercentiles({ list, title }),
        createGroupedProfitabilitySection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSection({ cohort: args, title }),
      createValuationSectionFull({ cohort: args, title }),
      createPricesSectionFull({ cohort: args, title }),
      createCostBasisSectionWithPercentiles({ cohort: args, title }),
      createProfitabilitySection({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySectionWithPeakRegret({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySectionWithNupl({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySectionWithNupl({ cohort: args, title }),
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
        createGroupedHoldingsSection({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSection({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySection({ cohort: args, title }),
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
        createGroupedHoldingsSectionAddress({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionAddress({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
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
        createGroupedHoldingsSection({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySection({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSection({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySection({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
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
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort, {
          extra: createRealizedCapRatioSeries(tree),
        }),
      },
      ...createSingleRealizedPnlSection(cohort, title, {
        extra: createRealizedPnlRatioSeries(tree),
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
      {
        name: "Capitalization",
        title: title("Realized Cap"),
        bottom: createSingleRealizedCapSeries(cohort, {
          extra: createRealizedCapRatioSeries(tree),
        }),
      },
      ...createSingleRealizedPnlSection(cohort, title, {
        extra: createRealizedPnlRatioSeries(tree),
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
 * @param {{ realized: RealizedWithExtras }} tree
 * @returns {AnyFetchedSeriesBlueprint[]}
 */
function createRealizedPnlRatioSeries(tree) {
  return [
    line({
      metric: tree.realized.realizedProfitToLossRatio,
      name: "P/L Ratio",
      color: colors.plRatio,
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
            // USD
            line({
              metric: tree.realized.realizedProfit.sum,
              name: "Profit",
              color: colors.profit,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedProfit7dEma,
              name: "Profit 7d EMA",
              color: colors.profit,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss.sum,
              name: "Loss",
              color: colors.loss,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss7dEma,
              name: "Loss 7d EMA",
              color: colors.loss,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.negRealizedLoss.sum,
              name: "Negative Loss",
              color: colors.loss,
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
            // % of R.Cap
            baseline({
              metric: tree.realized.realizedProfitRelToRealizedCap.sum,
              name: "Profit",
              color: colors.profit,
              unit: Unit.pctRcap,
            }),
            baseline({
              metric: tree.realized.realizedLossRelToRealizedCap.sum,
              name: "Loss",
              color: colors.loss,
              unit: Unit.pctRcap,
            }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Realized P&L (Total)"),
          bottom: [
            // USD
            line({
              metric: tree.realized.realizedProfit.cumulative,
              name: "Profit",
              color: colors.profit,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.realizedLoss.cumulative,
              name: "Loss",
              color: colors.loss,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.negRealizedLoss.cumulative,
              name: "Negative Loss",
              color: colors.loss,
              unit: Unit.usd,
              defaultActive: false,
            }),
            // % of R.Cap
            baseline({
              metric: tree.realized.realizedProfitRelToRealizedCap.cumulative,
              name: "Profit",
              color: colors.profit,
              unit: Unit.pctRcap,
            }),
            baseline({
              metric: tree.realized.realizedLossRelToRealizedCap.cumulative,
              name: "Loss",
              color: colors.loss,
              unit: Unit.pctRcap,
            }),
          ],
        },
      ],
    },
    {
      name: "Net P&L",
      tree: [
        {
          name: "Sum",
          title: title("Net Realized P&L"),
          bottom: [
            // USD
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
            // % of R.Cap
            baseline({
              metric: tree.realized.netRealizedPnlRelToRealizedCap.sum,
              name: "Net",
              unit: Unit.pctRcap,
            }),
          ],
        },
        {
          name: "Cumulative",
          title: title("Net Realized P&L (Total)"),
          bottom: [
            // USD
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
            // % of R.Cap
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
            // % of M.Cap
            baseline({
              metric:
                tree.realized.netRealizedPnlCumulative30dDeltaRelToMarketCap,
              name: "30d Change",
              unit: Unit.pctMcap,
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
              color: colors.profit,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInProfit.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.profit,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.sum,
              name: "Sum",
              color: colors.profit,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInProfit.sats.cumulative,
              name: "Cumulative",
              color: colors.profit,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.sum,
              name: "Sum",
              color: colors.profit,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.sentInProfit.dollars.cumulative,
              name: "Cumulative",
              color: colors.profit,
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
              color: colors.loss,
              unit: Unit.btc,
            }),
            line({
              metric: tree.realized.sentInLoss.bitcoin.cumulative,
              name: "Cumulative",
              color: colors.loss,
              unit: Unit.btc,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.sum,
              name: "Sum",
              color: colors.loss,
              unit: Unit.sats,
            }),
            line({
              metric: tree.realized.sentInLoss.sats.cumulative,
              name: "Cumulative",
              color: colors.loss,
              unit: Unit.sats,
              defaultActive: false,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.sum,
              name: "Sum",
              color: colors.loss,
              unit: Unit.usd,
            }),
            line({
              metric: tree.realized.sentInLoss.dollars.cumulative,
              name: "Cumulative",
              color: colors.loss,
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
            color: colors.profit,
          }),
        },
        {
          name: "In Loss 14d EMA",
          title: title("Sent In Loss 14d EMA"),
          bottom: satsBtcUsd({
            pattern: tree.realized.sentInLoss14dEma,
            name: "14d EMA",
            color: colors.loss,
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
    bottom: createSingleSoprSeries(cohort.tree),
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
        color: colors.bi.p1,
        unit: Unit.ratio,
        base: 1,
      }),
      baseline({
        metric: tree.realized.adjustedSopr7dEma,
        name: "Adj. 7d EMA",
        color: colors.bi.p2,
        unit: Unit.ratio,
        defaultActive: false,
        base: 1,
      }),
      baseline({
        metric: tree.realized.adjustedSopr30dEma,
        name: "Adj. 30d EMA",
        color: colors.bi.p3,
        unit: Unit.ratio,
        defaultActive: false,
        base: 1,
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
          base: 1,
        }),
        baseline({
          metric: tree.realized.adjustedSopr7dEma,
          name: `${name} 7d`,
          color,
          unit: Unit.ratio,
          defaultActive: false,
          base: 1,
        }),
        baseline({
          metric: tree.realized.adjustedSopr30dEma,
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
      color: colors.profit,
      unit: Unit.pctMcap,
    }),
    line({
      metric: rel.unrealizedLossRelToMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctMcap,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToMarketCap,
      name: "Negative Loss",
      color: colors.loss,
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
      color: colors.profit,
      unit: Unit.pctOwnMcap,
    }),
    line({
      metric: rel.unrealizedLossRelToOwnMarketCap,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnMcap,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToOwnMarketCap,
      name: "Negative Loss",
      color: colors.loss,
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
      color: colors.profit,
      unit: Unit.pctOwnPnl,
    }),
    line({
      metric: rel.unrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Loss",
      color: colors.loss,
      unit: Unit.pctOwnPnl,
      defaultActive: false,
    }),
    line({
      metric: rel.negUnrealizedLossRelToOwnTotalUnrealizedPnl,
      name: "Negative Loss",
      color: colors.loss,
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
      color: colors.profit,
      unit: Unit.pctOwnRcap,
    }),
    baseline({
      metric: rel.investedCapitalInLossPct,
      name: "In Loss",
      color: colors.loss,
      unit: Unit.pctOwnRcap,
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
      metric: tree.unrealized.unrealizedProfit,
      name: "Profit",
      color: colors.profit,
      unit: Unit.usd,
    }),
    line({
      metric: tree.unrealized.unrealizedLoss,
      name: "Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.unrealized.negUnrealizedLoss,
      name: "Negative Loss",
      color: colors.loss,
      unit: Unit.usd,
      defaultActive: false,
    }),
    line({
      metric: tree.unrealized.totalUnrealizedPnl,
      name: "Total",
      color: colors.default,
      unit: Unit.usd,
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
        color: colors.bitcoin,
        unit: Unit.usd,
      }),
    ],
  };
}

/**
 * Create peak regret chart with RelToMarketCap metric
 * @param {{ unrealized: UnrealizedFullPattern, relative: RelativeWithPeakRegret }} tree
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
        color: colors.bitcoin,
        unit: Unit.usd,
      }),
      baseline({
        metric: tree.relative.unrealizedPeakRegretRelToMarketCap,
        name: "Peak Regret",
        color: colors.bitcoin,
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
        color: colors.profit,
        unit: Unit.usd,
      }),
      line({
        metric: tree.unrealized.investedCapitalInLoss,
        name: "In Loss",
        color: colors.loss,
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
 * @param {readonly { color: Color, name: string, tree: { unrealized: UnrealizedFullPattern, relative: RelativeWithPeakRegret } }[]} list
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
    name: "Profitability",
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
        bottom: createSingleSentimentSeries(tree),
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
          unit: Unit.pctOwnRcap,
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
          unit: Unit.pctOwnRcap,
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
    name: "Profitability",
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
    name: "Profitability",
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
