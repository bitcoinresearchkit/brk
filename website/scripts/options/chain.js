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
    distribution,
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
              name: "24h Sum",
              color: colors.pink,
              unit: Unit.count,
              defaultActive: false,
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
          name: "Since last block",
          title: `${poolName} Since Last Block`,
          bottom: [
            line({
              metric: pool.blocksSinceBlock,
              name: "Blocks",
              unit: Unit.count,
            }),
            line({
              metric: pool.daysSinceBlock,
              name: "Days",
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
                metric: blocks.count._24hBlockCount,
                name: "24h sum",
                color: colors.pink,
                unit: Unit.count,
                defaultActive: false,
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
                color: colors.orange,
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
              line({
                metric: blocks.totalSize,
                name: "total",
                color: colors.purple,
                unit: Unit.bytes,
                defaultActive: false,
              }),
              ...fromFullnessPattern(blocks.vbytes, Unit.vb),
              ...fromFullnessPattern(blocks.weight, Unit.wu),
              line({
                metric: blocks.weight.sum,
                name: "sum",
                color: colors.stat.sum,
                unit: Unit.wu,
                defaultActive: false,
              }),
              line({
                metric: blocks.weight.cumulative,
                name: "cumulative",
                color: colors.stat.cumulative,
                unit: Unit.wu,
                defaultActive: false,
              }),
            ],
          },
          {
            name: "Fullness",
            title: "Block Fullness",
            bottom: fromFullnessPattern(blocks.fullness, Unit.percentage),
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
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              ...satsBtcUsd(transactions.volume.sentSum, "Sent"),
              ...satsBtcUsd(
                transactions.volume.receivedSum,
                "Received",
                colors.cyan,
                {
                  defaultActive: false,
                },
              ),
              line({
                metric: transactions.volume.annualizedVolume.bitcoin,
                name: "annualized",
                color: colors.red,
                unit: Unit.btc,
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
            name: "Fee Rate",
            title: "Fee Rate",
            bottom: fromFeeRatePattern(transactions.fees.feeRate, Unit.feeRate),
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

      // Scripts
      {
        name: "Scripts",
        tree: [
          {
            name: "Count",
            tree: [
              { name: "P2PKH", title: "P2PKH Output Count", bottom: fromDollarsPattern(scripts.count.p2pkh, Unit.count) },
              { name: "P2SH", title: "P2SH Output Count", bottom: fromDollarsPattern(scripts.count.p2sh, Unit.count) },
              { name: "P2WPKH", title: "P2WPKH Output Count", bottom: fromDollarsPattern(scripts.count.p2wpkh, Unit.count) },
              { name: "P2WSH", title: "P2WSH Output Count", bottom: fromDollarsPattern(scripts.count.p2wsh, Unit.count) },
              { name: "P2TR", title: "P2TR Output Count", bottom: fromDollarsPattern(scripts.count.p2tr, Unit.count) },
              { name: "P2PK33", title: "P2PK33 Output Count", bottom: fromDollarsPattern(scripts.count.p2pk33, Unit.count) },
              { name: "P2PK65", title: "P2PK65 Output Count", bottom: fromDollarsPattern(scripts.count.p2pk65, Unit.count) },
              { name: "P2MS", title: "P2MS Output Count", bottom: fromDollarsPattern(scripts.count.p2ms, Unit.count) },
              { name: "P2A", title: "P2A Output Count", bottom: fromDollarsPattern(scripts.count.p2a, Unit.count) },
              { name: "OP_RETURN", title: "OP_RETURN Output Count", bottom: fromDollarsPattern(scripts.count.opreturn, Unit.count) },
              { name: "SegWit", title: "SegWit Output Count", bottom: fromDollarsPattern(scripts.count.segwit, Unit.count) },
              { name: "Empty", title: "Empty Output Count", bottom: fromDollarsPattern(scripts.count.emptyoutput, Unit.count) },
              { name: "Unknown", title: "Unknown Output Count", bottom: fromDollarsPattern(scripts.count.unknownoutput, Unit.count) },
            ],
          },
          {
            name: "Adoption",
            tree: [
              {
                name: "SegWit",
                title: "SegWit Adoption",
                bottom: [
                  line({ metric: scripts.count.segwitAdoption.base, name: "base", unit: Unit.percentage }),
                  line({ metric: scripts.count.segwitAdoption.sum, name: "sum", color: colors.stat.sum, unit: Unit.percentage }),
                  line({ metric: scripts.count.segwitAdoption.cumulative, name: "cumulative", color: colors.stat.cumulative, unit: Unit.percentage, defaultActive: false }),
                ],
              },
              {
                name: "Taproot",
                title: "Taproot Adoption",
                bottom: [
                  line({ metric: scripts.count.taprootAdoption.base, name: "base", unit: Unit.percentage }),
                  line({ metric: scripts.count.taprootAdoption.sum, name: "sum", color: colors.stat.sum, unit: Unit.percentage }),
                  line({ metric: scripts.count.taprootAdoption.cumulative, name: "cumulative", color: colors.stat.cumulative, unit: Unit.percentage, defaultActive: false }),
                ],
              },
            ],
          },
          {
            name: "Value",
            tree: [
              { name: "OP_RETURN", title: "OP_RETURN Value", bottom: fromCoinbasePattern(scripts.value.opreturn) },
            ],
          },
        ],
      },

      // Supply
      {
        name: "Supply",
        tree: [
          {
            name: "Circulating",
            title: "Circulating Supply",
            bottom: fromSupplyPattern(supply.circulating, "Supply"),
          },
          {
            name: "Inflation",
            title: "Inflation Rate",
            bottom: [
              dots({
                metric: supply.inflation,
                name: "Rate",
                unit: Unit.percentage,
              }),
            ],
          },
          {
            name: "Unspendable",
            title: "Unspendable Supply",
            bottom: fromValuePattern(supply.burned.unspendable),
          },
          {
            name: "OP_RETURN",
            title: "OP_RETURN Supply",
            bottom: fromValuePattern(supply.burned.opreturn),
          },
        ],
      },

      // Rewards
      {
        name: "Rewards",
        tree: [
          {
            name: "Coinbase",
            title: "Coinbase Rewards",
            bottom: [
              ...fromCoinbasePattern(blocks.rewards.coinbase),
              ...satsBtcUsd(blocks.rewards._24hCoinbaseSum, "24h sum", colors.pink, { defaultActive: false }),
            ],
          },
          {
            name: "Subsidy",
            title: "Block Subsidy",
            bottom: [
              ...fromCoinbasePattern(blocks.rewards.subsidy),
              line({
                metric: blocks.rewards.subsidyDominance,
                name: "Dominance",
                color: colors.purple,
                unit: Unit.percentage,
                defaultActive: false,
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
            name: "Fee",
            title: "Transaction Fees",
            bottom: [
              line({
                metric: transactions.fees.fee.bitcoin.sum,
                name: "sum",
                unit: Unit.btc,
              }),
              line({
                metric: transactions.fees.fee.bitcoin.cumulative,
                name: "cumulative",
                color: colors.stat.cumulative,
                unit: Unit.btc,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.fee.sats.sum,
                name: "sum",
                unit: Unit.sats,
              }),
              line({
                metric: transactions.fees.fee.sats.cumulative,
                name: "cumulative",
                color: colors.stat.cumulative,
                unit: Unit.sats,
                defaultActive: false,
              }),
              line({
                metric: transactions.fees.fee.dollars.sum,
                name: "sum",
                unit: Unit.usd,
              }),
              line({
                metric: transactions.fees.fee.dollars.cumulative,
                name: "cumulative",
                color: colors.stat.cumulative,
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
            name: "Unclaimed",
            title: "Unclaimed Rewards",
            bottom: fromValuePattern(
              blocks.rewards.unclaimedRewards,
              "Unclaimed",
            ),
          },
        ],
      },

      // Addresses
      {
        name: "Addresses",
        tree: [
          {
            name: "Count",
            tree: [
              {
                name: "All",
                title: "Total Address Count",
                bottom: [
                  line({
                    metric: distribution.addrCount.all,
                    name: "Loaded",
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.all,
                    name: "Empty",
                    color: colors.gray,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                ],
              },
              {
                name: "Empty by Type",
                title: "Empty Address Count by Type",
                bottom: [
                  line({
                    metric: distribution.emptyAddrCount.p2pkh,
                    name: "P2PKH",
                    color: colors.orange,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2sh,
                    name: "P2SH",
                    color: colors.yellow,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2wpkh,
                    name: "P2WPKH",
                    color: colors.green,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2wsh,
                    name: "P2WSH",
                    color: colors.teal,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2tr,
                    name: "P2TR",
                    color: colors.purple,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2pk65,
                    name: "P2PK65",
                    color: colors.pink,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2pk33,
                    name: "P2PK33",
                    color: colors.red,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                  line({
                    metric: distribution.emptyAddrCount.p2a,
                    name: "P2A",
                    color: colors.blue,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                ],
              },
              {
                name: "By Type",
                title: "Address Count by Type",
                bottom: [
                  line({
                    metric: distribution.addrCount.p2pkh,
                    name: "P2PKH",
                    color: colors.orange,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.addrCount.p2sh,
                    name: "P2SH",
                    color: colors.yellow,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.addrCount.p2wpkh,
                    name: "P2WPKH",
                    color: colors.green,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.addrCount.p2wsh,
                    name: "P2WSH",
                    color: colors.teal,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.addrCount.p2tr,
                    name: "P2TR",
                    color: colors.purple,
                    unit: Unit.count,
                  }),
                  line({
                    metric: distribution.addrCount.p2pk65,
                    name: "P2PK65",
                    color: colors.pink,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                  line({
                    metric: distribution.addrCount.p2pk33,
                    name: "P2PK33",
                    color: colors.red,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                  line({
                    metric: distribution.addrCount.p2a,
                    name: "P2A",
                    color: colors.blue,
                    unit: Unit.count,
                    defaultActive: false,
                  }),
                ],
              },
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
              line({
                metric: blocks.difficulty.asHash,
                name: "Difficulty",
                color: colors.default,
                unit: Unit.hashRate,
                options: { lineStyle: 1 },
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
                metric: blocks.difficulty.epoch,
                name: "Epoch",
                color: colors.teal,
                unit: Unit.epoch,
              }),
              line({
                metric: blocks.difficulty.blocksBeforeNextAdjustment,
                name: "before next",
                color: colors.indigo,
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.difficulty.daysBeforeNextAdjustment,
                name: "before next",
                color: colors.purple,
                unit: Unit.days,
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
              priceLine({ ctx, number: 0, unit: Unit.percentage }),
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
                metric: blocks.halving.epoch,
                name: "Epoch",
                color: colors.purple,
                unit: Unit.epoch,
              }),
              line({
                metric: blocks.halving.blocksBeforeNextHalving,
                name: "before next",
                unit: Unit.blocks,
              }),
              line({
                metric: blocks.halving.daysBeforeNextHalving,
                name: "before next",
                color: colors.blue,
                unit: Unit.days,
              }),
            ],
          },
          {
            name: "Puell Multiple",
            title: "Puell Multiple",
            bottom: [
              baseline({
                metric: market.indicators.puellMultiple,
                name: "Puell Multiple",
                unit: Unit.ratio,
                base: 1,
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
    ],
  };
}
