/** Mining section - Network security and miner economics */

import { Unit } from "../utils/units.js";
import { entries, includes } from "../utils/array.js";
import { colors } from "../utils/colors.js";
import {
  line,
  baseline,
  dots,
  dotted,
  distributionBtcSatsUsd,
  rollingWindowsTree,
  ROLLING_WINDOWS,
  percentRatio,
  percentRatioDots,
} from "./series.js";
import {
  satsBtcUsd,
  satsBtcUsdFrom,
  revenueBtcSatsUsd,
} from "./shared.js";
import { brk } from "../client.js";

/** Major pools to show in Compare section (by current hashrate dominance) */
const MAJOR_POOL_IDS = /** @type {const} */ ([
  "foundryusa", // ~32% - largest pool
  "antpool", // ~18% - Bitmain-owned
  "viabtc", // ~14% - independent
  "f2pool", // ~10% - one of the oldest pools
  "marapool", // MARA Holdings
  "braiinspool", // formerly Slush Pool
  "spiderpool", // growing Asian pool
  "ocean", // decentralization-focused
]);

/**
 * AntPool & friends - pools sharing AntPool's block templates
 * Based on b10c's research: https://b10c.me/blog/015-bitcoin-mining-centralization/
 * Collectively ~35-40% of network hashrate
 */
const ANTPOOL_AND_FRIENDS_IDS = /** @type {const} */ ([
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
]);

/**
 * Create Mining section
 * @returns {PartialOptionsGroup}
 */
