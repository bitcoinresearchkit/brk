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
  chartsFromSumPerBlock,
  rollingWindowsTree,
  distributionWindowsTree,
  mapWindows,
  ROLLING_WINDOWS,
  chartsFromBlockAnd6b,
  percentRatio,
  percentRatioDots,
  rollingPercentRatioTree,
} from "./series.js";
import { satsBtcUsd, satsBtcUsdFrom } from "./shared.js";

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
    addresses,
    cohorts,
  } = brk.metrics;

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
      getMetric: (t) => addresses.funded[t],
    },
    {
      name: "Empty",
      title: "Empty Address Count by Type",
      /** @param {AddressableType} t */
      getMetric: (t) => addresses.empty[t],
    },
    {
      name: "Total",
      title: "Total Address Count by Type",
      /** @param {AddressableType} t */
      getMetric: (t) => addresses.total[t],
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
          metric: addresses.funded[key],
          name: "Funded",
          unit: Unit.count,
        }),
        line({
          metric: addresses.empty[key],
          name: "Empty",
          color: colors.gray,
          unit: Unit.count,
          defaultActive: false,
        }),
        line({
          metric: addresses.total[key],
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
          windows: addresses.delta[key].change,
          title: `${titlePrefix}Address Count Change`,
          unit: Unit.count,
          series: baseline,
        }),
        {
          name: "New",
          tree: (() => {
            const p = addresses.new[key];
            const t = `${titlePrefix}New Address Count`;
            return [
              {
                name: "Sum",
                title: t,
                bottom: [
                  line({ metric: p.base, name: "base", unit: Unit.count }),
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
                    metric: p.cumulative,
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
                  metric: addresses.activity[key].reactivated.height,
                  name: "base",
                  unit: Unit.count,
                }),
                line({
                  metric: addresses.activity[key].reactivated._24h,
                  name: "24h avg",
                  color: colors.stat.avg,
                  unit: Unit.count,
                }),
              ],
            },
            rollingWindowsTree({
              windows: addresses.activity[key].reactivated,
              title: `${titlePrefix}Reactivated Addresses`,
              unit: Unit.count,
            }),
          ],
        },
        rollingPercentRatioTree({
          windows: addresses.delta[key].rate,
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
                metric: addresses.activity[key][t.key].height,
                name: "base",
                unit: Unit.count,
              }),
              line({
                metric: addresses.activity[key][t.key]._24h,
                name: "24h avg",
                color: colors.stat.avg,
                unit: Unit.count,
              }),
            ],
          },
          rollingWindowsTree({
            windows: addresses.activity[key][t.key],
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
              metric: c.getMetric(t.key),
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
            metric: addresses.new[t.key].base,
            name: t.name,
            color: t.color,
            unit: Unit.count,
          }),
          line({
            metric: addresses.new[t.key].sum._24h,
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
                metric: addresses.activity[t.key].reactivated.height,
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
                metric: addresses.activity[t.key].reactivated[w.key],
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
              pattern: addresses.delta[t.key].rate[w.key],
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
                  metric: addresses.activity[t.key][tr.key].height,
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
                  metric: addresses.activity[t.key][tr.key][w.key],
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
            metric: /** @type {CountPattern<number>} */ (scripts.count[t.key])
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
            metric: /** @type {CountPattern<number>} */ (scripts.count[t.key])
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
            name: "Market Cap",
            tree: [
              {
                name: "Base",
                title: "Market Cap",
                bottom: [
                  line({
                    metric: supply.marketCap.usd,
                    name: "Market Cap",
                    unit: Unit.usd,
                  }),
                ],
              },
              rollingWindowsTree({
                windows: mapWindows(
                  supply.marketCap.delta.change,
                  (c) => c.usd,
                ),
                title: "Market Cap Change",
                unit: Unit.usd,
                series: baseline,
              }),
              rollingPercentRatioTree({
                windows: supply.marketCap.delta.rate,
                title: "Market Cap Growth Rate",
              }),
            ],
          },
          {
            name: "Hodled or Lost",
            title: "Hodled or Lost Supply",
            bottom: satsBtcUsd({
              pattern: supply.hodledOrLost,
              name: "Supply",
            }),
          },
          rollingWindowsTree({
            windows: supply.marketMinusRealizedCapGrowthRate,
            title: "Market - Realized Cap Growth Rate",
            unit: Unit.ratio,
          }),
          {
            name: "Unspendable",
            tree: [
              {
                name: "Base",
                title: "Unspendable Supply",
                bottom: satsBtcUsdFrom({
                  source: supply.burned.unspendable,
                  key: "base",
                  name: "sum",
                }),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Unspendable Supply Rolling",
                    bottom: ROLLING_WINDOWS.flatMap((w) =>
                      satsBtcUsd({
                        pattern: supply.burned.unspendable.sum[w.key],
                        name: w.name,
                        color: w.color,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Unspendable Supply ${w.name}`,
                    bottom: satsBtcUsd({
                      pattern: supply.burned.unspendable.sum[w.key],
                      name: w.name,
                      color: w.color,
                    }),
                  })),
                ],
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
            tree: [
              {
                name: "Base",
                title: "OP_RETURN Burned",
                bottom: satsBtcUsd({
                  pattern: supply.burned.opReturn.base,
                  name: "sum",
                }),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "OP_RETURN Burned Rolling",
                    bottom: ROLLING_WINDOWS.flatMap((w) =>
                      satsBtcUsd({
                        pattern: supply.burned.opReturn.sum[w.key],
                        name: w.name,
                        color: w.color,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `OP_RETURN Burned ${w.name}`,
                    bottom: satsBtcUsd({
                      pattern: supply.burned.opReturn.sum[w.key],
                      name: w.name,
                      color: w.color,
                    }),
                  })),
                ],
              },
              {
                name: "Cumulative",
                title: "OP_RETURN Burned (Total)",
                bottom: satsBtcUsd({
                  pattern: supply.burned.opReturn.cumulative,
                  name: "all-time",
                }),
              },
            ],
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
            name: "Fees",
            tree: [
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
            ],
          },
          {
            name: "Volume",
            tree: [
              {
                name: "Transferred",
                title: "Transaction Volume",
                bottom: [
                  ...satsBtcUsd({
                    pattern: transactions.volume.sentSum.base,
                    name: "Sent",
                  }),
                  ...satsBtcUsd({
                    pattern: transactions.volume.receivedSum.base,
                    name: "Received",
                    color: colors.entity.output,
                  }),
                ],
              },
              {
                name: "Sent Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Sent Volume Rolling",
                    bottom: ROLLING_WINDOWS.flatMap((w) =>
                      satsBtcUsd({
                        pattern: transactions.volume.sentSum.sum[w.key],
                        name: w.name,
                        color: w.color,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Sent Volume ${w.name}`,
                    bottom: satsBtcUsd({
                      pattern: transactions.volume.sentSum.sum[w.key],
                      name: w.name,
                      color: w.color,
                    }),
                  })),
                ],
              },
              {
                name: "Sent Cumulative",
                title: "Sent Volume (Total)",
                bottom: satsBtcUsd({
                  pattern: transactions.volume.sentSum.cumulative,
                  name: "all-time",
                }),
              },
              {
                name: "Received Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Received Volume Rolling",
                    bottom: ROLLING_WINDOWS.flatMap((w) =>
                      satsBtcUsd({
                        pattern: transactions.volume.receivedSum.sum[w.key],
                        name: w.name,
                        color: w.color,
                      }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Received Volume ${w.name}`,
                    bottom: satsBtcUsd({
                      pattern: transactions.volume.receivedSum.sum[w.key],
                      name: w.name,
                      color: w.color,
                    }),
                  })),
                ],
              },
              {
                name: "Received Cumulative",
                title: "Received Volume (Total)",
                bottom: satsBtcUsd({
                  pattern: transactions.volume.receivedSum.cumulative,
                  name: "all-time",
                }),
              },
            ],
          },
          {
            name: "Size",
            tree: [
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
            ],
          },
          {
            name: "Versions",
            tree: [
              {
                name: "Base",
                title: "Transaction Versions",
                bottom: entries(transactions.versions).map(
                  ([v, data], i, arr) =>
                    line({
                      metric: data.base,
                      name: v,
                      color: colors.at(i, arr.length),
                      unit: Unit.count,
                    }),
                ),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Transaction Versions Rolling",
                    bottom: entries(transactions.versions).flatMap(
                      ([v, data], i, arr) =>
                        ROLLING_WINDOWS.map((w) =>
                          line({
                            metric: data.sum[w.key],
                            name: `${v} ${w.name}`,
                            color: colors.at(i, arr.length),
                            unit: Unit.count,
                          }),
                        ),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Transaction Versions (${w.name})`,
                    bottom: entries(transactions.versions).map(
                      ([v, data], i, arr) =>
                        line({
                          metric: data.sum[w.key],
                          name: v,
                          color: colors.at(i, arr.length),
                          unit: Unit.count,
                        }),
                    ),
                  })),
                ],
              },
              {
                name: "Cumulative",
                title: "Transaction Versions (Total)",
                bottom: entries(transactions.versions).map(
                  ([v, data], i, arr) =>
                    line({
                      metric: data.cumulative,
                      name: v,
                      color: colors.at(i, arr.length),
                      unit: Unit.count,
                    }),
                ),
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
                name: "Base",
                title: "Block Count",
                bottom: [
                  line({
                    metric: blocks.count.total.base,
                    name: "base",
                    unit: Unit.count,
                  }),
                  line({
                    metric: blocks.count.target,
                    name: "Target",
                    color: colors.gray,
                    unit: Unit.count,
                    options: { lineStyle: 4 },
                  }),
                ],
              },
              rollingWindowsTree({
                windows: blocks.count.total.sum,
                title: "Block Count",
                unit: Unit.count,
              }),
              {
                name: "Cumulative",
                title: "Block Count (Total)",
                bottom: [
                  line({
                    metric: blocks.count.total.cumulative,
                    name: "all-time",
                    unit: Unit.count,
                  }),
                ],
              },
            ],
          },
          {
            name: "Interval",
            tree: [
              {
                name: "Base",
                title: "Block Interval",
                bottom: [
                  dots({
                    metric: blocks.interval.height,
                    name: "base",
                    unit: Unit.secs,
                  }),
                  line({
                    metric: blocks.interval._24h,
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
                unit: Unit.secs,
              }),
            ],
          },
          {
            name: "Size",
            tree: [
              {
                name: "Base",
                title: "Block Size",
                bottom: [
                  line({
                    metric: blocks.size.total,
                    name: "base",
                    unit: Unit.bytes,
                  }),
                ],
              },
              rollingWindowsTree({
                windows: blocks.size.sum,
                title: "Block Size",
                unit: Unit.bytes,
              }),
              distributionWindowsTree({
                pattern: blocks.size,
                base: blocks.size.total,
                title: "Block Size",
                unit: Unit.bytes,
              }),
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
                name: "Base",
                title: "Block Weight",
                bottom: [
                  line({
                    metric: blocks.weight.raw,
                    name: "base",
                    unit: Unit.wu,
                  }),
                ],
              },
              rollingWindowsTree({
                windows: blocks.weight.sum,
                title: "Block Weight",
                unit: Unit.wu,
              }),
              distributionWindowsTree({
                pattern: blocks.weight,
                base: blocks.weight.raw,
                title: "Block Weight",
                unit: Unit.wu,
              }),
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
                name: "Base",
                title: "Block vBytes",
                bottom: [
                  line({
                    metric: blocks.vbytes.base,
                    name: "base",
                    unit: Unit.vb,
                  }),
                ],
              },
              rollingWindowsTree({
                windows: blocks.vbytes.sum,
                title: "Block vBytes",
                unit: Unit.vb,
              }),
              distributionWindowsTree({
                pattern: blocks.vbytes,
                base: blocks.vbytes.base,
                title: "Block vBytes",
                unit: Unit.vb,
              }),
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
            bottom: percentRatioDots({
              pattern: blocks.fullness,
              name: "base",
            }),
          },
          {
            name: "Difficulty",
            tree: [
              {
                name: "Base",
                title: "Mining Difficulty",
                bottom: [
                  line({
                    metric: blocks.difficulty.value,
                    name: "Difficulty",
                    unit: Unit.count,
                  }),
                ],
              },
              {
                name: "Adjustment",
                title: "Difficulty Adjustment",
                bottom: percentRatioDots({
                  pattern: blocks.difficulty.adjustment,
                  name: "Adjustment",
                }),
              },
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
                metric: outputs.count.unspent,
                name: "Count",
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "30d Change",
            title: "UTXO Count 30d Change",
            bottom: [
              baseline({
                metric: cohorts.utxo.all.outputs.unspentCount.delta.change._1m,
                name: "30d Change",
                unit: Unit.count,
              }),
            ],
          },
          {
            name: "Flow",
            title: "UTXO Flow",
            bottom: [
              line({
                metric: outputs.count.total.sum,
                name: "Created",
                color: colors.entity.output,
                unit: Unit.count,
              }),
              line({
                metric: inputs.count.sum,
                name: "Spent",
                color: colors.entity.input,
                unit: Unit.count,
              }),
            ],
          },
        ],
      },
      {
        name: "Inputs",
        tree: chartsFromSumPerBlock({
          pattern: inputs.count,
          title: "Input Count",
          unit: Unit.count,
        }),
      },
      {
        name: "Outputs",
        tree: chartsFromSumPerBlock({
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
            metric: transactions.volume.txPerSec,
            name: "TX/sec",
            color: colors.entity.tx,
            unit: Unit.perSec,
          }),
          dots({
            metric: transactions.volume.inputsPerSec,
            name: "Inputs/sec",
            color: colors.entity.input,
            unit: Unit.perSec,
          }),
          dots({
            metric: transactions.volume.outputsPerSec,
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
                      metric: c.getMetric(t.key),
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
                    metric: addresses.new[t.key].base,
                    name: t.name,
                    color: t.color,
                    unit: Unit.count,
                    defaultActive: t.defaultActive,
                  }),
                  line({
                    metric: addresses.new[t.key].sum._24h,
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
                        metric: addresses.activity[t.key].reactivated.height,
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
                        metric: addresses.activity[t.key].reactivated[w.key],
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
                      pattern: addresses.delta[t.key].rate[w.key],
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
                          metric: addresses.activity[t.key][tr.key].height,
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
                          metric: addresses.activity[t.key][tr.key][w.key],
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
                        metric: /** @type {CountPattern<number>} */ (
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
                    metric: scripts.adoption.segwit.percent,
                    name: "SegWit",
                    color: colors.segwit,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.adoption.segwit.ratio,
                    name: "SegWit",
                    color: colors.segwit,
                    unit: Unit.ratio,
                  }),
                  line({
                    metric: scripts.adoption.taproot.percent,
                    name: "Taproot",
                    color: taprootAddresses[1].color,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.adoption.taproot.ratio,
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
                    metric: scripts.adoption.segwit.percent,
                    name: "Adoption",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.adoption.segwit.ratio,
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
                    metric: scripts.adoption.taproot.percent,
                    name: "Adoption",
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: scripts.adoption.taproot.ratio,
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
