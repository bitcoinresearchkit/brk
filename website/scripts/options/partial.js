/** Partial options - Main entry point */

import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderWithNupl,
  createCohortFolderLongTerm,
  createCohortFolderAgeRange,
  createCohortFolderMinAge,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderBasicWithoutMarketCap,
  createCohortFolderWithoutRelative,
  createCohortFolderAddress,
  createAddressCohortFolder,
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
    // Debug explorer (disabled)
    // ...(localhost
    //   ? [
    //       {
    //         kind: /** @type {const} */ ("explorer"),
    //         name: "Explorer",
    //         title: "Debug explorer",
    //       },
    //     ]
    //   : []),

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
            // Overview - All UTXOs (adjustedSopr + percentiles but no RelToMarketCap)
            createCohortFolderAll({ ...cohortAll, name: "Overview" }),

            // STH vs LTH - Direct comparison (before individual cohorts)
            createCohortFolderWithNupl({
              name: "STH vs LTH",
              title: "STH vs LTH",
              list: [termShort, termLong],
            }),

            // STH - Short term holder cohort (Full capability)
            createCohortFolderFull(termShort),

            // LTH - Long term holder cohort (own market cap + nupl + peak regret + P/L ratio)
            createCohortFolderLongTerm(termLong),

            // Ages cohorts
            {
              name: "UTXO Ages",
              tree: [
                // Younger Than (< X old)
                {
                  name: "Younger Than",
                  tree: [
                    createCohortFolderWithAdjusted({
                      name: "Compare",
                      title: "Max Age",
                      list: upToDate,
                    }),
                    ...upToDate.map(createCohortFolderWithAdjusted),
                  ],
                },
                // Older Than (≥ X old)
                {
                  name: "Older Than",
                  tree: [
                    createCohortFolderMinAge({
                      name: "Compare",
                      title: "Min Age",
                      list: fromDate,
                    }),
                    ...fromDate.map(createCohortFolderMinAge),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderAgeRange({
                      name: "Compare",
                      title: "Age Ranges",
                      list: dateRange,
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
                    createCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Max Size",
                      list: utxosUnderAmount,
                    }),
                    ...utxosUnderAmount.map(
                      createCohortFolderBasicWithMarketCap,
                    ),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createCohortFolderBasicWithMarketCap({
                      name: "Compare",
                      title: "Min Size",
                      list: utxosAboveAmount,
                    }),
                    ...utxosAboveAmount.map(
                      createCohortFolderBasicWithMarketCap,
                    ),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderBasicWithoutMarketCap({
                      name: "Compare",
                      title: "Size Ranges",
                      list: utxosAmountRanges,
                    }),
                    ...utxosAmountRanges.map(
                      createCohortFolderBasicWithoutMarketCap,
                    ),
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
                    createAddressCohortFolder({
                      name: "Compare",
                      title: "Max Balance",
                      list: addressesUnderAmount,
                    }),
                    ...addressesUnderAmount.map(createAddressCohortFolder),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createAddressCohortFolder({
                      name: "Compare",
                      title: "Min Balance",
                      list: addressesAboveAmount,
                    }),
                    ...addressesAboveAmount.map(createAddressCohortFolder),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createAddressCohortFolder({
                      name: "Compare",
                      title: "Balance Ranges",
                      list: addressesAmountRanges,
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
                createCohortFolderAddress({
                  name: "Compare",
                  title: "Script Types",
                  list: typeAddressable,
                }),
                ...typeAddressable.map(createCohortFolderAddress),
                ...typeOther.map(createCohortFolderWithoutRelative),
              ],
            },

            // Epochs - CohortBasicWithoutMarketCap (no RelToMarketCap)
            {
              name: "Epochs",
              tree: [
                createCohortFolderBasicWithoutMarketCap({
                  name: "Compare",
                  title: "Epochs",
                  list: epoch,
                }),
                ...epoch.map(createCohortFolderBasicWithoutMarketCap),
              ],
            },

            // Years - CohortBasicWithoutMarketCap (no RelToMarketCap)
            {
              name: "Years",
              tree: [
                createCohortFolderBasicWithoutMarketCap({
                  name: "Compare",
                  title: "Years",
                  list: year,
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

    // Table section (disabled)
    // {
    //   kind: /** @type {const} */ ("table"),
    //   title: "Table",
    //   name: "Table",
    // },

    // Simulations section (disabled)
    // {
    //   name: "Simulations",
    //   tree: [
    //     {
    //       kind: /** @type {const} */ ("simulation"),
    //       name: "Save In Bitcoin",
    //       title: "Save In Bitcoin",
    //     },
    //   ],
    // },

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
