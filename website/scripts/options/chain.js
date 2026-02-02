/** Chain section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, baseline, dots, dotted } from "./series.js";
import { satsBtcUsd } from "./shared.js";
import { spendableTypeColors } from "./colors/index.js";

/** Major pools to show in Compare section (by current hashrate dominance) */
const MAJOR_POOL_IDS = [
  "foundryusa", // ~32% - largest pool
  "antpool", // ~18% - Bitmain-owned
  "viabtc", // ~14% - independent
  "f2pool", // ~10% - one of the oldest pools
  "marapool", // MARA Holdings
  "braiinspool", // formerly Slush Pool
  "spiderpool", // growing Asian pool
  "ocean", // decentralization-focused
];

/**
 * AntPool & friends - pools sharing AntPool's block templates
 * Based on b10c's research: https://b10c.me/blog/015-bitcoin-mining-centralization/
 * Collectively ~35-40% of network hashrate
 */
const ANTPOOL_AND_FRIENDS_IDS = [
  "antpool", // Bitmain-owned, template source
  "poolin", // shares AntPool templates
  "btccom", // CloverPool (formerly BTC.com)
  "braiinspool", // shares AntPool templates
  "ultimuspool", // shares AntPool templates
  "binancepool", // shares AntPool templates
  "secpool", // shares AntPool templates
  "sigmapoolcom", // SigmaPool
  "rawpool", // shares AntPool templates
  "luxor", // shares AntPool templates
];

