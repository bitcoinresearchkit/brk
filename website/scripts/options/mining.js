/** Mining section - Network security and miner economics */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, baseline, dots, dotted } from "./series.js";
import { satsBtcUsd } from "./shared.js";

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
 * Create Mining section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createMiningSection(ctx) {
  const {
    colors,
    brk,
    fromSumStatsPattern,
    fromCoinbasePattern,
    fromValuePattern,
  } = ctx;
  const { blocks, transactions, pools } = brk.metrics;

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
          title: `Dominance: ${poolName}`,
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
          name: "Blocks Mined",
          title: `Blocks Mined: ${poolName}`,
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
          title: `Rewards: ${poolName}`,
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
          name: "Since Last Block",
          title: `Since Last Block: ${poolName}`,
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
    };
  });

  return {
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
              priceLine({ ctx, number: 0, unit: Unit.percentage }),
            ],
          },
          {
            name: "Countdown",
            title: "Next Difficulty Adjustment",
            bottom: [
              line({
                metric: blocks.difficulty.blocksBeforeNextAdjustment,
                name: "Remaining",
                color: colors.indigo,
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.difficulty.daysBeforeNextAdjustment,
                name: "Remaining",
                color: colors.purple,
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
            name: "Fees",
            title: "Transaction Fee Revenue",
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

      // Economics
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
                color: colors.blue,
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
                title: "Dominance: Major Pools",
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
                title: "Blocks Mined: Major Pools (1m)",
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
                title: "Dominance: AntPool & Friends",
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
                title: "Blocks Mined: AntPool & Friends (1m)",
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
          {
            name: "Individual",
            tree: poolsTree,
          },
        ],
      },
    ],
  };
}
