/** Partial options - Main entry point */

import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderLongTerm,
  createCohortFolderAgeRangeWithMatured,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderWithoutRelative,
  createCohortFolderAddress,
  createAddressCohortFolder,
  createGroupedCohortFolderWithAdjusted,
  createGroupedCohortFolderWithNupl,
  createGroupedCohortFolderAgeRangeWithMatured,
  createGroupedCohortFolderBasicWithMarketCap,
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
    {
      name: "Charts",
      tree: [
        createMarketSection(),

        createNetworkSection(),

        createMiningSection(),

        {
          name: "Distribution",
          tree: [
            createCohortFolderAll({ ...cohortAll, name: "Overview" }),

            createGroupedCohortFolderWithNupl({
              name: "STH vs LTH",
              title: "STH vs LTH",
              list: [termShort, termLong],
              all: cohortAll,
            }),

            createCohortFolderFull(termShort),

            createCohortFolderLongTerm(termLong),

            {
              name: "UTXO Age",
              tree: [
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

            {
              name: "UTXO Size",
              tree: [
                {
                  name: "Less Than",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Under Amount",
                      list: utxosUnderAmount,
                      all: cohortAll,
                    }),
                    ...utxosUnderAmount.map(
                      createCohortFolderBasicWithMarketCap,
                    ),
                  ],
                },
                {
                  name: "More Than",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Over Amount",
                      list: utxosOverAmount,
                      all: cohortAll,
                    }),
                    ...utxosOverAmount.map(
                      createCohortFolderBasicWithMarketCap,
                    ),
                  ],
                },
                {
                  name: "Range",
                  tree: [
                    createGroupedCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Amount Ranges",
                      list: utxosAmountRange,
                      all: cohortAll,
                    }),
                    ...utxosAmountRange.map(
                      createCohortFolderBasicWithMarketCap,
                    ),
                  ],
                },
              ],
            },

            createUtxoProfitabilitySection({
              range: profitabilityRange,
              profit: profitabilityProfit,
              loss: profitabilityLoss,
            }),

            {
              name: "Address Balance",
              tree: [
                {
                  name: "Less Than",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Under Balance",
                      list: addressesUnderAmount,
                      all: cohortAll,
                    }),
                    ...addressesUnderAmount.map(createAddressCohortFolder),
                  ],
                },
                {
                  name: "More Than",
                  tree: [
                    createGroupedAddressCohortFolder({
                      name: "Compare",
                      title: "Over Balance",
                      list: addressesOverAmount,
                      all: cohortAll,
                    }),
                    ...addressesOverAmount.map(createAddressCohortFolder),
                  ],
                },
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
          ],
        },

        createInvestingSection(),

        {
          name: "Frameworks",
          tree: [createCointimeSection()],
        },
      ],
    },

    {
      name: "API",
      url: () => "/api",
      title: "API documentation",
    },

    {
      name: "Source",
      url: () => "https://bitcoinresearchkit.org",
      title: "Bitcoin Research Kit",
    },
  ];
}
