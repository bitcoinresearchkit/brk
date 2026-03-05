// Auto-generated BRK JavaScript client
// Do not edit manually

// Type definitions

/**
 * Bitcoin address string
 *
 * @typedef {string} Address
 */
/**
 * Address statistics on the blockchain (confirmed transactions only)
 *
 * Based on mempool.space's format with type_index extension.
 *
 * @typedef {Object} AddressChainStats
 * @property {number} fundedTxoCount - Total number of transaction outputs that funded this address
 * @property {Sats} fundedTxoSum - Total amount in satoshis received by this address across all funded outputs
 * @property {number} spentTxoCount - Total number of transaction outputs spent from this address
 * @property {Sats} spentTxoSum - Total amount in satoshis spent from this address
 * @property {number} txCount - Total number of confirmed transactions involving this address
 * @property {TypeIndex} typeIndex - Index of this address within its type on the blockchain
 */
/**
 * Address statistics in the mempool (unconfirmed transactions only)
 *
 * Based on mempool.space's format.
 *
 * @typedef {Object} AddressMempoolStats
 * @property {number} fundedTxoCount - Number of unconfirmed transaction outputs funding this address
 * @property {Sats} fundedTxoSum - Total amount in satoshis being received in unconfirmed transactions
 * @property {number} spentTxoCount - Number of unconfirmed transaction inputs spending from this address
 * @property {Sats} spentTxoSum - Total amount in satoshis being spent in unconfirmed transactions
 * @property {number} txCount - Number of unconfirmed transactions involving this address
 */
/**
 * @typedef {Object} AddressParam
 * @property {Address} address
 */
/**
 * Address information compatible with mempool.space API format
 *
 * @typedef {Object} AddressStats
 * @property {Address} address - Bitcoin address string
 * @property {AddressChainStats} chainStats - Statistics for confirmed transactions on the blockchain
 * @property {(AddressMempoolStats|null)=} mempoolStats - Statistics for unconfirmed transactions in the mempool
 */
/**
 * @typedef {Object} AddressTxidsParam
 * @property {(Txid|null)=} afterTxid - Txid to paginate from (return transactions before this one)
 * @property {number=} limit - Maximum number of results to return. Defaults to 25 if not specified.
 */
/**
 * Address validation result
 *
 * @typedef {Object} AddressValidation
 * @property {boolean} isvalid - Whether the address is valid
 * @property {?string=} address - The validated address
 * @property {?string=} scriptPubKey - The scriptPubKey in hex
 * @property {?boolean=} isscript - Whether this is a script address (P2SH)
 * @property {?boolean=} iswitness - Whether this is a witness address
 * @property {?number=} witnessVersion - Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
 * @property {?string=} witnessProgram - Witness program in hex
 */
/**
 * Unified index for any address type (funded or empty)
 *
 * @typedef {TypeIndex} AnyAddressIndex
 */
/**
 * Unsigned basis points stored as u16.
 * 1 bp = 0.0001. Range: 0–6.5535.
 * Use for bounded 0–1 ratios (dominance, adoption, liveliness, etc.).
 *
 * @typedef {number} BasisPoints16
 */
/**
 * Unsigned basis points stored as u32.
 * 1 bp = 0.0001. Range: 0–429,496.7295.
 * Use for unbounded unsigned ratios (MVRV, NVT, SOPR, etc.).
 *
 * @typedef {number} BasisPoints32
 */
/**
 * Signed basis points stored as i16.
 * 1 bp = 0.0001. Range: -3.2767 to +3.2767.
 * Use for signed bounded ratios (NUPL, net PnL ratios, etc.).
 *
 * @typedef {number} BasisPointsSigned16
 */
/**
 * Signed basis points stored as i32.
 * 1 bp = 0.0001. Range: -214,748.3647 to +214,748.3647.
 * Use for unbounded signed values (returns, growth rates, volatility, z-scores, etc.).
 *
 * @typedef {number} BasisPointsSigned32
 */
/**
 * Bitcoin amount as floating point (1 BTC = 100,000,000 satoshis)
 *
 * @typedef {number} Bitcoin
 */
/**
 * Position within a .blk file, encoding file index and byte offset
 *
 * @typedef {number} BlkPosition
 */
/**
 * @typedef {Object} BlockCountParam
 * @property {number} blockCount - Number of recent blocks to include
 */
/**
 * A single block fees data point.
 *
 * @typedef {Object} BlockFeesEntry
 * @property {Height} avgHeight
 * @property {Timestamp} timestamp
 * @property {Sats} avgFees
 */
/**
 * Block hash
 *
 * @typedef {string} BlockHash
 */
/**
 * @typedef {Object} BlockHashParam
 * @property {BlockHash} hash
 */
/**
 * @typedef {Object} BlockHashStartIndex
 * @property {BlockHash} hash - Bitcoin block hash
 * @property {TxIndex} startIndex - Starting transaction index within the block (0-based)
 */
/**
 * @typedef {Object} BlockHashTxIndex
 * @property {BlockHash} hash - Bitcoin block hash
 * @property {TxIndex} index - Transaction index within the block (0-based)
 */
/**
 * Block information returned by the API
 *
 * @typedef {Object} BlockInfo
 * @property {BlockHash} id - Block hash
 * @property {Height} height - Block height
 * @property {number} txCount - Number of transactions in the block
 * @property {number} size - Block size in bytes
 * @property {Weight} weight - Block weight in weight units
 * @property {Timestamp} timestamp - Block timestamp (Unix time)
 * @property {number} difficulty - Block difficulty as a floating point number
 */
/**
 * A single block rewards data point.
 *
 * @typedef {Object} BlockRewardsEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgRewards
 */
/**
 * A single block size data point.
 *
 * @typedef {Object} BlockSizeEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgSize
 */
/**
 * Combined block sizes and weights response.
 *
 * @typedef {Object} BlockSizesWeights
 * @property {BlockSizeEntry[]} sizes
 * @property {BlockWeightEntry[]} weights
 */
/**
 * Block status indicating whether block is in the best chain
 *
 * @typedef {Object} BlockStatus
 * @property {boolean} inBestChain - Whether this block is in the best chain
 * @property {(Height|null)=} height - Block height (only if in best chain)
 * @property {(BlockHash|null)=} nextBest - Hash of the next block in the best chain (only if in best chain and not tip)
 */
/**
 * Block information returned for timestamp queries
 *
 * @typedef {Object} BlockTimestamp
 * @property {Height} height - Block height
 * @property {BlockHash} hash - Block hash
 * @property {string} timestamp - Block timestamp in ISO 8601 format
 */
/**
 * A single block weight data point.
 *
 * @typedef {Object} BlockWeightEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgWeight
 */
/**
 * Unsigned cents (u64) - for values that should never be negative.
 * Used for invested capital, realized cap, etc.
 *
 * @typedef {number} Cents
 */
/**
 * Cents × Sats (u128) - price in cents multiplied by amount in sats.
 * Uses u128 because large amounts at any price can overflow u64.
 *
 * @typedef {number} CentsSats
 */
/**
 * Signed cents (i64) - for values that can be negative.
 * Used for profit/loss calculations, deltas, etc.
 *
 * @typedef {number} CentsSigned
 */
/**
 * Raw cents squared (u128) - stores cents² × sats without division.
 * Used for precise accumulation of investor cap values: Σ(price² × sats).
 * investor_price = investor_cap_raw / realized_cap_raw
 *
 * @typedef {number} CentsSquaredSats
 */
/**
 * Closing price value for a time period
 *
 * @typedef {Cents} Close
 */
/**
 * Cohort identifier for cost basis distribution.
 *
 * @typedef {string} Cohort
 */
/**
 * Bucket type for cost basis aggregation.
 * Options: raw (no aggregation), lin200/lin500/lin1000 (linear $200/$500/$1000),
 * log10/log50/log100/log200 (logarithmic with 10/50/100/200 buckets per decade).
 *
 * @typedef {("raw"|"lin200"|"lin500"|"lin1000"|"log10"|"log50"|"log100"|"log200")} CostBasisBucket
 */
/**
 * Path parameters for cost basis dates endpoint.
 *
 * @typedef {Object} CostBasisCohortParam
 * @property {Cohort} cohort
 */
/**
 * Path parameters for cost basis distribution endpoint.
 *
 * @typedef {Object} CostBasisParams
 * @property {Cohort} cohort
 * @property {string} date
 */
/**
 * Query parameters for cost basis distribution endpoint.
 *
 * @typedef {Object} CostBasisQuery
 * @property {CostBasisBucket=} bucket - Bucket type for aggregation. Default: raw (no aggregation).
 * @property {CostBasisValue=} value - Value type to return. Default: supply.
 */
/**
 * Value type for cost basis distribution.
 * Options: supply (BTC), realized (USD, price × supply), unrealized (USD, spot × supply).
 *
 * @typedef {("supply"|"realized"|"unrealized")} CostBasisValue
 */
/**
 * Data range with output format for API query parameters
 *
 * @typedef {Object} DataRangeFormat
 * @property {?number=} start - Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
 * @property {?number=} end - Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
 * @property {Format=} format - Format of the output
 */
/**
 * Date in YYYYMMDD format stored as u32
 *
 * @typedef {number} Date
 */
/** @typedef {number} Day1 */
/** @typedef {number} Day3 */
/**
 * Difficulty adjustment information.
 *
 * @typedef {Object} DifficultyAdjustment
 * @property {number} progressPercent - Progress through current difficulty epoch (0-100%)
 * @property {number} difficultyChange - Estimated difficulty change at next retarget (%)
 * @property {number} estimatedRetargetDate - Estimated Unix timestamp of next retarget
 * @property {number} remainingBlocks - Blocks remaining until retarget
 * @property {number} remainingTime - Estimated seconds until retarget
 * @property {number} previousRetarget - Previous difficulty adjustment (%)
 * @property {Height} nextRetargetHeight - Height of next retarget
 * @property {number} timeAvg - Average block time in current epoch (seconds)
 * @property {number} adjustedTimeAvg - Time-adjusted average (accounting for timestamp manipulation)
 * @property {number} timeOffset - Time offset from expected schedule (seconds)
 */
/**
 * A single difficulty adjustment entry.
 * Serializes as array: [timestamp, height, difficulty, change_percent]
 *
 * @typedef {Object} DifficultyAdjustmentEntry
 * @property {Timestamp} timestamp
 * @property {Height} height
 * @property {number} difficulty
 * @property {number} changePercent
 */
/**
 * A single difficulty data point.
 *
 * @typedef {Object} DifficultyEntry
 * @property {Timestamp} timestamp - Unix timestamp of the difficulty adjustment.
 * @property {number} difficulty - Difficulty value.
 * @property {Height} height - Block height of the adjustment.
 */
/** @typedef {number} DifficultyEpoch */
/**
 * Disk usage of the indexed data
 *
 * @typedef {Object} DiskUsage
 * @property {string} brk - Human-readable brk data size (e.g., "48.8 GiB")
 * @property {number} brkBytes - brk data size in bytes
 * @property {string} bitcoin - Human-readable Bitcoin blocks directory size
 * @property {number} bitcoinBytes - Bitcoin blocks directory size in bytes
 * @property {number} ratio - brk as percentage of Bitcoin data
 */
/**
 * US Dollar amount as floating point
 *
 * @typedef {number} Dollars
 */
/**
 * Data of an empty address
 *
 * @typedef {Object} EmptyAddressData
 * @property {number} txCount - Total transaction count
 * @property {number} fundedTxoCount - Total funded/spent transaction output count (equal since address is empty)
 * @property {Sats} transfered - Total satoshis transferred
 */
/** @typedef {TypeIndex} EmptyAddressIndex */
/** @typedef {TypeIndex} EmptyOutputIndex */
/**
 * Fee rate in sats/vB
 *
 * @typedef {number} FeeRate
 */
/**
 * Output format for API responses
 *
 * @typedef {("json"|"csv")} Format
 */
/**
 * Data for a funded (non-empty) address with current balance
 *
 * @typedef {Object} FundedAddressData
 * @property {number} txCount - Total transaction count
 * @property {number} fundedTxoCount - Number of transaction outputs funded to this address
 * @property {number} spentTxoCount - Number of transaction outputs spent by this address
 * @property {Sats} received - Satoshis received by this address
 * @property {Sats} sent - Satoshis sent by this address
 * @property {CentsSats} realizedCapRaw - The realized capitalization: Σ(price × sats)
 * @property {CentsSquaredSats} investorCapRaw - The investor capitalization: Σ(price² × sats)
 */
/** @typedef {TypeIndex} FundedAddressIndex */
/** @typedef {number} HalvingEpoch */
/**
 * A single hashrate data point.
 *
 * @typedef {Object} HashrateEntry
 * @property {Timestamp} timestamp - Unix timestamp.
 * @property {number} avgHashrate - Average hashrate (H/s).
 */
/**
 * Summary of network hashrate and difficulty data.
 *
 * @typedef {Object} HashrateSummary
 * @property {HashrateEntry[]} hashrates - Historical hashrate data points.
 * @property {DifficultyEntry[]} difficulty - Historical difficulty adjustments.
 * @property {number} currentHashrate - Current network hashrate (H/s).
 * @property {number} currentDifficulty - Current network difficulty.
 */
/**
 * Server health status
 *
 * @typedef {Object} Health
 * @property {string} status
 * @property {string} service
 * @property {string} timestamp
 * @property {string} startedAt - Server start time (ISO 8601)
 * @property {number} uptimeSeconds - Uptime in seconds
 */
/**
 * Block height
 *
 * @typedef {number} Height
 */
/**
 * @typedef {Object} HeightParam
 * @property {Height} height
 */
/**
 * Hex-encoded string
 *
 * @typedef {string} Hex
 */
/**
 * Highest price value for a time period
 *
 * @typedef {Cents} High
 */
/** @typedef {number} Hour1 */
/** @typedef {number} Hour12 */
/** @typedef {number} Hour4 */
/**
 * Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
 * block-based (height, txindex), and address/output type indexes.
 *
 * @typedef {("minute10"|"minute30"|"hour1"|"hour4"|"hour12"|"day1"|"day3"|"week1"|"month1"|"month3"|"month6"|"year1"|"year10"|"halvingepoch"|"difficultyepoch"|"height"|"txindex"|"txinindex"|"txoutindex"|"emptyoutputindex"|"opreturnindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"unknownoutputindex"|"fundedaddressindex"|"emptyaddressindex"|"pairoutputindex")} Index
 */
/**
 * Information about an available index and its query aliases
 *
 * @typedef {Object} IndexInfo
 * @property {Index} index - The canonical index name
 * @property {string[]} aliases - All Accepted query aliases
 */
/**
 * Maximum number of results to return. Defaults to 100 if not specified.
 *
 * @typedef {number} Limit
 */
/**
 * @typedef {Object} LimitParam
 * @property {Limit=} limit
 */
/**
 * Lowest price value for a time period
 *
 * @typedef {Cents} Low
 */
/**
 * Block info in a mempool.space like format for fee estimation.
 *
 * @typedef {Object} MempoolBlock
 * @property {number} blockSize - Total block size in weight units
 * @property {number} blockVSize - Total block virtual size in vbytes
 * @property {number} nTx - Number of transactions in the projected block
 * @property {Sats} totalFees - Total fees in satoshis
 * @property {FeeRate} medianFee - Median fee rate in sat/vB
 * @property {FeeRate[]} feeRange - Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]
 */
/**
 * Mempool statistics
 *
 * @typedef {Object} MempoolInfo
 * @property {number} count - Number of transactions in the mempool
 * @property {VSize} vsize - Total virtual size of all transactions in the mempool (vbytes)
 * @property {Sats} totalFee - Total fees of all transactions in the mempool (satoshis)
 */
/**
 * Metric name
 *
 * @typedef {string} Metric
 */
/**
 * Metric count statistics - distinct metrics and total metric-index combinations
 *
 * @typedef {Object} MetricCount
 * @property {number} distinctMetrics - Number of unique metrics available (e.g., realized_price, market_cap)
 * @property {number} totalEndpoints - Total number of metric-index combinations across all timeframes
 * @property {number} lazyEndpoints - Number of lazy (computed on-the-fly) metric-index combinations
 * @property {number} storedEndpoints - Number of eager (stored on disk) metric-index combinations
 */
/**
 * MetricLeaf with JSON Schema for client generation
 *
 * @typedef {Object} MetricLeafWithSchema
 * @property {string} name - The metric name/identifier
 * @property {string} kind - The Rust type (e.g., "Sats", "StoredF64")
 * @property {Index[]} indexes - Available indexes for this metric
 * @property {string} type - JSON Schema type (e.g., "integer", "number", "string", "boolean", "array", "object")
 */
/**
 * @typedef {Object} MetricParam
 * @property {Metric} metric
 */
/**
 * Selection of metrics to query
 *
 * @typedef {Object} MetricSelection
 * @property {Metrics} metrics - Requested metrics
 * @property {Index} index - Index to query
 * @property {?number=} start - Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
 * @property {?number=} end - Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
 * @property {Format=} format - Format of the output
 */
/**
 * Legacy metric selection parameters (deprecated)
 *
 * @typedef {Object} MetricSelectionLegacy
 * @property {Index} index
 * @property {Metrics} ids
 * @property {?number=} start - Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
 * @property {?number=} end - Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
 * @property {Format=} format - Format of the output
 */
/**
 * @typedef {Object} MetricWithIndex
 * @property {Metric} metric - Metric name
 * @property {Index} index - Aggregation index
 */
/**
 * Comma-separated list of metric names
 *
 * @typedef {string} Metrics
 */
/** @typedef {number} Minute10 */
/** @typedef {number} Minute30 */
/** @typedef {number} Month1 */
/** @typedef {number} Month3 */
/** @typedef {number} Month6 */
/**
 * OHLC (Open, High, Low, Close) data in cents
 *
 * @typedef {Object} OHLCCents
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * OHLC (Open, High, Low, Close) data in dollars
 *
 * @typedef {Object} OHLCDollars
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * OHLC (Open, High, Low, Close) data in satoshis
 *
 * @typedef {Object} OHLCSats
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/** @typedef {TypeIndex} OpReturnIndex */
/**
 * Opening price value for a time period
 *
 * @typedef {Cents} Open
 */
/** @typedef {number} OutPoint */
/**
 * Type (P2PKH, P2WPKH, P2SH, P2TR, etc.)
 *
 * @typedef {("p2pk65"|"p2pk33"|"p2pkh"|"p2ms"|"p2sh"|"opreturn"|"p2wpkh"|"p2wsh"|"p2tr"|"p2a"|"empty"|"unknown")} OutputType
 */
/** @typedef {TypeIndex} P2AAddressIndex */
/** @typedef {U8x2} P2ABytes */
/** @typedef {TypeIndex} P2MSOutputIndex */
/** @typedef {TypeIndex} P2PK33AddressIndex */
/** @typedef {U8x33} P2PK33Bytes */
/** @typedef {TypeIndex} P2PK65AddressIndex */
/** @typedef {U8x65} P2PK65Bytes */
/** @typedef {TypeIndex} P2PKHAddressIndex */
/** @typedef {U8x20} P2PKHBytes */
/** @typedef {TypeIndex} P2SHAddressIndex */
/** @typedef {U8x20} P2SHBytes */
/** @typedef {TypeIndex} P2TRAddressIndex */
/** @typedef {U8x32} P2TRBytes */
/** @typedef {TypeIndex} P2WPKHAddressIndex */
/** @typedef {U8x20} P2WPKHBytes */
/** @typedef {TypeIndex} P2WSHAddressIndex */
/** @typedef {U8x32} P2WSHBytes */
/**
 * A paginated list of available metric names (1000 per page)
 *
 * @typedef {Object} PaginatedMetrics
 * @property {number} currentPage - Current page number (0-indexed)
 * @property {number} maxPage - Maximum valid page index (0-indexed)
 * @property {string[]} metrics - List of metric names (max 1000 per page)
 */
/**
 * Pagination parameters for paginated API endpoints
 *
 * @typedef {Object} Pagination
 * @property {?number=} page - Pagination index
 */
/**
 * Block counts for different time periods
 *
 * @typedef {Object} PoolBlockCounts
 * @property {number} all - Total blocks mined (all time)
 * @property {number} _24h - Blocks mined in last 24 hours
 * @property {number} _1w - Blocks mined in last week
 */
/**
 * Pool's share of total blocks for different time periods
 *
 * @typedef {Object} PoolBlockShares
 * @property {number} all - Share of all blocks (0.0 - 1.0)
 * @property {number} _24h - Share of blocks in last 24 hours
 * @property {number} _1w - Share of blocks in last week
 */
/**
 * Detailed pool information with statistics across time periods
 *
 * @typedef {Object} PoolDetail
 * @property {PoolDetailInfo} pool - Pool information
 * @property {PoolBlockCounts} blockCount - Block counts for different time periods
 * @property {PoolBlockShares} blockShare - Pool's share of total blocks for different time periods
 * @property {number} estimatedHashrate - Estimated hashrate based on blocks mined
 * @property {?number=} reportedHashrate - Self-reported hashrate (if available)
 */
/**
 * Pool information for detail view
 *
 * @typedef {Object} PoolDetailInfo
 * @property {number} id - Unique pool identifier
 * @property {string} name - Pool name
 * @property {string} link - Pool website URL
 * @property {string[]} addresses - Known payout addresses
 * @property {string[]} regexes - Coinbase tag patterns (regexes)
 * @property {PoolSlug} slug - URL-friendly pool identifier
 */
/**
 * Basic pool information for listing all pools
 *
 * @typedef {Object} PoolInfo
 * @property {string} name - Pool name
 * @property {PoolSlug} slug - URL-friendly pool identifier
 * @property {number} uniqueId - Unique numeric pool identifier
 */
/** @typedef {("unknown"|"blockfills"|"ultimuspool"|"terrapool"|"luxor"|"onethash"|"btccom"|"bitfarms"|"huobipool"|"wayicn"|"canoepool"|"btctop"|"bitcoincom"|"pool175btc"|"gbminers"|"axbt"|"asicminer"|"bitminter"|"bitcoinrussia"|"btcserv"|"simplecoinus"|"btcguild"|"eligius"|"ozcoin"|"eclipsemc"|"maxbtc"|"triplemining"|"coinlab"|"pool50btc"|"ghashio"|"stminingcorp"|"bitparking"|"mmpool"|"polmine"|"kncminer"|"bitalo"|"f2pool"|"hhtt"|"megabigpower"|"mtred"|"nmcbit"|"yourbtcnet"|"givemecoins"|"braiinspool"|"antpool"|"multicoinco"|"bcpoolio"|"cointerra"|"kanopool"|"solock"|"ckpool"|"nicehash"|"bitclub"|"bitcoinaffiliatenetwork"|"btcc"|"bwpool"|"exxbw"|"bitsolo"|"bitfury"|"twentyoneinc"|"digitalbtc"|"eightbaochi"|"mybtccoinpool"|"tbdice"|"hashpool"|"nexious"|"bravomining"|"hotpool"|"okexpool"|"bcmonster"|"onehash"|"bixin"|"tatmaspool"|"viabtc"|"connectbtc"|"batpool"|"waterhole"|"dcexploration"|"dcex"|"btpool"|"fiftyeightcoin"|"bitcoinindia"|"shawnp0wers"|"phashio"|"rigpool"|"haozhuzhu"|"sevenpool"|"miningkings"|"hashbx"|"dpool"|"rawpool"|"haominer"|"helix"|"bitcoinukraine"|"poolin"|"secretsuperstar"|"tigerpoolnet"|"sigmapoolcom"|"okpooltop"|"hummerpool"|"tangpool"|"bytepool"|"spiderpool"|"novablock"|"miningcity"|"binancepool"|"minerium"|"lubiancom"|"okkong"|"aaopool"|"emcdpool"|"foundryusa"|"sbicrypto"|"arkpool"|"purebtccom"|"marapool"|"kucoinpool"|"entrustcharitypool"|"okminer"|"titan"|"pegapool"|"btcnuggets"|"cloudhashing"|"digitalxmintsy"|"telco214"|"btcpoolparty"|"multipool"|"transactioncoinmining"|"btcdig"|"trickysbtcpool"|"btcmp"|"eobot"|"unomp"|"patels"|"gogreenlight"|"bitcoinindiapool"|"ekanembtc"|"canoe"|"tiger"|"onem1x"|"zulupool"|"secpool"|"ocean"|"whitepool"|"wiz"|"wk057"|"futurebitapollosolo"|"carbonnegative"|"portlandhodl"|"phoenix"|"neopool"|"maxipool"|"bitfufupool"|"gdpool"|"miningdutch"|"publicpool"|"miningsquared"|"innopolistech"|"btclab"|"parasite"|"redrockpool"|"est3lar")} PoolSlug */
/**
 * @typedef {Object} PoolSlugParam
 * @property {PoolSlug} slug
 */
/**
 * Mining pool with block statistics for a time period
 *
 * @typedef {Object} PoolStats
 * @property {number} poolId - Unique pool identifier
 * @property {string} name - Pool name
 * @property {string} link - Pool website URL
 * @property {number} blockCount - Number of blocks mined in the time period
 * @property {number} rank - Pool ranking by block count (1 = most blocks)
 * @property {number} emptyBlocks - Number of empty blocks mined
 * @property {PoolSlug} slug - URL-friendly pool identifier
 * @property {number} share - Pool's share of total blocks (0.0 - 1.0)
 */
/**
 * Mining pools response for a time period
 *
 * @typedef {Object} PoolsSummary
 * @property {PoolStats[]} pools - List of pools sorted by block count descending
 * @property {number} blockCount - Total blocks in the time period
 * @property {number} lastEstimatedHashrate - Estimated network hashrate (hashes per second)
 */
/**
 * Transaction locktime
 *
 * @typedef {number} RawLockTime
 */
/**
 * Recommended fee rates in sat/vB
 *
 * @typedef {Object} RecommendedFees
 * @property {FeeRate} fastestFee - Fee rate for fastest confirmation (next block)
 * @property {FeeRate} halfHourFee - Fee rate for confirmation within ~30 minutes (3 blocks)
 * @property {FeeRate} hourFee - Fee rate for confirmation within ~1 hour (6 blocks)
 * @property {FeeRate} economyFee - Fee rate for economical confirmation
 * @property {FeeRate} minimumFee - Minimum relay fee rate
 */
/**
 * Block reward statistics over a range of blocks
 *
 * @typedef {Object} RewardStats
 * @property {Height} startBlock - First block in the range
 * @property {Height} endBlock - Last block in the range
 * @property {Sats} totalReward
 * @property {Sats} totalFee
 * @property {number} totalTx
 */
/**
 * Satoshis
 *
 * @typedef {number} Sats
 */
/**
 * Fractional satoshis (f64) - for representing USD prices in sats
 *
 * Formula: `sats_fract = usd_value * 100_000_000 / btc_price`
 *
 * When BTC is $100,000:
 * - $1 = 1,000 sats
 * - $0.001 = 1 sat
 * - $0.0001 = 0.1 sats (fractional)
 *
 * @typedef {number} SatsFract
 */
/**
 * Signed satoshis (i64) - for values that can be negative.
 * Used for changes, deltas, profit/loss calculations, etc.
 *
 * @typedef {number} SatsSigned
 */
/**
 * Fixed-size boolean value optimized for on-disk storage (stored as u8)
 *
 * @typedef {number} StoredBool
 */
/**
 * Stored 32-bit floating point value
 *
 * @typedef {number} StoredF32
 */
/**
 * Fixed-size 64-bit floating point value optimized for on-disk storage
 *
 * @typedef {number} StoredF64
 */
/** @typedef {number} StoredI8 */
/** @typedef {number} StoredU16 */
/**
 * Fixed-size 32-bit unsigned integer optimized for on-disk storage
 *
 * @typedef {number} StoredU32
 */
/**
 * Fixed-size 64-bit unsigned integer optimized for on-disk storage
 *
 * @typedef {number} StoredU64
 */
/**
 * Current supply state tracking UTXO count and total value
 *
 * @typedef {Object} SupplyState
 * @property {number} utxoCount - Number of unspent transaction outputs
 * @property {Sats} value - Total value in satoshis
 */
/**
 * Sync status of the indexer
 *
 * @typedef {Object} SyncStatus
 * @property {Height} indexedHeight - Height of the last indexed block
 * @property {Height} tipHeight - Height of the chain tip (from Bitcoin node)
 * @property {Height} blocksBehind - Number of blocks behind the tip
 * @property {string} lastIndexedAt - Human-readable timestamp of the last indexed block (ISO 8601)
 * @property {Timestamp} lastIndexedAtUnix - Unix timestamp of the last indexed block
 */
/**
 * Time period for mining statistics.
 *
 * Used to specify the lookback window for pool statistics, hashrate calculations,
 * and other time-based mining metrics.
 *
 * @typedef {("24h"|"3d"|"1w"|"1m"|"3m"|"6m"|"1y"|"2y"|"3y")} TimePeriod
 */
/**
 * @typedef {Object} TimePeriodParam
 * @property {TimePeriod} timePeriod
 */
/**
 * UNIX timestamp in seconds
 *
 * @typedef {number} Timestamp
 */
/**
 * @typedef {Object} TimestampParam
 * @property {Timestamp} timestamp
 */
/**
 * Transaction information compatible with mempool.space API format
 *
 * @typedef {Object} Transaction
 * @property {(TxIndex|null)=} index
 * @property {Txid} txid
 * @property {TxVersion} version
 * @property {RawLockTime} locktime
 * @property {number} size - Transaction size in bytes
 * @property {Weight} weight - Transaction weight
 * @property {number} sigops - Number of signature operations
 * @property {Sats} fee - Transaction fee in satoshis
 * @property {TxIn[]} vin - Transaction inputs
 * @property {TxOut[]} vout - Transaction outputs
 * @property {TxStatus} status
 */
/**
 * Hierarchical tree node for organizing metrics into categories
 *
 * @typedef {({ [key: string]: TreeNode }|MetricLeafWithSchema)} TreeNode
 */
/**
 * Transaction input
 *
 * @typedef {Object} TxIn
 * @property {Txid} txid - Transaction ID of the output being spent
 * @property {Vout} vout
 * @property {(TxOut|null)=} prevout - Information about the previous output being spent
 * @property {string} scriptsig - Signature script (for non-SegWit inputs)
 * @property {string} scriptsigAsm - Signature script in assembly format
 * @property {boolean} isCoinbase - Whether this input is a coinbase (block reward) input
 * @property {number} sequence - Input sequence number
 * @property {?string=} innerRedeemscriptAsm - Inner redeemscript in assembly format (for P2SH-wrapped SegWit)
 */
/** @typedef {number} TxInIndex */
/** @typedef {number} TxIndex */
/**
 * Transaction output
 *
 * @typedef {Object} TxOut
 * @property {string} scriptpubkey - Script pubkey (locking script)
 * @property {Sats} value - Value of the output in satoshis
 */
/** @typedef {number} TxOutIndex */
/**
 * Status of an output indicating whether it has been spent
 *
 * @typedef {Object} TxOutspend
 * @property {boolean} spent - Whether the output has been spent
 * @property {(Txid|null)=} txid - Transaction ID of the spending transaction (only present if spent)
 * @property {(Vin|null)=} vin - Input index in the spending transaction (only present if spent)
 * @property {(TxStatus|null)=} status - Status of the spending transaction (only present if spent)
 */
/**
 * Transaction confirmation status
 *
 * @typedef {Object} TxStatus
 * @property {boolean} confirmed - Whether the transaction is confirmed
 * @property {(Height|null)=} blockHeight - Block height (only present if confirmed)
 * @property {(BlockHash|null)=} blockHash - Block hash (only present if confirmed)
 * @property {(Timestamp|null)=} blockTime - Block timestamp (only present if confirmed)
 */
/**
 * Transaction version number
 *
 * @typedef {number} TxVersion
 */
/**
 * Transaction ID (hash)
 *
 * @typedef {string} Txid
 */
/**
 * @typedef {Object} TxidParam
 * @property {Txid} txid
 */
/**
 * Transaction output reference (txid + output index)
 *
 * @typedef {Object} TxidVout
 * @property {Txid} txid - Transaction ID
 * @property {Vout} vout - Output index
 */
/**
 * Index within its type (e.g., 0 for first P2WPKH address)
 *
 * @typedef {number} TypeIndex
 */
/** @typedef {number[]} U8x2 */
/** @typedef {number[]} U8x20 */
/** @typedef {number[]} U8x32 */
/** @typedef {string} U8x33 */
/** @typedef {string} U8x65 */
/** @typedef {TypeIndex} UnknownOutputIndex */
/**
 * Unspent transaction output
 *
 * @typedef {Object} Utxo
 * @property {Txid} txid
 * @property {Vout} vout
 * @property {TxStatus} status
 * @property {Sats} value
 */
/**
 * Virtual size in vbytes (weight / 4, rounded up)
 *
 * @typedef {number} VSize
 */
/**
 * @typedef {Object} ValidateAddressParam
 * @property {string} address - Bitcoin address to validate (can be any string)
 */
/**
 * Version tracking for data schema and computed values.
 *
 * Used to detect when stored data needs to be recomputed due to changes
 * in computation logic or source data versions. Supports validation
 * against persisted versions to ensure compatibility.
 *
 * @typedef {number} Version
 */
/**
 * Input index in the spending transaction
 *
 * @typedef {number} Vin
 */
/**
 * Index of the output being spent in the previous transaction
 *
 * @typedef {number} Vout
 */
/** @typedef {number} Week1 */
/**
 * Transaction or block weight in weight units (WU)
 *
 * @typedef {number} Weight
 */
/** @typedef {number} Year1 */
/** @typedef {number} Year10 */

/**
 * @typedef {Object} BrkClientOptions
 * @property {string} baseUrl - Base URL for the API
 * @property {number} [timeout] - Request timeout in milliseconds
 * @property {string|boolean} [cache] - Enable browser cache with default name (true), custom name (string), or disable (false). No effect in Node.js. Default: true
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (/** @type {VoidFunction} */ fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);
const _defaultCacheName = '__BRK_CLIENT__';

/**
 * @param {string|boolean|undefined} cache
 * @returns {Promise<Cache | null>}
 */
const _openCache = (cache) => {
  if (!_isBrowser || cache === false) return Promise.resolve(null);
  const name = typeof cache === 'string' ? cache : _defaultCacheName;
  return caches.open(name).catch(() => null);
};

/**
 * Custom error class for BRK client errors
 */
class BrkError extends Error {
  /**
   * @param {string} message
   * @param {number} [status]
   */
  constructor(message, status) {
    super(message);
    this.name = 'BrkError';
    this.status = status;
  }
}

// Date conversion constants and helpers
const _GENESIS = new Date(2009, 0, 3);  // day1 0, week1 0
const _DAY_ONE = new Date(2009, 0, 9);  // day1 1 (6 day gap after genesis)
const _MS_PER_DAY = 86400000;
const _MS_PER_WEEK = 7 * _MS_PER_DAY;
const _EPOCH_MS = 1230768000000;
const _DATE_INDEXES = new Set([
  'minute10', 'minute30',
  'hour1', 'hour4', 'hour12',
  'day1', 'day3', 'week1',
  'month1', 'month3', 'month6',
  'year1', 'year10',
]);

/** @param {number} months @returns {globalThis.Date} */
const _addMonths = (months) => new Date(2009, months, 1);

/**
 * Convert an index value to a Date for date-based indexes.
 * @param {Index} index - The index type
 * @param {number} i - The index value
 * @returns {globalThis.Date}
 */
function indexToDate(index, i) {
  switch (index) {
    case 'minute10': return new Date(_EPOCH_MS + i * 600000);
    case 'minute30': return new Date(_EPOCH_MS + i * 1800000);
    case 'hour1': return new Date(_EPOCH_MS + i * 3600000);
    case 'hour4': return new Date(_EPOCH_MS + i * 14400000);
    case 'hour12': return new Date(_EPOCH_MS + i * 43200000);
    case 'day1': return i === 0 ? _GENESIS : new Date(_DAY_ONE.getTime() + (i - 1) * _MS_PER_DAY);
    case 'day3': return new Date(_EPOCH_MS - 86400000 + i * 259200000);
    case 'week1': return new Date(_GENESIS.getTime() + i * _MS_PER_WEEK);
    case 'month1': return _addMonths(i);
    case 'month3': return _addMonths(i * 3);
    case 'month6': return _addMonths(i * 6);
    case 'year1': return new Date(2009 + i, 0, 1);
    case 'year10': return new Date(2009 + i * 10, 0, 1);
    default: throw new Error(`${index} is not a date-based index`);
  }
}

/**
 * Convert a Date to an index value for date-based indexes.
 * Returns the floor index (latest index whose date is <= the given date).
 * @param {Index} index - The index type
 * @param {globalThis.Date} d - The date to convert
 * @returns {number}
 */
