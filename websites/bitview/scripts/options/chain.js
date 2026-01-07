/** Chain section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";

/**
 * Create Chain section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createChainSection(ctx) {
  const { colors, brk, s, createPriceLine } = ctx;
  const { mergeMetricPatterns } = brk;
  const {
    blocks,
    transactions,
    pools,
    inputs,
    outputs,
    market,
    scripts,
    supply,
  } = brk.tree.computed;
  const { indexed } = brk.tree;

  /**
   * Create sum/cumulative series from a BlockCountPattern
   * @template T
   * @param {BlockCountPattern<T>} pattern
   * @param {string} name
   * @param {Color} [sumColor]
   * @param {Color} [cumulativeColor]
   * @param {Unit} unit
   */
  const fromBlockCount = (pattern, name, unit, sumColor, cumulativeColor) => [
    s({
      metric: mergeMetricPatterns(pattern.base, pattern.sum),
      name: `${name} sum`,
      color: sumColor,
      unit,
    }),
    s({
      metric: pattern.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from BlockSizePattern (has average, min, max, percentiles)
   * @template T
   * @param {BlockSizePattern<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   */
  const fromBlockSize = (pattern, name, unit) => [
    s({ metric: pattern.distribution.average, name: `${name} avg`, unit }),
    s({
      metric: pattern.sum,
      name: `${name} sum`,
      color: colors.blue,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.cumulative,
      name: `${name} cumulative`,
      color: colors.indigo,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.min,
      name: `${name} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.max,
      name: `${name} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct10,
      name: `${name} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct25,
      name: `${name} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.median,
      name: `${name} median`,
      color: colors.purple,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct75,
      name: `${name} pct75`,
      color: colors.violet,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct90,
      name: `${name} pct90`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from CountPattern2 (has distribution with percentiles, no height index on cumulative)
   * @template T
   * @param {CountPattern2<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   */
  const fromCountPattern2 = (pattern, name, unit) => [
    s({ metric: pattern.average, name: `${name} Average`, unit }),
    s({
      metric: pattern.sum,
      name: `${name} sum`,
      color: colors.blue,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.cumulative,
      name: `${name} cumulative`,
      color: colors.indigo,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.min,
      name: `${name} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.max,
      name: `${name} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct10,
      name: `${name} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct25,
      name: `${name} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.median,
      name: `${name} median`,
      color: colors.purple,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct75,
      name: `${name} pct75`,
      color: colors.violet,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.distribution.percentiles.pct90,
      name: `${name} pct90`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from BlockIntervalPattern (has average, min, max, percentiles)
   * @template T
   * @param {BlockIntervalPattern<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   */
  const fromBlockInterval = (pattern, name, unit) => [
    s({ metric: pattern.average, name: `${name} avg`, unit }),
    s({
      metric: pattern.min,
      name: `${name} min`,
      color: colors.red,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.max,
      name: `${name} max`,
      color: colors.green,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.percentiles.pct10,
      name: `${name} pct10`,
      color: colors.rose,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.percentiles.pct25,
      name: `${name} pct25`,
      color: colors.pink,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.percentiles.median,
      name: `${name} median`,
      color: colors.purple,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.percentiles.pct75,
      name: `${name} pct75`,
      color: colors.violet,
      unit,
      defaultActive: false,
    }),
    s({
      metric: pattern.percentiles.pct90,
      name: `${name} pct90`,
      color: colors.fuchsia,
      unit,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from DollarsPattern (has base, cumulative)
   * @template T
   * @param {DollarsPattern<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   * @param {Color} [sumColor]
   * @param {Color} [cumulativeColor]
   */
  const fromBitcoin = (pattern, name, unit, sumColor, cumulativeColor) => [
    s({ metric: pattern.base, name: `${name}`, color: sumColor, unit }),
    s({
      metric: pattern.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor ?? colors.blue,
      unit,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from CoinbasePattern (has sats, bitcoin, dollars as BitcoinPattern)
   * BitcoinPattern has .base and .cumulative (no .sum)
   * @param {CoinbasePattern} pattern
   * @param {string} name
   * @param {Color} sumColor
   * @param {Color} cumulativeColor
   */
  const fromCoinbase = (pattern, name, sumColor, cumulativeColor) => [
    s({
      metric: pattern.sats.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.sats,
    }),
    s({
      metric: pattern.sats.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.sats,
      defaultActive: false,
    }),
    s({
      metric: pattern.bitcoin.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.btc,
    }),
    s({
      metric: pattern.bitcoin.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.btc,
      defaultActive: false,
    }),
    s({
      metric: pattern.dollars.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.usd,
    }),
    s({
      metric: pattern.dollars.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.usd,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from ValuePattern (has sats, bitcoin, dollars as BlockCountPattern)
   * BlockCountPattern has .base, .sum, and .cumulative
   * @param {ValuePattern} pattern
   * @param {string} name
   * @param {Color} sumColor
   * @param {Color} cumulativeColor
   */
  const fromValuePattern = (pattern, name, sumColor, cumulativeColor) => [
    s({
      metric: pattern.sats.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.sats,
    }),
    s({
      metric: pattern.sats.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.sats,
      defaultActive: false,
    }),
    s({
      metric: pattern.bitcoin.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.btc,
    }),
    s({
      metric: pattern.bitcoin.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.btc,
      defaultActive: false,
    }),
    s({
      metric: pattern.dollars.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.usd,
    }),
    s({
      metric: pattern.dollars.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.usd,
      defaultActive: false,
    }),
  ];

  /**
   * Create series from RewardPattern (has .base as Indexes2<Sats>, plus bitcoin/dollars as BlockCountPattern, sats as SatsPattern)
   * Note: SatsPattern only has cumulative and sum, so we use pattern.base for raw sats
   * @param {RewardPattern} pattern
   * @param {string} name
   * @param {Color} sumColor
   * @param {Color} cumulativeColor
   */
  const fromRewardPattern = (pattern, name, sumColor, cumulativeColor) => [
    s({
      metric: pattern.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.sats,
    }),
    s({
      metric: pattern.sats.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.sats,
      defaultActive: false,
    }),
    s({
      metric: pattern.bitcoin.base,
      name: `${name}`,
      color: sumColor,
      unit: Unit.btc,
    }),
    s({
      metric: pattern.bitcoin.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.btc,
      defaultActive: false,
    }),
    s({
      metric: pattern.dollarsSource,
      name: `${name}`,
      color: sumColor,
      unit: Unit.usd,
    }),
    s({
      metric: pattern.dollars.cumulative,
      name: `${name} cumulative`,
      color: cumulativeColor,
      unit: Unit.usd,
      defaultActive: false,
    }),
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
          title: `Mining Dominance of ${poolName}`,
          bottom: [
            s({
              metric: mergeMetricPatterns(
                pool._1dDominance.base,
                pool._1dDominance.sum,
              ),
              name: "1d",
              color: colors.rose,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            s({
              metric: pool._1wDominance,
              name: "1w",
              color: colors.red,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            s({ metric: pool._1mDominance, name: "1m", unit: Unit.percentage }),
            s({
              metric: pool._1yDominance,
              name: "1y",
              color: colors.lime,
              unit: Unit.percentage,
              defaultActive: false,
            }),
            s({
              metric: mergeMetricPatterns(
                pool.dominance.base,
                pool.dominance.sum,
              ),
              name: "all time",
              color: colors.teal,
              unit: Unit.percentage,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Blocks mined",
          title: `Blocks mined by ${poolName}`,
          bottom: [
            s({
              metric: mergeMetricPatterns(
                pool.blocksMined.base,
                pool.blocksMined.sum,
              ),
              name: "Sum",
              unit: Unit.count,
            }),
            s({
              metric: pool.blocksMined.cumulative,
              name: "Cumulative",
              color: colors.blue,
              unit: Unit.count,
            }),
            s({
              metric: pool._1wBlocksMined,
              name: "1w Sum",
              color: colors.red,
              unit: Unit.count,
              defaultActive: false,
            }),
            s({
              metric: pool._1mBlocksMined,
              name: "1m Sum",
              color: colors.pink,
              unit: Unit.count,
              defaultActive: false,
            }),
            s({
              metric: pool._1yBlocksMined,
              name: "1y Sum",
              color: colors.purple,
              unit: Unit.count,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Rewards",
          title: `Rewards collected by ${poolName}`,
          bottom: [
            ...fromValuePattern(
              pool.coinbase,
              "coinbase",
              colors.orange,
              colors.red,
            ),
            ...fromRewardPattern(
              pool.subsidy,
              "subsidy",
              colors.lime,
              colors.emerald,
            ),
            ...fromRewardPattern(pool.fee, "fee", colors.cyan, colors.indigo),
          ],
        },
        {
          name: "Days since block",
          title: `Days since ${poolName} mined a block`,
          bottom: [
            s({
              metric: pool.daysSinceBlock,
              name: "Since block",
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
              ...fromBlockCount(blocks.count.blockCount, "Block", Unit.count),
              s({
                metric: blocks.count.blockCountTarget,
                name: "Target",
                color: colors.gray,
                unit: Unit.count,
                options: { lineStyle: 4 },
              }),
              s({
                metric: blocks.count._1wBlockCount,
                name: "1w sum",
                color: colors.red,
                unit: Unit.count,
                defaultActive: false,
              }),
              s({
                metric: blocks.count._1mBlockCount,
                name: "1m sum",
                color: colors.pink,
                unit: Unit.count,
                defaultActive: false,
              }),
              s({
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
              s({
                metric: blocks.interval.interval,
                name: "Interval",
                unit: Unit.secs,
              }),
              ...fromBlockInterval(
                blocks.interval.blockInterval,
                "Interval",
                Unit.secs,
              ),
              createPriceLine({ unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              s({
                metric: blocks.size.vbytes,
                name: "vbytes raw",
                unit: Unit.vb,
              }),
              s({
                metric: indexed.block.weight,
                name: "weight raw",
                unit: Unit.wu,
              }),
              ...fromBlockSize(blocks.size.blockSize, "size", Unit.bytes),
              ...fromBlockSize(blocks.weight.blockWeight, "weight", Unit.wu),
              ...fromBlockSize(blocks.size.blockVbytes, "vbytes", Unit.vb),
            ],
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
            bottom: fromBitcoin(
              transactions.count.txCount,
              "Count",
              Unit.count,
            ),
          },
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              s({
                metric: mergeMetricPatterns(
                  transactions.volume.sentSum.sats.base,
                  transactions.volume.sentSum.sats.rest,
                ),
                name: "Sent",
                unit: Unit.sats,
              }),
              s({
                metric: transactions.volume.sentSum.bitcoin,
                name: "Sent",
                unit: Unit.btc,
              }),
              s({
                metric: mergeMetricPatterns(
                  transactions.volume.sentSum.dollars.base,
                  transactions.volume.sentSum.dollars.rest,
                ),
                name: "Sent",
                unit: Unit.usd,
              }),
              s({
                metric: transactions.volume.annualizedVolume,
                name: "annualized",
                color: colors.red,
                unit: Unit.sats,
                defaultActive: false,
              }),
              s({
                metric: transactions.volume.annualizedVolumeBtc,
                name: "annualized",
                color: colors.red,
                unit: Unit.btc,
                defaultActive: false,
              }),
              s({
                metric: transactions.volume.annualizedVolumeUsd,
                name: "annualized",
                color: colors.lime,
                unit: Unit.usd,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Size",
            title: "Transaction Size",
            bottom: [
              ...fromBlockInterval(
                transactions.size.txWeight,
                "weight",
                Unit.wu,
              ),
              ...fromBlockInterval(transactions.size.txVsize, "vsize", Unit.vb),
            ],
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromBlockCount(
                transactions.versions.txV1,
                "v1",
                Unit.count,
                colors.orange,
                colors.red,
              ),
              ...fromBlockCount(
                transactions.versions.txV2,
                "v2",
                Unit.count,
                colors.cyan,
                colors.blue,
              ),
              ...fromBlockCount(
                transactions.versions.txV3,
                "v3",
                Unit.count,
                colors.lime,
                colors.green,
              ),
            ],
          },
          {
            name: "Velocity",
            title: "Transactions Velocity",
            bottom: [
              s({
                metric: brk.mergeMetricPatterns(
                  supply.velocity.btc.dateindex,
                  supply.velocity.btc.rest,
                ),
                name: "bitcoin",
                unit: Unit.ratio,
              }),
              s({
                metric: brk.mergeMetricPatterns(
                  supply.velocity.usd.dateindex,
                  supply.velocity.usd.rest,
                ),
                name: "dollars",
                color: colors.emerald,
                unit: Unit.ratio,
              }),
            ],
          },
          {
            name: "Speed",
            title: "Transactions Per Second",
            bottom: [
              s({
                metric: transactions.volume.txPerSec,
                name: "Transactions",
                unit: Unit.perSec,
              }),
            ],
          },
        ],
      },

      // Input
      {
        name: "Input",
        tree: [
          {
            name: "Count",
            title: "Transaction Input Count",
            bottom: [
              ...fromCountPattern2(inputs.count.count, "Input", Unit.count),
            ],
          },
          {
            name: "Speed",
            title: "Inputs Per Second",
            bottom: [
              s({
                metric: transactions.volume.inputsPerSec,
                name: "Inputs",
                unit: Unit.perSec,
              }),
            ],
          },
        ],
      },

      // Output
      {
        name: "Output",
        tree: [
          {
            name: "Count",
            title: "Transaction Output Count",
            bottom: [
              ...fromCountPattern2(outputs.count.count, "Output", Unit.count),
            ],
          },
          {
            name: "Speed",
            title: "Outputs Per Second",
            bottom: [
              s({
                metric: transactions.volume.outputsPerSec,
                name: "Outputs",
                unit: Unit.perSec,
              }),
            ],
          },
        ],
      },

      // UTXO
      {
        name: "UTXO",
        tree: [
          {
            name: "Count",
            title: "UTXO Count",
            bottom: [
              s({
                metric: mergeMetricPatterns(
                  outputs.count.utxoCount.base,
                  outputs.count.utxoCount.sum,
                ),
                name: "Count",
                unit: Unit.count,
              }),
            ],
          },
        ],
      },

      // Coinbase
      {
        name: "Coinbase",
        title: "Coinbase Rewards",
        bottom: fromCoinbase(
          blocks.rewards.coinbase,
          "Coinbase",
          colors.orange,
          colors.red,
        ),
      },

      // Subsidy
      {
        name: "Subsidy",
        title: "Block Subsidy",
        bottom: [
          ...fromCoinbase(
            blocks.rewards.subsidy,
            "Subsidy",
            colors.lime,
            colors.emerald,
          ),
          s({
            metric: blocks.rewards.subsidyDominance,
            name: "Dominance",
            color: colors.purple,
            unit: Unit.percentage,
            defaultActive: false,
          }),
        ],
      },

      // Fee
      {
        name: "Fee",
        tree: [
          {
            name: "Total",
            title: "Transaction Fees",
            bottom: [
              s({
                metric: transactions.fees.fee.sats.sum,
                name: "Sum",
                unit: Unit.sats,
              }),
              s({
                metric: transactions.fees.fee.sats.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.sats,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.fee.bitcoin.sum,
                name: "Sum",
                unit: Unit.btc,
              }),
              s({
                metric: transactions.fees.fee.bitcoin.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.btc,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.fee.dollars.sum,
                name: "Sum",
                unit: Unit.usd,
              }),
              s({
                metric: transactions.fees.fee.dollars.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.usd,
                defaultActive: false,
              }),
              s({
                metric: blocks.rewards.feeDominance,
                name: "Dominance",
                color: colors.purple,
                unit: Unit.percentage,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Rate",
            title: "Fee Rate",
            bottom: [
              s({
                metric: transactions.fees.feeRate.base,
                name: "Rate",
                unit: Unit.feeRate,
              }),
              s({
                metric: transactions.fees.feeRate.average,
                name: "Average",
                color: colors.blue,
                unit: Unit.feeRate,
              }),
              s({
                metric: transactions.fees.feeRate.percentiles.median,
                name: "Median",
                color: colors.purple,
                unit: Unit.feeRate,
              }),
              s({
                metric: transactions.fees.feeRate.min,
                name: "Min",
                color: colors.red,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.max,
                name: "Max",
                color: colors.green,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.percentiles.pct10,
                name: "pct10",
                color: colors.rose,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.percentiles.pct25,
                name: "pct25",
                color: colors.pink,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.percentiles.pct75,
                name: "pct75",
                color: colors.violet,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.percentiles.pct90,
                name: "pct90",
                color: colors.fuchsia,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
            ],
          },
        ],
      },

      // Mining
      {
        name: "Mining",
        tree: [
          {
            name: "Hashrate",
            title: "Network Hashrate",
            bottom: [
              s({
                metric: blocks.mining.hashRate,
                name: "Hashrate",
                unit: Unit.hashRate,
              }),
              s({
                metric: blocks.mining.hashRate1wSma,
                name: "1w SMA",
                color: colors.red,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              s({
                metric: blocks.mining.hashRate1mSma,
                name: "1m SMA",
                color: colors.orange,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              s({
                metric: blocks.mining.hashRate2mSma,
                name: "2m SMA",
                color: colors.yellow,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
              s({
                metric: blocks.mining.hashRate1ySma,
                name: "1y SMA",
                color: colors.lime,
                unit: Unit.hashRate,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Difficulty",
            title: "Network Difficulty",
            bottom: [
              s({
                metric: blocks.mining.difficulty,
                name: "Difficulty",
                unit: Unit.difficulty,
              }),
              s({
                metric: mergeMetricPatterns(
                  blocks.mining.difficultyAdjustment.base,
                  blocks.mining.difficultyAdjustment.rest,
                ),
                name: "Adjustment",
                color: colors.orange,
                unit: Unit.percentage,
                defaultActive: false,
              }),
              s({
                metric: blocks.mining.difficultyAsHash,
                name: "As hash",
                color: colors.default,
                unit: Unit.hashRate,
                defaultActive: false,
                options: { lineStyle: 1 },
              }),
              s({
                metric: blocks.difficulty.blocksBeforeNextDifficultyAdjustment,
                name: "Blocks until adj.",
                color: colors.indigo,
                unit: Unit.blocks,
                defaultActive: false,
              }),
              s({
                metric: blocks.difficulty.daysBeforeNextDifficultyAdjustment,
                name: "Days until adj.",
                color: colors.purple,
                unit: Unit.days,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Hash Price",
            title: "Hash Price",
            bottom: [
              s({
                metric: blocks.mining.hashPriceThs,
                name: "TH/s",
                color: colors.emerald,
                unit: Unit.usdPerThsPerDay,
              }),
              s({
                metric: blocks.mining.hashPricePhs,
                name: "PH/s",
                color: colors.emerald,
                unit: Unit.usdPerPhsPerDay,
              }),
              s({
                metric: blocks.mining.hashPriceRebound,
                name: "Rebound",
                color: colors.yellow,
                unit: Unit.percentage,
              }),
              s({
                metric: blocks.mining.hashPriceThsMin,
                name: "TH/s Min",
                color: colors.red,
                unit: Unit.usdPerThsPerDay,
                options: { lineStyle: 1 },
              }),
              s({
                metric: blocks.mining.hashPricePhsMin,
                name: "PH/s Min",
                color: colors.red,
                unit: Unit.usdPerPhsPerDay,
                options: { lineStyle: 1 },
              }),
            ],
          },
          {
            name: "Hash Value",
            title: "Hash Value",
            bottom: [
              s({
                metric: blocks.mining.hashValueThs,
                name: "TH/s",
                color: colors.orange,
                unit: Unit.satsPerThsPerDay,
              }),
              s({
                metric: blocks.mining.hashValuePhs,
                name: "PH/s",
                color: colors.orange,
                unit: Unit.satsPerPhsPerDay,
              }),
              s({
                metric: blocks.mining.hashValueRebound,
                name: "Rebound",
                color: colors.yellow,
                unit: Unit.percentage,
              }),
              s({
                metric: blocks.mining.hashValueThsMin,
                name: "TH/s Min",
                color: colors.red,
                unit: Unit.satsPerThsPerDay,
                options: { lineStyle: 1 },
              }),
              s({
                metric: blocks.mining.hashValuePhsMin,
                name: "PH/s Min",
                color: colors.red,
                unit: Unit.satsPerPhsPerDay,
                options: { lineStyle: 1 },
              }),
            ],
          },
          {
            name: "Halving",
            title: "Halving Info",
            bottom: [
              s({
                metric: blocks.halving.blocksBeforeNextHalving,
                name: "Blocks until halving",
                unit: Unit.blocks,
              }),
              s({
                metric: blocks.halving.daysBeforeNextHalving,
                name: "Days until halving",
                color: colors.orange,
                unit: Unit.days,
              }),
              s({
                metric: blocks.halving.halvingepoch,
                name: "Halving epoch",
                color: colors.purple,
                unit: Unit.epoch,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Puell Multiple",
            title: "Puell Multiple",
            bottom: [
              s({
                metric: market.indicators.puellMultiple,
                name: "Puell Multiple",
                unit: Unit.ratio,
              }),
              createPriceLine({ unit: Unit.ratio, number: 1 }),
            ],
          },
        ],
      },

      // Pools
      {
        name: "Pools",
        tree: poolsTree,
      },

      // Unspendable
      {
        name: "Unspendable",
        tree: [
          {
            name: "OP_RETURN",
            tree: [
              {
                name: "Outputs",
                title: "OP_RETURN Outputs",
                bottom: fromBitcoin(
                  scripts.count.opreturnCount,
                  "Count",
                  Unit.count,
                ),
              },
            ],
          },
        ],
      },

      // Inflation
      {
        name: "Inflation",
        title: "Inflation Rate",
        bottom: [
          s({
            metric: mergeMetricPatterns(
              supply.inflation.indexes.dateindex,
              supply.inflation.indexes.rest,
            ),
            name: "Rate",
            unit: Unit.percentage,
          }),
        ],
      },
    ],
  };
}
