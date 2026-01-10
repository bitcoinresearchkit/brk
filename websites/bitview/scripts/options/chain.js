/** Chain section builder - typed tree-based patterns */

import { Unit } from "../utils/units.js";

/**
 * Create Chain section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createChainSection(ctx) {
  const {
    colors,
    brk,
    s,
    createPriceLine,
    fromSizePattern,
    fromFullnessPattern,
    fromFeeRatePattern,
    fromCoinbasePattern,
    fromValuePattern,
    fromBlockCountWithUnit,
    fromIntervalPattern,
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
  } = brk.tree;

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
          title: `Blocks mined by ${poolName}`,
          bottom: [
            s({
              metric: pool.blocksMined.sum,
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
            ...fromValuePattern(pool.coinbase, "coinbase", colors.orange, colors.red),
            ...fromValuePattern(pool.subsidy, "subsidy", colors.lime, colors.emerald),
            ...fromValuePattern(pool.fee, "fee", colors.cyan, colors.indigo),
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
              ...fromBlockCountWithUnit(blocks.count.blockCount, "Block", Unit.count),
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
              ...fromIntervalPattern(blocks.interval, "Interval", Unit.secs),
              createPriceLine({ unit: Unit.secs, name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              ...fromSizePattern(blocks.size, "Size", Unit.bytes),
              ...fromFullnessPattern(blocks.vbytes, "Vbytes", Unit.vb),
              ...fromFullnessPattern(blocks.weight, "Weight", Unit.wu),
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
            bottom: fromFullnessPattern(
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
                metric: transactions.volume.sentSum.sats,
                name: "Sent",
                unit: Unit.sats,
              }),
              s({
                metric: transactions.volume.sentSum.bitcoin,
                name: "Sent",
                unit: Unit.btc,
              }),
              s({
                metric: transactions.volume.sentSum.dollars,
                name: "Sent",
                unit: Unit.usd,
              }),
              s({
                metric: transactions.volume.annualizedVolume.sats,
                name: "annualized",
                color: colors.red,
                unit: Unit.sats,
                defaultActive: false,
              }),
              s({
                metric: transactions.volume.annualizedVolume.bitcoin,
                name: "annualized",
                color: colors.red,
                unit: Unit.btc,
                defaultActive: false,
              }),
              s({
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
              ...fromFeeRatePattern(transactions.size.weight, "weight", Unit.wu),
              ...fromFeeRatePattern(transactions.size.vsize, "vsize", Unit.vb),
            ],
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromBlockCountWithUnit(transactions.versions.v1, "v1", Unit.count, colors.orange, colors.red),
              ...fromBlockCountWithUnit(transactions.versions.v2, "v2", Unit.count, colors.cyan, colors.blue),
              ...fromBlockCountWithUnit(transactions.versions.v3, "v3", Unit.count, colors.lime, colors.green),
            ],
          },
          {
            name: "Velocity",
            title: "Transactions Velocity",
            bottom: [
              s({
                metric: supply.velocity.btc,
                name: "bitcoin",
                unit: Unit.ratio,
              }),
              s({
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
            bottom: [...fromSizePattern(inputs.count, "Input", Unit.count)],
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
              ...fromSizePattern(
                outputs.count.totalCount,
                "Output",
                Unit.count,
              ),
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

      {
        name: "UTXO",
        tree: [
          {
            name: "Count",
            title: "UTXO Count",
            bottom: fromFullnessPattern(outputs.count.utxoCount, "Count", Unit.count),
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
                metric: transactions.fees.feeRate.median,
                name: "Median",
                color: colors.purple,
                unit: Unit.feeRate,
              }),
              s({
                metric: transactions.fees.feeRate.average,
                name: "Average",
                color: colors.blue,
                unit: Unit.feeRate,
                defaultActive: false,
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
                metric: transactions.fees.feeRate.pct10,
                name: "pct10",
                color: colors.rose,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.pct25,
                name: "pct25",
                color: colors.pink,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
                metric: transactions.fees.feeRate.pct75,
                name: "pct75",
                color: colors.violet,
                unit: Unit.feeRate,
                defaultActive: false,
              }),
              s({
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
                metric: blocks.difficulty.raw,
                name: "Difficulty",
                unit: Unit.difficulty,
              }),
              s({
                metric: blocks.difficulty.adjustment,
                name: "Adjustment",
                color: colors.orange,
                unit: Unit.percentage,
                defaultActive: false,
              }),
              s({
                metric: blocks.difficulty.asHash,
                name: "As hash",
                color: colors.default,
                unit: Unit.hashRate,
                defaultActive: false,
                options: { lineStyle: 1 },
              }),
              s({
                metric: blocks.difficulty.blocksBeforeNextAdjustment,
                name: "Blocks until adj.",
                color: colors.indigo,
                unit: Unit.blocks,
                defaultActive: false,
              }),
              s({
                metric: blocks.difficulty.daysBeforeNextAdjustment,
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
                metric: blocks.halving.epoch,
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
                bottom: fromFullnessPattern(
                  scripts.count.opreturn,
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
            metric: supply.inflation,
            name: "Rate",
            unit: Unit.percentage,
          }),
        ],
      },
    ],
  };
}