function dateToIndex(index, d) {
  const ms = d.getTime();
  switch (index) {
    case 'minute10': return Math.floor((ms - _EPOCH_MS) / 600000);
    case 'minute30': return Math.floor((ms - _EPOCH_MS) / 1800000);
    case 'hour1': return Math.floor((ms - _EPOCH_MS) / 3600000);
    case 'hour4': return Math.floor((ms - _EPOCH_MS) / 14400000);
    case 'hour12': return Math.floor((ms - _EPOCH_MS) / 43200000);
    case 'day1': {
      if (ms < _DAY_ONE.getTime()) return 0;
      return 1 + Math.floor((ms - _DAY_ONE.getTime()) / _MS_PER_DAY);
    }
    case 'day3': return Math.floor((ms - _EPOCH_MS + 86400000) / 259200000);
    case 'week1': return Math.floor((ms - _GENESIS.getTime()) / _MS_PER_WEEK);
    case 'month1': return (d.getFullYear() - 2009) * 12 + d.getMonth();
    case 'month3': return (d.getFullYear() - 2009) * 4 + Math.floor(d.getMonth() / 3);
    case 'month6': return (d.getFullYear() - 2009) * 2 + Math.floor(d.getMonth() / 6);
    case 'year1': return d.getFullYear() - 2009;
    case 'year10': return Math.floor((d.getFullYear() - 2009) / 10);
    default: throw new Error(`${index} is not a date-based index`);
  }
}

/**
 * Wrap raw metric data with helper methods.
 * @template T
 * @param {MetricData<T>} raw - Raw JSON response
 * @returns {DateMetricData<T>}
 */
