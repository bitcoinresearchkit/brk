/** Partial options - Main entry point */

import { localhost } from "../utils/env.js";
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

// Re-export types for external consumers
export * from "./types.js";

/**
 * Create partial options tree
 * @param {Object} args
 * @param {Colors} args.colors
 * @param {BrkClient} args.brk
 * @returns {PartialOptionsTree}
 */
export function createPartialOptions({ colors, brk }) {
  // Create context with all helpers
  const ctx = createContext({ colors, brk });

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
    // Debug explorer (localhost only)
    ...(localhost
      ? [
          {
            kind: /** @type {const} */ ("explorer"),
            name: "Explorer",
            title: "Debug explorer",
          },
        ]
      : []),

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
              name: "terms",
              tree: [
                // Individual cohorts with their specific capabilities
                createCohortFolderFull(ctx, termShort),
                createCohortFolderWithPercentiles(ctx, termLong),
              ],
            },

            // Epochs - CohortBasic (neither adjustedSopr nor percentiles)
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

            // Types - CohortBasic
            {
              name: "types",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "Type",
                  list: type,
                }),
                ...type.map(mapBasic),
              ],
            },

            // UTXOs Up to age - CohortWithAdjusted (adjustedSopr only)
            {
              name: "UTXOs Up to age",
              tree: [
                createCohortFolderWithAdjusted(ctx, {
                  name: "Compare",
                  title: "UTXOs Up To Age",
                  list: upToDate,
                }),
                ...upToDate.map(mapWithAdjusted),
              ],
            },

            // UTXOs from age - CohortBasic
            {
              name: "UTXOs from age",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "UTXOs from age",
                  list: fromDate,
                }),
                ...fromDate.map(mapBasic),
              ],
            },

            // UTXOs age ranges - CohortWithPercentiles (percentiles only)
            {
              name: "UTXOs age Ranges",
              tree: [
                createCohortFolderWithPercentiles(ctx, {
                  name: "Compare",
                  title: "UTXOs Age Range",
                  list: dateRange,
                }),
                ...dateRange.map(mapWithPercentiles),
              ],
            },

            // UTXOs under amounts - CohortBasic
            {
              name: "UTXOs under amounts",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "UTXOs under amount",
                  list: utxosUnderAmount,
                }),
                ...utxosUnderAmount.map(mapBasic),
              ],
            },

            // UTXOs above amounts - CohortBasic
            {
              name: "UTXOs Above Amounts",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "UTXOs Above Amount",
                  list: utxosAboveAmount,
                }),
                ...utxosAboveAmount.map(mapBasic),
              ],
            },

            // UTXOs between amounts - CohortBasic
            {
              name: "UTXOs between amounts",
              tree: [
                createCohortFolderBasic(ctx, {
                  name: "Compare",
                  title: "UTXOs between amounts",
                  list: utxosAmountRanges,
                }),
                ...utxosAmountRanges.map(mapBasic),
              ],
            },

            // Addresses under amount (TYPE SAFE - uses createAddressCohortFolder!)
            {
              name: "Addresses under amount",
              tree: [
                createAddressCohortFolder(ctx, {
                  name: "Compare",
                  title: "Addresses under Amount",
                  list: addressesUnderAmount,
                }),
                ...addressesUnderAmount.map(mapAddressCohorts),
              ],
            },

            // Addresses above amount (TYPE SAFE - uses createAddressCohortFolder!)
            {
              name: "Addresses above amount",
              tree: [
                createAddressCohortFolder(ctx, {
                  name: "Compare",
                  title: "Addresses above amount",
                  list: addressesAboveAmount,
                }),
                ...addressesAboveAmount.map(mapAddressCohorts),
              ],
            },

            // Addresses between amounts (TYPE SAFE - uses createAddressCohortFolder!)
            {
              name: "Addresses between amounts",
              tree: [
                createAddressCohortFolder(ctx, {
                  name: "Compare",
                  title: "Addresses between amounts",
                  list: addressesAmountRanges,
                }),
                ...addressesAmountRanges.map(mapAddressCohorts),
              ],
            },
          ],
        },

        // Cointime section
        createCointimeSection(ctx),
      ],
    },

    // Table section
    {
      kind: /** @type {const} */ ("table"),
      title: "Table",
      name: "Table",
    },

    // Simulations section
    {
      name: "Simulations",
      tree: [
        {
          kind: /** @type {const} */ ("simulation"),
          name: "Save In Bitcoin",
          title: "Save In Bitcoin",
        },
      ],
    },

    // Tools section
    {
      name: "Tools",
      tree: [
        {
          name: "Documentation",
          tree: [
            {
              name: "API",
              url: () => "/api",
              title: "API documentation",
            },
            {
              name: "MCP",
              url: () =>
                "https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_mcp/README.md#brk_mcp",
              title: "Model Context Protocol documentation",
            },
            {
              name: "Crate",
              url: () => "/crate",
              title: "View on crates.io",
            },
            {
              name: "Source",
              url: () => "/github",
              title: "Source code and issues",
            },
            {
              name: "Changelog",
              url: () => "/changelog",
              title: "Release notes and changelog",
            },
          ],
        },
        {
          name: "Hosting",
          tree: [
            {
              name: "Status",
              url: () => "/status",
              title: "Service status and uptime",
            },
            {
              name: "Self-host",
              url: () => "/install",
              title: "Install and run yourself",
            },
            {
              name: "Service",
              url: () => "/service",
              title: "Hosted service offering",
            },
          ],
        },
        {
          name: "Community",
          tree: [
            {
              name: "Discord",
              url: () => "/discord",
              title: "Join the Discord server",
            },
            {
              name: "GitHub",
              url: () => "/github",
              title: "Source code and issues",
            },
            {
              name: "Nostr",
              url: () => "/nostr",
              title: "Follow on Nostr",
            },
          ],
        },
      ],
    },

    // Donate
    {
      name: "Donate",
      qrcode: true,
      url: () => "bitcoin:bc1q098zsm89m7kgyze338vfejhpdt92ua9p3peuve",
      title: "Bitcoin address for donations",
    },

    // Share
    {
      name: "Share",
      qrcode: true,
      url: () => window.location.href,
      title: "Share",
    },
  ];
}
