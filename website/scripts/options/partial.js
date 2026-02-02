/** Partial options - Main entry point */

import { createContext } from "./context.js";
import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderWithNupl,
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
import { colors } from "../chart/colors.js";

// Re-export types for external consumers
export * from "./types.js";
export * from "./context.js";

/**
 * Create partial options tree
 * @param {Object} args
 * @param {BrkClient} args.brk
 * @returns {PartialOptionsTree}
 */
export function createPartialOptions({ brk }) {
  // Create context with all helpers
  const ctx = createContext({ brk });

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
  } = buildCohortData(colors, brk);

  // Helpers to map cohorts by capability type
  /** @param {CohortWithAdjusted} cohort */
  const mapWithAdjusted = (cohort) =>
    createCohortFolderWithAdjusted(ctx, cohort);
  /** @param {CohortAgeRange} cohort */
  const mapAgeRange = (cohort) => createCohortFolderAgeRange(ctx, cohort);
  /** @param {CohortBasicWithMarketCap} cohort */
  const mapBasicWithMarketCap = (cohort) =>
    createCohortFolderBasicWithMarketCap(ctx, cohort);
  /** @param {CohortMinAge} cohort */
  const mapMinAge = (cohort) => createCohortFolderMinAge(ctx, cohort);
  /** @param {CohortBasicWithoutMarketCap} cohort */
  const mapBasicWithoutMarketCap = (cohort) =>
    createCohortFolderBasicWithoutMarketCap(ctx, cohort);
  /** @param {CohortWithoutRelative} cohort */
  const mapWithoutRelative = (cohort) =>
    createCohortFolderWithoutRelative(ctx, cohort);
  /** @param {CohortAddress} cohort */
  const mapAddress = (cohort) => createCohortFolderAddress(ctx, cohort);
  /** @param {AddressCohortObject} cohort */
  const mapAddressCohorts = (cohort) => createAddressCohortFolder(ctx, cohort);

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
        createMarketSection(ctx),

        // Network section (on-chain activity)
        createNetworkSection(ctx),

        // Mining section (security & economics)
        createMiningSection(ctx),

        // Cohorts section
        {
          name: "Distribution",
          tree: [
            // Overview - All UTXOs (adjustedSopr + percentiles but no RelToMarketCap)
            createCohortFolderAll(ctx, { ...cohortAll, name: "Overview" }),

            // STH - Short term holder cohort (Full capability)
            createCohortFolderFull(ctx, termShort),

            // LTH - Long term holder cohort (nupl)
            createCohortFolderWithNupl(ctx, termLong),

            // STH vs LTH - Direct comparison
            createCohortFolderWithNupl(ctx, {
              name: "STH vs LTH",
              title: "STH vs LTH",
              list: [termShort, termLong],
            }),

            // Ages cohorts
            {
              name: "Ages",
              tree: [
                // Younger Than (< X old)
                {
                  name: "Younger Than",
                  tree: [
                    createCohortFolderWithAdjusted(ctx, {
                      name: "Compare",
                      title: "Max Age",
                      list: upToDate,
                    }),
                    ...upToDate.map(mapWithAdjusted),
                  ],
                },
                // Older Than (≥ X old)
                {
                  name: "Older Than",
                  tree: [
                    createCohortFolderMinAge(ctx, {
                      name: "Compare",
                      title: "Min Age",
                      list: fromDate,
                    }),
                    ...fromDate.map(mapMinAge),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderAgeRange(ctx, {
                      name: "Compare",
                      title: "Age Ranges",
                      list: dateRange,
                    }),
                    ...dateRange.map(mapAgeRange),
                  ],
                },
              ],
            },

            // Sizes cohorts (UTXO size)
            {
              name: "Sizes",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createCohortFolderBasicWithMarketCap(ctx, {
                      name: "Compare",
                      title: "Max Size",
                      list: utxosUnderAmount,
                    }),
                    ...utxosUnderAmount.map(mapBasicWithMarketCap),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createCohortFolderBasicWithMarketCap(ctx, {
                      name: "Compare",
                      title: "Min Size",
                      list: utxosAboveAmount,
                    }),
                    ...utxosAboveAmount.map(mapBasicWithMarketCap),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderBasicWithoutMarketCap(ctx, {
                      name: "Compare",
                      title: "Size Ranges",
                      list: utxosAmountRanges,
                    }),
                    ...utxosAmountRanges.map(mapBasicWithoutMarketCap),
                  ],
                },
              ],
            },

            // Balances cohorts (Address balance)
            {
              name: "Balances",
              tree: [
                // Less Than (< X sats)
                {
                  name: "Less Than",
                  tree: [
                    createAddressCohortFolder(ctx, {
                      name: "Compare",
                      title: "Max Balance",
                      list: addressesUnderAmount,
                    }),
                    ...addressesUnderAmount.map(mapAddressCohorts),
                  ],
                },
                // More Than (≥ X sats)
                {
                  name: "More Than",
                  tree: [
                    createAddressCohortFolder(ctx, {
                      name: "Compare",
                      title: "Min Balance",
                      list: addressesAboveAmount,
                    }),
                    ...addressesAboveAmount.map(mapAddressCohorts),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createAddressCohortFolder(ctx, {
                      name: "Compare",
                      title: "Balance Ranges",
                      list: addressesAmountRanges,
                    }),
                    ...addressesAmountRanges.map(mapAddressCohorts),
                  ],
                },
              ],
            },

            // Script Types - addressable types have addrCount, others don't
            {
              name: "Script Types",
              tree: [
                createCohortFolderAddress(ctx, {
                  name: "Compare",
                  title: "Script Types",
                  list: typeAddressable,
                }),
                ...typeAddressable.map(mapAddress),
                ...typeOther.map(mapWithoutRelative),
              ],
            },

            // Epochs - CohortBasicWithoutMarketCap (no RelToMarketCap)
            {
              name: "Epochs",
              tree: [
                createCohortFolderBasicWithoutMarketCap(ctx, {
                  name: "Compare",
                  title: "Epochs",
                  list: epoch,
                }),
                ...epoch.map(mapBasicWithoutMarketCap),
              ],
            },

            // Years - CohortBasicWithoutMarketCap (no RelToMarketCap)
            {
              name: "Years",
              tree: [
                createCohortFolderBasicWithoutMarketCap(ctx, {
                  name: "Compare",
                  title: "Years",
                  list: year,
                }),
                ...year.map(mapBasicWithoutMarketCap),
              ],
            },
          ],
        },

        // Frameworks section
        {
          name: "Frameworks",
          tree: [createCointimeSection(ctx)],
        },

        // Investing section
        createInvestingSection(ctx),
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
