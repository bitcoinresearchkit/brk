/** Network section - On-chain activity and health */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, dots } from "./series.js";
import { satsBtcUsd } from "./shared.js";
import { spendableTypeColors } from "./colors/index.js";

/**
 * Create Network section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createNetworkSection(ctx) {
  const {
    colors,
    brk,
    fromSumStatsPattern,
    fromBaseStatsPattern,
    fromFullStatsPattern,
    fromStatsPattern,
    fromCoinbasePattern,
    fromValuePattern,
    fromCountPattern,
    fromSupplyPattern,
  } = ctx;
  const {
    blocks,
    transactions,
    inputs,
    outputs,
    scripts,
    supply,
    distribution,
  } = brk.metrics;

  // Address types for mapping (using spendableTypeColors for consistency)
  /** @type {ReadonlyArray<{key: AddressableType, name: string, color: Color, defaultActive?: boolean}>} */
  const addressTypes = [
    { key: "p2pkh", name: "P2PKH", color: colors[spendableTypeColors.p2pkh] },
    { key: "p2sh", name: "P2SH", color: colors[spendableTypeColors.p2sh] },
    { key: "p2wpkh", name: "P2WPKH", color: colors[spendableTypeColors.p2wpkh] },
    { key: "p2wsh", name: "P2WSH", color: colors[spendableTypeColors.p2wsh] },
    { key: "p2tr", name: "P2TR", color: colors[spendableTypeColors.p2tr] },
    { key: "p2pk65", name: "P2PK65", color: colors[spendableTypeColors.p2pk65], defaultActive: false },
    { key: "p2pk33", name: "P2PK33", color: colors[spendableTypeColors.p2pk33], defaultActive: false },
    { key: "p2a", name: "P2A", color: colors[spendableTypeColors.p2a], defaultActive: false },
  ];

  // Script types for output count comparisons (address types + non-addressable scripts)
  /** @type {ReadonlyArray<{key: AddressableType | "p2ms" | "opreturn" | "emptyoutput" | "unknownoutput", name: string, color: Color, defaultActive?: boolean}>} */
  const scriptTypes = [
    { key: "p2pkh", name: "P2PKH", color: colors[spendableTypeColors.p2pkh] },
    { key: "p2sh", name: "P2SH", color: colors[spendableTypeColors.p2sh] },
    { key: "p2wpkh", name: "P2WPKH", color: colors[spendableTypeColors.p2wpkh] },
    { key: "p2wsh", name: "P2WSH", color: colors[spendableTypeColors.p2wsh] },
    { key: "p2tr", name: "P2TR", color: colors[spendableTypeColors.p2tr] },
    { key: "p2pk65", name: "P2PK65", color: colors[spendableTypeColors.p2pk65], defaultActive: false },
    { key: "p2pk33", name: "P2PK33", color: colors[spendableTypeColors.p2pk33], defaultActive: false },
    { key: "p2a", name: "P2A", color: colors[spendableTypeColors.p2a], defaultActive: false },
    { key: "p2ms", name: "P2MS", color: colors[spendableTypeColors.p2ms], defaultActive: false },
    { key: "opreturn", name: "OP_RETURN", color: colors[spendableTypeColors.opreturn], defaultActive: false },
    { key: "emptyoutput", name: "Empty", color: colors[spendableTypeColors.empty], defaultActive: false },
    { key: "unknownoutput", name: "Unknown", color: colors[spendableTypeColors.unknown], defaultActive: false },
  ];

  // Activity types for mapping
  /** @type {ReadonlyArray<{key: "sending" | "receiving" | "both" | "reactivated" | "balanceIncreased" | "balanceDecreased", name: string, title: string, compareTitle: string}>} */
  const activityTypes = [
    { key: "sending", name: "Sending", title: "Sending Address Count", compareTitle: "Sending Address Count by Type" },
    { key: "receiving", name: "Receiving", title: "Receiving Address Count", compareTitle: "Receiving Address Count by Type" },
    { key: "both", name: "Both", title: "Addresses Sending & Receiving (Same Block)", compareTitle: "Addresses Sending & Receiving by Type" },
    { key: "reactivated", name: "Reactivated", title: "Reactivated Address Count (Was Empty)", compareTitle: "Reactivated Address Count by Type" },
    { key: "balanceIncreased", name: "Balance Increased", title: "Addresses with Increased Balance", compareTitle: "Addresses with Increased Balance by Type" },
    { key: "balanceDecreased", name: "Balance Decreased", title: "Addresses with Decreased Balance", compareTitle: "Addresses with Decreased Balance by Type" },
  ];

  // Count types for comparison charts
  /** @type {ReadonlyArray<{key: "addrCount" | "emptyAddrCount" | "totalAddrCount", name: string, title: string}>} */
  const countTypes = [
    { key: "addrCount", name: "Loaded", title: "Address Count by Type" },
    { key: "emptyAddrCount", name: "Empty", title: "Empty Address Count by Type" },
    { key: "totalAddrCount", name: "Total", title: "Total Address Count by Type" },
  ];

  /**
   * Create address metrics tree for a given type key
   * @param {AddressableType | "all"} key
   * @param {string} titlePrefix
   */
  const createAddressMetricsTree = (key, titlePrefix) => [
    {
      name: "Count",
      title: `${titlePrefix}Address Count`,
      bottom: [
        line({
          metric: distribution.addrCount[key],
          name: "Loaded",
          unit: Unit.count,
        }),
        line({
          metric: distribution.totalAddrCount[key],
          name: "Total",
          color: colors.default,
          unit: Unit.count,
          defaultActive: false,
        }),
        line({
          metric: distribution.emptyAddrCount[key],
          name: "Empty",
          color: colors.gray,
          unit: Unit.count,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "New",
      title: `${titlePrefix}New Address Count`,
      bottom: fromFullStatsPattern({ pattern: distribution.newAddrCount[key], unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
    },
    {
      name: "Growth Rate",
      title: `${titlePrefix}Address Growth Rate`,
      bottom: fromBaseStatsPattern({ pattern: distribution.growthRate[key], unit: Unit.ratio }),
    },
    {
      name: "Activity",
      tree: activityTypes.map((a) => ({
        name: a.name,
        title: `${titlePrefix}${a.name} Address Count`,
        bottom: fromBaseStatsPattern({
          pattern: distribution.addressActivity[key][a.key],
          unit: Unit.count,
        }),
      })),
    },
  ];

  return {
    name: "Network",
    tree: [
      // Supply
      {
        name: "Supply",
        tree: [
          {
            name: "Circulating",
            title: "Circulating Supply",
            bottom: fromSupplyPattern({ pattern: supply.circulating, title: "Supply" }),
          },
          {
            name: "Inflation",
            title: "Inflation Rate",
            bottom: [
              dots({
                metric: supply.inflation,
                name: "Rate",
                unit: Unit.percentage,
              }),
            ],
          },
          {
            name: "Unspendable",
            title: "Unspendable Supply",
            bottom: fromValuePattern({ pattern: supply.burned.unspendable }),
          },
          {
            name: "OP_RETURN",
            title: "OP_RETURN Burned",
            bottom: fromCoinbasePattern({ pattern: scripts.value.opreturn }),
          },
        ],
      },

      // Transactions
      {
        name: "Transactions",
        tree: [
          {
            name: "Count",
            title: "Transaction Count",
            bottom: fromFullStatsPattern({ pattern: transactions.count.txCount, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
          },
          {
            name: "Per Second",
            title: "Transactions Per Second",
            bottom: [
              dots({
                metric: transactions.volume.txPerSec,
                name: "TPS",
                unit: Unit.perSec,
              }),
            ],
          },
          {
            name: "Fee Rate",
            title: "Fee Rate",
            bottom: fromStatsPattern({ pattern: transactions.fees.feeRate, unit: Unit.feeRate }),
          },
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              ...satsBtcUsd({ pattern: transactions.volume.sentSum, name: "Sent" }),
              ...satsBtcUsd({
                pattern: transactions.volume.receivedSum,
                name: "Received",
                color: colors.cyan,
                defaultActive: false,
              }),
              ...satsBtcUsd({
                pattern: transactions.volume.annualizedVolume,
                name: "Annualized",
                color: colors.red,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Size",
            title: "Transaction Size",
            bottom: [
              ...fromStatsPattern({ pattern: transactions.size.weight, unit: Unit.wu }),
              ...fromStatsPattern({ pattern: transactions.size.vsize, unit: Unit.vb }),
            ],
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromCountPattern({
                pattern: transactions.versions.v1,
                unit: Unit.count,
                cumulativeUnit: Unit.countCumulative,
                title: "v1",
                color: colors.orange,
              }),
              ...fromCountPattern({
                pattern: transactions.versions.v2,
                unit: Unit.count,
                cumulativeUnit: Unit.countCumulative,
                title: "v2",
                color: colors.cyan,
              }),
              ...fromCountPattern({
                pattern: transactions.versions.v3,
                unit: Unit.count,
                cumulativeUnit: Unit.countCumulative,
                title: "v3",
                color: colors.lime,
              }),
            ],
          },
          {
            name: "Velocity",
            title: "Transaction Velocity",
            bottom: [
              line({
                metric: supply.velocity.btc,
                name: "Bitcoin",
                unit: Unit.ratio,
              }),
              line({
                metric: supply.velocity.usd,
                name: "Dollars",
                color: colors.emerald,
                unit: Unit.ratio,
              }),
            ],
          },
        ],
      },

      // Blocks
      {
        name: "Blocks",
        tree: [
          {
            name: "Count",
            title: "Block Count",
            bottom: [
              ...fromCountPattern({ pattern: blocks.count.blockCount, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
              line({
                metric: blocks.count.blockCountTarget,
                name: "Target",
                color: colors.gray,
                unit: Unit.count,
                options: { lineStyle: 4 },
              }),
              line({
                metric: blocks.count._24hBlockCount,
                name: "24h sum",
                color: colors.pink,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: blocks.count._1wBlockCount,
                name: "1w sum",
                color: colors.red,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: blocks.count._1mBlockCount,
                name: "1m sum",
                color: colors.orange,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: blocks.count._1yBlockCount,
                name: "1y sum",
                color: colors.purple,
                unit: Unit.count,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Interval",
            title: "Block Interval",
            bottom: [
              ...fromBaseStatsPattern({ pattern: blocks.interval, unit: Unit.secs, avgActive: false }),
              priceLine({ ctx, unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              ...fromSumStatsPattern({ pattern: blocks.size, unit: Unit.bytes, cumulativeUnit: Unit.bytesCumulative }),
              line({
                metric: blocks.totalSize,
                name: "Total",
                color: colors.purple,
                unit: Unit.bytes,
                defaultActive: false,
              }),
              ...fromBaseStatsPattern({ pattern: blocks.vbytes, unit: Unit.vb }),
              ...fromBaseStatsPattern({ pattern: blocks.weight, unit: Unit.wu }),
            ],
          },
          {
            name: "Fullness",
            title: "Block Fullness",
            bottom: fromBaseStatsPattern({ pattern: blocks.fullness, unit: Unit.percentage }),
          },
        ],
      },

      // UTXOs
      {
        name: "UTXOs",
        tree: [
          {
            name: "UTXO Count",
            title: "UTXO Count",
            bottom: [
              line({
                metric: outputs.count.utxoCount,
                name: "Count",
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Inputs",
            tree: [
              {
                name: "Count",
                title: "Input Count",
                bottom: [...fromSumStatsPattern({ pattern: inputs.count, unit: Unit.count, cumulativeUnit: Unit.countCumulative })],
              },
              {
                name: "Rate",
                title: "Inputs Per Second",
                bottom: [
                  dots({
                    metric: transactions.volume.inputsPerSec,
                    name: "Inputs/sec",
                    unit: Unit.perSec,
                  }),
                ],
              },
            ],
          },
          {
            name: "Outputs",
            tree: [
              {
                name: "Count",
                title: "Output Count",
                bottom: [...fromSumStatsPattern({ pattern: outputs.count.totalCount, unit: Unit.count, cumulativeUnit: Unit.countCumulative })],
              },
              {
                name: "Rate",
                title: "Outputs Per Second",
                bottom: [
                  dots({
                    metric: transactions.volume.outputsPerSec,
                    name: "Outputs/sec",
                    unit: Unit.perSec,
                  }),
                ],
              },
            ],
          },
        ],
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          // Overview - global metrics for all addresses
          { name: "Overview", tree: createAddressMetricsTree("all", "") },

          // Compare - cross-type comparisons
          {
            name: "Compare",
            tree: [
              {
                name: "Count",
                tree: countTypes.map((c) => ({
                  name: c.name,
                  title: c.title,
                  bottom: addressTypes.map((t) =>
                    line({
                      metric: distribution[c.key][t.key],
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                  ),
                })),
              },
              {
                name: "New",
                title: "New Address Count by Type",
                bottom: addressTypes.flatMap((t) => [
                  dots({
                    metric: distribution.newAddrCount[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric: distribution.newAddrCount[t.key].average,
                    name: `${t.name} Avg`,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                ]),
              },
              {
                name: "Growth Rate",
                title: "Address Growth Rate by Type",
                bottom: addressTypes.flatMap((t) => [
                  dots({
                    metric: distribution.growthRate[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric: distribution.growthRate[t.key].average,
                    name: `${t.name} Avg`,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: false,
                  }),
                ]),
              },
              {
                name: "Activity",
                tree: activityTypes.map((a) => ({
                  name: a.name,
                  title: a.compareTitle,
                  bottom: addressTypes.flatMap((t) => [
                    dots({
                      metric: distribution.addressActivity[t.key][a.key].base,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                    line({
                      metric: distribution.addressActivity[t.key][a.key].average,
                      name: `${t.name} Avg`,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: false,
                    }),
                  ]),
                })),
              },
            ],
          },

          // Individual address types
          ...addressTypes.map((t) => ({
            name: t.name,
            tree: createAddressMetricsTree(t.key, `${t.name} `),
          })),
        ],
      },

      // Scripts
      {
        name: "Scripts",
        tree: [
          {
            name: "Output Counts",
            tree: [
              // Compare section
              {
                name: "Compare",
                title: "Output Count by Script Type",
                bottom: scriptTypes.map((t) =>
                  line({
                    metric: scripts.count[t.key].cumulative,
                    name: t.name,
                    color: t.color,
                    unit: Unit.countCumulative,
                    defaultActive: t.defaultActive,
                  }),
                ),
              },
              // Legacy scripts
              {
                name: "Legacy",
                tree: [
                  {
                    name: "P2PKH",
                    title: "P2PKH Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2pkh, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2PK33",
                    title: "P2PK33 Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2pk33, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2PK65",
                    title: "P2PK65 Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2pk65, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                ],
              },
              // Script Hash
              {
                name: "Script Hash",
                tree: [
                  {
                    name: "P2SH",
                    title: "P2SH Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2sh, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2MS",
                    title: "P2MS Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2ms, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                ],
              },
              // SegWit scripts
              {
                name: "SegWit",
                tree: [
                  {
                    name: "All SegWit",
                    title: "SegWit Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.segwit, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2WPKH",
                    title: "P2WPKH Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2wpkh, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2WSH",
                    title: "P2WSH Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2wsh, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                ],
              },
              // Taproot scripts
              {
                name: "Taproot",
                tree: [
                  {
                    name: "P2TR",
                    title: "P2TR Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2tr, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "P2A",
                    title: "P2A Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.p2a, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                ],
              },
              // Other scripts
              {
                name: "Other",
                tree: [
                  {
                    name: "OP_RETURN",
                    title: "OP_RETURN Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.opreturn, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "Empty",
                    title: "Empty Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.emptyoutput, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                  {
                    name: "Unknown",
                    title: "Unknown Output Count",
                    bottom: fromFullStatsPattern({ pattern: scripts.count.unknownoutput, unit: Unit.count, cumulativeUnit: Unit.countCumulative }),
                  },
                ],
              },
            ],
          },
          {
            name: "Adoption",
            tree: [
              {
                name: "SegWit",
                title: "SegWit Adoption",
                bottom: [
                  line({
                    metric: scripts.count.segwitAdoption.base,
                    name: "Base",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.segwitAdoption.sum,
                    name: "Sum",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.segwitAdoption.cumulative,
                    name: "Cumulative",
                    color: colors.red,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "Taproot",
                title: "Taproot Adoption",
                bottom: [
                  line({
                    metric: scripts.count.taprootAdoption.base,
                    name: "Base",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.taprootAdoption.sum,
                    name: "Sum",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.taprootAdoption.cumulative,
                    name: "Cumulative",
                    color: colors.red,
                    unit: Unit.percentage,
                  }),
                ],
              },
            ],
          },
        ],
      },

    ],
  };
}
