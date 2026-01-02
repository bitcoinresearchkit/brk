/** Partial options - Main entry point */

import { localhost } from "../../utils/env.js";
import { createContext } from "./context.js";
import {
  buildCohortData,
  createUtxoCohortFolder,
  createAddressCohortFolder,
} from "./cohorts/index.js";
import { createMarketSection } from "./market.js";
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
    cohortAllForComparison,
    terms,
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

  // Helper to map UTXO cohorts
  const mapUtxoCohorts = (/** @type {any} */ cohort) => createUtxoCohortFolder(ctx, cohort);

  // Helper to map Address cohorts
  const mapAddressCohorts = (/** @type {any} */ cohort) => createAddressCohortFolder(ctx, cohort);

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
            // All UTXOs
            createUtxoCohortFolder(ctx, cohortAll),

            // Terms (STH/LTH)
            {
              name: "terms",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs Term",
                  list: [...terms, cohortAllForComparison],
                }),
                ...terms.map(mapUtxoCohorts),
              ],
            },

            // Epochs
            {
              name: "Epochs",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "Epoch",
                  list: [...epoch, cohortAllForComparison],
                }),
                ...epoch.map(mapUtxoCohorts),
              ],
            },

            // Types
            {
              name: "types",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "Type",
                  list: [...type, cohortAllForComparison],
                }),
                ...type.map(mapUtxoCohorts),
              ],
            },

            // UTXOs Up to age
            {
              name: "UTXOs Up to age",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs Up To Age",
                  list: [...upToDate, cohortAllForComparison],
                }),
                ...upToDate.map(mapUtxoCohorts),
              ],
            },

            // UTXOs from age
            {
              name: "UTXOs from age",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs from age",
                  list: [...fromDate, cohortAllForComparison],
                }),
                ...fromDate.map(mapUtxoCohorts),
              ],
            },

            // UTXOs age ranges
            {
              name: "UTXOs age Ranges",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs Age Range",
                  list: [...dateRange, cohortAllForComparison],
                }),
                ...dateRange.map(mapUtxoCohorts),
              ],
            },

            // UTXOs under amounts
            {
              name: "UTXOs under amounts",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs under amount",
                  list: [...utxosUnderAmount, cohortAllForComparison],
                }),
                ...utxosUnderAmount.map(mapUtxoCohorts),
              ],
            },

            // UTXOs above amounts
            {
              name: "UTXOs Above Amounts",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs Above Amount",
                  list: [...utxosAboveAmount, cohortAllForComparison],
                }),
                ...utxosAboveAmount.map(mapUtxoCohorts),
              ],
            },

            // UTXOs between amounts
            {
              name: "UTXOs between amounts",
              tree: [
                createUtxoCohortFolder(ctx, {
                  name: "Compare",
                  title: "UTXOs between amounts",
                  list: [...utxosAmountRanges, cohortAllForComparison],
                }),
                ...utxosAmountRanges.map(mapUtxoCohorts),
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