/**
 * Create Chain section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createChainSection(ctx) {
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
    pools,
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
    {
      key: "p2wpkh",
      name: "P2WPKH",
      color: colors[spendableTypeColors.p2wpkh],
    },
    { key: "p2wsh", name: "P2WSH", color: colors[spendableTypeColors.p2wsh] },
    { key: "p2tr", name: "P2TR", color: colors[spendableTypeColors.p2tr] },
    {
      key: "p2pk65",
      name: "P2PK65",
      color: colors[spendableTypeColors.p2pk65],
      defaultActive: false,
    },
    {
      key: "p2pk33",
      name: "P2PK33",
      color: colors[spendableTypeColors.p2pk33],
      defaultActive: false,
    },
    {
      key: "p2a",
      name: "P2A",
      color: colors[spendableTypeColors.p2a],
      defaultActive: false,
    },
  ];

  // Activity types for mapping
  /** @type {ReadonlyArray<{key: "sending" | "receiving" | "both" | "reactivated" | "balanceIncreased" | "balanceDecreased", name: string, title: string, compareTitle: string}>} */
  const activityTypes = [
    {
      key: "sending",
      name: "Sending",
      title: "Sending Address Count",
      compareTitle: "Sending Address Count by Type",
    },
    {
      key: "receiving",
      name: "Receiving",
      title: "Receiving Address Count",
      compareTitle: "Receiving Address Count by Type",
    },
    {
      key: "both",
      name: "Both",
      title: "Addresses Sending & Receiving (Same Block)",
      compareTitle: "Addresses Sending & Receiving by Type",
    },
    {
      key: "reactivated",
      name: "Reactivated",
      title: "Reactivated Address Count (Was Empty)",
      compareTitle: "Reactivated Address Count by Type",
    },
    {
      key: "balanceIncreased",
      name: "Balance Increased",
      title: "Addresses with Increased Balance",
      compareTitle: "Addresses with Increased Balance by Type",
    },
    {
      key: "balanceDecreased",
      name: "Balance Decreased",
      title: "Addresses with Decreased Balance",
      compareTitle: "Addresses with Decreased Balance by Type",
    },
  ];

  // Count types for comparison charts
  /** @type {ReadonlyArray<{key: "addrCount" | "emptyAddrCount" | "totalAddrCount", name: string, title: string}>} */
  const countTypes = [
    { key: "addrCount", name: "Loaded", title: "Address Count by Type" },
    {
      key: "emptyAddrCount",
      name: "Empty",
      title: "Empty Address Count by Type",
    },
    {
      key: "totalAddrCount",
      name: "Total",
      title: "Total Address Count by Type",
    },
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
      bottom: fromFullStatsPattern({
        pattern: distribution.newAddrCount[key],
        unit: Unit.count,
      }),
    },
    {
      name: "Growth Rate",
      title: `${titlePrefix}Address Growth Rate`,
      bottom: fromBaseStatsPattern({
        pattern: distribution.growthRate[key],
        unit: Unit.ratio,
      }),
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

  // Build pools tree dynamically
  const poolEntries = Object.entries(pools.vecs);
  const poolsTree = poolEntries.map(([key, pool]) => {
    const poolName =
      brk.POOL_ID_TO_POOL_NAME[
        /** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (key.toLowerCase())
      ] || key;
    return {
      name: poolName,
      tree: [
        {
          name: "Dominance",
          title: `${poolName} Dominance`,
          bottom: [
            dots({
              metric: pool._24hDominance,
              name: "24h",
              color: colors.pink,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            line({
              metric: pool._1wDominance,
              name: "1w",
              color: colors.red,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            line({
              metric: pool._1mDominance,
              name: "1m",
              unit: Unit.percentage,
            }),
            line({
              metric: pool._1yDominance,
              name: "1y",
              color: colors.lime,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            line({
              metric: pool.dominance,
              name: "All Time",
              color: colors.teal,
              unit: Unit.percentage,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Blocks mined",
          title: `${poolName} Blocks`,
          bottom: [
            dots({
              metric: pool.blocksMined.sum,
              name: "Sum",
              unit: Unit.count,
            }),
            line({
              metric: pool.blocksMined.cumulative,
              name: "Cumulative",
              color: colors.blue,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
              metric: pool._24hBlocksMined,
              name: "24h sum",
              color: colors.pink,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
              metric: pool._1wBlocksMined,
              name: "1w sum",
              color: colors.red,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
              metric: pool._1mBlocksMined,
              name: "1m sum",
              color: colors.pink,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
              metric: pool._1yBlocksMined,
              name: "1y sum",
              color: colors.purple,
              unit: Unit.count,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Rewards",
          title: `${poolName} Rewards`,
          bottom: [
            ...fromValuePattern({
              pattern: pool.coinbase,
              title: "coinbase",
              sumColor: colors.orange,
              cumulativeColor: colors.red,
            }),
            ...fromValuePattern({
              pattern: pool.subsidy,
              title: "subsidy",
              sumColor: colors.lime,
              cumulativeColor: colors.emerald,
            }),
            ...fromValuePattern({
              pattern: pool.fee,
              title: "fee",
              sumColor: colors.cyan,
              cumulativeColor: colors.indigo,
            }),
          ],
        },
        {
          name: "Since last block",
          title: `${poolName} Since Last Block`,
          bottom: [
            line({
              metric: pool.blocksSinceBlock,
              name: "Blocks",
              unit: Unit.count,
            }),
            line({
              metric: pool.daysSinceBlock,
              name: "Days",
              unit: Unit.days,
            }),
          ],
        },
      ],
    };
  });

  return {
    name: "Chain",
    tree: [
      // Block
      {
        name: "Block",
        tree: [
          {
            name: "Count",
            title: "Block Count",
            bottom: [
              ...fromCountPattern({
                pattern: blocks.count.blockCount,
                unit: Unit.count,
              }),
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
              ...fromBaseStatsPattern({
                pattern: blocks.interval,
                unit: Unit.secs,
                avgActive: false,
              }),
              priceLine({ ctx, unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              ...fromSumStatsPattern({
                pattern: blocks.size,
                unit: Unit.bytes,
              }),
              line({
                metric: blocks.totalSize,
                name: "Total",
                color: colors.purple,
                unit: Unit.bytes,
                defaultActive: false,
              }),
              ...fromBaseStatsPattern({
                pattern: blocks.vbytes,
                unit: Unit.vb,
              }),
              ...fromBaseStatsPattern({
                pattern: blocks.weight,
                unit: Unit.wu,
              }),
              line({
                metric: blocks.weight.sum,
                name: "Sum",
                color: colors.stat.sum,
                unit: Unit.wu,
                defaultActive: false,
              }),
              line({
                metric: blocks.weight.cumulative,
                name: "Cumulative",
                color: colors.stat.cumulative,
                unit: Unit.wu,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Fullness",
            title: "Block Fullness",
            bottom: fromBaseStatsPattern({
              pattern: blocks.fullness,
              unit: Unit.percentage,
            }),
          },
        ],
      },

      // Transaction
      {
        name: "Transaction",
        tree: [
          {
            name: "Count",
            title: "Transaction Count",
            bottom: fromFullStatsPattern({
              pattern: transactions.count.txCount,
              unit: Unit.count,
            }),
          },
          {
            name: "Speed",
            title: "Transactions Per Second",
            bottom: [
              dots({
                metric: transactions.volume.txPerSec,
                name: "Transactions",
                unit: Unit.perSec,
              }),
            ],
          },
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              ...satsBtcUsd({
                pattern: transactions.volume.sentSum,
                name: "Sent",
              }),
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
              ...fromStatsPattern({
                pattern: transactions.size.weight,
                unit: Unit.wu,
              }),
              ...fromStatsPattern({
                pattern: transactions.size.vsize,
                unit: Unit.vb,
              }),
            ],
          },
          {
            name: "Fee Rate",
            title: "Fee Rate",
            bottom: fromStatsPattern({
              pattern: transactions.fees.feeRate,
              unit: Unit.feeRate,
            }),
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromCountPattern({
                pattern: transactions.versions.v1,
                unit: Unit.count,
                title: "v1",
                sumColor: colors.orange,
                cumulativeColor: colors.red,
              }),
              ...fromCountPattern({
                pattern: transactions.versions.v2,
                unit: Unit.count,
                title: "v2",
                sumColor: colors.cyan,
                cumulativeColor: colors.blue,
              }),
              ...fromCountPattern({
                pattern: transactions.versions.v3,
                unit: Unit.count,
                title: "v3",
                sumColor: colors.lime,
                cumulativeColor: colors.green,
              }),
            ],
          },
          {
            name: "Velocity",
            title: "Transactions Velocity",
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

      // UTXO Set (merged Input, Output, UTXO)
      {
        name: "UTXO Set",
        tree: [
          {
            name: "Input Count",
            title: "Input Count",
            bottom: [
              ...fromSumStatsPattern({
                pattern: inputs.count,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Output Count",
            title: "Output Count",
            bottom: [
              ...fromSumStatsPattern({
                pattern: outputs.count.totalCount,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Inputs/sec",
            title: "Inputs Per Second",
            bottom: [
              dots({
                metric: transactions.volume.inputsPerSec,
                name: "Inputs",
                unit: Unit.perSec,
              }),
            ],
          },
          {
            name: "Outputs/sec",
            title: "Outputs Per Second",
            bottom: [
              dots({
                metric: transactions.volume.outputsPerSec,
                name: "Outputs",
                unit: Unit.perSec,
              }),
            ],
          },
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
        ],
      },

      // Scripts
      {
        name: "Scripts",
        tree: [
          {
            name: "Count",
            tree: [
              // Legacy scripts
              {
                name: "Legacy",
                tree: [
                  {
                    name: "P2PKH",
                    title: "P2PKH Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2pkh,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2PK33",
                    title: "P2PK33 Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2pk33,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2PK65",
                    title: "P2PK65 Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2pk65,
                      unit: Unit.count,
                    }),
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
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2sh,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2MS",
                    title: "P2MS Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2ms,
                      unit: Unit.count,
                    }),
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
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.segwit,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2WPKH",
                    title: "P2WPKH Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2wpkh,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2WSH",
                    title: "P2WSH Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2wsh,
                      unit: Unit.count,
                    }),
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
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2tr,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "P2A",
                    title: "P2A Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.p2a,
                      unit: Unit.count,
                    }),
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
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.opreturn,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "Empty",
                    title: "Empty Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.emptyoutput,
                      unit: Unit.count,
                    }),
                  },
                  {
                    name: "Unknown",
                    title: "Unknown Output Count",
                    bottom: fromFullStatsPattern({
                      pattern: scripts.count.unknownoutput,
                      unit: Unit.count,
                    }),
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
                    color: colors.stat.sum,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.segwitAdoption.cumulative,
                    name: "Cumulative",
                    color: colors.stat.cumulative,
                    unit: Unit.percentage,
                    defaultActive: false,
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
                    color: colors.stat.sum,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.taprootAdoption.cumulative,
                    name: "Cumulative",
                    color: colors.stat.cumulative,
                    unit: Unit.percentage,
                    defaultActive: false,
                  }),
                ],
              },
            ],
          },
          {
            name: "OP_RETURN Value",
            title: "OP_RETURN Value",
            bottom: fromCoinbasePattern({ pattern: scripts.value.opreturn }),
          },
        ],
      },

      // Supply
      {
        name: "Supply",
        tree: [
          {
            name: "Circulating",
            title: "Circulating Supply",
            bottom: fromSupplyPattern({
              pattern: supply.circulating,
              title: "Supply",
            }),
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
            title: "OP_RETURN Supply",
            bottom: fromValuePattern({ pattern: supply.burned.opreturn }),
          },
        ],
      },

      // Rewards
      {
        name: "Rewards",
        tree: [
          {
            name: "Coinbase",
            title: "Coinbase Rewards",
            bottom: [
              ...fromCoinbasePattern({ pattern: blocks.rewards.coinbase }),
              ...satsBtcUsd({
                pattern: blocks.rewards._24hCoinbaseSum,
                name: "24h sum",
                color: colors.pink,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Subsidy",
            title: "Block Subsidy",
            bottom: [
              ...fromCoinbasePattern({ pattern: blocks.rewards.subsidy }),
              line({
                metric: blocks.rewards.subsidyDominance,
                name: "Dominance",
                color: colors.purple,
                unit: Unit.percentage,
                defaultActive: false,
              }),
              line({
                metric: blocks.rewards.subsidyUsd1ySma,
                name: "1y SMA",
                color: colors.lime,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Fee",
            title: "Transaction Fees",
            bottom: [
              ...fromSumStatsPattern({
                pattern: transactions.fees.fee.bitcoin,
                unit: Unit.btc,
              }),
              ...fromSumStatsPattern({
                pattern: transactions.fees.fee.sats,
                unit: Unit.sats,
              }),
              ...fromSumStatsPattern({
                pattern: transactions.fees.fee.dollars,
                unit: Unit.usd,
              }),
              line({
                metric: blocks.rewards.feeDominance,
                name: "Dominance",
                color: colors.purple,
                unit: Unit.percentage,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Unclaimed",
            title: "Unclaimed Rewards",
            bottom: fromValuePattern({
              pattern: blocks.rewards.unclaimedRewards,
              title: "Unclaimed",
            }),
          },
        ],
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          // Overview - global metrics for all addresses
          { name: "Overview", tree: createAddressMetricsTree("all", "") },

          // Compare - cross-type comparisons (base + average, system selects appropriate one)
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
                  line({
                    metric: distribution.newAddrCount[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric: distribution.newAddrCount[t.key].average,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ]),
              },
              {
                name: "Growth Rate",
                title: "Address Growth Rate by Type",
                bottom: addressTypes.flatMap((t) => [
                  line({
                    metric: distribution.growthRate[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric: distribution.growthRate[t.key].average,
                    name: t.name,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: t.defaultActive,
                  }),
                ]),
              },
              {
                name: "Activity",
                tree: activityTypes.map((a) => ({
                  name: a.name,
                  title: a.compareTitle,
                  bottom: addressTypes.flatMap((t) => [
                    line({
                      metric: distribution.addressActivity[t.key][a.key].base,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                    line({
                      metric:
                        distribution.addressActivity[t.key][a.key].average,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                  ]),
                })),
              },
            ],
          },

          // Individual address types - each with same structure as Overview
          ...addressTypes.map((t) => ({
            name: t.name,
            tree: createAddressMetricsTree(t.key, `${t.name} `),
          })),
        ],
      },

      // Mining
      {
        name: "Mining",
        tree: [
          // Hashrate
          {
            name: "Hashrate",
            title: "Network Hashrate",
            bottom: [
              dots({
                metric: blocks.mining.hashRate,
                name: "Hashrate",
                unit: Unit.hashRate,
              }),
              line({
                metric: blocks.mining.hashRate1wSma,
                name: "1w SMA",
                color: colors.red,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: blocks.mining.hashRate1mSma,
                name: "1m SMA",
                color: colors.orange,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: blocks.mining.hashRate2mSma,
                name: "2m SMA",
                color: colors.yellow,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: blocks.mining.hashRate1ySma,
                name: "1y SMA",
                color: colors.lime,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              dotted({
                metric: blocks.difficulty.asHash,
                name: "Difficulty",
                color: colors.default,
                unit: Unit.hashRate,
              }),
            ],
          },

          // Difficulty group
          {
            name: "Difficulty",
            tree: [
              {
                name: "Level",
                title: "Network Difficulty",
                bottom: [
                  line({
                    metric: blocks.difficulty.raw,
                    name: "Difficulty",
                    unit: Unit.difficulty,
                  }),
                  line({
                    metric: blocks.difficulty.epoch,
                    name: "Epoch",
                    color: colors.teal,
                    unit: Unit.epoch,
                  }),
                ],
              },
              {
                name: "Adjustment",
                title: "Difficulty Adjustment",
                bottom: [
                  baseline({
                    metric: blocks.difficulty.adjustment,
                    name: "Difficulty Change",
                    unit: Unit.percentage,
                  }),
                  priceLine({ ctx, number: 0, unit: Unit.percentage }),
                ],
              },
              {
                name: "Countdown",
                title: "Next Adjustment",
                bottom: [
                  line({
                    metric: blocks.difficulty.blocksBeforeNextAdjustment,
                    name: "Before Next",
                    color: colors.indigo,
                    unit: Unit.blocks,
                  }),
                  line({
                    metric: blocks.difficulty.daysBeforeNextAdjustment,
                    name: "Before Next",
                    color: colors.purple,
                    unit: Unit.days,
                  }),
                ],
              },
            ],
          },

          // Economics group
          {
            name: "Economics",
            tree: [
              {
                name: "Hash Price",
                title: "Hash Price",
                bottom: [
                  line({
                    metric: blocks.mining.hashPriceThs,
                    name: "TH/s",
                    color: colors.emerald,
                    unit: Unit.usdPerThsPerDay,
                  }),
                  line({
                    metric: blocks.mining.hashPricePhs,
                    name: "PH/s",
                    color: colors.emerald,
                    unit: Unit.usdPerPhsPerDay,
                  }),
                  line({
                    metric: blocks.mining.hashPriceRebound,
                    name: "Rebound",
                    color: colors.yellow,
                    unit: Unit.percentage,
                  }),
                  dotted({
                    metric: blocks.mining.hashPriceThsMin,
                    name: "TH/s Min",
                    color: colors.red,
                    unit: Unit.usdPerThsPerDay,
                  }),
                  dotted({
                    metric: blocks.mining.hashPricePhsMin,
                    name: "PH/s Min",
                    color: colors.red,
                    unit: Unit.usdPerPhsPerDay,
                  }),
                ],
              },
              {
                name: "Hash Value",
                title: "Hash Value",
                bottom: [
                  line({
                    metric: blocks.mining.hashValueThs,
                    name: "TH/s",
                    color: colors.orange,
                    unit: Unit.satsPerThsPerDay,
                  }),
                  line({
                    metric: blocks.mining.hashValuePhs,
                    name: "PH/s",
                    color: colors.orange,
                    unit: Unit.satsPerPhsPerDay,
                  }),
                  line({
                    metric: blocks.mining.hashValueRebound,
                    name: "Rebound",
                    color: colors.yellow,
                    unit: Unit.percentage,
                  }),
                  dotted({
                    metric: blocks.mining.hashValueThsMin,
                    name: "TH/s Min",
                    color: colors.red,
                    unit: Unit.satsPerThsPerDay,
                  }),
                  dotted({
                    metric: blocks.mining.hashValuePhsMin,
                    name: "PH/s Min",
                    color: colors.red,
                    unit: Unit.satsPerPhsPerDay,
                  }),
                ],
              },
            ],
          },

          // Halving (at top level for quick access)
          {
            name: "Halving",
            title: "Halving",
            bottom: [
              line({
                metric: blocks.halving.epoch,
                name: "Epoch",
                color: colors.purple,
                unit: Unit.epoch,
              }),
              line({
                metric: blocks.halving.blocksBeforeNextHalving,
                name: "Before Next",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.halving.daysBeforeNextHalving,
                name: "Before Next",
                color: colors.blue,
                unit: Unit.days,
              }),
            ],
          },
        ],
      },

      // Pools
      {
        name: "Pools",
        tree: [
          // Compare section (major pools only)
          {
            name: "Compare",
            tree: [
              {
                name: "Dominance",
                title: "Pool Dominance (Major Pools)",
                bottom: poolEntries
                  .filter(([key]) => MAJOR_POOL_IDS.includes(key.toLowerCase()))
                  .map(([key, pool]) => {
                    const poolName =
                      brk.POOL_ID_TO_POOL_NAME[
                        /** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (
                          key.toLowerCase()
                        )
                      ] || key;
                    return line({
                      metric: pool._1mDominance,
                      name: poolName,
                      unit: Unit.percentage,
                    });
                  }),
              },
              {
                name: "Blocks Mined",
                title: "Blocks Mined - 1m (Major Pools)",
                bottom: poolEntries
                  .filter(([key]) => MAJOR_POOL_IDS.includes(key.toLowerCase()))
                  .map(([key, pool]) => {
                    const poolName =
                      brk.POOL_ID_TO_POOL_NAME[
                        /** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (
                          key.toLowerCase()
                        )
                      ] || key;
                    return line({
                      metric: pool._1mBlocksMined,
                      name: poolName,
                      unit: Unit.count,
                    });
                  }),
              },
            ],
          },
          // AntPool & friends - pools sharing block templates
          {
            name: "AntPool & Friends",
            tree: [
              {
                name: "Dominance",
                title: "AntPool & Friends Dominance",
                bottom: poolEntries
                  .filter(([key]) =>
                    ANTPOOL_AND_FRIENDS_IDS.includes(key.toLowerCase()),
                  )
                  .map(([key, pool]) => {
                    const poolName =
                      brk.POOL_ID_TO_POOL_NAME[
                        /** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (
                          key.toLowerCase()
                        )
                      ] || key;
                    return line({
                      metric: pool._1mDominance,
                      name: poolName,
                      unit: Unit.percentage,
                    });
                  }),
              },
              {
                name: "Blocks Mined",
                title: "AntPool & Friends Blocks Mined (1m)",
                bottom: poolEntries
                  .filter(([key]) =>
                    ANTPOOL_AND_FRIENDS_IDS.includes(key.toLowerCase()),
                  )
                  .map(([key, pool]) => {
                    const poolName =
                      brk.POOL_ID_TO_POOL_NAME[
                        /** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (
                          key.toLowerCase()
                        )
                      ] || key;
                    return line({
                      metric: pool._1mBlocksMined,
                      name: poolName,
                      unit: Unit.count,
                    });
                  }),
              },
            ],
          },
          // Individual pools
          ...poolsTree,
        ],
      },
    ],
  };
}
