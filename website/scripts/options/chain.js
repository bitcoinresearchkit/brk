/** Chain section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";
import { priceLine } from "./constants.js";
import { line, baseline, dots } from "./series.js";
import { satsBtcUsd } from "./shared.js";

/**
 * Create Chain section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createChainSection(ctx) {
  const {
    colors,
    brk,
    fromSizePattern,
    fromFullnessPattern,
    fromDollarsPattern,
    fromFeeRatePattern,
    fromCoinbasePattern,
    fromValuePattern,
    fromBlockCountWithUnit,
    fromIntervalPattern,
    fromSupplyPattern,
  } = ctx;
  const {
    blocks,
    transactions,
    pools,
    inputs,
    outputs,
    market,
    scripts,
    supply,
  } = brk.metrics;

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
          title: `${poolName} Dominance`,
          bottom: [
            line({
              metric: pool._24hDominance,
              name: "24h",
              color: colors.orange,
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
              name: "all time",
              color: colors.teal,
              unit: Unit.percentage,
              defaultActive: false,
            }),
          ],
        },
        {
          name: "Blocks mined",
          title: `${poolName} Blocks`,
          bottom: [
            line({
              metric: pool.blocksMined.sum,
              name: "Sum",
              unit: Unit.count,
            }),
            line({
              metric: pool.blocksMined.cumulative,
              name: "Cumulative",
              color: colors.blue,
              unit: Unit.count,
            }),
            line({
              metric: pool._1wBlocksMined,
              name: "1w Sum",
              color: colors.red,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
              metric: pool._1mBlocksMined,
              name: "1m Sum",
              color: colors.pink,
              unit: Unit.count,
              defaultActive: false,
            }),
            line({
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
          title: `${poolName} Rewards`,
          bottom: [
            ...fromValuePattern(
              pool.coinbase,
              "coinbase",
              colors.orange,
              colors.red,
            ),
            ...fromValuePattern(
              pool.subsidy,
              "subsidy",
              colors.lime,
              colors.emerald,
            ),
            ...fromValuePattern(pool.fee, "fee", colors.cyan, colors.indigo),
          ],
        },
        {
          name: "Days since block",
          title: `${poolName} Last Block`,
          bottom: [
            line({
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
              ...fromBlockCountWithUnit(blocks.count.blockCount, Unit.count),
              line({
                metric: blocks.count.blockCountTarget,
                name: "Target",
                color: colors.gray,
                unit: Unit.count,
                options: { lineStyle: 4 },
              }),
              line({
                metric: blocks.count._1wBlockCount,
                name: "1w sum",
                color: colors.red,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
                metric: blocks.count._1mBlockCount,
                name: "1m sum",
                color: colors.pink,
                unit: Unit.count,
                defaultActive: false,
              }),
              line({
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
              ...fromIntervalPattern(blocks.interval, Unit.secs),
              priceLine({ ctx, unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              ...fromSizePattern(blocks.size, Unit.bytes),
              ...fromFullnessPattern(blocks.vbytes, Unit.vb),
              ...fromFullnessPattern(blocks.weight, Unit.wu),
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
            bottom: fromDollarsPattern(transactions.count.txCount, Unit.count),
          },
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              ...satsBtcUsd(transactions.volume.sentSum, "Sent"),
              ...satsBtcUsd(transactions.volume.receivedSum, "Received", colors.cyan, {
                defaultActive: false,
              }),
              line({
                metric: transactions.volume.annualizedVolume.sats,
                name: "annualized",
                color: colors.red,
                unit: Unit.sats,
                defaultActive: false,
              }),
              line({
                metric: transactions.volume.annualizedVolume.bitcoin,
                name: "annualized",
                color: colors.red,
                unit: Unit.btc,
                defaultActive: false,
              }),
              line({
                metric: transactions.volume.annualizedVolume.dollars,
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
              ...fromFeeRatePattern(transactions.size.weight, Unit.wu),
              ...fromFeeRatePattern(transactions.size.vsize, Unit.vb),
            ],
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromBlockCountWithUnit(
                transactions.versions.v1,
                Unit.count,
                "v1",
                colors.orange,
                colors.red,
              ),
              ...fromBlockCountWithUnit(
                transactions.versions.v2,
                Unit.count,
                "v2",
                colors.cyan,
                colors.blue,
              ),
              ...fromBlockCountWithUnit(
                transactions.versions.v3,
                Unit.count,
                "v3",
                colors.lime,
                colors.green,
              ),
            ],
          },
          {
            name: "Velocity",
            title: "Transactions Velocity",
            bottom: [
              line({
                metric: supply.velocity.btc,
                name: "bitcoin",
                unit: Unit.ratio,
              }),
              line({
                metric: supply.velocity.usd,
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
              dots({
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
            title: "Input Count",
            bottom: [...fromSizePattern(inputs.count, Unit.count)],
          },
          {
            name: "Speed",
            title: "Inputs Per Second",
            bottom: [
              dots({
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
            title: "Output Count",
            bottom: [...fromSizePattern(outputs.count.totalCount, Unit.count)],
          },
          {
            name: "Speed",
            title: "Outputs Per Second",
            bottom: [
              dots({
                metric: transactions.volume.outputsPerSec,
                name: "Outputs",
                unit: Unit.perSec,
              }),
            ],
          },
        ],
      },

      {
        name: "UTXO",
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
        ],
      },

      // Coinbase
      {
        name: "Coinbase",
        title: "Coinbase Rewards",
        bottom: fromCoinbasePattern(blocks.rewards.coinbase, "Coinbase"),
      },

      // Subsidy
      {
        name: "Subsidy",
        title: "Block Subsidy",
        bottom: [
          ...fromCoinbasePattern(blocks.rewards.subsidy, "Subsidy"),
          line({
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
              line({
                metric: transactions.fees.fee.sats.sum,
                name: "Sum",
                unit: Unit.sats,
              }),
              line({
                metric: transactions.fees.fee.sats.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.sats,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.fee.bitcoin.sum,
                name: "Sum",
                unit: Unit.btc,
              }),
              line({
                metric: transactions.fees.fee.bitcoin.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.btc,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.fee.dollars.sum,
                name: "Sum",
                unit: Unit.usd,
              }),
              line({
                metric: transactions.fees.fee.dollars.cumulative,
                name: "Cumulative",
                color: colors.blue,
                unit: Unit.usd,
                defaultActive: false,
              }),
              line({
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
              line({
                metric: transactions.fees.feeRate.median,
                name: "Median",
                color: colors.purple,
                unit: Unit.feeRate,
              }),
              line({
                metric: transactions.fees.feeRate.average,
                name: "Average",
                color: colors.blue,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.min,
                name: "Min",
                color: colors.red,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.max,
                name: "Max",
                color: colors.green,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.pct10,
                name: "pct10",
                color: colors.rose,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.pct25,
                name: "pct25",
                color: colors.pink,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.pct75,
                name: "pct75",
                color: colors.violet,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.feeRate.pct90,
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
            ],
          },
          {
            name: "Difficulty",
            title: "Network Difficulty",
            bottom: [
              line({
                metric: blocks.difficulty.raw,
                name: "Difficulty",
                unit: Unit.difficulty,
              }),
              line({
                metric: blocks.difficulty.adjustment,
                name: "Adjustment",
                color: colors.orange,
                unit: Unit.percentage,
                defaultActive: false,
              }),
              line({
                metric: blocks.difficulty.asHash,
                name: "As hash",
                color: colors.default,
                unit: Unit.hashRate,
                defaultActive: false,
                options: { lineStyle: 1 },
              }),
              line({
                metric: blocks.difficulty.blocksBeforeNextAdjustment,
                name: "Blocks until adj.",
                color: colors.indigo,
                unit: Unit.blocks,
                defaultActive: false,
              }),
              line({
                metric: blocks.difficulty.daysBeforeNextAdjustment,
                name: "Days until adj.",
                color: colors.purple,
                unit: Unit.days,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Adjustment",
            title: "Difficulty Adjustment",
            bottom: [
              baseline({
                metric: blocks.difficulty.adjustment,
                name: "Difficulty Change",
                unit: Unit.percentage,
              }),
            ],
          },
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
              line({
                metric: blocks.mining.hashPriceThsMin,
                name: "TH/s Min",
                color: colors.red,
                unit: Unit.usdPerThsPerDay,
                options: { lineStyle: 1 },
              }),
              line({
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
              line({
                metric: blocks.mining.hashValueThsMin,
                name: "TH/s Min",
                color: colors.red,
                unit: Unit.satsPerThsPerDay,
                options: { lineStyle: 1 },
              }),
              line({
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
            title: "Halving",
            bottom: [
              line({
                metric: blocks.halving.blocksBeforeNextHalving,
                name: "Blocks before next",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.halving.daysBeforeNextHalving,
                name: "Days before next",
                color: colors.orange,
                unit: Unit.days,
              }),
              line({
                metric: blocks.halving.epoch,
                name: "Epoch",
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
              line({
                metric: market.indicators.puellMultiple,
                name: "Puell Multiple",
                unit: Unit.ratio,
              }),
              priceLine({ ctx, unit: Unit.ratio, number: 1 }),
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
            name: "Supply",
            title: "Unspendable Supply",
            bottom: fromValuePattern(supply.burned.unspendable, "Supply"),
          },
          {
            name: "OP_RETURN",
            tree: [
              {
                name: "Outputs",
                title: "OP_RETURN Outputs",
                bottom: fromFullnessPattern(scripts.count.opreturn, Unit.count),
              },
              {
                name: "Supply",
                title: "OP_RETURN Supply",
                bottom: fromValuePattern(supply.burned.opreturn, "Supply"),
              },
            ],
          },
        ],
      },

      // Supply
      {
        name: "Supply",
        title: "Circulating Supply",
        bottom: fromSupplyPattern(supply.circulating, "Supply"),
      },

      // Inflation
      {
        name: "Inflation",
        title: "Inflation Rate",
        bottom: [
          line({
            metric: supply.inflation,
            name: "Rate",
            unit: Unit.percentage,
          }),
        ],
      },

      // Unclaimed Rewards
      {
        name: "Unclaimed Rewards",
        title: "Unclaimed Rewards",
        bottom: fromValuePattern(blocks.rewards.unclaimedRewards, "Unclaimed"),
      },
    ],
  };
}