function _wrapMetricData(raw) {
  const { index, start, end, data } = raw;
  const _dateBased = _DATE_INDEXES.has(index);
  return /** @type {DateMetricData<T>} */ ({
    ...raw,
    isDateBased: _dateBased,
    indexes() {
      /** @type {number[]} */
      const result = [];
      for (let i = start; i < end; i++) result.push(i);
      return result;
    },
    keys() {
      return this.indexes();
    },
    entries() {
      /** @type {Array<[number, T]>} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([start + i, data[i]]);
      return result;
    },
    toMap() {
      /** @type {Map<number, T>} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(start + i, data[i]);
      return map;
    },
    *[Symbol.iterator]() {
      for (let i = 0; i < data.length; i++) yield /** @type {[number, T]} */ ([start + i, data[i]]);
    },
    // DateMetricData methods (only meaningful for date-based indexes)
    dates() {
      /** @type {globalThis.Date[]} */
      const result = [];
      for (let i = start; i < end; i++) result.push(indexToDate(index, i));
      return result;
    },
    dateEntries() {
      /** @type {Array<[globalThis.Date, T]>} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([indexToDate(index, start + i), data[i]]);
      return result;
    },
    toDateMap() {
      /** @type {Map<globalThis.Date, T>} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(indexToDate(index, start + i), data[i]);
      return map;
    },
  });
}

/**
 * @template T
 * @typedef {Object} MetricDataBase
 * @property {number} version - Version of the metric data
 * @property {Index} index - The index type used for this query
 * @property {number} total - Total number of data points
 * @property {number} start - Start index (inclusive)
 * @property {number} end - End index (exclusive)
 * @property {string} stamp - ISO 8601 timestamp of when the response was generated
 * @property {T[]} data - The metric data
 * @property {boolean} isDateBased - Whether this metric uses a date-based index
 * @property {() => number[]} indexes - Get index numbers
 * @property {() => number[]} keys - Get keys as index numbers (alias for indexes)
 * @property {() => Array<[number, T]>} entries - Get [index, value] pairs
 * @property {() => Map<number, T>} toMap - Convert to Map<index, value>
 */

/** @template T @typedef {MetricDataBase<T> & Iterable<[number, T]>} MetricData */

/**
 * @template T
 * @typedef {Object} DateMetricDataExtras
 * @property {() => globalThis.Date[]} dates - Get dates for each data point
 * @property {() => Array<[globalThis.Date, T]>} dateEntries - Get [date, value] pairs
 * @property {() => Map<globalThis.Date, T>} toDateMap - Convert to Map<date, value>
 */

/** @template T @typedef {MetricData<T> & DateMetricDataExtras<T>} DateMetricData */
/** @typedef {MetricData<any>} AnyMetricData */

/** @template T @typedef {(onfulfilled?: (value: MetricData<T>) => any, onrejected?: (reason: Error) => never) => Promise<MetricData<T>>} Thenable */
/** @template T @typedef {(onfulfilled?: (value: DateMetricData<T>) => any, onrejected?: (reason: Error) => never) => Promise<DateMetricData<T>>} DateThenable */

/**
 * @template T
 * @typedef {Object} MetricEndpointBuilder
 * @property {(index: number) => SingleItemBuilder<T>} get - Get single item at index
 * @property {(start?: number, end?: number) => RangeBuilder<T>} slice - Slice by index
 * @property {(n: number) => RangeBuilder<T>} first - Get first n items
 * @property {(n: number) => RangeBuilder<T>} last - Get last n items
 * @property {(n: number) => SkippedBuilder<T>} skip - Skip first n items, chain with take()
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch all data
 * @property {() => Promise<string>} fetchCsv - Fetch all data as CSV
 * @property {Thenable<T>} then - Thenable (await endpoint)
 * @property {string} path - The endpoint path
 */

/**
 * @template T
 * @typedef {Object} DateMetricEndpointBuilder
 * @property {(index: number | globalThis.Date) => DateSingleItemBuilder<T>} get - Get single item at index or Date
 * @property {(start?: number | globalThis.Date, end?: number | globalThis.Date) => DateRangeBuilder<T>} slice - Slice by index or Date
 * @property {(n: number) => DateRangeBuilder<T>} first - Get first n items
 * @property {(n: number) => DateRangeBuilder<T>} last - Get last n items
 * @property {(n: number) => DateSkippedBuilder<T>} skip - Skip first n items, chain with take()
 * @property {(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>} fetch - Fetch all data
 * @property {() => Promise<string>} fetchCsv - Fetch all data as CSV
 * @property {DateThenable<T>} then - Thenable (await endpoint)
 * @property {string} path - The endpoint path
 */

/** @typedef {MetricEndpointBuilder<any>} AnyMetricEndpointBuilder */

/** @template T @typedef {Object} SingleItemBuilder
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch the item
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/** @template T @typedef {Object} DateSingleItemBuilder
 * @property {(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>} fetch - Fetch the item
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {DateThenable<T>} then - Thenable
 */

/** @template T @typedef {Object} SkippedBuilder
 * @property {(n: number) => RangeBuilder<T>} take - Take n items after skipped position
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch from skipped position to end
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/** @template T @typedef {Object} DateSkippedBuilder
 * @property {(n: number) => DateRangeBuilder<T>} take - Take n items after skipped position
 * @property {(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>} fetch - Fetch from skipped position to end
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {DateThenable<T>} then - Thenable
 */

/** @template T @typedef {Object} RangeBuilder
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch the range
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/** @template T @typedef {Object} DateRangeBuilder
 * @property {(onUpdate?: (value: DateMetricData<T>) => void) => Promise<DateMetricData<T>>} fetch - Fetch the range
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {DateThenable<T>} then - Thenable
 */

/**
 * @template T
 * @typedef {Object} MetricPattern
 * @property {string} name - The metric name
 * @property {Readonly<Partial<Record<Index, MetricEndpointBuilder<T>>>>} by - Index endpoints as lazy getters
 * @property {() => readonly Index[]} indexes - Get the list of available indexes
 * @property {(index: Index) => MetricEndpointBuilder<T>|undefined} get - Get an endpoint for a specific index
 */

/** @typedef {MetricPattern<any>} AnyMetricPattern */

/**
 * Create a metric endpoint builder with typestate pattern.
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @param {Index} index - The index name
 * @returns {DateMetricEndpointBuilder<T>}
 */
function _endpoint(client, name, index) {
  const p = `/api/metric/${name}/${index}`;

  /**
   * @param {number} [start]
   * @param {number} [end]
   * @param {string} [format]
   * @returns {string}
   */
  const buildPath = (start, end, format) => {
    const params = new URLSearchParams();
    if (start !== undefined) params.set('start', String(start));
    if (end !== undefined) params.set('end', String(end));
    if (format) params.set('format', format);
    const query = params.toString();
    return query ? `${p}?${query}` : p;
  };

  /**
   * @param {number} [start]
   * @param {number} [end]
   * @returns {DateRangeBuilder<T>}
   */
  const rangeBuilder = (start, end) => ({
    fetch(onUpdate) { return client._fetchMetricData(buildPath(start, end), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, end, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} idx
   * @returns {DateSingleItemBuilder<T>}
   */
  const singleItemBuilder = (idx) => ({
    fetch(onUpdate) { return client._fetchMetricData(buildPath(idx, idx + 1), onUpdate); },
    fetchCsv() { return client.getText(buildPath(idx, idx + 1, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} start
   * @returns {DateSkippedBuilder<T>}
   */
  const skippedBuilder = (start) => ({
    take(n) { return rangeBuilder(start, start + n); },
    fetch(onUpdate) { return client._fetchMetricData(buildPath(start, undefined), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, undefined, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /** @type {DateMetricEndpointBuilder<T>} */
  const endpoint = {
    get(idx) { if (idx instanceof Date) idx = dateToIndex(index, idx); return singleItemBuilder(idx); },
    slice(start, end) {
      if (start instanceof Date) start = dateToIndex(index, start);
      if (end instanceof Date) end = dateToIndex(index, end);
      return rangeBuilder(start, end);
    },
    first(n) { return rangeBuilder(undefined, n); },
    last(n) { return n === 0 ? rangeBuilder(undefined, 0) : rangeBuilder(-n, undefined); },
    skip(n) { return skippedBuilder(n); },
    fetch(onUpdate) { return client._fetchMetricData(buildPath(), onUpdate); },
    fetchCsv() { return client.getText(buildPath(undefined, undefined, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
    get path() { return p; },
  };

  return endpoint;
}

/**
 * Base HTTP client for making requests with caching support
 */
class BrkClientBase {
  /**
   * @param {BrkClientOptions|string} options
   */
  constructor(options) {
    const isString = typeof options === 'string';
    const rawUrl = isString ? options : options.baseUrl;
    this.baseUrl = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;
    this.timeout = isString ? 5000 : (options.timeout ?? 5000);
    /** @type {Promise<Cache | null>} */
    this._cachePromise = _openCache(isString ? undefined : options.cache);
    /** @type {Cache | null} */
    this._cache = null;
    this._cachePromise.then(c => this._cache = c);
  }

  /**
   * @param {string} path
   * @returns {Promise<Response>}
   */
  async get(path) {
    const url = `${this.baseUrl}${path}`;
    const res = await fetch(url, { signal: AbortSignal.timeout(this.timeout) });
    if (!res.ok) throw new BrkError(`HTTP ${res.status}: ${url}`, res.status);
    return res;
  }

  /**
   * Make a GET request - races cache vs network, first to resolve calls onUpdate
   * @template T
   * @param {string} path
   * @param {(value: T) => void} [onUpdate] - Called when data is available (may be called twice: cache then network)
   * @returns {Promise<T>}
   */
  async getJson(path, onUpdate) {
    const url = `${this.baseUrl}${path}`;
    const cache = this._cache ?? await this._cachePromise;

    let resolved = false;
    /** @type {Response | null} */
    let cachedRes = null;

    // Race cache vs network - first to resolve calls onUpdate
    const cachePromise = cache?.match(url).then(async (res) => {
      cachedRes = res ?? null;
      if (!res) return null;
      const json = await res.json();
      if (!resolved && onUpdate) {
        resolved = true;
        onUpdate(json);
      }
      return json;
    });

    const networkPromise = this.get(path).then(async (res) => {
      const cloned = res.clone();
      const json = await res.json();
      // Skip update if ETag matches and cache already delivered
      if (cachedRes?.headers.get('ETag') === res.headers.get('ETag')) {
        if (!resolved && onUpdate) {
          resolved = true;
          onUpdate(json);
        }
        return json;
      }
      resolved = true;
      if (onUpdate) {
        onUpdate(json);
      }
      if (cache) _runIdle(() => cache.put(url, cloned));
      return json;
    });

    try {
      return await networkPromise;
    } catch (e) {
      // Network failed - wait for cache
      const cachedJson = await cachePromise?.catch(() => null);
      if (cachedJson) return cachedJson;
      throw e;
    }
  }

  /**
   * Make a GET request and return raw text (for CSV responses)
   * @param {string} path
   * @returns {Promise<string>}
   */
  async getText(path) {
    const res = await this.get(path);
    return res.text();
  }

  /**
   * Fetch metric data and wrap with helper methods (internal)
   * @template T
   * @param {string} path
   * @param {(value: DateMetricData<T>) => void} [onUpdate]
   * @returns {Promise<DateMetricData<T>>}
   */
  async _fetchMetricData(path, onUpdate) {
    const wrappedOnUpdate = onUpdate ? (/** @type {MetricData<T>} */ raw) => onUpdate(_wrapMetricData(raw)) : undefined;
    const raw = await this.getJson(path, wrappedOnUpdate);
    return _wrapMetricData(raw);
  }
}

/**
 * Build metric name with suffix.
 * @param {string} acc - Accumulated prefix
 * @param {string} s - Metric suffix
 * @returns {string}
 */
const _m = (acc, s) => s ? (acc ? `${acc}_${s}` : s) : acc;

/**
 * Build metric name with prefix.
 * @param {string} prefix - Prefix to prepend
 * @param {string} acc - Accumulated name
 * @returns {string}
 */
const _p = (prefix, acc) => acc ? `${prefix}_${acc}` : prefix;


// Index group constants and factory

const _i1 = /** @type {const} */ (["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch", "height"]);
const _i2 = /** @type {const} */ (["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch"]);
const _i3 = /** @type {const} */ (["minute10"]);
const _i4 = /** @type {const} */ (["minute30"]);
const _i5 = /** @type {const} */ (["hour1"]);
const _i6 = /** @type {const} */ (["hour4"]);
const _i7 = /** @type {const} */ (["hour12"]);
const _i8 = /** @type {const} */ (["day1"]);
const _i9 = /** @type {const} */ (["day3"]);
const _i10 = /** @type {const} */ (["week1"]);
const _i11 = /** @type {const} */ (["month1"]);
const _i12 = /** @type {const} */ (["month3"]);
const _i13 = /** @type {const} */ (["month6"]);
const _i14 = /** @type {const} */ (["year1"]);
const _i15 = /** @type {const} */ (["year10"]);
const _i16 = /** @type {const} */ (["halvingepoch"]);
const _i17 = /** @type {const} */ (["difficultyepoch"]);
const _i18 = /** @type {const} */ (["height"]);
const _i19 = /** @type {const} */ (["txindex"]);
const _i20 = /** @type {const} */ (["txinindex"]);
const _i21 = /** @type {const} */ (["txoutindex"]);
const _i22 = /** @type {const} */ (["emptyoutputindex"]);
const _i23 = /** @type {const} */ (["opreturnindex"]);
const _i24 = /** @type {const} */ (["p2aaddressindex"]);
const _i25 = /** @type {const} */ (["p2msoutputindex"]);
const _i26 = /** @type {const} */ (["p2pk33addressindex"]);
const _i27 = /** @type {const} */ (["p2pk65addressindex"]);
const _i28 = /** @type {const} */ (["p2pkhaddressindex"]);
const _i29 = /** @type {const} */ (["p2shaddressindex"]);
const _i30 = /** @type {const} */ (["p2traddressindex"]);
const _i31 = /** @type {const} */ (["p2wpkhaddressindex"]);
const _i32 = /** @type {const} */ (["p2wshaddressindex"]);
const _i33 = /** @type {const} */ (["unknownoutputindex"]);
const _i34 = /** @type {const} */ (["fundedaddressindex"]);
const _i35 = /** @type {const} */ (["emptyaddressindex"]);

/**
 * Generic metric pattern factory.
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @param {readonly Index[]} indexes - The supported indexes
 */
function _mp(client, name, indexes) {
  const by = {};
  for (const idx of indexes) {
    Object.defineProperty(by, idx, {
      get() { return _endpoint(client, name, idx); },
      enumerable: true,
      configurable: true
    });
  }
  return {
    name,
    by,
    /** @returns {readonly Index[]} */
    indexes() { return indexes; },
    /** @param {Index} index @returns {MetricEndpointBuilder<T>|undefined} */
    get(index) { return indexes.includes(index) ? _endpoint(client, name, index) : undefined; }
  };
}

/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halvingepoch: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern1 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern1<T>} */
function createMetricPattern1(client, name) { return /** @type {MetricPattern1<T>} */ (_mp(client, name, _i1)); }
/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halvingepoch: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern2 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern2<T>} */
function createMetricPattern2(client, name) { return /** @type {MetricPattern2<T>} */ (_mp(client, name, _i2)); }
/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern3 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern3<T>} */
function createMetricPattern3(client, name) { return /** @type {MetricPattern3<T>} */ (_mp(client, name, _i3)); }
/** @template T @typedef {{ name: string, by: { readonly minute30: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern4 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern4<T>} */
function createMetricPattern4(client, name) { return /** @type {MetricPattern4<T>} */ (_mp(client, name, _i4)); }
/** @template T @typedef {{ name: string, by: { readonly hour1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern5 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern5<T>} */
function createMetricPattern5(client, name) { return /** @type {MetricPattern5<T>} */ (_mp(client, name, _i5)); }
/** @template T @typedef {{ name: string, by: { readonly hour4: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern6 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern6<T>} */
function createMetricPattern6(client, name) { return /** @type {MetricPattern6<T>} */ (_mp(client, name, _i6)); }
/** @template T @typedef {{ name: string, by: { readonly hour12: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern7 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern7<T>} */
function createMetricPattern7(client, name) { return /** @type {MetricPattern7<T>} */ (_mp(client, name, _i7)); }
/** @template T @typedef {{ name: string, by: { readonly day1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern8 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern8<T>} */
function createMetricPattern8(client, name) { return /** @type {MetricPattern8<T>} */ (_mp(client, name, _i8)); }
/** @template T @typedef {{ name: string, by: { readonly day3: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern9 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern9<T>} */
function createMetricPattern9(client, name) { return /** @type {MetricPattern9<T>} */ (_mp(client, name, _i9)); }
/** @template T @typedef {{ name: string, by: { readonly week1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern10 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern10<T>} */
function createMetricPattern10(client, name) { return /** @type {MetricPattern10<T>} */ (_mp(client, name, _i10)); }
/** @template T @typedef {{ name: string, by: { readonly month1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern11 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern11<T>} */
function createMetricPattern11(client, name) { return /** @type {MetricPattern11<T>} */ (_mp(client, name, _i11)); }
/** @template T @typedef {{ name: string, by: { readonly month3: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern12 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern12<T>} */
function createMetricPattern12(client, name) { return /** @type {MetricPattern12<T>} */ (_mp(client, name, _i12)); }
/** @template T @typedef {{ name: string, by: { readonly month6: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern13 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern13<T>} */
function createMetricPattern13(client, name) { return /** @type {MetricPattern13<T>} */ (_mp(client, name, _i13)); }
/** @template T @typedef {{ name: string, by: { readonly year1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern14 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern14<T>} */
function createMetricPattern14(client, name) { return /** @type {MetricPattern14<T>} */ (_mp(client, name, _i14)); }
/** @template T @typedef {{ name: string, by: { readonly year10: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern15 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern15<T>} */
function createMetricPattern15(client, name) { return /** @type {MetricPattern15<T>} */ (_mp(client, name, _i15)); }
/** @template T @typedef {{ name: string, by: { readonly halvingepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern16 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern16<T>} */
function createMetricPattern16(client, name) { return /** @type {MetricPattern16<T>} */ (_mp(client, name, _i16)); }
/** @template T @typedef {{ name: string, by: { readonly difficultyepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern17 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern17<T>} */
function createMetricPattern17(client, name) { return /** @type {MetricPattern17<T>} */ (_mp(client, name, _i17)); }
/** @template T @typedef {{ name: string, by: { readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern18 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern18<T>} */
function createMetricPattern18(client, name) { return /** @type {MetricPattern18<T>} */ (_mp(client, name, _i18)); }
/** @template T @typedef {{ name: string, by: { readonly txindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern19 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern19<T>} */
function createMetricPattern19(client, name) { return /** @type {MetricPattern19<T>} */ (_mp(client, name, _i19)); }
/** @template T @typedef {{ name: string, by: { readonly txinindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern20 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern20<T>} */
function createMetricPattern20(client, name) { return /** @type {MetricPattern20<T>} */ (_mp(client, name, _i20)); }
/** @template T @typedef {{ name: string, by: { readonly txoutindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern21 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern21<T>} */
function createMetricPattern21(client, name) { return /** @type {MetricPattern21<T>} */ (_mp(client, name, _i21)); }
/** @template T @typedef {{ name: string, by: { readonly emptyoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern22 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern22<T>} */
function createMetricPattern22(client, name) { return /** @type {MetricPattern22<T>} */ (_mp(client, name, _i22)); }
/** @template T @typedef {{ name: string, by: { readonly opreturnindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern23 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern23<T>} */
function createMetricPattern23(client, name) { return /** @type {MetricPattern23<T>} */ (_mp(client, name, _i23)); }
/** @template T @typedef {{ name: string, by: { readonly p2aaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern24 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern24<T>} */
function createMetricPattern24(client, name) { return /** @type {MetricPattern24<T>} */ (_mp(client, name, _i24)); }
/** @template T @typedef {{ name: string, by: { readonly p2msoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern25 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern25<T>} */
function createMetricPattern25(client, name) { return /** @type {MetricPattern25<T>} */ (_mp(client, name, _i25)); }
/** @template T @typedef {{ name: string, by: { readonly p2pk33addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern26 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern26<T>} */
function createMetricPattern26(client, name) { return /** @type {MetricPattern26<T>} */ (_mp(client, name, _i26)); }
/** @template T @typedef {{ name: string, by: { readonly p2pk65addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern27 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern27<T>} */
function createMetricPattern27(client, name) { return /** @type {MetricPattern27<T>} */ (_mp(client, name, _i27)); }
/** @template T @typedef {{ name: string, by: { readonly p2pkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern28 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern28<T>} */
function createMetricPattern28(client, name) { return /** @type {MetricPattern28<T>} */ (_mp(client, name, _i28)); }
/** @template T @typedef {{ name: string, by: { readonly p2shaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern29 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern29<T>} */
function createMetricPattern29(client, name) { return /** @type {MetricPattern29<T>} */ (_mp(client, name, _i29)); }
/** @template T @typedef {{ name: string, by: { readonly p2traddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern30 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern30<T>} */
function createMetricPattern30(client, name) { return /** @type {MetricPattern30<T>} */ (_mp(client, name, _i30)); }
/** @template T @typedef {{ name: string, by: { readonly p2wpkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern31 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern31<T>} */
function createMetricPattern31(client, name) { return /** @type {MetricPattern31<T>} */ (_mp(client, name, _i31)); }
/** @template T @typedef {{ name: string, by: { readonly p2wshaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern32 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern32<T>} */
function createMetricPattern32(client, name) { return /** @type {MetricPattern32<T>} */ (_mp(client, name, _i32)); }
/** @template T @typedef {{ name: string, by: { readonly unknownoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern33 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern33<T>} */
function createMetricPattern33(client, name) { return /** @type {MetricPattern33<T>} */ (_mp(client, name, _i33)); }
/** @template T @typedef {{ name: string, by: { readonly fundedaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern34 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern34<T>} */
function createMetricPattern34(client, name) { return /** @type {MetricPattern34<T>} */ (_mp(client, name, _i34)); }
/** @template T @typedef {{ name: string, by: { readonly emptyaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern35 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern35<T>} */
function createMetricPattern35(client, name) { return /** @type {MetricPattern35<T>} */ (_mp(client, name, _i35)); }

// Reusable structural pattern factories

/**
 * @typedef {Object} AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern
 * @property {_1m1w1y24hPattern<StoredF64>} adjustedSopr
 * @property {_1m1wPattern2} adjustedSoprEma
 * @property {MetricPattern1<Cents>} adjustedValueCreated
 * @property {_1m1w1y24hPattern<Cents>} adjustedValueCreatedSum
 * @property {MetricPattern1<Cents>} adjustedValueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} adjustedValueDestroyedSum
 * @property {MetricPattern18<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {CentsUsdPattern} grossPnl
 * @property {_1m1w1y24hPattern<Cents>} grossPnlSum
 * @property {MetricPattern18<CentsSquaredSats>} investorCapRaw
 * @property {CentsSatsUsdPattern} investorPrice
 * @property {BpsRatioPattern} investorPriceRatio
 * @property {RatioPattern} investorPriceRatioExt
 * @property {MetricPattern1<Cents>} lossValueCreated
 * @property {MetricPattern1<Cents>} lossValueDestroyed
 * @property {CentsSatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {MetricPattern1<CentsSigned>} netPnlChange1m
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToMarketCap
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToRealizedCap
 * @property {CumulativeHeightPattern<CentsSigned>} netRealizedPnl
 * @property {MetricPattern1<CentsSigned>} netRealizedPnlEma1w
 * @property {BpsPercentRatioPattern} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Cents>} peakRegret
 * @property {BpsPercentRatioPattern} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Cents>} profitValueCreated
 * @property {MetricPattern1<Cents>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<CentsSigned>} realizedCapChange1m
 * @property {BpsPercentRatioPattern} realizedCapRelToOwnMarketCap
 * @property {CumulativeHeightPattern<Cents>} realizedLoss
 * @property {MetricPattern1<Cents>} realizedLossEma1w
 * @property {BpsPercentRatioPattern} realizedLossRelToRealizedCap
 * @property {_1m1w1y24hPattern<Cents>} realizedLossSum
 * @property {CentsSatsUsdPattern} realizedPrice
 * @property {BpsRatioPattern} realizedPriceRatio
 * @property {RatioPattern} realizedPriceRatioExt
 * @property {CumulativeHeightPattern<Cents>} realizedProfit
 * @property {MetricPattern1<Cents>} realizedProfitEma1w
 * @property {BpsPercentRatioPattern} realizedProfitRelToRealizedCap
 * @property {_1m1w1y24hPattern<Cents>} realizedProfitSum
 * @property {_1m1w1y24hPattern<StoredF64>} realizedProfitToLossRatio
 * @property {_1m1w1y24hPattern2} sellSideRiskRatio
 * @property {_1m1wPattern} sellSideRiskRatio24hEma
 * @property {BaseCumulativePattern} sentInLoss
 * @property {_2wPattern} sentInLossEma
 * @property {BaseCumulativePattern} sentInProfit
 * @property {_2wPattern} sentInProfitEma
 * @property {_1m1w1y24hPattern<StoredF64>} sopr
 * @property {_1m1wPattern2} sopr24hEma
 * @property {CentsSatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Cents>} valueCreated
 * @property {_1m1w1y24hPattern<Cents>} valueCreatedSum
 * @property {MetricPattern1<Cents>} valueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} valueDestroyedSum
 */

/**
 * Create a AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern}
 */
function createAdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, acc) {
  return {
    adjustedSopr: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_sopr')),
    adjustedSoprEma: create_1m1wPattern2(client, _m(acc, 'adjusted_sopr_24h_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    adjustedValueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_value_destroyed')),
    capRaw: createMetricPattern18(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    grossPnl: createCentsUsdPattern(client, _m(acc, 'realized_gross_pnl')),
    grossPnlSum: create_1m1w1y24hPattern(client, _m(acc, 'gross_pnl_sum')),
    investorCapRaw: createMetricPattern18(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createCentsSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceRatio: createBpsRatioPattern(client, _m(acc, 'investor_price_ratio')),
    investorPriceRatioExt: createRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netPnlChange1m: createMetricPattern1(client, _m(acc, 'net_pnl_change_1m')),
    netPnlChange1mRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_market_cap')),
    netPnlChange1mRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_realized_cap')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlEma1w: createMetricPattern1(client, _m(acc, 'net_realized_pnl_ema_1w')),
    netRealizedPnlRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapChange1m: createMetricPattern1(client, _m(acc, 'realized_cap_change_1m')),
    realizedCapRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLossEma1w: createMetricPattern1(client, _m(acc, 'realized_loss_ema_1w')),
    realizedLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedLossSum: create_1m1w1y24hPattern(client, _m(acc, 'realized_loss')),
    realizedPrice: createCentsSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceRatio: createBpsRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedPriceRatioExt: createRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfitEma1w: createMetricPattern1(client, _m(acc, 'realized_profit_ema_1w')),
    realizedProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitSum: create_1m1w1y24hPattern(client, _m(acc, 'realized_profit')),
    realizedProfitToLossRatio: create_1m1w1y24hPattern(client, _m(acc, 'realized_profit_to_loss_ratio')),
    sellSideRiskRatio: create_1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio24hEma: create_1m1wPattern(client, _m(acc, 'sell_side_risk_ratio_24h_ema')),
    sentInLoss: createBaseCumulativePattern(client, _m(acc, 'sent_in_loss')),
    sentInLossEma: create_2wPattern(client, _m(acc, 'sent_in_loss_ema_2w')),
    sentInProfit: createBaseCumulativePattern(client, _m(acc, 'sent_in_profit')),
    sentInProfitEma: create_2wPattern(client, _m(acc, 'sent_in_profit_ema_2w')),
    sopr: create_1m1w1y24hPattern(client, _m(acc, 'sopr')),
    sopr24hEma: create_1m1wPattern2(client, _m(acc, 'sopr_24h_ema')),
    upperPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2
 * @property {_1m1w1y24hPattern<StoredF64>} adjustedSopr
 * @property {_1m1wPattern2} adjustedSoprEma
 * @property {MetricPattern1<Cents>} adjustedValueCreated
 * @property {_1m1w1y24hPattern<Cents>} adjustedValueCreatedSum
 * @property {MetricPattern1<Cents>} adjustedValueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} adjustedValueDestroyedSum
 * @property {MetricPattern18<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {CentsUsdPattern} grossPnl
 * @property {_1m1w1y24hPattern<Cents>} grossPnlSum
 * @property {MetricPattern18<CentsSquaredSats>} investorCapRaw
 * @property {CentsSatsUsdPattern} investorPrice
 * @property {BpsRatioPattern} investorPriceRatio
 * @property {MetricPattern1<Cents>} lossValueCreated
 * @property {MetricPattern1<Cents>} lossValueDestroyed
 * @property {CentsSatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {MetricPattern1<CentsSigned>} netPnlChange1m
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToMarketCap
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToRealizedCap
 * @property {CumulativeHeightPattern<CentsSigned>} netRealizedPnl
 * @property {MetricPattern1<CentsSigned>} netRealizedPnlEma1w
 * @property {BpsPercentRatioPattern} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Cents>} peakRegret
 * @property {BpsPercentRatioPattern} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Cents>} profitValueCreated
 * @property {MetricPattern1<Cents>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<CentsSigned>} realizedCapChange1m
 * @property {CumulativeHeightPattern<Cents>} realizedLoss
 * @property {MetricPattern1<Cents>} realizedLossEma1w
 * @property {BpsPercentRatioPattern} realizedLossRelToRealizedCap
 * @property {CentsSatsUsdPattern} realizedPrice
 * @property {BpsRatioPattern} realizedPriceRatio
 * @property {CumulativeHeightPattern<Cents>} realizedProfit
 * @property {MetricPattern1<Cents>} realizedProfitEma1w
 * @property {BpsPercentRatioPattern} realizedProfitRelToRealizedCap
 * @property {_1m1w1y24hPattern2} sellSideRiskRatio
 * @property {_1m1wPattern} sellSideRiskRatio24hEma
 * @property {BaseCumulativePattern} sentInLoss
 * @property {_2wPattern} sentInLossEma
 * @property {BaseCumulativePattern} sentInProfit
 * @property {_2wPattern} sentInProfitEma
 * @property {_1m1w1y24hPattern<StoredF64>} sopr
 * @property {_1m1wPattern2} sopr24hEma
 * @property {CentsSatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Cents>} valueCreated
 * @property {_1m1w1y24hPattern<Cents>} valueCreatedSum
 * @property {MetricPattern1<Cents>} valueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} valueDestroyedSum
 */

/**
 * Create a AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2}
 */
function createAdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2(client, acc) {
  return {
    adjustedSopr: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_sopr')),
    adjustedSoprEma: create_1m1wPattern2(client, _m(acc, 'adjusted_sopr_24h_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    adjustedValueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'adjusted_value_destroyed')),
    capRaw: createMetricPattern18(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    grossPnl: createCentsUsdPattern(client, _m(acc, 'realized_gross_pnl')),
    grossPnlSum: create_1m1w1y24hPattern(client, _m(acc, 'gross_pnl_sum')),
    investorCapRaw: createMetricPattern18(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createCentsSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceRatio: createBpsRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netPnlChange1m: createMetricPattern1(client, _m(acc, 'net_pnl_change_1m')),
    netPnlChange1mRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_market_cap')),
    netPnlChange1mRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_realized_cap')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlEma1w: createMetricPattern1(client, _m(acc, 'net_realized_pnl_ema_1w')),
    netRealizedPnlRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapChange1m: createMetricPattern1(client, _m(acc, 'realized_cap_change_1m')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLossEma1w: createMetricPattern1(client, _m(acc, 'realized_loss_ema_1w')),
    realizedLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createCentsSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceRatio: createBpsRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfitEma1w: createMetricPattern1(client, _m(acc, 'realized_profit_ema_1w')),
    realizedProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    sellSideRiskRatio: create_1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio24hEma: create_1m1wPattern(client, _m(acc, 'sell_side_risk_ratio_24h_ema')),
    sentInLoss: createBaseCumulativePattern(client, _m(acc, 'sent_in_loss')),
    sentInLossEma: create_2wPattern(client, _m(acc, 'sent_in_loss_ema_2w')),
    sentInProfit: createBaseCumulativePattern(client, _m(acc, 'sent_in_profit')),
    sentInProfitEma: create_2wPattern(client, _m(acc, 'sent_in_profit_ema_2w')),
    sopr: create_1m1w1y24hPattern(client, _m(acc, 'sopr')),
    sopr24hEma: create_1m1wPattern2(client, _m(acc, 'sopr_24h_ema')),
    upperPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2
 * @property {MetricPattern18<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {CentsUsdPattern} grossPnl
 * @property {_1m1w1y24hPattern<Cents>} grossPnlSum
 * @property {MetricPattern18<CentsSquaredSats>} investorCapRaw
 * @property {CentsSatsUsdPattern} investorPrice
 * @property {BpsRatioPattern} investorPriceRatio
 * @property {RatioPattern} investorPriceRatioExt
 * @property {MetricPattern1<Cents>} lossValueCreated
 * @property {MetricPattern1<Cents>} lossValueDestroyed
 * @property {CentsSatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {MetricPattern1<CentsSigned>} netPnlChange1m
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToMarketCap
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToRealizedCap
 * @property {CumulativeHeightPattern<CentsSigned>} netRealizedPnl
 * @property {MetricPattern1<CentsSigned>} netRealizedPnlEma1w
 * @property {BpsPercentRatioPattern} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Cents>} peakRegret
 * @property {BpsPercentRatioPattern} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Cents>} profitValueCreated
 * @property {MetricPattern1<Cents>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<CentsSigned>} realizedCapChange1m
 * @property {BpsPercentRatioPattern} realizedCapRelToOwnMarketCap
 * @property {CumulativeHeightPattern<Cents>} realizedLoss
 * @property {MetricPattern1<Cents>} realizedLossEma1w
 * @property {BpsPercentRatioPattern} realizedLossRelToRealizedCap
 * @property {_1m1w1y24hPattern<Cents>} realizedLossSum
 * @property {CentsSatsUsdPattern} realizedPrice
 * @property {BpsRatioPattern} realizedPriceRatio
 * @property {RatioPattern} realizedPriceRatioExt
 * @property {CumulativeHeightPattern<Cents>} realizedProfit
 * @property {MetricPattern1<Cents>} realizedProfitEma1w
 * @property {BpsPercentRatioPattern} realizedProfitRelToRealizedCap
 * @property {_1m1w1y24hPattern<Cents>} realizedProfitSum
 * @property {_1m1w1y24hPattern<StoredF64>} realizedProfitToLossRatio
 * @property {_1m1w1y24hPattern2} sellSideRiskRatio
 * @property {_1m1wPattern} sellSideRiskRatio24hEma
 * @property {BaseCumulativePattern} sentInLoss
 * @property {_2wPattern} sentInLossEma
 * @property {BaseCumulativePattern} sentInProfit
 * @property {_2wPattern} sentInProfitEma
 * @property {_1m1w1y24hPattern<StoredF64>} sopr
 * @property {_1m1wPattern2} sopr24hEma
 * @property {CentsSatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Cents>} valueCreated
 * @property {_1m1w1y24hPattern<Cents>} valueCreatedSum
 * @property {MetricPattern1<Cents>} valueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} valueDestroyedSum
 */

/**
 * Create a CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2}
 */
function createCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2(client, acc) {
  return {
    capRaw: createMetricPattern18(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    grossPnl: createCentsUsdPattern(client, _m(acc, 'realized_gross_pnl')),
    grossPnlSum: create_1m1w1y24hPattern(client, _m(acc, 'gross_pnl_sum')),
    investorCapRaw: createMetricPattern18(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createCentsSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceRatio: createBpsRatioPattern(client, _m(acc, 'investor_price_ratio')),
    investorPriceRatioExt: createRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netPnlChange1m: createMetricPattern1(client, _m(acc, 'net_pnl_change_1m')),
    netPnlChange1mRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_market_cap')),
    netPnlChange1mRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_realized_cap')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlEma1w: createMetricPattern1(client, _m(acc, 'net_realized_pnl_ema_1w')),
    netRealizedPnlRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapChange1m: createMetricPattern1(client, _m(acc, 'realized_cap_change_1m')),
    realizedCapRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLossEma1w: createMetricPattern1(client, _m(acc, 'realized_loss_ema_1w')),
    realizedLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedLossSum: create_1m1w1y24hPattern(client, _m(acc, 'realized_loss')),
    realizedPrice: createCentsSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceRatio: createBpsRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedPriceRatioExt: createRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfitEma1w: createMetricPattern1(client, _m(acc, 'realized_profit_ema_1w')),
    realizedProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitSum: create_1m1w1y24hPattern(client, _m(acc, 'realized_profit')),
    realizedProfitToLossRatio: create_1m1w1y24hPattern(client, _m(acc, 'realized_profit_to_loss_ratio')),
    sellSideRiskRatio: create_1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio24hEma: create_1m1wPattern(client, _m(acc, 'sell_side_risk_ratio_24h_ema')),
    sentInLoss: createBaseCumulativePattern(client, _m(acc, 'sent_in_loss')),
    sentInLossEma: create_2wPattern(client, _m(acc, 'sent_in_loss_ema_2w')),
    sentInProfit: createBaseCumulativePattern(client, _m(acc, 'sent_in_profit')),
    sentInProfitEma: create_2wPattern(client, _m(acc, 'sent_in_profit_ema_2w')),
    sopr: create_1m1w1y24hPattern(client, _m(acc, 'sopr')),
    sopr24hEma: create_1m1wPattern2(client, _m(acc, 'sopr_24h_ema')),
    upperPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern
 * @property {MetricPattern18<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {CentsUsdPattern} grossPnl
 * @property {_1m1w1y24hPattern<Cents>} grossPnlSum
 * @property {MetricPattern18<CentsSquaredSats>} investorCapRaw
 * @property {CentsSatsUsdPattern} investorPrice
 * @property {BpsRatioPattern} investorPriceRatio
 * @property {MetricPattern1<Cents>} lossValueCreated
 * @property {MetricPattern1<Cents>} lossValueDestroyed
 * @property {CentsSatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {MetricPattern1<CentsSigned>} netPnlChange1m
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToMarketCap
 * @property {BpsPercentRatioPattern} netPnlChange1mRelToRealizedCap
 * @property {CumulativeHeightPattern<CentsSigned>} netRealizedPnl
 * @property {MetricPattern1<CentsSigned>} netRealizedPnlEma1w
 * @property {BpsPercentRatioPattern} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Cents>} peakRegret
 * @property {BpsPercentRatioPattern} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Cents>} profitValueCreated
 * @property {MetricPattern1<Cents>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<CentsSigned>} realizedCapChange1m
 * @property {CumulativeHeightPattern<Cents>} realizedLoss
 * @property {MetricPattern1<Cents>} realizedLossEma1w
 * @property {BpsPercentRatioPattern} realizedLossRelToRealizedCap
 * @property {CentsSatsUsdPattern} realizedPrice
 * @property {BpsRatioPattern} realizedPriceRatio
 * @property {CumulativeHeightPattern<Cents>} realizedProfit
 * @property {MetricPattern1<Cents>} realizedProfitEma1w
 * @property {BpsPercentRatioPattern} realizedProfitRelToRealizedCap
 * @property {_1m1w1y24hPattern2} sellSideRiskRatio
 * @property {_1m1wPattern} sellSideRiskRatio24hEma
 * @property {BaseCumulativePattern} sentInLoss
 * @property {_2wPattern} sentInLossEma
 * @property {BaseCumulativePattern} sentInProfit
 * @property {_2wPattern} sentInProfitEma
 * @property {_1m1w1y24hPattern<StoredF64>} sopr
 * @property {_1m1wPattern2} sopr24hEma
 * @property {CentsSatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Cents>} valueCreated
 * @property {_1m1w1y24hPattern<Cents>} valueCreatedSum
 * @property {MetricPattern1<Cents>} valueDestroyed
 * @property {_1m1w1y24hPattern<Cents>} valueDestroyedSum
 */

/**
 * Create a CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern}
 */
function createCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, acc) {
  return {
    capRaw: createMetricPattern18(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    grossPnl: createCentsUsdPattern(client, _m(acc, 'realized_gross_pnl')),
    grossPnlSum: create_1m1w1y24hPattern(client, _m(acc, 'gross_pnl_sum')),
    investorCapRaw: createMetricPattern18(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createCentsSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceRatio: createBpsRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netPnlChange1m: createMetricPattern1(client, _m(acc, 'net_pnl_change_1m')),
    netPnlChange1mRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_market_cap')),
    netPnlChange1mRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_realized_cap')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlEma1w: createMetricPattern1(client, _m(acc, 'net_realized_pnl_ema_1w')),
    netRealizedPnlRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapChange1m: createMetricPattern1(client, _m(acc, 'realized_cap_change_1m')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLossEma1w: createMetricPattern1(client, _m(acc, 'realized_loss_ema_1w')),
    realizedLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createCentsSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceRatio: createBpsRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfitEma1w: createMetricPattern1(client, _m(acc, 'realized_profit_ema_1w')),
    realizedProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    sellSideRiskRatio: create_1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio24hEma: create_1m1wPattern(client, _m(acc, 'sell_side_risk_ratio_24h_ema')),
    sentInLoss: createBaseCumulativePattern(client, _m(acc, 'sent_in_loss')),
    sentInLossEma: create_2wPattern(client, _m(acc, 'sent_in_loss_ema_2w')),
    sentInProfit: createBaseCumulativePattern(client, _m(acc, 'sent_in_profit')),
    sentInProfitEma: create_2wPattern(client, _m(acc, 'sent_in_profit_ema_2w')),
    sopr: create_1m1w1y24hPattern(client, _m(acc, 'sopr')),
    sopr24hEma: create_1m1wPattern2(client, _m(acc, 'sopr_24h_ema')),
    upperPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreatedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyedSum: create_1m1w1y24hPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern
 * @property {CentsSatsUsdPattern} _0sdPrice
 * @property {MetricPattern1<StoredF32>} m05sd
 * @property {CentsSatsUsdPattern} m05sdPrice
 * @property {MetricPattern1<StoredF32>} m15sd
 * @property {CentsSatsUsdPattern} m15sdPrice
 * @property {MetricPattern1<StoredF32>} m1sd
 * @property {CentsSatsUsdPattern} m1sdPrice
 * @property {MetricPattern1<StoredF32>} m25sd
 * @property {CentsSatsUsdPattern} m25sdPrice
 * @property {MetricPattern1<StoredF32>} m2sd
 * @property {CentsSatsUsdPattern} m2sdPrice
 * @property {MetricPattern1<StoredF32>} m3sd
 * @property {CentsSatsUsdPattern} m3sdPrice
 * @property {MetricPattern1<StoredF32>} p05sd
 * @property {CentsSatsUsdPattern} p05sdPrice
 * @property {MetricPattern1<StoredF32>} p15sd
 * @property {CentsSatsUsdPattern} p15sdPrice
 * @property {MetricPattern1<StoredF32>} p1sd
 * @property {CentsSatsUsdPattern} p1sdPrice
 * @property {MetricPattern1<StoredF32>} p25sd
 * @property {CentsSatsUsdPattern} p25sdPrice
 * @property {MetricPattern1<StoredF32>} p2sd
 * @property {CentsSatsUsdPattern} p2sdPrice
 * @property {MetricPattern1<StoredF32>} p3sd
 * @property {CentsSatsUsdPattern} p3sdPrice
 * @property {MetricPattern1<StoredF32>} sd
 * @property {MetricPattern1<StoredF32>} sma
 * @property {MetricPattern1<StoredF32>} zscore
 */

/**
 * Create a _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern}
 */
function create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc) {
  return {
    _0sdPrice: createCentsSatsUsdPattern(client, _m(acc, '0sd_4y')),
    m05sd: createMetricPattern1(client, _m(acc, 'm0_5sd_4y')),
    m05sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm0_5sd_4y')),
    m15sd: createMetricPattern1(client, _m(acc, 'm1_5sd_4y')),
    m15sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm1_5sd_4y')),
    m1sd: createMetricPattern1(client, _m(acc, 'm1sd_4y')),
    m1sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm1sd_4y')),
    m25sd: createMetricPattern1(client, _m(acc, 'm2_5sd_4y')),
    m25sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm2_5sd_4y')),
    m2sd: createMetricPattern1(client, _m(acc, 'm2sd_4y')),
    m2sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm2sd_4y')),
    m3sd: createMetricPattern1(client, _m(acc, 'm3sd_4y')),
    m3sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'm3sd_4y')),
    p05sd: createMetricPattern1(client, _m(acc, 'p0_5sd_4y')),
    p05sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p0_5sd_4y')),
    p15sd: createMetricPattern1(client, _m(acc, 'p1_5sd_4y')),
    p15sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p1_5sd_4y')),
    p1sd: createMetricPattern1(client, _m(acc, 'p1sd_4y')),
    p1sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p1sd_4y')),
    p25sd: createMetricPattern1(client, _m(acc, 'p2_5sd_4y')),
    p25sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p2_5sd_4y')),
    p2sd: createMetricPattern1(client, _m(acc, 'p2sd_4y')),
    p2sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p2sd_4y')),
    p3sd: createMetricPattern1(client, _m(acc, 'p3sd_4y')),
    p3sdPrice: createCentsSatsUsdPattern(client, _m(acc, 'p3sd_4y')),
    sd: createMetricPattern1(client, _m(acc, 'sd_4y')),
    sma: createMetricPattern1(client, _m(acc, 'sma_4y')),
    zscore: createMetricPattern1(client, _m(acc, 'zscore_4y')),
  };
}

/**
 * @typedef {Object} BpsRatioPattern2
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {BpsRatioPattern} ratioPct1
 * @property {CentsSatsUsdPattern} ratioPct1Price
 * @property {BpsRatioPattern} ratioPct2
 * @property {CentsSatsUsdPattern} ratioPct2Price
 * @property {BpsRatioPattern} ratioPct5
 * @property {CentsSatsUsdPattern} ratioPct5Price
 * @property {BpsRatioPattern} ratioPct95
 * @property {CentsSatsUsdPattern} ratioPct95Price
 * @property {BpsRatioPattern} ratioPct98
 * @property {CentsSatsUsdPattern} ratioPct98Price
 * @property {BpsRatioPattern} ratioPct99
 * @property {CentsSatsUsdPattern} ratioPct99Price
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd1y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd2y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd4y
 * @property {BpsRatioPattern} ratioSma1m
 * @property {BpsRatioPattern} ratioSma1w
 */

/**
 * Create a BpsRatioPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsRatioPattern2}
 */
function createBpsRatioPattern2(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    ratio: createMetricPattern1(client, acc),
    ratioPct1: createBpsRatioPattern(client, _m(acc, 'pct1')),
    ratioPct1Price: createCentsSatsUsdPattern(client, _m(acc, 'pct1')),
    ratioPct2: createBpsRatioPattern(client, _m(acc, 'pct2')),
    ratioPct2Price: createCentsSatsUsdPattern(client, _m(acc, 'pct2')),
    ratioPct5: createBpsRatioPattern(client, _m(acc, 'pct5')),
    ratioPct5Price: createCentsSatsUsdPattern(client, _m(acc, 'pct5')),
    ratioPct95: createBpsRatioPattern(client, _m(acc, 'pct95')),
    ratioPct95Price: createCentsSatsUsdPattern(client, _m(acc, 'pct95')),
    ratioPct98: createBpsRatioPattern(client, _m(acc, 'pct98')),
    ratioPct98Price: createCentsSatsUsdPattern(client, _m(acc, 'pct98')),
    ratioPct99: createBpsRatioPattern(client, _m(acc, 'pct99')),
    ratioPct99Price: createCentsSatsUsdPattern(client, _m(acc, 'pct99')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd1y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd2y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd4y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSma1m: createBpsRatioPattern(client, _m(acc, 'sma_1m')),
    ratioSma1w: createBpsRatioPattern(client, _m(acc, 'sma_1w')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern2
 * @property {BpsPercentRatioPattern} investedCapitalInLossRelToRealizedCap
 * @property {BpsPercentRatioPattern} investedCapitalInProfitRelToRealizedCap
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToOwnMarketCap
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToMarketCap
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {BpsPercentRatioPattern} supplyInLossRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} supplyInLossRelToOwnSupply
 * @property {BpsPercentRatioPattern} supplyInProfitRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} supplyInProfitRelToOwnSupply
 * @property {BpsPercentRatioPattern} supplyRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} unrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} unrealizedLossRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} unrealizedLossRelToOwnMarketCap
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToMarketCap
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToOwnMarketCap
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern2}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern2(client, acc) {
  return {
    investedCapitalInLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'invested_capital_in_loss_rel_to_realized_cap')),
    investedCapitalInProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'invested_capital_in_profit_rel_to_realized_cap')),
    negUnrealizedLossRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    negUnrealizedLossRelToOwnGrossPnl: createBpsPercentRatioPattern(client, _m(acc, 'neg_unrealized_loss_rel_to_own_gross_pnl')),
    negUnrealizedLossRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap')),
    netUnrealizedPnlRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    netUnrealizedPnlRelToOwnGrossPnl: createBpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_own_gross_pnl')),
    netUnrealizedPnlRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedLossRelToOwnGrossPnl: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_own_gross_pnl')),
    unrealizedLossRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedProfitRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
    unrealizedProfitRelToOwnGrossPnl: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_own_gross_pnl')),
    unrealizedProfitRelToOwnMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
  };
}

/**
 * @typedef {Object} Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern
 * @property {CentsSatsUsdPattern} pct05
 * @property {CentsSatsUsdPattern} pct10
 * @property {CentsSatsUsdPattern} pct15
 * @property {CentsSatsUsdPattern} pct20
 * @property {CentsSatsUsdPattern} pct25
 * @property {CentsSatsUsdPattern} pct30
 * @property {CentsSatsUsdPattern} pct35
 * @property {CentsSatsUsdPattern} pct40
 * @property {CentsSatsUsdPattern} pct45
 * @property {CentsSatsUsdPattern} pct50
 * @property {CentsSatsUsdPattern} pct55
 * @property {CentsSatsUsdPattern} pct60
 * @property {CentsSatsUsdPattern} pct65
 * @property {CentsSatsUsdPattern} pct70
 * @property {CentsSatsUsdPattern} pct75
 * @property {CentsSatsUsdPattern} pct80
 * @property {CentsSatsUsdPattern} pct85
 * @property {CentsSatsUsdPattern} pct90
 * @property {CentsSatsUsdPattern} pct95
 */

/**
 * Create a Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern}
 */
function createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, acc) {
  return {
    pct05: createCentsSatsUsdPattern(client, _m(acc, 'pct05')),
    pct10: createCentsSatsUsdPattern(client, _m(acc, 'pct10')),
    pct15: createCentsSatsUsdPattern(client, _m(acc, 'pct15')),
    pct20: createCentsSatsUsdPattern(client, _m(acc, 'pct20')),
    pct25: createCentsSatsUsdPattern(client, _m(acc, 'pct25')),
    pct30: createCentsSatsUsdPattern(client, _m(acc, 'pct30')),
    pct35: createCentsSatsUsdPattern(client, _m(acc, 'pct35')),
    pct40: createCentsSatsUsdPattern(client, _m(acc, 'pct40')),
    pct45: createCentsSatsUsdPattern(client, _m(acc, 'pct45')),
    pct50: createCentsSatsUsdPattern(client, _m(acc, 'pct50')),
    pct55: createCentsSatsUsdPattern(client, _m(acc, 'pct55')),
    pct60: createCentsSatsUsdPattern(client, _m(acc, 'pct60')),
    pct65: createCentsSatsUsdPattern(client, _m(acc, 'pct65')),
    pct70: createCentsSatsUsdPattern(client, _m(acc, 'pct70')),
    pct75: createCentsSatsUsdPattern(client, _m(acc, 'pct75')),
    pct80: createCentsSatsUsdPattern(client, _m(acc, 'pct80')),
    pct85: createCentsSatsUsdPattern(client, _m(acc, 'pct85')),
    pct90: createCentsSatsUsdPattern(client, _m(acc, 'pct90')),
    pct95: createCentsSatsUsdPattern(client, _m(acc, 'pct95')),
  };
}

/**
 * @typedef {Object} RatioPattern
 * @property {BpsRatioPattern} ratioPct1
 * @property {CentsSatsUsdPattern} ratioPct1Price
 * @property {BpsRatioPattern} ratioPct2
 * @property {CentsSatsUsdPattern} ratioPct2Price
 * @property {BpsRatioPattern} ratioPct5
 * @property {CentsSatsUsdPattern} ratioPct5Price
 * @property {BpsRatioPattern} ratioPct95
 * @property {CentsSatsUsdPattern} ratioPct95Price
 * @property {BpsRatioPattern} ratioPct98
 * @property {CentsSatsUsdPattern} ratioPct98Price
 * @property {BpsRatioPattern} ratioPct99
 * @property {CentsSatsUsdPattern} ratioPct99Price
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd1y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd2y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd4y
 * @property {BpsRatioPattern} ratioSma1m
 * @property {BpsRatioPattern} ratioSma1w
 */

/**
 * Create a RatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioPattern}
 */
function createRatioPattern(client, acc) {
  return {
    ratioPct1: createBpsRatioPattern(client, _m(acc, 'pct1')),
    ratioPct1Price: createCentsSatsUsdPattern(client, _m(acc, 'pct1')),
    ratioPct2: createBpsRatioPattern(client, _m(acc, 'pct2')),
    ratioPct2Price: createCentsSatsUsdPattern(client, _m(acc, 'pct2')),
    ratioPct5: createBpsRatioPattern(client, _m(acc, 'pct5')),
    ratioPct5Price: createCentsSatsUsdPattern(client, _m(acc, 'pct5')),
    ratioPct95: createBpsRatioPattern(client, _m(acc, 'pct95')),
    ratioPct95Price: createCentsSatsUsdPattern(client, _m(acc, 'pct95')),
    ratioPct98: createBpsRatioPattern(client, _m(acc, 'pct98')),
    ratioPct98Price: createCentsSatsUsdPattern(client, _m(acc, 'pct98')),
    ratioPct99: createBpsRatioPattern(client, _m(acc, 'pct99')),
    ratioPct99Price: createCentsSatsUsdPattern(client, _m(acc, 'pct99')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd1y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd2y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSd4y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
    ratioSma1m: createBpsRatioPattern(client, _m(acc, 'sma_1m')),
    ratioSma1w: createBpsRatioPattern(client, _m(acc, 'sma_1w')),
  };
}

/**
 * @typedef {Object} GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern
 * @property {CentsUsdPattern} greedIndex
 * @property {CentsUsdPattern} grossPnl
 * @property {CentsUsdPattern} investedCapitalInLoss
 * @property {MetricPattern18<CentsSats>} investedCapitalInLossRaw
 * @property {CentsUsdPattern} investedCapitalInProfit
 * @property {MetricPattern18<CentsSats>} investedCapitalInProfitRaw
 * @property {MetricPattern18<CentsSquaredSats>} investorCapInLossRaw
 * @property {MetricPattern18<CentsSquaredSats>} investorCapInProfitRaw
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {CentsUsdPattern} netSentiment
 * @property {CentsUsdPattern} netUnrealizedPnl
 * @property {CentsUsdPattern} painIndex
 * @property {BtcCentsSatsUsdPattern} supplyInLoss
 * @property {BtcCentsSatsUsdPattern} supplyInProfit
 * @property {CentsUsdPattern} unrealizedLoss
 * @property {CentsUsdPattern} unrealizedProfit
 */

/**
 * Create a GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern}
 */
function createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, acc) {
  return {
    greedIndex: createCentsUsdPattern(client, _m(acc, 'greed_index')),
    grossPnl: createCentsUsdPattern(client, _m(acc, 'unrealized_gross_pnl')),
    investedCapitalInLoss: createCentsUsdPattern(client, _m(acc, 'invested_capital_in_loss')),
    investedCapitalInLossRaw: createMetricPattern18(client, _m(acc, 'invested_capital_in_loss_raw')),
    investedCapitalInProfit: createCentsUsdPattern(client, _m(acc, 'invested_capital_in_profit')),
    investedCapitalInProfitRaw: createMetricPattern18(client, _m(acc, 'invested_capital_in_profit_raw')),
    investorCapInLossRaw: createMetricPattern18(client, _m(acc, 'investor_cap_in_loss_raw')),
    investorCapInProfitRaw: createMetricPattern18(client, _m(acc, 'investor_cap_in_profit_raw')),
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netSentiment: createCentsUsdPattern(client, _m(acc, 'net_sentiment')),
    netUnrealizedPnl: createCentsUsdPattern(client, _m(acc, 'net_unrealized_pnl')),
    painIndex: createCentsUsdPattern(client, _m(acc, 'pain_index')),
    supplyInLoss: createBtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createBtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_profit')),
    unrealizedLoss: createCentsUsdPattern(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createCentsUsdPattern(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern2
 * @property {BpsPercentRatioPattern} _10y
 * @property {BpsPercentRatioPattern} _1m
 * @property {BpsPercentRatioPattern} _1w
 * @property {BpsPercentRatioPattern} _1y
 * @property {BpsPercentRatioPattern} _2y
 * @property {BpsPercentRatioPattern} _3m
 * @property {BpsPercentRatioPattern} _3y
 * @property {BpsPercentRatioPattern} _4y
 * @property {BpsPercentRatioPattern} _5y
 * @property {BpsPercentRatioPattern} _6m
 * @property {BpsPercentRatioPattern} _6y
 * @property {BpsPercentRatioPattern} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, acc) {
  return {
    _10y: createBpsPercentRatioPattern(client, _m(acc, '10y')),
    _1m: createBpsPercentRatioPattern(client, _m(acc, '1m')),
    _1w: createBpsPercentRatioPattern(client, _m(acc, '1w')),
    _1y: createBpsPercentRatioPattern(client, _m(acc, '1y')),
    _2y: createBpsPercentRatioPattern(client, _m(acc, '2y')),
    _3m: createBpsPercentRatioPattern(client, _m(acc, '3m')),
    _3y: createBpsPercentRatioPattern(client, _m(acc, '3y')),
    _4y: createBpsPercentRatioPattern(client, _m(acc, '4y')),
    _5y: createBpsPercentRatioPattern(client, _m(acc, '5y')),
    _6m: createBpsPercentRatioPattern(client, _m(acc, '6m')),
    _6y: createBpsPercentRatioPattern(client, _m(acc, '6y')),
    _8y: createBpsPercentRatioPattern(client, _m(acc, '8y')),
  };
}

/**
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern3
 * @property {BtcCentsSatsUsdPattern} _10y
 * @property {BtcCentsSatsUsdPattern} _1m
 * @property {BtcCentsSatsUsdPattern} _1w
 * @property {BtcCentsSatsUsdPattern} _1y
 * @property {BtcCentsSatsUsdPattern} _2y
 * @property {BtcCentsSatsUsdPattern} _3m
 * @property {BtcCentsSatsUsdPattern} _3y
 * @property {BtcCentsSatsUsdPattern} _4y
 * @property {BtcCentsSatsUsdPattern} _5y
 * @property {BtcCentsSatsUsdPattern} _6m
 * @property {BtcCentsSatsUsdPattern} _6y
 * @property {BtcCentsSatsUsdPattern} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, acc) {
  return {
    _10y: createBtcCentsSatsUsdPattern(client, _m(acc, '10y')),
    _1m: createBtcCentsSatsUsdPattern(client, _m(acc, '1m')),
    _1w: createBtcCentsSatsUsdPattern(client, _m(acc, '1w')),
    _1y: createBtcCentsSatsUsdPattern(client, _m(acc, '1y')),
    _2y: createBtcCentsSatsUsdPattern(client, _m(acc, '2y')),
    _3m: createBtcCentsSatsUsdPattern(client, _m(acc, '3m')),
    _3y: createBtcCentsSatsUsdPattern(client, _m(acc, '3y')),
    _4y: createBtcCentsSatsUsdPattern(client, _m(acc, '4y')),
    _5y: createBtcCentsSatsUsdPattern(client, _m(acc, '5y')),
    _6m: createBtcCentsSatsUsdPattern(client, _m(acc, '6m')),
    _6y: createBtcCentsSatsUsdPattern(client, _m(acc, '6y')),
    _8y: createBtcCentsSatsUsdPattern(client, _m(acc, '8y')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern
 * @property {BpsPercentRatioPattern} investedCapitalInLossRelToRealizedCap
 * @property {BpsPercentRatioPattern} investedCapitalInProfitRelToRealizedCap
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {BpsPercentRatioPattern} supplyInLossRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} supplyInLossRelToOwnSupply
 * @property {BpsPercentRatioPattern} supplyInProfitRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} supplyInProfitRelToOwnSupply
 * @property {BpsPercentRatioPattern} supplyRelToCirculatingSupply
 * @property {BpsPercentRatioPattern} unrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToMarketCap
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc) {
  return {
    investedCapitalInLossRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'invested_capital_in_loss_rel_to_realized_cap')),
    investedCapitalInProfitRelToRealizedCap: createBpsPercentRatioPattern(client, _m(acc, 'invested_capital_in_profit_rel_to_realized_cap')),
    negUnrealizedLossRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    netUnrealizedPnlRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createBpsPercentRatioPattern(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern
 * @property {MetricPattern18<StoredU64>} average
 * @property {MetricPattern18<StoredU64>} cumulative
 * @property {MetricPattern18<StoredU64>} max
 * @property {MetricPattern18<StoredU64>} median
 * @property {MetricPattern18<StoredU64>} min
 * @property {MetricPattern18<StoredU64>} pct10
 * @property {MetricPattern18<StoredU64>} pct25
 * @property {MetricPattern18<StoredU64>} pct75
 * @property {MetricPattern18<StoredU64>} pct90
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern} rolling
 * @property {MetricPattern18<StoredU64>} sum
 */

/**
 * Create a AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern}
 */
function createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, acc) {
  return {
    average: createMetricPattern18(client, _m(acc, 'average')),
    cumulative: createMetricPattern18(client, _m(acc, 'cumulative')),
    max: createMetricPattern18(client, _m(acc, 'max')),
    median: createMetricPattern18(client, _m(acc, 'median')),
    min: createMetricPattern18(client, _m(acc, 'min')),
    pct10: createMetricPattern18(client, _m(acc, 'p10')),
    pct25: createMetricPattern18(client, _m(acc, 'p25')),
    pct75: createMetricPattern18(client, _m(acc, 'p75')),
    pct90: createMetricPattern18(client, _m(acc, 'p90')),
    rolling: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc),
    sum: createMetricPattern18(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern
 * @property {_1m1w1y24hPattern<StoredU64>} average
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {MetricPattern18<StoredU64>} height
 * @property {_1m1w1y24hPattern<StoredU64>} max
 * @property {_1m1w1y24hPattern<StoredU64>} median
 * @property {_1m1w1y24hPattern<StoredU64>} min
 * @property {_1m1w1y24hPattern<StoredU64>} pct10
 * @property {_1m1w1y24hPattern<StoredU64>} pct25
 * @property {_1m1w1y24hPattern<StoredU64>} pct75
 * @property {_1m1w1y24hPattern<StoredU64>} pct90
 * @property {_1m1w1y24hPattern<StoredU64>} sum
 */

/**
 * Create a AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern}
 */
function createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc) {
  return {
    average: create_1m1w1y24hPattern(client, _m(acc, 'average')),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    height: createMetricPattern18(client, acc),
    max: create_1m1w1y24hPattern(client, _m(acc, 'max')),
    median: create_1m1w1y24hPattern(client, _m(acc, 'median')),
    min: create_1m1w1y24hPattern(client, _m(acc, 'min')),
    pct10: create_1m1w1y24hPattern(client, _m(acc, 'p10')),
    pct25: create_1m1w1y24hPattern(client, _m(acc, 'p25')),
    pct75: create_1m1w1y24hPattern(client, _m(acc, 'p75')),
    pct90: create_1m1w1y24hPattern(client, _m(acc, 'p90')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageGainsLossesRsiStochPattern
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {BpsPercentRatioPattern} rsi
 * @property {BpsPercentRatioPattern} rsiMax
 * @property {BpsPercentRatioPattern} rsiMin
 * @property {BpsPercentRatioPattern} stochRsi
 * @property {BpsPercentRatioPattern} stochRsiD
 * @property {BpsPercentRatioPattern} stochRsiK
 */

/**
 * Create a AverageGainsLossesRsiStochPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageGainsLossesRsiStochPattern}
 */
function createAverageGainsLossesRsiStochPattern(client, acc) {
  return {
    averageGain: createMetricPattern1(client, _m(acc, 'average_gain_24h')),
    averageLoss: createMetricPattern1(client, _m(acc, 'average_loss_24h')),
    gains: createMetricPattern1(client, _m(acc, 'gains_24h')),
    losses: createMetricPattern1(client, _m(acc, 'losses_24h')),
    rsi: createBpsPercentRatioPattern(client, _m(acc, '24h')),
    rsiMax: createBpsPercentRatioPattern(client, _m(acc, 'max_24h')),
    rsiMin: createBpsPercentRatioPattern(client, _m(acc, 'min_24h')),
    stochRsi: createBpsPercentRatioPattern(client, _m(acc, 'stoch_24h')),
    stochRsiD: createBpsPercentRatioPattern(client, _m(acc, 'stoch_d_24h')),
    stochRsiK: createBpsPercentRatioPattern(client, _m(acc, 'stoch_k_24h')),
  };
}

/**
 * @typedef {Object} ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {MetricPattern1<StoredF64>} addrCountChange1m
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {ChangeHalvedTotalPattern} supply
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern}
 */
function createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoinblocksCoindaysSentPattern(client, acc),
    addrCount: createMetricPattern1(client, _m(acc, 'addr_count')),
    addrCountChange1m: createMetricPattern1(client, _m(acc, 'addr_count_change_1m')),
    costBasis: createMaxMinPattern(client, _m(acc, 'cost_basis')),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: createChangeHalvedTotalPattern(client, _m(acc, 'supply')),
    unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern
 * @property {ChangeCountPattern} all
 * @property {ChangeCountPattern} p2a
 * @property {ChangeCountPattern} p2pk33
 * @property {ChangeCountPattern} p2pk65
 * @property {ChangeCountPattern} p2pkh
 * @property {ChangeCountPattern} p2sh
 * @property {ChangeCountPattern} p2tr
 * @property {ChangeCountPattern} p2wpkh
 * @property {ChangeCountPattern} p2wsh
 */

/**
 * Create a AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern}
 */
function createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, acc) {
  return {
    all: createChangeCountPattern(client, acc),
    p2a: createChangeCountPattern(client, _p('p2a', acc)),
    p2pk33: createChangeCountPattern(client, _p('p2pk33', acc)),
    p2pk65: createChangeCountPattern(client, _p('p2pk65', acc)),
    p2pkh: createChangeCountPattern(client, _p('p2pkh', acc)),
    p2sh: createChangeCountPattern(client, _p('p2sh', acc)),
    p2tr: createChangeCountPattern(client, _p('p2tr', acc)),
    p2wpkh: createChangeCountPattern(client, _p('p2wpkh', acc)),
    p2wsh: createChangeCountPattern(client, _p('p2wsh', acc)),
  };
}

/**
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2
 * @property {BtcCentsSatsUsdPattern} average
 * @property {BtcCentsSatsUsdPattern} max
 * @property {BtcCentsSatsUsdPattern} median
 * @property {BtcCentsSatsUsdPattern} min
 * @property {BtcCentsSatsUsdPattern} pct10
 * @property {BtcCentsSatsUsdPattern} pct25
 * @property {BtcCentsSatsUsdPattern} pct75
 * @property {BtcCentsSatsUsdPattern} pct90
 * @property {BtcCentsSatsUsdPattern} sum
 */

/**
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, acc) {
  return {
    average: createBtcCentsSatsUsdPattern(client, _m(acc, 'average')),
    max: createBtcCentsSatsUsdPattern(client, _m(acc, 'max')),
    median: createBtcCentsSatsUsdPattern(client, _m(acc, 'median')),
    min: createBtcCentsSatsUsdPattern(client, _m(acc, 'min')),
    pct10: createBtcCentsSatsUsdPattern(client, _m(acc, 'p10')),
    pct25: createBtcCentsSatsUsdPattern(client, _m(acc, 'p25')),
    pct75: createBtcCentsSatsUsdPattern(client, _m(acc, 'p75')),
    pct90: createBtcCentsSatsUsdPattern(client, _m(acc, 'p90')),
    sum: createBtcCentsSatsUsdPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern
 * @property {_1m1w1y24hPattern<StoredU64>} average
 * @property {_1m1w1y24hPattern<StoredU64>} max
 * @property {_1m1w1y24hPattern<StoredU64>} median
 * @property {_1m1w1y24hPattern<StoredU64>} min
 * @property {_1m1w1y24hPattern<StoredU64>} pct10
 * @property {_1m1w1y24hPattern<StoredU64>} pct25
 * @property {_1m1w1y24hPattern<StoredU64>} pct75
 * @property {_1m1w1y24hPattern<StoredU64>} pct90
 * @property {_1m1w1y24hPattern<StoredU64>} sum
 */

/**
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc) {
  return {
    average: create_1m1w1y24hPattern(client, _m(acc, 'average')),
    max: create_1m1w1y24hPattern(client, _m(acc, 'max')),
    median: create_1m1w1y24hPattern(client, _m(acc, 'median')),
    min: create_1m1w1y24hPattern(client, _m(acc, 'min')),
    pct10: create_1m1w1y24hPattern(client, _m(acc, 'p10')),
    pct25: create_1m1w1y24hPattern(client, _m(acc, 'p25')),
    pct75: create_1m1w1y24hPattern(client, _m(acc, 'p75')),
    pct90: create_1m1w1y24hPattern(client, _m(acc, 'p90')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @template T
 * @typedef {Object} AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern
 * @property {_1m1w1y24hPattern<T>} average
 * @property {MetricPattern18<T>} height
 * @property {_1m1w1y24hPattern<T>} max
 * @property {_1m1w1y24hPattern<T>} median
 * @property {_1m1w1y24hPattern<T>} min
 * @property {_1m1w1y24hPattern<T>} pct10
 * @property {_1m1w1y24hPattern<T>} pct25
 * @property {_1m1w1y24hPattern<T>} pct75
 * @property {_1m1w1y24hPattern<T>} pct90
 */

/**
 * Create a AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>}
 */
function createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc) {
  return {
    average: create_1m1w1y24hPattern(client, _m(acc, 'average')),
    height: createMetricPattern18(client, acc),
    max: create_1m1w1y24hPattern(client, _m(acc, 'max')),
    median: create_1m1w1y24hPattern(client, _m(acc, 'median')),
    min: create_1m1w1y24hPattern(client, _m(acc, 'min')),
    pct10: create_1m1w1y24hPattern(client, _m(acc, 'p10')),
    pct25: create_1m1w1y24hPattern(client, _m(acc, 'p25')),
    pct75: create_1m1w1y24hPattern(client, _m(acc, 'p75')),
    pct90: create_1m1w1y24hPattern(client, _m(acc, 'p90')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hBtcCentsSatsUsdPattern
 * @property {BtcCentsSatsUsdPattern} _1m
 * @property {BtcCentsSatsUsdPattern} _1w
 * @property {BtcCentsSatsUsdPattern} _1y
 * @property {BtcCentsSatsUsdPattern} _24h
 * @property {MetricPattern18<Bitcoin>} btc
 * @property {MetricPattern18<Cents>} cents
 * @property {MetricPattern18<Sats>} sats
 * @property {MetricPattern18<Dollars>} usd
 */

/**
 * Create a _1m1w1y24hBtcCentsSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hBtcCentsSatsUsdPattern}
 */
function create_1m1w1y24hBtcCentsSatsUsdPattern(client, acc) {
  return {
    _1m: createBtcCentsSatsUsdPattern(client, _m(acc, '1m')),
    _1w: createBtcCentsSatsUsdPattern(client, _m(acc, '1w')),
    _1y: createBtcCentsSatsUsdPattern(client, _m(acc, '1y')),
    _24h: createBtcCentsSatsUsdPattern(client, _m(acc, '24h')),
    btc: createMetricPattern18(client, _m(acc, 'btc')),
    cents: createMetricPattern18(client, _m(acc, 'cents')),
    sats: createMetricPattern18(client, acc),
    usd: createMetricPattern18(client, _m(acc, 'usd')),
  };
}

/**
 * @template T
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern
 * @property {MetricPattern18<T>} average
 * @property {MetricPattern18<T>} max
 * @property {MetricPattern18<T>} median
 * @property {MetricPattern18<T>} min
 * @property {MetricPattern18<T>} pct10
 * @property {MetricPattern18<T>} pct25
 * @property {MetricPattern18<T>} pct75
 * @property {MetricPattern18<T>} pct90
 */

/**
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc) {
  return {
    average: createMetricPattern18(client, _m(acc, 'average')),
    max: createMetricPattern18(client, _m(acc, 'max')),
    median: createMetricPattern18(client, _m(acc, 'median')),
    min: createMetricPattern18(client, _m(acc, 'min')),
    pct10: createMetricPattern18(client, _m(acc, 'p10')),
    pct25: createMetricPattern18(client, _m(acc, 'p25')),
    pct75: createMetricPattern18(client, _m(acc, 'p75')),
    pct90: createMetricPattern18(client, _m(acc, 'p90')),
  };
}

/**
 * @typedef {Object} _10y2y3y4y5y6y8yPattern
 * @property {BpsPercentRatioPattern} _10y
 * @property {BpsPercentRatioPattern} _2y
 * @property {BpsPercentRatioPattern} _3y
 * @property {BpsPercentRatioPattern} _4y
 * @property {BpsPercentRatioPattern} _5y
 * @property {BpsPercentRatioPattern} _6y
 * @property {BpsPercentRatioPattern} _8y
 */

/**
 * Create a _10y2y3y4y5y6y8yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y2y3y4y5y6y8yPattern}
 */
function create_10y2y3y4y5y6y8yPattern(client, acc) {
  return {
    _10y: createBpsPercentRatioPattern(client, _m(acc, '10y')),
    _2y: createBpsPercentRatioPattern(client, _m(acc, '2y')),
    _3y: createBpsPercentRatioPattern(client, _m(acc, '3y')),
    _4y: createBpsPercentRatioPattern(client, _m(acc, '4y')),
    _5y: createBpsPercentRatioPattern(client, _m(acc, '5y')),
    _6y: createBpsPercentRatioPattern(client, _m(acc, '6y')),
    _8y: createBpsPercentRatioPattern(client, _m(acc, '8y')),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {InvestedMaxMinPercentilesPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern2} relative
 * @property {ChangeHalvedTotalPattern} supply
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoinblocksCoindaysSentPattern(client, acc),
    costBasis: createInvestedMaxMinPercentilesPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern2(client, acc),
    supply: createChangeHalvedTotalPattern(client, _m(acc, 'supply')),
    unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {ChangeHalvedTotalPattern} supply
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, acc) {
  return {
    activity: createCoinblocksCoindaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, _m(acc, 'cost_basis')),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createAdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern2(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: createChangeHalvedTotalPattern(client, _m(acc, 'supply')),
    unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {ChangeHalvedTotalPattern} supply
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, acc) {
  return {
    activity: createCoinblocksCoindaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, _m(acc, 'cost_basis')),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: createChangeHalvedTotalPattern(client, _m(acc, 'supply')),
    unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _1m1w1y24hBaseCumulativePattern
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2} _1m
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2} _1w
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2} _1y
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2} _24h
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 */

/**
 * Create a _1m1w1y24hBaseCumulativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hBaseCumulativePattern}
 */
function create_1m1w1y24hBaseCumulativePattern(client, acc) {
  return {
    _1m: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, '1m')),
    _1w: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, '1w')),
    _1y: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, '1y')),
    _24h: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, '24h')),
    base: createBtcCentsSatsUsdPattern(client, acc),
    cumulative: createBtcCentsSatsUsdPattern(client, _m(acc, 'cumulative')),
  };
}

/**
 * @typedef {Object} BalanceBothReactivatedReceivingSendingPattern
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} balanceDecreased
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} balanceIncreased
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} both
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} reactivated
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} receiving
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} sending
 */

/**
 * Create a BalanceBothReactivatedReceivingSendingPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BalanceBothReactivatedReceivingSendingPattern}
 */
function createBalanceBothReactivatedReceivingSendingPattern(client, acc) {
  return {
    balanceDecreased: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_decreased')),
    balanceIncreased: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_increased')),
    both: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'both')),
    reactivated: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'reactivated')),
    receiving: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'receiving')),
    sending: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'sending')),
  };
}

/**
 * @typedef {Object} EmaHistogramLineSignalPattern
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} histogram
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 */

/**
 * Create a EmaHistogramLineSignalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {EmaHistogramLineSignalPattern}
 */
function createEmaHistogramLineSignalPattern(client, acc) {
  return {
    emaFast: createMetricPattern1(client, _m(acc, 'ema_fast_24h')),
    emaSlow: createMetricPattern1(client, _m(acc, 'ema_slow_24h')),
    histogram: createMetricPattern1(client, _m(acc, 'histogram_24h')),
    line: createMetricPattern1(client, _m(acc, 'line_24h')),
    signal: createMetricPattern1(client, _m(acc, 'signal_24h')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern2
 * @property {BpsPercentRatioPattern} _1m
 * @property {BpsPercentRatioPattern} _1w
 * @property {BpsPercentRatioPattern} _1y
 * @property {BpsPercentRatioPattern} _24h
 */

/**
 * Create a _1m1w1y24hPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern2}
 */
function create_1m1w1y24hPattern2(client, acc) {
  return {
    _1m: createBpsPercentRatioPattern(client, _m(acc, '1m')),
    _1w: createBpsPercentRatioPattern(client, _m(acc, '1w')),
    _1y: createBpsPercentRatioPattern(client, _m(acc, '1y')),
    _24h: createBpsPercentRatioPattern(client, _m(acc, '24h')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern5
 * @property {BtcCentsSatsUsdPattern} _1m
 * @property {BtcCentsSatsUsdPattern} _1w
 * @property {BtcCentsSatsUsdPattern} _1y
 * @property {BtcCentsSatsUsdPattern} _24h
 */

/**
 * Create a _1m1w1y24hPattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern5}
 */
function create_1m1w1y24hPattern5(client, acc) {
  return {
    _1m: createBtcCentsSatsUsdPattern(client, _m(acc, '1m')),
    _1w: createBtcCentsSatsUsdPattern(client, _m(acc, '1w')),
    _1y: createBtcCentsSatsUsdPattern(client, _m(acc, '1y')),
    _24h: createBtcCentsSatsUsdPattern(client, _m(acc, '24h')),
  };
}

/**
 * @typedef {Object} BlocksDominanceRewardsPattern
 * @property {CumulativeHeightSumPattern<StoredU32>} blocksMined
 * @property {BpsPercentRatioPattern} dominance
 * @property {_1m1w1y24hPattern2} dominanceRolling
 * @property {BaseCumulativeSumPattern} rewards
 */

/**
 * Create a BlocksDominanceRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlocksDominanceRewardsPattern}
 */
function createBlocksDominanceRewardsPattern(client, acc) {
  return {
    blocksMined: createCumulativeHeightSumPattern(client, _m(acc, 'blocks_mined')),
    dominance: createBpsPercentRatioPattern(client, _m(acc, 'dominance')),
    dominanceRolling: create_1m1w1y24hPattern2(client, _m(acc, 'dominance')),
    rewards: createBaseCumulativeSumPattern(client, _m(acc, 'rewards')),
  };
}

/**
 * @typedef {Object} BtcCentsSatsUsdPattern
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcCentsSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcCentsSatsUsdPattern}
 */
function createBtcCentsSatsUsdPattern(client, acc) {
  return {
    btc: createMetricPattern1(client, _m(acc, 'btc')),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    sats: createMetricPattern1(client, acc),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} CoinblocksCoindaysSentPattern
 * @property {CumulativeHeightSumPattern<StoredF64>} coinblocksDestroyed
 * @property {CumulativeHeightSumPattern<StoredF64>} coindaysDestroyed
 * @property {BaseCumulativePattern} sent
 * @property {_2wPattern} sentEma
 */

/**
 * Create a CoinblocksCoindaysSentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinblocksCoindaysSentPattern}
 */
function createCoinblocksCoindaysSentPattern(client, acc) {
  return {
    coinblocksDestroyed: createCumulativeHeightSumPattern(client, _m(acc, 'coinblocks_destroyed')),
    coindaysDestroyed: createCumulativeHeightSumPattern(client, _m(acc, 'coindays_destroyed')),
    sent: createBaseCumulativePattern(client, _m(acc, 'sent')),
    sentEma: create_2wPattern(client, _m(acc, 'sent_ema_2w')),
  };
}

/**
 * @typedef {Object} InvestedMaxMinPercentilesPattern
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} investedCapital
 * @property {CentsSatsUsdPattern} max
 * @property {CentsSatsUsdPattern} min
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} percentiles
 */

/**
 * Create a InvestedMaxMinPercentilesPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedMaxMinPercentilesPattern}
 */
function createInvestedMaxMinPercentilesPattern(client, acc) {
  return {
    investedCapital: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital')),
    max: createCentsSatsUsdPattern(client, _m(acc, 'cost_basis_max')),
    min: createCentsSatsUsdPattern(client, _m(acc, 'cost_basis_min')),
    percentiles: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis')),
  };
}

/**
 * @template T
 * @typedef {Object} _1m1w1y24hPattern
 * @property {MetricPattern1<T>} _1m
 * @property {MetricPattern1<T>} _1w
 * @property {MetricPattern1<T>} _1y
 * @property {MetricPattern1<T>} _24h
 */

/**
 * Create a _1m1w1y24hPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern<T>}
 */
function create_1m1w1y24hPattern(client, acc) {
  return {
    _1m: createMetricPattern1(client, _m(acc, '1m')),
    _1w: createMetricPattern1(client, _m(acc, '1w')),
    _1y: createMetricPattern1(client, _m(acc, '1y')),
    _24h: createMetricPattern1(client, _m(acc, '24h')),
  };
}

/**
 * @typedef {Object} BaseCumulativeSumPattern
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 * @property {_1m1w1y24hPattern5} sum
 */

/**
 * Create a BaseCumulativeSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern}
 */
function createBaseCumulativeSumPattern(client, acc) {
  return {
    base: createBtcCentsSatsUsdPattern(client, acc),
    cumulative: createBtcCentsSatsUsdPattern(client, _m(acc, 'cumulative')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern2
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<BasisPoints16>} bps
 * @property {MetricPattern1<StoredF32>} percent
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsPercentRatioPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsPercentRatioPattern2}
 */
function createBpsPercentRatioPattern2(client, acc) {
  return {
    bps: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern
 * @property {MetricPattern1<BasisPoints16>} bps
 * @property {MetricPattern1<StoredF32>} percent
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsPercentRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsPercentRatioPattern}
 */
function createBpsPercentRatioPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} BpsPriceRatioPattern
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {CentsSatsUsdPattern} price
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsPriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsPriceRatioPattern}
 */
function createBpsPriceRatioPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'ratio_bps')),
    price: createCentsSatsUsdPattern(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} CentsSatsUsdPattern2
 * @property {MetricPattern2<Cents>} cents
 * @property {MetricPattern2<Sats>} sats
 * @property {MetricPattern2<Dollars>} usd
 */

/**
 * Create a CentsSatsUsdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsSatsUsdPattern2}
 */
function createCentsSatsUsdPattern2(client, acc) {
  return {
    cents: createMetricPattern2(client, _m(acc, 'cents')),
    sats: createMetricPattern2(client, _m(acc, 'sats')),
    usd: createMetricPattern2(client, acc),
  };
}

/**
 * @typedef {Object} CentsSatsUsdPattern
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsSatsUsdPattern}
 */
function createCentsSatsUsdPattern(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} ChangeHalvedTotalPattern
 * @property {BtcCentsSatsUsdPattern} change1m
 * @property {BtcCentsSatsUsdPattern} halved
 * @property {BtcCentsSatsUsdPattern} total
 */

/**
 * Create a ChangeHalvedTotalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ChangeHalvedTotalPattern}
 */
function createChangeHalvedTotalPattern(client, acc) {
  return {
    change1m: createBtcCentsSatsUsdPattern(client, _m(acc, 'change_1m')),
    halved: createBtcCentsSatsUsdPattern(client, _m(acc, 'halved')),
    total: createBtcCentsSatsUsdPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} _6bBlockTxindexPattern
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} _6b
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} block
 * @property {MetricPattern19<T>} txindex
 */

/**
 * Create a _6bBlockTxindexPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_6bBlockTxindexPattern<T>}
 */
function create_6bBlockTxindexPattern(client, acc) {
  return {
    _6b: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '6b')),
    block: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc),
    txindex: createMetricPattern19(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeHeightSumPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern18<T>} height
 * @property {_1m1w1y24hPattern<T>} sum
 */

/**
 * Create a CumulativeHeightSumPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CumulativeHeightSumPattern<T>}
 */
function createCumulativeHeightSumPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    height: createMetricPattern18(client, acc),
    sum: create_1m1w1y24hPattern(client, acc),
  };
}

/**
 * @typedef {Object} _1m1wPattern
 * @property {BpsPercentRatioPattern} _1m
 * @property {BpsPercentRatioPattern} _1w
 */

/**
 * Create a _1m1wPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1wPattern}
 */
function create_1m1wPattern(client, acc) {
  return {
    _1m: createBpsPercentRatioPattern(client, _m(acc, '1m')),
    _1w: createBpsPercentRatioPattern(client, _m(acc, '1w')),
  };
}

/**
 * @typedef {Object} _1m1wPattern2
 * @property {MetricPattern1<StoredF64>} _1m
 * @property {MetricPattern1<StoredF64>} _1w
 */

/**
 * Create a _1m1wPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1wPattern2}
 */
function create_1m1wPattern2(client, acc) {
  return {
    _1m: createMetricPattern1(client, _m(acc, '1m')),
    _1w: createMetricPattern1(client, _m(acc, '1w')),
  };
}

/**
 * @typedef {Object} BaseCumulativePattern
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 */

/**
 * Create a BaseCumulativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativePattern}
 */
function createBaseCumulativePattern(client, acc) {
  return {
    base: createBtcCentsSatsUsdPattern(client, acc),
    cumulative: createBtcCentsSatsUsdPattern(client, _m(acc, 'cumulative')),
  };
}

/**
 * @typedef {Object} BpsRatioPattern
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsRatioPattern}
 */
function createBpsRatioPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    ratio: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} CentsUsdPattern
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsUsdPattern}
 */
function createCentsUsdPattern(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} ChangeCountPattern
 * @property {MetricPattern1<StoredF64>} change1m
 * @property {MetricPattern1<StoredU64>} count
 */

/**
 * Create a ChangeCountPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ChangeCountPattern}
 */
function createChangeCountPattern(client, acc) {
  return {
    change1m: createMetricPattern1(client, _m(acc, 'change_1m')),
    count: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} MaxMinPattern
 * @property {CentsSatsUsdPattern} max
 * @property {CentsSatsUsdPattern} min
 */

/**
 * Create a MaxMinPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {MaxMinPattern}
 */
function createMaxMinPattern(client, acc) {
  return {
    max: createCentsSatsUsdPattern(client, _m(acc, 'max')),
    min: createCentsSatsUsdPattern(client, _m(acc, 'min')),
  };
}

/**
 * @typedef {Object} SdSmaPattern
 * @property {MetricPattern1<StoredF32>} sd
 * @property {MetricPattern1<StoredF32>} sma
 */

/**
 * Create a SdSmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SdSmaPattern}
 */
function createSdSmaPattern(client, acc) {
  return {
    sd: createMetricPattern1(client, _m(acc, 'sd_1y')),
    sma: createMetricPattern1(client, _m(acc, 'sma_1y')),
  };
}

/**
 * @typedef {Object} UtxoPattern
 * @property {MetricPattern1<StoredU64>} utxoCount
 * @property {MetricPattern1<StoredF64>} utxoCountChange1m
 */

/**
 * Create a UtxoPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UtxoPattern}
 */
function createUtxoPattern(client, acc) {
  return {
    utxoCount: createMetricPattern1(client, acc),
    utxoCountChange1m: createMetricPattern1(client, _m(acc, 'change_1m')),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeHeightPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern18<T>} height
 */

/**
 * Create a CumulativeHeightPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CumulativeHeightPattern<T>}
 */
function createCumulativeHeightPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    height: createMetricPattern18(client, acc),
  };
}

/**
 * @typedef {Object} _2wPattern
 * @property {BtcCentsSatsUsdPattern} _2w
 */

/**
 * Create a _2wPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_2wPattern}
 */
function create_2wPattern(client, acc) {
  return {
    _2w: createBtcCentsSatsUsdPattern(client, acc),
  };
}

// Catalog tree typedefs

/**
 * @typedef {Object} MetricsTree
 * @property {MetricsTree_Blocks} blocks
 * @property {MetricsTree_Transactions} transactions
 * @property {MetricsTree_Inputs} inputs
 * @property {MetricsTree_Outputs} outputs
 * @property {MetricsTree_Addresses} addresses
 * @property {MetricsTree_Scripts} scripts
 * @property {MetricsTree_Mining} mining
 * @property {MetricsTree_Positions} positions
 * @property {MetricsTree_Cointime} cointime
 * @property {MetricsTree_Constants} constants
 * @property {MetricsTree_Indexes} indexes
 * @property {MetricsTree_Market} market
 * @property {MetricsTree_Pools} pools
 * @property {MetricsTree_Prices} prices
 * @property {MetricsTree_Distribution} distribution
 * @property {MetricsTree_Supply} supply
 */

/**
 * @typedef {Object} MetricsTree_Blocks
 * @property {MetricPattern18<BlockHash>} blockhash
 * @property {MetricsTree_Blocks_Difficulty} difficulty
 * @property {MetricsTree_Blocks_Time} time
 * @property {MetricPattern18<StoredU64>} totalSize
 * @property {MetricsTree_Blocks_Weight} weight
 * @property {MetricsTree_Blocks_Count} count
 * @property {AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<Timestamp>} interval
 * @property {MetricsTree_Blocks_Halving} halving
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} vbytes
 * @property {MetricsTree_Blocks_Size} size
 * @property {BpsPercentRatioPattern2} fullness
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Difficulty
 * @property {MetricPattern1<StoredF64>} raw
 * @property {MetricPattern1<StoredF64>} asHash
 * @property {BpsPercentRatioPattern} adjustment
 * @property {MetricPattern1<DifficultyEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextAdjustment
 * @property {MetricPattern1<StoredF32>} daysBeforeNextAdjustment
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time
 * @property {MetricPattern1<Timestamp>} timestamp
 * @property {MetricPattern18<Date>} date
 * @property {MetricPattern18<Timestamp>} timestampMonotonic
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Weight
 * @property {MetricPattern18<Weight>} base
 * @property {MetricPattern1<Weight>} cumulative
 * @property {_1m1w1y24hPattern<Weight>} sum
 * @property {_1m1w1y24hPattern<Weight>} average
 * @property {_1m1w1y24hPattern<Weight>} min
 * @property {_1m1w1y24hPattern<Weight>} max
 * @property {_1m1w1y24hPattern<Weight>} pct10
 * @property {_1m1w1y24hPattern<Weight>} pct25
 * @property {_1m1w1y24hPattern<Weight>} median
 * @property {_1m1w1y24hPattern<Weight>} pct75
 * @property {_1m1w1y24hPattern<Weight>} pct90
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Count
 * @property {MetricPattern1<StoredU64>} blockCountTarget
 * @property {CumulativeHeightSumPattern<StoredU32>} blockCount
 * @property {_1m1w1y24hPattern<StoredU32>} blockCountSum
 * @property {MetricPattern18<Height>} height1hAgo
 * @property {MetricPattern18<Height>} height24hAgo
 * @property {MetricPattern18<Height>} height3dAgo
 * @property {MetricPattern18<Height>} height1wAgo
 * @property {MetricPattern18<Height>} height8dAgo
 * @property {MetricPattern18<Height>} height9dAgo
 * @property {MetricPattern18<Height>} height12dAgo
 * @property {MetricPattern18<Height>} height13dAgo
 * @property {MetricPattern18<Height>} height2wAgo
 * @property {MetricPattern18<Height>} height21dAgo
 * @property {MetricPattern18<Height>} height26dAgo
 * @property {MetricPattern18<Height>} height1mAgo
 * @property {MetricPattern18<Height>} height34dAgo
 * @property {MetricPattern18<Height>} height55dAgo
 * @property {MetricPattern18<Height>} height2mAgo
 * @property {MetricPattern18<Height>} height9wAgo
 * @property {MetricPattern18<Height>} height12wAgo
 * @property {MetricPattern18<Height>} height89dAgo
 * @property {MetricPattern18<Height>} height3mAgo
 * @property {MetricPattern18<Height>} height14wAgo
 * @property {MetricPattern18<Height>} height111dAgo
 * @property {MetricPattern18<Height>} height144dAgo
 * @property {MetricPattern18<Height>} height6mAgo
 * @property {MetricPattern18<Height>} height26wAgo
 * @property {MetricPattern18<Height>} height200dAgo
 * @property {MetricPattern18<Height>} height9mAgo
 * @property {MetricPattern18<Height>} height350dAgo
 * @property {MetricPattern18<Height>} height12mAgo
 * @property {MetricPattern18<Height>} height1yAgo
 * @property {MetricPattern18<Height>} height14mAgo
 * @property {MetricPattern18<Height>} height2yAgo
 * @property {MetricPattern18<Height>} height26mAgo
 * @property {MetricPattern18<Height>} height3yAgo
 * @property {MetricPattern18<Height>} height200wAgo
 * @property {MetricPattern18<Height>} height4yAgo
 * @property {MetricPattern18<Height>} height5yAgo
 * @property {MetricPattern18<Height>} height6yAgo
 * @property {MetricPattern18<Height>} height8yAgo
 * @property {MetricPattern18<Height>} height9yAgo
 * @property {MetricPattern18<Height>} height10yAgo
 * @property {MetricPattern18<Height>} height12yAgo
 * @property {MetricPattern18<Height>} height14yAgo
 * @property {MetricPattern18<Height>} height26yAgo
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Halving
 * @property {MetricPattern1<HalvingEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextHalving
 * @property {MetricPattern1<StoredF32>} daysBeforeNextHalving
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Size
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {_1m1w1y24hPattern<StoredU64>} sum
 * @property {_1m1w1y24hPattern<StoredU64>} average
 * @property {_1m1w1y24hPattern<StoredU64>} min
 * @property {_1m1w1y24hPattern<StoredU64>} max
 * @property {_1m1w1y24hPattern<StoredU64>} pct10
 * @property {_1m1w1y24hPattern<StoredU64>} pct25
 * @property {_1m1w1y24hPattern<StoredU64>} median
 * @property {_1m1w1y24hPattern<StoredU64>} pct75
 * @property {_1m1w1y24hPattern<StoredU64>} pct90
 */

/**
 * @typedef {Object} MetricsTree_Transactions
 * @property {MetricPattern18<TxIndex>} firstTxindex
 * @property {MetricPattern19<Height>} height
 * @property {MetricPattern19<Txid>} txid
 * @property {MetricPattern19<TxVersion>} txversion
 * @property {MetricPattern19<RawLockTime>} rawlocktime
 * @property {MetricPattern19<StoredU32>} baseSize
 * @property {MetricPattern19<StoredU32>} totalSize
 * @property {MetricPattern19<StoredBool>} isExplicitlyRbf
 * @property {MetricPattern19<TxInIndex>} firstTxinindex
 * @property {MetricPattern19<TxOutIndex>} firstTxoutindex
 * @property {MetricsTree_Transactions_Count} count
 * @property {MetricsTree_Transactions_Size} size
 * @property {MetricsTree_Transactions_Fees} fees
 * @property {MetricsTree_Transactions_Versions} versions
 * @property {MetricsTree_Transactions_Volume} volume
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Count
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} txCount
 * @property {MetricPattern19<StoredBool>} isCoinbase
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Size
 * @property {_6bBlockTxindexPattern<VSize>} vsize
 * @property {_6bBlockTxindexPattern<Weight>} weight
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees
 * @property {MetricPattern19<Sats>} inputValue
 * @property {MetricPattern19<Sats>} outputValue
 * @property {_6bBlockTxindexPattern<Sats>} fee
 * @property {_6bBlockTxindexPattern<FeeRate>} feeRate
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Versions
 * @property {CumulativeHeightSumPattern<StoredU64>} v1
 * @property {CumulativeHeightSumPattern<StoredU64>} v2
 * @property {CumulativeHeightSumPattern<StoredU64>} v3
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Volume
 * @property {_1m1w1y24hBtcCentsSatsUsdPattern} sentSum
 * @property {_1m1w1y24hBtcCentsSatsUsdPattern} receivedSum
 * @property {BtcCentsSatsUsdPattern} annualizedVolume
 * @property {MetricPattern1<StoredF32>} txPerSec
 * @property {MetricPattern1<StoredF32>} outputsPerSec
 * @property {MetricPattern1<StoredF32>} inputsPerSec
 */

/**
 * @typedef {Object} MetricsTree_Inputs
 * @property {MetricPattern18<TxInIndex>} firstTxinindex
 * @property {MetricPattern20<OutPoint>} outpoint
 * @property {MetricPattern20<TxIndex>} txindex
 * @property {MetricPattern20<OutputType>} outputtype
 * @property {MetricPattern20<TypeIndex>} typeindex
 * @property {MetricsTree_Inputs_Spent} spent
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} count
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Spent
 * @property {MetricPattern20<TxOutIndex>} txoutindex
 * @property {MetricPattern20<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Outputs
 * @property {MetricPattern18<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern21<Sats>} value
 * @property {MetricPattern21<OutputType>} outputtype
 * @property {MetricPattern21<TypeIndex>} typeindex
 * @property {MetricPattern21<TxIndex>} txindex
 * @property {MetricsTree_Outputs_Spent} spent
 * @property {MetricsTree_Outputs_Count} count
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Spent
 * @property {MetricPattern21<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Count
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} totalCount
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * @typedef {Object} MetricsTree_Addresses
 * @property {MetricPattern18<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {MetricPattern18<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {MetricPattern18<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {MetricPattern18<P2SHAddressIndex>} firstP2shaddressindex
 * @property {MetricPattern18<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {MetricPattern18<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {MetricPattern18<P2TRAddressIndex>} firstP2traddressindex
 * @property {MetricPattern18<P2AAddressIndex>} firstP2aaddressindex
 * @property {MetricPattern27<P2PK65Bytes>} p2pk65bytes
 * @property {MetricPattern26<P2PK33Bytes>} p2pk33bytes
 * @property {MetricPattern28<P2PKHBytes>} p2pkhbytes
 * @property {MetricPattern29<P2SHBytes>} p2shbytes
 * @property {MetricPattern31<P2WPKHBytes>} p2wpkhbytes
 * @property {MetricPattern32<P2WSHBytes>} p2wshbytes
 * @property {MetricPattern30<P2TRBytes>} p2trbytes
 * @property {MetricPattern24<P2ABytes>} p2abytes
 */

/**
 * @typedef {Object} MetricsTree_Scripts
 * @property {MetricPattern18<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {MetricPattern18<OpReturnIndex>} firstOpreturnindex
 * @property {MetricPattern18<P2MSOutputIndex>} firstP2msoutputindex
 * @property {MetricPattern18<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricPattern22<TxIndex>} emptyToTxindex
 * @property {MetricPattern23<TxIndex>} opreturnToTxindex
 * @property {MetricPattern25<TxIndex>} p2msToTxindex
 * @property {MetricPattern33<TxIndex>} unknownToTxindex
 * @property {MetricsTree_Scripts_Count} count
 * @property {MetricsTree_Scripts_Value} value
 * @property {MetricsTree_Scripts_Adoption} adoption
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Count
 * @property {CumulativeHeightSumPattern<StoredU64>} p2a
 * @property {CumulativeHeightSumPattern<StoredU64>} p2ms
 * @property {CumulativeHeightSumPattern<StoredU64>} p2pk33
 * @property {CumulativeHeightSumPattern<StoredU64>} p2pk65
 * @property {CumulativeHeightSumPattern<StoredU64>} p2pkh
 * @property {CumulativeHeightSumPattern<StoredU64>} p2sh
 * @property {CumulativeHeightSumPattern<StoredU64>} p2tr
 * @property {CumulativeHeightSumPattern<StoredU64>} p2wpkh
 * @property {CumulativeHeightSumPattern<StoredU64>} p2wsh
 * @property {CumulativeHeightSumPattern<StoredU64>} opreturn
 * @property {CumulativeHeightSumPattern<StoredU64>} emptyoutput
 * @property {CumulativeHeightSumPattern<StoredU64>} unknownoutput
 * @property {CumulativeHeightSumPattern<StoredU64>} segwit
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value
 * @property {_1m1w1y24hBaseCumulativePattern} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Adoption
 * @property {BpsPercentRatioPattern} taproot
 * @property {BpsPercentRatioPattern} segwit
 */

/**
 * @typedef {Object} MetricsTree_Mining
 * @property {MetricsTree_Mining_Rewards} rewards
 * @property {MetricsTree_Mining_Hashrate} hashrate
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards
 * @property {_1m1w1y24hBaseCumulativePattern} coinbase
 * @property {_1m1w1y24hBaseCumulativePattern} subsidy
 * @property {_1m1w1y24hBaseCumulativePattern} fees
 * @property {BaseCumulativeSumPattern} unclaimedRewards
 * @property {BpsPercentRatioPattern} feeDominance
 * @property {_1m1w1y24hPattern2} feeDominanceRolling
 * @property {BpsPercentRatioPattern} subsidyDominance
 * @property {_1m1w1y24hPattern2} subsidyDominanceRolling
 * @property {CentsUsdPattern} subsidySma1y
 */

/**
 * @typedef {Object} MetricsTree_Mining_Hashrate
 * @property {MetricPattern1<StoredF64>} hashRate
 * @property {MetricPattern1<StoredF64>} hashRateSma1w
 * @property {MetricPattern1<StoredF64>} hashRateSma1m
 * @property {MetricPattern1<StoredF64>} hashRateSma2m
 * @property {MetricPattern1<StoredF64>} hashRateSma1y
 * @property {MetricPattern1<StoredF64>} hashRateAth
 * @property {BpsPercentRatioPattern} hashRateDrawdown
 * @property {MetricPattern1<StoredF32>} hashPriceThs
 * @property {MetricPattern1<StoredF32>} hashPriceThsMin
 * @property {MetricPattern1<StoredF32>} hashPricePhs
 * @property {MetricPattern1<StoredF32>} hashPricePhsMin
 * @property {BpsPercentRatioPattern} hashPriceRebound
 * @property {MetricPattern1<StoredF32>} hashValueThs
 * @property {MetricPattern1<StoredF32>} hashValueThsMin
 * @property {MetricPattern1<StoredF32>} hashValuePhs
 * @property {MetricPattern1<StoredF32>} hashValuePhsMin
 * @property {BpsPercentRatioPattern} hashValueRebound
 */

/**
 * @typedef {Object} MetricsTree_Positions
 * @property {MetricPattern18<BlkPosition>} blockPosition
 * @property {MetricPattern19<BlkPosition>} txPosition
 */

/**
 * @typedef {Object} MetricsTree_Cointime
 * @property {MetricsTree_Cointime_Activity} activity
 * @property {MetricsTree_Cointime_Supply} supply
 * @property {MetricsTree_Cointime_Value} value
 * @property {MetricsTree_Cointime_Cap} cap
 * @property {MetricsTree_Cointime_Pricing} pricing
 * @property {MetricsTree_Cointime_Adjusted} adjusted
 * @property {MetricsTree_Cointime_ReserveRisk} reserveRisk
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Activity
 * @property {CumulativeHeightSumPattern<StoredF64>} coinblocksCreated
 * @property {CumulativeHeightSumPattern<StoredF64>} coinblocksStored
 * @property {MetricPattern1<StoredF64>} liveliness
 * @property {MetricPattern1<StoredF64>} vaultedness
 * @property {MetricPattern1<StoredF64>} activityToVaultednessRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Supply
 * @property {BtcCentsSatsUsdPattern} vaultedSupply
 * @property {BtcCentsSatsUsdPattern} activeSupply
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Value
 * @property {CumulativeHeightSumPattern<StoredF64>} cointimeValueDestroyed
 * @property {CumulativeHeightSumPattern<StoredF64>} cointimeValueCreated
 * @property {CumulativeHeightSumPattern<StoredF64>} cointimeValueStored
 * @property {CumulativeHeightSumPattern<StoredF64>} vocdd
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Cap
 * @property {CentsUsdPattern} thermoCap
 * @property {CentsUsdPattern} investorCap
 * @property {CentsUsdPattern} vaultedCap
 * @property {CentsUsdPattern} activeCap
 * @property {CentsUsdPattern} cointimeCap
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Pricing
 * @property {CentsSatsUsdPattern} vaultedPrice
 * @property {BpsRatioPattern2} vaultedPriceRatio
 * @property {CentsSatsUsdPattern} activePrice
 * @property {BpsRatioPattern2} activePriceRatio
 * @property {CentsSatsUsdPattern} trueMarketMean
 * @property {BpsRatioPattern2} trueMarketMeanRatio
 * @property {CentsSatsUsdPattern} cointimePrice
 * @property {BpsRatioPattern2} cointimePriceRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Adjusted
 * @property {BpsPercentRatioPattern} cointimeAdjInflationRate
 * @property {MetricPattern1<StoredF64>} cointimeAdjTxVelocityBtc
 * @property {MetricPattern1<StoredF64>} cointimeAdjTxVelocityUsd
 */

/**
 * @typedef {Object} MetricsTree_Cointime_ReserveRisk
 * @property {MetricPattern18<StoredF64>} vocddMedian1y
 * @property {MetricPattern18<StoredF64>} hodlBank
 * @property {MetricPattern1<StoredF64>} reserveRisk
 */

/**
 * @typedef {Object} MetricsTree_Constants
 * @property {MetricPattern1<StoredU16>} constant0
 * @property {MetricPattern1<StoredU16>} constant1
 * @property {MetricPattern1<StoredU16>} constant2
 * @property {MetricPattern1<StoredU16>} constant3
 * @property {MetricPattern1<StoredU16>} constant4
 * @property {MetricPattern1<StoredU16>} constant20
 * @property {MetricPattern1<StoredU16>} constant30
 * @property {MetricPattern1<StoredF32>} constant382
 * @property {MetricPattern1<StoredU16>} constant50
 * @property {MetricPattern1<StoredF32>} constant618
 * @property {MetricPattern1<StoredU16>} constant70
 * @property {MetricPattern1<StoredU16>} constant80
 * @property {MetricPattern1<StoredU16>} constant100
 * @property {MetricPattern1<StoredU16>} constant600
 * @property {MetricPattern1<StoredI8>} constantMinus1
 * @property {MetricPattern1<StoredI8>} constantMinus2
 * @property {MetricPattern1<StoredI8>} constantMinus3
 * @property {MetricPattern1<StoredI8>} constantMinus4
 */

/**
 * @typedef {Object} MetricsTree_Indexes
 * @property {MetricsTree_Indexes_Address} address
 * @property {MetricsTree_Indexes_Height} height
 * @property {MetricsTree_Indexes_Difficultyepoch} difficultyepoch
 * @property {MetricsTree_Indexes_Halvingepoch} halvingepoch
 * @property {MetricsTree_Indexes_Minute10} minute10
 * @property {MetricsTree_Indexes_Minute30} minute30
 * @property {MetricsTree_Indexes_Hour1} hour1
 * @property {MetricsTree_Indexes_Hour4} hour4
 * @property {MetricsTree_Indexes_Hour12} hour12
 * @property {MetricsTree_Indexes_Day1} day1
 * @property {MetricsTree_Indexes_Day3} day3
 * @property {MetricsTree_Indexes_Week1} week1
 * @property {MetricsTree_Indexes_Month1} month1
 * @property {MetricsTree_Indexes_Month3} month3
 * @property {MetricsTree_Indexes_Month6} month6
 * @property {MetricsTree_Indexes_Year1} year1
 * @property {MetricsTree_Indexes_Year10} year10
 * @property {MetricsTree_Indexes_Txindex} txindex
 * @property {MetricsTree_Indexes_Txinindex} txinindex
 * @property {MetricsTree_Indexes_Txoutindex} txoutindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address
 * @property {MetricsTree_Indexes_Address_P2pk33} p2pk33
 * @property {MetricsTree_Indexes_Address_P2pk65} p2pk65
 * @property {MetricsTree_Indexes_Address_P2pkh} p2pkh
 * @property {MetricsTree_Indexes_Address_P2sh} p2sh
 * @property {MetricsTree_Indexes_Address_P2tr} p2tr
 * @property {MetricsTree_Indexes_Address_P2wpkh} p2wpkh
 * @property {MetricsTree_Indexes_Address_P2wsh} p2wsh
 * @property {MetricsTree_Indexes_Address_P2a} p2a
 * @property {MetricsTree_Indexes_Address_P2ms} p2ms
 * @property {MetricsTree_Indexes_Address_Empty} empty
 * @property {MetricsTree_Indexes_Address_Unknown} unknown
 * @property {MetricsTree_Indexes_Address_Opreturn} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pk33
 * @property {MetricPattern26<P2PK33AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pk65
 * @property {MetricPattern27<P2PK65AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pkh
 * @property {MetricPattern28<P2PKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2sh
 * @property {MetricPattern29<P2SHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2tr
 * @property {MetricPattern30<P2TRAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wpkh
 * @property {MetricPattern31<P2WPKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wsh
 * @property {MetricPattern32<P2WSHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2a
 * @property {MetricPattern24<P2AAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2ms
 * @property {MetricPattern25<P2MSOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Empty
 * @property {MetricPattern22<EmptyOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Unknown
 * @property {MetricPattern33<UnknownOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Opreturn
 * @property {MetricPattern23<OpReturnIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Height
 * @property {MetricPattern18<Height>} identity
 * @property {MetricPattern18<Minute10>} minute10
 * @property {MetricPattern18<Minute30>} minute30
 * @property {MetricPattern18<Hour1>} hour1
 * @property {MetricPattern18<Hour4>} hour4
 * @property {MetricPattern18<Hour12>} hour12
 * @property {MetricPattern18<Day1>} day1
 * @property {MetricPattern18<Day3>} day3
 * @property {MetricPattern18<DifficultyEpoch>} difficultyepoch
 * @property {MetricPattern18<HalvingEpoch>} halvingepoch
 * @property {MetricPattern18<Week1>} week1
 * @property {MetricPattern18<Month1>} month1
 * @property {MetricPattern18<Month3>} month3
 * @property {MetricPattern18<Month6>} month6
 * @property {MetricPattern18<Year1>} year1
 * @property {MetricPattern18<Year10>} year10
 * @property {MetricPattern18<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Difficultyepoch
 * @property {MetricPattern17<DifficultyEpoch>} identity
 * @property {MetricPattern17<Height>} firstHeight
 * @property {MetricPattern17<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Halvingepoch
 * @property {MetricPattern16<HalvingEpoch>} identity
 * @property {MetricPattern16<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute10
 * @property {MetricPattern3<Minute10>} identity
 * @property {MetricPattern3<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute30
 * @property {MetricPattern4<Minute30>} identity
 * @property {MetricPattern4<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour1
 * @property {MetricPattern5<Hour1>} identity
 * @property {MetricPattern5<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour4
 * @property {MetricPattern6<Hour4>} identity
 * @property {MetricPattern6<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour12
 * @property {MetricPattern7<Hour12>} identity
 * @property {MetricPattern7<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Day1
 * @property {MetricPattern8<Day1>} identity
 * @property {MetricPattern8<Date>} date
 * @property {MetricPattern8<Height>} firstHeight
 * @property {MetricPattern8<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Day3
 * @property {MetricPattern9<Day3>} identity
 * @property {MetricPattern9<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Week1
 * @property {MetricPattern10<Week1>} identity
 * @property {MetricPattern10<Date>} date
 * @property {MetricPattern10<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month1
 * @property {MetricPattern11<Month1>} identity
 * @property {MetricPattern11<Date>} date
 * @property {MetricPattern11<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month3
 * @property {MetricPattern12<Month3>} identity
 * @property {MetricPattern12<Date>} date
 * @property {MetricPattern12<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month6
 * @property {MetricPattern13<Month6>} identity
 * @property {MetricPattern13<Date>} date
 * @property {MetricPattern13<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Year1
 * @property {MetricPattern14<Year1>} identity
 * @property {MetricPattern14<Date>} date
 * @property {MetricPattern14<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Year10
 * @property {MetricPattern15<Year10>} identity
 * @property {MetricPattern15<Date>} date
 * @property {MetricPattern15<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txindex
 * @property {MetricPattern19<TxIndex>} identity
 * @property {MetricPattern19<StoredU64>} inputCount
 * @property {MetricPattern19<StoredU64>} outputCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txinindex
 * @property {MetricPattern20<TxInIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txoutindex
 * @property {MetricPattern21<TxOutIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Market
 * @property {MetricsTree_Market_Ath} ath
 * @property {MetricsTree_Market_Lookback} lookback
 * @property {MetricsTree_Market_Returns} returns
 * @property {MetricsTree_Market_Volatility} volatility
 * @property {MetricsTree_Market_Range} range
 * @property {MetricsTree_Market_MovingAverage} movingAverage
 * @property {MetricsTree_Market_Dca} dca
 * @property {MetricsTree_Market_Indicators} indicators
 */

/**
 * @typedef {Object} MetricsTree_Market_Ath
 * @property {CentsSatsUsdPattern} priceAth
 * @property {BpsPercentRatioPattern} priceDrawdown
 * @property {MetricPattern1<StoredF32>} daysSincePriceAth
 * @property {MetricPattern2<StoredF32>} yearsSincePriceAth
 * @property {MetricPattern1<StoredF32>} maxDaysBetweenPriceAth
 * @property {MetricPattern2<StoredF32>} maxYearsBetweenPriceAth
 */

/**
 * @typedef {Object} MetricsTree_Market_Lookback
 * @property {CentsSatsUsdPattern} _24h
 * @property {CentsSatsUsdPattern} _1w
 * @property {CentsSatsUsdPattern} _1m
 * @property {CentsSatsUsdPattern} _3m
 * @property {CentsSatsUsdPattern} _6m
 * @property {CentsSatsUsdPattern} _1y
 * @property {CentsSatsUsdPattern} _2y
 * @property {CentsSatsUsdPattern} _3y
 * @property {CentsSatsUsdPattern} _4y
 * @property {CentsSatsUsdPattern} _5y
 * @property {CentsSatsUsdPattern} _6y
 * @property {CentsSatsUsdPattern} _8y
 * @property {CentsSatsUsdPattern} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns
 * @property {MetricsTree_Market_Returns_PriceReturn} priceReturn
 * @property {_10y2y3y4y5y6y8yPattern} priceCagr
 * @property {MetricsTree_Market_Returns_PriceReturn24hSd1w} priceReturn24hSd1w
 * @property {MetricsTree_Market_Returns_PriceReturn24hSd1m} priceReturn24hSd1m
 * @property {SdSmaPattern} priceReturn24hSd1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturn
 * @property {BpsPercentRatioPattern} _24h
 * @property {BpsPercentRatioPattern} _1w
 * @property {BpsPercentRatioPattern} _1m
 * @property {BpsPercentRatioPattern} _3m
 * @property {BpsPercentRatioPattern} _6m
 * @property {BpsPercentRatioPattern} _1y
 * @property {BpsPercentRatioPattern} _2y
 * @property {BpsPercentRatioPattern} _3y
 * @property {BpsPercentRatioPattern} _4y
 * @property {BpsPercentRatioPattern} _5y
 * @property {BpsPercentRatioPattern} _6y
 * @property {BpsPercentRatioPattern} _8y
 * @property {BpsPercentRatioPattern} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturn24hSd1w
 * @property {MetricPattern1<StoredF32>} sma
 * @property {MetricPattern1<StoredF32>} sd
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturn24hSd1m
 * @property {MetricPattern1<StoredF32>} sma
 * @property {MetricPattern1<StoredF32>} sd
 */

/**
 * @typedef {Object} MetricsTree_Market_Volatility
 * @property {MetricPattern1<StoredF32>} priceVolatility1w
 * @property {MetricPattern1<StoredF32>} priceVolatility1m
 * @property {MetricPattern1<StoredF32>} priceVolatility1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Range
 * @property {CentsSatsUsdPattern} priceMin1w
 * @property {CentsSatsUsdPattern} priceMax1w
 * @property {CentsSatsUsdPattern} priceMin2w
 * @property {CentsSatsUsdPattern} priceMax2w
 * @property {CentsSatsUsdPattern} priceMin1m
 * @property {CentsSatsUsdPattern} priceMax1m
 * @property {CentsSatsUsdPattern} priceMin1y
 * @property {CentsSatsUsdPattern} priceMax1y
 * @property {MetricPattern1<StoredF32>} priceTrueRange
 * @property {MetricPattern1<StoredF32>} priceTrueRangeSum2w
 * @property {BpsPercentRatioPattern} priceChoppinessIndex2w
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage
 * @property {BpsPriceRatioPattern} priceSma1w
 * @property {BpsPriceRatioPattern} priceSma8d
 * @property {BpsPriceRatioPattern} priceSma13d
 * @property {BpsPriceRatioPattern} priceSma21d
 * @property {BpsPriceRatioPattern} priceSma1m
 * @property {BpsPriceRatioPattern} priceSma34d
 * @property {BpsPriceRatioPattern} priceSma55d
 * @property {BpsPriceRatioPattern} priceSma89d
 * @property {BpsPriceRatioPattern} priceSma111d
 * @property {BpsPriceRatioPattern} priceSma144d
 * @property {BpsPriceRatioPattern} priceSma200d
 * @property {BpsPriceRatioPattern} priceSma350d
 * @property {BpsPriceRatioPattern} priceSma1y
 * @property {BpsPriceRatioPattern} priceSma2y
 * @property {BpsPriceRatioPattern} priceSma200w
 * @property {BpsPriceRatioPattern} priceSma4y
 * @property {BpsPriceRatioPattern} priceEma1w
 * @property {BpsPriceRatioPattern} priceEma8d
 * @property {BpsPriceRatioPattern} priceEma12d
 * @property {BpsPriceRatioPattern} priceEma13d
 * @property {BpsPriceRatioPattern} priceEma21d
 * @property {BpsPriceRatioPattern} priceEma26d
 * @property {BpsPriceRatioPattern} priceEma1m
 * @property {BpsPriceRatioPattern} priceEma34d
 * @property {BpsPriceRatioPattern} priceEma55d
 * @property {BpsPriceRatioPattern} priceEma89d
 * @property {BpsPriceRatioPattern} priceEma144d
 * @property {BpsPriceRatioPattern} priceEma200d
 * @property {BpsPriceRatioPattern} priceEma1y
 * @property {BpsPriceRatioPattern} priceEma2y
 * @property {BpsPriceRatioPattern} priceEma200w
 * @property {BpsPriceRatioPattern} priceEma4y
 * @property {CentsSatsUsdPattern} priceSma200dX24
 * @property {CentsSatsUsdPattern} priceSma200dX08
 * @property {CentsSatsUsdPattern} priceSma350dX2
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca
 * @property {MetricPattern18<Sats>} dcaSatsPerDay
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} periodStack
 * @property {MetricsTree_Market_Dca_PeriodCostBasis} periodCostBasis
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2} periodReturn
 * @property {_10y2y3y4y5y6y8yPattern} periodCagr
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} periodLumpSumStack
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2} periodLumpSumReturn
 * @property {MetricsTree_Market_Dca_ClassStack} classStack
 * @property {MetricsTree_Market_Dca_ClassCostBasis} classCostBasis
 * @property {MetricsTree_Market_Dca_ClassReturn} classReturn
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_PeriodCostBasis
 * @property {CentsSatsUsdPattern} _1w
 * @property {CentsSatsUsdPattern} _1m
 * @property {CentsSatsUsdPattern} _3m
 * @property {CentsSatsUsdPattern} _6m
 * @property {CentsSatsUsdPattern} _1y
 * @property {CentsSatsUsdPattern} _2y
 * @property {CentsSatsUsdPattern} _3y
 * @property {CentsSatsUsdPattern} _4y
 * @property {CentsSatsUsdPattern} _5y
 * @property {CentsSatsUsdPattern} _6y
 * @property {CentsSatsUsdPattern} _8y
 * @property {CentsSatsUsdPattern} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassStack
 * @property {BtcCentsSatsUsdPattern} from2015
 * @property {BtcCentsSatsUsdPattern} from2016
 * @property {BtcCentsSatsUsdPattern} from2017
 * @property {BtcCentsSatsUsdPattern} from2018
 * @property {BtcCentsSatsUsdPattern} from2019
 * @property {BtcCentsSatsUsdPattern} from2020
 * @property {BtcCentsSatsUsdPattern} from2021
 * @property {BtcCentsSatsUsdPattern} from2022
 * @property {BtcCentsSatsUsdPattern} from2023
 * @property {BtcCentsSatsUsdPattern} from2024
 * @property {BtcCentsSatsUsdPattern} from2025
 * @property {BtcCentsSatsUsdPattern} from2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassCostBasis
 * @property {CentsSatsUsdPattern} from2015
 * @property {CentsSatsUsdPattern} from2016
 * @property {CentsSatsUsdPattern} from2017
 * @property {CentsSatsUsdPattern} from2018
 * @property {CentsSatsUsdPattern} from2019
 * @property {CentsSatsUsdPattern} from2020
 * @property {CentsSatsUsdPattern} from2021
 * @property {CentsSatsUsdPattern} from2022
 * @property {CentsSatsUsdPattern} from2023
 * @property {CentsSatsUsdPattern} from2024
 * @property {CentsSatsUsdPattern} from2025
 * @property {CentsSatsUsdPattern} from2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassReturn
 * @property {BpsPercentRatioPattern} from2015
 * @property {BpsPercentRatioPattern} from2016
 * @property {BpsPercentRatioPattern} from2017
 * @property {BpsPercentRatioPattern} from2018
 * @property {BpsPercentRatioPattern} from2019
 * @property {BpsPercentRatioPattern} from2020
 * @property {BpsPercentRatioPattern} from2021
 * @property {BpsPercentRatioPattern} from2022
 * @property {BpsPercentRatioPattern} from2023
 * @property {BpsPercentRatioPattern} from2024
 * @property {BpsPercentRatioPattern} from2025
 * @property {BpsPercentRatioPattern} from2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators
 * @property {BpsRatioPattern} puellMultiple
 * @property {BpsRatioPattern} nvt
 * @property {MetricsTree_Market_Indicators_Rsi} rsi
 * @property {BpsPercentRatioPattern} stochK
 * @property {BpsPercentRatioPattern} stochD
 * @property {BpsRatioPattern} piCycle
 * @property {MetricsTree_Market_Indicators_Macd} macd
 * @property {BpsPercentRatioPattern} gini
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi
 * @property {AverageGainsLossesRsiStochPattern} _24h
 * @property {MetricsTree_Market_Indicators_Rsi_1w} _1w
 * @property {MetricsTree_Market_Indicators_Rsi_1m} _1m
 * @property {MetricsTree_Market_Indicators_Rsi_1y} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1w
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern} rsi
 * @property {BpsPercentRatioPattern} rsiMin
 * @property {BpsPercentRatioPattern} rsiMax
 * @property {BpsPercentRatioPattern} stochRsi
 * @property {BpsPercentRatioPattern} stochRsiK
 * @property {BpsPercentRatioPattern} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1m
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern} rsi
 * @property {BpsPercentRatioPattern} rsiMin
 * @property {BpsPercentRatioPattern} rsiMax
 * @property {BpsPercentRatioPattern} stochRsi
 * @property {BpsPercentRatioPattern} stochRsiK
 * @property {BpsPercentRatioPattern} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1y
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern} rsi
 * @property {BpsPercentRatioPattern} rsiMin
 * @property {BpsPercentRatioPattern} rsiMax
 * @property {BpsPercentRatioPattern} stochRsi
 * @property {BpsPercentRatioPattern} stochRsiK
 * @property {BpsPercentRatioPattern} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd
 * @property {EmaHistogramLineSignalPattern} _24h
 * @property {MetricsTree_Market_Indicators_Macd_1w} _1w
 * @property {MetricsTree_Market_Indicators_Macd_1m} _1m
 * @property {MetricsTree_Market_Indicators_Macd_1y} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1w
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1m
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1y
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Pools
 * @property {MetricPattern18<PoolSlug>} heightToPool
 * @property {MetricsTree_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} MetricsTree_Pools_Vecs
 * @property {BlocksDominanceRewardsPattern} unknown
 * @property {BlocksDominanceRewardsPattern} blockfills
 * @property {BlocksDominanceRewardsPattern} ultimuspool
 * @property {BlocksDominanceRewardsPattern} terrapool
 * @property {BlocksDominanceRewardsPattern} luxor
 * @property {BlocksDominanceRewardsPattern} onethash
 * @property {BlocksDominanceRewardsPattern} btccom
 * @property {BlocksDominanceRewardsPattern} bitfarms
 * @property {BlocksDominanceRewardsPattern} huobipool
 * @property {BlocksDominanceRewardsPattern} wayicn
 * @property {BlocksDominanceRewardsPattern} canoepool
 * @property {BlocksDominanceRewardsPattern} btctop
 * @property {BlocksDominanceRewardsPattern} bitcoincom
 * @property {BlocksDominanceRewardsPattern} pool175btc
 * @property {BlocksDominanceRewardsPattern} gbminers
 * @property {BlocksDominanceRewardsPattern} axbt
 * @property {BlocksDominanceRewardsPattern} asicminer
 * @property {BlocksDominanceRewardsPattern} bitminter
 * @property {BlocksDominanceRewardsPattern} bitcoinrussia
 * @property {BlocksDominanceRewardsPattern} btcserv
 * @property {BlocksDominanceRewardsPattern} simplecoinus
 * @property {BlocksDominanceRewardsPattern} btcguild
 * @property {BlocksDominanceRewardsPattern} eligius
 * @property {BlocksDominanceRewardsPattern} ozcoin
 * @property {BlocksDominanceRewardsPattern} eclipsemc
 * @property {BlocksDominanceRewardsPattern} maxbtc
 * @property {BlocksDominanceRewardsPattern} triplemining
 * @property {BlocksDominanceRewardsPattern} coinlab
 * @property {BlocksDominanceRewardsPattern} pool50btc
 * @property {BlocksDominanceRewardsPattern} ghashio
 * @property {BlocksDominanceRewardsPattern} stminingcorp
 * @property {BlocksDominanceRewardsPattern} bitparking
 * @property {BlocksDominanceRewardsPattern} mmpool
 * @property {BlocksDominanceRewardsPattern} polmine
 * @property {BlocksDominanceRewardsPattern} kncminer
 * @property {BlocksDominanceRewardsPattern} bitalo
 * @property {BlocksDominanceRewardsPattern} f2pool
 * @property {BlocksDominanceRewardsPattern} hhtt
 * @property {BlocksDominanceRewardsPattern} megabigpower
 * @property {BlocksDominanceRewardsPattern} mtred
 * @property {BlocksDominanceRewardsPattern} nmcbit
 * @property {BlocksDominanceRewardsPattern} yourbtcnet
 * @property {BlocksDominanceRewardsPattern} givemecoins
 * @property {BlocksDominanceRewardsPattern} braiinspool
 * @property {BlocksDominanceRewardsPattern} antpool
 * @property {BlocksDominanceRewardsPattern} multicoinco
 * @property {BlocksDominanceRewardsPattern} bcpoolio
 * @property {BlocksDominanceRewardsPattern} cointerra
 * @property {BlocksDominanceRewardsPattern} kanopool
 * @property {BlocksDominanceRewardsPattern} solock
 * @property {BlocksDominanceRewardsPattern} ckpool
 * @property {BlocksDominanceRewardsPattern} nicehash
 * @property {BlocksDominanceRewardsPattern} bitclub
 * @property {BlocksDominanceRewardsPattern} bitcoinaffiliatenetwork
 * @property {BlocksDominanceRewardsPattern} btcc
 * @property {BlocksDominanceRewardsPattern} bwpool
 * @property {BlocksDominanceRewardsPattern} exxbw
 * @property {BlocksDominanceRewardsPattern} bitsolo
 * @property {BlocksDominanceRewardsPattern} bitfury
 * @property {BlocksDominanceRewardsPattern} twentyoneinc
 * @property {BlocksDominanceRewardsPattern} digitalbtc
 * @property {BlocksDominanceRewardsPattern} eightbaochi
 * @property {BlocksDominanceRewardsPattern} mybtccoinpool
 * @property {BlocksDominanceRewardsPattern} tbdice
 * @property {BlocksDominanceRewardsPattern} hashpool
 * @property {BlocksDominanceRewardsPattern} nexious
 * @property {BlocksDominanceRewardsPattern} bravomining
 * @property {BlocksDominanceRewardsPattern} hotpool
 * @property {BlocksDominanceRewardsPattern} okexpool
 * @property {BlocksDominanceRewardsPattern} bcmonster
 * @property {BlocksDominanceRewardsPattern} onehash
 * @property {BlocksDominanceRewardsPattern} bixin
 * @property {BlocksDominanceRewardsPattern} tatmaspool
 * @property {BlocksDominanceRewardsPattern} viabtc
 * @property {BlocksDominanceRewardsPattern} connectbtc
 * @property {BlocksDominanceRewardsPattern} batpool
 * @property {BlocksDominanceRewardsPattern} waterhole
 * @property {BlocksDominanceRewardsPattern} dcexploration
 * @property {BlocksDominanceRewardsPattern} dcex
 * @property {BlocksDominanceRewardsPattern} btpool
 * @property {BlocksDominanceRewardsPattern} fiftyeightcoin
 * @property {BlocksDominanceRewardsPattern} bitcoinindia
 * @property {BlocksDominanceRewardsPattern} shawnp0wers
 * @property {BlocksDominanceRewardsPattern} phashio
 * @property {BlocksDominanceRewardsPattern} rigpool
 * @property {BlocksDominanceRewardsPattern} haozhuzhu
 * @property {BlocksDominanceRewardsPattern} sevenpool
 * @property {BlocksDominanceRewardsPattern} miningkings
 * @property {BlocksDominanceRewardsPattern} hashbx
 * @property {BlocksDominanceRewardsPattern} dpool
 * @property {BlocksDominanceRewardsPattern} rawpool
 * @property {BlocksDominanceRewardsPattern} haominer
 * @property {BlocksDominanceRewardsPattern} helix
 * @property {BlocksDominanceRewardsPattern} bitcoinukraine
 * @property {BlocksDominanceRewardsPattern} poolin
 * @property {BlocksDominanceRewardsPattern} secretsuperstar
 * @property {BlocksDominanceRewardsPattern} tigerpoolnet
 * @property {BlocksDominanceRewardsPattern} sigmapoolcom
 * @property {BlocksDominanceRewardsPattern} okpooltop
 * @property {BlocksDominanceRewardsPattern} hummerpool
 * @property {BlocksDominanceRewardsPattern} tangpool
 * @property {BlocksDominanceRewardsPattern} bytepool
 * @property {BlocksDominanceRewardsPattern} spiderpool
 * @property {BlocksDominanceRewardsPattern} novablock
 * @property {BlocksDominanceRewardsPattern} miningcity
 * @property {BlocksDominanceRewardsPattern} binancepool
 * @property {BlocksDominanceRewardsPattern} minerium
 * @property {BlocksDominanceRewardsPattern} lubiancom
 * @property {BlocksDominanceRewardsPattern} okkong
 * @property {BlocksDominanceRewardsPattern} aaopool
 * @property {BlocksDominanceRewardsPattern} emcdpool
 * @property {BlocksDominanceRewardsPattern} foundryusa
 * @property {BlocksDominanceRewardsPattern} sbicrypto
 * @property {BlocksDominanceRewardsPattern} arkpool
 * @property {BlocksDominanceRewardsPattern} purebtccom
 * @property {BlocksDominanceRewardsPattern} marapool
 * @property {BlocksDominanceRewardsPattern} kucoinpool
 * @property {BlocksDominanceRewardsPattern} entrustcharitypool
 * @property {BlocksDominanceRewardsPattern} okminer
 * @property {BlocksDominanceRewardsPattern} titan
 * @property {BlocksDominanceRewardsPattern} pegapool
 * @property {BlocksDominanceRewardsPattern} btcnuggets
 * @property {BlocksDominanceRewardsPattern} cloudhashing
 * @property {BlocksDominanceRewardsPattern} digitalxmintsy
 * @property {BlocksDominanceRewardsPattern} telco214
 * @property {BlocksDominanceRewardsPattern} btcpoolparty
 * @property {BlocksDominanceRewardsPattern} multipool
 * @property {BlocksDominanceRewardsPattern} transactioncoinmining
 * @property {BlocksDominanceRewardsPattern} btcdig
 * @property {BlocksDominanceRewardsPattern} trickysbtcpool
 * @property {BlocksDominanceRewardsPattern} btcmp
 * @property {BlocksDominanceRewardsPattern} eobot
 * @property {BlocksDominanceRewardsPattern} unomp
 * @property {BlocksDominanceRewardsPattern} patels
 * @property {BlocksDominanceRewardsPattern} gogreenlight
 * @property {BlocksDominanceRewardsPattern} bitcoinindiapool
 * @property {BlocksDominanceRewardsPattern} ekanembtc
 * @property {BlocksDominanceRewardsPattern} canoe
 * @property {BlocksDominanceRewardsPattern} tiger
 * @property {BlocksDominanceRewardsPattern} onem1x
 * @property {BlocksDominanceRewardsPattern} zulupool
 * @property {BlocksDominanceRewardsPattern} secpool
 * @property {BlocksDominanceRewardsPattern} ocean
 * @property {BlocksDominanceRewardsPattern} whitepool
 * @property {BlocksDominanceRewardsPattern} wiz
 * @property {BlocksDominanceRewardsPattern} wk057
 * @property {BlocksDominanceRewardsPattern} futurebitapollosolo
 * @property {BlocksDominanceRewardsPattern} carbonnegative
 * @property {BlocksDominanceRewardsPattern} portlandhodl
 * @property {BlocksDominanceRewardsPattern} phoenix
 * @property {BlocksDominanceRewardsPattern} neopool
 * @property {BlocksDominanceRewardsPattern} maxipool
 * @property {BlocksDominanceRewardsPattern} bitfufupool
 * @property {BlocksDominanceRewardsPattern} gdpool
 * @property {BlocksDominanceRewardsPattern} miningdutch
 * @property {BlocksDominanceRewardsPattern} publicpool
 * @property {BlocksDominanceRewardsPattern} miningsquared
 * @property {BlocksDominanceRewardsPattern} innopolistech
 * @property {BlocksDominanceRewardsPattern} btclab
 * @property {BlocksDominanceRewardsPattern} parasite
 * @property {BlocksDominanceRewardsPattern} redrockpool
 * @property {BlocksDominanceRewardsPattern} est3lar
 */

/**
 * @typedef {Object} MetricsTree_Prices
 * @property {MetricsTree_Prices_Split} split
 * @property {MetricsTree_Prices_Ohlc} ohlc
 * @property {MetricsTree_Prices_Price} price
 */

/**
 * @typedef {Object} MetricsTree_Prices_Split
 * @property {CentsSatsUsdPattern2} open
 * @property {CentsSatsUsdPattern2} high
 * @property {CentsSatsUsdPattern2} low
 * @property {MetricsTree_Prices_Split_Close} close
 */

/**
 * @typedef {Object} MetricsTree_Prices_Split_Close
 * @property {MetricPattern2<Cents>} cents
 * @property {MetricPattern2<Dollars>} usd
 * @property {MetricPattern2<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Prices_Ohlc
 * @property {MetricPattern2<OHLCCents>} cents
 * @property {MetricPattern2<OHLCDollars>} usd
 * @property {MetricPattern2<OHLCSats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Prices_Price
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Distribution
 * @property {MetricPattern18<SupplyState>} supplyState
 * @property {MetricsTree_Distribution_AnyAddressIndexes} anyAddressIndexes
 * @property {MetricsTree_Distribution_AddressesData} addressesData
 * @property {MetricsTree_Distribution_UtxoCohorts} utxoCohorts
 * @property {MetricsTree_Distribution_AddressCohorts} addressCohorts
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} addrCount
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} emptyAddrCount
 * @property {MetricsTree_Distribution_AddressActivity} addressActivity
 * @property {MetricsTree_Distribution_TotalAddrCount} totalAddrCount
 * @property {MetricsTree_Distribution_NewAddrCount} newAddrCount
 * @property {MetricsTree_Distribution_GrowthRate} growthRate
 * @property {MetricPattern34<FundedAddressIndex>} fundedaddressindex
 * @property {MetricPattern35<EmptyAddressIndex>} emptyaddressindex
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AnyAddressIndexes
 * @property {MetricPattern24<AnyAddressIndex>} p2a
 * @property {MetricPattern26<AnyAddressIndex>} p2pk33
 * @property {MetricPattern27<AnyAddressIndex>} p2pk65
 * @property {MetricPattern28<AnyAddressIndex>} p2pkh
 * @property {MetricPattern29<AnyAddressIndex>} p2sh
 * @property {MetricPattern30<AnyAddressIndex>} p2tr
 * @property {MetricPattern31<AnyAddressIndex>} p2wpkh
 * @property {MetricPattern32<AnyAddressIndex>} p2wsh
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressesData
 * @property {MetricPattern34<FundedAddressData>} funded
 * @property {MetricPattern35<EmptyAddressData>} empty
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts
 * @property {MetricsTree_Distribution_UtxoCohorts_All} all
 * @property {MetricsTree_Distribution_UtxoCohorts_Sth} sth
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} lth
 * @property {MetricsTree_Distribution_UtxoCohorts_AgeRange} ageRange
 * @property {MetricsTree_Distribution_UtxoCohorts_MaxAge} maxAge
 * @property {MetricsTree_Distribution_UtxoCohorts_MinAge} minAge
 * @property {MetricsTree_Distribution_UtxoCohorts_GeAmount} geAmount
 * @property {MetricsTree_Distribution_UtxoCohorts_AmountRange} amountRange
 * @property {MetricsTree_Distribution_UtxoCohorts_LtAmount} ltAmount
 * @property {MetricsTree_Distribution_UtxoCohorts_Epoch} epoch
 * @property {MetricsTree_Distribution_UtxoCohorts_Class} class
 * @property {MetricsTree_Distribution_UtxoCohorts_Type} type
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All
 * @property {ChangeHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern} realized
 * @property {InvestedMaxMinPercentilesPattern} costBasis
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 * @property {MetricsTree_Distribution_UtxoCohorts_All_Relative} relative
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All_Relative
 * @property {BpsPercentRatioPattern} supplyInProfitRelToOwnSupply
 * @property {BpsPercentRatioPattern} supplyInLossRelToOwnSupply
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToMarketCap
 * @property {BpsPercentRatioPattern} unrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToMarketCap
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {BpsPercentRatioPattern} investedCapitalInProfitRelToRealizedCap
 * @property {BpsPercentRatioPattern} investedCapitalInLossRelToRealizedCap
 * @property {BpsPercentRatioPattern} unrealizedProfitRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} unrealizedLossRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} negUnrealizedLossRelToOwnGrossPnl
 * @property {BpsPercentRatioPattern} netUnrealizedPnlRelToOwnGrossPnl
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Sth
 * @property {ChangeHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSentPattern} activity
 * @property {AdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern} realized
 * @property {InvestedMaxMinPercentilesPattern} costBasis
 * @property {GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern} unrealized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern2} relative
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_AgeRange
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} upTo1h
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1hTo1d
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1dTo1w
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1wTo1m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1mTo2m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _2mTo3m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _3mTo4m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _4mTo5m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _5mTo6m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _6mTo1y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1yTo2y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _2yTo3y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _3yTo4y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _4yTo5y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _5yTo6y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _6yTo7y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _7yTo8y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _8yTo10y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10yTo12y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} _12yTo15y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern} from15y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MaxAge
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1w
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _2m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _3m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _4m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _5m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _6m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _2y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _3y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _4y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _5y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _6y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _7y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _8y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _12y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _15y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MinAge
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1d
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1w
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _3m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _4m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _5m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _6m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _3y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _4y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _5y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _6y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _7y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _8y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _12y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_GeAmount
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1sat
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kBtc
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_AmountRange
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _0sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1satTo10sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10satsTo100sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100satsTo1kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kSatsTo10kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kSatsTo100kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100kSatsTo1mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1mSatsTo10mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10mSatsTo1btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1btcTo10btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10btcTo100btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100btcTo1kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kBtcTo10kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kBtcTo100kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100kBtcOrMore
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_LtAmount
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _10kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _100kBtc
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Epoch
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _0
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _1
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _3
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _4
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Class
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2009
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2010
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2011
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2012
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2013
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2014
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2015
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2016
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2017
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2018
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2019
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2020
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2021
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2022
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2023
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2024
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2025
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} _2026
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Type
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pk65
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pk33
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pkh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2ms
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2sh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2wpkh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2wsh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2tr
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2a
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} unknown
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} empty
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts
 * @property {MetricsTree_Distribution_AddressCohorts_GeAmount} geAmount
 * @property {MetricsTree_Distribution_AddressCohorts_AmountRange} amountRange
 * @property {MetricsTree_Distribution_AddressCohorts_LtAmount} ltAmount
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_GeAmount
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1sat
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kBtc
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_AmountRange
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _0sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1satTo10sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10satsTo100sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100satsTo1kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kSatsTo10kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kSatsTo100kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100kSatsTo1mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1mSatsTo10mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10mSatsTo1btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1btcTo10btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10btcTo100btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100btcTo1kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kBtcTo10kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kBtcTo100kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100kBtcOrMore
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_LtAmount
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100sats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100kSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10mSats
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100btc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _1kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _10kBtc
 * @property {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern} _100kBtc
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressActivity
 * @property {BalanceBothReactivatedReceivingSendingPattern} all
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2pk65
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2pk33
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2pkh
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2sh
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2wpkh
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2wsh
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2tr
 * @property {BalanceBothReactivatedReceivingSendingPattern} p2a
 */

/**
 * @typedef {Object} MetricsTree_Distribution_TotalAddrCount
 * @property {MetricPattern1<StoredU64>} all
 * @property {MetricPattern1<StoredU64>} p2pk65
 * @property {MetricPattern1<StoredU64>} p2pk33
 * @property {MetricPattern1<StoredU64>} p2pkh
 * @property {MetricPattern1<StoredU64>} p2sh
 * @property {MetricPattern1<StoredU64>} p2wpkh
 * @property {MetricPattern1<StoredU64>} p2wsh
 * @property {MetricPattern1<StoredU64>} p2tr
 * @property {MetricPattern1<StoredU64>} p2a
 */

/**
 * @typedef {Object} MetricsTree_Distribution_NewAddrCount
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} all
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2pk65
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2pk33
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2pkh
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2sh
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2wpkh
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2wsh
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2tr
 * @property {AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern} p2a
 */

/**
 * @typedef {Object} MetricsTree_Distribution_GrowthRate
 * @property {BpsPercentRatioPattern2} all
 * @property {BpsPercentRatioPattern2} p2pk65
 * @property {BpsPercentRatioPattern2} p2pk33
 * @property {BpsPercentRatioPattern2} p2pkh
 * @property {BpsPercentRatioPattern2} p2sh
 * @property {BpsPercentRatioPattern2} p2wpkh
 * @property {BpsPercentRatioPattern2} p2wsh
 * @property {BpsPercentRatioPattern2} p2tr
 * @property {BpsPercentRatioPattern2} p2a
 */

/**
 * @typedef {Object} MetricsTree_Supply
 * @property {BtcCentsSatsUsdPattern} circulating
 * @property {MetricsTree_Supply_Burned} burned
 * @property {BpsPercentRatioPattern} inflationRate
 * @property {MetricsTree_Supply_Velocity} velocity
 * @property {MetricPattern1<Dollars>} marketCap
 * @property {BpsPercentRatioPattern} marketCapGrowthRate
 * @property {BpsPercentRatioPattern} realizedCapGrowthRate
 * @property {MetricPattern1<BasisPointsSigned32>} marketMinusRealizedCapGrowthRate
 */

/**
 * @typedef {Object} MetricsTree_Supply_Burned
 * @property {BaseCumulativeSumPattern} opreturn
 * @property {BaseCumulativeSumPattern} unspendable
 */

/**
 * @typedef {Object} MetricsTree_Supply_Velocity
 * @property {MetricPattern1<StoredF64>} btc
 * @property {MetricPattern1<StoredF64>} usd
 */

/**
 * Main BRK client with metrics tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
  VERSION = "v0.1.9";

  INDEXES = /** @type {const} */ ([
    "minute10",
    "minute30",
    "hour1",
    "hour4",
    "hour12",
    "day1",
    "day3",
    "week1",
    "month1",
    "month3",
    "month6",
    "year1",
    "year10",
    "halvingepoch",
    "difficultyepoch",
    "height",
    "txindex",
    "txinindex",
    "txoutindex",
    "emptyoutputindex",
    "opreturnindex",
    "p2aaddressindex",
    "p2msoutputindex",
    "p2pk33addressindex",
    "p2pk65addressindex",
    "p2pkhaddressindex",
    "p2shaddressindex",
    "p2traddressindex",
    "p2wpkhaddressindex",
    "p2wshaddressindex",
    "unknownoutputindex",
    "fundedaddressindex",
    "emptyaddressindex",
    "pairoutputindex"
  ]);

  POOL_ID_TO_POOL_NAME = /** @type {const} */ ({
    "unknown": "Unknown",
    "blockfills": "BlockFills",
    "ultimuspool": "ULTIMUSPOOL",
    "terrapool": "Terra Pool",
    "luxor": "Luxor",
    "onethash": "1THash",
    "btccom": "BTC.com",
    "bitfarms": "Bitfarms",
    "huobipool": "Huobi.pool",
    "wayicn": "WAYI.CN",
    "canoepool": "CanoePool",
    "btctop": "BTC.TOP",
    "bitcoincom": "Bitcoin.com",
    "pool175btc": "175btc",
    "gbminers": "GBMiners",
    "axbt": "A-XBT",
    "asicminer": "ASICMiner",
    "bitminter": "BitMinter",
    "bitcoinrussia": "BitcoinRussia",
    "btcserv": "BTCServ",
    "simplecoinus": "simplecoin.us",
    "btcguild": "BTC Guild",
    "eligius": "Eligius",
    "ozcoin": "OzCoin",
    "eclipsemc": "EclipseMC",
    "maxbtc": "MaxBTC",
    "triplemining": "TripleMining",
    "coinlab": "CoinLab",
    "pool50btc": "50BTC",
    "ghashio": "GHash.IO",
    "stminingcorp": "ST Mining Corp",
    "bitparking": "Bitparking",
    "mmpool": "mmpool",
    "polmine": "Polmine",
    "kncminer": "KnCMiner",
    "bitalo": "Bitalo",
    "f2pool": "F2Pool",
    "hhtt": "HHTT",
    "megabigpower": "MegaBigPower",
    "mtred": "Mt Red",
    "nmcbit": "NMCbit",
    "yourbtcnet": "Yourbtc.net",
    "givemecoins": "Give Me Coins",
    "braiinspool": "Braiins Pool",
    "antpool": "AntPool",
    "multicoinco": "MultiCoin.co",
    "bcpoolio": "bcpool.io",
    "cointerra": "Cointerra",
    "kanopool": "KanoPool",
    "solock": "Solo CK",
    "ckpool": "CKPool",
    "nicehash": "NiceHash",
    "bitclub": "BitClub",
    "bitcoinaffiliatenetwork": "Bitcoin Affiliate Network",
    "btcc": "BTCC",
    "bwpool": "BWPool",
    "exxbw": "EXX&BW",
    "bitsolo": "Bitsolo",
    "bitfury": "BitFury",
    "twentyoneinc": "21 Inc.",
    "digitalbtc": "digitalBTC",
    "eightbaochi": "8baochi",
    "mybtccoinpool": "myBTCcoin Pool",
    "tbdice": "TBDice",
    "hashpool": "HASHPOOL",
    "nexious": "Nexious",
    "bravomining": "Bravo Mining",
    "hotpool": "HotPool",
    "okexpool": "OKExPool",
    "bcmonster": "BCMonster",
    "onehash": "1Hash",
    "bixin": "Bixin",
    "tatmaspool": "TATMAS Pool",
    "viabtc": "ViaBTC",
    "connectbtc": "ConnectBTC",
    "batpool": "BATPOOL",
    "waterhole": "Waterhole",
    "dcexploration": "DCExploration",
    "dcex": "DCEX",
    "btpool": "BTPOOL",
    "fiftyeightcoin": "58COIN",
    "bitcoinindia": "Bitcoin India",
    "shawnp0wers": "shawnp0wers",
    "phashio": "PHash.IO",
    "rigpool": "RigPool",
    "haozhuzhu": "HAOZHUZHU",
    "sevenpool": "7pool",
    "miningkings": "MiningKings",
    "hashbx": "HashBX",
    "dpool": "DPOOL",
    "rawpool": "Rawpool",
    "haominer": "haominer",
    "helix": "Helix",
    "bitcoinukraine": "Bitcoin-Ukraine",
    "poolin": "Poolin",
    "secretsuperstar": "SecretSuperstar",
    "tigerpoolnet": "tigerpool.net",
    "sigmapoolcom": "Sigmapool.com",
    "okpooltop": "okpool.top",
    "hummerpool": "Hummerpool",
    "tangpool": "Tangpool",
    "bytepool": "BytePool",
    "spiderpool": "SpiderPool",
    "novablock": "NovaBlock",
    "miningcity": "MiningCity",
    "binancepool": "Binance Pool",
    "minerium": "Minerium",
    "lubiancom": "Lubian.com",
    "okkong": "OKKONG",
    "aaopool": "AAO Pool",
    "emcdpool": "EMCDPool",
    "foundryusa": "Foundry USA",
    "sbicrypto": "SBI Crypto",
    "arkpool": "ArkPool",
    "purebtccom": "PureBTC.COM",
    "marapool": "MARA Pool",
    "kucoinpool": "KuCoinPool",
    "entrustcharitypool": "Entrust Charity Pool",
    "okminer": "OKMINER",
    "titan": "Titan",
    "pegapool": "PEGA Pool",
    "btcnuggets": "BTC Nuggets",
    "cloudhashing": "CloudHashing",
    "digitalxmintsy": "digitalX Mintsy",
    "telco214": "Telco 214",
    "btcpoolparty": "BTC Pool Party",
    "multipool": "Multipool",
    "transactioncoinmining": "transactioncoinmining",
    "btcdig": "BTCDig",
    "trickysbtcpool": "Tricky's BTC Pool",
    "btcmp": "BTCMP",
    "eobot": "Eobot",
    "unomp": "UNOMP",
    "patels": "Patels",
    "gogreenlight": "GoGreenLight",
    "bitcoinindiapool": "BitcoinIndia",
    "ekanembtc": "EkanemBTC",
    "canoe": "CANOE",
    "tiger": "tiger",
    "onem1x": "1M1X",
    "zulupool": "Zulupool",
    "secpool": "SECPOOL",
    "ocean": "OCEAN",
    "whitepool": "WhitePool",
    "wiz": "wiz",
    "wk057": "wk057",
    "futurebitapollosolo": "FutureBit Apollo Solo",
    "carbonnegative": "Carbon Negative",
    "portlandhodl": "Portland.HODL",
    "phoenix": "Phoenix",
    "neopool": "Neopool",
    "maxipool": "MaxiPool",
    "bitfufupool": "BitFuFuPool",
    "gdpool": "GDPool",
    "miningdutch": "Mining-Dutch",
    "publicpool": "Public Pool",
    "miningsquared": "Mining Squared",
    "innopolistech": "Innopolis Tech",
    "btclab": "BTCLab",
    "parasite": "Parasite",
    "redrockpool": "RedRock Pool",
    "est3lar": "Est3lar"
  });

  TERM_NAMES = /** @type {const} */ ({
    "short": {
      "id": "sth",
      "short": "STH",
      "long": "Short Term Holders"
    },
    "long": {
      "id": "lth",
      "short": "LTH",
      "long": "Long Term Holders"
    }
  });

  EPOCH_NAMES = /** @type {const} */ ({
    "_0": {
      "id": "epoch_0",
      "short": "0",
      "long": "Epoch 0"
    },
    "_1": {
      "id": "epoch_1",
      "short": "1",
      "long": "Epoch 1"
    },
    "_2": {
      "id": "epoch_2",
      "short": "2",
      "long": "Epoch 2"
    },
    "_3": {
      "id": "epoch_3",
      "short": "3",
      "long": "Epoch 3"
    },
    "_4": {
      "id": "epoch_4",
      "short": "4",
      "long": "Epoch 4"
    }
  });

  CLASS_NAMES = /** @type {const} */ ({
    "_2009": {
      "id": "class_2009",
      "short": "2009",
      "long": "Class 2009"
    },
    "_2010": {
      "id": "class_2010",
      "short": "2010",
      "long": "Class 2010"
    },
    "_2011": {
      "id": "class_2011",
      "short": "2011",
      "long": "Class 2011"
    },
    "_2012": {
      "id": "class_2012",
      "short": "2012",
      "long": "Class 2012"
    },
    "_2013": {
      "id": "class_2013",
      "short": "2013",
      "long": "Class 2013"
    },
    "_2014": {
      "id": "class_2014",
      "short": "2014",
      "long": "Class 2014"
    },
    "_2015": {
      "id": "class_2015",
      "short": "2015",
      "long": "Class 2015"
    },
    "_2016": {
      "id": "class_2016",
      "short": "2016",
      "long": "Class 2016"
    },
    "_2017": {
      "id": "class_2017",
      "short": "2017",
      "long": "Class 2017"
    },
    "_2018": {
      "id": "class_2018",
      "short": "2018",
      "long": "Class 2018"
    },
    "_2019": {
      "id": "class_2019",
      "short": "2019",
      "long": "Class 2019"
    },
    "_2020": {
      "id": "class_2020",
      "short": "2020",
      "long": "Class 2020"
    },
    "_2021": {
      "id": "class_2021",
      "short": "2021",
      "long": "Class 2021"
    },
    "_2022": {
      "id": "class_2022",
      "short": "2022",
      "long": "Class 2022"
    },
    "_2023": {
      "id": "class_2023",
      "short": "2023",
      "long": "Class 2023"
    },
    "_2024": {
      "id": "class_2024",
      "short": "2024",
      "long": "Class 2024"
    },
    "_2025": {
      "id": "class_2025",
      "short": "2025",
      "long": "Class 2025"
    },
    "_2026": {
      "id": "class_2026",
      "short": "2026",
      "long": "Class 2026"
    }
  });

  SPENDABLE_TYPE_NAMES = /** @type {const} */ ({
    "p2pk65": {
      "id": "p2pk65",
      "short": "P2PK65",
      "long": "Pay to Public Key (65 bytes)"
    },
    "p2pk33": {
      "id": "p2pk33",
      "short": "P2PK33",
      "long": "Pay to Public Key (33 bytes)"
    },
    "p2pkh": {
      "id": "p2pkh",
      "short": "P2PKH",
      "long": "Pay to Public Key Hash"
    },
    "p2ms": {
      "id": "p2ms",
      "short": "P2MS",
      "long": "Pay to Multisig"
    },
    "p2sh": {
      "id": "p2sh",
      "short": "P2SH",
      "long": "Pay to Script Hash"
    },
    "p2wpkh": {
      "id": "p2wpkh",
      "short": "P2WPKH",
      "long": "Pay to Witness Public Key Hash"
    },
    "p2wsh": {
      "id": "p2wsh",
      "short": "P2WSH",
      "long": "Pay to Witness Script Hash"
    },
    "p2tr": {
      "id": "p2tr",
      "short": "P2TR",
      "long": "Pay to Taproot"
    },
    "p2a": {
      "id": "p2a",
      "short": "P2A",
      "long": "Pay to Anchor"
    },
    "unknown": {
      "id": "unknown_outputs",
      "short": "Unknown",
      "long": "Unknown Output Type"
    },
    "empty": {
      "id": "empty_outputs",
      "short": "Empty",
      "long": "Empty Output"
    }
  });

  AGE_RANGE_NAMES = /** @type {const} */ ({
    "upTo1h": {
      "id": "under_1h_old",
      "short": "<1h",
      "long": "Under 1 Hour Old"
    },
    "_1hTo1d": {
      "id": "1h_to_1d_old",
      "short": "1h-1d",
      "long": "1 Hour to 1 Day Old"
    },
    "_1dTo1w": {
      "id": "1d_to_1w_old",
      "short": "1d-1w",
      "long": "1 Day to 1 Week Old"
    },
    "_1wTo1m": {
      "id": "1w_to_1m_old",
      "short": "1w-1m",
      "long": "1 Week to 1 Month Old"
    },
    "_1mTo2m": {
      "id": "1m_to_2m_old",
      "short": "1m-2m",
      "long": "1 to 2 Months Old"
    },
    "_2mTo3m": {
      "id": "2m_to_3m_old",
      "short": "2m-3m",
      "long": "2 to 3 Months Old"
    },
    "_3mTo4m": {
      "id": "3m_to_4m_old",
      "short": "3m-4m",
      "long": "3 to 4 Months Old"
    },
    "_4mTo5m": {
      "id": "4m_to_5m_old",
      "short": "4m-5m",
      "long": "4 to 5 Months Old"
    },
    "_5mTo6m": {
      "id": "5m_to_6m_old",
      "short": "5m-6m",
      "long": "5 to 6 Months Old"
    },
    "_6mTo1y": {
      "id": "6m_to_1y_old",
      "short": "6m-1y",
      "long": "6 Months to 1 Year Old"
    },
    "_1yTo2y": {
      "id": "1y_to_2y_old",
      "short": "1y-2y",
      "long": "1 to 2 Years Old"
    },
    "_2yTo3y": {
      "id": "2y_to_3y_old",
      "short": "2y-3y",
      "long": "2 to 3 Years Old"
    },
    "_3yTo4y": {
      "id": "3y_to_4y_old",
      "short": "3y-4y",
      "long": "3 to 4 Years Old"
    },
    "_4yTo5y": {
      "id": "4y_to_5y_old",
      "short": "4y-5y",
      "long": "4 to 5 Years Old"
    },
    "_5yTo6y": {
      "id": "5y_to_6y_old",
      "short": "5y-6y",
      "long": "5 to 6 Years Old"
    },
    "_6yTo7y": {
      "id": "6y_to_7y_old",
      "short": "6y-7y",
      "long": "6 to 7 Years Old"
    },
    "_7yTo8y": {
      "id": "7y_to_8y_old",
      "short": "7y-8y",
      "long": "7 to 8 Years Old"
    },
    "_8yTo10y": {
      "id": "8y_to_10y_old",
      "short": "8y-10y",
      "long": "8 to 10 Years Old"
    },
    "_10yTo12y": {
      "id": "10y_to_12y_old",
      "short": "10y-12y",
      "long": "10 to 12 Years Old"
    },
    "_12yTo15y": {
      "id": "12y_to_15y_old",
      "short": "12y-15y",
      "long": "12 to 15 Years Old"
    },
    "from15y": {
      "id": "over_15y_old",
      "short": "15y+",
      "long": "15+ Years Old"
    }
  });

  MAX_AGE_NAMES = /** @type {const} */ ({
    "_1w": {
      "id": "under_1w_old",
      "short": "<1w",
      "long": "Under 1 Week Old"
    },
    "_1m": {
      "id": "under_1m_old",
      "short": "<1m",
      "long": "Under 1 Month Old"
    },
    "_2m": {
      "id": "under_2m_old",
      "short": "<2m",
      "long": "Under 2 Months Old"
    },
    "_3m": {
      "id": "under_3m_old",
      "short": "<3m",
      "long": "Under 3 Months Old"
    },
    "_4m": {
      "id": "under_4m_old",
      "short": "<4m",
      "long": "Under 4 Months Old"
    },
    "_5m": {
      "id": "under_5m_old",
      "short": "<5m",
      "long": "Under 5 Months Old"
    },
    "_6m": {
      "id": "under_6m_old",
      "short": "<6m",
      "long": "Under 6 Months Old"
    },
    "_1y": {
      "id": "under_1y_old",
      "short": "<1y",
      "long": "Under 1 Year Old"
    },
    "_2y": {
      "id": "under_2y_old",
      "short": "<2y",
      "long": "Under 2 Years Old"
    },
    "_3y": {
      "id": "under_3y_old",
      "short": "<3y",
      "long": "Under 3 Years Old"
    },
    "_4y": {
      "id": "under_4y_old",
      "short": "<4y",
      "long": "Under 4 Years Old"
    },
    "_5y": {
      "id": "under_5y_old",
      "short": "<5y",
      "long": "Under 5 Years Old"
    },
    "_6y": {
      "id": "under_6y_old",
      "short": "<6y",
      "long": "Under 6 Years Old"
    },
    "_7y": {
      "id": "under_7y_old",
      "short": "<7y",
      "long": "Under 7 Years Old"
    },
    "_8y": {
      "id": "under_8y_old",
      "short": "<8y",
      "long": "Under 8 Years Old"
    },
    "_10y": {
      "id": "under_10y_old",
      "short": "<10y",
      "long": "Under 10 Years Old"
    },
    "_12y": {
      "id": "under_12y_old",
      "short": "<12y",
      "long": "Under 12 Years Old"
    },
    "_15y": {
      "id": "under_15y_old",
      "short": "<15y",
      "long": "Under 15 Years Old"
    }
  });

  MIN_AGE_NAMES = /** @type {const} */ ({
    "_1d": {
      "id": "over_1d_old",
      "short": "1d+",
      "long": "Over 1 Day Old"
    },
    "_1w": {
      "id": "over_1w_old",
      "short": "1w+",
      "long": "Over 1 Week Old"
    },
    "_1m": {
      "id": "over_1m_old",
      "short": "1m+",
      "long": "Over 1 Month Old"
    },
    "_2m": {
      "id": "over_2m_old",
      "short": "2m+",
      "long": "Over 2 Months Old"
    },
    "_3m": {
      "id": "over_3m_old",
      "short": "3m+",
      "long": "Over 3 Months Old"
    },
    "_4m": {
      "id": "over_4m_old",
      "short": "4m+",
      "long": "Over 4 Months Old"
    },
    "_5m": {
      "id": "over_5m_old",
      "short": "5m+",
      "long": "Over 5 Months Old"
    },
    "_6m": {
      "id": "over_6m_old",
      "short": "6m+",
      "long": "Over 6 Months Old"
    },
    "_1y": {
      "id": "over_1y_old",
      "short": "1y+",
      "long": "Over 1 Year Old"
    },
    "_2y": {
      "id": "over_2y_old",
      "short": "2y+",
      "long": "Over 2 Years Old"
    },
    "_3y": {
      "id": "over_3y_old",
      "short": "3y+",
      "long": "Over 3 Years Old"
    },
    "_4y": {
      "id": "over_4y_old",
      "short": "4y+",
      "long": "Over 4 Years Old"
    },
    "_5y": {
      "id": "over_5y_old",
      "short": "5y+",
      "long": "Over 5 Years Old"
    },
    "_6y": {
      "id": "over_6y_old",
      "short": "6y+",
      "long": "Over 6 Years Old"
    },
    "_7y": {
      "id": "over_7y_old",
      "short": "7y+",
      "long": "Over 7 Years Old"
    },
    "_8y": {
      "id": "over_8y_old",
      "short": "8y+",
      "long": "Over 8 Years Old"
    },
    "_10y": {
      "id": "over_10y_old",
      "short": "10y+",
      "long": "Over 10 Years Old"
    },
    "_12y": {
      "id": "over_12y_old",
      "short": "12y+",
      "long": "Over 12 Years Old"
    }
  });

  AMOUNT_RANGE_NAMES = /** @type {const} */ ({
    "_0sats": {
      "id": "with_0sats",
      "short": "0 sats",
      "long": "0 Sats"
    },
    "_1satTo10sats": {
      "id": "above_1sat_under_10sats",
      "short": "1-10 sats",
      "long": "1-10 Sats"
    },
    "_10satsTo100sats": {
      "id": "above_10sats_under_100sats",
      "short": "10-100 sats",
      "long": "10-100 Sats"
    },
    "_100satsTo1kSats": {
      "id": "above_100sats_under_1k_sats",
      "short": "100-1k sats",
      "long": "100-1K Sats"
    },
    "_1kSatsTo10kSats": {
      "id": "above_1k_sats_under_10k_sats",
      "short": "1k-10k sats",
      "long": "1K-10K Sats"
    },
    "_10kSatsTo100kSats": {
      "id": "above_10k_sats_under_100k_sats",
      "short": "10k-100k sats",
      "long": "10K-100K Sats"
    },
    "_100kSatsTo1mSats": {
      "id": "above_100k_sats_under_1m_sats",
      "short": "100k-1M sats",
      "long": "100K-1M Sats"
    },
    "_1mSatsTo10mSats": {
      "id": "above_1m_sats_under_10m_sats",
      "short": "1M-10M sats",
      "long": "1M-10M Sats"
    },
    "_10mSatsTo1btc": {
      "id": "above_10m_sats_under_1btc",
      "short": "0.1-1 BTC",
      "long": "0.1-1 BTC"
    },
    "_1btcTo10btc": {
      "id": "above_1btc_under_10btc",
      "short": "1-10 BTC",
      "long": "1-10 BTC"
    },
    "_10btcTo100btc": {
      "id": "above_10btc_under_100btc",
      "short": "10-100 BTC",
      "long": "10-100 BTC"
    },
    "_100btcTo1kBtc": {
      "id": "above_100btc_under_1k_btc",
      "short": "100-1k BTC",
      "long": "100-1K BTC"
    },
    "_1kBtcTo10kBtc": {
      "id": "above_1k_btc_under_10k_btc",
      "short": "1k-10k BTC",
      "long": "1K-10K BTC"
    },
    "_10kBtcTo100kBtc": {
      "id": "above_10k_btc_under_100k_btc",
      "short": "10k-100k BTC",
      "long": "10K-100K BTC"
    },
    "_100kBtcOrMore": {
      "id": "above_100k_btc",
      "short": "100k+ BTC",
      "long": "100K+ BTC"
    }
  });

  GE_AMOUNT_NAMES = /** @type {const} */ ({
    "_1sat": {
      "id": "over_1sat",
      "short": "1+ sats",
      "long": "Over 1 Sat"
    },
    "_10sats": {
      "id": "over_10sats",
      "short": "10+ sats",
      "long": "Over 10 Sats"
    },
    "_100sats": {
      "id": "over_100sats",
      "short": "100+ sats",
      "long": "Over 100 Sats"
    },
    "_1kSats": {
      "id": "over_1k_sats",
      "short": "1k+ sats",
      "long": "Over 1K Sats"
    },
    "_10kSats": {
      "id": "over_10k_sats",
      "short": "10k+ sats",
      "long": "Over 10K Sats"
    },
    "_100kSats": {
      "id": "over_100k_sats",
      "short": "100k+ sats",
      "long": "Over 100K Sats"
    },
    "_1mSats": {
      "id": "over_1m_sats",
      "short": "1M+ sats",
      "long": "Over 1M Sats"
    },
    "_10mSats": {
      "id": "over_10m_sats",
      "short": "0.1+ BTC",
      "long": "Over 0.1 BTC"
    },
    "_1btc": {
      "id": "over_1btc",
      "short": "1+ BTC",
      "long": "Over 1 BTC"
    },
    "_10btc": {
      "id": "over_10btc",
      "short": "10+ BTC",
      "long": "Over 10 BTC"
    },
    "_100btc": {
      "id": "over_100btc",
      "short": "100+ BTC",
      "long": "Over 100 BTC"
    },
    "_1kBtc": {
      "id": "over_1k_btc",
      "short": "1k+ BTC",
      "long": "Over 1K BTC"
    },
    "_10kBtc": {
      "id": "over_10k_btc",
      "short": "10k+ BTC",
      "long": "Over 10K BTC"
    }
  });

  LT_AMOUNT_NAMES = /** @type {const} */ ({
    "_10sats": {
      "id": "under_10sats",
      "short": "<10 sats",
      "long": "Under 10 Sats"
    },
    "_100sats": {
      "id": "under_100sats",
      "short": "<100 sats",
      "long": "Under 100 Sats"
    },
    "_1kSats": {
      "id": "under_1k_sats",
      "short": "<1k sats",
      "long": "Under 1K Sats"
    },
    "_10kSats": {
      "id": "under_10k_sats",
      "short": "<10k sats",
      "long": "Under 10K Sats"
    },
    "_100kSats": {
      "id": "under_100k_sats",
      "short": "<100k sats",
      "long": "Under 100K Sats"
    },
    "_1mSats": {
      "id": "under_1m_sats",
      "short": "<1M sats",
      "long": "Under 1M Sats"
    },
    "_10mSats": {
      "id": "under_10m_sats",
      "short": "<0.1 BTC",
      "long": "Under 0.1 BTC"
    },
    "_1btc": {
      "id": "under_1btc",
      "short": "<1 BTC",
      "long": "Under 1 BTC"
    },
    "_10btc": {
      "id": "under_10btc",
      "short": "<10 BTC",
      "long": "Under 10 BTC"
    },
    "_100btc": {
      "id": "under_100btc",
      "short": "<100 BTC",
      "long": "Under 100 BTC"
    },
    "_1kBtc": {
      "id": "under_1k_btc",
      "short": "<1k BTC",
      "long": "Under 1K BTC"
    },
    "_10kBtc": {
      "id": "under_10k_btc",
      "short": "<10k BTC",
      "long": "Under 10K BTC"
    },
    "_100kBtc": {
      "id": "under_100k_btc",
      "short": "<100k BTC",
      "long": "Under 100K BTC"
    }
  });

  /**
   * Convert an index value to a Date for date-based indexes.
   * @param {Index} index - The index type
   * @param {number} i - The index value
   * @returns {globalThis.Date}
   */
  indexToDate(index, i) {
    return indexToDate(index, i);
  }

  /**
   * Convert a Date to an index value for date-based indexes.
   * @param {Index} index - The index type
   * @param {globalThis.Date} d - The date to convert
   * @returns {number}
   */
  dateToIndex(index, d) {
    return dateToIndex(index, d);
  }


  /**
   * @param {BrkClientOptions|string} options
   */
  constructor(options) {
    super(options);
    /** @type {MetricsTree} */
    this.metrics = this._buildTree('');
  }

  /**
   * @private
   * @param {string} basePath
   * @returns {MetricsTree}
   */
  _buildTree(basePath) {
    return {
      blocks: {
        blockhash: createMetricPattern18(this, 'blockhash'),
        difficulty: {
          raw: createMetricPattern1(this, 'difficulty'),
          asHash: createMetricPattern1(this, 'difficulty_as_hash'),
          adjustment: createBpsPercentRatioPattern(this, 'difficulty_adjustment'),
          epoch: createMetricPattern1(this, 'difficulty_epoch'),
          blocksBeforeNextAdjustment: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
          daysBeforeNextAdjustment: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
        },
        time: {
          timestamp: createMetricPattern1(this, 'timestamp'),
          date: createMetricPattern18(this, 'date'),
          timestampMonotonic: createMetricPattern18(this, 'timestamp_monotonic'),
        },
        totalSize: createMetricPattern18(this, 'total_size'),
        weight: {
          base: createMetricPattern18(this, 'block_weight'),
          cumulative: createMetricPattern1(this, 'block_weight_cumulative'),
          sum: create_1m1w1y24hPattern(this, 'block_weight_sum'),
          average: create_1m1w1y24hPattern(this, 'block_weight_average'),
          min: create_1m1w1y24hPattern(this, 'block_weight_min'),
          max: create_1m1w1y24hPattern(this, 'block_weight_max'),
          pct10: create_1m1w1y24hPattern(this, 'block_weight_p10'),
          pct25: create_1m1w1y24hPattern(this, 'block_weight_p25'),
          median: create_1m1w1y24hPattern(this, 'block_weight_median'),
          pct75: create_1m1w1y24hPattern(this, 'block_weight_p75'),
          pct90: create_1m1w1y24hPattern(this, 'block_weight_p90'),
        },
        count: {
          blockCountTarget: createMetricPattern1(this, 'block_count_target'),
          blockCount: createCumulativeHeightSumPattern(this, 'block_count'),
          blockCountSum: create_1m1w1y24hPattern(this, 'block_count_sum'),
          height1hAgo: createMetricPattern18(this, 'height_1h_ago'),
          height24hAgo: createMetricPattern18(this, 'height_24h_ago'),
          height3dAgo: createMetricPattern18(this, 'height_3d_ago'),
          height1wAgo: createMetricPattern18(this, 'height_1w_ago'),
          height8dAgo: createMetricPattern18(this, 'height_8d_ago'),
          height9dAgo: createMetricPattern18(this, 'height_9d_ago'),
          height12dAgo: createMetricPattern18(this, 'height_12d_ago'),
          height13dAgo: createMetricPattern18(this, 'height_13d_ago'),
          height2wAgo: createMetricPattern18(this, 'height_2w_ago'),
          height21dAgo: createMetricPattern18(this, 'height_21d_ago'),
          height26dAgo: createMetricPattern18(this, 'height_26d_ago'),
          height1mAgo: createMetricPattern18(this, 'height_1m_ago'),
          height34dAgo: createMetricPattern18(this, 'height_34d_ago'),
          height55dAgo: createMetricPattern18(this, 'height_55d_ago'),
          height2mAgo: createMetricPattern18(this, 'height_2m_ago'),
          height9wAgo: createMetricPattern18(this, 'height_9w_ago'),
          height12wAgo: createMetricPattern18(this, 'height_12w_ago'),
          height89dAgo: createMetricPattern18(this, 'height_89d_ago'),
          height3mAgo: createMetricPattern18(this, 'height_3m_ago'),
          height14wAgo: createMetricPattern18(this, 'height_14w_ago'),
          height111dAgo: createMetricPattern18(this, 'height_111d_ago'),
          height144dAgo: createMetricPattern18(this, 'height_144d_ago'),
          height6mAgo: createMetricPattern18(this, 'height_6m_ago'),
          height26wAgo: createMetricPattern18(this, 'height_26w_ago'),
          height200dAgo: createMetricPattern18(this, 'height_200d_ago'),
          height9mAgo: createMetricPattern18(this, 'height_9m_ago'),
          height350dAgo: createMetricPattern18(this, 'height_350d_ago'),
          height12mAgo: createMetricPattern18(this, 'height_12m_ago'),
          height1yAgo: createMetricPattern18(this, 'height_1y_ago'),
          height14mAgo: createMetricPattern18(this, 'height_14m_ago'),
          height2yAgo: createMetricPattern18(this, 'height_2y_ago'),
          height26mAgo: createMetricPattern18(this, 'height_26m_ago'),
          height3yAgo: createMetricPattern18(this, 'height_3y_ago'),
          height200wAgo: createMetricPattern18(this, 'height_200w_ago'),
          height4yAgo: createMetricPattern18(this, 'height_4y_ago'),
          height5yAgo: createMetricPattern18(this, 'height_5y_ago'),
          height6yAgo: createMetricPattern18(this, 'height_6y_ago'),
          height8yAgo: createMetricPattern18(this, 'height_8y_ago'),
          height9yAgo: createMetricPattern18(this, 'height_9y_ago'),
          height10yAgo: createMetricPattern18(this, 'height_10y_ago'),
          height12yAgo: createMetricPattern18(this, 'height_12y_ago'),
          height14yAgo: createMetricPattern18(this, 'height_14y_ago'),
          height26yAgo: createMetricPattern18(this, 'height_26y_ago'),
        },
        interval: createAverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'block_interval'),
        halving: {
          epoch: createMetricPattern1(this, 'halving_epoch'),
          blocksBeforeNextHalving: createMetricPattern1(this, 'blocks_before_next_halving'),
          daysBeforeNextHalving: createMetricPattern1(this, 'days_before_next_halving'),
        },
        vbytes: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'block_vbytes'),
        size: {
          cumulative: createMetricPattern1(this, 'block_size_cumulative'),
          sum: create_1m1w1y24hPattern(this, 'block_size_sum'),
          average: create_1m1w1y24hPattern(this, 'block_size_average'),
          min: create_1m1w1y24hPattern(this, 'block_size_min'),
          max: create_1m1w1y24hPattern(this, 'block_size_max'),
          pct10: create_1m1w1y24hPattern(this, 'block_size_p10'),
          pct25: create_1m1w1y24hPattern(this, 'block_size_p25'),
          median: create_1m1w1y24hPattern(this, 'block_size_median'),
          pct75: create_1m1w1y24hPattern(this, 'block_size_p75'),
          pct90: create_1m1w1y24hPattern(this, 'block_size_p90'),
        },
        fullness: createBpsPercentRatioPattern2(this, 'block_fullness'),
      },
      transactions: {
        firstTxindex: createMetricPattern18(this, 'first_txindex'),
        height: createMetricPattern19(this, 'height'),
        txid: createMetricPattern19(this, 'txid'),
        txversion: createMetricPattern19(this, 'txversion'),
        rawlocktime: createMetricPattern19(this, 'rawlocktime'),
        baseSize: createMetricPattern19(this, 'base_size'),
        totalSize: createMetricPattern19(this, 'total_size'),
        isExplicitlyRbf: createMetricPattern19(this, 'is_explicitly_rbf'),
        firstTxinindex: createMetricPattern19(this, 'first_txinindex'),
        firstTxoutindex: createMetricPattern19(this, 'first_txoutindex'),
        count: {
          txCount: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'tx_count'),
          isCoinbase: createMetricPattern19(this, 'is_coinbase'),
        },
        size: {
          vsize: create_6bBlockTxindexPattern(this, 'tx_vsize'),
          weight: create_6bBlockTxindexPattern(this, 'tx_weight'),
        },
        fees: {
          inputValue: createMetricPattern19(this, 'input_value'),
          outputValue: createMetricPattern19(this, 'output_value'),
          fee: create_6bBlockTxindexPattern(this, 'fee'),
          feeRate: create_6bBlockTxindexPattern(this, 'fee_rate'),
        },
        versions: {
          v1: createCumulativeHeightSumPattern(this, 'tx_v1'),
          v2: createCumulativeHeightSumPattern(this, 'tx_v2'),
          v3: createCumulativeHeightSumPattern(this, 'tx_v3'),
        },
        volume: {
          sentSum: create_1m1w1y24hBtcCentsSatsUsdPattern(this, 'sent_sum'),
          receivedSum: create_1m1w1y24hBtcCentsSatsUsdPattern(this, 'received_sum'),
          annualizedVolume: createBtcCentsSatsUsdPattern(this, 'annualized_volume'),
          txPerSec: createMetricPattern1(this, 'tx_per_sec'),
          outputsPerSec: createMetricPattern1(this, 'outputs_per_sec'),
          inputsPerSec: createMetricPattern1(this, 'inputs_per_sec'),
        },
      },
      inputs: {
        firstTxinindex: createMetricPattern18(this, 'first_txinindex'),
        outpoint: createMetricPattern20(this, 'outpoint'),
        txindex: createMetricPattern20(this, 'txindex'),
        outputtype: createMetricPattern20(this, 'outputtype'),
        typeindex: createMetricPattern20(this, 'typeindex'),
        spent: {
          txoutindex: createMetricPattern20(this, 'txoutindex'),
          value: createMetricPattern20(this, 'value'),
        },
        count: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'input_count'),
      },
      outputs: {
        firstTxoutindex: createMetricPattern18(this, 'first_txoutindex'),
        value: createMetricPattern21(this, 'value'),
        outputtype: createMetricPattern21(this, 'outputtype'),
        typeindex: createMetricPattern21(this, 'typeindex'),
        txindex: createMetricPattern21(this, 'txindex'),
        spent: {
          txinindex: createMetricPattern21(this, 'txinindex'),
        },
        count: {
          totalCount: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'output_count'),
          utxoCount: createMetricPattern1(this, 'exact_utxo_count'),
        },
      },
      addresses: {
        firstP2pk65addressindex: createMetricPattern18(this, 'first_p2pk65addressindex'),
        firstP2pk33addressindex: createMetricPattern18(this, 'first_p2pk33addressindex'),
        firstP2pkhaddressindex: createMetricPattern18(this, 'first_p2pkhaddressindex'),
        firstP2shaddressindex: createMetricPattern18(this, 'first_p2shaddressindex'),
        firstP2wpkhaddressindex: createMetricPattern18(this, 'first_p2wpkhaddressindex'),
        firstP2wshaddressindex: createMetricPattern18(this, 'first_p2wshaddressindex'),
        firstP2traddressindex: createMetricPattern18(this, 'first_p2traddressindex'),
        firstP2aaddressindex: createMetricPattern18(this, 'first_p2aaddressindex'),
        p2pk65bytes: createMetricPattern27(this, 'p2pk65bytes'),
        p2pk33bytes: createMetricPattern26(this, 'p2pk33bytes'),
        p2pkhbytes: createMetricPattern28(this, 'p2pkhbytes'),
        p2shbytes: createMetricPattern29(this, 'p2shbytes'),
        p2wpkhbytes: createMetricPattern31(this, 'p2wpkhbytes'),
        p2wshbytes: createMetricPattern32(this, 'p2wshbytes'),
        p2trbytes: createMetricPattern30(this, 'p2trbytes'),
        p2abytes: createMetricPattern24(this, 'p2abytes'),
      },
      scripts: {
        firstEmptyoutputindex: createMetricPattern18(this, 'first_emptyoutputindex'),
        firstOpreturnindex: createMetricPattern18(this, 'first_opreturnindex'),
        firstP2msoutputindex: createMetricPattern18(this, 'first_p2msoutputindex'),
        firstUnknownoutputindex: createMetricPattern18(this, 'first_unknownoutputindex'),
        emptyToTxindex: createMetricPattern22(this, 'txindex'),
        opreturnToTxindex: createMetricPattern23(this, 'txindex'),
        p2msToTxindex: createMetricPattern25(this, 'txindex'),
        unknownToTxindex: createMetricPattern33(this, 'txindex'),
        count: {
          p2a: createCumulativeHeightSumPattern(this, 'p2a_count'),
          p2ms: createCumulativeHeightSumPattern(this, 'p2ms_count'),
          p2pk33: createCumulativeHeightSumPattern(this, 'p2pk33_count'),
          p2pk65: createCumulativeHeightSumPattern(this, 'p2pk65_count'),
          p2pkh: createCumulativeHeightSumPattern(this, 'p2pkh_count'),
          p2sh: createCumulativeHeightSumPattern(this, 'p2sh_count'),
          p2tr: createCumulativeHeightSumPattern(this, 'p2tr_count'),
          p2wpkh: createCumulativeHeightSumPattern(this, 'p2wpkh_count'),
          p2wsh: createCumulativeHeightSumPattern(this, 'p2wsh_count'),
          opreturn: createCumulativeHeightSumPattern(this, 'opreturn_count'),
          emptyoutput: createCumulativeHeightSumPattern(this, 'emptyoutput_count'),
          unknownoutput: createCumulativeHeightSumPattern(this, 'unknownoutput_count'),
          segwit: createCumulativeHeightSumPattern(this, 'segwit_count'),
        },
        value: {
          opreturn: create_1m1w1y24hBaseCumulativePattern(this, 'opreturn_value'),
        },
        adoption: {
          taproot: createBpsPercentRatioPattern(this, 'taproot_adoption'),
          segwit: createBpsPercentRatioPattern(this, 'segwit_adoption'),
        },
      },
      mining: {
        rewards: {
          coinbase: create_1m1w1y24hBaseCumulativePattern(this, 'coinbase'),
          subsidy: create_1m1w1y24hBaseCumulativePattern(this, 'subsidy'),
          fees: create_1m1w1y24hBaseCumulativePattern(this, 'fees'),
          unclaimedRewards: createBaseCumulativeSumPattern(this, 'unclaimed_rewards'),
          feeDominance: createBpsPercentRatioPattern(this, 'fee_dominance'),
          feeDominanceRolling: create_1m1w1y24hPattern2(this, 'fee_dominance'),
          subsidyDominance: createBpsPercentRatioPattern(this, 'subsidy_dominance'),
          subsidyDominanceRolling: create_1m1w1y24hPattern2(this, 'subsidy_dominance'),
          subsidySma1y: createCentsUsdPattern(this, 'subsidy_sma_1y'),
        },
        hashrate: {
          hashRate: createMetricPattern1(this, 'hash_rate'),
          hashRateSma1w: createMetricPattern1(this, 'hash_rate_sma_1w'),
          hashRateSma1m: createMetricPattern1(this, 'hash_rate_sma_1m'),
          hashRateSma2m: createMetricPattern1(this, 'hash_rate_sma_2m'),
          hashRateSma1y: createMetricPattern1(this, 'hash_rate_sma_1y'),
          hashRateAth: createMetricPattern1(this, 'hash_rate_ath'),
          hashRateDrawdown: createBpsPercentRatioPattern(this, 'hash_rate_drawdown'),
          hashPriceThs: createMetricPattern1(this, 'hash_price_ths'),
          hashPriceThsMin: createMetricPattern1(this, 'hash_price_ths_min'),
          hashPricePhs: createMetricPattern1(this, 'hash_price_phs'),
          hashPricePhsMin: createMetricPattern1(this, 'hash_price_phs_min'),
          hashPriceRebound: createBpsPercentRatioPattern(this, 'hash_price_rebound'),
          hashValueThs: createMetricPattern1(this, 'hash_value_ths'),
          hashValueThsMin: createMetricPattern1(this, 'hash_value_ths_min'),
          hashValuePhs: createMetricPattern1(this, 'hash_value_phs'),
          hashValuePhsMin: createMetricPattern1(this, 'hash_value_phs_min'),
          hashValueRebound: createBpsPercentRatioPattern(this, 'hash_value_rebound'),
        },
      },
      positions: {
        blockPosition: createMetricPattern18(this, 'position'),
        txPosition: createMetricPattern19(this, 'position'),
      },
      cointime: {
        activity: {
          coinblocksCreated: createCumulativeHeightSumPattern(this, 'coinblocks_created'),
          coinblocksStored: createCumulativeHeightSumPattern(this, 'coinblocks_stored'),
          liveliness: createMetricPattern1(this, 'liveliness'),
          vaultedness: createMetricPattern1(this, 'vaultedness'),
          activityToVaultednessRatio: createMetricPattern1(this, 'activity_to_vaultedness_ratio'),
        },
        supply: {
          vaultedSupply: createBtcCentsSatsUsdPattern(this, 'vaulted_supply'),
          activeSupply: createBtcCentsSatsUsdPattern(this, 'active_supply'),
        },
        value: {
          cointimeValueDestroyed: createCumulativeHeightSumPattern(this, 'cointime_value_destroyed'),
          cointimeValueCreated: createCumulativeHeightSumPattern(this, 'cointime_value_created'),
          cointimeValueStored: createCumulativeHeightSumPattern(this, 'cointime_value_stored'),
          vocdd: createCumulativeHeightSumPattern(this, 'vocdd'),
        },
        cap: {
          thermoCap: createCentsUsdPattern(this, 'thermo_cap'),
          investorCap: createCentsUsdPattern(this, 'investor_cap'),
          vaultedCap: createCentsUsdPattern(this, 'vaulted_cap'),
          activeCap: createCentsUsdPattern(this, 'active_cap'),
          cointimeCap: createCentsUsdPattern(this, 'cointime_cap'),
        },
        pricing: {
          vaultedPrice: createCentsSatsUsdPattern(this, 'vaulted_price'),
          vaultedPriceRatio: createBpsRatioPattern2(this, 'vaulted_price_ratio'),
          activePrice: createCentsSatsUsdPattern(this, 'active_price'),
          activePriceRatio: createBpsRatioPattern2(this, 'active_price_ratio'),
          trueMarketMean: createCentsSatsUsdPattern(this, 'true_market_mean'),
          trueMarketMeanRatio: createBpsRatioPattern2(this, 'true_market_mean_ratio'),
          cointimePrice: createCentsSatsUsdPattern(this, 'cointime_price'),
          cointimePriceRatio: createBpsRatioPattern2(this, 'cointime_price_ratio'),
        },
        adjusted: {
          cointimeAdjInflationRate: createBpsPercentRatioPattern(this, 'cointime_adj_inflation_rate'),
          cointimeAdjTxVelocityBtc: createMetricPattern1(this, 'cointime_adj_tx_velocity_btc'),
          cointimeAdjTxVelocityUsd: createMetricPattern1(this, 'cointime_adj_tx_velocity_usd'),
        },
        reserveRisk: {
          vocddMedian1y: createMetricPattern18(this, 'vocdd_median_1y'),
          hodlBank: createMetricPattern18(this, 'hodl_bank'),
          reserveRisk: createMetricPattern1(this, 'reserve_risk'),
        },
      },
      constants: {
        constant0: createMetricPattern1(this, 'constant_0'),
        constant1: createMetricPattern1(this, 'constant_1'),
        constant2: createMetricPattern1(this, 'constant_2'),
        constant3: createMetricPattern1(this, 'constant_3'),
        constant4: createMetricPattern1(this, 'constant_4'),
        constant20: createMetricPattern1(this, 'constant_20'),
        constant30: createMetricPattern1(this, 'constant_30'),
        constant382: createMetricPattern1(this, 'constant_38_2'),
        constant50: createMetricPattern1(this, 'constant_50'),
        constant618: createMetricPattern1(this, 'constant_61_8'),
        constant70: createMetricPattern1(this, 'constant_70'),
        constant80: createMetricPattern1(this, 'constant_80'),
        constant100: createMetricPattern1(this, 'constant_100'),
        constant600: createMetricPattern1(this, 'constant_600'),
        constantMinus1: createMetricPattern1(this, 'constant_minus_1'),
        constantMinus2: createMetricPattern1(this, 'constant_minus_2'),
        constantMinus3: createMetricPattern1(this, 'constant_minus_3'),
        constantMinus4: createMetricPattern1(this, 'constant_minus_4'),
      },
      indexes: {
        address: {
          p2pk33: {
            identity: createMetricPattern26(this, 'p2pk33addressindex'),
          },
          p2pk65: {
            identity: createMetricPattern27(this, 'p2pk65addressindex'),
          },
          p2pkh: {
            identity: createMetricPattern28(this, 'p2pkhaddressindex'),
          },
          p2sh: {
            identity: createMetricPattern29(this, 'p2shaddressindex'),
          },
          p2tr: {
            identity: createMetricPattern30(this, 'p2traddressindex'),
          },
          p2wpkh: {
            identity: createMetricPattern31(this, 'p2wpkhaddressindex'),
          },
          p2wsh: {
            identity: createMetricPattern32(this, 'p2wshaddressindex'),
          },
          p2a: {
            identity: createMetricPattern24(this, 'p2aaddressindex'),
          },
          p2ms: {
            identity: createMetricPattern25(this, 'p2msoutputindex'),
          },
          empty: {
            identity: createMetricPattern22(this, 'emptyoutputindex'),
          },
          unknown: {
            identity: createMetricPattern33(this, 'unknownoutputindex'),
          },
          opreturn: {
            identity: createMetricPattern23(this, 'opreturnindex'),
          },
        },
        height: {
          identity: createMetricPattern18(this, 'height'),
          minute10: createMetricPattern18(this, 'minute10'),
          minute30: createMetricPattern18(this, 'minute30'),
          hour1: createMetricPattern18(this, 'hour1'),
          hour4: createMetricPattern18(this, 'hour4'),
          hour12: createMetricPattern18(this, 'hour12'),
          day1: createMetricPattern18(this, 'day1'),
          day3: createMetricPattern18(this, 'day3'),
          difficultyepoch: createMetricPattern18(this, 'difficultyepoch'),
          halvingepoch: createMetricPattern18(this, 'halvingepoch'),
          week1: createMetricPattern18(this, 'week1'),
          month1: createMetricPattern18(this, 'month1'),
          month3: createMetricPattern18(this, 'month3'),
          month6: createMetricPattern18(this, 'month6'),
          year1: createMetricPattern18(this, 'year1'),
          year10: createMetricPattern18(this, 'year10'),
          txindexCount: createMetricPattern18(this, 'txindex_count'),
        },
        difficultyepoch: {
          identity: createMetricPattern17(this, 'difficultyepoch'),
          firstHeight: createMetricPattern17(this, 'first_height'),
          heightCount: createMetricPattern17(this, 'height_count'),
        },
        halvingepoch: {
          identity: createMetricPattern16(this, 'halvingepoch'),
          firstHeight: createMetricPattern16(this, 'first_height'),
        },
        minute10: {
          identity: createMetricPattern3(this, 'minute10'),
          firstHeight: createMetricPattern3(this, 'minute10_first_height'),
        },
        minute30: {
          identity: createMetricPattern4(this, 'minute30'),
          firstHeight: createMetricPattern4(this, 'minute30_first_height'),
        },
        hour1: {
          identity: createMetricPattern5(this, 'hour1'),
          firstHeight: createMetricPattern5(this, 'hour1_first_height'),
        },
        hour4: {
          identity: createMetricPattern6(this, 'hour4'),
          firstHeight: createMetricPattern6(this, 'hour4_first_height'),
        },
        hour12: {
          identity: createMetricPattern7(this, 'hour12'),
          firstHeight: createMetricPattern7(this, 'hour12_first_height'),
        },
        day1: {
          identity: createMetricPattern8(this, 'day1'),
          date: createMetricPattern8(this, 'date'),
          firstHeight: createMetricPattern8(this, 'first_height'),
          heightCount: createMetricPattern8(this, 'height_count'),
        },
        day3: {
          identity: createMetricPattern9(this, 'day3'),
          firstHeight: createMetricPattern9(this, 'day3_first_height'),
        },
        week1: {
          identity: createMetricPattern10(this, 'week1'),
          date: createMetricPattern10(this, 'date'),
          firstHeight: createMetricPattern10(this, 'week1_first_height'),
        },
        month1: {
          identity: createMetricPattern11(this, 'month1'),
          date: createMetricPattern11(this, 'date'),
          firstHeight: createMetricPattern11(this, 'month1_first_height'),
        },
        month3: {
          identity: createMetricPattern12(this, 'month3'),
          date: createMetricPattern12(this, 'date'),
          firstHeight: createMetricPattern12(this, 'month3_first_height'),
        },
        month6: {
          identity: createMetricPattern13(this, 'month6'),
          date: createMetricPattern13(this, 'date'),
          firstHeight: createMetricPattern13(this, 'month6_first_height'),
        },
        year1: {
          identity: createMetricPattern14(this, 'year1'),
          date: createMetricPattern14(this, 'date'),
          firstHeight: createMetricPattern14(this, 'year1_first_height'),
        },
        year10: {
          identity: createMetricPattern15(this, 'year10'),
          date: createMetricPattern15(this, 'date'),
          firstHeight: createMetricPattern15(this, 'year10_first_height'),
        },
        txindex: {
          identity: createMetricPattern19(this, 'txindex'),
          inputCount: createMetricPattern19(this, 'input_count'),
          outputCount: createMetricPattern19(this, 'output_count'),
        },
        txinindex: {
          identity: createMetricPattern20(this, 'txinindex'),
        },
        txoutindex: {
          identity: createMetricPattern21(this, 'txoutindex'),
        },
      },
      market: {
        ath: {
          priceAth: createCentsSatsUsdPattern(this, 'price_ath'),
          priceDrawdown: createBpsPercentRatioPattern(this, 'price_drawdown'),
          daysSincePriceAth: createMetricPattern1(this, 'days_since_price_ath'),
          yearsSincePriceAth: createMetricPattern2(this, 'years_since_price_ath'),
          maxDaysBetweenPriceAth: createMetricPattern1(this, 'max_days_between_price_ath'),
          maxYearsBetweenPriceAth: createMetricPattern2(this, 'max_years_between_price_ath'),
        },
        lookback: {
          _24h: createCentsSatsUsdPattern(this, 'price_lookback_24h'),
          _1w: createCentsSatsUsdPattern(this, 'price_lookback_1w'),
          _1m: createCentsSatsUsdPattern(this, 'price_lookback_1m'),
          _3m: createCentsSatsUsdPattern(this, 'price_lookback_3m'),
          _6m: createCentsSatsUsdPattern(this, 'price_lookback_6m'),
          _1y: createCentsSatsUsdPattern(this, 'price_lookback_1y'),
          _2y: createCentsSatsUsdPattern(this, 'price_lookback_2y'),
          _3y: createCentsSatsUsdPattern(this, 'price_lookback_3y'),
          _4y: createCentsSatsUsdPattern(this, 'price_lookback_4y'),
          _5y: createCentsSatsUsdPattern(this, 'price_lookback_5y'),
          _6y: createCentsSatsUsdPattern(this, 'price_lookback_6y'),
          _8y: createCentsSatsUsdPattern(this, 'price_lookback_8y'),
          _10y: createCentsSatsUsdPattern(this, 'price_lookback_10y'),
        },
        returns: {
          priceReturn: {
            _24h: createBpsPercentRatioPattern(this, 'price_return_24h'),
            _1w: createBpsPercentRatioPattern(this, 'price_return_1w'),
            _1m: createBpsPercentRatioPattern(this, 'price_return_1m'),
            _3m: createBpsPercentRatioPattern(this, 'price_return_3m'),
            _6m: createBpsPercentRatioPattern(this, 'price_return_6m'),
            _1y: createBpsPercentRatioPattern(this, 'price_return_1y'),
            _2y: createBpsPercentRatioPattern(this, 'price_return_2y'),
            _3y: createBpsPercentRatioPattern(this, 'price_return_3y'),
            _4y: createBpsPercentRatioPattern(this, 'price_return_4y'),
            _5y: createBpsPercentRatioPattern(this, 'price_return_5y'),
            _6y: createBpsPercentRatioPattern(this, 'price_return_6y'),
            _8y: createBpsPercentRatioPattern(this, 'price_return_8y'),
            _10y: createBpsPercentRatioPattern(this, 'price_return_10y'),
          },
          priceCagr: create_10y2y3y4y5y6y8yPattern(this, 'price_cagr'),
          priceReturn24hSd1w: {
            sma: createMetricPattern1(this, 'price_return_24h_sma_1w'),
            sd: createMetricPattern1(this, 'price_return_24h_sd_1w'),
          },
          priceReturn24hSd1m: {
            sma: createMetricPattern1(this, 'price_return_24h_sma_1m'),
            sd: createMetricPattern1(this, 'price_return_24h_sd_1m'),
          },
          priceReturn24hSd1y: createSdSmaPattern(this, 'price_return_24h'),
        },
        volatility: {
          priceVolatility1w: createMetricPattern1(this, 'price_volatility_1w'),
          priceVolatility1m: createMetricPattern1(this, 'price_volatility_1m'),
          priceVolatility1y: createMetricPattern1(this, 'price_volatility_1y'),
        },
        range: {
          priceMin1w: createCentsSatsUsdPattern(this, 'price_min_1w'),
          priceMax1w: createCentsSatsUsdPattern(this, 'price_max_1w'),
          priceMin2w: createCentsSatsUsdPattern(this, 'price_min_2w'),
          priceMax2w: createCentsSatsUsdPattern(this, 'price_max_2w'),
          priceMin1m: createCentsSatsUsdPattern(this, 'price_min_1m'),
          priceMax1m: createCentsSatsUsdPattern(this, 'price_max_1m'),
          priceMin1y: createCentsSatsUsdPattern(this, 'price_min_1y'),
          priceMax1y: createCentsSatsUsdPattern(this, 'price_max_1y'),
          priceTrueRange: createMetricPattern1(this, 'price_true_range'),
          priceTrueRangeSum2w: createMetricPattern1(this, 'price_true_range_sum_2w'),
          priceChoppinessIndex2w: createBpsPercentRatioPattern(this, 'price_choppiness_index_2w'),
        },
        movingAverage: {
          priceSma1w: createBpsPriceRatioPattern(this, 'price_sma_1w'),
          priceSma8d: createBpsPriceRatioPattern(this, 'price_sma_8d'),
          priceSma13d: createBpsPriceRatioPattern(this, 'price_sma_13d'),
          priceSma21d: createBpsPriceRatioPattern(this, 'price_sma_21d'),
          priceSma1m: createBpsPriceRatioPattern(this, 'price_sma_1m'),
          priceSma34d: createBpsPriceRatioPattern(this, 'price_sma_34d'),
          priceSma55d: createBpsPriceRatioPattern(this, 'price_sma_55d'),
          priceSma89d: createBpsPriceRatioPattern(this, 'price_sma_89d'),
          priceSma111d: createBpsPriceRatioPattern(this, 'price_sma_111d'),
          priceSma144d: createBpsPriceRatioPattern(this, 'price_sma_144d'),
          priceSma200d: createBpsPriceRatioPattern(this, 'price_sma_200d'),
          priceSma350d: createBpsPriceRatioPattern(this, 'price_sma_350d'),
          priceSma1y: createBpsPriceRatioPattern(this, 'price_sma_1y'),
          priceSma2y: createBpsPriceRatioPattern(this, 'price_sma_2y'),
          priceSma200w: createBpsPriceRatioPattern(this, 'price_sma_200w'),
          priceSma4y: createBpsPriceRatioPattern(this, 'price_sma_4y'),
          priceEma1w: createBpsPriceRatioPattern(this, 'price_ema_1w'),
          priceEma8d: createBpsPriceRatioPattern(this, 'price_ema_8d'),
          priceEma12d: createBpsPriceRatioPattern(this, 'price_ema_12d'),
          priceEma13d: createBpsPriceRatioPattern(this, 'price_ema_13d'),
          priceEma21d: createBpsPriceRatioPattern(this, 'price_ema_21d'),
          priceEma26d: createBpsPriceRatioPattern(this, 'price_ema_26d'),
          priceEma1m: createBpsPriceRatioPattern(this, 'price_ema_1m'),
          priceEma34d: createBpsPriceRatioPattern(this, 'price_ema_34d'),
          priceEma55d: createBpsPriceRatioPattern(this, 'price_ema_55d'),
          priceEma89d: createBpsPriceRatioPattern(this, 'price_ema_89d'),
          priceEma144d: createBpsPriceRatioPattern(this, 'price_ema_144d'),
          priceEma200d: createBpsPriceRatioPattern(this, 'price_ema_200d'),
          priceEma1y: createBpsPriceRatioPattern(this, 'price_ema_1y'),
          priceEma2y: createBpsPriceRatioPattern(this, 'price_ema_2y'),
          priceEma200w: createBpsPriceRatioPattern(this, 'price_ema_200w'),
          priceEma4y: createBpsPriceRatioPattern(this, 'price_ema_4y'),
          priceSma200dX24: createCentsSatsUsdPattern(this, 'price_sma_200d_x2_4'),
          priceSma200dX08: createCentsSatsUsdPattern(this, 'price_sma_200d_x0_8'),
          priceSma350dX2: createCentsSatsUsdPattern(this, 'price_sma_350d_x2'),
        },
        dca: {
          dcaSatsPerDay: createMetricPattern18(this, 'dca_sats_per_day'),
          periodStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'dca_stack'),
          periodCostBasis: {
            _1w: createCentsSatsUsdPattern(this, 'dca_cost_basis_1w'),
            _1m: createCentsSatsUsdPattern(this, 'dca_cost_basis_1m'),
            _3m: createCentsSatsUsdPattern(this, 'dca_cost_basis_3m'),
            _6m: createCentsSatsUsdPattern(this, 'dca_cost_basis_6m'),
            _1y: createCentsSatsUsdPattern(this, 'dca_cost_basis_1y'),
            _2y: createCentsSatsUsdPattern(this, 'dca_cost_basis_2y'),
            _3y: createCentsSatsUsdPattern(this, 'dca_cost_basis_3y'),
            _4y: createCentsSatsUsdPattern(this, 'dca_cost_basis_4y'),
            _5y: createCentsSatsUsdPattern(this, 'dca_cost_basis_5y'),
            _6y: createCentsSatsUsdPattern(this, 'dca_cost_basis_6y'),
            _8y: createCentsSatsUsdPattern(this, 'dca_cost_basis_8y'),
            _10y: createCentsSatsUsdPattern(this, 'dca_cost_basis_10y'),
          },
          periodReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_return'),
          periodCagr: create_10y2y3y4y5y6y8yPattern(this, 'dca_cagr'),
          periodLumpSumStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'lump_sum_stack'),
          periodLumpSumReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_return'),
          classStack: {
            from2015: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2015'),
            from2016: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2016'),
            from2017: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2017'),
            from2018: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2018'),
            from2019: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2019'),
            from2020: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2020'),
            from2021: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2021'),
            from2022: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2022'),
            from2023: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2023'),
            from2024: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2024'),
            from2025: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2025'),
            from2026: createBtcCentsSatsUsdPattern(this, 'dca_stack_from_2026'),
          },
          classCostBasis: {
            from2015: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2015'),
            from2016: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2016'),
            from2017: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2017'),
            from2018: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2018'),
            from2019: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2019'),
            from2020: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2020'),
            from2021: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2021'),
            from2022: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2022'),
            from2023: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2023'),
            from2024: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2024'),
            from2025: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2025'),
            from2026: createCentsSatsUsdPattern(this, 'dca_cost_basis_from_2026'),
          },
          classReturn: {
            from2015: createBpsPercentRatioPattern(this, 'dca_return_from_2015'),
            from2016: createBpsPercentRatioPattern(this, 'dca_return_from_2016'),
            from2017: createBpsPercentRatioPattern(this, 'dca_return_from_2017'),
            from2018: createBpsPercentRatioPattern(this, 'dca_return_from_2018'),
            from2019: createBpsPercentRatioPattern(this, 'dca_return_from_2019'),
            from2020: createBpsPercentRatioPattern(this, 'dca_return_from_2020'),
            from2021: createBpsPercentRatioPattern(this, 'dca_return_from_2021'),
            from2022: createBpsPercentRatioPattern(this, 'dca_return_from_2022'),
            from2023: createBpsPercentRatioPattern(this, 'dca_return_from_2023'),
            from2024: createBpsPercentRatioPattern(this, 'dca_return_from_2024'),
            from2025: createBpsPercentRatioPattern(this, 'dca_return_from_2025'),
            from2026: createBpsPercentRatioPattern(this, 'dca_return_from_2026'),
          },
        },
        indicators: {
          puellMultiple: createBpsRatioPattern(this, 'puell_multiple'),
          nvt: createBpsRatioPattern(this, 'nvt'),
          rsi: {
            _24h: createAverageGainsLossesRsiStochPattern(this, 'rsi'),
            _1w: {
              gains: createMetricPattern1(this, 'rsi_gains_1w'),
              losses: createMetricPattern1(this, 'rsi_losses_1w'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1w'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1w'),
              rsi: createBpsPercentRatioPattern(this, 'rsi_1w'),
              rsiMin: createBpsPercentRatioPattern(this, 'rsi_min_1w'),
              rsiMax: createBpsPercentRatioPattern(this, 'rsi_max_1w'),
              stochRsi: createBpsPercentRatioPattern(this, 'rsi_stoch_1w'),
              stochRsiK: createBpsPercentRatioPattern(this, 'rsi_stoch_k_1w'),
              stochRsiD: createBpsPercentRatioPattern(this, 'rsi_stoch_d_1w'),
            },
            _1m: {
              gains: createMetricPattern1(this, 'rsi_gains_1m'),
              losses: createMetricPattern1(this, 'rsi_losses_1m'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1m'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1m'),
              rsi: createBpsPercentRatioPattern(this, 'rsi_1m'),
              rsiMin: createBpsPercentRatioPattern(this, 'rsi_min_1m'),
              rsiMax: createBpsPercentRatioPattern(this, 'rsi_max_1m'),
              stochRsi: createBpsPercentRatioPattern(this, 'rsi_stoch_1m'),
              stochRsiK: createBpsPercentRatioPattern(this, 'rsi_stoch_k_1m'),
              stochRsiD: createBpsPercentRatioPattern(this, 'rsi_stoch_d_1m'),
            },
            _1y: {
              gains: createMetricPattern1(this, 'rsi_gains_1y'),
              losses: createMetricPattern1(this, 'rsi_losses_1y'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1y'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1y'),
              rsi: createBpsPercentRatioPattern(this, 'rsi_1y'),
              rsiMin: createBpsPercentRatioPattern(this, 'rsi_min_1y'),
              rsiMax: createBpsPercentRatioPattern(this, 'rsi_max_1y'),
              stochRsi: createBpsPercentRatioPattern(this, 'rsi_stoch_1y'),
              stochRsiK: createBpsPercentRatioPattern(this, 'rsi_stoch_k_1y'),
              stochRsiD: createBpsPercentRatioPattern(this, 'rsi_stoch_d_1y'),
            },
          },
          stochK: createBpsPercentRatioPattern(this, 'stoch_k'),
          stochD: createBpsPercentRatioPattern(this, 'stoch_d'),
          piCycle: createBpsRatioPattern(this, 'pi_cycle'),
          macd: {
            _24h: createEmaHistogramLineSignalPattern(this, 'macd'),
            _1w: {
              emaFast: createMetricPattern1(this, 'macd_ema_fast_1w'),
              emaSlow: createMetricPattern1(this, 'macd_ema_slow_1w'),
              line: createMetricPattern1(this, 'macd_line_1w'),
              signal: createMetricPattern1(this, 'macd_signal_1w'),
              histogram: createMetricPattern1(this, 'macd_histogram_1w'),
            },
            _1m: {
              emaFast: createMetricPattern1(this, 'macd_ema_fast_1m'),
              emaSlow: createMetricPattern1(this, 'macd_ema_slow_1m'),
              line: createMetricPattern1(this, 'macd_line_1m'),
              signal: createMetricPattern1(this, 'macd_signal_1m'),
              histogram: createMetricPattern1(this, 'macd_histogram_1m'),
            },
            _1y: {
              emaFast: createMetricPattern1(this, 'macd_ema_fast_1y'),
              emaSlow: createMetricPattern1(this, 'macd_ema_slow_1y'),
              line: createMetricPattern1(this, 'macd_line_1y'),
              signal: createMetricPattern1(this, 'macd_signal_1y'),
              histogram: createMetricPattern1(this, 'macd_histogram_1y'),
            },
          },
          gini: createBpsPercentRatioPattern(this, 'gini'),
        },
      },
      pools: {
        heightToPool: createMetricPattern18(this, 'pool'),
        vecs: {
          unknown: createBlocksDominanceRewardsPattern(this, 'unknown'),
          blockfills: createBlocksDominanceRewardsPattern(this, 'blockfills'),
          ultimuspool: createBlocksDominanceRewardsPattern(this, 'ultimuspool'),
          terrapool: createBlocksDominanceRewardsPattern(this, 'terrapool'),
          luxor: createBlocksDominanceRewardsPattern(this, 'luxor'),
          onethash: createBlocksDominanceRewardsPattern(this, 'onethash'),
          btccom: createBlocksDominanceRewardsPattern(this, 'btccom'),
          bitfarms: createBlocksDominanceRewardsPattern(this, 'bitfarms'),
          huobipool: createBlocksDominanceRewardsPattern(this, 'huobipool'),
          wayicn: createBlocksDominanceRewardsPattern(this, 'wayicn'),
          canoepool: createBlocksDominanceRewardsPattern(this, 'canoepool'),
          btctop: createBlocksDominanceRewardsPattern(this, 'btctop'),
          bitcoincom: createBlocksDominanceRewardsPattern(this, 'bitcoincom'),
          pool175btc: createBlocksDominanceRewardsPattern(this, 'pool175btc'),
          gbminers: createBlocksDominanceRewardsPattern(this, 'gbminers'),
          axbt: createBlocksDominanceRewardsPattern(this, 'axbt'),
          asicminer: createBlocksDominanceRewardsPattern(this, 'asicminer'),
          bitminter: createBlocksDominanceRewardsPattern(this, 'bitminter'),
          bitcoinrussia: createBlocksDominanceRewardsPattern(this, 'bitcoinrussia'),
          btcserv: createBlocksDominanceRewardsPattern(this, 'btcserv'),
          simplecoinus: createBlocksDominanceRewardsPattern(this, 'simplecoinus'),
          btcguild: createBlocksDominanceRewardsPattern(this, 'btcguild'),
          eligius: createBlocksDominanceRewardsPattern(this, 'eligius'),
          ozcoin: createBlocksDominanceRewardsPattern(this, 'ozcoin'),
          eclipsemc: createBlocksDominanceRewardsPattern(this, 'eclipsemc'),
          maxbtc: createBlocksDominanceRewardsPattern(this, 'maxbtc'),
          triplemining: createBlocksDominanceRewardsPattern(this, 'triplemining'),
          coinlab: createBlocksDominanceRewardsPattern(this, 'coinlab'),
          pool50btc: createBlocksDominanceRewardsPattern(this, 'pool50btc'),
          ghashio: createBlocksDominanceRewardsPattern(this, 'ghashio'),
          stminingcorp: createBlocksDominanceRewardsPattern(this, 'stminingcorp'),
          bitparking: createBlocksDominanceRewardsPattern(this, 'bitparking'),
          mmpool: createBlocksDominanceRewardsPattern(this, 'mmpool'),
          polmine: createBlocksDominanceRewardsPattern(this, 'polmine'),
          kncminer: createBlocksDominanceRewardsPattern(this, 'kncminer'),
          bitalo: createBlocksDominanceRewardsPattern(this, 'bitalo'),
          f2pool: createBlocksDominanceRewardsPattern(this, 'f2pool'),
          hhtt: createBlocksDominanceRewardsPattern(this, 'hhtt'),
          megabigpower: createBlocksDominanceRewardsPattern(this, 'megabigpower'),
          mtred: createBlocksDominanceRewardsPattern(this, 'mtred'),
          nmcbit: createBlocksDominanceRewardsPattern(this, 'nmcbit'),
          yourbtcnet: createBlocksDominanceRewardsPattern(this, 'yourbtcnet'),
          givemecoins: createBlocksDominanceRewardsPattern(this, 'givemecoins'),
          braiinspool: createBlocksDominanceRewardsPattern(this, 'braiinspool'),
          antpool: createBlocksDominanceRewardsPattern(this, 'antpool'),
          multicoinco: createBlocksDominanceRewardsPattern(this, 'multicoinco'),
          bcpoolio: createBlocksDominanceRewardsPattern(this, 'bcpoolio'),
          cointerra: createBlocksDominanceRewardsPattern(this, 'cointerra'),
          kanopool: createBlocksDominanceRewardsPattern(this, 'kanopool'),
          solock: createBlocksDominanceRewardsPattern(this, 'solock'),
          ckpool: createBlocksDominanceRewardsPattern(this, 'ckpool'),
          nicehash: createBlocksDominanceRewardsPattern(this, 'nicehash'),
          bitclub: createBlocksDominanceRewardsPattern(this, 'bitclub'),
          bitcoinaffiliatenetwork: createBlocksDominanceRewardsPattern(this, 'bitcoinaffiliatenetwork'),
          btcc: createBlocksDominanceRewardsPattern(this, 'btcc'),
          bwpool: createBlocksDominanceRewardsPattern(this, 'bwpool'),
          exxbw: createBlocksDominanceRewardsPattern(this, 'exxbw'),
          bitsolo: createBlocksDominanceRewardsPattern(this, 'bitsolo'),
          bitfury: createBlocksDominanceRewardsPattern(this, 'bitfury'),
          twentyoneinc: createBlocksDominanceRewardsPattern(this, 'twentyoneinc'),
          digitalbtc: createBlocksDominanceRewardsPattern(this, 'digitalbtc'),
          eightbaochi: createBlocksDominanceRewardsPattern(this, 'eightbaochi'),
          mybtccoinpool: createBlocksDominanceRewardsPattern(this, 'mybtccoinpool'),
          tbdice: createBlocksDominanceRewardsPattern(this, 'tbdice'),
          hashpool: createBlocksDominanceRewardsPattern(this, 'hashpool'),
          nexious: createBlocksDominanceRewardsPattern(this, 'nexious'),
          bravomining: createBlocksDominanceRewardsPattern(this, 'bravomining'),
          hotpool: createBlocksDominanceRewardsPattern(this, 'hotpool'),
          okexpool: createBlocksDominanceRewardsPattern(this, 'okexpool'),
          bcmonster: createBlocksDominanceRewardsPattern(this, 'bcmonster'),
          onehash: createBlocksDominanceRewardsPattern(this, 'onehash'),
          bixin: createBlocksDominanceRewardsPattern(this, 'bixin'),
          tatmaspool: createBlocksDominanceRewardsPattern(this, 'tatmaspool'),
          viabtc: createBlocksDominanceRewardsPattern(this, 'viabtc'),
          connectbtc: createBlocksDominanceRewardsPattern(this, 'connectbtc'),
          batpool: createBlocksDominanceRewardsPattern(this, 'batpool'),
          waterhole: createBlocksDominanceRewardsPattern(this, 'waterhole'),
          dcexploration: createBlocksDominanceRewardsPattern(this, 'dcexploration'),
          dcex: createBlocksDominanceRewardsPattern(this, 'dcex'),
          btpool: createBlocksDominanceRewardsPattern(this, 'btpool'),
          fiftyeightcoin: createBlocksDominanceRewardsPattern(this, 'fiftyeightcoin'),
          bitcoinindia: createBlocksDominanceRewardsPattern(this, 'bitcoinindia'),
          shawnp0wers: createBlocksDominanceRewardsPattern(this, 'shawnp0wers'),
          phashio: createBlocksDominanceRewardsPattern(this, 'phashio'),
          rigpool: createBlocksDominanceRewardsPattern(this, 'rigpool'),
          haozhuzhu: createBlocksDominanceRewardsPattern(this, 'haozhuzhu'),
          sevenpool: createBlocksDominanceRewardsPattern(this, 'sevenpool'),
          miningkings: createBlocksDominanceRewardsPattern(this, 'miningkings'),
          hashbx: createBlocksDominanceRewardsPattern(this, 'hashbx'),
          dpool: createBlocksDominanceRewardsPattern(this, 'dpool'),
          rawpool: createBlocksDominanceRewardsPattern(this, 'rawpool'),
          haominer: createBlocksDominanceRewardsPattern(this, 'haominer'),
          helix: createBlocksDominanceRewardsPattern(this, 'helix'),
          bitcoinukraine: createBlocksDominanceRewardsPattern(this, 'bitcoinukraine'),
          poolin: createBlocksDominanceRewardsPattern(this, 'poolin'),
          secretsuperstar: createBlocksDominanceRewardsPattern(this, 'secretsuperstar'),
          tigerpoolnet: createBlocksDominanceRewardsPattern(this, 'tigerpoolnet'),
          sigmapoolcom: createBlocksDominanceRewardsPattern(this, 'sigmapoolcom'),
          okpooltop: createBlocksDominanceRewardsPattern(this, 'okpooltop'),
          hummerpool: createBlocksDominanceRewardsPattern(this, 'hummerpool'),
          tangpool: createBlocksDominanceRewardsPattern(this, 'tangpool'),
          bytepool: createBlocksDominanceRewardsPattern(this, 'bytepool'),
          spiderpool: createBlocksDominanceRewardsPattern(this, 'spiderpool'),
          novablock: createBlocksDominanceRewardsPattern(this, 'novablock'),
          miningcity: createBlocksDominanceRewardsPattern(this, 'miningcity'),
          binancepool: createBlocksDominanceRewardsPattern(this, 'binancepool'),
          minerium: createBlocksDominanceRewardsPattern(this, 'minerium'),
          lubiancom: createBlocksDominanceRewardsPattern(this, 'lubiancom'),
          okkong: createBlocksDominanceRewardsPattern(this, 'okkong'),
          aaopool: createBlocksDominanceRewardsPattern(this, 'aaopool'),
          emcdpool: createBlocksDominanceRewardsPattern(this, 'emcdpool'),
          foundryusa: createBlocksDominanceRewardsPattern(this, 'foundryusa'),
          sbicrypto: createBlocksDominanceRewardsPattern(this, 'sbicrypto'),
          arkpool: createBlocksDominanceRewardsPattern(this, 'arkpool'),
          purebtccom: createBlocksDominanceRewardsPattern(this, 'purebtccom'),
          marapool: createBlocksDominanceRewardsPattern(this, 'marapool'),
          kucoinpool: createBlocksDominanceRewardsPattern(this, 'kucoinpool'),
          entrustcharitypool: createBlocksDominanceRewardsPattern(this, 'entrustcharitypool'),
          okminer: createBlocksDominanceRewardsPattern(this, 'okminer'),
          titan: createBlocksDominanceRewardsPattern(this, 'titan'),
          pegapool: createBlocksDominanceRewardsPattern(this, 'pegapool'),
          btcnuggets: createBlocksDominanceRewardsPattern(this, 'btcnuggets'),
          cloudhashing: createBlocksDominanceRewardsPattern(this, 'cloudhashing'),
          digitalxmintsy: createBlocksDominanceRewardsPattern(this, 'digitalxmintsy'),
          telco214: createBlocksDominanceRewardsPattern(this, 'telco214'),
          btcpoolparty: createBlocksDominanceRewardsPattern(this, 'btcpoolparty'),
          multipool: createBlocksDominanceRewardsPattern(this, 'multipool'),
          transactioncoinmining: createBlocksDominanceRewardsPattern(this, 'transactioncoinmining'),
          btcdig: createBlocksDominanceRewardsPattern(this, 'btcdig'),
          trickysbtcpool: createBlocksDominanceRewardsPattern(this, 'trickysbtcpool'),
          btcmp: createBlocksDominanceRewardsPattern(this, 'btcmp'),
          eobot: createBlocksDominanceRewardsPattern(this, 'eobot'),
          unomp: createBlocksDominanceRewardsPattern(this, 'unomp'),
          patels: createBlocksDominanceRewardsPattern(this, 'patels'),
          gogreenlight: createBlocksDominanceRewardsPattern(this, 'gogreenlight'),
          bitcoinindiapool: createBlocksDominanceRewardsPattern(this, 'bitcoinindiapool'),
          ekanembtc: createBlocksDominanceRewardsPattern(this, 'ekanembtc'),
          canoe: createBlocksDominanceRewardsPattern(this, 'canoe'),
          tiger: createBlocksDominanceRewardsPattern(this, 'tiger'),
          onem1x: createBlocksDominanceRewardsPattern(this, 'onem1x'),
          zulupool: createBlocksDominanceRewardsPattern(this, 'zulupool'),
          secpool: createBlocksDominanceRewardsPattern(this, 'secpool'),
          ocean: createBlocksDominanceRewardsPattern(this, 'ocean'),
          whitepool: createBlocksDominanceRewardsPattern(this, 'whitepool'),
          wiz: createBlocksDominanceRewardsPattern(this, 'wiz'),
          wk057: createBlocksDominanceRewardsPattern(this, 'wk057'),
          futurebitapollosolo: createBlocksDominanceRewardsPattern(this, 'futurebitapollosolo'),
          carbonnegative: createBlocksDominanceRewardsPattern(this, 'carbonnegative'),
          portlandhodl: createBlocksDominanceRewardsPattern(this, 'portlandhodl'),
          phoenix: createBlocksDominanceRewardsPattern(this, 'phoenix'),
          neopool: createBlocksDominanceRewardsPattern(this, 'neopool'),
          maxipool: createBlocksDominanceRewardsPattern(this, 'maxipool'),
          bitfufupool: createBlocksDominanceRewardsPattern(this, 'bitfufupool'),
          gdpool: createBlocksDominanceRewardsPattern(this, 'gdpool'),
          miningdutch: createBlocksDominanceRewardsPattern(this, 'miningdutch'),
          publicpool: createBlocksDominanceRewardsPattern(this, 'publicpool'),
          miningsquared: createBlocksDominanceRewardsPattern(this, 'miningsquared'),
          innopolistech: createBlocksDominanceRewardsPattern(this, 'innopolistech'),
          btclab: createBlocksDominanceRewardsPattern(this, 'btclab'),
          parasite: createBlocksDominanceRewardsPattern(this, 'parasite'),
          redrockpool: createBlocksDominanceRewardsPattern(this, 'redrockpool'),
          est3lar: createBlocksDominanceRewardsPattern(this, 'est3lar'),
        },
      },
      prices: {
        split: {
          open: createCentsSatsUsdPattern2(this, 'price_open'),
          high: createCentsSatsUsdPattern2(this, 'price_high'),
          low: createCentsSatsUsdPattern2(this, 'price_low'),
          close: {
            cents: createMetricPattern2(this, 'price_close_cents'),
            usd: createMetricPattern2(this, 'price_close'),
            sats: createMetricPattern2(this, 'price_close_sats'),
          },
        },
        ohlc: {
          cents: createMetricPattern2(this, 'price_ohlc_cents'),
          usd: createMetricPattern2(this, 'price_ohlc'),
          sats: createMetricPattern2(this, 'price_ohlc_sats'),
        },
        price: {
          cents: createMetricPattern1(this, 'price_cents'),
          usd: createMetricPattern1(this, 'price'),
          sats: createMetricPattern1(this, 'price_sats'),
        },
      },
      distribution: {
        supplyState: createMetricPattern18(this, 'supply_state'),
        anyAddressIndexes: {
          p2a: createMetricPattern24(this, 'anyaddressindex'),
          p2pk33: createMetricPattern26(this, 'anyaddressindex'),
          p2pk65: createMetricPattern27(this, 'anyaddressindex'),
          p2pkh: createMetricPattern28(this, 'anyaddressindex'),
          p2sh: createMetricPattern29(this, 'anyaddressindex'),
          p2tr: createMetricPattern30(this, 'anyaddressindex'),
          p2wpkh: createMetricPattern31(this, 'anyaddressindex'),
          p2wsh: createMetricPattern32(this, 'anyaddressindex'),
        },
        addressesData: {
          funded: createMetricPattern34(this, 'fundedaddressdata'),
          empty: createMetricPattern35(this, 'emptyaddressdata'),
        },
        utxoCohorts: {
          all: {
            supply: createChangeHalvedTotalPattern(this, 'supply'),
            outputs: createUtxoPattern(this, 'utxo_count'),
            activity: createCoinblocksCoindaysSentPattern(this, ''),
            realized: createAdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(this, ''),
            costBasis: createInvestedMaxMinPercentilesPattern(this, ''),
            unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(this, ''),
            relative: {
              supplyInProfitRelToOwnSupply: createBpsPercentRatioPattern(this, 'supply_in_profit_rel_to_own_supply'),
              supplyInLossRelToOwnSupply: createBpsPercentRatioPattern(this, 'supply_in_loss_rel_to_own_supply'),
              unrealizedProfitRelToMarketCap: createBpsPercentRatioPattern(this, 'unrealized_profit_rel_to_market_cap'),
              unrealizedLossRelToMarketCap: createBpsPercentRatioPattern(this, 'unrealized_loss_rel_to_market_cap'),
              negUnrealizedLossRelToMarketCap: createBpsPercentRatioPattern(this, 'neg_unrealized_loss_rel_to_market_cap'),
              netUnrealizedPnlRelToMarketCap: createBpsPercentRatioPattern(this, 'net_unrealized_pnl_rel_to_market_cap'),
              nupl: createMetricPattern1(this, 'nupl'),
              investedCapitalInProfitRelToRealizedCap: createBpsPercentRatioPattern(this, 'invested_capital_in_profit_rel_to_realized_cap'),
              investedCapitalInLossRelToRealizedCap: createBpsPercentRatioPattern(this, 'invested_capital_in_loss_rel_to_realized_cap'),
              unrealizedProfitRelToOwnGrossPnl: createBpsPercentRatioPattern(this, 'unrealized_profit_rel_to_own_gross_pnl'),
              unrealizedLossRelToOwnGrossPnl: createBpsPercentRatioPattern(this, 'unrealized_loss_rel_to_own_gross_pnl'),
              negUnrealizedLossRelToOwnGrossPnl: createBpsPercentRatioPattern(this, 'neg_unrealized_loss_rel_to_own_gross_pnl'),
              netUnrealizedPnlRelToOwnGrossPnl: createBpsPercentRatioPattern(this, 'net_unrealized_pnl_rel_to_own_gross_pnl'),
            },
          },
          sth: {
            supply: createChangeHalvedTotalPattern(this, 'sth_supply'),
            outputs: createUtxoPattern(this, 'sth_utxo_count'),
            activity: createCoinblocksCoindaysSentPattern(this, 'sth'),
            realized: createAdjustedCapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(this, 'sth'),
            costBasis: createInvestedMaxMinPercentilesPattern(this, 'sth'),
            unrealized: createGreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(this, 'sth'),
            relative: createInvestedNegNetNuplSupplyUnrealizedPattern2(this, 'sth'),
          },
          lth: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'lth'),
          ageRange: {
            upTo1h: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_under_1h_old'),
            _1hTo1d: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_1h_to_1d_old'),
            _1dTo1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_1d_to_1w_old'),
            _1wTo1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_1w_to_1m_old'),
            _1mTo2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_1m_to_2m_old'),
            _2mTo3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_2m_to_3m_old'),
            _3mTo4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_3m_to_4m_old'),
            _4mTo5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_4m_to_5m_old'),
            _5mTo6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_5m_to_6m_old'),
            _6mTo1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_6m_to_1y_old'),
            _1yTo2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_1y_to_2y_old'),
            _2yTo3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_2y_to_3y_old'),
            _3yTo4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_3y_to_4y_old'),
            _4yTo5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_4y_to_5y_old'),
            _5yTo6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_5y_to_6y_old'),
            _6yTo7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_6y_to_7y_old'),
            _7yTo8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_7y_to_8y_old'),
            _8yTo10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_8y_to_10y_old'),
            _10yTo12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_10y_to_12y_old'),
            _12yTo15y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_12y_to_15y_old'),
            from15y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'utxos_over_15y_old'),
          },
          maxAge: {
            _1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1w_old'),
            _1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1m_old'),
            _2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_2m_old'),
            _3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_3m_old'),
            _4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_4m_old'),
            _5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_5m_old'),
            _6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_6m_old'),
            _1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1y_old'),
            _2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_2y_old'),
            _3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_3y_old'),
            _4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_4y_old'),
            _5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_5y_old'),
            _6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_6y_old'),
            _7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_7y_old'),
            _8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_8y_old'),
            _10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10y_old'),
            _12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_12y_old'),
            _15y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_15y_old'),
          },
          minAge: {
            _1d: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1d_old'),
            _1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1w_old'),
            _1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1m_old'),
            _2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_2m_old'),
            _3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_3m_old'),
            _4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_4m_old'),
            _5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_5m_old'),
            _6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_6m_old'),
            _1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1y_old'),
            _2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_2y_old'),
            _3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_3y_old'),
            _4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_4y_old'),
            _5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_5y_old'),
            _6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_6y_old'),
            _7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_7y_old'),
            _8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_8y_old'),
            _10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10y_old'),
            _12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_12y_old'),
          },
          geAmount: {
            _1sat: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1sat'),
            _10sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10sats'),
            _100sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_100sats'),
            _1kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1k_sats'),
            _10kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10k_sats'),
            _100kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_100k_sats'),
            _1mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1m_sats'),
            _10mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10m_sats'),
            _1btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1btc'),
            _10btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10btc'),
            _100btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_100btc'),
            _1kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_1k_btc'),
            _10kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_over_10k_btc'),
          },
          amountRange: {
            _0sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_with_0sats'),
            _1satTo10sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_1sat_under_10sats'),
            _10satsTo100sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_10sats_under_100sats'),
            _100satsTo1kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_100sats_under_1k_sats'),
            _1kSatsTo10kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_1k_sats_under_10k_sats'),
            _10kSatsTo100kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_10k_sats_under_100k_sats'),
            _100kSatsTo1mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_100k_sats_under_1m_sats'),
            _1mSatsTo10mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_1m_sats_under_10m_sats'),
            _10mSatsTo1btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_10m_sats_under_1btc'),
            _1btcTo10btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_1btc_under_10btc'),
            _10btcTo100btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_10btc_under_100btc'),
            _100btcTo1kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_100btc_under_1k_btc'),
            _1kBtcTo10kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_1k_btc_under_10k_btc'),
            _10kBtcTo100kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_10k_btc_under_100k_btc'),
            _100kBtcOrMore: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_above_100k_btc'),
          },
          ltAmount: {
            _10sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_10sats'),
            _100sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_100sats'),
            _1kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_1k_sats'),
            _10kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_10k_sats'),
            _100kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_100k_sats'),
            _1mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_1m_sats'),
            _10mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_10m_sats'),
            _1btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_1btc'),
            _10btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_10btc'),
            _100btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_100btc'),
            _1kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_1k_btc'),
            _10kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_10k_btc'),
            _100kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'utxos_under_100k_btc'),
          },
          epoch: {
            _0: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'epoch_0'),
            _1: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'epoch_1'),
            _2: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'epoch_2'),
            _3: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'epoch_3'),
            _4: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'epoch_4'),
          },
          class: {
            _2009: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2009'),
            _2010: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2010'),
            _2011: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2011'),
            _2012: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2012'),
            _2013: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2013'),
            _2014: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2014'),
            _2015: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2015'),
            _2016: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2016'),
            _2017: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2017'),
            _2018: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2018'),
            _2019: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2019'),
            _2020: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2020'),
            _2021: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2021'),
            _2022: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2022'),
            _2023: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2023'),
            _2024: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2024'),
            _2025: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2025'),
            _2026: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'class_2026'),
          },
          type: {
            p2pk65: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pk65'),
            p2pk33: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pk33'),
            p2pkh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pkh'),
            p2ms: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2ms'),
            p2sh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2sh'),
            p2wpkh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2wpkh'),
            p2wsh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2wsh'),
            p2tr: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2tr'),
            p2a: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2a'),
            unknown: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'unknown_outputs'),
            empty: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'empty_outputs'),
          },
        },
        addressCohorts: {
          geAmount: {
            _1sat: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_1sat'),
            _10sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_10sats'),
            _100sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_100sats'),
            _1kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_1k_sats'),
            _10kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_10k_sats'),
            _100kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_100k_sats'),
            _1mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_1m_sats'),
            _10mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_10m_sats'),
            _1btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_1btc'),
            _10btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_10btc'),
            _100btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_100btc'),
            _1kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_1k_btc'),
            _10kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_over_10k_btc'),
          },
          amountRange: {
            _0sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_with_0sats'),
            _1satTo10sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_1sat_under_10sats'),
            _10satsTo100sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_10sats_under_100sats'),
            _100satsTo1kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_100sats_under_1k_sats'),
            _1kSatsTo10kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_1k_sats_under_10k_sats'),
            _10kSatsTo100kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_10k_sats_under_100k_sats'),
            _100kSatsTo1mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_100k_sats_under_1m_sats'),
            _1mSatsTo10mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_1m_sats_under_10m_sats'),
            _10mSatsTo1btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_10m_sats_under_1btc'),
            _1btcTo10btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_1btc_under_10btc'),
            _10btcTo100btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_10btc_under_100btc'),
            _100btcTo1kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_100btc_under_1k_btc'),
            _1kBtcTo10kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_1k_btc_under_10k_btc'),
            _10kBtcTo100kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_10k_btc_under_100k_btc'),
            _100kBtcOrMore: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_above_100k_btc'),
          },
          ltAmount: {
            _10sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_10sats'),
            _100sats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_100sats'),
            _1kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_1k_sats'),
            _10kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_10k_sats'),
            _100kSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_100k_sats'),
            _1mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_1m_sats'),
            _10mSats: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_10m_sats'),
            _1btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_1btc'),
            _10btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_10btc'),
            _100btc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_100btc'),
            _1kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_1k_btc'),
            _10kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_10k_btc'),
            _100kBtc: createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(this, 'addrs_under_100k_btc'),
          },
        },
        addrCount: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(this, 'addr_count'),
        emptyAddrCount: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(this, 'empty_addr_count'),
        addressActivity: {
          all: createBalanceBothReactivatedReceivingSendingPattern(this, 'address_activity'),
          p2pk65: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2pk65_address_activity'),
          p2pk33: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2pk33_address_activity'),
          p2pkh: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2pkh_address_activity'),
          p2sh: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2sh_address_activity'),
          p2wpkh: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2wpkh_address_activity'),
          p2wsh: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2wsh_address_activity'),
          p2tr: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2tr_address_activity'),
          p2a: createBalanceBothReactivatedReceivingSendingPattern(this, 'p2a_address_activity'),
        },
        totalAddrCount: {
          all: createMetricPattern1(this, 'total_addr_count'),
          p2pk65: createMetricPattern1(this, 'p2pk65_total_addr_count'),
          p2pk33: createMetricPattern1(this, 'p2pk33_total_addr_count'),
          p2pkh: createMetricPattern1(this, 'p2pkh_total_addr_count'),
          p2sh: createMetricPattern1(this, 'p2sh_total_addr_count'),
          p2wpkh: createMetricPattern1(this, 'p2wpkh_total_addr_count'),
          p2wsh: createMetricPattern1(this, 'p2wsh_total_addr_count'),
          p2tr: createMetricPattern1(this, 'p2tr_total_addr_count'),
          p2a: createMetricPattern1(this, 'p2a_total_addr_count'),
        },
        newAddrCount: {
          all: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'new_addr_count'),
          p2pk65: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2pk65_new_addr_count'),
          p2pk33: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2pk33_new_addr_count'),
          p2pkh: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2pkh_new_addr_count'),
          p2sh: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2sh_new_addr_count'),
          p2wpkh: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2wpkh_new_addr_count'),
          p2wsh: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2wsh_new_addr_count'),
          p2tr: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2tr_new_addr_count'),
          p2a: createAverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'p2a_new_addr_count'),
        },
        growthRate: {
          all: createBpsPercentRatioPattern2(this, 'growth_rate'),
          p2pk65: createBpsPercentRatioPattern2(this, 'p2pk65_growth_rate'),
          p2pk33: createBpsPercentRatioPattern2(this, 'p2pk33_growth_rate'),
          p2pkh: createBpsPercentRatioPattern2(this, 'p2pkh_growth_rate'),
          p2sh: createBpsPercentRatioPattern2(this, 'p2sh_growth_rate'),
          p2wpkh: createBpsPercentRatioPattern2(this, 'p2wpkh_growth_rate'),
          p2wsh: createBpsPercentRatioPattern2(this, 'p2wsh_growth_rate'),
          p2tr: createBpsPercentRatioPattern2(this, 'p2tr_growth_rate'),
          p2a: createBpsPercentRatioPattern2(this, 'p2a_growth_rate'),
        },
        fundedaddressindex: createMetricPattern34(this, 'fundedaddressindex'),
        emptyaddressindex: createMetricPattern35(this, 'emptyaddressindex'),
      },
      supply: {
        circulating: createBtcCentsSatsUsdPattern(this, 'circulating_supply'),
        burned: {
          opreturn: createBaseCumulativeSumPattern(this, 'opreturn_supply'),
          unspendable: createBaseCumulativeSumPattern(this, 'unspendable_supply'),
        },
        inflationRate: createBpsPercentRatioPattern(this, 'inflation_rate'),
        velocity: {
          btc: createMetricPattern1(this, 'velocity_btc'),
          usd: createMetricPattern1(this, 'velocity_usd'),
        },
        marketCap: createMetricPattern1(this, 'market_cap'),
        marketCapGrowthRate: createBpsPercentRatioPattern(this, 'market_cap_growth_rate'),
        realizedCapGrowthRate: createBpsPercentRatioPattern(this, 'realized_cap_growth_rate'),
        marketMinusRealizedCapGrowthRate: createMetricPattern1(this, 'market_minus_realized_cap_growth_rate'),
      },
    };
  }

  /**
   * Create a dynamic metric endpoint builder for any metric/index combination.
   *
   * Use this for programmatic access when the metric name is determined at runtime.
   * For type-safe access, use the `metrics` tree instead.
   *
   * @param {string} metric - The metric name
   * @param {Index} index - The index name
   * @returns {MetricEndpointBuilder<unknown>}
   */
  metric(metric, index) {
    return _endpoint(this, metric, index);
  }

  /**
   * Compact OpenAPI specification
   *
   * Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.
   *
   * Endpoint: `GET /api.json`
   * @returns {Promise<*>}
   */
  async getApi() {
    return this.getJson(`/api.json`);
  }

  /**
   * Address information
   *
   * Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*
   *
   * Endpoint: `GET /api/address/{address}`
   *
   * @param {Address} address
   * @returns {Promise<AddressStats>}
   */
  async getAddress(address) {
    return this.getJson(`/api/address/${address}`);
  }

  /**
   * Address transaction IDs
   *
   * Get transaction IDs for an address, newest first. Use after_txid for pagination.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*
   *
   * Endpoint: `GET /api/address/{address}/txs`
   *
   * @param {Address} address
   * @param {string=} [after_txid] - Txid to paginate from (return transactions before this one)
   * @param {number=} [limit] - Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getAddressTxs(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    const path = `/api/address/${address}/txs${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Address confirmed transactions
   *
   * Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*
   *
   * Endpoint: `GET /api/address/{address}/txs/chain`
   *
   * @param {Address} address
   * @param {string=} [after_txid] - Txid to paginate from (return transactions before this one)
   * @param {number=} [limit] - Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getAddressConfirmedTxs(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    const path = `/api/address/${address}/txs/chain${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Address mempool transactions
   *
   * Get unconfirmed transaction IDs for an address from the mempool (up to 50).
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*
   *
   * Endpoint: `GET /api/address/{address}/txs/mempool`
   *
   * @param {Address} address
   * @returns {Promise<Txid[]>}
   */
  async getAddressMempoolTxs(address) {
    return this.getJson(`/api/address/${address}/txs/mempool`);
  }

  /**
   * Address UTXOs
   *
   * Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*
   *
   * Endpoint: `GET /api/address/{address}/utxo`
   *
   * @param {Address} address
   * @returns {Promise<Utxo[]>}
   */
  async getAddressUtxos(address) {
    return this.getJson(`/api/address/${address}/utxo`);
  }

  /**
   * Block by height
   *
   * Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*
   *
   * Endpoint: `GET /api/block-height/{height}`
   *
   * @param {Height} height
   * @returns {Promise<BlockInfo>}
   */
  async getBlockByHeight(height) {
    return this.getJson(`/api/block-height/${height}`);
  }

  /**
   * Block information
   *
   * Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*
   *
   * Endpoint: `GET /api/block/{hash}`
   *
   * @param {BlockHash} hash
   * @returns {Promise<BlockInfo>}
   */
  async getBlock(hash) {
    return this.getJson(`/api/block/${hash}`);
  }

  /**
   * Raw block
   *
   * Returns the raw block data in binary format.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*
   *
   * Endpoint: `GET /api/block/{hash}/raw`
   *
   * @param {BlockHash} hash
   * @returns {Promise<number[]>}
   */
  async getBlockRaw(hash) {
    return this.getJson(`/api/block/${hash}/raw`);
  }

  /**
   * Block status
   *
   * Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*
   *
   * Endpoint: `GET /api/block/{hash}/status`
   *
   * @param {BlockHash} hash
   * @returns {Promise<BlockStatus>}
   */
  async getBlockStatus(hash) {
    return this.getJson(`/api/block/${hash}/status`);
  }

  /**
   * Transaction ID at index
   *
   * Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-id)*
   *
   * Endpoint: `GET /api/block/{hash}/txid/{index}`
   *
   * @param {BlockHash} hash - Bitcoin block hash
   * @param {TxIndex} index - Transaction index within the block (0-based)
   * @returns {Promise<Txid>}
   */
  async getBlockTxid(hash, index) {
    return this.getJson(`/api/block/${hash}/txid/${index}`);
  }

  /**
   * Block transaction IDs
   *
   * Retrieve all transaction IDs in a block. Returns an array of txids in block order.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*
   *
   * Endpoint: `GET /api/block/{hash}/txids`
   *
   * @param {BlockHash} hash
   * @returns {Promise<Txid[]>}
   */
  async getBlockTxids(hash) {
    return this.getJson(`/api/block/${hash}/txids`);
  }

  /**
   * Block transactions (paginated)
   *
   * Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*
   *
   * Endpoint: `GET /api/block/{hash}/txs/{start_index}`
   *
   * @param {BlockHash} hash - Bitcoin block hash
   * @param {TxIndex} start_index - Starting transaction index within the block (0-based)
   * @returns {Promise<Transaction[]>}
   */
  async getBlockTxs(hash, start_index) {
    return this.getJson(`/api/block/${hash}/txs/${start_index}`);
  }

  /**
   * Recent blocks
   *
   * Retrieve the last 10 blocks. Returns block metadata for each block.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*
   *
   * Endpoint: `GET /api/blocks`
   * @returns {Promise<BlockInfo[]>}
   */
  async getBlocks() {
    return this.getJson(`/api/blocks`);
  }

  /**
   * Blocks from height
   *
   * Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*
   *
   * Endpoint: `GET /api/blocks/{height}`
   *
   * @param {Height} height
   * @returns {Promise<BlockInfo[]>}
   */
  async getBlocksFromHeight(height) {
    return this.getJson(`/api/blocks/${height}`);
  }

  /**
   * Mempool statistics
   *
   * Get current mempool statistics including transaction count, total vsize, and total fees.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*
   *
   * Endpoint: `GET /api/mempool/info`
   * @returns {Promise<MempoolInfo>}
   */
  async getMempool() {
    return this.getJson(`/api/mempool/info`);
  }

  /**
   * Live BTC/USD price
   *
   * Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.
   *
   * Endpoint: `GET /api/mempool/price`
   * @returns {Promise<Dollars>}
   */
  async getLivePrice() {
    return this.getJson(`/api/mempool/price`);
  }

  /**
   * Mempool transaction IDs
   *
   * Get all transaction IDs currently in the mempool.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*
   *
   * Endpoint: `GET /api/mempool/txids`
   * @returns {Promise<Txid[]>}
   */
  async getMempoolTxids() {
    return this.getJson(`/api/mempool/txids`);
  }

  /**
   * Get supported indexes for a metric
   *
   * Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on day1, week1, and month1.
   *
   * Endpoint: `GET /api/metric/{metric}`
   *
   * @param {Metric} metric
   * @returns {Promise<Index[]>}
   */
  async getMetricInfo(metric) {
    return this.getJson(`/api/metric/${metric}`);
  }

  /**
   * Get metric data
   *
   * Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).
   *
   * Endpoint: `GET /api/metric/{metric}/{index}`
   *
   * @param {Metric} metric - Metric name
   * @param {Index} index - Aggregation index
   * @param {number=} [start] - Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
   * @param {number=} [end] - Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
   * @param {string=} [limit] - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
   * @param {Format=} [format] - Format of the output
   * @returns {Promise<AnyMetricData | string>}
   */
  async getMetric(metric, index, start, end, limit, format) {
    const params = new URLSearchParams();
    if (start !== undefined) params.set('start', String(start));
    if (end !== undefined) params.set('end', String(end));
    if (limit !== undefined) params.set('limit', String(limit));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    const path = `/api/metric/${metric}/${index}${query ? '?' + query : ''}`;
    if (format === 'csv') {
      return this.getText(path);
    }
    return this.getJson(path);
  }

  /**
   * Metrics catalog
   *
   * Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.
   *
   * Endpoint: `GET /api/metrics`
   * @returns {Promise<TreeNode>}
   */
  async getMetricsTree() {
    return this.getJson(`/api/metrics`);
  }

  /**
   * Bulk metric data
   *
   * Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects. For a single metric, use `get_metric` instead.
   *
   * Endpoint: `GET /api/metrics/bulk`
   *
   * @param {Metrics} [metrics] - Requested metrics
   * @param {Index} [index] - Index to query
   * @param {number=} [start] - Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
   * @param {number=} [end] - Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
   * @param {string=} [limit] - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
   * @param {Format=} [format] - Format of the output
   * @returns {Promise<AnyMetricData[] | string>}
   */
  async getMetrics(metrics, index, start, end, limit, format) {
    const params = new URLSearchParams();
    params.set('metrics', String(metrics));
    params.set('index', String(index));
    if (start !== undefined) params.set('start', String(start));
    if (end !== undefined) params.set('end', String(end));
    if (limit !== undefined) params.set('limit', String(limit));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    const path = `/api/metrics/bulk${query ? '?' + query : ''}`;
    if (format === 'csv') {
      return this.getText(path);
    }
    return this.getJson(path);
  }

  /**
   * Available cost basis cohorts
   *
   * List available cohorts for cost basis distribution.
   *
   * Endpoint: `GET /api/metrics/cost-basis`
   * @returns {Promise<string[]>}
   */
  async getCostBasisCohorts() {
    return this.getJson(`/api/metrics/cost-basis`);
  }

  /**
   * Available cost basis dates
   *
   * List available dates for a cohort's cost basis distribution.
   *
   * Endpoint: `GET /api/metrics/cost-basis/{cohort}/dates`
   *
   * @param {Cohort} cohort
   * @returns {Promise<Date[]>}
   */
  async getCostBasisDates(cohort) {
    return this.getJson(`/api/metrics/cost-basis/${cohort}/dates`);
  }

  /**
   * Cost basis distribution
   *
   * Get the cost basis distribution for a cohort on a specific date.
   *
   * Query params:
   * - `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
   * - `value`: supply (default, in BTC), realized (USD), unrealized (USD)
   *
   * Endpoint: `GET /api/metrics/cost-basis/{cohort}/{date}`
   *
   * @param {Cohort} cohort
   * @param {string} date
   * @param {CostBasisBucket=} [bucket] - Bucket type for aggregation. Default: raw (no aggregation).
   * @param {CostBasisValue=} [value] - Value type to return. Default: supply.
   * @returns {Promise<Object>}
   */
  async getCostBasis(cohort, date, bucket, value) {
    const params = new URLSearchParams();
    if (bucket !== undefined) params.set('bucket', String(bucket));
    if (value !== undefined) params.set('value', String(value));
    const query = params.toString();
    const path = `/api/metrics/cost-basis/${cohort}/${date}${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Metric count
   *
   * Returns the number of metrics available per index type.
   *
   * Endpoint: `GET /api/metrics/count`
   * @returns {Promise<MetricCount[]>}
   */
  async getMetricsCount() {
    return this.getJson(`/api/metrics/count`);
  }

  /**
   * List available indexes
   *
   * Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.
   *
   * Endpoint: `GET /api/metrics/indexes`
   * @returns {Promise<IndexInfo[]>}
   */
  async getIndexes() {
    return this.getJson(`/api/metrics/indexes`);
  }

  /**
   * Metrics list
   *
   * Paginated flat list of all available metric names. Use `page` query param for pagination.
   *
   * Endpoint: `GET /api/metrics/list`
   *
   * @param {number=} [page] - Pagination index
   * @returns {Promise<PaginatedMetrics>}
   */
  async listMetrics(page) {
    const params = new URLSearchParams();
    if (page !== undefined) params.set('page', String(page));
    const query = params.toString();
    const path = `/api/metrics/list${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Search metrics
   *
   * Fuzzy search for metrics by name. Supports partial matches and typos.
   *
   * Endpoint: `GET /api/metrics/search/{metric}`
   *
   * @param {Metric} metric
   * @param {Limit=} [limit]
   * @returns {Promise<Metric[]>}
   */
  async searchMetrics(metric, limit) {
    const params = new URLSearchParams();
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    const path = `/api/metrics/search/${metric}${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Disk usage
   *
   * Returns the disk space used by BRK and Bitcoin data.
   *
   * Endpoint: `GET /api/server/disk`
   * @returns {Promise<DiskUsage>}
   */
  async getDiskUsage() {
    return this.getJson(`/api/server/disk`);
  }

  /**
   * Sync status
   *
   * Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.
   *
   * Endpoint: `GET /api/server/sync`
   * @returns {Promise<SyncStatus>}
   */
  async getSyncStatus() {
    return this.getJson(`/api/server/sync`);
  }

  /**
   * Transaction information
   *
   * Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*
   *
   * Endpoint: `GET /api/tx/{txid}`
   *
   * @param {Txid} txid
   * @returns {Promise<Transaction>}
   */
  async getTx(txid) {
    return this.getJson(`/api/tx/${txid}`);
  }

  /**
   * Transaction hex
   *
   * Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*
   *
   * Endpoint: `GET /api/tx/{txid}/hex`
   *
   * @param {Txid} txid
   * @returns {Promise<Hex>}
   */
  async getTxHex(txid) {
    return this.getJson(`/api/tx/${txid}/hex`);
  }

  /**
   * Output spend status
   *
   * Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*
   *
   * Endpoint: `GET /api/tx/{txid}/outspend/{vout}`
   *
   * @param {Txid} txid - Transaction ID
   * @param {Vout} vout - Output index
   * @returns {Promise<TxOutspend>}
   */
  async getTxOutspend(txid, vout) {
    return this.getJson(`/api/tx/${txid}/outspend/${vout}`);
  }

  /**
   * All output spend statuses
   *
   * Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*
   *
   * Endpoint: `GET /api/tx/{txid}/outspends`
   *
   * @param {Txid} txid
   * @returns {Promise<TxOutspend[]>}
   */
  async getTxOutspends(txid) {
    return this.getJson(`/api/tx/${txid}/outspends`);
  }

  /**
   * Transaction status
   *
   * Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*
   *
   * Endpoint: `GET /api/tx/{txid}/status`
   *
   * @param {Txid} txid
   * @returns {Promise<TxStatus>}
   */
  async getTxStatus(txid) {
    return this.getJson(`/api/tx/${txid}/status`);
  }

  /**
   * Difficulty adjustment
   *
   * Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*
   *
   * Endpoint: `GET /api/v1/difficulty-adjustment`
   * @returns {Promise<DifficultyAdjustment>}
   */
  async getDifficultyAdjustment() {
    return this.getJson(`/api/v1/difficulty-adjustment`);
  }

  /**
   * Projected mempool blocks
   *
   * Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*
   *
   * Endpoint: `GET /api/v1/fees/mempool-blocks`
   * @returns {Promise<MempoolBlock[]>}
   */
  async getMempoolBlocks() {
    return this.getJson(`/api/v1/fees/mempool-blocks`);
  }

  /**
   * Recommended fees
   *
   * Get recommended fee rates for different confirmation targets based on current mempool state.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*
   *
   * Endpoint: `GET /api/v1/fees/recommended`
   * @returns {Promise<RecommendedFees>}
   */
  async getRecommendedFees() {
    return this.getJson(`/api/v1/fees/recommended`);
  }

  /**
   * Block fee rates (WIP)
   *
   * **Work in progress.** Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*
   *
   * Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<*>}
   */
  async getBlockFeeRates(time_period) {
    return this.getJson(`/api/v1/mining/blocks/fee-rates/${time_period}`);
  }

  /**
   * Block fees
   *
   * Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*
   *
   * Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<BlockFeesEntry[]>}
   */
  async getBlockFees(time_period) {
    return this.getJson(`/api/v1/mining/blocks/fees/${time_period}`);
  }

  /**
   * Block rewards
   *
   * Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*
   *
   * Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<BlockRewardsEntry[]>}
   */
  async getBlockRewards(time_period) {
    return this.getJson(`/api/v1/mining/blocks/rewards/${time_period}`);
  }

  /**
   * Block sizes and weights
   *
   * Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*
   *
   * Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<BlockSizesWeights>}
   */
  async getBlockSizesWeights(time_period) {
    return this.getJson(`/api/v1/mining/blocks/sizes-weights/${time_period}`);
  }

  /**
   * Block by timestamp
   *
   * Find the block closest to a given UNIX timestamp.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*
   *
   * Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`
   *
   * @param {Timestamp} timestamp
   * @returns {Promise<BlockTimestamp>}
   */
  async getBlockByTimestamp(timestamp) {
    return this.getJson(`/api/v1/mining/blocks/timestamp/${timestamp}`);
  }

  /**
   * Difficulty adjustments (all time)
   *
   * Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
   *
   * Endpoint: `GET /api/v1/mining/difficulty-adjustments`
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getDifficultyAdjustments() {
    return this.getJson(`/api/v1/mining/difficulty-adjustments`);
  }

  /**
   * Difficulty adjustments
   *
   * Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
   *
   * Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getDifficultyAdjustmentsByPeriod(time_period) {
    return this.getJson(`/api/v1/mining/difficulty-adjustments/${time_period}`);
  }

  /**
   * Network hashrate (all time)
   *
   * Get network hashrate and difficulty data for all time.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*
   *
   * Endpoint: `GET /api/v1/mining/hashrate`
   * @returns {Promise<HashrateSummary>}
   */
  async getHashrate() {
    return this.getJson(`/api/v1/mining/hashrate`);
  }

  /**
   * Network hashrate
   *
   * Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*
   *
   * Endpoint: `GET /api/v1/mining/hashrate/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<HashrateSummary>}
   */
  async getHashrateByPeriod(time_period) {
    return this.getJson(`/api/v1/mining/hashrate/${time_period}`);
  }

  /**
   * Mining pool details
   *
   * Get detailed information about a specific mining pool including block counts and shares for different time periods.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*
   *
   * Endpoint: `GET /api/v1/mining/pool/{slug}`
   *
   * @param {PoolSlug} slug
   * @returns {Promise<PoolDetail>}
   */
  async getPool(slug) {
    return this.getJson(`/api/v1/mining/pool/${slug}`);
  }

  /**
   * List all mining pools
   *
   * Get list of all known mining pools with their identifiers.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*
   *
   * Endpoint: `GET /api/v1/mining/pools`
   * @returns {Promise<PoolInfo[]>}
   */
  async getPools() {
    return this.getJson(`/api/v1/mining/pools`);
  }

  /**
   * Mining pool statistics
   *
   * Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*
   *
   * Endpoint: `GET /api/v1/mining/pools/{time_period}`
   *
   * @param {TimePeriod} time_period
   * @returns {Promise<PoolsSummary>}
   */
  async getPoolStats(time_period) {
    return this.getJson(`/api/v1/mining/pools/${time_period}`);
  }

  /**
   * Mining reward statistics
   *
   * Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*
   *
   * Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`
   *
   * @param {number} block_count - Number of recent blocks to include
   * @returns {Promise<RewardStats>}
   */
  async getRewardStats(block_count) {
    return this.getJson(`/api/v1/mining/reward-stats/${block_count}`);
  }

  /**
   * Validate address
   *
   * Validate a Bitcoin address and get information about its type and scriptPubKey.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*
   *
   * Endpoint: `GET /api/v1/validate-address/{address}`
   *
   * @param {string} address - Bitcoin address to validate (can be any string)
   * @returns {Promise<AddressValidation>}
   */
  async validateAddress(address) {
    return this.getJson(`/api/v1/validate-address/${address}`);
  }

  /**
   * Health check
   *
   * Returns the health status of the API server, including uptime information.
   *
   * Endpoint: `GET /health`
   * @returns {Promise<Health>}
   */
  async getHealth() {
    return this.getJson(`/health`);
  }

  /**
   * OpenAPI specification
   *
   * Full OpenAPI 3.1 specification for this API.
   *
   * Endpoint: `GET /openapi.json`
   * @returns {Promise<*>}
   */
  async getOpenapi() {
    return this.getJson(`/openapi.json`);
  }

  /**
   * API version
   *
   * Returns the current version of the API server
   *
   * Endpoint: `GET /version`
   * @returns {Promise<string>}
   */
  async getVersion() {
    return this.getJson(`/version`);
  }

}

export { BrkClient, BrkError };
