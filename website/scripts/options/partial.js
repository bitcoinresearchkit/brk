/** Partial options - Main entry point */

import { createContext } from "./context.js";
import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderWithPercentiles,
  createCohortFolderLongTerm,
  createCohortFolderAgeRange,
  createCohortFolderBasicWithMarketCap,
  createCohortFolderBasicWithoutMarketCap,
  createCohortFolderAddress,
  createAddressCohortFolder,
} from "./distribution/index.js";
import { createMarketSection } from "./market/index.js";
import { createChainSection } from "./chain.js";
import { createCointimeSection } from "./cointime.js";
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
  /** @param {CohortBasicWithoutMarketCap} cohort */
  const mapBasicWithoutMarketCap = (cohort) =>
    createCohortFolderBasicWithoutMarketCap(ctx, cohort);
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

        // Chain section
        createChainSection(ctx),

        // Cohorts section
        {
          name: "Distribution",
          tree: [
            // All UTXOs - CohortAll (adjustedSopr + percentiles but no RelToMarketCap)
            createCohortFolderAll(ctx, cohortAll),

            // Terms (STH/LTH) - Short is Full, Long is LongTerm
            {
              name: "Terms",
              tree: [
                // Compare folder uses WithPercentiles (common capabilities)
                createCohortFolderWithPercentiles(ctx, {
                  name: "Compare",
                  title: "Term",
                  list: [termShort, termLong],
                }),
                // Individual cohorts with their specific capabilities
                createCohortFolderFull(ctx, termShort),
                createCohortFolderLongTerm(ctx, termLong),
              ],
            },

            // Types - addressable types have addrCount, others don't
            {
              name: "Types",
              tree: [
                createCohortFolderAddress(ctx, {
                  name: "Compare",
                  title: "Type",
                  list: typeAddressable,
                }),
                ...typeAddressable.map(mapAddress),
                ...typeOther.map(mapBasicWithoutMarketCap),
              ],
            },

            // Age cohorts
            {
              name: "Age",
              tree: [
                // Up To (< X old)
                {
                  name: "Up To",
                  tree: [
                    createCohortFolderWithAdjusted(ctx, {
                      name: "Compare",
                      title: "Age Up To",
                      list: upToDate,
                    }),
                    ...upToDate.map(mapWithAdjusted),
                  ],
                },
                // At Least (≥ X old)
                {
                  name: "At Least",
                  tree: [
                    createCohortFolderBasicWithMarketCap(ctx, {
                      name: "Compare",
                      title: "Age At Least",
                      list: fromDate,
                    }),
                    ...fromDate.map(mapBasicWithMarketCap),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderAgeRange(ctx, {
                      name: "Compare",
                      title: "Age Range",
                      list: dateRange,
                    }),
                    ...dateRange.map(mapAgeRange),
                  ],
                },
              ],
            },

            // Amount cohorts (UTXO size)
            {
              name: "Amount",
              tree: [
                // Under (< X sats)
                {
                  name: "Under",
                  tree: [
                    createCohortFolderBasicWithMarketCap(ctx, {
                      name: "Compare",
                      title: "Amount Under",
                      list: utxosUnderAmount,
                    }),
                    ...utxosUnderAmount.map(mapBasicWithMarketCap),
                  ],
                },
                // Above (≥ X sats)
                {
                  name: "Above",
                  tree: [
                    createCohortFolderBasicWithMarketCap(ctx, {
                      name: "Compare",
                      title: "Amount Above",
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
                      title: "Amount Range",
                      list: utxosAmountRanges,
                    }),
                    ...utxosAmountRanges.map(mapBasicWithoutMarketCap),
                  ],
                },
              ],
            },

            // Balance cohorts (Address balance)
            {
              name: "Balance",
              tree: [
                // Under (< X sats)
                {
                  name: "Under",
                  tree: [
                    createAddressCohortFolder(ctx, {
                      name: "Compare",
                      title: "Balance Under",
                      list: addressesUnderAmount,
                    }),
                    ...addressesUnderAmount.map(mapAddressCohorts),
                  ],
                },
                // Above (≥ X sats)
                {
                  name: "Above",
                  tree: [
                    createAddressCohortFolder(ctx, {
                      name: "Compare",
                      title: "Balance Above",
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
                      title: "Balance Range",
                      list: addressesAmountRanges,
                    }),
                    ...addressesAmountRanges.map(mapAddressCohorts),
                  ],
                },
              ],
            },

            // Epochs - CohortBasicWithoutMarketCap (no RelToMarketCap)
            {
              name: "Epochs",
              tree: [
                createCohortFolderBasicWithoutMarketCap(ctx, {
                  name: "Compare",
                  title: "Epoch",
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
                  title: "Year",
                  list: year,
                }),
                ...year.map(mapBasicWithoutMarketCap),
              ],
            },
          ],
        },

        // Cointime section
        createCointimeSection(ctx),
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
