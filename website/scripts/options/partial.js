/** Partial options - Main entry point */

import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderLongTerm,
  createCohortFolderAgeRangeWithMatured,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderBasicWithoutMarketCap,
  createCohortFolderWithoutRelative,
  createCohortFolderAddress,
  createAddressCohortFolder,
  createGroupedCohortFolderWithAdjusted,
  createGroupedCohortFolderWithNupl,
  createGroupedCohortFolderAgeRangeWithMatured,
  createGroupedCohortFolderBasicWithMarketCap,
  createGroupedCohortFolderBasicWithoutMarketCap,
  createGroupedCohortFolderAddress,
  createGroupedAddressCohortFolder,
  createUtxoProfitabilitySection,
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
    underAge,
    overAge,
    ageRange,
    epoch,
    utxosOverAmount,
    addressesOverAmount,
    utxosUnderAmount,
    addressesUnderAmount,
    utxosAmountRange,
    addressesAmountRange,
    typeAddressable,
    typeOther,
    class: class_,
    profitabilityRange,
    profitabilityProfit,
    profitabilityLoss,
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
              name: "UTXO Age",
              tree: [
                // Younger Than (< X old)
                {
                  name: "Younger Than",
                  tree: [
                    createGroupedCohortFolderWithAdjusted({
                      name: "Compare",
                      title: "Under Age",
                      list: underAge,
                      all: cohortAll,
                    }),
                    ...underAge.map(createCohortFolderWithAdjusted),
                  ],
                },
                // Older Than (≥ X old)
                {
                  name: "Older Than",
                  tree: [
                    createGroupedCohortFolderWithAdjusted({
                      name: "Compare",
                      title: "Over Age",
                      list: overAge,
                      all: cohortAll,
                    }),
                    ...overAge.map(createCohortFolderWithAdjusted),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedCohortFolderAgeRangeWithMatured({
                      name: "Compare",
                      title: "Age Ranges",
                      list: ageRange,
                      all: cohortAll,
                    }),
                    ...ageRange.map(createCohortFolderAgeRangeWithMatured),
                  ],
                },
              ],
            },

            // Sizes cohorts (UTXO size)
            {
              name: "UTXO Size",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Over Amount",
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
                      title: "Under Amount",
                      list: utxosOverAmount,
                      all: cohortAll,
                    }),
                    ...utxosOverAmount.map(createCohortFolderBasicWithMarketCap),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedCohortFolderBasicWithoutMarketCap({
                      name: "Compare",
                      title: "Amount Ranges",
                      list: utxosAmountRange,
                      all: cohortAll,
                    }),
                    ...utxosAmountRange.map(createCohortFolderBasicWithoutMarketCap),
                  ],
                },
              ],
            },

            // Balances cohorts (Address balance)
            {
              name: "Address Balance",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Over Balance",
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
                      title: "Under Balance",
                      list: addressesOverAmount,
                      all: cohortAll,
                    }),
                    ...addressesOverAmount.map(createAddressCohortFolder),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Balance Ranges",
                      list: addressesAmountRange,
                      all: cohortAll,
                    }),
                    ...addressesAmountRange.map(createAddressCohortFolder),
                  ],
                },
              ],
            },

            // Script Types - addressable types have addrCount, others don't
            {
              name: "Script Type",
              tree: [
                createGroupedCohortFolderAddress({
                  name: "Compare",
                  title: "Script Type",
                  list: typeAddressable,
                  all: cohortAll,
                }),
                ...typeAddressable.map(createCohortFolderAddress),
                ...typeOther.map(createCohortFolderWithoutRelative),
              ],
            },

            // Epochs
            {
              name: "Epoch",
              tree: [
                createGroupedCohortFolderWithAdjusted({
                  name: "Compare",
                  title: "Epoch",
                  list: epoch,
                  all: cohortAll,
                }),
                ...epoch.map(createCohortFolderWithAdjusted),
              ],
            },

            // Classes
            {
              name: "Class",
              tree: [
                createGroupedCohortFolderWithAdjusted({
                  name: "Compare",
                  title: "Class",
                  list: class_,
                  all: cohortAll,
                }),
                ...class_.map(createCohortFolderWithAdjusted),
              ],
            },

            // UTXO Profitability bands
            createUtxoProfitabilitySection({
              range: profitabilityRange,
              profit: profitabilityProfit,
              loss: profitabilityLoss,
            }),
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
