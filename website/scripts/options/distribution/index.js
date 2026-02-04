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

// ============================================================================
// Single Cohort Folder Builders
// ============================================================================

/**
 * All folder: for the special "All" cohort
 * @param {CohortAll} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAll(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionAll({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionAll({ cohort, title }),
      createActivitySectionWithAdjusted({ cohort, title }),
    ],
  };
}

/**
 * Full folder: adjustedSopr + percentiles + RelToMarketCap
 * @param {CohortFull} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderFull(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionFull({ cohort, title }),
      createActivitySectionWithAdjusted({ cohort, title }),
    ],
  };
}

/**
 * Adjusted folder: adjustedSopr only, no percentiles
 * @param {CohortWithAdjusted} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithAdjusted(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionWithPeakRegret({ cohort, title }),
      createActivitySectionWithAdjusted({ cohort, title }),
    ],
  };
}

/**
 * Folder for cohorts with nupl + percentiles
 * @param {CohortWithNuplPercentiles} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithNupl(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionWithNupl({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * LongTerm folder: has own market cap + NUPL + peak regret + P/L ratio
 * @param {CohortLongTerm} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderLongTerm(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionLongTerm({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Age range folder: no nupl
 * @param {CohortAgeRange} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAgeRange(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithOwnSupply({ cohort, title }),
      createValuationSectionFull({ cohort, title }),
      createPricesSectionFull({ cohort, title }),
      createCostBasisSectionWithPercentiles({ cohort, title }),
      createProfitabilitySectionWithInvestedCapitalPct({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * MinAge folder: has peakRegret in unrealized
 * @param {CohortMinAge} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderMinAge(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionWithPeakRegret({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Basic folder WITH RelToMarketCap
 * @param {CohortBasicWithMarketCap} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithMarketCap(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithRelative({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionWithNupl({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Basic folder WITHOUT RelToMarketCap
 * @param {CohortBasicWithoutMarketCap} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderBasicWithoutMarketCap(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionWithOwnSupply({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionBasicWithInvestedCapitalPct({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Address folder: like basic but with address count
 * @param {CohortAddress} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderAddress(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionAddress({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionBasicWithInvestedCapitalPct({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Folder for cohorts WITHOUT relative section
 * @param {CohortWithoutRelative} cohort
 * @returns {PartialOptionsGroup}
 */
export function createCohortFolderWithoutRelative(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSection({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySection({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

/**
 * Address amount cohort folder: has NUPL + addrCount
 * @param {AddressCohortObject} cohort
 * @returns {PartialOptionsGroup}
 */
export function createAddressCohortFolder(cohort) {
  const title = formatCohortTitle(cohort.name);
  return {
    name: cohort.name || "all",
    tree: [
      createHoldingsSectionAddressAmount({ cohort, title }),
      createValuationSection({ cohort, title }),
      createPricesSectionBasic({ cohort, title }),
      createCostBasisSection({ cohort, title }),
      createProfitabilitySectionWithNupl({ cohort, title }),
      createActivitySection({ cohort, title }),
    ],
  };
}

// ============================================================================
// Grouped Cohort Folder Builders
// ============================================================================

/**
 * @param {CohortGroupFull} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderFull({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSectionWithOwnMarketCap({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSectionWithPercentiles({ list, all, title }),
      createGroupedProfitabilitySectionWithNupl({ list, all, title }),
      createGroupedActivitySectionWithAdjusted({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupWithAdjusted} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderWithAdjusted({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionWithPeakRegret({ list, all, title }),
      createGroupedActivitySectionWithAdjusted({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupWithNuplPercentiles} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderWithNupl({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSectionWithPercentiles({ list, all, title }),
      createGroupedProfitabilitySectionWithNupl({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupLongTerm} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderLongTerm({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSectionWithOwnMarketCap({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSectionWithPercentiles({ list, all, title }),
      createGroupedProfitabilitySectionLongTerm({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupAgeRange} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderAgeRange({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithOwnSupply({ list, all, title }),
      createGroupedValuationSectionWithOwnMarketCap({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSectionWithPercentiles({ list, all, title }),
      createGroupedProfitabilitySectionWithInvestedCapitalPct({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupMinAge} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderMinAge({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionWithPeakRegret({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupBasicWithMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderBasicWithMarketCap({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithRelative({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionWithNupl({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupBasicWithoutMarketCap} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderBasicWithoutMarketCap({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionWithOwnSupply({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupAddress} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderAddress({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionAddress({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionBasicWithInvestedCapitalPct({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {CohortGroupWithoutRelative} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedCohortFolderWithoutRelative({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSection({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySection({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}

/**
 * @param {AddressCohortGroupObject} args
 * @returns {PartialOptionsGroup}
 */
export function createGroupedAddressCohortFolder({ name, title: groupTitle, list, all }) {
  const title = formatCohortTitle(groupTitle);
  return {
    name: name || "all",
    tree: [
      createGroupedHoldingsSectionAddressAmount({ list, all, title }),
      createGroupedValuationSection({ list, all, title }),
      createGroupedPricesSection({ list, all, title }),
      createGroupedCostBasisSection({ list, all, title }),
      createGroupedProfitabilitySectionWithNupl({ list, all, title }),
      createGroupedActivitySection({ list, all, title }),
    ],
  };
}
