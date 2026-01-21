/** Partial options - Main entry point */

import { createContext } from "./context.js";
import {
  buildCohortData,
  createCohortFolderAll,
  createCohortFolderFull,
  createCohortFolderWithAdjusted,
  createCohortFolderWithPercentiles,
  createCohortFolderBasic,
  createAddressCohortFolder,
} from "./cohorts/index.js";
import { createMarketSection } from "./market/index.js";
import { createChainSection } from "./chain.js";
import { createCointimeSection } from "./cointime.js";
import { colors } from "../chart/colors.js";

// Re-export types for external consumers
export * from "./types.js";

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
    type,
    year,
  } = buildCohortData(colors, brk);

  // Helpers to map cohorts by capability type
  /** @param {CohortWithAdjusted} cohort */
  const mapWithAdjusted = (cohort) => createCohortFolderWithAdjusted(ctx, cohort);
  /** @param {CohortWithPercentiles} cohort */
  const mapWithPercentiles = (cohort) => createCohortFolderWithPercentiles(ctx, cohort);
  /** @param {CohortBasic} cohort */
  const mapBasic = (cohort) => createCohortFolderBasic(ctx, cohort);
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
          name: "Cohorts",
          tree: [
            // All UTXOs - CohortAll (adjustedSopr + percentiles but no RelToMarketCap)
            createCohortFolderAll(ctx, cohortAll),

            // Terms (STH/LTH) - Short is Full, Long is WithPercentiles
            {
              name: "Terms",
              tree: [
                // Individual cohorts with their specific capabilities
                createCohortFolderFull(ctx, termShort),
                createCohortFolderWithPercentiles(ctx, termLong),
              ],
            },

            // Types - CohortBasic
            {
              name: "Types",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "Type",
                  list: type,
                }),
                ...type.map(mapBasic),
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
                    createCohortFolderBasic(ctx, {
                      name: "Compare",
                      title: "Age At Least",
                      list: fromDate,
                    }),
                    ...fromDate.map(mapBasic),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderWithPercentiles(ctx, {
                      name: "Compare",
                      title: "Age Range",
                      list: dateRange,
                    }),
                    ...dateRange.map(mapWithPercentiles),
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
                    createCohortFolderBasic(ctx, {
                      name: "Compare",
                      title: "Amount Under",
                      list: utxosUnderAmount,
                    }),
                    ...utxosUnderAmount.map(mapBasic),
                  ],
                },
                // Above (≥ X sats)
                {
                  name: "Above",
                  tree: [
                    createCohortFolderBasic(ctx, {
                      name: "Compare",
                      title: "Amount Above",
                      list: utxosAboveAmount,
                    }),
                    ...utxosAboveAmount.map(mapBasic),
                  ],
                },
                // Range
                {
                  name: "Range",
                  tree: [
                    createCohortFolderBasic(ctx, {
                      name: "Compare",
                      title: "Amount Range",
                      list: utxosAmountRanges,
                    }),
                    ...utxosAmountRanges.map(mapBasic),
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

            // Epochs - CohortBasic
            {
              name: "Epochs",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "Epoch",
                  list: epoch,
                }),
                ...epoch.map(mapBasic),
              ],
            },

            // Years - CohortBasic
            {
              name: "Years",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "Year",
                  list: year,
                }),
                ...year.map(mapBasic),
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
