/** Partial options - Main entry point */

import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderLongTerm,
  createCohortFolderAgeRange,
  createCohortFolderMinAge,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderBasicWithoutMarketCap,
  createCohortFolderWithoutRelative,
  createCohortFolderAddress,
  createAddressCohortFolder,
  createGroupedCohortFolderWithAdjusted,
  createGroupedCohortFolderWithNupl,
  createGroupedCohortFolderAgeRange,
  createGroupedCohortFolderMinAge,
  createGroupedCohortFolderBasicWithMarketCap,
  createGroupedCohortFolderBasicWithoutMarketCap,
  createGroupedCohortFolderAddress,
  createGroupedAddressCohortFolder,
} from "./distribution/index.js";
import { createMarketSection } from "./market.js";
import { createNetworkSection } from "./network.js";
import { createMiningSection } from "./mining.js";
import { createCointimeSection } from "./cointime.js";
import { createInvestingSection } from "./investing.js";

// Re-export types for external consumers
export * from "./types.js";

/**
 * Create partial options tree
 * @returns {PartialOptionsTree}
 */
export function createPartialOptions() {
  // Build cohort data
  const {
    cohortAll,
    termShort,
    termLong,
    upToDate,
    fromDate,
    dateRange,
    epoch,
    utxosAboveAmount,
    addressesAboveAmount,
    utxosUnderAmount,
    addressesUnderAmount,
    utxosAmountRanges,
    addressesAmountRanges,
    typeAddressable,
    typeOther,
    year,
  } = buildCohortData();

  return [
    // Charts section
    {
      name: "Charts",
      tree: [
        // Market section
        createMarketSection(),

        // Network section (on-chain activity)
        createNetworkSection(),

        // Mining section (security & economics)
        createMiningSection(),

        // Cohorts section
        {
          name: "Distribution",
          tree: [
            // Overview - All UTXOs
            createCohortFolderAll({ ...cohortAll, name: "Overview" }),

            // STH vs LTH - Direct comparison
            createGroupedCohortFolderWithNupl({
              name: "STH vs LTH",
              title: "STH vs LTH",
              list: [termShort, termLong],
              all: cohortAll,
            }),

            // STH - Short term holder cohort
            createCohortFolderFull(termShort),

            // LTH - Long term holder cohort
            createCohortFolderLongTerm(termLong),

            // Ages cohorts
            {
              name: "UTXO Ages",
              tree: [
                // Younger Than (< X old)
                {
                  name: "Younger Than",
                  tree: [
                    createGroupedCohortFolderWithAdjusted({
                      name: "Compare",
                      title: "Max Age",
                      list: upToDate,
                      all: cohortAll,
                    }),
                    ...upToDate.map(createCohortFolderWithAdjusted),
                  ],
                },
                // Older Than (≥ X old)
                {
                  name: "Older Than",
                  tree: [
                    createGroupedCohortFolderMinAge({
                      name: "Compare",
                      title: "Min Age",
                      list: fromDate,
                      all: cohortAll,
                    }),
                    ...fromDate.map(createCohortFolderMinAge),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedCohortFolderAgeRange({
                      name: "Compare",
                      title: "Age Ranges",
                      list: dateRange,
                      all: cohortAll,
                    }),
                    ...dateRange.map(createCohortFolderAgeRange),
                  ],
                },
              ],
            },

            // Sizes cohorts (UTXO size)
            {
              name: "UTXO Sizes",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Max Size",
                      list: utxosUnderAmount,
                      all: cohortAll,
                    }),
                    ...utxosUnderAmount.map(createCohortFolderBasicWithMarketCap),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Min Size",
                      list: utxosAboveAmount,
                      all: cohortAll,
                    }),
                    ...utxosAboveAmount.map(createCohortFolderBasicWithMarketCap),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedCohortFolderBasicWithoutMarketCap({
                      name: "Compare",
                      title: "Size Ranges",
                      list: utxosAmountRanges,
                      all: cohortAll,
                    }),
                    ...utxosAmountRanges.map(createCohortFolderBasicWithoutMarketCap),
                  ],
                },
              ],
            },

            // Balances cohorts (Address balance)
            {
              name: "Address Balances",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Max Balance",
                      list: addressesUnderAmount,
                      all: cohortAll,
                    }),
                    ...addressesUnderAmount.map(createAddressCohortFolder),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Min Balance",
                      list: addressesAboveAmount,
                      all: cohortAll,
                    }),
                    ...addressesAboveAmount.map(createAddressCohortFolder),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Balance Ranges",
                      list: addressesAmountRanges,
                      all: cohortAll,
                    }),
                    ...addressesAmountRanges.map(createAddressCohortFolder),
                  ],
                },
              ],
            },

            // Script Types - addressable types have addrCount, others don't
            {
              name: "Script Types",
              tree: [
                createGroupedCohortFolderAddress({
                  name: "Compare",
                  title: "Script Types",
                  list: typeAddressable,
                  all: cohortAll,
                }),
                ...typeAddressable.map(createCohortFolderAddress),
                ...typeOther.map(createCohortFolderWithoutRelative),
              ],
            },

            // Epochs
            {
              name: "Epochs",
              tree: [
                createGroupedCohortFolderBasicWithoutMarketCap({
                  name: "Compare",
                  title: "Epochs",
                  list: epoch,
                  all: cohortAll,
                }),
                ...epoch.map(createCohortFolderBasicWithoutMarketCap),
              ],
            },

            // Years
            {
              name: "Years",
              tree: [
                createGroupedCohortFolderBasicWithoutMarketCap({
                  name: "Compare",
                  title: "Years",
                  list: year,
                  all: cohortAll,
                }),
                ...year.map(createCohortFolderBasicWithoutMarketCap),
              ],
            },
          ],
        },

        // Frameworks section
        {
          name: "Frameworks",
          tree: [createCointimeSection()],
        },

        // Investing section
        createInvestingSection(),
      ],
    },

    // API documentation
    {
      name: "API",
      url: () => "/api",
      title: "API documentation",
    },

    // Project link
    {
      name: "Source",
      url: () => "https://bitcoinresearchkit.org",
      title: "Bitcoin Research Kit",
    },
  ];
}
