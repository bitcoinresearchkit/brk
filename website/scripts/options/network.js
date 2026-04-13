/** Network section - On-chain activity and health */

import { colors } from "../utils/colors.js";
import { brk } from "../utils/client.js";
import { Unit } from "../utils/units.js";
import { entries } from "../utils/array.js";
import {
  line,
  fromSupplyPattern,
  chartsFromFull,
  chartsFromFullPerBlock,
  chartsFromCount,
  chartsFromCountEntries,
  chartsFromPercentCumulative,
  chartsFromAggregatedPerBlock,
  averagesArray,
  simpleDeltaTree,
  ROLLING_WINDOWS,
  chartsFromBlockAnd6b,
  multiSeriesTree,
  percentRatioDots,
} from "./series.js";
import {
  satsBtcUsd,
  satsBtcUsdFrom,
  satsBtcUsdFullTree,
  formatCohortTitle,
} from "./shared.js";

/**
 * Create Network section
 * @returns {PartialOptionsGroup}
 */
export function createNetworkSection() {
  const { blocks, transactions, inputs, outputs, supply, addrs, cohorts } =
    brk.series;

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
    {
      key: "opReturn",
      name: "OP_RETURN",
      color: st.opReturn,
      defaultActive: true,
    },
    {
      key: "empty",
      name: "Empty",
      color: st.empty,
      defaultActive: false,
    },
    {
      key: "unknown",
      name: "Unknown",
      color: st.unknown,
      defaultActive: false,
    },
  ]);

  // All output types = addressable + non-addressable (12 total)
  const outputTypes = [...addressTypes, ...nonAddressableTypes];
  // Spendable input types: every output type can fund an input *except* OP_RETURN
  const inputTypes = [
    ...addressTypes,
    ...nonAddressableTypes.filter((t) => t.key !== "opReturn"),
  ];

  // Transacting types (transaction participation)
  const activityTypes = /** @type {const} */ ([
    { key: "sending", name: "Sending" },
    { key: "receiving", name: "Receiving" },
    { key: "both", name: "Sending & Receiving" },
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
    {
      name: "Funded Reused",
      title: "Funded Reused Address Count by Type",
      /** @param {AddressableType} t */
      getSeries: (t) => addrs.reused.count.funded[t],
    },
    {
      name: "Total Reused",
      title: "Total Reused Address Count by Type",
      /** @param {AddressableType} t */
      getSeries: (t) => addrs.reused.count.total[t],
    },
  ]);

  const countMetrics = /** @type {const} */ ([
    { key: "funded", name: "Funded", color: undefined },
    { key: "empty", name: "Empty", color: colors.gray },
    { key: "total", name: "Total", color: colors.default },
  ]);

  /**
   * @param {AddressableType | "all"} key
   * @param {string} [typeName]
   */
  const createAddressSeriesTree = (key, typeName) => {
    const title = formatCohortTitle(typeName);
    return [
      {
        name: "Count",
        tree: [
          {
            name: "Compare",
            title: title("Address Count"),
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
            title: title(`${m.name} Addresses`),
            bottom: [
              line({
                series: addrs[m.key][key],
                name: m.name,
                unit: Unit.count,
              }),
            ],
          })),
        ],
      },
      {
        name: "Reused",
        tree: [
          {
            name: "Compare",
            title: title("Reused Address Count"),
            bottom: [
              line({
                series: addrs.reused.count.funded[key],
                name: "Funded",
                unit: Unit.count,
              }),
              line({
                series: addrs.reused.count.total[key],
                name: "Total",
                color: colors.gray,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Funded",
            title: title("Funded Reused Addresses"),
            bottom: [
              line({
                series: addrs.reused.count.funded[key],
                name: "Funded Reused",
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Total",
            title: title("Total Reused Addresses"),
            bottom: [
              line({
                series: addrs.reused.count.total[key],
                name: "Total Reused",
                color: colors.gray,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Uses",
            tree: chartsFromCount({
              pattern: addrs.reused.uses.reusedAddrUseCount[key],
              title,
              metric: "Reused Address Uses",
              unit: Unit.count,
            }),
          },
          {
            name: "Share",
            tree: chartsFromPercentCumulative({
              pattern: addrs.reused.uses.reusedAddrUsePercent[key],
              title,
              metric: "Share of Outputs to Reused Addresses",
            }),
          },
        ],
      },
      {
        name: "Exposed",
        tree: [
          {
            name: "Compare",
            title: title("Quantum Exposed Address Count"),
            bottom: [
              line({
                series: addrs.exposed.count.funded[key],
                name: "Funded",
                unit: Unit.count,
              }),
              line({
                series: addrs.exposed.count.total[key],
                name: "Total",
                color: colors.gray,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Funded",
            title: title("Funded Quantum Exposed Address Count"),
            bottom: [
              line({
                series: addrs.exposed.count.funded[key],
                name: "Funded Exposed",
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Total",
            title: title("Total Quantum Exposed Address Count"),
            bottom: [
              line({
                series: addrs.exposed.count.total[key],
                name: "Total Exposed",
                color: colors.gray,
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Supply",
            title: title("Supply in Quantum Exposed Addresses"),
            bottom: satsBtcUsd({
              pattern: addrs.exposed.supply[key],
              name: "Supply",
            }),
          },
        ],
      },
      ...simpleDeltaTree({
        delta: addrs.delta[key],
        title,
        metric: "Address Count",
        unit: Unit.count,
      }),
      {
        name: "New",
        tree: chartsFromCount({
          pattern: addrs.new[key],
          title,
          metric: "New Addresses",
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
              title: title(`${w.title} Active Addresses`),
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
          ...activityTypes.map((t) => ({
            name: t.name,
            tree: averagesArray({
              windows: addrs.activity[key][t.key],
              title,
              metric: `${t.name} Addresses`,
              unit: Unit.count,
            }),
          })),
        ],
      },
    ];
  };

  /**
   * Build a "By Type" subtree: Compare (count / tx count / tx %) plus a
   * per-type drill-down with the same three metrics.
   *
   * @template {string} K
   * @param {Object} args
   * @param {string} args.label - Singular noun for count/tree labels ("Output" / "Prev-Out")
   * @param {Readonly<Record<K, CountPattern<number>>>} args.count
   * @param {Readonly<Record<K, CountPattern<number>>>} args.txCount
   * @param {Readonly<Record<K, PercentRatioCumulativePattern>>} args.txPercent
   * @param {ReadonlyArray<{key: K, name: string, color: Color, defaultActive: boolean}>} args.types
   * @returns {PartialOptionsTree}
   */
  const createByTypeTree = ({ label, count, txCount, txPercent, types }) => {
    const lowerLabel = label.toLowerCase();
    return [
      {
        name: "Compare",
        tree: [
          {
            name: "Count",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `${w.title} ${label} Count by Type`,
                bottom: types.map((t) =>
                  line({
                    series: count[t.key].sum[w.key],
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ),
              })),
              {
                name: "Cumulative",
                title: `Cumulative ${label} Count by Type`,
                bottom: types.map((t) =>
                  line({
                    series: count[t.key].cumulative,
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
            name: "TX Count",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `${w.title} Transactions by ${label} Type`,
                bottom: types.map((t) =>
                  line({
                    series: txCount[t.key].sum[w.key],
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ),
              })),
              {
                name: "Cumulative",
                title: `Cumulative Transactions by ${label} Type`,
                bottom: types.map((t) =>
                  line({
                    series: txCount[t.key].cumulative,
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
            name: "TX Share",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `${w.title} Share of Transactions by ${label} Type`,
                bottom: types.map((t) =>
                  line({
                    series: txPercent[t.key][w.key].percent,
                    name: t.name,
                    color: t.color,
                    unit: Unit.percentage,
                    defaultActive: t.defaultActive,
                  }),
                ),
              })),
              {
                name: "Cumulative",
                title: `Cumulative Share of Transactions by ${label} Type`,
                bottom: types.map((t) =>
                  line({
                    series: txPercent[t.key].percent,
                    name: t.name,
                    color: t.color,
                    unit: Unit.percentage,
                    defaultActive: t.defaultActive,
                  }),
                ),
              },
            ],
          },
        ],
      },
      ...types.map((t) => ({
        name: t.name,
        tree: [
          {
            name: "Count",
            tree: chartsFromCount({
              pattern: count[t.key],
              metric: `${t.name} ${label} Count`,
              unit: Unit.count,
              color: t.color,
            }),
          },
          {
            name: "TX Count",
            tree: chartsFromCount({
              pattern: txCount[t.key],
              metric: `Transactions with ${t.name} ${lowerLabel}`,
              unit: Unit.count,
              color: t.color,
            }),
          },
          {
            name: "TX Share",
            tree: chartsFromPercentCumulative({
              pattern: txPercent[t.key],
              metric: `Share of Transactions with ${t.name} ${lowerLabel}`,
              color: t.color,
            }),
          },
        ],
      })),
    ];
  };

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
            tree: [
              {
                name: "Total",
                title: "Unspendable Supply",
                bottom: satsBtcUsdFrom({
                  source: supply.burned,
                  key: "cumulative",
                  name: "All Time",
                }),
              },
              {
                name: "OP_RETURN",
                title: "OP_RETURN Burned",
                bottom: satsBtcUsd({
                  pattern: outputs.value.opReturn.cumulative,
                  name: "All Time",
                }),
              },
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
                title: `${w.title} Block Count`,
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
              {
                name: "Cumulative",
                title: "Cumulative Block Count",
                bottom: [
                  {
                    series: blocks.count.total.cumulative,
                    title: "All Time",
                    unit: Unit.count,
                  },
                ],
              },
            ],
          },
          {
            name: "Interval",
            tree: averagesArray({
              windows: blocks.interval,
              metric: "Block Interval",
              unit: Unit.secs,
            }),
          },
          {
            name: "Size",
            tree: chartsFromFull({
              pattern: blocks.size,
              metric: "Block Size",
              unit: Unit.bytes,
            }),
          },
          {
            name: "Weight",
            tree: chartsFromFull({
              pattern: blocks.weight,
              metric: "Block Weight",
              unit: Unit.wu,
            }),
          },
          {
            name: "vBytes",
            tree: chartsFromFull({
              pattern: blocks.vbytes,
              metric: "Block vBytes",
              unit: Unit.vb,
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
              metric: "Transaction Count",
              unit: Unit.count,
            }),
          },
          {
            name: "Per Second",
            tree: averagesArray({
              windows: transactions.volume.txPerSec,
              metric: "Transactions per Second",
              unit: Unit.perSec,
            }),
          },
          {
            name: "Volume",
            tree: satsBtcUsdFullTree({
              pattern: transactions.volume.transferVolume,
              metric: "Transaction Volume",
            }),
          },
          {
            name: "Effective Fee Rate",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.fees.effectiveFeeRate,
              metric: "Effective Transaction Fee Rate",
              unit: Unit.feeRate,
            }),
          },
          {
            name: "Fee",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.fees.fee,
              metric: "Transaction Fee",
              unit: Unit.sats,
            }),
          },
          {
            name: "Weight",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.size.weight,
              metric: "Transaction Weight",
              unit: Unit.wu,
            }),
          },
          {
            name: "vSize",
            tree: chartsFromBlockAnd6b({
              pattern: transactions.size.vsize,
              metric: "Transaction vSize",
              unit: Unit.vb,
            }),
          },
          {
            name: "Versions",
            tree: chartsFromCountEntries({
              entries: entries(transactions.versions),
              metric: "Transaction Versions",
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

      // UTXOs
      {
        name: "UTXOs",
        tree: [
          {
            name: "Count",
            title: "UTXO Count",
            bottom: [
              line({
                series: outputs.unspent.count,
                name: "Count",
                unit: Unit.count,
              }),
            ],
          },
          ...simpleDeltaTree({
            delta: cohorts.utxo.all.outputs.unspentCount.delta,
            metric: "UTXO Count",
            unit: Unit.count,
          }),
          {
            name: "Flow",
            tree: multiSeriesTree({
              entries: [
                {
                  name: "Created",
                  color: colors.entity.output,
                  average: outputs.count.total.rolling.average,
                  sum: outputs.count.total.rolling.sum,
                  cumulative: outputs.count.total.cumulative,
                },
                {
                  name: "Spent",
                  color: colors.entity.input,
                  average: inputs.count.rolling.average,
                  sum: inputs.count.rolling.sum,
                  cumulative: inputs.count.cumulative,
                },
              ],
              metric: "UTXO Flow",
              unit: Unit.count,
            }),
          },
        ],
      },
      {
        name: "Outputs",
        tree: [
          {
            name: "Count",
            tree: chartsFromAggregatedPerBlock({
              pattern: outputs.count.total,
              metric: "Output Count",
              unit: Unit.count,
            }),
          },
          {
            name: "Per Second",
            tree: averagesArray({
              windows: outputs.perSec,
              metric: "Outputs per Second",
              unit: Unit.perSec,
            }),
          },
          {
            name: "By Type",
            tree: createByTypeTree({
              label: "Output",
              count: outputs.byType.outputCount,
              txCount: outputs.byType.txCount,
              txPercent: outputs.byType.txPercent,
              types: outputTypes,
            }),
          },
        ],
      },
      {
        name: "Inputs",
        tree: [
          {
            name: "Count",
            tree: chartsFromAggregatedPerBlock({
              pattern: inputs.count,
              metric: "Input Count",
              unit: Unit.count,
            }),
          },
          {
            name: "Per Second",
            tree: averagesArray({
              windows: inputs.perSec,
              metric: "Inputs per Second",
              unit: Unit.perSec,
            }),
          },
          {
            name: "By Type",
            tree: createByTypeTree({
              label: "Prev-Out",
              count: inputs.byType.inputCount,
              txCount: inputs.byType.txCount,
              txPercent: inputs.byType.txPercent,
              types: inputTypes,
            }),
          },
        ],
      },
      {
        name: "Throughput",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `${w.title} Throughput`,
          bottom: [
            line({
              series: transactions.volume.txPerSec[w.key],
              name: "TX/sec",
              color: colors.entity.tx,
              unit: Unit.perSec,
            }),
            line({
              series: inputs.perSec[w.key],
              name: "Inputs/sec",
              color: colors.entity.input,
              unit: Unit.perSec,
            }),
            line({
              series: outputs.perSec[w.key],
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
          ...createAddressSeriesTree("all"),
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
                tree: createAddressSeriesTree(t.key, t.name),
              })),
            ],
          },
        ],
      },
    ],
  };
}
