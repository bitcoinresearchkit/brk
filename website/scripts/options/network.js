/** Network section - On-chain activity and health */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, dots } from "./series.js";
import { satsBtcUsd, satsBtcUsdFrom } from "./shared.js";
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
    fromBaseStatsPattern,
    fromStatsPattern,
    fromSupplyPattern,
    chartsFromFull,
    chartsFromSum,
    chartsFromValueFull,
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

  // Address types for mapping (newest to oldest)
  const addressTypes = /** @type {const} */ ([
    {
      key: "p2a",
      name: "P2A",
      color: colors[spendableTypeColors.p2a],
      defaultActive: false,
    },
    {
      key: "p2tr",
      name: "P2TR",
      color: colors[spendableTypeColors.p2tr],
      defaultActive: true,
    },
    {
      key: "p2wsh",
      name: "P2WSH",
      color: colors[spendableTypeColors.p2wsh],
      defaultActive: true,
    },
    {
      key: "p2wpkh",
      name: "P2WPKH",
      color: colors[spendableTypeColors.p2wpkh],
      defaultActive: true,
    },
    {
      key: "p2sh",
      name: "P2SH",
      color: colors[spendableTypeColors.p2sh],
      defaultActive: true,
    },
    {
      key: "p2pkh",
      name: "P2PKH",
      color: colors[spendableTypeColors.p2pkh],
      defaultActive: true,
    },
    {
      key: "p2pk33",
      name: "P2PK33",
      color: colors[spendableTypeColors.p2pk33],
      defaultActive: false,
    },
    {
      key: "p2pk65",
      name: "P2PK65",
      color: colors[spendableTypeColors.p2pk65],
      defaultActive: false,
    },
  ]);

  // Address type groups (newest to oldest)
  const legacyAddresses = /** @type {const} */ ([
    { key: "p2sh", name: "P2SH", color: colors[spendableTypeColors.p2sh] },
    { key: "p2pkh", name: "P2PKH", color: colors[spendableTypeColors.p2pkh] },
    {
      key: "p2pk33",
      name: "P2PK33",
      color: colors[spendableTypeColors.p2pk33],
    },
    {
      key: "p2pk65",
      name: "P2PK65",
      color: colors[spendableTypeColors.p2pk65],
    },
  ]);
  const segwitAddresses = /** @type {const} */ ([
    { key: "p2wsh", name: "P2WSH", color: colors[spendableTypeColors.p2wsh] },
    {
      key: "p2wpkh",
      name: "P2WPKH",
      color: colors[spendableTypeColors.p2wpkh],
    },
  ]);
  const taprootAddresses = /** @type {const} */ ([
    { key: "p2a", name: "P2A", color: colors[spendableTypeColors.p2a] },
    { key: "p2tr", name: "P2TR", color: colors[spendableTypeColors.p2tr] },
  ]);

  // Script types for output count comparisons (newest to oldest, then non-addressable)
  const scriptTypes = /** @type {const} */ ([
    {
      key: "p2a",
      name: "P2A",
      color: colors[spendableTypeColors.p2a],
      defaultActive: false,
    },
    {
      key: "p2tr",
      name: "P2TR",
      color: colors[spendableTypeColors.p2tr],
      defaultActive: true,
    },
    {
      key: "p2wsh",
      name: "P2WSH",
      color: colors[spendableTypeColors.p2wsh],
      defaultActive: true,
    },
    {
      key: "p2wpkh",
      name: "P2WPKH",
      color: colors[spendableTypeColors.p2wpkh],
      defaultActive: true,
    },
    {
      key: "p2sh",
      name: "P2SH",
      color: colors[spendableTypeColors.p2sh],
      defaultActive: true,
    },
    {
      key: "p2pkh",
      name: "P2PKH",
      color: colors[spendableTypeColors.p2pkh],
      defaultActive: true,
    },
    {
      key: "p2pk33",
      name: "P2PK33",
      color: colors[spendableTypeColors.p2pk33],
      defaultActive: false,
    },
    {
      key: "p2pk65",
      name: "P2PK65",
      color: colors[spendableTypeColors.p2pk65],
      defaultActive: false,
    },
    {
      key: "p2ms",
      name: "P2MS",
      color: colors[spendableTypeColors.p2ms],
      defaultActive: false,
    },
    {
      key: "opreturn",
      name: "OP_RETURN",
      color: colors[spendableTypeColors.opreturn],
      defaultActive: false,
    },
    {
      key: "emptyoutput",
      name: "Empty",
      color: colors[spendableTypeColors.empty],
      defaultActive: false,
    },
    {
      key: "unknownoutput",
      name: "Unknown",
      color: colors[spendableTypeColors.unknown],
      defaultActive: false,
    },
  ]);

  // Transacting types (transaction participation)
  const transactingTypes = /** @type {const} */ ([
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
      name: "Sending & Receiving",
      title: "Addresses Sending & Receiving",
      compareTitle: "Addresses Sending & Receiving by Type",
    },
  ]);

  // Balance change types
  const balanceTypes = /** @type {const} */ ([
    {
      key: "balanceIncreased",
      name: "Increased",
      title: "Addresses with Increased Balance",
      compareTitle: "Addresses with Increased Balance by Type",
    },
    {
      key: "balanceDecreased",
      name: "Decreased",
      title: "Addresses with Decreased Balance",
      compareTitle: "Addresses with Decreased Balance by Type",
    },
  ]);

  // Count types for comparison charts
  const countTypes = /** @type {const} */ ([
    { key: "addrCount", name: "Funded", title: "Address Count by Type" },
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
  ]);

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
          name: "Funded",
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
      tree: chartsFromFull({
        pattern: distribution.newAddrCount[key],
        title: `${titlePrefix}New Address Count`,
        unit: Unit.count,
      }),
    },
    {
      name: "Reactivated",
      title: `${titlePrefix}Reactivated Address Count`,
      bottom: fromBaseStatsPattern({
        pattern: distribution.addressActivity[key].reactivated,
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
      name: "Transacting",
      tree: transactingTypes.map((t) => ({
        name: t.name,
        title: `${titlePrefix}${t.title}`,
        bottom: fromBaseStatsPattern({
          pattern: distribution.addressActivity[key][t.key],
          unit: Unit.count,
        }),
      })),
    },
    {
      name: "Balance",
      tree: balanceTypes.map((b) => ({
        name: b.name,
        title: `${titlePrefix}${b.title}`,
        bottom: fromBaseStatsPattern({
          pattern: distribution.addressActivity[key][b.key],
          unit: Unit.count,
        }),
      })),
    },
  ];

  /**
   * Create Compare charts for an address group
   * @template {AddressableType} K
   * @param {string} groupName
   * @param {ReadonlyArray<{key: K, name: string, color: Color}>} types
   */
  const createAddressCompare = (groupName, types) => ({
    name: "Compare",
    tree: [
      {
        name: "Count",
        tree: countTypes.map((c) => ({
          name: c.name,
          title: `${groupName} ${c.title}`,
          bottom: types.map((t) =>
            line({
              metric: distribution[c.key][t.key],
              name: t.name,
              color: t.color,
              unit: Unit.count,
            }),
          ),
        })),
      },
      {
        name: "New",
        title: `${groupName} New Address Count`,
        bottom: types.flatMap((t) => [
          line({
            metric: distribution.newAddrCount[t.key].base,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
          line({
            metric: distribution.newAddrCount[t.key].sum,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
        ]),
      },
      {
        name: "Reactivated",
        title: `${groupName} Reactivated Address Count`,
        bottom: types.flatMap((t) => [
          line({
            metric: distribution.addressActivity[t.key].reactivated.base,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
          line({
            metric: distribution.addressActivity[t.key].reactivated.average,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
        ]),
      },
      {
        name: "Growth Rate",
        title: `${groupName} Address Growth Rate`,
        bottom: types.flatMap((t) => [
          dots({
            metric: distribution.growthRate[t.key].base,
            name: t.name,
            color: t.color,
            unit: Unit.ratio,
          }),
          dots({
            metric: distribution.growthRate[t.key].average,
            name: t.name,
            color: t.color,
            unit: Unit.ratio,
          }),
        ]),
      },
      {
        name: "Transacting",
        tree: transactingTypes.map((tr) => ({
          name: tr.name,
          title: `${groupName} ${tr.compareTitle}`,
          bottom: types.flatMap((t) => [
            line({
              metric: distribution.addressActivity[t.key][tr.key].base,
              name: t.name,
              color: t.color,
              unit: Unit.count,
            }),
            line({
              metric: distribution.addressActivity[t.key][tr.key].average,
              name: t.name,
              color: t.color,
              unit: Unit.count,
            }),
          ]),
        })),
      },
      {
        name: "Balance",
        tree: balanceTypes.map((b) => ({
          name: b.name,
          title: `${groupName} ${b.compareTitle}`,
          bottom: types.flatMap((t) => [
            line({
              metric: distribution.addressActivity[t.key][b.key].base,
              name: t.name,
              color: t.color,
              unit: Unit.count,
            }),
            line({
              metric: distribution.addressActivity[t.key][b.key].average,
              name: t.name,
              color: t.color,
              unit: Unit.count,
            }),
          ]),
        })),
      },
    ],
  });

  // Script type groups for Output Counts (newest to oldest)
  const legacyScripts = /** @type {const} */ ([
    { key: "p2pkh", name: "P2PKH", color: colors[spendableTypeColors.p2pkh] },
    {
      key: "p2pk33",
      name: "P2PK33",
      color: colors[spendableTypeColors.p2pk33],
    },
    {
      key: "p2pk65",
      name: "P2PK65",
      color: colors[spendableTypeColors.p2pk65],
    },
  ]);
  const scriptHashScripts = /** @type {const} */ ([
    { key: "p2sh", name: "P2SH", color: colors[spendableTypeColors.p2sh] },
    { key: "p2ms", name: "P2MS", color: colors[spendableTypeColors.p2ms] },
  ]);
  const segwitScripts = /** @type {const} */ ([
    { key: "segwit", name: "All SegWit", color: colors.cyan },
    { key: "p2wsh", name: "P2WSH", color: colors[spendableTypeColors.p2wsh] },
    {
      key: "p2wpkh",
      name: "P2WPKH",
      color: colors[spendableTypeColors.p2wpkh],
    },
  ]);
  const otherScripts = /** @type {const} */ ([
    {
      key: "opreturn",
      name: "OP_RETURN",
      color: colors[spendableTypeColors.opreturn],
    },
    {
      key: "emptyoutput",
      name: "Empty",
      color: colors[spendableTypeColors.empty],
    },
    {
      key: "unknownoutput",
      name: "Unknown",
      color: colors[spendableTypeColors.unknown],
    },
  ]);

  /**
   * Create Compare charts for a script group
   * @template {keyof typeof scripts.count} K
   * @param {string} groupName
   * @param {ReadonlyArray<{key: K, name: string, color: Color}>} types
   */
  const createScriptCompare = (groupName, types) => ({
    name: "Compare",
    tree: [
      {
        name: "Sum",
        title: `${groupName} Output Count`,
        bottom: types.map((t) =>
          line({
            metric: scripts.count[t.key].sum,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
        ),
      },
      {
        name: "Cumulative",
        title: `${groupName} Output Count (Total)`,
        bottom: types.map((t) =>
          line({
            metric: scripts.count[t.key].cumulative,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
        ),
      },
    ],
  });

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
            tree: [
              {
                name: "Sum",
                title: "Unspendable Supply",
                bottom: satsBtcUsdFrom({
                  source: supply.burned.unspendable,
                  key: "sum",
                  name: "sum",
                }),
              },
              {
                name: "Cumulative",
                title: "Unspendable Supply (Total)",
                bottom: satsBtcUsdFrom({
                  source: supply.burned.unspendable,
                  key: "cumulative",
                  name: "all-time",
                }),
              },
            ],
          },
          {
            name: "OP_RETURN",
            tree: chartsFromValueFull({
              pattern: scripts.value.opreturn,
              title: "OP_RETURN Burned",
            }),
          },
        ],
      },

      // Transactions
      {
        name: "Transactions",
        tree: [
          {
            name: "Count",
            tree: chartsFromFull({
              pattern: transactions.count.txCount,
              title: "Transaction Count",
              unit: Unit.count,
            }),
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
            name: "Volume",
            tree: [
              {
                name: "Transferred",
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
                  }),
                ],
              },
              {
                name: "Annualized",
                title: "Annualized Transaction Volume",
                bottom: satsBtcUsd({
                  pattern: transactions.volume.annualizedVolume,
                  name: "Annualized",
                }),
              },
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
            name: "Versions",
            tree: [
              {
                name: "Sum",
                title: "Transaction Versions",
                bottom: [
                  line({
                    metric: transactions.versions.v1.sum,
                    name: "v1",
                    color: colors.orange,
                    unit: Unit.count,
                  }),
                  line({
                    metric: transactions.versions.v2.sum,
                    name: "v2",
                    color: colors.cyan,
                    unit: Unit.count,
                  }),
                  line({
                    metric: transactions.versions.v3.sum,
                    name: "v3",
                    color: colors.lime,
                    unit: Unit.count,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Transaction Versions (Total)",
                bottom: [
                  line({
                    metric: transactions.versions.v1.cumulative,
                    name: "v1",
                    color: colors.orange,
                    unit: Unit.count,
                  }),
                  line({
                    metric: transactions.versions.v2.cumulative,
                    name: "v2",
                    color: colors.cyan,
                    unit: Unit.count,
                  }),
                  line({
                    metric: transactions.versions.v3.cumulative,
                    name: "v3",
                    color: colors.lime,
                    unit: Unit.count,
                  }),
                ],
              },
            ],
          },
          {
            name: "Velocity",
            title: "Transaction Velocity",
            bottom: [
              line({
                metric: supply.velocity.btc,
                name: "BTC",
                unit: Unit.ratio,
              }),
              line({
                metric: supply.velocity.usd,
                name: "USD",
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
            tree: [
              {
                name: "Sum",
                title: "Block Count",
                bottom: [
                  line({
                    metric: blocks.count.blockCount.sum,
                    name: "sum",
                    unit: Unit.count,
                  }),
                  line({
                    metric: blocks.count.blockCountTarget,
                    name: "Target",
                    color: colors.gray,
                    unit: Unit.count,
                    options: { lineStyle: 4 },
                  }),
                ],
              },
              {
                name: "Rolling",
                title: "Block Count (Rolling)",
                bottom: [
                  line({
                    metric: blocks.count._24hBlockCount,
                    name: "24h",
                    color: colors.pink,
                    unit: Unit.count,
                  }),
                  line({
                    metric: blocks.count._1wBlockCount,
                    name: "1w",
                    color: colors.red,
                    unit: Unit.count,
                  }),
                  line({
                    metric: blocks.count._1mBlockCount,
                    name: "1m",
                    color: colors.orange,
                    unit: Unit.count,
                  }),
                  line({
                    metric: blocks.count._1yBlockCount,
                    name: "1y",
                    color: colors.purple,
                    unit: Unit.count,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Block Count (Total)",
                bottom: [
                  line({
                    metric: blocks.count.blockCount.cumulative,
                    name: "all-time",
                    unit: Unit.count,
                  }),
                ],
              },
            ],
          },
          {
            name: "Interval",
            title: "Block Interval",
            bottom: [
              ...fromBaseStatsPattern({
                pattern: blocks.interval,
                unit: Unit.secs,
              }),
              priceLine({ ctx, unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            tree: [
              {
                name: "Sum",
                title: "Block Size",
                bottom: [
                  line({
                    metric: blocks.totalSize,
                    name: "base",
                    unit: Unit.bytes,
                  }),
                  line({
                    metric: blocks.size.sum,
                    name: "sum",
                    unit: Unit.bytes,
                  }),
                ],
              },
              {
                name: "Distribution",
                title: "Block Size Distribution",
                bottom: [
                  line({
                    metric: blocks.totalSize,
                    name: "base",
                    unit: Unit.bytes,
                  }),
                  ...fromStatsPattern({
                    pattern: blocks.size,
                    unit: Unit.bytes,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Block Size (Total)",
                bottom: [
                  line({
                    metric: blocks.size.cumulative,
                    name: "all-time",
                    unit: Unit.bytes,
                  }),
                ],
              },
            ],
          },
          {
            name: "Weight",
            tree: [
              {
                name: "Sum",
                title: "Block Weight",
                bottom: [
                  line({
                    metric: blocks.weight.base,
                    name: "base",
                    unit: Unit.wu,
                  }),
                  line({
                    metric: blocks.weight.sum,
                    name: "sum",
                    unit: Unit.wu,
                  }),
                ],
              },
              {
                name: "Distribution",
                title: "Block Weight Distribution",
                bottom: [
                  line({
                    metric: blocks.weight.base,
                    name: "base",
                    unit: Unit.wu,
                  }),
                  ...fromStatsPattern({
                    pattern: blocks.weight,
                    unit: Unit.wu,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Block Weight (Total)",
                bottom: [
                  line({
                    metric: blocks.weight.cumulative,
                    name: "all-time",
                    unit: Unit.wu,
                  }),
                ],
              },
            ],
          },
          {
            name: "vBytes",
            tree: [
              {
                name: "Sum",
                title: "Block vBytes",
                bottom: [
                  line({
                    metric: blocks.vbytes.base,
                    name: "base",
                    unit: Unit.vb,
                  }),
                  line({
                    metric: blocks.vbytes.sum,
                    name: "sum",
                    unit: Unit.vb,
                  }),
                ],
              },
              {
                name: "Distribution",
                title: "Block vBytes Distribution",
                bottom: [
                  line({
                    metric: blocks.vbytes.base,
                    name: "base",
                    unit: Unit.vb,
                  }),
                  ...fromStatsPattern({
                    pattern: blocks.vbytes,
                    unit: Unit.vb,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Block vBytes (Total)",
                bottom: [
                  line({
                    metric: blocks.vbytes.cumulative,
                    name: "all-time",
                    unit: Unit.vb,
                  }),
                ],
              },
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

      // UTXOs
      {
        name: "UTXOs",
        tree: [
          {
            name: "Count",
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
            name: "Flow",
            title: "UTXO Flow",
            bottom: [
              line({
                metric: outputs.count.totalCount.sum,
                name: "Created",
                color: colors.lime,
                unit: Unit.count,
              }),
              line({
                metric: inputs.count.sum,
                name: "Spent",
                color: colors.red,
                unit: Unit.count,
              }),
            ],
          },
        ],
      },
      {
        name: "Inputs",
        tree: chartsFromSum({
          pattern: inputs.count,
          title: "Input Count",
          unit: Unit.count,
        }),
      },
      {
        name: "Outputs",
        tree: chartsFromSum({
          pattern: outputs.count.totalCount,
          title: "Output Count",
          unit: Unit.count,
        }),
      },
      {
        name: "Activity Rate",
        title: "Activity Rate",
        bottom: [
          dots({
            metric: transactions.volume.txPerSec,
            name: "TX/sec",
            color: colors.red,
            unit: Unit.perSec,
          }),
          dots({
            metric: transactions.volume.inputsPerSec,
            name: "Inputs/sec",
            unit: Unit.perSec,
          }),
          dots({
            metric: transactions.volume.outputsPerSec,
            name: "Outputs/sec",
            color: colors.cyan,
            unit: Unit.perSec,
          }),
        ],
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          // Overview - global metrics for all addresses
          { name: "Overview", tree: createAddressMetricsTree("all", "") },

          // Top-level Compare - all types
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
                    metric: distribution.newAddrCount[t.key].sum,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ]),
              },
              {
                name: "Reactivated",
                title: "Reactivated Address Count by Type",
                bottom: addressTypes.flatMap((t) => [
                  line({
                    metric:
                      distribution.addressActivity[t.key].reactivated.base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric:
                      distribution.addressActivity[t.key].reactivated.average,
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
                  dots({
                    metric: distribution.growthRate[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: t.defaultActive,
                  }),
                  dots({
                    metric: distribution.growthRate[t.key].average,
                    name: t.name,
                    color: t.color,
                    unit: Unit.ratio,
                    defaultActive: t.defaultActive,
                  }),
                ]),
              },
              {
                name: "Transacting",
                tree: transactingTypes.map((tr) => ({
                  name: tr.name,
                  title: tr.compareTitle,
                  bottom: addressTypes.flatMap((t) => [
                    line({
                      metric: distribution.addressActivity[t.key][tr.key].base,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                    line({
                      metric:
                        distribution.addressActivity[t.key][tr.key].average,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                  ]),
                })),
              },
              {
                name: "Balance",
                tree: balanceTypes.map((b) => ({
                  name: b.name,
                  title: b.compareTitle,
                  bottom: addressTypes.flatMap((t) => [
                    line({
                      metric: distribution.addressActivity[t.key][b.key].base,
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                    line({
                      metric:
                        distribution.addressActivity[t.key][b.key].average,
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

          // Legacy (pre-SegWit)
          {
            name: "Legacy",
            tree: [
              createAddressCompare("Legacy", legacyAddresses),
              ...legacyAddresses.map((t) => ({
                name: t.name,
                tree: createAddressMetricsTree(t.key, `${t.name} `),
              })),
            ],
          },

          // SegWit
          {
            name: "SegWit",
            tree: [
              createAddressCompare("SegWit", segwitAddresses),
              ...segwitAddresses.map((t) => ({
                name: t.name,
                tree: createAddressMetricsTree(t.key, `${t.name} `),
              })),
            ],
          },

          // Taproot
          {
            name: "Taproot",
            tree: [
              createAddressCompare("Taproot", taprootAddresses),
              ...taprootAddresses.map((t) => ({
                name: t.name,
                tree: createAddressMetricsTree(t.key, `${t.name} `),
              })),
            ],
          },
        ],
      },

      // Scripts
      {
        name: "Scripts",
        tree: [
          {
            name: "By Type",
            tree: [
              // Compare section
              {
                name: "Compare",
                tree: [
                  {
                    name: "Sum",
                    title: "Output Count by Script Type",
                    bottom: scriptTypes.map((t) =>
                      line({
                        metric: scripts.count[t.key].sum,
                        name: t.name,
                        color: t.color,
                        unit: Unit.count,
                        defaultActive: t.defaultActive,
                      }),
                    ),
                  },
                  {
                    name: "Cumulative",
                    title: "Output Count by Script Type (Total)",
                    bottom: scriptTypes.map((t) =>
                      line({
                        metric: scripts.count[t.key].cumulative,
                        name: t.name,
                        color: t.color,
                        unit: Unit.count,
                        defaultActive: t.defaultActive,
                      }),
                    ),
                  },
                ],
              },
              {
                name: "Legacy",
                tree: [
                  createScriptCompare("Legacy", legacyScripts),
                  ...legacyScripts.map((t) => ({
                    name: t.name,
                    tree: chartsFromFull({
                      pattern: scripts.count[t.key],
                      title: `${t.name} Output Count`,
                      unit: Unit.count,
                    }),
                  })),
                ],
              },
              {
                name: "Script Hash",
                tree: [
                  createScriptCompare("Script Hash", scriptHashScripts),
                  ...scriptHashScripts.map((t) => ({
                    name: t.name,
                    tree: chartsFromFull({
                      pattern: scripts.count[t.key],
                      title: `${t.name} Output Count`,
                      unit: Unit.count,
                    }),
                  })),
                ],
              },
              {
                name: "SegWit",
                tree: [
                  createScriptCompare("SegWit", segwitScripts),
                  ...segwitScripts.map((t) => ({
                    name: t.name,
                    tree: chartsFromFull({
                      pattern: scripts.count[t.key],
                      title: `${t.name} Output Count`,
                      unit: Unit.count,
                    }),
                  })),
                ],
              },
              {
                name: "Taproot",
                tree: [
                  createScriptCompare("Taproot", taprootAddresses),
                  ...taprootAddresses.map((t) => ({
                    name: t.name,
                    tree: chartsFromFull({
                      pattern: scripts.count[t.key],
                      title: `${t.name} Output Count`,
                      unit: Unit.count,
                    }),
                  })),
                ],
              },
              {
                name: "Other",
                tree: [
                  createScriptCompare("Other", otherScripts),
                  ...otherScripts.map((t) => ({
                    name: t.name,
                    tree: chartsFromFull({
                      pattern: scripts.count[t.key],
                      title: `${t.name} Output Count`,
                      unit: Unit.count,
                    }),
                  })),
                ],
              },
            ],
          },
          {
            name: "Adoption",
            tree: [
              {
                name: "Compare",
                title: "Script Adoption",
                bottom: [
                  line({
                    metric: scripts.count.segwitAdoption.cumulative,
                    name: "SegWit",
                    color: colors.cyan,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.count.taprootAdoption.cumulative,
                    name: "Taproot",
                    color: colors.orange,
                    unit: Unit.percentage,
                  }),
                ],
              },
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
                    name: "All-Time",
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
                    name: "All-Time",
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
