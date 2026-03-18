/** Network section - On-chain activity and health */

import { colors } from "../utils/colors.js";
import { brk } from "../client.js";
import { Unit } from "../utils/units.js";
import { entries } from "../utils/array.js";
import { priceLine } from "./constants.js";
import {
  line,
  dots,
  baseline,
  fromSupplyPattern,
  chartsFromFullPerBlock,
  chartsFromCount,
  chartsFromCountEntries,
  chartsFromAggregatedPerBlock,
  rollingWindowsTree,

  ROLLING_WINDOWS,
  chartsFromBlockAnd6b,
  multiSeriesTree,
  simpleDeltaTree,
  percentRatio,
  percentRatioDots,
  rollingPercentRatioTree,
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
    {
      key: "opReturn",
      name: "OP_RETURN",
      color: st.opReturn,
      defaultActive: false,
    },
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

  // Address type groups (by era)
  const taprootAddresses = /** @type {const} */ ([
    { key: "p2a", name: "P2A", color: st.p2a },
    { key: "p2tr", name: "P2TR", color: st.p2tr },
  ]);
  const segwitAddresses = /** @type {const} */ ([
    { key: "p2wsh", name: "P2WSH", color: st.p2wsh },
    { key: "p2wpkh", name: "P2WPKH", color: st.p2wpkh },
  ]);
  const legacyAddresses = /** @type {const} */ ([
    { key: "p2sh", name: "P2SH", color: st.p2sh },
    { key: "p2pkh", name: "P2PKH", color: st.p2pkh },
    { key: "p2pk33", name: "P2PK33", color: st.p2pk33 },
    { key: "p2pk65", name: "P2PK65", color: st.p2pk65 },
  ]);

  // Transacting types (transaction participation)
  const transactingTypes = /** @type {const} */ ([
    {
      key: "sending",
      name: "Sending",
      title: "Unique Sending Addresses per Block",
      compareTitle: "Unique Sending Addresses per Block by Type",
    },
    {
      key: "receiving",
      name: "Receiving",
      title: "Unique Receiving Addresses per Block",
      compareTitle: "Unique Receiving Addresses per Block by Type",
    },
    {
      key: "both",
      name: "Sending & Receiving",
      title: "Unique Addresses Sending & Receiving per Block",
      compareTitle: "Unique Addresses Sending & Receiving per Block by Type",
    },
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

  /**
   * Create address series tree for a given type key
   * @param {AddressableType | "all"} key
   * @param {string} titlePrefix
   */
  const createAddressSeriesTree = (key, titlePrefix) => [
    {
      name: "Count",
      title: `${titlePrefix}Address Count`,
      bottom: [
        line({
          series: addrs.funded[key],
          name: "Funded",
          unit: Unit.count,
        }),
        line({
          series: addrs.empty[key],
          name: "Empty",
          color: colors.gray,
          unit: Unit.count,
          defaultActive: false,
        }),
        line({
          series: addrs.total[key],
          name: "Total",
          color: colors.default,
          unit: Unit.count,
          defaultActive: false,
        }),
      ],
    },
    {
      name: "Trends",
      tree: [
        rollingWindowsTree({
          windows: addrs.delta[key].absolute,
          title: `${titlePrefix}Address Count Change`,
          unit: Unit.count,
          series: baseline,
        }),
        {
          name: "New",
          tree: (() => {
            const p = addrs.new[key];
            const t = `${titlePrefix}New Address Count`;
            return [
              {
                name: "Sum",
                title: t,
                bottom: [
                  line({ series: p.base, name: "base", unit: Unit.count }),
                ],
              },
              rollingWindowsTree({
                windows: p.sum,
                title: t,
                unit: Unit.count,
              }),
              {
                name: "Cumulative",
                title: `${t} (Total)`,
                bottom: [
                  line({
                    series: p.cumulative,
                    name: "all-time",
                    unit: Unit.count,
                  }),
                ],
              },
            ];
          })(),
        },
        {
          name: "Reactivated",
          tree: [
            {
              name: "Base",
              title: `${titlePrefix}Reactivated Addresses per Block`,
              bottom: [
                dots({
                  series: addrs.activity[key].reactivated.base,
                  name: "base",
                  unit: Unit.count,
                }),
                line({
                  series: addrs.activity[key].reactivated._24h,
                  name: "24h avg",
                  color: colors.stat.avg,
                  unit: Unit.count,
                }),
              ],
            },
            rollingWindowsTree({
              windows: addrs.activity[key].reactivated,
              title: `${titlePrefix}Reactivated Addresses`,
              unit: Unit.count,
            }),
          ],
        },
        rollingPercentRatioTree({
          windows: addrs.delta[key].rate,
          title: `${titlePrefix}Address Growth Rate`,
        }),
      ],
    },
    {
      name: "Transacting",
      tree: transactingTypes.map((t) => ({
        name: t.name,
        tree: [
          {
            name: "Base",
            title: `${titlePrefix}${t.title}`,
            bottom: [
              dots({
                series: addrs.activity[key][t.key].base,
                name: "base",
                unit: Unit.count,
              }),
              line({
                series: addrs.activity[key][t.key]._24h,
                name: "24h avg",
                color: colors.stat.avg,
                unit: Unit.count,
              }),
            ],
          },
          rollingWindowsTree({
            windows: addrs.activity[key][t.key],
            title: `${titlePrefix}${t.title.replace(" per Block", "")}`,
            unit: Unit.count,
          }),
        ],
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
              series: c.getSeries(t.key),
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
            series: addrs.new[t.key].base,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
          line({
            series: addrs.new[t.key].sum._24h,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
        ]),
      },
      {
        name: "Reactivated",
        tree: [
          {
            name: "Base",
            title: `${groupName} Reactivated Addresses per Block`,
            bottom: types.map((t) =>
              line({
                series: addrs.activity[t.key].reactivated.base,
                name: t.name,
                color: t.color,
                unit: Unit.count,
              }),
            ),
          },
          ...ROLLING_WINDOWS.map((w) => ({
            name: w.name,
            title: `${groupName} Reactivated Addresses (${w.name})`,
            bottom: types.map((t) =>
              line({
                series: addrs.activity[t.key].reactivated[w.key],
                name: t.name,
                color: t.color,
                unit: Unit.count,
              }),
            ),
          })),
        ],
      },
      {
        name: "Growth Rate",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `${groupName} Address Growth Rate (${w.name})`,
          bottom: types.flatMap((t) =>
            percentRatio({
              pattern: addrs.delta[t.key].rate[w.key],
              name: t.name,
              color: t.color,
            }),
          ),
        })),
      },
      {
        name: "Transacting",
        tree: transactingTypes.map((tr) => ({
          name: tr.name,
          tree: [
            {
              name: "Base",
              title: `${groupName} ${tr.compareTitle}`,
              bottom: types.map((t) =>
                line({
                  series: addrs.activity[t.key][tr.key].base,
                  name: t.name,
                  color: t.color,
                  unit: Unit.count,
                }),
              ),
            },
            ...ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: `${groupName} ${tr.compareTitle} (${w.name})`,
              bottom: types.map((t) =>
                line({
                  series: addrs.activity[t.key][tr.key][w.key],
                  name: t.name,
                  color: t.color,
                  unit: Unit.count,
                }),
              ),
            })),
          ],
        })),
      },
    ],
  });

  // Script type groups for Output Counts
  const legacyScripts = legacyAddresses.slice(1); // p2pkh, p2pk33, p2pk65
  const scriptHashScripts = [legacyAddresses[0], nonAddressableTypes[0]]; // p2sh, p2ms
  const segwitScripts = [
    /** @type {const} */ ({
      key: "segwit",
      name: "All SegWit",
      color: colors.segwit,
    }),
    ...segwitAddresses,
  ];
  const otherScripts = nonAddressableTypes.slice(1); // opreturn, empty, unknown

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
            series: /** @type {CountPattern<number>} */ (scripts.count[t.key])
              .sum._24h,
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
            series: /** @type {CountPattern<number>} */ (scripts.count[t.key])
              .cumulative,
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
            tree: [
              {
                name: "Compare",
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
              {
                name: "Native",
                title: "Transaction Velocity (BTC)",
                bottom: [
                  line({
                    series: supply.velocity.native,
                    name: "BTC",
                    unit: Unit.ratio,
                  }),
                ],
              },
              {
                name: "Fiat",
                title: "Transaction Velocity (USD)",
                bottom: [
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
                    title: "Block Count Rolling",
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
                    title: `Block Count ${w.name}`,
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
                  { series: blocks.count.total.cumulative, title: "all-time", unit: Unit.count },
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
              rollingWindowsTree({
                windows: blocks.interval,
                title: "Block Interval",
                name: "Averages",
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
        name: "Activity Rate",
        title: "Activity Rate",
        bottom: [
          dots({
            series: transactions.volume.txPerSec,
            name: "TX/sec",
            color: colors.entity.tx,
            unit: Unit.perSec,
          }),
          dots({
            series: transactions.volume.inputsPerSec,
            name: "Inputs/sec",
            color: colors.entity.input,
            unit: Unit.perSec,
          }),
          dots({
            series: transactions.volume.outputsPerSec,
            name: "Outputs/sec",
            color: colors.entity.output,
            unit: Unit.perSec,
          }),
        ],
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          // Overview - global series for all addresses
          { name: "Overview", tree: createAddressSeriesTree("all", "") },

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
                      series: c.getSeries(t.key),
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
                    series: addrs.new[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    series: addrs.new[t.key].sum._24h,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                ]),
              },
              {
                name: "Reactivated",
                tree: [
                  {
                    name: "Base",
                    title: "Reactivated Addresses per Block by Type",
                    bottom: addressTypes.map((t) =>
                      line({
                        series: addrs.activity[t.key].reactivated.base,
                        name: t.name,
                        color: t.color,
                        unit: Unit.count,
                        defaultActive: t.defaultActive,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Reactivated Addresses by Type (${w.name})`,
                    bottom: addressTypes.map((t) =>
                      line({
                        series: addrs.activity[t.key].reactivated[w.key],
                        name: t.name,
                        color: t.color,
                        unit: Unit.count,
                        defaultActive: t.defaultActive,
                      }),
                    ),
                  })),
                ],
              },
              {
                name: "Growth Rate",
                tree: ROLLING_WINDOWS.map((w) => ({
                  name: w.name,
                  title: `Address Growth Rate by Type (${w.name})`,
                  bottom: addressTypes.flatMap((t) =>
                    percentRatio({
                      pattern: addrs.delta[t.key].rate[w.key],
                      name: t.name,
                      color: t.color,
                      defaultActive: t.defaultActive,
                    }),
                  ),
                })),
              },
              {
                name: "Transacting",
                tree: transactingTypes.map((tr) => ({
                  name: tr.name,
                  tree: [
                    {
                      name: "Base",
                      title: tr.compareTitle,
                      bottom: addressTypes.map((t) =>
                        line({
                          series: addrs.activity[t.key][tr.key].base,
                          name: t.name,
                          color: t.color,
                          unit: Unit.count,
                          defaultActive: t.defaultActive,
                        }),
                      ),
                    },
                    ...ROLLING_WINDOWS.map((w) => ({
                      name: w.name,
                      title: `${tr.compareTitle} (${w.name})`,
                      bottom: addressTypes.map((t) =>
                        line({
                          series: addrs.activity[t.key][tr.key][w.key],
                          name: t.name,
                          color: t.color,
                          unit: Unit.count,
                          defaultActive: t.defaultActive,
                        }),
                      ),
                    })),
                  ],
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
                tree: createAddressSeriesTree(t.key, `${t.name} `),
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
                tree: createAddressSeriesTree(t.key, `${t.name} `),
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
                        series: /** @type {CountPattern<number>} */ (
                          scripts.count[t.key]
                        ).sum._24h,
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
              {
                name: "Legacy",
                tree: [
                  createScriptCompare("Legacy", legacyScripts),
                  ...legacyScripts.map((t) => ({
                    name: t.name,
                    tree: chartsFromCount({
                      pattern: /** @type {CountPattern<number>} */ (
                        scripts.count[t.key]
                      ),
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
                    tree: chartsFromCount({
                      pattern: /** @type {CountPattern<number>} */ (
                        scripts.count[t.key]
                      ),
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
                    tree: chartsFromCount({
                      pattern: /** @type {CountPattern<number>} */ (
                        scripts.count[t.key]
                      ),
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
                    tree: chartsFromCount({
                      pattern: /** @type {CountPattern<number>} */ (
                        scripts.count[t.key]
                      ),
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
                    tree: chartsFromCount({
                      pattern: /** @type {CountPattern<number>} */ (
                        scripts.count[t.key]
                      ),
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
                    series: scripts.adoption.segwit.percent,
                    name: "SegWit",
                    color: colors.segwit,
                    unit: Unit.percentage,
                  }),
                  line({
                    series: scripts.adoption.segwit.ratio,
                    name: "SegWit",
                    color: colors.segwit,
                    unit: Unit.ratio,
                  }),
                  line({
                    series: scripts.adoption.taproot.percent,
                    name: "Taproot",
                    color: taprootAddresses[1].color,
                    unit: Unit.percentage,
                  }),
                  line({
                    series: scripts.adoption.taproot.ratio,
                    name: "Taproot",
                    color: taprootAddresses[1].color,
                    unit: Unit.ratio,
                  }),
                ],
              },
              {
                name: "SegWit",
                title: "SegWit Adoption",
                bottom: [
                  line({
                    series: scripts.adoption.segwit.percent,
                    name: "Adoption",
                    unit: Unit.percentage,
                  }),
                  line({
                    series: scripts.adoption.segwit.ratio,
                    name: "Adoption",
                    unit: Unit.ratio,
                  }),
                ],
              },
              {
                name: "Taproot",
                title: "Taproot Adoption",
                bottom: [
                  line({
                    series: scripts.adoption.taproot.percent,
                    name: "Adoption",
                    unit: Unit.percentage,
                  }),
                  line({
                    series: scripts.adoption.taproot.ratio,
                    name: "Adoption",
                    unit: Unit.ratio,
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
