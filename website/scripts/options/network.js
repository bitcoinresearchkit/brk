/** Network section - On-chain activity and health */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { Unit } from "../utils/units.js";
import { entries } from "../utils/array.js";
import { priceLine } from "./constants.js";
import {
  line,
  dots,
  fromSupplyPattern,
  chartsFromFullPerBlock,
  chartsFromCount,
  chartsFromCountEntries,
  chartsFromAggregatedPerBlock,
  averagesTree,
  simpleDeltaTree,
  ROLLING_WINDOWS,
  chartsFromBlockAnd6b,
  multiSeriesTree,
  percentRatioDots,
} from "./series.js";
import { satsBtcUsd, satsBtcUsdFrom, satsBtcUsdFullTree } from "./shared.js";

/**
 * Create Network section
 * @returns {PartialOptionsGroup}
 */
export function createNetworkSection() {
  const {
    blocks,
    transactions,
    inputs,
    outputs,
    scripts,
    supply,
    addrs,
    cohorts,
  } = brk.series;

  const st = colors.scriptType;

  // Addressable types - newest to oldest (for addresses/counts that only support addressable types)
  const addressTypes = /** @type {const} */ ([
    { key: "p2a", name: "P2A", color: st.p2a, defaultActive: false },
    { key: "p2tr", name: "P2TR", color: st.p2tr, defaultActive: true },
    { key: "p2wsh", name: "P2WSH", color: st.p2wsh, defaultActive: true },
    { key: "p2wpkh", name: "P2WPKH", color: st.p2wpkh, defaultActive: true },
    { key: "p2sh", name: "P2SH", color: st.p2sh, defaultActive: true },
    { key: "p2pkh", name: "P2PKH", color: st.p2pkh, defaultActive: true },
    { key: "p2pk33", name: "P2PK33", color: st.p2pk33, defaultActive: false },
    { key: "p2pk65", name: "P2PK65", color: st.p2pk65, defaultActive: false },
  ]);

  // Non-addressable script types
  const nonAddressableTypes = /** @type {const} */ ([
    { key: "p2ms", name: "P2MS", color: st.p2ms, defaultActive: false },
    { key: "opReturn", name: "OP_RETURN", color: st.opReturn, defaultActive: true },
    {
      key: "emptyOutput",
      name: "Empty",
      color: st.empty,
      defaultActive: false,
    },
    {
      key: "unknownOutput",
      name: "Unknown",
      color: st.unknown,
      defaultActive: false,
    },
  ]);

  // All script types = addressable + non-addressable
  const scriptTypes = [...addressTypes, ...nonAddressableTypes];


  // Transacting types (transaction participation)
  const activityTypes = /** @type {const} */ ([
    { key: "sending", name: "Sending" },
    { key: "receiving", name: "Receiving" },
    { key: "both", name: "Both" },
    { key: "reactivated", name: "Reactivated" },
  ]);

  const countTypes = /** @type {const} */ ([
    {
      name: "Funded",
      title: "Address Count by Type",
      /** @param {AddressableType} t */
      getSeries: (t) => addrs.funded[t],
    },
    {
      name: "Empty",
      title: "Empty Address Count by Type",
      /** @param {AddressableType} t */
      getSeries: (t) => addrs.empty[t],
    },
    {
      name: "Total",
      title: "Total Address Count by Type",
      /** @param {AddressableType} t */
      getSeries: (t) => addrs.total[t],
    },
  ]);

  const countMetrics = /** @type {const} */ ([
    { key: "funded", name: "Funded", color: undefined },
    { key: "empty", name: "Empty", color: colors.gray },
    { key: "total", name: "Total", color: colors.default },
  ]);

  /**
   * @param {AddressableType | "all"} key
   * @param {string} titlePrefix
   */
  const createAddressSeriesTree = (key, titlePrefix) => [
    {
      name: "Count",
      tree: [
        {
          name: "Compare",
          title: `${titlePrefix}Address Count`,
          bottom: countMetrics.map((m) =>
            line({
              series: addrs[m.key][key],
              name: m.name,
              color: m.color,
              unit: Unit.count,
            }),
          ),
        },
        ...countMetrics.map((m) => ({
          name: m.name,
          title: `${titlePrefix}${m.name} Addresses`,
          bottom: [
            line({ series: addrs[m.key][key], name: m.name, unit: Unit.count }),
          ],
        })),
      ],
    },
    ...simpleDeltaTree({
      delta: addrs.delta[key],
      title: `${titlePrefix}Address Count`,
      unit: Unit.count,
    }),
    {
      name: "New",
      tree: chartsFromCount({
        pattern: addrs.new[key],
        title: `${titlePrefix}New Addresses`,
        unit: Unit.count,
      }),
    },
    {
      name: "Activity",
      tree: [
        {
          name: "Compare",
          tree: ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: `${titlePrefix}Active Addresses ${w.title} Average`,
            bottom: activityTypes.map((t, i) =>
              line({
                series: addrs.activity[key][t.key][w.key],
                name: t.name,
                color: colors.at(i, activityTypes.length),
                unit: Unit.count,
              }),
            ),
          })),
        },
        ...activityTypes.map((t) =>
          averagesTree({
            windows: addrs.activity[key][t.key],
            title: `${titlePrefix}${t.name} Addresses`,
            unit: Unit.count,
            name: t.name,
          }),
        ),
      ],
    },
  ];

  /** @type {Record<string, typeof scriptTypes[number]>} */
  const byKey = Object.fromEntries(scriptTypes.map((t) => [t.key, t]));

  const scriptGroups = [
    { name: "Legacy", types: [byKey.p2pkh, byKey.p2pk33, byKey.p2pk65] },
    { name: "Script Hash", types: [byKey.p2sh, byKey.p2ms] },
    { name: "SegWit", types: [byKey.p2wsh, byKey.p2wpkh] },
    { name: "Taproot", types: [byKey.p2a, byKey.p2tr] },
    { name: "Other", types: [byKey.opReturn, byKey.emptyOutput, byKey.unknownOutput] },
  ];

  /**
   * @template {keyof typeof scripts.count} K
   * @param {string} groupName
   * @param {ReadonlyArray<{key: K, name: string, color: Color}>} types
   */
  const createScriptGroup = (groupName, types) => ({
    name: groupName,
    tree: [
      {
        name: "Compare",
        tree: [
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: `${groupName} Output Count ${w.title} Sum`,
            bottom: types.map((t) =>
              line({
                series: /** @type {CountPattern<number>} */ (scripts.count[t.key]).sum[w.key],
                name: t.name,
                color: t.color,
                unit: Unit.count,
              }),
            ),
          })),
          {
            name: "Cumulative",
            title: `${groupName} Output Count (Total)`,
            bottom: types.map((t) =>
              line({
                series: scripts.count[t.key].cumulative,
                name: t.name,
                color: t.color,
                unit: Unit.count,
              }),
            ),
          },
        ],
      },
      ...types.map((t) => ({
        name: t.name,
        tree: chartsFromCount({
          pattern: /** @type {CountPattern<number>} */ (scripts.count[t.key]),
          title: `${t.name} Output Count`,
          unit: Unit.count,
        }),
      })),
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
            bottom: percentRatioDots({
              pattern: supply.inflationRate,
              name: "Rate",
            }),
          },
          {
            name: "Hodled or Lost",
            title: "Hodled or Lost Supply",
            bottom: satsBtcUsd({
              pattern: supply.hodledOrLost,
              name: "Supply",
            }),
          },

          {
            name: "Unspendable",
            title: "Unspendable Supply",
            bottom: satsBtcUsdFrom({
              source: supply.burned,
              key: "cumulative",
              name: "all-time",
            }),
          },
          {
            name: "OP_RETURN",
            title: "OP_RETURN Burned",
            bottom: satsBtcUsd({
              pattern: scripts.value.opReturn.cumulative,
              name: "all-time",
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
            tree: chartsFromFullPerBlock({
              pattern: transactions.count.total,
              title: "Transaction Count",
              unit: Unit.count,
            }),
          },
          {
            name: "Volume",
            tree: satsBtcUsdFullTree({
              pattern: transactions.volume.transferVolume,
              name: "base",
              title: "Transaction Volume",
            }),
          },
          {
            name: "Fee Rate",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.fees.feeRate,
              title: "Transaction Fee Rate",
              unit: Unit.feeRate,
            }),
          },
          {
            name: "Fee",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.fees.fee,
              title: "Transaction Fee",
              unit: Unit.sats,
            }),
          },
          {
            name: "Weight",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.size.weight,
              title: "Transaction Weight",
              unit: Unit.wu,
            }),
          },
          {
            name: "vSize",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.size.vsize,
              title: "Transaction vSize",
              unit: Unit.vb,
            }),
          },
          {
            name: "Versions",
            tree: chartsFromCountEntries({
              entries: entries(transactions.versions),
              title: "Transaction Versions",
              unit: Unit.count,
            }),
          },
          {
            name: "Velocity",
            title: "Transaction Velocity",
            bottom: [
              line({
                series: supply.velocity.native,
                name: "BTC",
                unit: Unit.ratio,
              }),
              line({
                series: supply.velocity.fiat,
                name: "USD",
                color: colors.usd,
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
                name: "Sums",
                tree: [
                  {
                    name: "Compare",
                    title: "Block Count",
                    bottom: ROLLING_WINDOWS.map((w) =>
                      line({
                        series: blocks.count.total.sum[w.key],
                        name: w.name,
                        color: w.color,
                        unit: Unit.count,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Block Count ${w.title} Sum`,
                    bottom: [
                      line({
                        series: blocks.count.total.sum[w.key],
                        name: "Actual",
                        unit: Unit.count,
                      }),
                      line({
                        series: blocks.count.target[w.key],
                        name: "Target",
                        color: colors.gray,
                        unit: Unit.count,
                        options: { lineStyle: 4 },
                      }),
                    ],
                  })),
                ],
              },
              {
                name: "Cumulative",
                title: "Block Count (Total)",
                bottom: [
                  {
                    series: blocks.count.total.cumulative,
                    title: "all-time",
                    unit: Unit.count,
                  },
                ],
              },
            ],
          },
          {
            name: "Interval",
            tree: [
              {
                name: "Per Block",
                title: "Block Interval",
                bottom: [
                  dots({
                    series: blocks.interval.base,
                    name: "base",
                    unit: Unit.secs,
                  }),
                  line({
                    series: blocks.interval._24h,
                    name: "24h avg",
                    color: colors.stat.avg,
                    unit: Unit.secs,
                  }),
                  priceLine({ unit: Unit.secs, name: "Target", number: 600 }),
                ],
              },
              averagesTree({
                windows: blocks.interval,
                title: "Block Interval",
                unit: Unit.secs,
              }),
            ],
          },
          {
            name: "Size",
            tree: chartsFromFullPerBlock({
              pattern: blocks.size,
              title: "Block Size",
              unit: Unit.bytes,
            }),
          },
          {
            name: "Weight",
            tree: chartsFromFullPerBlock({
              pattern: blocks.weight,
              title: "Block Weight",
              unit: Unit.wu,
            }),
          },
          {
            name: "vBytes",
            tree: chartsFromFullPerBlock({
              pattern: blocks.vbytes,
              title: "Block vBytes",
              unit: Unit.vb,
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
                series: outputs.count.unspent,
                name: "Count",
                unit: Unit.count,
              }),
            ],
          },
          ...simpleDeltaTree({
            delta: cohorts.utxo.all.outputs.unspentCount.delta,
            title: "UTXO Count",
            unit: Unit.count,
          }),
          {
            name: "Flow",
            tree: multiSeriesTree({
              entries: [
                {
                  name: "Created",
                  color: colors.entity.output,
                  base: outputs.count.total.sum,
                  rolling: outputs.count.total.rolling.sum,
                  cumulative: outputs.count.total.cumulative,
                },
                {
                  name: "Spent",
                  color: colors.entity.input,
                  base: inputs.count.sum,
                  rolling: inputs.count.rolling.sum,
                  cumulative: inputs.count.cumulative,
                },
              ],
              title: "UTXO Flow",
              unit: Unit.count,
            }),
          },
        ],
      },
      {
        name: "Inputs",
        tree: chartsFromAggregatedPerBlock({
          pattern: inputs.count,
          title: "Input Count",
          unit: Unit.count,
        }),
      },
      {
        name: "Outputs",
        tree: chartsFromAggregatedPerBlock({
          pattern: outputs.count.total,
          title: "Output Count",
          unit: Unit.count,
        }),
      },
      {
        name: "Throughput",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `Throughput ${w.title} Average`,
          bottom: [
            line({
              series: transactions.volume.txPerSec[w.key],
              name: "TX/sec",
              color: colors.entity.tx,
              unit: Unit.perSec,
            }),
            line({
              series: transactions.volume.inputsPerSec[w.key],
              name: "Inputs/sec",
              color: colors.entity.input,
              unit: Unit.perSec,
            }),
            line({
              series: transactions.volume.outputsPerSec[w.key],
              name: "Outputs/sec",
              color: colors.entity.output,
              unit: Unit.perSec,
            }),
          ],
        })),
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          ...createAddressSeriesTree("all", ""),
          {
            name: "By Type",
            tree: [
              {
                name: "Compare",
                tree: countTypes.map((c) => ({
                  name: c.name,
                  title: c.title,
                  bottom: addressTypes.map((t) =>
                    line({
                      series: c.getSeries(t.key),
                      name: t.name,
                      color: t.color,
                      unit: Unit.count,
                      defaultActive: t.defaultActive,
                    }),
                  ),
                })),
              },
              ...addressTypes.map((t) => ({
                name: t.name,
                tree: createAddressSeriesTree(t.key, `${t.name} `),
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
            name: "Compare",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `Output Count by Script Type ${w.title} Sum`,
                bottom: scriptTypes.map((t) =>
                  line({
                    series: /** @type {CountPattern<number>} */ (scripts.count[t.key]).sum[w.key],
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ),
              })),
              {
                name: "Cumulative",
                title: "Output Count by Script Type (Total)",
                bottom: scriptTypes.map((t) =>
                  line({
                    series: scripts.count[t.key].cumulative,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ),
              },
            ],
          },
          ...scriptGroups.map((g) => createScriptGroup(g.name, g.types)),
        ],
      },
    ],
  };
}
