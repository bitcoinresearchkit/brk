/** Chain section builder - typed tree-based patterns */

/**
 * Create Chain section
 * @param {PartialContext} ctx
 * @returns {PartialOptionsGroup}
 */
export function createChainSection(ctx) {
  const { colors, brk, s, createPriceLine } = ctx;
  const { blocks, transactions, pools, inputs, outputs, market, scripts, supply } = brk.tree.computed;
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
    s({ metric: pattern.base, name: `${name} sum`, color: sumColor, unit }),
    s({ metric: pattern.cumulative, name: `${name} cumulative`, color: cumulativeColor ?? colors.blue, unit, defaultActive: false }),
  ];

  /**
   * Create series from BlockSizePattern (has average, min, max, percentiles)
   * @template T
   * @param {BlockSizePattern<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   */
  const fromBlockSize = (pattern, name, unit) => [
    s({ metric: pattern.average, name: `${name} avg`, unit }),
    s({ metric: pattern.sum, name: `${name} sum`, color: colors.blue, unit, defaultActive: false }),
    s({ metric: pattern.cumulative, name: `${name} cumulative`, color: colors.indigo, unit, defaultActive: false }),
    s({ metric: pattern.min, name: `${name} min`, color: colors.red, unit, defaultActive: false }),
    s({ metric: pattern.max, name: `${name} max`, color: colors.green, unit, defaultActive: false }),
    s({ metric: pattern.pct10, name: `${name} pct10`, color: colors.rose, unit, defaultActive: false }),
    s({ metric: pattern.pct25, name: `${name} pct25`, color: colors.pink, unit, defaultActive: false }),
    s({ metric: pattern.median, name: `${name} median`, color: colors.purple, unit, defaultActive: false }),
    s({ metric: pattern.pct75, name: `${name} pct75`, color: colors.violet, unit, defaultActive: false }),
    s({ metric: pattern.pct90, name: `${name} pct90`, color: colors.fuchsia, unit, defaultActive: false }),
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
    s({ metric: pattern.min, name: `${name} min`, color: colors.red, unit, defaultActive: false }),
    s({ metric: pattern.max, name: `${name} max`, color: colors.green, unit, defaultActive: false }),
    s({ metric: pattern.pct10, name: `${name} pct10`, color: colors.rose, unit, defaultActive: false }),
    s({ metric: pattern.pct25, name: `${name} pct25`, color: colors.pink, unit, defaultActive: false }),
    s({ metric: pattern.median, name: `${name} median`, color: colors.purple, unit, defaultActive: false }),
    s({ metric: pattern.pct75, name: `${name} pct75`, color: colors.violet, unit, defaultActive: false }),
    s({ metric: pattern.pct90, name: `${name} pct90`, color: colors.fuchsia, unit, defaultActive: false }),
  ];

  /**
   * Create series from BitcoinPattern (has base, cumulative)
   * @template T
   * @param {BitcoinPattern<T>} pattern
   * @param {string} name
   * @param {Unit} unit
   * @param {Color} [sumColor]
   * @param {Color} [cumulativeColor]
   */
  const fromBitcoin = (pattern, name, unit, sumColor, cumulativeColor) => [
    s({ metric: pattern.base, name: `${name}`, color: sumColor, unit }),
    s({ metric: pattern.cumulative, name: `${name} cumulative`, color: cumulativeColor ?? colors.blue, unit, defaultActive: false }),
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
    s({ metric: pattern.sats.base, name: `${name}`, color: sumColor, unit: "sats" }),
    s({ metric: pattern.sats.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "sats", defaultActive: false }),
    s({ metric: pattern.bitcoin.base, name: `${name}`, color: sumColor, unit: "btc" }),
    s({ metric: pattern.bitcoin.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "btc", defaultActive: false }),
    s({ metric: pattern.dollars.base, name: `${name}`, color: sumColor, unit: "usd" }),
    s({ metric: pattern.dollars.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "usd", defaultActive: false }),
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
    s({ metric: pattern.sats.base, name: `${name}`, color: sumColor, unit: "sats" }),
    s({ metric: pattern.sats.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "sats", defaultActive: false }),
    s({ metric: pattern.bitcoin.base, name: `${name}`, color: sumColor, unit: "btc" }),
    s({ metric: pattern.bitcoin.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "btc", defaultActive: false }),
    s({ metric: pattern.dollars.base, name: `${name}`, color: sumColor, unit: "usd" }),
    s({ metric: pattern.dollars.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "usd", defaultActive: false }),
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
    s({ metric: pattern.base, name: `${name}`, color: sumColor, unit: "sats" }),
    s({ metric: pattern.sats.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "sats", defaultActive: false }),
    s({ metric: pattern.bitcoin.base, name: `${name}`, color: sumColor, unit: "btc" }),
    s({ metric: pattern.bitcoin.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "btc", defaultActive: false }),
    s({ metric: pattern.dollars.base, name: `${name}`, color: sumColor, unit: "usd" }),
    s({ metric: pattern.dollars.cumulative, name: `${name} cumulative`, color: cumulativeColor, unit: "usd", defaultActive: false }),
  ];

  // Build pools tree dynamically
  const poolEntries = Object.entries(pools.vecs);
  const poolsTree = poolEntries.map(([key, pool]) => {
    const poolName = brk.POOL_ID_TO_POOL_NAME[/** @type {keyof typeof brk.POOL_ID_TO_POOL_NAME} */ (key.toLowerCase())] || key;
    return {
      name: poolName,
      tree: [
        {
          name: "Dominance",
          title: `Mining Dominance of ${poolName}`,
          bottom: [
            s({ metric: pool._1dDominance.base, name: "1d", color: colors.rose, unit: "percentage", defaultActive: false }),
            s({ metric: pool._1wDominance, name: "1w", color: colors.red, unit: "percentage", defaultActive: false }),
            s({ metric: pool._1mDominance, name: "1m", unit: "percentage" }),
            s({ metric: pool._1yDominance, name: "1y", color: colors.lime, unit: "percentage", defaultActive: false }),
            s({ metric: pool.dominance.base, name: "all time", color: colors.teal, unit: "percentage", defaultActive: false }),
          ],
        },
        {
          name: "Blocks mined",
          title: `Blocks mined by ${poolName}`,
          bottom: [
            s({ metric: pool.blocksMined.base, name: "Sum", unit: "count" }),
            s({ metric: pool.blocksMined.cumulative, name: "Cumulative", color: colors.blue, unit: "count" }),
            s({ metric: pool._1wBlocksMined, name: "1w Sum", color: colors.red, unit: "count", defaultActive: false }),
            s({ metric: pool._1mBlocksMined, name: "1m Sum", color: colors.pink, unit: "count", defaultActive: false }),
            s({ metric: pool._1yBlocksMined, name: "1y Sum", color: colors.purple, unit: "count", defaultActive: false }),
          ],
        },
        {
          name: "Rewards",
          title: `Rewards collected by ${poolName}`,
          bottom: [
            ...fromValuePattern(pool.coinbase, "coinbase", colors.orange, colors.red),
            ...fromRewardPattern(pool.subsidy, "subsidy", colors.lime, colors.emerald),
            ...fromRewardPattern(pool.fee, "fee", colors.cyan, colors.indigo),
          ],
        },
        {
          name: "Days since block",
          title: `Days since ${poolName} mined a block`,
          bottom: [
            s({ metric: pool.daysSinceBlock, name: "Since block", unit: "days" }),
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
              ...fromBlockCount(blocks.count.blockCount, "Block", "count"),
              s({ metric: blocks.count.blockCountTarget, name: "Target", color: colors.gray, unit: "count", options: { lineStyle: 4 } }),
              s({ metric: blocks.count._1wBlockCount, name: "1w sum", color: colors.red, unit: "count", defaultActive: false }),
              s({ metric: blocks.count._1mBlockCount, name: "1m sum", color: colors.pink, unit: "count", defaultActive: false }),
              s({ metric: blocks.count._1yBlockCount, name: "1y sum", color: colors.purple, unit: "count", defaultActive: false }),
            ],
          },
          {
            name: "Interval",
            title: "Block Interval",
            bottom: [
              s({ metric: blocks.interval.interval, name: "Interval", unit: "secs" }),
              ...fromBlockInterval(blocks.interval.blockInterval, "Interval", "secs"),
              createPriceLine({ unit: "secs", name: "Target", number: 600 }),
            ],
          },
          {
            name: "Size",
            title: "Block Size",
            bottom: [
              s({ metric: blocks.size.vbytes, name: "vbytes raw", unit: "vb" }),
              s({ metric: indexed.block.weight, name: "weight raw", unit: "wu" }),
              ...fromBlockSize(blocks.size.blockSize, "size", "bytes"),
              ...fromBlockSize(blocks.weight.blockWeight, "weight", "wu"),
              ...fromBlockSize(blocks.size.blockVbytes, "vbytes", "vb"),
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
            bottom: fromBitcoin(transactions.count.txCount, "Count", "count"),
          },
          {
            name: "Volume",
            title: "Transaction Volume",
            bottom: [
              s({ metric: transactions.volume.sentSum.sats, name: "Sent", unit: "sats" }),
              s({ metric: transactions.volume.sentSum.bitcoin.base, name: "Sent", unit: "btc" }),
              s({ metric: transactions.volume.sentSum.dollars, name: "Sent", unit: "usd" }),
              s({ metric: transactions.volume.annualizedVolume, name: "annualized", color: colors.red, unit: "sats", defaultActive: false }),
              s({ metric: transactions.volume.annualizedVolumeBtc, name: "annualized", color: colors.red, unit: "btc", defaultActive: false }),
              s({ metric: transactions.volume.annualizedVolumeUsd, name: "annualized", color: colors.lime, unit: "usd", defaultActive: false }),
            ],
          },
          {
            name: "Size",
            title: "Transaction Size",
            bottom: [
              ...fromBlockInterval(transactions.size.txWeight, "weight", "wu"),
              ...fromBlockInterval(transactions.size.txVsize, "vsize", "vb"),
            ],
          },
          {
            name: "Versions",
            title: "Transaction Versions",
            bottom: [
              ...fromBlockCount(transactions.versions.txV1, "v1", "count", colors.orange, colors.red),
              ...fromBlockCount(transactions.versions.txV2, "v2", "count", colors.cyan, colors.blue),
              ...fromBlockCount(transactions.versions.txV3, "v3", "count", colors.lime, colors.green),
            ],
          },
          {
            name: "Velocity",
            title: "Transactions Velocity",
            bottom: [
              s({ metric: supply.velocity.btc, name: "bitcoin", unit: "ratio" }),
              s({ metric: supply.velocity.usd, name: "dollars", color: colors.emerald, unit: "ratio" }),
            ],
          },
          {
            name: "Speed",
            title: "Transactions Per Second",
            bottom: [
              s({ metric: transactions.volume.txPerSec, name: "Transactions", unit: "/sec" }),
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
              ...fromBlockSize(inputs.count.count, "Input", "count"),
            ],
          },
          {
            name: "Speed",
            title: "Inputs Per Second",
            bottom: [
              s({ metric: transactions.volume.inputsPerSec, name: "Inputs", unit: "/sec" }),
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
              ...fromBlockSize(outputs.count.count, "Output", "count"),
            ],
          },
          {
            name: "Speed",
            title: "Outputs Per Second",
            bottom: [
              s({ metric: transactions.volume.outputsPerSec, name: "Outputs", unit: "/sec" }),
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
              s({ metric: outputs.count.utxoCount.base, name: "Count", unit: "count" }),
            ],
          },
        ],
      },

      // Coinbase
      {
        name: "Coinbase",
        title: "Coinbase Rewards",
        bottom: fromCoinbase(blocks.rewards.coinbase, "Coinbase", colors.orange, colors.red),
      },

      // Subsidy
      {
        name: "Subsidy",
        title: "Block Subsidy",
        bottom: [
          ...fromCoinbase(blocks.rewards.subsidy, "Subsidy", colors.lime, colors.emerald),
          s({ metric: blocks.rewards.subsidyDominance, name: "Dominance", color: colors.purple, unit: "percentage", defaultActive: false }),
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
              s({ metric: transactions.fees.fee.sats.sum, name: "Sum", unit: "sats" }),
              s({ metric: transactions.fees.fee.sats.cumulative, name: "Cumulative", color: colors.blue, unit: "sats", defaultActive: false }),
              s({ metric: transactions.fees.fee.bitcoin.sum, name: "Sum", unit: "btc" }),
              s({ metric: transactions.fees.fee.bitcoin.cumulative, name: "Cumulative", color: colors.blue, unit: "btc", defaultActive: false }),
              s({ metric: transactions.fees.fee.dollars.sum, name: "Sum", unit: "usd" }),
              s({ metric: transactions.fees.fee.dollars.cumulative, name: "Cumulative", color: colors.blue, unit: "usd", defaultActive: false }),
              s({ metric: blocks.rewards.feeDominance, name: "Dominance", color: colors.purple, unit: "percentage", defaultActive: false }),
            ],
          },
          {
            name: "Rate",
            title: "Fee Rate",
            bottom: [
              s({ metric: transactions.fees.feeRate.base, name: "Rate", unit: "sat/vb" }),
              s({ metric: transactions.fees.feeRate.average, name: "Average", color: colors.blue, unit: "sat/vb" }),
              s({ metric: transactions.fees.feeRate.median, name: "Median", color: colors.purple, unit: "sat/vb" }),
              s({ metric: transactions.fees.feeRate.min, name: "Min", color: colors.red, unit: "sat/vb", defaultActive: false }),
              s({ metric: transactions.fees.feeRate.max, name: "Max", color: colors.green, unit: "sat/vb", defaultActive: false }),
              s({ metric: transactions.fees.feeRate.pct10, name: "pct10", color: colors.rose, unit: "sat/vb", defaultActive: false }),
              s({ metric: transactions.fees.feeRate.pct25, name: "pct25", color: colors.pink, unit: "sat/vb", defaultActive: false }),
              s({ metric: transactions.fees.feeRate.pct75, name: "pct75", color: colors.violet, unit: "sat/vb", defaultActive: false }),
              s({ metric: transactions.fees.feeRate.pct90, name: "pct90", color: colors.fuchsia, unit: "sat/vb", defaultActive: false }),
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
              s({ metric: blocks.mining.hashRate, name: "Hashrate", unit: "h/s" }),
              s({ metric: blocks.mining.hashRate1wSma, name: "1w SMA", color: colors.red, unit: "h/s", defaultActive: false }),
              s({ metric: blocks.mining.hashRate1mSma, name: "1m SMA", color: colors.orange, unit: "h/s", defaultActive: false }),
              s({ metric: blocks.mining.hashRate2mSma, name: "2m SMA", color: colors.yellow, unit: "h/s", defaultActive: false }),
              s({ metric: blocks.mining.hashRate1ySma, name: "1y SMA", color: colors.lime, unit: "h/s", defaultActive: false }),
            ],
          },
          {
            name: "Difficulty",
            title: "Network Difficulty",
            bottom: [
              s({ metric: blocks.mining.difficulty, name: "Difficulty", unit: "difficulty" }),
              s({ metric: blocks.mining.difficultyAdjustment, name: "Adjustment", color: colors.orange, unit: "percentage", defaultActive: false }),
              s({ metric: blocks.mining.difficultyAsHash, name: "As hash", color: colors.default, unit: "h/s", defaultActive: false, options: { lineStyle: 1 } }),
              s({ metric: blocks.difficulty.blocksBeforeNextDifficultyAdjustment, name: "Blocks until adj.", color: colors.indigo, unit: "blocks", defaultActive: false }),
              s({ metric: blocks.difficulty.daysBeforeNextDifficultyAdjustment, name: "Days until adj.", color: colors.purple, unit: "days", defaultActive: false }),
            ],
          },
          {
            name: "Hash Price",
            title: "Hash Price",
            bottom: [
              s({ metric: blocks.mining.hashPriceThs, name: "TH/s", color: colors.emerald, unit: "usd/(th/s)/day" }),
              s({ metric: blocks.mining.hashPricePhs, name: "PH/s", color: colors.emerald, unit: "usd/(ph/s)/day" }),
              s({ metric: blocks.mining.hashPriceRebound, name: "Rebound", color: colors.yellow, unit: "percentage" }),
              s({ metric: blocks.mining.hashPriceThsMin, name: "TH/s Min", color: colors.red, unit: "usd/(th/s)/day", options: { lineStyle: 1 } }),
              s({ metric: blocks.mining.hashPricePhsMin, name: "PH/s Min", color: colors.red, unit: "usd/(ph/s)/day", options: { lineStyle: 1 } }),
            ],
          },
          {
            name: "Hash Value",
            title: "Hash Value",
            bottom: [
              s({ metric: blocks.mining.hashValueThs, name: "TH/s", color: colors.orange, unit: "sats/(th/s)/day" }),
              s({ metric: blocks.mining.hashValuePhs, name: "PH/s", color: colors.orange, unit: "sats/(ph/s)/day" }),
              s({ metric: blocks.mining.hashValueRebound, name: "Rebound", color: colors.yellow, unit: "percentage" }),
              s({ metric: blocks.mining.hashValueThsMin, name: "TH/s Min", color: colors.red, unit: "sats/(th/s)/day", options: { lineStyle: 1 } }),
              s({ metric: blocks.mining.hashValuePhsMin, name: "PH/s Min", color: colors.red, unit: "sats/(ph/s)/day", options: { lineStyle: 1 } }),
            ],
          },
          {
            name: "Halving",
            title: "Halving Info",
            bottom: [
              s({ metric: blocks.halving.blocksBeforeNextHalving, name: "Blocks until halving", unit: "blocks" }),
              s({ metric: blocks.halving.daysBeforeNextHalving, name: "Days until halving", color: colors.orange, unit: "days" }),
              s({ metric: blocks.halving.halvingepoch, name: "Halving epoch", color: colors.purple, unit: "epoch", defaultActive: false }),
            ],
          },
          {
            name: "Puell Multiple",
            title: "Puell Multiple",
            bottom: [
              s({ metric: market.indicators.puellMultiple, name: "Puell Multiple", unit: "ratio" }),
              createPriceLine({ unit: "ratio", number: 1 }),
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
                bottom: fromBitcoin(scripts.count.opreturnCount, "Count", "count"),
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
          s({ metric: supply.inflation.indexes, name: "Rate", unit: "percentage" }),
        ],
      },
    ],
  };
}
