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
} from "./series.js";
import {
  satsBtcUsd,
  satsBtcUsdFrom,
  satsBtcUsdFromFull,
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
  const { blocks, transactions, pools, mining } = brk.metrics;

  // Pre-compute pool entries with resolved names
  const poolData = entries(pools.vecs).map(([id, pool]) => ({
    id,
    name: brk.POOL_ID_TO_POOL_NAME[id],
    pool,
  }));

  // Filtered pool groups for comparisons
  const majorPools = poolData.filter((p) => includes(MAJOR_POOL_IDS, p.id));
  const antpoolFriends = poolData.filter((p) =>
    includes(ANTPOOL_AND_FRIENDS_IDS, p.id),
  );

  // Build individual pool trees
  const poolsTree = poolData.map(({ name, pool }) => ({
    name,
    tree: [
      {
        name: "Dominance",
        title: `Dominance: ${name}`,
        bottom: [
          dots({
            metric: pool.dominance24h,
            name: "24h",
            color: colors.time._24h,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          line({
            metric: pool.dominance1w,
            name: "1w",
            color: colors.time._1w,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          line({
            metric: pool.dominance1m,
            name: "1m",
            color: colors.time._1m,
            unit: Unit.percentage,
          }),
          line({
            metric: pool.dominance1y,
            name: "1y",
            color: colors.time._1y,
            unit: Unit.percentage,
            defaultActive: false,
          }),
          line({
            metric: pool.dominance,
            name: "All Time",
            color: colors.time.all,
            unit: Unit.percentage,
            defaultActive: false,
          }),
        ],
      },
      {
        name: "Blocks Mined",
        tree: [
          {
            name: "Sum",
            title: `Blocks Mined: ${name}`,
            bottom: [
              line({
                metric: pool.blocksMined.sum,
                name: "sum",
                unit: Unit.count,
              }),
              line({
                metric: pool.blocksMined24hSum,
                name: "24h",
                color: colors.time._24h,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: pool.blocksMined1wSum,
                name: "1w",
                color: colors.time._1w,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: pool.blocksMined1mSum,
                name: "1m",
                color: colors.time._1m,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: pool.blocksMined1ySum,
                name: "1y",
                color: colors.time._1y,
                unit: Unit.count,
                defaultActive: false,
              }),
            ],
          },
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
            bottom: revenueBtcSatsUsd({
              coinbase: pool.coinbase,
              subsidy: pool.subsidy,
              fee: pool.fee,
              key: "sum",
            }),
          },
          {
            name: "Cumulative",
            title: `Rewards: ${name} (Total)`,
            bottom: revenueBtcSatsUsd({
              coinbase: pool.coinbase,
              subsidy: pool.subsidy,
              fee: pool.fee,
              key: "cumulative",
            }),
          },
        ],
      },
      {
        name: "Since Last Block",
        title: `Since Last Block: ${name}`,
        bottom: [
          line({
            metric: pool.blocksSinceBlock,
            name: "Elapsed",
            unit: Unit.blocks,
          }),
          line({
            metric: pool.daysSinceBlock,
            name: "Elapsed",
            unit: Unit.days,
          }),
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
                metric: mining.hashrate.hashRate,
                name: "Hashrate",
                unit: Unit.hashRate,
              }),
              line({
                metric: mining.hashrate.hashRate1wSma,
                name: "1w SMA",
                color: colors.time._1w,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.hashRate1mSma,
                name: "1m SMA",
                color: colors.time._1m,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.hashRate2mSma,
                name: "2m SMA",
                color: colors.indicator.main,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                metric: mining.hashrate.hashRate1ySma,
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
                metric: mining.hashrate.hashRateAth,
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
                metric: mining.hashrate.hashRateAth,
                name: "ATH",
                color: colors.loss,
                unit: Unit.hashRate,
              }),
              dots({
                metric: mining.hashrate.hashRate,
                name: "Hashrate",
                color: colors.bitcoin,
                unit: Unit.hashRate,
              }),
            ],
          },
          {
            name: "Drawdown",
            title: "Network Hashrate Drawdown",
            bottom: [
              line({
                metric: mining.hashrate.hashRateDrawdown,
                name: "Drawdown",
                unit: Unit.percentage,
                color: colors.loss,
              }),
            ],
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
                metric: blocks.difficulty.raw,
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
                metric: blocks.difficulty.adjustment,
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
                metric: blocks.difficulty.blocksBeforeNextAdjustment,
                name: "Remaining",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.difficulty.daysBeforeNextAdjustment,
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
                  fee: transactions.fees.fee,
                  key: "sum",
                }),
              },
              {
                name: "Cumulative",
                title: "Revenue Comparison (Total)",
                bottom: revenueBtcSatsUsd({
                  coinbase: mining.rewards.coinbase,
                  subsidy: mining.rewards.subsidy,
                  fee: transactions.fees.fee,
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
                bottom: [
                  ...satsBtcUsdFromFull({
                    source: mining.rewards.coinbase,
                    key: "base",
                    name: "sum",
                  }),
                  ...satsBtcUsdFrom({
                    source: mining.rewards.coinbase,
                    key: "sum",
                    name: "sum",
                  }),
                  ...satsBtcUsd({
                    pattern: mining.rewards.coinbase24hSum,
                    name: "24h",
                    color: colors.time._24h,
                    defaultActive: false,
                  }),
                ],
              },
              {
                name: "Rolling",
                tree: [
                  {
                    name: "Compare",
                    title: "Coinbase Rolling Sum",
                    bottom: [
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase24hSum,
                        name: "24h",
                        color: colors.time._24h,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase7dSum,
                        name: "7d",
                        color: colors.time._1w,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase30dSum,
                        name: "30d",
                        color: colors.time._1m,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.coinbase1ySum,
                        name: "1y",
                        color: colors.time._1y,
                      }),
                    ],
                  },
                  {
                    name: "24h",
                    title: "Coinbase 24h Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase24hSum,
                      name: "24h",
                      color: colors.time._24h,
                    }),
                  },
                  {
                    name: "7d",
                    title: "Coinbase 7d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase7dSum,
                      name: "7d",
                      color: colors.time._1w,
                    }),
                  },
                  {
                    name: "30d",
                    title: "Coinbase 30d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase30dSum,
                      name: "30d",
                      color: colors.time._1m,
                    }),
                  },
                  {
                    name: "1y",
                    title: "Coinbase 1y Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.coinbase1ySum,
                      name: "1y",
                      color: colors.time._1y,
                    }),
                  },
                ],
              },
              {
                name: "Distribution",
                title: "Coinbase Rewards per Block Distribution",
                bottom: distributionBtcSatsUsd(mining.rewards.coinbase),
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
                  ...satsBtcUsdFromFull({
                    source: mining.rewards.subsidy,
                    key: "base",
                    name: "sum",
                  }),
                  ...satsBtcUsdFrom({
                    source: mining.rewards.subsidy,
                    key: "sum",
                    name: "sum",
                  }),
                  line({
                    metric: mining.rewards.subsidyUsd1ySma,
                    name: "1y SMA",
                    color: colors.time._1y,
                    unit: Unit.usd,
                    defaultActive: false,
                  }),
                ],
              },
              {
                name: "Distribution",
                title: "Block Subsidy Distribution",
                bottom: distributionBtcSatsUsd(mining.rewards.subsidy),
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
                  source: transactions.fees.fee,
                  key: "sum",
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
                        pattern: mining.rewards.fee24hSum,
                        name: "24h",
                        color: colors.time._24h,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fee7dSum,
                        name: "7d",
                        color: colors.time._1w,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fee30dSum,
                        name: "30d",
                        color: colors.time._1m,
                      }),
                      ...satsBtcUsd({
                        pattern: mining.rewards.fee1ySum,
                        name: "1y",
                        color: colors.time._1y,
                      }),
                    ],
                  },
                  {
                    name: "24h",
                    title: "Fee 24h Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fee24hSum,
                      name: "24h",
                      color: colors.time._24h,
                    }),
                  },
                  {
                    name: "7d",
                    title: "Fee 7d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fee7dSum,
                      name: "7d",
                      color: colors.time._1w,
                    }),
                  },
                  {
                    name: "30d",
                    title: "Fee 30d Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fee30dSum,
                      name: "30d",
                      color: colors.time._1m,
                    }),
                  },
                  {
                    name: "1y",
                    title: "Fee 1y Rolling Sum",
                    bottom: satsBtcUsd({
                      pattern: mining.rewards.fee1ySum,
                      name: "1y",
                      color: colors.time._1y,
                    }),
                  },
                ],
              },
              {
                name: "Distribution",
                title: "Transaction Fee Revenue per Block Distribution",
                bottom: distributionBtcSatsUsd(transactions.fees.fee),
              },
              {
                name: "Cumulative",
                title: "Transaction Fee Revenue (Total)",
                bottom: satsBtcUsdFrom({
                  source: transactions.fees.fee,
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
                      line({
                        metric: mining.rewards.subsidyDominance,
                        name: "All-time",
                        color: colors.time.all,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.subsidyDominance24h,
                        name: "24h",
                        color: colors.time._24h,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.subsidyDominance7d,
                        name: "7d",
                        color: colors.time._1w,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.subsidyDominance30d,
                        name: "30d",
                        color: colors.time._1m,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.subsidyDominance1y,
                        name: "1y",
                        color: colors.time._1y,
                        unit: Unit.percentage,
                      }),
                    ],
                  },
                  {
                    name: "Fees",
                    title: "Fee Dominance",
                    bottom: [
                      line({
                        metric: mining.rewards.feeDominance,
                        name: "All-time",
                        color: colors.time.all,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.feeDominance24h,
                        name: "24h",
                        color: colors.time._24h,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.feeDominance7d,
                        name: "7d",
                        color: colors.time._1w,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.feeDominance30d,
                        name: "30d",
                        color: colors.time._1m,
                        unit: Unit.percentage,
                      }),
                      line({
                        metric: mining.rewards.feeDominance1y,
                        name: "1y",
                        color: colors.time._1y,
                        unit: Unit.percentage,
                      }),
                    ],
                  },
                ],
              },
              {
                name: "All-time",
                title: "Revenue Dominance (All-time)",
                bottom: [
                  line({
                    metric: mining.rewards.subsidyDominance,
                    name: "Subsidy",
                    color: colors.mining.subsidy,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: mining.rewards.feeDominance,
                    name: "Fees",
                    color: colors.mining.fee,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "24h",
                title: "Revenue Dominance (24h)",
                bottom: [
                  line({
                    metric: mining.rewards.subsidyDominance24h,
                    name: "Subsidy",
                    color: colors.mining.subsidy,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: mining.rewards.feeDominance24h,
                    name: "Fees",
                    color: colors.mining.fee,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "7d",
                title: "Revenue Dominance (7d)",
                bottom: [
                  line({
                    metric: mining.rewards.subsidyDominance7d,
                    name: "Subsidy",
                    color: colors.mining.subsidy,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: mining.rewards.feeDominance7d,
                    name: "Fees",
                    color: colors.mining.fee,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "30d",
                title: "Revenue Dominance (30d)",
                bottom: [
                  line({
                    metric: mining.rewards.subsidyDominance30d,
                    name: "Subsidy",
                    color: colors.mining.subsidy,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: mining.rewards.feeDominance30d,
                    name: "Fees",
                    color: colors.mining.fee,
                    unit: Unit.percentage,
                  }),
                ],
              },
              {
                name: "1y",
                title: "Revenue Dominance (1y)",
                bottom: [
                  line({
                    metric: mining.rewards.subsidyDominance1y,
                    name: "Subsidy",
                    color: colors.mining.subsidy,
                    unit: Unit.percentage,
                  }),
                  line({
                    metric: mining.rewards.feeDominance1y,
                    name: "Fees",
                    color: colors.mining.fee,
                    unit: Unit.percentage,
                  }),
                ],
              },
            ],
          },
          {
            name: "Unclaimed",
            tree: [
              {
                name: "Sum",
                title: "Unclaimed Rewards",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.unclaimedRewards,
                  key: "sum",
                  name: "sum",
                }),
              },
              {
                name: "Cumulative",
                title: "Unclaimed Rewards (Total)",
                bottom: satsBtcUsdFrom({
                  source: mining.rewards.unclaimedRewards,
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
                metric: mining.hashrate.hashPriceThs,
                name: "TH/s",
                color: colors.usd,
                unit: Unit.usdPerThsPerDay,
              }),
              line({
                metric: mining.hashrate.hashPricePhs,
                name: "PH/s",
                color: colors.usd,
                unit: Unit.usdPerPhsPerDay,
              }),
              dotted({
                metric: mining.hashrate.hashPriceThsMin,
                name: "TH/s Min",
                color: colors.stat.min,
                unit: Unit.usdPerThsPerDay,
              }),
              dotted({
                metric: mining.hashrate.hashPricePhsMin,
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
                metric: mining.hashrate.hashValueThs,
                name: "TH/s",
                color: colors.bitcoin,
                unit: Unit.satsPerThsPerDay,
              }),
              line({
                metric: mining.hashrate.hashValuePhs,
                name: "PH/s",
                color: colors.bitcoin,
                unit: Unit.satsPerPhsPerDay,
              }),
              dotted({
                metric: mining.hashrate.hashValueThsMin,
                name: "TH/s Min",
                color: colors.stat.min,
                unit: Unit.satsPerThsPerDay,
              }),
              dotted({
                metric: mining.hashrate.hashValuePhsMin,
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
              line({
                metric: mining.hashrate.hashPriceRebound,
                name: "Hash Price",
                color: colors.usd,
                unit: Unit.percentage,
              }),
              line({
                metric: mining.hashrate.hashValueRebound,
                name: "Hash Value",
                color: colors.bitcoin,
                unit: Unit.percentage,
              }),
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
                metric: blocks.halving.blocksBeforeNextHalving,
                name: "Remaining",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.halving.daysBeforeNextHalving,
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
                bottom: majorPools.map((p, i) =>
                  line({
                    metric: p.pool.dominance1m,
                    name: p.name,
                    color: colors.at(i, majorPools.length),
                    unit: Unit.percentage,
                  }),
                ),
              },
              {
                name: "Blocks Mined",
                title: "Blocks Mined: Major Pools (1m)",
                bottom: majorPools.map((p, i) =>
                  line({
                    metric: p.pool.blocksMined1mSum,
                    name: p.name,
                    color: colors.at(i, majorPools.length),
                    unit: Unit.count,
                  }),
                ),
              },
              {
                name: "Total Rewards",
                title: "Total Rewards: Major Pools",
                bottom: majorPools.flatMap((p, i) =>
                  satsBtcUsdFrom({
                    source: p.pool.coinbase,
                    key: "sum",
                    name: p.name,
                    color: colors.at(i, majorPools.length),
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
                bottom: antpoolFriends.map((p, i) =>
                  line({
                    metric: p.pool.dominance1m,
                    name: p.name,
                    color: colors.at(i, antpoolFriends.length),
                    unit: Unit.percentage,
                  }),
                ),
              },
              {
                name: "Blocks Mined",
                title: "Blocks Mined: AntPool & Friends (1m)",
                bottom: antpoolFriends.map((p, i) =>
                  line({
                    metric: p.pool.blocksMined1mSum,
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
                    source: p.pool.coinbase,
                    key: "sum",
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
            tree: poolsTree,
          },
        ],
      },
    ],
  };
}
