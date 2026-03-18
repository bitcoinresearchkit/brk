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
  statsAtWindow,
  ROLLING_WINDOWS,
  percentRatio,
  chartsFromCount,
} from "./series.js";
import {
  satsBtcUsdFrom,
  satsBtcUsdFullTree,
  revenueBtcSatsUsd,
} from "./shared.js";
import { brk } from "../client.js";

/** Major pools to show in Compare section (by current hashrate dominance) */
const MAJOR_POOL_IDS = /** @type {const} */ ([
  "foundryusa",
  "antpool",
  "viabtc",
  "f2pool",
  "marapool",
  "braiinspool",
  "spiderpool",
  "ocean",
]);

/**
 * AntPool & friends - pools sharing AntPool's block templates
 * Based on b10c's research: https://b10c.me/blog/015-bitcoin-mining-centralization/
 */
const ANTPOOL_AND_FRIENDS_IDS = /** @type {const} */ ([
  "antpool",
  "poolin",
  "btccom",
  "braiinspool",
  "ultimuspool",
  "binancepool",
  "secpool",
  "sigmapoolcom",
  "rawpool",
  "luxor",
]);

/**
 * Create Mining section
 * @returns {PartialOptionsGroup}
 */
export function createMiningSection() {
  const { blocks, pools, mining } = brk.series;

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

  const featuredPools = majorPoolData.filter((p) =>
    includes(MAJOR_POOL_IDS, p.id),
  );
  const antpoolFriends = majorPoolData.filter((p) =>
    includes(ANTPOOL_AND_FRIENDS_IDS, p.id),
  );

  /** @param {string} title @param {{ _24h: any, _1w: any, _1m: any, _1y: any, percent: any, ratio: any }} dominance */
  const dominanceTree = (title, dominance) => ({
    name: "Dominance",
    tree: [
      {
        name: "Compare",
        title,
        bottom: [
          ...ROLLING_WINDOWS.flatMap((w) =>
            percentRatio({ pattern: dominance[w.key], name: w.name, color: w.color, defaultActive: w.key !== "_24h" }),
          ),
          ...percentRatio({ pattern: dominance, name: "All Time", color: colors.time.all }),
        ],
      },
      ...ROLLING_WINDOWS.map((w) => ({
        name: w.name,
        title: `${title} ${w.title}`,
        bottom: percentRatio({ pattern: dominance[w.key], name: w.name, color: w.color }),
      })),
      {
        name: "All Time",
        title: `${title} All Time`,
        bottom: percentRatio({ pattern: dominance, name: "All Time", color: colors.time.all }),
      },
    ],
  });

  /**
   * @param {typeof majorPoolData} poolList
   */
  const createPoolTree = (poolList) =>
    poolList.map(({ name, pool }) => ({
      name,
      tree: [
        dominanceTree(`Dominance: ${name}`, pool.dominance),
        {
          name: "Blocks Mined",
          tree: chartsFromCount({
            pattern: pool.blocksMined,
            title: `Blocks Mined: ${name}`,
            unit: Unit.count,
          }),
        },
        {
          name: "Rewards",
          tree: satsBtcUsdFullTree({
            pattern: pool.rewards,
            name: "Rewards",
            title: `Rewards: ${name}`,
          }),
        },
      ],
    }));

  /**
   * @param {typeof minorPoolData} poolList
   */
  const createMinorPoolTree = (poolList) =>
    poolList.map(({ name, pool }) => ({
      name,
      tree: [
        {
          name: "Dominance",
          title: `Dominance: ${name}`,
          bottom: percentRatio({ pattern: pool.dominance, name: "All Time", color: colors.time.all }),
        },
        {
          name: "Blocks Mined",
          tree: chartsFromCount({
            pattern: pool.blocksMined,
            title: `Blocks Mined: ${name}`,
            unit: Unit.count,
          }),
        },
      ],
    }));

  /**
   * @param {string} groupTitle
   * @param {typeof majorPoolData} poolList
   */
  const createPoolCompare = (groupTitle, poolList) => ({
    name: "Compare",
    tree: [
      {
        name: "Dominance",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `Dominance: ${groupTitle} ${w.title}`,
          bottom: poolList.flatMap((p, i) =>
            percentRatio({
              pattern: p.pool.dominance[w.key],
              name: p.name,
              color: colors.at(i, poolList.length),
            }),
          ),
        })),
      },
      {
        name: "Blocks Mined",
        tree: ROLLING_WINDOWS.map((w) => ({
          name: w.name,
          title: `Blocks Mined: ${groupTitle} ${w.title} Sum`,
          bottom: poolList.map((p, i) =>
            line({
              series: p.pool.blocksMined.sum[w.key],
              name: p.name,
              color: colors.at(i, poolList.length),
              unit: Unit.count,
            }),
          ),
        })),
      },
    ],
  });


  return {
    name: "Mining",
    tree: [
      {
        name: "Hashrate",
        tree: [
          {
            name: "Current",
            title: "Network Hashrate",
            bottom: [
              dots({
                series: mining.hashrate.rate.base,
                name: "Hashrate",
                unit: Unit.hashRate,
              }),
              line({
                series: mining.hashrate.rate.sma._1w,
                name: "1w SMA",
                color: colors.time._1w,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                series: mining.hashrate.rate.sma._1m,
                name: "1m SMA",
                color: colors.time._1m,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                series: mining.hashrate.rate.sma._2m,
                name: "2m SMA",
                color: colors.indicator.main,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              line({
                series: mining.hashrate.rate.sma._1y,
                name: "1y SMA",
                color: colors.time._1y,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              dotted({
                series: blocks.difficulty.hashrate,
                name: "Difficulty",
                color: colors.default,
                unit: Unit.hashRate,
              }),
              line({
                series: mining.hashrate.rate.ath,
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
                series: mining.hashrate.rate.ath,
                name: "ATH",
                color: colors.loss,
                unit: Unit.hashRate,
              }),
              dots({
                series: mining.hashrate.rate.base,
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

      {
        name: "Revenue",
        tree: [
          {
            name: "Compare",
            tree: [
              {
                name: "Per Block",
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
            tree: satsBtcUsdFullTree({
              pattern: mining.rewards.coinbase,
              name: "Coinbase",
              title: "Coinbase Rewards",
            }),
          },
          {
            name: "Subsidy",
            tree: [
              {
                name: "Per Block",
                title: "Block Subsidy",
                bottom: [
                  ...satsBtcUsdFrom({
                    source: mining.rewards.subsidy,
                    key: "base",
                    name: "base",
                  }),
                  line({
                    series: mining.rewards.subsidy.sma1y.usd,
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
              ...satsBtcUsdFullTree({
                pattern: mining.rewards.fees,
                name: "Fees",
                title: "Transaction Fee Revenue",
              }),
              {
                name: "Distributions",
                tree: ROLLING_WINDOWS.map((w) => ({
                  name: w.name,
                  title: `Fee Revenue per Block ${w.title} Distribution`,
                  bottom: distributionBtcSatsUsd(statsAtWindow(mining.rewards.fees, w.key)),
                })),
              },
            ],
          },
          {
            name: "Dominance",
            tree: [
              ...ROLLING_WINDOWS.map((w) => ({
                name: w.name,
                title: `Revenue Dominance ${w.title}`,
                bottom: [
                  ...percentRatio({ pattern: mining.rewards.subsidy.dominance[w.key], name: "Subsidy", color: colors.mining.subsidy }),
                  ...percentRatio({ pattern: mining.rewards.fees.dominance[w.key], name: "Fees", color: colors.mining.fee }),
                ],
              })),
              {
                name: "All-time",
                title: "Revenue Dominance (All-time)",
                bottom: [
                  ...percentRatio({ pattern: mining.rewards.subsidy.dominance, name: "Subsidy", color: colors.mining.subsidy }),
                  ...percentRatio({ pattern: mining.rewards.fees.dominance, name: "Fees", color: colors.mining.fee }),
                ],
              },
            ],
          },
          {
            name: "Fee Multiple",
            tree: ROLLING_WINDOWS.map((w) => ({
              name: w.name,
              title: `Fee-to-Subsidy Ratio ${w.title}`,
              bottom: [line({ series: mining.rewards.fees.toSubsidyRatio[w.key].ratio, name: "Ratio", color: colors.mining.fee, unit: Unit.ratio })],
            })),
          },
          {
            name: "Unclaimed",
            title: "Unclaimed Rewards (Total)",
            bottom: satsBtcUsdFrom({
              source: mining.rewards.unclaimed,
              key: "cumulative",
              name: "all-time",
            }),
          },
        ],
      },

      {
        name: "Economics",
        tree: [
          {
            name: "Hash Price",
            title: "Hash Price",
            bottom: [
              line({ series: mining.hashrate.price.ths, name: "TH/s", color: colors.usd, unit: Unit.usdPerThsPerDay }),
              line({ series: mining.hashrate.price.phs, name: "PH/s", color: colors.usd, unit: Unit.usdPerPhsPerDay }),
              dotted({ series: mining.hashrate.price.thsMin, name: "TH/s Min", color: colors.stat.min, unit: Unit.usdPerThsPerDay }),
              dotted({ series: mining.hashrate.price.phsMin, name: "PH/s Min", color: colors.stat.min, unit: Unit.usdPerPhsPerDay }),
            ],
          },
          {
            name: "Hash Value",
            title: "Hash Value",
            bottom: [
              line({ series: mining.hashrate.value.ths, name: "TH/s", color: colors.bitcoin, unit: Unit.satsPerThsPerDay }),
              line({ series: mining.hashrate.value.phs, name: "PH/s", color: colors.bitcoin, unit: Unit.satsPerPhsPerDay }),
              dotted({ series: mining.hashrate.value.thsMin, name: "TH/s Min", color: colors.stat.min, unit: Unit.satsPerThsPerDay }),
              dotted({ series: mining.hashrate.value.phsMin, name: "PH/s Min", color: colors.stat.min, unit: Unit.satsPerPhsPerDay }),
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

      {
        name: "Halving",
        tree: [
          {
            name: "Countdown",
            title: "Next Halving",
            bottom: [
              line({ series: blocks.halving.blocksToHalving, name: "Remaining", unit: Unit.blocks }),
              line({ series: blocks.halving.daysToHalving, name: "Remaining", unit: Unit.days }),
            ],
          },
          {
            name: "Epoch",
            title: "Halving Epoch",
            bottom: [line({ series: blocks.halving.epoch, name: "Epoch", unit: Unit.epoch })],
          },
        ],
      },

      {
        name: "Difficulty",
        tree: [
          {
            name: "Current",
            title: "Mining Difficulty",
            bottom: [line({ series: blocks.difficulty.value, name: "Difficulty", unit: Unit.difficulty })],
          },
          {
            name: "Adjustment",
            title: "Difficulty Adjustment",
            bottom: [baseline({ series: blocks.difficulty.adjustment.percent, name: "Change", unit: Unit.percentage })],
          },
          {
            name: "Countdown",
            title: "Next Difficulty Adjustment",
            bottom: [
              line({ series: blocks.difficulty.blocksToRetarget, name: "Remaining", unit: Unit.blocks }),
              line({ series: blocks.difficulty.daysToRetarget, name: "Remaining", unit: Unit.days }),
            ],
          },
          {
            name: "Epoch",
            title: "Difficulty Epoch",
            bottom: [line({ series: blocks.difficulty.epoch, name: "Epoch", unit: Unit.epoch })],
          },
        ],
      },
      {
        name: "Pools",
        tree: [
          createPoolCompare("Major Pools", featuredPools),
          {
            name: "AntPool & Friends",
            tree: [
              createPoolCompare("AntPool & Friends", antpoolFriends),
              ...createPoolTree(antpoolFriends),
            ],
          },
          {
            name: "Major",
            tree: createPoolTree(majorPoolData),
          },
          {
            name: "Minor",
            tree: createMinorPoolTree(minorPoolData),
          },
        ],
      },
    ],
  };
}
