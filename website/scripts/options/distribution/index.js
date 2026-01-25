/**
 * Cohort module - exports all cohort-related functionality
 */

// Cohort data builder
export { buildCohortData } from "./data.js";

// Cohort folder builders (type-safe!)
export {
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderWithPercentiles,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderBasicWithoutMarketCap,
  createCohortFolderAddress,
} from "./utxo.js";
export { createAddressCohortFolder } from "./address.js";

// Shared helpers
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