export function createMiningSection() {
  const { blocks, pools, mining } = brk.metrics;

  // Pre-compute pool entries with resolved names
  const majorPoolData = entries(pools.major).map(([id, pool]) => ({
    id,
    name: brk.POOL_ID_TO_POOL_NAME[id],
    pool,
  }));
  const minorPoolData = entries(pools.minor).map(([id, pool]) => ({
    id,
    name: brk.POOL_ID_TO_POOL_NAME[id],
    pool,
  }));

  // Filtered pool groups for comparisons (major pools only have windowed dominance)
  const featuredPools = majorPoolData.filter((p) =>
    includes(MAJOR_POOL_IDS, p.id),
  );
  const antpoolFriends = majorPoolData.filter((p) =>
    includes(ANTPOOL_AND_FRIENDS_IDS, p.id),
  );

  // Build individual pool trees
  const majorPoolsTree = majorPoolData.map(({ name, pool }) => ({
    name,
    tree: [
      {
        name: "Dominance",
        title: `Dominance: ${name}`,
        bottom: [
          ...percentRatioDots({ pattern: pool.dominance._24h, name: "24h", color: colors.time._24h, defaultActive: false }),
          ...percentRatio({ pattern: pool.dominance._1w, name: "1w", color: colors.time._1w, defaultActive: false }),
          ...percentRatio({ pattern: pool.dominance._1m, name: "1m", color: colors.time._1m }),
          ...percentRatio({ pattern: pool.dominance._1y, name: "1y", color: colors.time._1y, defaultActive: false }),
          ...percentRatio({ pattern: pool.dominance, name: "All Time", color: colors.time.all, defaultActive: false }),
        ],
      },
      {
        name: "Blocks Mined",
        tree: [
          {
            name: "Base",
            title: `Blocks Mined: ${name}`,
            bottom: [
              line({
                metric: pool.blocksMined.base,
                name: "base",
                unit: Unit.count,
              }),
            ],
          },
          rollingWindowsTree({ windows: pool.blocksMined.sum, title: `Blocks Mined: ${name}`, unit: Unit.count }),
          {
            name: "Cumulative",
            title: `Blocks Mined: ${name} (Total)`,
            bottom: [
              line({
                metric: pool.blocksMined.cumulative,
                name: "all-time",
                unit: Unit.count,
              }),
            ],
          },
        ],
      },
      {
        name: "Rewards",
        tree: [
          {
            name: "Sum",
            title: `Rewards: ${name}`,
            bottom: satsBtcUsdFrom({
              source: pool.rewards,
              key: "base",
              name: "sum",
            }),
          },
          {
            name: "Rolling",
            tree: [
              {
                name: "Compare",
                title: `Rewards: ${name} Rolling`,
                bottom: ROLLING_WINDOWS.flatMap((w) =>
                  satsBtcUsd({ pattern: pool.rewards.sum[w.key], name: w.name, color: w.color }),
                ),
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `Rewards: ${name} (${w.name})`,
                bottom: satsBtcUsd({ pattern: pool.rewards.sum[w.key], name: w.name, color: w.color }),
              })),
            ],
          },
          {
            name: "Cumulative",
            title: `Rewards: ${name} (Total)`,
            bottom: satsBtcUsdFrom({
              source: pool.rewards,
              key: "cumulative",
              name: "all-time",
            }),
          },
        ],
      },
    ],
  }));

  const minorPoolsTree = minorPoolData.map(({ name, pool }) => ({
    name,
    tree: [
      {
        name: "Dominance",
        title: `Dominance: ${name}`,
        bottom: percentRatio({ pattern: pool.dominance, name: "All Time", color: colors.time.all }),
      },
      {
        name: "Blocks Mined",
        tree: [
          {
            name: "Base",
            title: `Blocks Mined: ${name}`,
            bottom: [
              line({
                metric: pool.blocksMined.base,
                name: "base",
                unit: Unit.count,
              }),
            ],
          },
          rollingWindowsTree({ windows: pool.blocksMined.sum, title: `Blocks Mined: ${name}`, unit: Unit.count }),
          {
            name: "Cumulative",
            title: `Blocks Mined: ${name} (Total)`,
            bottom: [
              line({
                metric: pool.blocksMined.cumulative,
                name: "all-time",
                unit: Unit.count,
              }),
            ],
          },
        ],
      },
    ],
  }));

  return {
    name: "Mining",
    tree: [
      // Hashrate
      {
        name: "Hashrate",
        tree: [
          {
            name: "Current",
            title: "Network Hashrate",
            bottom: [
              dots({
                metric: mining.hashrate.rate.base,
                name: "Hashrate",
                unit: Unit.hashRate,
              }),
              line({
                metric: mining.hashrate.rate.sma._1w,
                name: "1w SMA",
                color: colors.time._1w,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.rate.sma._1m,
                name: "1m SMA",
                color: colors.time._1m,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.rate.sma._2m,
                name: "2m SMA",
                color: colors.indicator.main,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.rate.sma._1y,
                name: "1y SMA",
                color: colors.time._1y,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              dotted({
                metric: blocks.difficulty.asHash,
                name: "Difficulty",
                color: colors.default,
                unit: Unit.hashRate,
              }),
              line({
                metric: mining.hashrate.rate.ath,
                name: "ATH",
                color: colors.loss,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "ATH",
            title: "Network Hashrate ATH",
            bottom: [
              line({
                metric: mining.hashrate.rate.ath,
                name: "ATH",
                color: colors.loss,
                unit: Unit.hashRate,
              }),
              dots({
                metric: mining.hashrate.rate.base,
                name: "Hashrate",
                color: colors.bitcoin,
                unit: Unit.hashRate,
              }),
            ],
          },
          {
            name: "Drawdown",
            title: "Network Hashrate Drawdown",
            bottom: percentRatio({
              pattern: mining.hashrate.rate.drawdown,
              name: "Drawdown",
              color: colors.loss,
            }),
          },
        ],
      },

      // Difficulty
      {
        name: "Difficulty",
        tree: [
          {
            name: "Current",
            title: "Mining Difficulty",
            bottom: [
              line({
                metric: blocks.difficulty.value,
                name: "Difficulty",
                unit: Unit.difficulty,
              }),
            ],
          },
          {
            name: "Epoch",
            title: "Difficulty Epoch",
            bottom: [
              line({
                metric: blocks.difficulty.epoch,
                name: "Epoch",
                unit: Unit.epoch,
              }),
            ],
          },
          {
            name: "Adjustment",
            title: "Difficulty Adjustment",
            bottom: [
              baseline({
                metric: blocks.difficulty.adjustment.percent,
                name: "Change",
                unit: Unit.percentage,
              }),
            ],
          },
          {
            name: "Countdown",
            title: "Next Difficulty Adjustment",
            bottom: [
              line({
                metric: blocks.difficulty.blocksBeforeNext,
                name: "Remaining",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.difficulty.daysBeforeNext,
                name: "Remaining",
                unit: Unit.days,
              }),
            ],
          },
        ],
      },

      // Revenue
      {
        name: "Revenue",
        tree: [
          {
            name: "Compare",
            tree: [
              {
                name: "Sum",
                title: "Revenue Comparison",
                bottom: revenueBtcSatsUsd({
                  coinbase: mining.rewards.coinbase,
                  subsidy: mining.rewards.subsidy,
                  fee: mining.rewards.fees,
                  key: "base",
                }),
              },
              {
                name: "Cumulative",
                title: "Revenue Comparison (Total)",
                bottom: revenueBtcSatsUsd({
                  coinbase: mining.rewards.coinbase,
                  subsidy: mining.rewards.subsidy,
                  fee: mining.rewards.fees,
                  key: "cumulative",
                }),
              },
            ],
          },
          {
            name: "Coinbase",
            tree: [
              {
                name: "Sum",
                title: "Coinbase Rewards",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.coinbase,
                  key: "base",
                  name: "sum",
                }),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Coinbase Rolling Sum",
                    bottom: [
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase.sum._24h,
                        name: "24h",
                        color: colors.time._24h,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase.sum._1w,
                        name: "7d",
                        color: colors.time._1w,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase.sum._1m,
                        name: "30d",
                        color: colors.time._1m,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase.sum._1y,
                        name: "1y",
                        color: colors.time._1y,
                      }),
                    ],
                  },
                  {
                    name: "24h",
                    title: "Coinbase 24h Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase.sum._24h,
                      name: "24h",
                      color: colors.time._24h,
                    }),
                  },
                  {
                    name: "7d",
                    title: "Coinbase 7d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase.sum._1w,
                      name: "7d",
                      color: colors.time._1w,
                    }),
                  },
                  {
                    name: "30d",
                    title: "Coinbase 30d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase.sum._1m,
                      name: "30d",
                      color: colors.time._1m,
                    }),
                  },
                  {
                    name: "1y",
                    title: "Coinbase 1y Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase.sum._1y,
                      name: "1y",
                      color: colors.time._1y,
                    }),
                  },
                ],
              },
              {
                name: "Cumulative",
                title: "Coinbase Rewards (Total)",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.coinbase,
                  key: "cumulative",
                  name: "all-time",
                }),
              },
            ],
          },
          {
            name: "Subsidy",
            tree: [
              {
                name: "Sum",
                title: "Block Subsidy",
                bottom: [
                  ...satsBtcUsdFrom({
                    source: mining.rewards.subsidy,
                    key: "base",
                    name: "sum",
                  }),
                  line({
                    metric: mining.rewards.subsidy.sma1y.usd,
                    name: "1y SMA",
                    color: colors.time._1y,
                    unit: Unit.usd,
                    defaultActive: false,
                  }),
                ],
              },
              {
                name: "Cumulative",
                title: "Block Subsidy (Total)",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.subsidy,
                  key: "cumulative",
                  name: "all-time",
                }),
              },
            ],
          },
          {
            name: "Fees",
            tree: [
              {
                name: "Sum",
                title: "Transaction Fee Revenue per Block",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.fees,
                  key: "base",
                  name: "sum",
                }),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Fee Rolling Sum",
                    bottom: [
                      ...satsBtcUsd({
                        pattern: mining.rewards.fees.sum._24h,
                        name: "24h",
                        color: colors.time._24h,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fees.sum._1w,
                        name: "7d",
                        color: colors.time._1w,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fees.sum._1m,
                        name: "30d",
                        color: colors.time._1m,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fees.sum._1y,
                        name: "1y",
                        color: colors.time._1y,
                      }),
                    ],
                  },
                  {
                    name: "24h",
                    title: "Fee 24h Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fees.sum._24h,
                      name: "24h",
                      color: colors.time._24h,
                    }),
                  },
                  {
                    name: "7d",
                    title: "Fee 7d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fees.sum._1w,
                      name: "7d",
                      color: colors.time._1w,
                    }),
                  },
                  {
                    name: "30d",
                    title: "Fee 30d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fees.sum._1m,
                      name: "30d",
                      color: colors.time._1m,
                    }),
                  },
                  {
                    name: "1y",
                    title: "Fee 1y Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fees.sum._1y,
                      name: "1y",
                      color: colors.time._1y,
                    }),
                  },
                ],
              },
              {
                name: "Distribution",
                tree: ROLLING_WINDOWS.map((w) => ({
                  name: w.name,
                  title: `Fee Revenue per Block Distribution (${w.name})`,
                  bottom: distributionBtcSatsUsd(mining.rewards.fees[w.key]),
                })),
              },
              {
                name: "Cumulative",
                title: "Transaction Fee Revenue (Total)",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.fees,
                  key: "cumulative",
                  name: "all-time",
                }),
              },
            ],
          },
          {
            name: "Dominance",
            tree: [
              {
                name: "Compare",
                tree: [
                  {
                    name: "Subsidy",
                    title: "Subsidy Dominance",
                    bottom: [
                      ...percentRatio({ pattern: mining.rewards.subsidy.dominance, name: "All-time", color: colors.time.all }),
                      ...ROLLING_WINDOWS.flatMap((w) =>
                        percentRatio({ pattern: mining.rewards.subsidy.dominance[w.key], name: w.name, color: w.color }),
                      ),
                    ],
                  },
                  {
                    name: "Fees",
                    title: "Fee Dominance",
                    bottom: [
                      ...percentRatio({ pattern: mining.rewards.fees.dominance, name: "All-time", color: colors.time.all }),
                      ...ROLLING_WINDOWS.flatMap((w) =>
                        percentRatio({ pattern: mining.rewards.fees.dominance[w.key], name: w.name, color: w.color }),
                      ),
                    ],
                  },
                ],
              },
              {
                name: "All-time",
                title: "Revenue Dominance (All-time)",
                bottom: [
                  ...percentRatio({ pattern: mining.rewards.subsidy.dominance, name: "Subsidy", color: colors.mining.subsidy }),
                  ...percentRatio({ pattern: mining.rewards.fees.dominance, name: "Fees", color: colors.mining.fee }),
                ],
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `Revenue Dominance (${w.name})`,
                bottom: [
                  ...percentRatio({ pattern: mining.rewards.subsidy.dominance[w.key], name: "Subsidy", color: colors.mining.subsidy }),
                  ...percentRatio({ pattern: mining.rewards.fees.dominance[w.key], name: "Fees", color: colors.mining.fee }),
                ],
              })),
            ],
          },
          {
            name: "Fee Multiple",
            tree: [
              {
                name: "Compare",
                title: "Fee-to-Subsidy Ratio",
                bottom: ROLLING_WINDOWS.map((w) =>
                  line({ metric: mining.rewards.fees.ratioMultiple[w.key].ratio, name: w.name, color: w.color, unit: Unit.ratio }),
                ),
              },
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `Fee-to-Subsidy Ratio (${w.name})`,
                bottom: [line({ metric: mining.rewards.fees.ratioMultiple[w.key].ratio, name: w.name, color: w.color, unit: Unit.ratio })],
              })),
            ],
          },
          {
            name: "Unclaimed",
            tree: [
              {
                name: "Sum",
                title: "Unclaimed Rewards",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.unclaimed,
                  key: "base",
                  name: "sum",
                }),
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Unclaimed Rewards Rolling",
                    bottom: ROLLING_WINDOWS.flatMap((w) =>
                      satsBtcUsd({ pattern: mining.rewards.unclaimed.sum[w.key], name: w.name, color: w.color }),
                    ),
                  },
                  ...ROLLING_WINDOWS.map((w) => ({
                    name: w.name,
                    title: `Unclaimed Rewards ${w.name}`,
                    bottom: satsBtcUsd({ pattern: mining.rewards.unclaimed.sum[w.key], name: w.name, color: w.color }),
                  })),
                ],
              },
              {
                name: "Cumulative",
                title: "Unclaimed Rewards (Total)",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.unclaimed,
                  key: "cumulative",
                  name: "all-time",
                }),
              },
            ],
          },
        ],
      },

      // Economics
      {
        name: "Economics",
        tree: [
          {
            name: "Hash Price",
            title: "Hash Price",
            bottom: [
              line({
                metric: mining.hashrate.price.ths,
                name: "TH/s",
                color: colors.usd,
                unit: Unit.usdPerThsPerDay,
              }),
              line({
                metric: mining.hashrate.price.phs,
                name: "PH/s",
                color: colors.usd,
                unit: Unit.usdPerPhsPerDay,
              }),
              dotted({
                metric: mining.hashrate.price.thsMin,
                name: "TH/s Min",
                color: colors.stat.min,
                unit: Unit.usdPerThsPerDay,
              }),
              dotted({
                metric: mining.hashrate.price.phsMin,
                name: "PH/s Min",
                color: colors.stat.min,
                unit: Unit.usdPerPhsPerDay,
              }),
            ],
          },
          {
            name: "Hash Value",
            title: "Hash Value",
            bottom: [
              line({
                metric: mining.hashrate.value.ths,
                name: "TH/s",
                color: colors.bitcoin,
                unit: Unit.satsPerThsPerDay,
              }),
              line({
                metric: mining.hashrate.value.phs,
                name: "PH/s",
                color: colors.bitcoin,
                unit: Unit.satsPerPhsPerDay,
              }),
              dotted({
                metric: mining.hashrate.value.thsMin,
                name: "TH/s Min",
                color: colors.stat.min,
                unit: Unit.satsPerThsPerDay,
              }),
              dotted({
                metric: mining.hashrate.value.phsMin,
                name: "PH/s Min",
                color: colors.stat.min,
                unit: Unit.satsPerPhsPerDay,
              }),
            ],
          },
          {
            name: "Recovery",
            title: "Recovery",
            bottom: [
              ...percentRatio({ pattern: mining.hashrate.price.rebound, name: "Hash Price", color: colors.usd }),
              ...percentRatio({ pattern: mining.hashrate.value.rebound, name: "Hash Value", color: colors.bitcoin }),
            ],
          },
        ],
      },

      // Halving
      {
        name: "Halving",
        tree: [
          {
            name: "Countdown",
            title: "Next Halving",
            bottom: [
              line({
                metric: blocks.halving.blocksBeforeNext,
                name: "Remaining",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.halving.daysBeforeNext,
                name: "Remaining",
                unit: Unit.days,
              }),
            ],
          },
          {
            name: "Epoch",
            title: "Halving Epoch",
            bottom: [
              line({
                metric: blocks.halving.epoch,
                name: "Epoch",
                unit: Unit.epoch,
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
                title: "Dominance: Major Pools (1m)",
                bottom: featuredPools.flatMap((p, i) =>
                  percentRatio({
                    pattern: p.pool.dominance._1m,
                    name: p.name,
                    color: colors.at(i, featuredPools.length),
                  }),
                ),
              },
              {
                name: "Blocks Mined",
                title: "Blocks Mined: Major Pools (1m)",
                bottom: featuredPools.map((p, i) =>
                  line({
                    metric: p.pool.blocksMined.sum._1m,
                    name: p.name,
                    color: colors.at(i, featuredPools.length),
                    unit: Unit.count,
                  }),
                ),
              },
              {
                name: "Total Rewards",
                title: "Total Rewards: Major Pools",
                bottom: featuredPools.flatMap((p, i) =>
                  satsBtcUsdFrom({
                    source: p.pool.rewards,
                    key: "base",
                    name: p.name,
                    color: colors.at(i, featuredPools.length),
                  }),
                ),
              },
            ],
          },
          // AntPool & friends - pools sharing block templates
          {
            name: "AntPool & Friends",
            tree: [
              {
                name: "Dominance",
                title: "Dominance: AntPool & Friends (1m)",
                bottom: antpoolFriends.flatMap((p, i) =>
                  percentRatio({
                    pattern: p.pool.dominance._1m,
                    name: p.name,
                    color: colors.at(i, antpoolFriends.length),
                  }),
                ),
              },
              {
                name: "Blocks Mined",
                title: "Blocks Mined: AntPool & Friends (1m)",
                bottom: antpoolFriends.map((p, i) =>
                  line({
                    metric: p.pool.blocksMined.sum._1m,
                    name: p.name,
                    color: colors.at(i, antpoolFriends.length),
                    unit: Unit.count,
                  }),
                ),
              },
              {
                name: "Total Rewards",
                title: "Total Rewards: AntPool & Friends",
                bottom: antpoolFriends.flatMap((p, i) =>
                  satsBtcUsdFrom({
                    source: p.pool.rewards,
                    key: "base",
                    name: p.name,
                    color: colors.at(i, antpoolFriends.length),
                  }),
                ),
              },
            ],
          },
          // All pools
          {
            name: "All Pools",
            tree: [...majorPoolsTree, ...minorPoolsTree],
          },
        ],
      },
    ],
  };
}
