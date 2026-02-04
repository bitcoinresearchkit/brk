/**
 * Cohort module - exports all cohort-related functionality
 *
 * Folder builders compose sections from building blocks:
 * - holdings.js: Supply, UTXO Count, Address Count
 * - valuation.js: Realized Cap, Market Cap, MVRV
 * - prices.js: Realized Price, ratios
 * - cost-basis.js: Cost basis percentiles
 * - profitability.js: Unrealized/Realized P&L, Invested Capital
 * - activity.js: SOPR, Volume, Lifespan
 */

import { formatCohortTitle } from "../shared.js";

// Section builders
import {
  createHoldingsSection,
  createHoldingsSectionAll,
  createHoldingsSectionAddress,
  createHoldingsSectionAddressAmount,
  createHoldingsSectionWithRelative,
  createHoldingsSectionWithOwnSupply,
  createGroupedHoldingsSection,
  createGroupedHoldingsSectionAddress,
  createGroupedHoldingsSectionAddressAmount,
  createGroupedHoldingsSectionWithRelative,
  createGroupedHoldingsSectionWithOwnSupply,
} from "./holdings.js";
import {
  createValuationSection,
  createValuationSectionFull,
  createGroupedValuationSection,
  createGroupedValuationSectionWithOwnMarketCap,
} from "./valuation.js";
import {
  createPricesSectionFull,
  createPricesSectionBasic,
  createGroupedPricesSection,
} from "./prices.js";
import {
  createCostBasisSection,
  createCostBasisSectionWithPercentiles,
  createGroupedCostBasisSection,
  createGroupedCostBasisSectionWithPercentiles,
} from "./cost-basis.js";
import {
  createProfitabilitySection,
  createProfitabilitySectionAll,
  createProfitabilitySectionFull,
  createProfitabilitySectionWithNupl,
  createProfitabilitySectionWithPeakRegret,
  createProfitabilitySectionWithInvestedCapitalPct,
  createProfitabilitySectionBasicWithInvestedCapitalPct,
  createProfitabilitySectionLongTerm,
  createGroupedProfitabilitySection,
  createGroupedProfitabilitySectionWithNupl,
  createGroupedProfitabilitySectionWithPeakRegret,
  createGroupedProfitabilitySectionWithInvestedCapitalPct,
  createGroupedProfitabilitySectionBasicWithInvestedCapitalPct,
  createGroupedProfitabilitySectionLongTerm,
} from "./profitability.js";
import {
  createActivitySection,
  createActivitySectionWithAdjusted,
  createGroupedActivitySection,
  createGroupedActivitySectionWithAdjusted,
} from "./activity.js";

// Re-export data builder
export { buildCohortData } from "./data.js";

// Re-export shared helpers
export {
  createSingleSupplySeries,
  createGroupedSupplyTotalSeries,
  createGroupedSupplyInProfitSeries,
  createGroupedSupplyInLossSeries,
  createUtxoCountSeries,
  createAddressCountSeries,
  createRealizedPriceSeries,
  createRealizedPriceRatioSeries,
  createRealizedCapSeries,
  createCostBasisPercentilesSeries,
} from "./shared.js";

// ============================================================================
// Folder Builders
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
        createGroupedValuationSectionWithOwnMarketCap({ list, title }),
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
      createProfitabilitySectionFull({ cohort: args, title }),
      createActivitySectionWithAdjusted({ cohort: args, title }),
    ],
  };
}

/**
 * Adjusted folder: adjustedSopr only, no percentiles (maxAge.*)
 * Has Peak Regret metrics like minAge
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
        createGroupedProfitabilitySectionWithPeakRegret({ list, title }),
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
      createProfitabilitySectionWithPeakRegret({ cohort: args, title }),
      createActivitySectionWithAdjusted({ cohort: args, title }),
    ],
  };
}

/**
 * Folder for cohorts with nupl + percentiles (no longer used for term.long which has own folder)
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
 * LongTerm folder: term.long (has own market cap + NUPL + peak regret + P/L ratio)
 * @param {CohortLongTerm | CohortGroupLongTerm} args
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderLongTerm(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedHoldingsSectionWithRelative({ list, title }),
        createGroupedValuationSectionWithOwnMarketCap({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSectionWithPercentiles({ list, title }),
        createGroupedProfitabilitySectionLongTerm({ list, title }),
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
      createProfitabilitySectionLongTerm({ cohort: args, title }),
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
        createGroupedHoldingsSectionWithOwnSupply({ list, title }),
        createGroupedValuationSectionWithOwnMarketCap({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSectionWithPercentiles({ list, title }),
        createGroupedProfitabilitySectionWithInvestedCapitalPct({ list, title }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithOwnSupply({ cohort: args, title }),
      createValuationSectionFull({ cohort: args, title }),
      createPricesSectionFull({ cohort: args, title }),
      createCostBasisSectionWithPercentiles({ cohort: args, title }),
      createProfitabilitySectionWithInvestedCapitalPct({ cohort: args, title }),
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
        createGroupedProfitabilitySectionWithPeakRegret({ list, title }),
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
        createGroupedHoldingsSectionWithOwnSupply({ list, title }),
        createGroupedValuationSection({ list, title }),
        createGroupedPricesSection({ list, title }),
        createGroupedCostBasisSection({ list, title }),
        createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({
          list,
          title,
        }),
        createGroupedActivitySection({ list, title }),
      ],
    };
  }
  const title = formatCohortTitle(args.name);
  return {
    name: args.name || "all",
    tree: [
      createHoldingsSectionWithOwnSupply({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySectionBasicWithInvestedCapitalPct({
        cohort: args,
        title,
      }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}

/**
 * Address folder: like basic but with address count (addressable type cohorts)
 * Has invested capital percentage metrics
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
        createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({
          list,
          title,
        }),
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
      createProfitabilitySectionBasicWithInvestedCapitalPct({
        cohort: args,
        title,
      }),
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
 * Address amount cohort folder - for address balance cohorts (has NUPL + addrCount)
 * @param {AddressCohortObject | AddressCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createAddressCohortFolder(args) {
  if ("list" in args) {
    const { list } = args;
    const title = formatCohortTitle(args.title);
    return {
      name: args.name || "all",
      tree: [
        createGroupedHoldingsSectionAddressAmount({ list, title }),
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
      createHoldingsSectionAddressAmount({ cohort: args, title }),
      createValuationSection({ cohort: args, title }),
      createPricesSectionBasic({ cohort: args, title }),
      createCostBasisSection({ cohort: args, title }),
      createProfitabilitySectionWithNupl({ cohort: args, title }),
      createActivitySection({ cohort: args, title }),
    ],
  };
}
