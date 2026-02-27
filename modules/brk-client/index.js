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
 * @property {?number=} start - Inclusive starting index, if negative counts from end
 * @property {?number=} end - Exclusive ending index, if negative counts from end
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set)
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
 * @typedef {("minute1"|"minute5"|"minute10"|"minute30"|"hour1"|"hour4"|"hour12"|"day1"|"day3"|"week1"|"month1"|"month3"|"month6"|"year1"|"year10"|"halvingepoch"|"difficultyepoch"|"height"|"txindex"|"txinindex"|"txoutindex"|"emptyoutputindex"|"opreturnindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"unknownoutputindex"|"fundedaddressindex"|"emptyaddressindex"|"pairoutputindex")} Index
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
 * @property {?number=} start - Inclusive starting index, if negative counts from end
 * @property {?number=} end - Exclusive ending index, if negative counts from end
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set)
 * @property {Format=} format - Format of the output
 */
/**
 * Legacy metric selection parameters (deprecated)
 *
 * @typedef {Object} MetricSelectionLegacy
 * @property {Index} index
 * @property {Metrics} ids
 * @property {?number=} start - Inclusive starting index, if negative counts from end
 * @property {?number=} end - Exclusive ending index, if negative counts from end
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set)
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
/** @typedef {number} Minute1 */
/** @typedef {number} Minute10 */
/** @typedef {number} Minute30 */
/** @typedef {number} Minute5 */
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
  'minute1', 'minute5', 'minute10', 'minute30',
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
    case 'minute1': return new Date(_EPOCH_MS + i * 60000);
    case 'minute5': return new Date(_EPOCH_MS + i * 300000);
    case 'minute10': return new Date(_EPOCH_MS + i * 600000);
    case 'minute30': return new Date(_EPOCH_MS + i * 1800000);
    case 'hour1': return new Date(_EPOCH_MS + i * 3600000);
    case 'hour4': return new Date(_EPOCH_MS + i * 14400000);
    case 'hour12': return new Date(_EPOCH_MS + i * 43200000);
    case 'day1': return i === 0 ? _GENESIS : new Date(_DAY_ONE.getTime() + (i - 1) * _MS_PER_DAY);
    case 'day3': return new Date(_EPOCH_MS + i * 259200000);
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
    case 'minute1': return Math.floor((ms - _EPOCH_MS) / 60000);
    case 'minute5': return Math.floor((ms - _EPOCH_MS) / 300000);
    case 'minute10': return Math.floor((ms - _EPOCH_MS) / 600000);
    case 'minute30': return Math.floor((ms - _EPOCH_MS) / 1800000);
    case 'hour1': return Math.floor((ms - _EPOCH_MS) / 3600000);
    case 'hour4': return Math.floor((ms - _EPOCH_MS) / 14400000);
    case 'hour12': return Math.floor((ms - _EPOCH_MS) / 43200000);
    case 'day1': {
      if (ms < _DAY_ONE.getTime()) return 0;
      return 1 + Math.floor((ms - _DAY_ONE.getTime()) / _MS_PER_DAY);
    }
    case 'day3': return Math.floor((ms - _EPOCH_MS) / 259200000);
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

const _i1 = /** @type {const} */ (["minute1", "minute5", "minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch", "height"]);
const _i2 = /** @type {const} */ (["minute1", "minute5", "minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch"]);
const _i3 = /** @type {const} */ (["minute1"]);
const _i4 = /** @type {const} */ (["minute5"]);
const _i5 = /** @type {const} */ (["minute10"]);
const _i6 = /** @type {const} */ (["minute30"]);
const _i7 = /** @type {const} */ (["hour1"]);
const _i8 = /** @type {const} */ (["hour4"]);
const _i9 = /** @type {const} */ (["hour12"]);
const _i10 = /** @type {const} */ (["day1"]);
const _i11 = /** @type {const} */ (["day3"]);
const _i12 = /** @type {const} */ (["week1"]);
const _i13 = /** @type {const} */ (["month1"]);
const _i14 = /** @type {const} */ (["month3"]);
const _i15 = /** @type {const} */ (["month6"]);
const _i16 = /** @type {const} */ (["year1"]);
const _i17 = /** @type {const} */ (["year10"]);
const _i18 = /** @type {const} */ (["halvingepoch"]);
const _i19 = /** @type {const} */ (["difficultyepoch"]);
const _i20 = /** @type {const} */ (["height"]);
const _i21 = /** @type {const} */ (["txindex"]);
const _i22 = /** @type {const} */ (["txinindex"]);
const _i23 = /** @type {const} */ (["txoutindex"]);
const _i24 = /** @type {const} */ (["emptyoutputindex"]);
const _i25 = /** @type {const} */ (["opreturnindex"]);
const _i26 = /** @type {const} */ (["p2aaddressindex"]);
const _i27 = /** @type {const} */ (["p2msoutputindex"]);
const _i28 = /** @type {const} */ (["p2pk33addressindex"]);
const _i29 = /** @type {const} */ (["p2pk65addressindex"]);
const _i30 = /** @type {const} */ (["p2pkhaddressindex"]);
const _i31 = /** @type {const} */ (["p2shaddressindex"]);
const _i32 = /** @type {const} */ (["p2traddressindex"]);
const _i33 = /** @type {const} */ (["p2wpkhaddressindex"]);
const _i34 = /** @type {const} */ (["p2wshaddressindex"]);
const _i35 = /** @type {const} */ (["unknownoutputindex"]);
const _i36 = /** @type {const} */ (["fundedaddressindex"]);
const _i37 = /** @type {const} */ (["emptyaddressindex"]);

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

/** @template T @typedef {{ name: string, by: { readonly minute1: DateMetricEndpointBuilder<T>, readonly minute5: DateMetricEndpointBuilder<T>, readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halvingepoch: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern1 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern1<T>} */
function createMetricPattern1(client, name) { return /** @type {MetricPattern1<T>} */ (_mp(client, name, _i1)); }
/** @template T @typedef {{ name: string, by: { readonly minute1: DateMetricEndpointBuilder<T>, readonly minute5: DateMetricEndpointBuilder<T>, readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halvingepoch: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern2 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern2<T>} */
function createMetricPattern2(client, name) { return /** @type {MetricPattern2<T>} */ (_mp(client, name, _i2)); }
/** @template T @typedef {{ name: string, by: { readonly minute1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern3 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern3<T>} */
function createMetricPattern3(client, name) { return /** @type {MetricPattern3<T>} */ (_mp(client, name, _i3)); }
/** @template T @typedef {{ name: string, by: { readonly minute5: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern4 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern4<T>} */
function createMetricPattern4(client, name) { return /** @type {MetricPattern4<T>} */ (_mp(client, name, _i4)); }
/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern5 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern5<T>} */
function createMetricPattern5(client, name) { return /** @type {MetricPattern5<T>} */ (_mp(client, name, _i5)); }
/** @template T @typedef {{ name: string, by: { readonly minute30: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern6 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern6<T>} */
function createMetricPattern6(client, name) { return /** @type {MetricPattern6<T>} */ (_mp(client, name, _i6)); }
/** @template T @typedef {{ name: string, by: { readonly hour1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern7 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern7<T>} */
function createMetricPattern7(client, name) { return /** @type {MetricPattern7<T>} */ (_mp(client, name, _i7)); }
/** @template T @typedef {{ name: string, by: { readonly hour4: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern8 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern8<T>} */
function createMetricPattern8(client, name) { return /** @type {MetricPattern8<T>} */ (_mp(client, name, _i8)); }
/** @template T @typedef {{ name: string, by: { readonly hour12: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern9 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern9<T>} */
function createMetricPattern9(client, name) { return /** @type {MetricPattern9<T>} */ (_mp(client, name, _i9)); }
/** @template T @typedef {{ name: string, by: { readonly day1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern10 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern10<T>} */
function createMetricPattern10(client, name) { return /** @type {MetricPattern10<T>} */ (_mp(client, name, _i10)); }
/** @template T @typedef {{ name: string, by: { readonly day3: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern11 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern11<T>} */
function createMetricPattern11(client, name) { return /** @type {MetricPattern11<T>} */ (_mp(client, name, _i11)); }
/** @template T @typedef {{ name: string, by: { readonly week1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern12 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern12<T>} */
function createMetricPattern12(client, name) { return /** @type {MetricPattern12<T>} */ (_mp(client, name, _i12)); }
/** @template T @typedef {{ name: string, by: { readonly month1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern13 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern13<T>} */
function createMetricPattern13(client, name) { return /** @type {MetricPattern13<T>} */ (_mp(client, name, _i13)); }
/** @template T @typedef {{ name: string, by: { readonly month3: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern14 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern14<T>} */
function createMetricPattern14(client, name) { return /** @type {MetricPattern14<T>} */ (_mp(client, name, _i14)); }
/** @template T @typedef {{ name: string, by: { readonly month6: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern15 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern15<T>} */
function createMetricPattern15(client, name) { return /** @type {MetricPattern15<T>} */ (_mp(client, name, _i15)); }
/** @template T @typedef {{ name: string, by: { readonly year1: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern16 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern16<T>} */
function createMetricPattern16(client, name) { return /** @type {MetricPattern16<T>} */ (_mp(client, name, _i16)); }
/** @template T @typedef {{ name: string, by: { readonly year10: DateMetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern17 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern17<T>} */
function createMetricPattern17(client, name) { return /** @type {MetricPattern17<T>} */ (_mp(client, name, _i17)); }
/** @template T @typedef {{ name: string, by: { readonly halvingepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern18 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern18<T>} */
function createMetricPattern18(client, name) { return /** @type {MetricPattern18<T>} */ (_mp(client, name, _i18)); }
/** @template T @typedef {{ name: string, by: { readonly difficultyepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern19 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern19<T>} */
function createMetricPattern19(client, name) { return /** @type {MetricPattern19<T>} */ (_mp(client, name, _i19)); }
/** @template T @typedef {{ name: string, by: { readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern20 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern20<T>} */
function createMetricPattern20(client, name) { return /** @type {MetricPattern20<T>} */ (_mp(client, name, _i20)); }
/** @template T @typedef {{ name: string, by: { readonly txindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern21 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern21<T>} */
function createMetricPattern21(client, name) { return /** @type {MetricPattern21<T>} */ (_mp(client, name, _i21)); }
/** @template T @typedef {{ name: string, by: { readonly txinindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern22 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern22<T>} */
function createMetricPattern22(client, name) { return /** @type {MetricPattern22<T>} */ (_mp(client, name, _i22)); }
/** @template T @typedef {{ name: string, by: { readonly txoutindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern23 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern23<T>} */
function createMetricPattern23(client, name) { return /** @type {MetricPattern23<T>} */ (_mp(client, name, _i23)); }
/** @template T @typedef {{ name: string, by: { readonly emptyoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern24 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern24<T>} */
function createMetricPattern24(client, name) { return /** @type {MetricPattern24<T>} */ (_mp(client, name, _i24)); }
/** @template T @typedef {{ name: string, by: { readonly opreturnindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern25 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern25<T>} */
function createMetricPattern25(client, name) { return /** @type {MetricPattern25<T>} */ (_mp(client, name, _i25)); }
/** @template T @typedef {{ name: string, by: { readonly p2aaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern26 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern26<T>} */
function createMetricPattern26(client, name) { return /** @type {MetricPattern26<T>} */ (_mp(client, name, _i26)); }
/** @template T @typedef {{ name: string, by: { readonly p2msoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern27 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern27<T>} */
function createMetricPattern27(client, name) { return /** @type {MetricPattern27<T>} */ (_mp(client, name, _i27)); }
/** @template T @typedef {{ name: string, by: { readonly p2pk33addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern28 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern28<T>} */
function createMetricPattern28(client, name) { return /** @type {MetricPattern28<T>} */ (_mp(client, name, _i28)); }
/** @template T @typedef {{ name: string, by: { readonly p2pk65addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern29 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern29<T>} */
function createMetricPattern29(client, name) { return /** @type {MetricPattern29<T>} */ (_mp(client, name, _i29)); }
/** @template T @typedef {{ name: string, by: { readonly p2pkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern30 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern30<T>} */
function createMetricPattern30(client, name) { return /** @type {MetricPattern30<T>} */ (_mp(client, name, _i30)); }
/** @template T @typedef {{ name: string, by: { readonly p2shaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern31 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern31<T>} */
function createMetricPattern31(client, name) { return /** @type {MetricPattern31<T>} */ (_mp(client, name, _i31)); }
/** @template T @typedef {{ name: string, by: { readonly p2traddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern32 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern32<T>} */
function createMetricPattern32(client, name) { return /** @type {MetricPattern32<T>} */ (_mp(client, name, _i32)); }
/** @template T @typedef {{ name: string, by: { readonly p2wpkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern33 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern33<T>} */
function createMetricPattern33(client, name) { return /** @type {MetricPattern33<T>} */ (_mp(client, name, _i33)); }
/** @template T @typedef {{ name: string, by: { readonly p2wshaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern34 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern34<T>} */
function createMetricPattern34(client, name) { return /** @type {MetricPattern34<T>} */ (_mp(client, name, _i34)); }
/** @template T @typedef {{ name: string, by: { readonly unknownoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern35 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern35<T>} */
function createMetricPattern35(client, name) { return /** @type {MetricPattern35<T>} */ (_mp(client, name, _i35)); }
/** @template T @typedef {{ name: string, by: { readonly fundedaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern36 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern36<T>} */
function createMetricPattern36(client, name) { return /** @type {MetricPattern36<T>} */ (_mp(client, name, _i36)); }
/** @template T @typedef {{ name: string, by: { readonly emptyaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern37 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern37<T>} */
function createMetricPattern37(client, name) { return /** @type {MetricPattern37<T>} */ (_mp(client, name, _i37)); }

// Reusable structural pattern factories

/**
 * @typedef {Object} AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern
 * @property {MetricPattern1<StoredF64>} adjustedSopr
 * @property {MetricPattern1<StoredF64>} adjustedSopr1y
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h30dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h7dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr30d
 * @property {MetricPattern1<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr7d
 * @property {MetricPattern1<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueCreated1y
 * @property {MetricPattern1<Dollars>} adjustedValueCreated24h
 * @property {MetricPattern1<Dollars>} adjustedValueCreated30d
 * @property {MetricPattern1<Dollars>} adjustedValueCreated7d
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed1y
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed24h
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed30d
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed7d
 * @property {MetricPattern20<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern20<CentsSquaredSats>} investorCapRaw
 * @property {SatsUsdPattern} investorPrice
 * @property {MetricPattern1<Cents>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {RatioPattern3} investorPriceRatioExt
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {SatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {CumulativeHeightPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern1<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern1<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {CumulativeHeightPattern<Dollars>} realizedLoss
 * @property {MetricPattern1<Dollars>} realizedLoss1y
 * @property {MetricPattern1<Dollars>} realizedLoss24h
 * @property {MetricPattern1<Dollars>} realizedLoss30d
 * @property {MetricPattern1<Dollars>} realizedLoss7d
 * @property {MetricPattern1<Dollars>} realizedLoss7dEma
 * @property {MetricPattern1<StoredF32>} realizedLossRelToRealizedCap
 * @property {SatsUsdPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {RatioPattern3} realizedPriceRatioExt
 * @property {CumulativeHeightPattern<Dollars>} realizedProfit
 * @property {MetricPattern1<Dollars>} realizedProfit1y
 * @property {MetricPattern1<Dollars>} realizedProfit24h
 * @property {MetricPattern1<Dollars>} realizedProfit30d
 * @property {MetricPattern1<Dollars>} realizedProfit7d
 * @property {MetricPattern1<Dollars>} realizedProfit7dEma
 * @property {MetricPattern1<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio1y
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio24h
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio30d
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio7d
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern1<Dollars>} realizedValue1y
 * @property {MetricPattern1<Dollars>} realizedValue24h
 * @property {MetricPattern1<Dollars>} realizedValue30d
 * @property {MetricPattern1<Dollars>} realizedValue7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio1y
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h7dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7dEma
 * @property {BtcSatsUsdPattern2} sentInLoss
 * @property {BtcSatsUsdPattern} sentInLoss14dEma
 * @property {BtcSatsUsdPattern2} sentInProfit
 * @property {BtcSatsUsdPattern} sentInProfit14dEma
 * @property {MetricPattern1<StoredF64>} sopr
 * @property {MetricPattern1<StoredF64>} sopr1y
 * @property {MetricPattern1<StoredF64>} sopr24h
 * @property {MetricPattern1<StoredF64>} sopr24h30dEma
 * @property {MetricPattern1<StoredF64>} sopr24h7dEma
 * @property {MetricPattern1<StoredF64>} sopr30d
 * @property {MetricPattern1<StoredF64>} sopr30dEma
 * @property {MetricPattern1<StoredF64>} sopr7d
 * @property {MetricPattern1<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {SatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueCreated1y
 * @property {MetricPattern1<Dollars>} valueCreated24h
 * @property {MetricPattern1<Dollars>} valueCreated30d
 * @property {MetricPattern1<Dollars>} valueCreated7d
 * @property {MetricPattern1<Dollars>} valueDestroyed
 * @property {MetricPattern1<Dollars>} valueDestroyed1y
 * @property {MetricPattern1<Dollars>} valueDestroyed24h
 * @property {MetricPattern1<Dollars>} valueDestroyed30d
 * @property {MetricPattern1<Dollars>} valueDestroyed7d
 */

/**
 * Create a AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern}
 */
function createAdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc) {
  return {
    adjustedSopr: createMetricPattern1(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr1y: createMetricPattern1(client, _m(acc, 'adjusted_sopr_1y')),
    adjustedSopr24h: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h')),
    adjustedSopr24h30dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h_30d_ema')),
    adjustedSopr24h7dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h_7d_ema')),
    adjustedSopr30d: createMetricPattern1(client, _m(acc, 'adjusted_sopr_30d')),
    adjustedSopr30dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7d: createMetricPattern1(client, _m(acc, 'adjusted_sopr_7d')),
    adjustedSopr7dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueCreated1y: createMetricPattern1(client, _m(acc, 'adjusted_value_created_1y')),
    adjustedValueCreated24h: createMetricPattern1(client, _m(acc, 'adjusted_value_created_24h')),
    adjustedValueCreated30d: createMetricPattern1(client, _m(acc, 'adjusted_value_created_30d')),
    adjustedValueCreated7d: createMetricPattern1(client, _m(acc, 'adjusted_value_created_7d')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    adjustedValueDestroyed1y: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_1y')),
    adjustedValueDestroyed24h: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_24h')),
    adjustedValueDestroyed30d: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_30d')),
    adjustedValueDestroyed7d: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_7d')),
    capRaw: createMetricPattern20(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern20(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    investorPriceRatioExt: createRatioPattern3(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern1(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLoss1y: createMetricPattern1(client, _m(acc, 'realized_loss_1y')),
    realizedLoss24h: createMetricPattern1(client, _m(acc, 'realized_loss_24h')),
    realizedLoss30d: createMetricPattern1(client, _m(acc, 'realized_loss_30d')),
    realizedLoss7d: createMetricPattern1(client, _m(acc, 'realized_loss_7d')),
    realizedLoss7dEma: createMetricPattern1(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedPriceRatioExt: createRatioPattern3(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfit1y: createMetricPattern1(client, _m(acc, 'realized_profit_1y')),
    realizedProfit24h: createMetricPattern1(client, _m(acc, 'realized_profit_24h')),
    realizedProfit30d: createMetricPattern1(client, _m(acc, 'realized_profit_30d')),
    realizedProfit7d: createMetricPattern1(client, _m(acc, 'realized_profit_7d')),
    realizedProfit7dEma: createMetricPattern1(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio1y: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_1y')),
    realizedProfitToLossRatio24h: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_24h')),
    realizedProfitToLossRatio30d: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_30d')),
    realizedProfitToLossRatio7d: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_7d')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    realizedValue1y: createMetricPattern1(client, _m(acc, 'realized_value_1y')),
    realizedValue24h: createMetricPattern1(client, _m(acc, 'realized_value_24h')),
    realizedValue30d: createMetricPattern1(client, _m(acc, 'realized_value_30d')),
    realizedValue7d: createMetricPattern1(client, _m(acc, 'realized_value_7d')),
    sellSideRiskRatio: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio1y: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y')),
    sellSideRiskRatio24h: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h')),
    sellSideRiskRatio24h30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema')),
    sellSideRiskRatio24h7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema')),
    sellSideRiskRatio30d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d')),
    sellSideRiskRatio30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d')),
    sellSideRiskRatio7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern1(client, _m(acc, 'sopr')),
    sopr1y: createMetricPattern1(client, _m(acc, 'sopr_1y')),
    sopr24h: createMetricPattern1(client, _m(acc, 'sopr_24h')),
    sopr24h30dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_30d_ema')),
    sopr24h7dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_7d_ema')),
    sopr30d: createMetricPattern1(client, _m(acc, 'sopr_30d')),
    sopr30dEma: createMetricPattern1(client, _m(acc, 'sopr_30d_ema')),
    sopr7d: createMetricPattern1(client, _m(acc, 'sopr_7d')),
    sopr7dEma: createMetricPattern1(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    upperPriceBand: createSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreated1y: createMetricPattern1(client, _m(acc, 'value_created_1y')),
    valueCreated24h: createMetricPattern1(client, _m(acc, 'value_created_24h')),
    valueCreated30d: createMetricPattern1(client, _m(acc, 'value_created_30d')),
    valueCreated7d: createMetricPattern1(client, _m(acc, 'value_created_7d')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyed1y: createMetricPattern1(client, _m(acc, 'value_destroyed_1y')),
    valueDestroyed24h: createMetricPattern1(client, _m(acc, 'value_destroyed_24h')),
    valueDestroyed30d: createMetricPattern1(client, _m(acc, 'value_destroyed_30d')),
    valueDestroyed7d: createMetricPattern1(client, _m(acc, 'value_destroyed_7d')),
  };
}

/**
 * @typedef {Object} AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2
 * @property {MetricPattern1<StoredF64>} adjustedSopr
 * @property {MetricPattern1<StoredF64>} adjustedSopr1y
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h30dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr24h7dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr30d
 * @property {MetricPattern1<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern1<StoredF64>} adjustedSopr7d
 * @property {MetricPattern1<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueCreated1y
 * @property {MetricPattern1<Dollars>} adjustedValueCreated24h
 * @property {MetricPattern1<Dollars>} adjustedValueCreated30d
 * @property {MetricPattern1<Dollars>} adjustedValueCreated7d
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed1y
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed24h
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed30d
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed7d
 * @property {MetricPattern20<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern20<CentsSquaredSats>} investorCapRaw
 * @property {SatsUsdPattern} investorPrice
 * @property {MetricPattern1<Cents>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {SatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {CumulativeHeightPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern1<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern1<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {CumulativeHeightPattern<Dollars>} realizedLoss
 * @property {MetricPattern1<Dollars>} realizedLoss7dEma
 * @property {MetricPattern1<StoredF32>} realizedLossRelToRealizedCap
 * @property {SatsUsdPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {CumulativeHeightPattern<Dollars>} realizedProfit
 * @property {MetricPattern1<Dollars>} realizedProfit7dEma
 * @property {MetricPattern1<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern1<Dollars>} realizedValue1y
 * @property {MetricPattern1<Dollars>} realizedValue24h
 * @property {MetricPattern1<Dollars>} realizedValue30d
 * @property {MetricPattern1<Dollars>} realizedValue7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio1y
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h7dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7dEma
 * @property {BtcSatsUsdPattern2} sentInLoss
 * @property {BtcSatsUsdPattern} sentInLoss14dEma
 * @property {BtcSatsUsdPattern2} sentInProfit
 * @property {BtcSatsUsdPattern} sentInProfit14dEma
 * @property {MetricPattern1<StoredF64>} sopr
 * @property {MetricPattern1<StoredF64>} sopr1y
 * @property {MetricPattern1<StoredF64>} sopr24h
 * @property {MetricPattern1<StoredF64>} sopr24h30dEma
 * @property {MetricPattern1<StoredF64>} sopr24h7dEma
 * @property {MetricPattern1<StoredF64>} sopr30d
 * @property {MetricPattern1<StoredF64>} sopr30dEma
 * @property {MetricPattern1<StoredF64>} sopr7d
 * @property {MetricPattern1<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {SatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueCreated1y
 * @property {MetricPattern1<Dollars>} valueCreated24h
 * @property {MetricPattern1<Dollars>} valueCreated30d
 * @property {MetricPattern1<Dollars>} valueCreated7d
 * @property {MetricPattern1<Dollars>} valueDestroyed
 * @property {MetricPattern1<Dollars>} valueDestroyed1y
 * @property {MetricPattern1<Dollars>} valueDestroyed24h
 * @property {MetricPattern1<Dollars>} valueDestroyed30d
 * @property {MetricPattern1<Dollars>} valueDestroyed7d
 */

/**
 * Create a AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2}
 */
function createAdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc) {
  return {
    adjustedSopr: createMetricPattern1(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr1y: createMetricPattern1(client, _m(acc, 'adjusted_sopr_1y')),
    adjustedSopr24h: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h')),
    adjustedSopr24h30dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h_30d_ema')),
    adjustedSopr24h7dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_24h_7d_ema')),
    adjustedSopr30d: createMetricPattern1(client, _m(acc, 'adjusted_sopr_30d')),
    adjustedSopr30dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7d: createMetricPattern1(client, _m(acc, 'adjusted_sopr_7d')),
    adjustedSopr7dEma: createMetricPattern1(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueCreated1y: createMetricPattern1(client, _m(acc, 'adjusted_value_created_1y')),
    adjustedValueCreated24h: createMetricPattern1(client, _m(acc, 'adjusted_value_created_24h')),
    adjustedValueCreated30d: createMetricPattern1(client, _m(acc, 'adjusted_value_created_30d')),
    adjustedValueCreated7d: createMetricPattern1(client, _m(acc, 'adjusted_value_created_7d')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    adjustedValueDestroyed1y: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_1y')),
    adjustedValueDestroyed24h: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_24h')),
    adjustedValueDestroyed30d: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_30d')),
    adjustedValueDestroyed7d: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed_7d')),
    capRaw: createMetricPattern20(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern20(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern1(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern1(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern1(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    realizedValue1y: createMetricPattern1(client, _m(acc, 'realized_value_1y')),
    realizedValue24h: createMetricPattern1(client, _m(acc, 'realized_value_24h')),
    realizedValue30d: createMetricPattern1(client, _m(acc, 'realized_value_30d')),
    realizedValue7d: createMetricPattern1(client, _m(acc, 'realized_value_7d')),
    sellSideRiskRatio: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio1y: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y')),
    sellSideRiskRatio24h: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h')),
    sellSideRiskRatio24h30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema')),
    sellSideRiskRatio24h7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema')),
    sellSideRiskRatio30d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d')),
    sellSideRiskRatio30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d')),
    sellSideRiskRatio7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern1(client, _m(acc, 'sopr')),
    sopr1y: createMetricPattern1(client, _m(acc, 'sopr_1y')),
    sopr24h: createMetricPattern1(client, _m(acc, 'sopr_24h')),
    sopr24h30dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_30d_ema')),
    sopr24h7dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_7d_ema')),
    sopr30d: createMetricPattern1(client, _m(acc, 'sopr_30d')),
    sopr30dEma: createMetricPattern1(client, _m(acc, 'sopr_30d_ema')),
    sopr7d: createMetricPattern1(client, _m(acc, 'sopr_7d')),
    sopr7dEma: createMetricPattern1(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    upperPriceBand: createSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreated1y: createMetricPattern1(client, _m(acc, 'value_created_1y')),
    valueCreated24h: createMetricPattern1(client, _m(acc, 'value_created_24h')),
    valueCreated30d: createMetricPattern1(client, _m(acc, 'value_created_30d')),
    valueCreated7d: createMetricPattern1(client, _m(acc, 'value_created_7d')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyed1y: createMetricPattern1(client, _m(acc, 'value_destroyed_1y')),
    valueDestroyed24h: createMetricPattern1(client, _m(acc, 'value_destroyed_24h')),
    valueDestroyed30d: createMetricPattern1(client, _m(acc, 'value_destroyed_30d')),
    valueDestroyed7d: createMetricPattern1(client, _m(acc, 'value_destroyed_7d')),
  };
}

/**
 * @typedef {Object} CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2
 * @property {MetricPattern20<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern20<CentsSquaredSats>} investorCapRaw
 * @property {SatsUsdPattern} investorPrice
 * @property {MetricPattern1<Cents>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {RatioPattern3} investorPriceRatioExt
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {SatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {CumulativeHeightPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern1<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern1<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {CumulativeHeightPattern<Dollars>} realizedLoss
 * @property {MetricPattern1<Dollars>} realizedLoss1y
 * @property {MetricPattern1<Dollars>} realizedLoss24h
 * @property {MetricPattern1<Dollars>} realizedLoss30d
 * @property {MetricPattern1<Dollars>} realizedLoss7d
 * @property {MetricPattern1<Dollars>} realizedLoss7dEma
 * @property {MetricPattern1<StoredF32>} realizedLossRelToRealizedCap
 * @property {SatsUsdPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {RatioPattern3} realizedPriceRatioExt
 * @property {CumulativeHeightPattern<Dollars>} realizedProfit
 * @property {MetricPattern1<Dollars>} realizedProfit1y
 * @property {MetricPattern1<Dollars>} realizedProfit24h
 * @property {MetricPattern1<Dollars>} realizedProfit30d
 * @property {MetricPattern1<Dollars>} realizedProfit7d
 * @property {MetricPattern1<Dollars>} realizedProfit7dEma
 * @property {MetricPattern1<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio1y
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio24h
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio30d
 * @property {MetricPattern1<StoredF64>} realizedProfitToLossRatio7d
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern1<Dollars>} realizedValue1y
 * @property {MetricPattern1<Dollars>} realizedValue24h
 * @property {MetricPattern1<Dollars>} realizedValue30d
 * @property {MetricPattern1<Dollars>} realizedValue7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio1y
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h7dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7dEma
 * @property {BtcSatsUsdPattern2} sentInLoss
 * @property {BtcSatsUsdPattern} sentInLoss14dEma
 * @property {BtcSatsUsdPattern2} sentInProfit
 * @property {BtcSatsUsdPattern} sentInProfit14dEma
 * @property {MetricPattern1<StoredF64>} sopr
 * @property {MetricPattern1<StoredF64>} sopr1y
 * @property {MetricPattern1<StoredF64>} sopr24h
 * @property {MetricPattern1<StoredF64>} sopr24h30dEma
 * @property {MetricPattern1<StoredF64>} sopr24h7dEma
 * @property {MetricPattern1<StoredF64>} sopr30d
 * @property {MetricPattern1<StoredF64>} sopr30dEma
 * @property {MetricPattern1<StoredF64>} sopr7d
 * @property {MetricPattern1<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {SatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueCreated1y
 * @property {MetricPattern1<Dollars>} valueCreated24h
 * @property {MetricPattern1<Dollars>} valueCreated30d
 * @property {MetricPattern1<Dollars>} valueCreated7d
 * @property {MetricPattern1<Dollars>} valueDestroyed
 * @property {MetricPattern1<Dollars>} valueDestroyed1y
 * @property {MetricPattern1<Dollars>} valueDestroyed24h
 * @property {MetricPattern1<Dollars>} valueDestroyed30d
 * @property {MetricPattern1<Dollars>} valueDestroyed7d
 */

/**
 * Create a CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2}
 */
function createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc) {
  return {
    capRaw: createMetricPattern20(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern20(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    investorPriceRatioExt: createRatioPattern3(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern1(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLoss1y: createMetricPattern1(client, _m(acc, 'realized_loss_1y')),
    realizedLoss24h: createMetricPattern1(client, _m(acc, 'realized_loss_24h')),
    realizedLoss30d: createMetricPattern1(client, _m(acc, 'realized_loss_30d')),
    realizedLoss7d: createMetricPattern1(client, _m(acc, 'realized_loss_7d')),
    realizedLoss7dEma: createMetricPattern1(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedPriceRatioExt: createRatioPattern3(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfit1y: createMetricPattern1(client, _m(acc, 'realized_profit_1y')),
    realizedProfit24h: createMetricPattern1(client, _m(acc, 'realized_profit_24h')),
    realizedProfit30d: createMetricPattern1(client, _m(acc, 'realized_profit_30d')),
    realizedProfit7d: createMetricPattern1(client, _m(acc, 'realized_profit_7d')),
    realizedProfit7dEma: createMetricPattern1(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio1y: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_1y')),
    realizedProfitToLossRatio24h: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_24h')),
    realizedProfitToLossRatio30d: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_30d')),
    realizedProfitToLossRatio7d: createMetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_7d')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    realizedValue1y: createMetricPattern1(client, _m(acc, 'realized_value_1y')),
    realizedValue24h: createMetricPattern1(client, _m(acc, 'realized_value_24h')),
    realizedValue30d: createMetricPattern1(client, _m(acc, 'realized_value_30d')),
    realizedValue7d: createMetricPattern1(client, _m(acc, 'realized_value_7d')),
    sellSideRiskRatio: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio1y: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y')),
    sellSideRiskRatio24h: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h')),
    sellSideRiskRatio24h30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema')),
    sellSideRiskRatio24h7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema')),
    sellSideRiskRatio30d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d')),
    sellSideRiskRatio30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d')),
    sellSideRiskRatio7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern1(client, _m(acc, 'sopr')),
    sopr1y: createMetricPattern1(client, _m(acc, 'sopr_1y')),
    sopr24h: createMetricPattern1(client, _m(acc, 'sopr_24h')),
    sopr24h30dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_30d_ema')),
    sopr24h7dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_7d_ema')),
    sopr30d: createMetricPattern1(client, _m(acc, 'sopr_30d')),
    sopr30dEma: createMetricPattern1(client, _m(acc, 'sopr_30d_ema')),
    sopr7d: createMetricPattern1(client, _m(acc, 'sopr_7d')),
    sopr7dEma: createMetricPattern1(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    upperPriceBand: createSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreated1y: createMetricPattern1(client, _m(acc, 'value_created_1y')),
    valueCreated24h: createMetricPattern1(client, _m(acc, 'value_created_24h')),
    valueCreated30d: createMetricPattern1(client, _m(acc, 'value_created_30d')),
    valueCreated7d: createMetricPattern1(client, _m(acc, 'value_created_7d')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyed1y: createMetricPattern1(client, _m(acc, 'value_destroyed_1y')),
    valueDestroyed24h: createMetricPattern1(client, _m(acc, 'value_destroyed_24h')),
    valueDestroyed30d: createMetricPattern1(client, _m(acc, 'value_destroyed_30d')),
    valueDestroyed7d: createMetricPattern1(client, _m(acc, 'value_destroyed_7d')),
  };
}

/**
 * @typedef {Object} CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern
 * @property {MetricPattern20<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern20<CentsSquaredSats>} investorCapRaw
 * @property {SatsUsdPattern} investorPrice
 * @property {MetricPattern1<Cents>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {SatsUsdPattern} lowerPriceBand
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {MetricPattern1<Dollars>} negRealizedLoss
 * @property {CumulativeHeightPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern1<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern1<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {MetricPattern1<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeHeightPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<Cents>} realizedCapCents
 * @property {CumulativeHeightPattern<Dollars>} realizedLoss
 * @property {MetricPattern1<Dollars>} realizedLoss7dEma
 * @property {MetricPattern1<StoredF32>} realizedLossRelToRealizedCap
 * @property {SatsUsdPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {CumulativeHeightPattern<Dollars>} realizedProfit
 * @property {MetricPattern1<Dollars>} realizedProfit7dEma
 * @property {MetricPattern1<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern1<Dollars>} realizedValue1y
 * @property {MetricPattern1<Dollars>} realizedValue24h
 * @property {MetricPattern1<Dollars>} realizedValue30d
 * @property {MetricPattern1<Dollars>} realizedValue7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio1y
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio24h7dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7d
 * @property {MetricPattern1<StoredF32>} sellSideRiskRatio7dEma
 * @property {BtcSatsUsdPattern2} sentInLoss
 * @property {BtcSatsUsdPattern} sentInLoss14dEma
 * @property {BtcSatsUsdPattern2} sentInProfit
 * @property {BtcSatsUsdPattern} sentInProfit14dEma
 * @property {MetricPattern1<StoredF64>} sopr
 * @property {MetricPattern1<StoredF64>} sopr1y
 * @property {MetricPattern1<StoredF64>} sopr24h
 * @property {MetricPattern1<StoredF64>} sopr24h30dEma
 * @property {MetricPattern1<StoredF64>} sopr24h7dEma
 * @property {MetricPattern1<StoredF64>} sopr30d
 * @property {MetricPattern1<StoredF64>} sopr30dEma
 * @property {MetricPattern1<StoredF64>} sopr7d
 * @property {MetricPattern1<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {SatsUsdPattern} upperPriceBand
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueCreated1y
 * @property {MetricPattern1<Dollars>} valueCreated24h
 * @property {MetricPattern1<Dollars>} valueCreated30d
 * @property {MetricPattern1<Dollars>} valueCreated7d
 * @property {MetricPattern1<Dollars>} valueDestroyed
 * @property {MetricPattern1<Dollars>} valueDestroyed1y
 * @property {MetricPattern1<Dollars>} valueDestroyed24h
 * @property {MetricPattern1<Dollars>} valueDestroyed30d
 * @property {MetricPattern1<Dollars>} valueDestroyed7d
 */

/**
 * Create a CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern}
 */
function createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc) {
  return {
    capRaw: createMetricPattern20(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern20(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createSatsUsdPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    lowerPriceBand: createSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    negRealizedLoss: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeHeightPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createMetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeHeightPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern1(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedLoss: createCumulativeHeightPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern1(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createSatsUsdPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeHeightPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern1(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createMetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    realizedValue1y: createMetricPattern1(client, _m(acc, 'realized_value_1y')),
    realizedValue24h: createMetricPattern1(client, _m(acc, 'realized_value_24h')),
    realizedValue30d: createMetricPattern1(client, _m(acc, 'realized_value_30d')),
    realizedValue7d: createMetricPattern1(client, _m(acc, 'realized_value_7d')),
    sellSideRiskRatio: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio1y: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y')),
    sellSideRiskRatio24h: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h')),
    sellSideRiskRatio24h30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema')),
    sellSideRiskRatio24h7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema')),
    sellSideRiskRatio30d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d')),
    sellSideRiskRatio30dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7d: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d')),
    sellSideRiskRatio7dEma: createMetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern1(client, _m(acc, 'sopr')),
    sopr1y: createMetricPattern1(client, _m(acc, 'sopr_1y')),
    sopr24h: createMetricPattern1(client, _m(acc, 'sopr_24h')),
    sopr24h30dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_30d_ema')),
    sopr24h7dEma: createMetricPattern1(client, _m(acc, 'sopr_24h_7d_ema')),
    sopr30d: createMetricPattern1(client, _m(acc, 'sopr_30d')),
    sopr30dEma: createMetricPattern1(client, _m(acc, 'sopr_30d_ema')),
    sopr7d: createMetricPattern1(client, _m(acc, 'sopr_7d')),
    sopr7dEma: createMetricPattern1(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    upperPriceBand: createSatsUsdPattern(client, _m(acc, 'upper_price_band')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueCreated1y: createMetricPattern1(client, _m(acc, 'value_created_1y')),
    valueCreated24h: createMetricPattern1(client, _m(acc, 'value_created_24h')),
    valueCreated30d: createMetricPattern1(client, _m(acc, 'value_created_30d')),
    valueCreated7d: createMetricPattern1(client, _m(acc, 'value_created_7d')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
    valueDestroyed1y: createMetricPattern1(client, _m(acc, 'value_destroyed_1y')),
    valueDestroyed24h: createMetricPattern1(client, _m(acc, 'value_destroyed_24h')),
    valueDestroyed30d: createMetricPattern1(client, _m(acc, 'value_destroyed_30d')),
    valueDestroyed7d: createMetricPattern1(client, _m(acc, 'value_destroyed_7d')),
  };
}

/**
 * @typedef {Object} _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern
 * @property {SatsUsdPattern} _0sdUsd
 * @property {MetricPattern1<StoredF32>} m05sd
 * @property {SatsUsdPattern} m05sdUsd
 * @property {MetricPattern1<StoredF32>} m15sd
 * @property {SatsUsdPattern} m15sdUsd
 * @property {MetricPattern1<StoredF32>} m1sd
 * @property {SatsUsdPattern} m1sdUsd
 * @property {MetricPattern1<StoredF32>} m25sd
 * @property {SatsUsdPattern} m25sdUsd
 * @property {MetricPattern1<StoredF32>} m2sd
 * @property {SatsUsdPattern} m2sdUsd
 * @property {MetricPattern1<StoredF32>} m3sd
 * @property {SatsUsdPattern} m3sdUsd
 * @property {MetricPattern1<StoredF32>} p05sd
 * @property {SatsUsdPattern} p05sdUsd
 * @property {MetricPattern1<StoredF32>} p15sd
 * @property {SatsUsdPattern} p15sdUsd
 * @property {MetricPattern1<StoredF32>} p1sd
 * @property {SatsUsdPattern} p1sdUsd
 * @property {MetricPattern1<StoredF32>} p25sd
 * @property {SatsUsdPattern} p25sdUsd
 * @property {MetricPattern1<StoredF32>} p2sd
 * @property {SatsUsdPattern} p2sdUsd
 * @property {MetricPattern1<StoredF32>} p3sd
 * @property {SatsUsdPattern} p3sdUsd
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
    _0sdUsd: createSatsUsdPattern(client, _m(acc, '0sd_usd')),
    m05sd: createMetricPattern1(client, _m(acc, 'm0_5sd')),
    m05sdUsd: createSatsUsdPattern(client, _m(acc, 'm0_5sd_usd')),
    m15sd: createMetricPattern1(client, _m(acc, 'm1_5sd')),
    m15sdUsd: createSatsUsdPattern(client, _m(acc, 'm1_5sd_usd')),
    m1sd: createMetricPattern1(client, _m(acc, 'm1sd')),
    m1sdUsd: createSatsUsdPattern(client, _m(acc, 'm1sd_usd')),
    m25sd: createMetricPattern1(client, _m(acc, 'm2_5sd')),
    m25sdUsd: createSatsUsdPattern(client, _m(acc, 'm2_5sd_usd')),
    m2sd: createMetricPattern1(client, _m(acc, 'm2sd')),
    m2sdUsd: createSatsUsdPattern(client, _m(acc, 'm2sd_usd')),
    m3sd: createMetricPattern1(client, _m(acc, 'm3sd')),
    m3sdUsd: createSatsUsdPattern(client, _m(acc, 'm3sd_usd')),
    p05sd: createMetricPattern1(client, _m(acc, 'p0_5sd')),
    p05sdUsd: createSatsUsdPattern(client, _m(acc, 'p0_5sd_usd')),
    p15sd: createMetricPattern1(client, _m(acc, 'p1_5sd')),
    p15sdUsd: createSatsUsdPattern(client, _m(acc, 'p1_5sd_usd')),
    p1sd: createMetricPattern1(client, _m(acc, 'p1sd')),
    p1sdUsd: createSatsUsdPattern(client, _m(acc, 'p1sd_usd')),
    p25sd: createMetricPattern1(client, _m(acc, 'p2_5sd')),
    p25sdUsd: createSatsUsdPattern(client, _m(acc, 'p2_5sd_usd')),
    p2sd: createMetricPattern1(client, _m(acc, 'p2sd')),
    p2sdUsd: createSatsUsdPattern(client, _m(acc, 'p2sd_usd')),
    p3sd: createMetricPattern1(client, _m(acc, 'p3sd')),
    p3sdUsd: createSatsUsdPattern(client, _m(acc, 'p3sd_usd')),
    sd: createMetricPattern1(client, _m(acc, 'sd')),
    sma: createMetricPattern1(client, _m(acc, 'sma')),
    zscore: createMetricPattern1(client, _m(acc, 'zscore')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern2
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedPeakRegretRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern2}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern2(client, acc) {
  return {
    investedCapitalInLossPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct')),
    investedCapitalInProfitPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct')),
    negUnrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    negUnrealizedLossRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap')),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    netUnrealizedPnlRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap')),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedLossRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl')),
    unrealizedPeakRegretRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @typedef {Object} PriceRatioPattern
 * @property {SatsUsdPattern} price
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<StoredF32>} ratio1mSma
 * @property {MetricPattern1<StoredF32>} ratio1wSma
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio1ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio2ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio4ySd
 * @property {MetricPattern1<StoredF32>} ratioPct1
 * @property {SatsUsdPattern} ratioPct1Usd
 * @property {MetricPattern1<StoredF32>} ratioPct2
 * @property {SatsUsdPattern} ratioPct2Usd
 * @property {MetricPattern1<StoredF32>} ratioPct5
 * @property {SatsUsdPattern} ratioPct5Usd
 * @property {MetricPattern1<StoredF32>} ratioPct95
 * @property {SatsUsdPattern} ratioPct95Usd
 * @property {MetricPattern1<StoredF32>} ratioPct98
 * @property {SatsUsdPattern} ratioPct98Usd
 * @property {MetricPattern1<StoredF32>} ratioPct99
 * @property {SatsUsdPattern} ratioPct99Usd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd
 */

/**
 * Create a PriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PriceRatioPattern}
 */
function createPriceRatioPattern(client, acc) {
  return {
    price: createSatsUsdPattern(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
    ratio1mSma: createMetricPattern1(client, _m(acc, 'ratio_1m_sma')),
    ratio1wSma: createMetricPattern1(client, _m(acc, 'ratio_1w_sma')),
    ratio1ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_1y')),
    ratio2ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_2y')),
    ratio4ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_4y')),
    ratioPct1: createMetricPattern1(client, _m(acc, 'ratio_pct1')),
    ratioPct1Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct1_usd')),
    ratioPct2: createMetricPattern1(client, _m(acc, 'ratio_pct2')),
    ratioPct2Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct2_usd')),
    ratioPct5: createMetricPattern1(client, _m(acc, 'ratio_pct5')),
    ratioPct5Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct5_usd')),
    ratioPct95: createMetricPattern1(client, _m(acc, 'ratio_pct95')),
    ratioPct95Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct95_usd')),
    ratioPct98: createMetricPattern1(client, _m(acc, 'ratio_pct98')),
    ratioPct98Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct98_usd')),
    ratioPct99: createMetricPattern1(client, _m(acc, 'ratio_pct99')),
    ratioPct99Usd: createSatsUsdPattern(client, _m(acc, 'ratio_pct99_usd')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern
 * @property {SatsUsdPattern} pct05
 * @property {SatsUsdPattern} pct10
 * @property {SatsUsdPattern} pct15
 * @property {SatsUsdPattern} pct20
 * @property {SatsUsdPattern} pct25
 * @property {SatsUsdPattern} pct30
 * @property {SatsUsdPattern} pct35
 * @property {SatsUsdPattern} pct40
 * @property {SatsUsdPattern} pct45
 * @property {SatsUsdPattern} pct50
 * @property {SatsUsdPattern} pct55
 * @property {SatsUsdPattern} pct60
 * @property {SatsUsdPattern} pct65
 * @property {SatsUsdPattern} pct70
 * @property {SatsUsdPattern} pct75
 * @property {SatsUsdPattern} pct80
 * @property {SatsUsdPattern} pct85
 * @property {SatsUsdPattern} pct90
 * @property {SatsUsdPattern} pct95
 */

/**
 * Create a Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern}
 */
function createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, acc) {
  return {
    pct05: createSatsUsdPattern(client, _m(acc, 'pct05')),
    pct10: createSatsUsdPattern(client, _m(acc, 'pct10')),
    pct15: createSatsUsdPattern(client, _m(acc, 'pct15')),
    pct20: createSatsUsdPattern(client, _m(acc, 'pct20')),
    pct25: createSatsUsdPattern(client, _m(acc, 'pct25')),
    pct30: createSatsUsdPattern(client, _m(acc, 'pct30')),
    pct35: createSatsUsdPattern(client, _m(acc, 'pct35')),
    pct40: createSatsUsdPattern(client, _m(acc, 'pct40')),
    pct45: createSatsUsdPattern(client, _m(acc, 'pct45')),
    pct50: createSatsUsdPattern(client, _m(acc, 'pct50')),
    pct55: createSatsUsdPattern(client, _m(acc, 'pct55')),
    pct60: createSatsUsdPattern(client, _m(acc, 'pct60')),
    pct65: createSatsUsdPattern(client, _m(acc, 'pct65')),
    pct70: createSatsUsdPattern(client, _m(acc, 'pct70')),
    pct75: createSatsUsdPattern(client, _m(acc, 'pct75')),
    pct80: createSatsUsdPattern(client, _m(acc, 'pct80')),
    pct85: createSatsUsdPattern(client, _m(acc, 'pct85')),
    pct90: createSatsUsdPattern(client, _m(acc, 'pct90')),
    pct95: createSatsUsdPattern(client, _m(acc, 'pct95')),
  };
}

/**
 * @typedef {Object} RatioPattern
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<StoredF32>} ratio1mSma
 * @property {MetricPattern1<StoredF32>} ratio1wSma
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio1ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio2ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio4ySd
 * @property {MetricPattern1<StoredF32>} ratioPct1
 * @property {SatsUsdPattern} ratioPct1Usd
 * @property {MetricPattern1<StoredF32>} ratioPct2
 * @property {SatsUsdPattern} ratioPct2Usd
 * @property {MetricPattern1<StoredF32>} ratioPct5
 * @property {SatsUsdPattern} ratioPct5Usd
 * @property {MetricPattern1<StoredF32>} ratioPct95
 * @property {SatsUsdPattern} ratioPct95Usd
 * @property {MetricPattern1<StoredF32>} ratioPct98
 * @property {SatsUsdPattern} ratioPct98Usd
 * @property {MetricPattern1<StoredF32>} ratioPct99
 * @property {SatsUsdPattern} ratioPct99Usd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd
 */

/**
 * Create a RatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioPattern}
 */
function createRatioPattern(client, acc) {
  return {
    ratio: createMetricPattern1(client, acc),
    ratio1mSma: createMetricPattern1(client, _m(acc, '1m_sma')),
    ratio1wSma: createMetricPattern1(client, _m(acc, '1w_sma')),
    ratio1ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y')),
    ratio2ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y')),
    ratio4ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y')),
    ratioPct1: createMetricPattern1(client, _m(acc, 'pct1')),
    ratioPct1Usd: createSatsUsdPattern(client, _m(acc, 'pct1_usd')),
    ratioPct2: createMetricPattern1(client, _m(acc, 'pct2')),
    ratioPct2Usd: createSatsUsdPattern(client, _m(acc, 'pct2_usd')),
    ratioPct5: createMetricPattern1(client, _m(acc, 'pct5')),
    ratioPct5Usd: createSatsUsdPattern(client, _m(acc, 'pct5_usd')),
    ratioPct95: createMetricPattern1(client, _m(acc, 'pct95')),
    ratioPct95Usd: createSatsUsdPattern(client, _m(acc, 'pct95_usd')),
    ratioPct98: createMetricPattern1(client, _m(acc, 'pct98')),
    ratioPct98Usd: createSatsUsdPattern(client, _m(acc, 'pct98_usd')),
    ratioPct99: createMetricPattern1(client, _m(acc, 'pct99')),
    ratioPct99Usd: createSatsUsdPattern(client, _m(acc, 'pct99_usd')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
  };
}

/**
 * @typedef {Object} RatioPattern3
 * @property {MetricPattern1<StoredF32>} ratio1mSma
 * @property {MetricPattern1<StoredF32>} ratio1wSma
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio1ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio2ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio4ySd
 * @property {MetricPattern1<StoredF32>} ratioPct1
 * @property {SatsUsdPattern} ratioPct1Usd
 * @property {MetricPattern1<StoredF32>} ratioPct2
 * @property {SatsUsdPattern} ratioPct2Usd
 * @property {MetricPattern1<StoredF32>} ratioPct5
 * @property {SatsUsdPattern} ratioPct5Usd
 * @property {MetricPattern1<StoredF32>} ratioPct95
 * @property {SatsUsdPattern} ratioPct95Usd
 * @property {MetricPattern1<StoredF32>} ratioPct98
 * @property {SatsUsdPattern} ratioPct98Usd
 * @property {MetricPattern1<StoredF32>} ratioPct99
 * @property {SatsUsdPattern} ratioPct99Usd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratioSd
 */

/**
 * Create a RatioPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioPattern3}
 */
function createRatioPattern3(client, acc) {
  return {
    ratio1mSma: createMetricPattern1(client, _m(acc, '1m_sma')),
    ratio1wSma: createMetricPattern1(client, _m(acc, '1w_sma')),
    ratio1ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y')),
    ratio2ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y')),
    ratio4ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y')),
    ratioPct1: createMetricPattern1(client, _m(acc, 'pct1')),
    ratioPct1Usd: createSatsUsdPattern(client, _m(acc, 'pct1_usd')),
    ratioPct2: createMetricPattern1(client, _m(acc, 'pct2')),
    ratioPct2Usd: createSatsUsdPattern(client, _m(acc, 'pct2_usd')),
    ratioPct5: createMetricPattern1(client, _m(acc, 'pct5')),
    ratioPct5Usd: createSatsUsdPattern(client, _m(acc, 'pct5_usd')),
    ratioPct95: createMetricPattern1(client, _m(acc, 'pct95')),
    ratioPct95Usd: createSatsUsdPattern(client, _m(acc, 'pct95_usd')),
    ratioPct98: createMetricPattern1(client, _m(acc, 'pct98')),
    ratioPct98Usd: createSatsUsdPattern(client, _m(acc, 'pct98_usd')),
    ratioPct99: createMetricPattern1(client, _m(acc, 'pct99')),
    ratioPct99Usd: createSatsUsdPattern(client, _m(acc, 'pct99_usd')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
  };
}

/**
 * @typedef {Object} GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern
 * @property {MetricPattern1<Dollars>} greedIndex
 * @property {MetricPattern1<Dollars>} investedCapitalInLoss
 * @property {MetricPattern20<CentsSats>} investedCapitalInLossRaw
 * @property {MetricPattern1<Dollars>} investedCapitalInProfit
 * @property {MetricPattern20<CentsSats>} investedCapitalInProfitRaw
 * @property {MetricPattern20<CentsSquaredSats>} investorCapInLossRaw
 * @property {MetricPattern20<CentsSquaredSats>} investorCapInProfitRaw
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {MetricPattern1<Dollars>} netSentiment
 * @property {MetricPattern1<Dollars>} netUnrealizedPnl
 * @property {MetricPattern1<Dollars>} painIndex
 * @property {MetricPattern1<Dollars>} peakRegret
 * @property {BtcSatsUsdPattern} supplyInLoss
 * @property {BtcSatsUsdPattern} supplyInProfit
 * @property {MetricPattern1<Dollars>} totalUnrealizedPnl
 * @property {MetricPattern1<Dollars>} unrealizedLoss
 * @property {MetricPattern1<Dollars>} unrealizedProfit
 */

/**
 * Create a GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern}
 */
function createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc) {
  return {
    greedIndex: createMetricPattern1(client, _m(acc, 'greed_index')),
    investedCapitalInLoss: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss')),
    investedCapitalInLossRaw: createMetricPattern20(client, _m(acc, 'invested_capital_in_loss_raw')),
    investedCapitalInProfit: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit')),
    investedCapitalInProfitRaw: createMetricPattern20(client, _m(acc, 'invested_capital_in_profit_raw')),
    investorCapInLossRaw: createMetricPattern20(client, _m(acc, 'investor_cap_in_loss_raw')),
    investorCapInProfitRaw: createMetricPattern20(client, _m(acc, 'investor_cap_in_profit_raw')),
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netSentiment: createMetricPattern1(client, _m(acc, 'net_sentiment')),
    netUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl')),
    painIndex: createMetricPattern1(client, _m(acc, 'pain_index')),
    peakRegret: createMetricPattern1(client, _m(acc, 'unrealized_peak_regret')),
    supplyInLoss: createBtcSatsUsdPattern(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createBtcSatsUsdPattern(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern1(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern1(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @template T
 * @typedef {Object} Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern
 * @property {MetricPattern10<T>} day1
 * @property {MetricPattern11<T>} day3
 * @property {MetricPattern19<T>} difficultyepoch
 * @property {MetricPattern18<T>} halvingepoch
 * @property {MetricPattern7<T>} hour1
 * @property {MetricPattern9<T>} hour12
 * @property {MetricPattern8<T>} hour4
 * @property {MetricPattern3<T>} minute1
 * @property {MetricPattern5<T>} minute10
 * @property {MetricPattern6<T>} minute30
 * @property {MetricPattern4<T>} minute5
 * @property {MetricPattern13<T>} month1
 * @property {MetricPattern14<T>} month3
 * @property {MetricPattern15<T>} month6
 * @property {MetricPattern12<T>} week1
 * @property {MetricPattern16<T>} year1
 * @property {MetricPattern17<T>} year10
 */

/**
 * Create a Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern<T>}
 */
function createDay1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, acc) {
  return {
    day1: createMetricPattern10(client, _m(acc, 'day1')),
    day3: createMetricPattern11(client, _m(acc, 'day3')),
    difficultyepoch: createMetricPattern19(client, _m(acc, 'difficultyepoch')),
    halvingepoch: createMetricPattern18(client, _m(acc, 'halvingepoch')),
    hour1: createMetricPattern7(client, _m(acc, 'hour1')),
    hour12: createMetricPattern9(client, _m(acc, 'hour12')),
    hour4: createMetricPattern8(client, _m(acc, 'hour4')),
    minute1: createMetricPattern3(client, _m(acc, 'minute1')),
    minute10: createMetricPattern5(client, _m(acc, 'minute10')),
    minute30: createMetricPattern6(client, _m(acc, 'minute30')),
    minute5: createMetricPattern4(client, _m(acc, 'minute5')),
    month1: createMetricPattern13(client, _m(acc, 'month1')),
    month3: createMetricPattern14(client, _m(acc, 'month3')),
    month6: createMetricPattern15(client, _m(acc, 'month6')),
    week1: createMetricPattern12(client, _m(acc, 'week1')),
    year1: createMetricPattern16(client, _m(acc, 'year1')),
    year10: createMetricPattern17(client, _m(acc, 'year10')),
  };
}

/**
 * @typedef {Object} GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern
 * @property {MetricPattern1<Dollars>} greedIndex
 * @property {MetricPattern1<Dollars>} investedCapitalInLoss
 * @property {MetricPattern20<CentsSats>} investedCapitalInLossRaw
 * @property {MetricPattern1<Dollars>} investedCapitalInProfit
 * @property {MetricPattern20<CentsSats>} investedCapitalInProfitRaw
 * @property {MetricPattern20<CentsSquaredSats>} investorCapInLossRaw
 * @property {MetricPattern20<CentsSquaredSats>} investorCapInProfitRaw
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {MetricPattern1<Dollars>} netSentiment
 * @property {MetricPattern1<Dollars>} netUnrealizedPnl
 * @property {MetricPattern1<Dollars>} painIndex
 * @property {BtcSatsUsdPattern} supplyInLoss
 * @property {BtcSatsUsdPattern} supplyInProfit
 * @property {MetricPattern1<Dollars>} totalUnrealizedPnl
 * @property {MetricPattern1<Dollars>} unrealizedLoss
 * @property {MetricPattern1<Dollars>} unrealizedProfit
 */

/**
 * Create a GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern}
 */
function createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc) {
  return {
    greedIndex: createMetricPattern1(client, _m(acc, 'greed_index')),
    investedCapitalInLoss: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss')),
    investedCapitalInLossRaw: createMetricPattern20(client, _m(acc, 'invested_capital_in_loss_raw')),
    investedCapitalInProfit: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit')),
    investedCapitalInProfitRaw: createMetricPattern20(client, _m(acc, 'invested_capital_in_profit_raw')),
    investorCapInLossRaw: createMetricPattern20(client, _m(acc, 'investor_cap_in_loss_raw')),
    investorCapInProfitRaw: createMetricPattern20(client, _m(acc, 'investor_cap_in_profit_raw')),
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netSentiment: createMetricPattern1(client, _m(acc, 'net_sentiment')),
    netUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl')),
    painIndex: createMetricPattern1(client, _m(acc, 'pain_index')),
    supplyInLoss: createBtcSatsUsdPattern(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createBtcSatsUsdPattern(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern1(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern1(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} BlocksCoinbaseDaysDominanceFeeSubsidyPattern
 * @property {CumulativeHeightSumPattern<StoredU32>} blocksMined
 * @property {MetricPattern1<StoredU32>} blocksMined1mSum
 * @property {MetricPattern1<StoredU32>} blocksMined1wSum
 * @property {MetricPattern1<StoredU32>} blocksMined1ySum
 * @property {MetricPattern1<StoredU32>} blocksMined24hSum
 * @property {MetricPattern1<StoredU32>} blocksSinceBlock
 * @property {BtcSatsUsdPattern4} coinbase
 * @property {MetricPattern1<StoredU16>} daysSinceBlock
 * @property {MetricPattern1<StoredF32>} dominance
 * @property {MetricPattern1<StoredF32>} dominance1m
 * @property {MetricPattern1<StoredF32>} dominance1w
 * @property {MetricPattern1<StoredF32>} dominance1y
 * @property {MetricPattern1<StoredF32>} dominance24h
 * @property {BtcSatsUsdPattern4} fee
 * @property {BtcSatsUsdPattern4} subsidy
 */

/**
 * Create a BlocksCoinbaseDaysDominanceFeeSubsidyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlocksCoinbaseDaysDominanceFeeSubsidyPattern}
 */
function createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, acc) {
  return {
    blocksMined: createCumulativeHeightSumPattern(client, _m(acc, 'blocks_mined')),
    blocksMined1mSum: createMetricPattern1(client, _m(acc, 'blocks_mined_1m_sum')),
    blocksMined1wSum: createMetricPattern1(client, _m(acc, 'blocks_mined_1w_sum')),
    blocksMined1ySum: createMetricPattern1(client, _m(acc, 'blocks_mined_1y_sum')),
    blocksMined24hSum: createMetricPattern1(client, _m(acc, 'blocks_mined_24h_sum')),
    blocksSinceBlock: createMetricPattern1(client, _m(acc, 'blocks_since_block')),
    coinbase: createBtcSatsUsdPattern4(client, _m(acc, 'coinbase')),
    daysSinceBlock: createMetricPattern1(client, _m(acc, 'days_since_block')),
    dominance: createMetricPattern1(client, _m(acc, 'dominance')),
    dominance1m: createMetricPattern1(client, _m(acc, 'dominance_1m')),
    dominance1w: createMetricPattern1(client, _m(acc, 'dominance_1w')),
    dominance1y: createMetricPattern1(client, _m(acc, 'dominance_1y')),
    dominance24h: createMetricPattern1(client, _m(acc, 'dominance_24h')),
    fee: createBtcSatsUsdPattern4(client, _m(acc, 'fee')),
    subsidy: createBtcSatsUsdPattern4(client, _m(acc, 'subsidy')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern4
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedPeakRegretRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern4}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern4(client, acc) {
  return {
    investedCapitalInLossPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct')),
    investedCapitalInProfitPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct')),
    negUnrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedPeakRegretRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern3
 * @property {BtcSatsUsdPattern} _10y
 * @property {BtcSatsUsdPattern} _1m
 * @property {BtcSatsUsdPattern} _1w
 * @property {BtcSatsUsdPattern} _1y
 * @property {BtcSatsUsdPattern} _2y
 * @property {BtcSatsUsdPattern} _3m
 * @property {BtcSatsUsdPattern} _3y
 * @property {BtcSatsUsdPattern} _4y
 * @property {BtcSatsUsdPattern} _5y
 * @property {BtcSatsUsdPattern} _6m
 * @property {BtcSatsUsdPattern} _6y
 * @property {BtcSatsUsdPattern} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, acc) {
  return {
    _10y: createBtcSatsUsdPattern(client, _p('10y', acc)),
    _1m: createBtcSatsUsdPattern(client, _p('1m', acc)),
    _1w: createBtcSatsUsdPattern(client, _p('1w', acc)),
    _1y: createBtcSatsUsdPattern(client, _p('1y', acc)),
    _2y: createBtcSatsUsdPattern(client, _p('2y', acc)),
    _3m: createBtcSatsUsdPattern(client, _p('3m', acc)),
    _3y: createBtcSatsUsdPattern(client, _p('3y', acc)),
    _4y: createBtcSatsUsdPattern(client, _p('4y', acc)),
    _5y: createBtcSatsUsdPattern(client, _p('5y', acc)),
    _6m: createBtcSatsUsdPattern(client, _p('6m', acc)),
    _6y: createBtcSatsUsdPattern(client, _p('6y', acc)),
    _8y: createBtcSatsUsdPattern(client, _p('8y', acc)),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc) {
  return {
    investedCapitalInLossPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct')),
    investedCapitalInProfitPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct')),
    negUnrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @template T
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern2
 * @property {MetricPattern1<T>} _10y
 * @property {MetricPattern1<T>} _1m
 * @property {MetricPattern1<T>} _1w
 * @property {MetricPattern1<T>} _1y
 * @property {MetricPattern1<T>} _2y
 * @property {MetricPattern1<T>} _3m
 * @property {MetricPattern1<T>} _3y
 * @property {MetricPattern1<T>} _4y
 * @property {MetricPattern1<T>} _5y
 * @property {MetricPattern1<T>} _6m
 * @property {MetricPattern1<T>} _6y
 * @property {MetricPattern1<T>} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T>}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, acc) {
  return {
    _10y: createMetricPattern1(client, _p('10y', acc)),
    _1m: createMetricPattern1(client, _p('1m', acc)),
    _1w: createMetricPattern1(client, _p('1w', acc)),
    _1y: createMetricPattern1(client, _p('1y', acc)),
    _2y: createMetricPattern1(client, _p('2y', acc)),
    _3m: createMetricPattern1(client, _p('3m', acc)),
    _3y: createMetricPattern1(client, _p('3y', acc)),
    _4y: createMetricPattern1(client, _p('4y', acc)),
    _5y: createMetricPattern1(client, _p('5y', acc)),
    _6m: createMetricPattern1(client, _p('6m', acc)),
    _6y: createMetricPattern1(client, _p('6y', acc)),
    _8y: createMetricPattern1(client, _p('8y', acc)),
  };
}

/**
 * @template T
 * @typedef {Object} _201520162017201820192020202120222023202420252026Pattern2
 * @property {MetricPattern1<T>} _2015
 * @property {MetricPattern1<T>} _2016
 * @property {MetricPattern1<T>} _2017
 * @property {MetricPattern1<T>} _2018
 * @property {MetricPattern1<T>} _2019
 * @property {MetricPattern1<T>} _2020
 * @property {MetricPattern1<T>} _2021
 * @property {MetricPattern1<T>} _2022
 * @property {MetricPattern1<T>} _2023
 * @property {MetricPattern1<T>} _2024
 * @property {MetricPattern1<T>} _2025
 * @property {MetricPattern1<T>} _2026
 */

/**
 * Create a _201520162017201820192020202120222023202420252026Pattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_201520162017201820192020202120222023202420252026Pattern2<T>}
 */
function create_201520162017201820192020202120222023202420252026Pattern2(client, acc) {
  return {
    _2015: createMetricPattern1(client, _m(acc, '2015_returns')),
    _2016: createMetricPattern1(client, _m(acc, '2016_returns')),
    _2017: createMetricPattern1(client, _m(acc, '2017_returns')),
    _2018: createMetricPattern1(client, _m(acc, '2018_returns')),
    _2019: createMetricPattern1(client, _m(acc, '2019_returns')),
    _2020: createMetricPattern1(client, _m(acc, '2020_returns')),
    _2021: createMetricPattern1(client, _m(acc, '2021_returns')),
    _2022: createMetricPattern1(client, _m(acc, '2022_returns')),
    _2023: createMetricPattern1(client, _m(acc, '2023_returns')),
    _2024: createMetricPattern1(client, _m(acc, '2024_returns')),
    _2025: createMetricPattern1(client, _m(acc, '2025_returns')),
    _2026: createMetricPattern1(client, _m(acc, '2026_returns')),
  };
}

/**
 * @typedef {Object} AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern
 * @property {MetricPattern20<StoredU64>} average
 * @property {MetricPattern20<StoredU64>} cumulative
 * @property {MetricPattern20<StoredU64>} max
 * @property {MetricPattern20<StoredU64>} median
 * @property {MetricPattern20<StoredU64>} min
 * @property {MetricPattern20<StoredU64>} pct10
 * @property {MetricPattern20<StoredU64>} pct25
 * @property {MetricPattern20<StoredU64>} pct75
 * @property {MetricPattern20<StoredU64>} pct90
 * @property {AverageMaxMedianMinP10P25P75P90SumPattern} rolling
 * @property {MetricPattern20<StoredU64>} sum
 */

/**
 * Create a AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern}
 */
function createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, acc) {
  return {
    average: createMetricPattern20(client, _m(acc, 'average')),
    cumulative: createMetricPattern20(client, _m(acc, 'cumulative')),
    max: createMetricPattern20(client, _m(acc, 'max')),
    median: createMetricPattern20(client, _m(acc, 'median')),
    min: createMetricPattern20(client, _m(acc, 'min')),
    pct10: createMetricPattern20(client, _m(acc, 'pct10')),
    pct25: createMetricPattern20(client, _m(acc, 'pct25')),
    pct75: createMetricPattern20(client, _m(acc, 'pct75')),
    pct90: createMetricPattern20(client, _m(acc, 'pct90')),
    rolling: createAverageMaxMedianMinP10P25P75P90SumPattern(client, acc),
    sum: createMetricPattern20(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageCumulativeMaxMedianMinP10P25P75P90SumPattern
 * @property {_1y24h30d7dPattern<StoredU64>} average
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {_1y24h30d7dPattern<StoredU64>} max
 * @property {_1y24h30d7dPattern<StoredU64>} median
 * @property {_1y24h30d7dPattern<StoredU64>} min
 * @property {_1y24h30d7dPattern<StoredU64>} p10
 * @property {_1y24h30d7dPattern<StoredU64>} p25
 * @property {_1y24h30d7dPattern<StoredU64>} p75
 * @property {_1y24h30d7dPattern<StoredU64>} p90
 * @property {_1y24h30d7dPattern<StoredU64>} sum
 */

/**
 * Create a AverageCumulativeMaxMedianMinP10P25P75P90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageCumulativeMaxMedianMinP10P25P75P90SumPattern}
 */
function createAverageCumulativeMaxMedianMinP10P25P75P90SumPattern(client, acc) {
  return {
    average: create_1y24h30d7dPattern(client, _m(acc, 'average')),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    max: create_1y24h30d7dPattern(client, _m(acc, 'max')),
    median: create_1y24h30d7dPattern(client, _m(acc, 'median')),
    min: create_1y24h30d7dPattern(client, _m(acc, 'min')),
    p10: create_1y24h30d7dPattern(client, _m(acc, 'p10')),
    p25: create_1y24h30d7dPattern(client, _m(acc, 'p25')),
    p75: create_1y24h30d7dPattern(client, _m(acc, 'p75')),
    p90: create_1y24h30d7dPattern(client, _m(acc, 'p90')),
    sum: create_1y24h30d7dPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageGainsLossesRsiStochPattern
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} rsi
 * @property {MetricPattern1<StoredF32>} rsiMax
 * @property {MetricPattern1<StoredF32>} rsiMin
 * @property {MetricPattern1<StoredF32>} stochRsi
 * @property {MetricPattern1<StoredF32>} stochRsiD
 * @property {MetricPattern1<StoredF32>} stochRsiK
 */

/**
 * Create a AverageGainsLossesRsiStochPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageGainsLossesRsiStochPattern}
 */
function createAverageGainsLossesRsiStochPattern(client, acc) {
  return {
    averageGain: createMetricPattern1(client, _m(acc, 'avg_gain_1y')),
    averageLoss: createMetricPattern1(client, _m(acc, 'avg_loss_1y')),
    gains: createMetricPattern1(client, _m(acc, 'gains_1y')),
    losses: createMetricPattern1(client, _m(acc, 'losses_1y')),
    rsi: createMetricPattern1(client, _m(acc, '1y')),
    rsiMax: createMetricPattern1(client, _m(acc, 'rsi_max_1y')),
    rsiMin: createMetricPattern1(client, _m(acc, 'rsi_min_1y')),
    stochRsi: createMetricPattern1(client, _m(acc, 'stoch_rsi_1y')),
    stochRsiD: createMetricPattern1(client, _m(acc, 'stoch_rsi_d_1y')),
    stochRsiK: createMetricPattern1(client, _m(acc, 'stoch_rsi_k_1y')),
  };
}

/**
 * @typedef {Object} ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {MetricPattern1<StoredF64>} addrCount30dChange
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern}
 */
function createActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    addrCount: createMetricPattern1(client, _m(acc, 'addr_count')),
    addrCount30dChange: createMetricPattern1(client, _m(acc, 'addr_count_30d_change')),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern
 * @property {_30dCountPattern} all
 * @property {_30dCountPattern} p2a
 * @property {_30dCountPattern} p2pk33
 * @property {_30dCountPattern} p2pk65
 * @property {_30dCountPattern} p2pkh
 * @property {_30dCountPattern} p2sh
 * @property {_30dCountPattern} p2tr
 * @property {_30dCountPattern} p2wpkh
 * @property {_30dCountPattern} p2wsh
 */

/**
 * Create a AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern}
 */
function createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, acc) {
  return {
    all: create_30dCountPattern(client, acc),
    p2a: create_30dCountPattern(client, _p('p2a', acc)),
    p2pk33: create_30dCountPattern(client, _p('p2pk33', acc)),
    p2pk65: create_30dCountPattern(client, _p('p2pk65', acc)),
    p2pkh: create_30dCountPattern(client, _p('p2pkh', acc)),
    p2sh: create_30dCountPattern(client, _p('p2sh', acc)),
    p2tr: create_30dCountPattern(client, _p('p2tr', acc)),
    p2wpkh: create_30dCountPattern(client, _p('p2wpkh', acc)),
    p2wsh: create_30dCountPattern(client, _p('p2wsh', acc)),
  };
}

/**
 * @typedef {Object} AverageMaxMedianMinP10P25P75P90SumPattern
 * @property {_1y24h30d7dPattern<StoredU64>} average
 * @property {_1y24h30d7dPattern<StoredU64>} max
 * @property {_1y24h30d7dPattern<StoredU64>} median
 * @property {_1y24h30d7dPattern<StoredU64>} min
 * @property {_1y24h30d7dPattern<StoredU64>} p10
 * @property {_1y24h30d7dPattern<StoredU64>} p25
 * @property {_1y24h30d7dPattern<StoredU64>} p75
 * @property {_1y24h30d7dPattern<StoredU64>} p90
 * @property {_1y24h30d7dPattern<StoredU64>} sum
 */

/**
 * Create a AverageMaxMedianMinP10P25P75P90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinP10P25P75P90SumPattern}
 */
function createAverageMaxMedianMinP10P25P75P90SumPattern(client, acc) {
  return {
    average: create_1y24h30d7dPattern(client, _m(acc, 'average')),
    max: create_1y24h30d7dPattern(client, _m(acc, 'max')),
    median: create_1y24h30d7dPattern(client, _m(acc, 'median')),
    min: create_1y24h30d7dPattern(client, _m(acc, 'min')),
    p10: create_1y24h30d7dPattern(client, _m(acc, 'p10')),
    p25: create_1y24h30d7dPattern(client, _m(acc, 'p25')),
    p75: create_1y24h30d7dPattern(client, _m(acc, 'p75')),
    p90: create_1y24h30d7dPattern(client, _m(acc, 'p90')),
    sum: create_1y24h30d7dPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @template T
 * @typedef {Object} AverageHeightMaxMedianMinP10P25P75P90Pattern
 * @property {_1y24h30d7dPattern<T>} average
 * @property {MetricPattern20<T>} height
 * @property {_1y24h30d7dPattern<T>} max
 * @property {_1y24h30d7dPattern<T>} median
 * @property {_1y24h30d7dPattern<T>} min
 * @property {_1y24h30d7dPattern<T>} p10
 * @property {_1y24h30d7dPattern<T>} p25
 * @property {_1y24h30d7dPattern<T>} p75
 * @property {_1y24h30d7dPattern<T>} p90
 */

/**
 * Create a AverageHeightMaxMedianMinP10P25P75P90Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageHeightMaxMedianMinP10P25P75P90Pattern<T>}
 */
function createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, acc) {
  return {
    average: create_1y24h30d7dPattern(client, _m(acc, 'average')),
    height: createMetricPattern20(client, acc),
    max: create_1y24h30d7dPattern(client, _m(acc, 'max')),
    median: create_1y24h30d7dPattern(client, _m(acc, 'median')),
    min: create_1y24h30d7dPattern(client, _m(acc, 'min')),
    p10: create_1y24h30d7dPattern(client, _m(acc, 'p10')),
    p25: create_1y24h30d7dPattern(client, _m(acc, 'p25')),
    p75: create_1y24h30d7dPattern(client, _m(acc, 'p75')),
    p90: create_1y24h30d7dPattern(client, _m(acc, 'p90')),
  };
}

/**
 * @template T
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern
 * @property {MetricPattern20<T>} average
 * @property {MetricPattern20<T>} max
 * @property {MetricPattern20<T>} median
 * @property {MetricPattern20<T>} min
 * @property {MetricPattern20<T>} pct10
 * @property {MetricPattern20<T>} pct25
 * @property {MetricPattern20<T>} pct75
 * @property {MetricPattern20<T>} pct90
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
    average: createMetricPattern20(client, _m(acc, 'average')),
    max: createMetricPattern20(client, _m(acc, 'max')),
    median: createMetricPattern20(client, _m(acc, 'median')),
    min: createMetricPattern20(client, _m(acc, 'min')),
    pct10: createMetricPattern20(client, _m(acc, 'pct10')),
    pct25: createMetricPattern20(client, _m(acc, 'pct25')),
    pct75: createMetricPattern20(client, _m(acc, 'pct75')),
    pct90: createMetricPattern20(client, _m(acc, 'pct90')),
  };
}

/**
 * @typedef {Object} _10y2y3y4y5y6y8yPattern
 * @property {MetricPattern1<StoredF32>} _10y
 * @property {MetricPattern1<StoredF32>} _2y
 * @property {MetricPattern1<StoredF32>} _3y
 * @property {MetricPattern1<StoredF32>} _4y
 * @property {MetricPattern1<StoredF32>} _5y
 * @property {MetricPattern1<StoredF32>} _6y
 * @property {MetricPattern1<StoredF32>} _8y
 */

/**
 * Create a _10y2y3y4y5y6y8yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y2y3y4y5y6y8yPattern}
 */
function create_10y2y3y4y5y6y8yPattern(client, acc) {
  return {
    _10y: createMetricPattern1(client, _p('10y', acc)),
    _2y: createMetricPattern1(client, _p('2y', acc)),
    _3y: createMetricPattern1(client, _p('3y', acc)),
    _4y: createMetricPattern1(client, _p('4y', acc)),
    _5y: createMetricPattern1(client, _p('5y', acc)),
    _6y: createMetricPattern1(client, _p('6y', acc)),
    _8y: createMetricPattern1(client, _p('8y', acc)),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern2} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createInvestedMaxMinPercentilesSpotPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern2(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern4} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createAdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern4(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern4} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern4(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} BalanceBothReactivatedReceivingSendingPattern
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} balanceDecreased
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} balanceIncreased
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} both
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} reactivated
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} receiving
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredU32>} sending
 */

/**
 * Create a BalanceBothReactivatedReceivingSendingPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BalanceBothReactivatedReceivingSendingPattern}
 */
function createBalanceBothReactivatedReceivingSendingPattern(client, acc) {
  return {
    balanceDecreased: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'balance_decreased')),
    balanceIncreased: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'balance_increased')),
    both: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'both')),
    reactivated: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'reactivated')),
    receiving: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'receiving')),
    sending: createAverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'sending')),
  };
}

/**
 * @typedef {Object} CoinblocksCoindaysSatblocksSatdaysSentPattern
 * @property {CumulativeHeightSumPattern<StoredF64>} coinblocksDestroyed
 * @property {CumulativeHeightSumPattern<StoredF64>} coindaysDestroyed
 * @property {MetricPattern20<Sats>} satblocksDestroyed
 * @property {MetricPattern20<Sats>} satdaysDestroyed
 * @property {BtcSatsUsdPattern2} sent
 * @property {BtcSatsUsdPattern} sent14dEma
 */

/**
 * Create a CoinblocksCoindaysSatblocksSatdaysSentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinblocksCoindaysSatblocksSatdaysSentPattern}
 */
function createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc) {
  return {
    coinblocksDestroyed: createCumulativeHeightSumPattern(client, _m(acc, 'coinblocks_destroyed')),
    coindaysDestroyed: createCumulativeHeightSumPattern(client, _m(acc, 'coindays_destroyed')),
    satblocksDestroyed: createMetricPattern20(client, _m(acc, 'satblocks_destroyed')),
    satdaysDestroyed: createMetricPattern20(client, _m(acc, 'satdays_destroyed')),
    sent: createBtcSatsUsdPattern2(client, _m(acc, 'sent')),
    sent14dEma: createBtcSatsUsdPattern(client, _m(acc, 'sent_14d_ema')),
  };
}

/**
 * @typedef {Object} InvestedMaxMinPercentilesSpotPattern
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} investedCapital
 * @property {SatsUsdPattern} max
 * @property {SatsUsdPattern} min
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} percentiles
 * @property {MetricPattern1<StoredF32>} spotCostBasisPercentile
 * @property {MetricPattern1<StoredF32>} spotInvestedCapitalPercentile
 */

/**
 * Create a InvestedMaxMinPercentilesSpotPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedMaxMinPercentilesSpotPattern}
 */
function createInvestedMaxMinPercentilesSpotPattern(client, acc) {
  return {
    investedCapital: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital')),
    max: createSatsUsdPattern(client, _m(acc, 'max_cost_basis')),
    min: createSatsUsdPattern(client, _m(acc, 'min_cost_basis')),
    percentiles: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis')),
    spotCostBasisPercentile: createMetricPattern1(client, _m(acc, 'spot_cost_basis_percentile')),
    spotInvestedCapitalPercentile: createMetricPattern1(client, _m(acc, 'spot_invested_capital_percentile')),
  };
}

/**
 * @typedef {Object} _1y24h30d7dPattern2
 * @property {BtcSatsUsdPattern} _1y
 * @property {BtcSatsUsdPattern} _24h
 * @property {BtcSatsUsdPattern} _30d
 * @property {BtcSatsUsdPattern} _7d
 */

/**
 * Create a _1y24h30d7dPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1y24h30d7dPattern2}
 */
function create_1y24h30d7dPattern2(client, acc) {
  return {
    _1y: createBtcSatsUsdPattern(client, _m(acc, '1y')),
    _24h: createBtcSatsUsdPattern(client, _m(acc, '24h')),
    _30d: createBtcSatsUsdPattern(client, _m(acc, '30d')),
    _7d: createBtcSatsUsdPattern(client, _m(acc, '7d')),
  };
}

/**
 * @typedef {Object} BtcRollingSatsUsdPattern
 * @property {MetricPattern20<Bitcoin>} btc
 * @property {_1y24h30d7dPattern2} rolling
 * @property {MetricPattern20<Sats>} sats
 * @property {MetricPattern20<Dollars>} usd
 */

/**
 * Create a BtcRollingSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcRollingSatsUsdPattern}
 */
function createBtcRollingSatsUsdPattern(client, acc) {
  return {
    btc: createMetricPattern20(client, _m(acc, 'btc')),
    rolling: create_1y24h30d7dPattern2(client, acc),
    sats: createMetricPattern20(client, acc),
    usd: createMetricPattern20(client, _m(acc, 'usd')),
  };
}

/**
 * @template T
 * @typedef {Object} _1h24hBlockTxindexPattern
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} _1h
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} _24h
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>} block
 * @property {MetricPattern21<T>} txindex
 */

/**
 * Create a _1h24hBlockTxindexPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1h24hBlockTxindexPattern<T>}
 */
function create_1h24hBlockTxindexPattern(client, acc) {
  return {
    _1h: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '1h')),
    _24h: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '24h')),
    block: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc),
    txindex: createMetricPattern21(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} _1y24h30d7dPattern
 * @property {MetricPattern1<T>} _1y
 * @property {MetricPattern1<T>} _24h
 * @property {MetricPattern1<T>} _30d
 * @property {MetricPattern1<T>} _7d
 */

/**
 * Create a _1y24h30d7dPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1y24h30d7dPattern<T>}
 */
function create_1y24h30d7dPattern(client, acc) {
  return {
    _1y: createMetricPattern1(client, _m(acc, '1y')),
    _24h: createMetricPattern1(client, _m(acc, '24h')),
    _30d: createMetricPattern1(client, _m(acc, '30d')),
    _7d: createMetricPattern1(client, _m(acc, '7d')),
  };
}

/**
 * @typedef {Object} _30dHalvedTotalPattern
 * @property {BtcSatsUsdPattern} _30dChange
 * @property {BtcSatsUsdPattern} halved
 * @property {BtcSatsUsdPattern} total
 */

/**
 * Create a _30dHalvedTotalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_30dHalvedTotalPattern}
 */
function create_30dHalvedTotalPattern(client, acc) {
  return {
    _30dChange: createBtcSatsUsdPattern(client, _m(acc, '_30d_change')),
    halved: createBtcSatsUsdPattern(client, _m(acc, 'supply_halved')),
    total: createBtcSatsUsdPattern(client, _m(acc, 'supply')),
  };
}

/**
 * @typedef {Object} BtcSatsUsdPattern2
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {CumulativeHeightPattern<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcSatsUsdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcSatsUsdPattern2}
 */
function createBtcSatsUsdPattern2(client, acc) {
  return {
    btc: createMetricPattern1(client, _m(acc, 'btc')),
    sats: createCumulativeHeightPattern(client, acc),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} BtcSatsUsdPattern3
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {CumulativeHeightRollingPattern<Sats>} sats
 * @property {CumulativeHeightRollingPattern<Dollars>} usd
 */

/**
 * Create a BtcSatsUsdPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcSatsUsdPattern3}
 */
function createBtcSatsUsdPattern3(client, acc) {
  return {
    btc: createMetricPattern1(client, _m(acc, 'btc')),
    sats: createCumulativeHeightRollingPattern(client, acc),
    usd: createCumulativeHeightRollingPattern(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} BtcSatsUsdPattern4
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {CumulativeHeightSumPattern<Sats>} sats
 * @property {CumulativeHeightSumPattern<Dollars>} usd
 */

/**
 * Create a BtcSatsUsdPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcSatsUsdPattern4}
 */
function createBtcSatsUsdPattern4(client, acc) {
  return {
    btc: createMetricPattern1(client, _m(acc, 'btc')),
    sats: createCumulativeHeightSumPattern(client, acc),
    usd: createCumulativeHeightSumPattern(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} BtcSatsUsdPattern
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {MetricPattern1<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcSatsUsdPattern}
 */
function createBtcSatsUsdPattern(client, acc) {
  return {
    btc: createMetricPattern1(client, _m(acc, 'btc')),
    sats: createMetricPattern1(client, acc),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} CentsSatsUsdPattern
 * @property {Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern<OHLCCents>} cents
 * @property {Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern<OHLCSats>} sats
 * @property {Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern<OHLCDollars>} usd
 */

/**
 * Create a CentsSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsSatsUsdPattern}
 */
function createCentsSatsUsdPattern(client, acc) {
  return {
    cents: createDay1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, _m(acc, 'cents')),
    sats: createDay1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, _m(acc, 'sats')),
    usd: createDay1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, acc),
  };
}

/**
 * @typedef {Object} HistogramLineSignalPattern
 * @property {MetricPattern1<StoredF32>} histogram
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 */

/**
 * Create a HistogramLineSignalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {HistogramLineSignalPattern}
 */
function createHistogramLineSignalPattern(client, acc) {
  return {
    histogram: createMetricPattern1(client, _m(acc, 'histogram_1y')),
    line: createMetricPattern1(client, _m(acc, 'line_1y')),
    signal: createMetricPattern1(client, _m(acc, 'signal_1y')),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeHeightRollingPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern20<T>} height
 * @property {AverageMaxMedianMinP10P25P75P90SumPattern} rolling
 */

/**
 * Create a CumulativeHeightRollingPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CumulativeHeightRollingPattern<T>}
 */
function createCumulativeHeightRollingPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    height: createMetricPattern20(client, acc),
    rolling: createAverageMaxMedianMinP10P25P75P90SumPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeHeightSumPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern20<T>} height
 * @property {_1y24h30d7dPattern<T>} sum
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
    height: createMetricPattern20(client, acc),
    sum: create_1y24h30d7dPattern(client, acc),
  };
}

/**
 * @typedef {Object} _30dCountPattern
 * @property {MetricPattern1<StoredF64>} _30dChange
 * @property {MetricPattern1<StoredU64>} count
 */

/**
 * Create a _30dCountPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_30dCountPattern}
 */
function create_30dCountPattern(client, acc) {
  return {
    _30dChange: createMetricPattern1(client, _m(acc, '30d_change')),
    count: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} BaseRestPattern
 * @property {MetricPattern20<StoredU64>} base
 * @property {AverageCumulativeMaxMedianMinP10P25P75P90SumPattern} rest
 */

/**
 * Create a BaseRestPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseRestPattern}
 */
function createBaseRestPattern(client, acc) {
  return {
    base: createMetricPattern20(client, acc),
    rest: createAverageCumulativeMaxMedianMinP10P25P75P90SumPattern(client, acc),
  };
}

/**
 * @typedef {Object} MaxMinPattern
 * @property {SatsUsdPattern} max
 * @property {SatsUsdPattern} min
 */

/**
 * Create a MaxMinPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {MaxMinPattern}
 */
function createMaxMinPattern(client, acc) {
  return {
    max: createSatsUsdPattern(client, _m(acc, 'max_cost_basis')),
    min: createSatsUsdPattern(client, _m(acc, 'min_cost_basis')),
  };
}

/**
 * @typedef {Object} SatsUsdPattern
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a SatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SatsUsdPattern}
 */
function createSatsUsdPattern(client, acc) {
  return {
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, acc),
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
    sd: createMetricPattern1(client, _m(acc, 'sd')),
    sma: createMetricPattern1(client, _m(acc, 'sma')),
  };
}

/**
 * @typedef {Object} UtxoPattern
 * @property {MetricPattern1<StoredU64>} utxoCount
 * @property {MetricPattern1<StoredF64>} utxoCount30dChange
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
    utxoCount30dChange: createMetricPattern1(client, _m(acc, '30d_change')),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeHeightPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern20<T>} height
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
    height: createMetricPattern20(client, acc),
  };
}

/**
 * @typedef {Object} RatioPattern2
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a RatioPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioPattern2}
 */
function createRatioPattern2(client, acc) {
  return {
    ratio: createMetricPattern1(client, acc),
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
 * @property {MetricPattern20<BlockHash>} blockhash
 * @property {MetricsTree_Blocks_Difficulty} difficulty
 * @property {MetricsTree_Blocks_Time} time
 * @property {MetricPattern20<StoredU64>} totalSize
 * @property {MetricsTree_Blocks_Weight} weight
 * @property {MetricsTree_Blocks_Count} count
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<Timestamp>} interval
 * @property {MetricsTree_Blocks_Halving} halving
 * @property {CumulativeHeightRollingPattern<StoredU64>} vbytes
 * @property {AverageCumulativeMaxMedianMinP10P25P75P90SumPattern} size
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} fullness
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Difficulty
 * @property {MetricPattern1<StoredF64>} raw
 * @property {MetricPattern1<StoredF32>} asHash
 * @property {MetricPattern1<StoredF32>} adjustment
 * @property {MetricPattern1<DifficultyEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextAdjustment
 * @property {MetricPattern1<StoredF32>} daysBeforeNextAdjustment
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time
 * @property {MetricsTree_Blocks_Time_Timestamp} timestamp
 * @property {MetricPattern20<Date>} date
 * @property {MetricPattern20<Timestamp>} timestampMonotonic
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time_Timestamp
 * @property {MetricPattern20<Timestamp>} base
 * @property {MetricPattern3<Timestamp>} minute1
 * @property {MetricPattern4<Timestamp>} minute5
 * @property {MetricPattern5<Timestamp>} minute10
 * @property {MetricPattern6<Timestamp>} minute30
 * @property {MetricPattern7<Timestamp>} hour1
 * @property {MetricPattern8<Timestamp>} hour4
 * @property {MetricPattern9<Timestamp>} hour12
 * @property {MetricPattern10<Timestamp>} day1
 * @property {MetricPattern11<Timestamp>} day3
 * @property {MetricPattern12<Timestamp>} week1
 * @property {MetricPattern13<Timestamp>} month1
 * @property {MetricPattern14<Timestamp>} month3
 * @property {MetricPattern15<Timestamp>} month6
 * @property {MetricPattern16<Timestamp>} year1
 * @property {MetricPattern17<Timestamp>} year10
 * @property {MetricPattern18<Timestamp>} halvingepoch
 * @property {MetricPattern19<Timestamp>} difficultyepoch
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Weight
 * @property {MetricPattern20<Weight>} base
 * @property {MetricPattern1<Weight>} cumulative
 * @property {_1y24h30d7dPattern<Weight>} sum
 * @property {_1y24h30d7dPattern<Weight>} average
 * @property {_1y24h30d7dPattern<Weight>} min
 * @property {_1y24h30d7dPattern<Weight>} max
 * @property {_1y24h30d7dPattern<Weight>} p10
 * @property {_1y24h30d7dPattern<Weight>} p25
 * @property {_1y24h30d7dPattern<Weight>} median
 * @property {_1y24h30d7dPattern<Weight>} p75
 * @property {_1y24h30d7dPattern<Weight>} p90
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Count
 * @property {MetricPattern1<StoredU64>} blockCountTarget
 * @property {CumulativeHeightSumPattern<StoredU32>} blockCount
 * @property {_1y24h30d7dPattern<StoredU32>} blockCountSum
 * @property {MetricPattern20<Height>} height1hAgo
 * @property {MetricPattern20<Height>} height24hAgo
 * @property {MetricPattern20<Height>} height3dAgo
 * @property {MetricPattern20<Height>} height1wAgo
 * @property {MetricPattern20<Height>} height8dAgo
 * @property {MetricPattern20<Height>} height9dAgo
 * @property {MetricPattern20<Height>} height12dAgo
 * @property {MetricPattern20<Height>} height13dAgo
 * @property {MetricPattern20<Height>} height2wAgo
 * @property {MetricPattern20<Height>} height21dAgo
 * @property {MetricPattern20<Height>} height26dAgo
 * @property {MetricPattern20<Height>} height1mAgo
 * @property {MetricPattern20<Height>} height34dAgo
 * @property {MetricPattern20<Height>} height55dAgo
 * @property {MetricPattern20<Height>} height2mAgo
 * @property {MetricPattern20<Height>} height89dAgo
 * @property {MetricPattern20<Height>} height111dAgo
 * @property {MetricPattern20<Height>} height144dAgo
 * @property {MetricPattern20<Height>} height3mAgo
 * @property {MetricPattern20<Height>} height6mAgo
 * @property {MetricPattern20<Height>} height200dAgo
 * @property {MetricPattern20<Height>} height350dAgo
 * @property {MetricPattern20<Height>} height1yAgo
 * @property {MetricPattern20<Height>} height2yAgo
 * @property {MetricPattern20<Height>} height200wAgo
 * @property {MetricPattern20<Height>} height3yAgo
 * @property {MetricPattern20<Height>} height4yAgo
 * @property {MetricPattern20<Height>} height5yAgo
 * @property {MetricPattern20<Height>} height6yAgo
 * @property {MetricPattern20<Height>} height8yAgo
 * @property {MetricPattern20<Height>} height10yAgo
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Halving
 * @property {MetricPattern1<HalvingEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextHalving
 * @property {MetricPattern1<StoredF32>} daysBeforeNextHalving
 */

/**
 * @typedef {Object} MetricsTree_Transactions
 * @property {MetricPattern20<TxIndex>} firstTxindex
 * @property {MetricPattern21<Height>} height
 * @property {MetricPattern21<Txid>} txid
 * @property {MetricPattern21<TxVersion>} txversion
 * @property {MetricPattern21<RawLockTime>} rawlocktime
 * @property {MetricPattern21<StoredU32>} baseSize
 * @property {MetricPattern21<StoredU32>} totalSize
 * @property {MetricPattern21<StoredBool>} isExplicitlyRbf
 * @property {MetricPattern21<TxInIndex>} firstTxinindex
 * @property {MetricPattern21<TxOutIndex>} firstTxoutindex
 * @property {MetricsTree_Transactions_Count} count
 * @property {MetricsTree_Transactions_Size} size
 * @property {MetricsTree_Transactions_Fees} fees
 * @property {MetricsTree_Transactions_Versions} versions
 * @property {MetricsTree_Transactions_Volume} volume
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Count
 * @property {CumulativeHeightRollingPattern<StoredU64>} txCount
 * @property {MetricPattern21<StoredBool>} isCoinbase
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Size
 * @property {_1h24hBlockTxindexPattern<VSize>} vsize
 * @property {_1h24hBlockTxindexPattern<Weight>} weight
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees
 * @property {MetricPattern21<Sats>} inputValue
 * @property {MetricPattern21<Sats>} outputValue
 * @property {_1h24hBlockTxindexPattern<Sats>} fee
 * @property {_1h24hBlockTxindexPattern<FeeRate>} feeRate
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Versions
 * @property {CumulativeHeightSumPattern<StoredU64>} v1
 * @property {CumulativeHeightSumPattern<StoredU64>} v2
 * @property {CumulativeHeightSumPattern<StoredU64>} v3
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Volume
 * @property {BtcRollingSatsUsdPattern} sentSum
 * @property {BtcRollingSatsUsdPattern} receivedSum
 * @property {BtcSatsUsdPattern} annualizedVolume
 * @property {MetricPattern1<StoredF32>} txPerSec
 * @property {MetricPattern1<StoredF32>} outputsPerSec
 * @property {MetricPattern1<StoredF32>} inputsPerSec
 */

/**
 * @typedef {Object} MetricsTree_Inputs
 * @property {MetricPattern20<TxInIndex>} firstTxinindex
 * @property {MetricPattern22<OutPoint>} outpoint
 * @property {MetricPattern22<TxIndex>} txindex
 * @property {MetricPattern22<OutputType>} outputtype
 * @property {MetricPattern22<TypeIndex>} typeindex
 * @property {MetricsTree_Inputs_Spent} spent
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} count
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Spent
 * @property {MetricPattern22<TxOutIndex>} txoutindex
 * @property {MetricPattern22<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Outputs
 * @property {MetricPattern20<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern23<Sats>} value
 * @property {MetricPattern23<OutputType>} outputtype
 * @property {MetricPattern23<TypeIndex>} typeindex
 * @property {MetricPattern23<TxIndex>} txindex
 * @property {MetricsTree_Outputs_Spent} spent
 * @property {MetricsTree_Outputs_Count} count
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Spent
 * @property {MetricPattern23<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Count
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} totalCount
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * @typedef {Object} MetricsTree_Addresses
 * @property {MetricPattern20<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {MetricPattern20<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {MetricPattern20<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {MetricPattern20<P2SHAddressIndex>} firstP2shaddressindex
 * @property {MetricPattern20<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {MetricPattern20<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {MetricPattern20<P2TRAddressIndex>} firstP2traddressindex
 * @property {MetricPattern20<P2AAddressIndex>} firstP2aaddressindex
 * @property {MetricPattern29<P2PK65Bytes>} p2pk65bytes
 * @property {MetricPattern28<P2PK33Bytes>} p2pk33bytes
 * @property {MetricPattern30<P2PKHBytes>} p2pkhbytes
 * @property {MetricPattern31<P2SHBytes>} p2shbytes
 * @property {MetricPattern33<P2WPKHBytes>} p2wpkhbytes
 * @property {MetricPattern34<P2WSHBytes>} p2wshbytes
 * @property {MetricPattern32<P2TRBytes>} p2trbytes
 * @property {MetricPattern26<P2ABytes>} p2abytes
 */

/**
 * @typedef {Object} MetricsTree_Scripts
 * @property {MetricPattern20<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {MetricPattern20<OpReturnIndex>} firstOpreturnindex
 * @property {MetricPattern20<P2MSOutputIndex>} firstP2msoutputindex
 * @property {MetricPattern20<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricPattern24<TxIndex>} emptyToTxindex
 * @property {MetricPattern25<TxIndex>} opreturnToTxindex
 * @property {MetricPattern27<TxIndex>} p2msToTxindex
 * @property {MetricPattern35<TxIndex>} unknownToTxindex
 * @property {MetricsTree_Scripts_Count} count
 * @property {MetricsTree_Scripts_Value} value
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
 * @property {MetricPattern1<StoredF32>} taprootAdoption
 * @property {MetricPattern1<StoredF32>} segwitAdoption
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value
 * @property {BtcSatsUsdPattern3} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Mining
 * @property {MetricsTree_Mining_Rewards} rewards
 * @property {MetricsTree_Mining_Hashrate} hashrate
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards
 * @property {BtcSatsUsdPattern3} coinbase
 * @property {BtcSatsUsdPattern3} subsidy
 * @property {BtcSatsUsdPattern3} fees
 * @property {BtcSatsUsdPattern4} unclaimedRewards
 * @property {MetricPattern1<StoredF32>} feeDominance
 * @property {MetricPattern1<StoredF32>} feeDominance24h
 * @property {MetricPattern1<StoredF32>} feeDominance7d
 * @property {MetricPattern1<StoredF32>} feeDominance30d
 * @property {MetricPattern1<StoredF32>} feeDominance1y
 * @property {MetricPattern1<StoredF32>} subsidyDominance
 * @property {MetricPattern1<StoredF32>} subsidyDominance24h
 * @property {MetricPattern1<StoredF32>} subsidyDominance7d
 * @property {MetricPattern1<StoredF32>} subsidyDominance30d
 * @property {MetricPattern1<StoredF32>} subsidyDominance1y
 * @property {MetricPattern1<Dollars>} subsidyUsd1ySma
 */

/**
 * @typedef {Object} MetricsTree_Mining_Hashrate
 * @property {MetricPattern1<StoredF64>} hashRate
 * @property {MetricPattern1<StoredF64>} hashRate1wSma
 * @property {MetricPattern1<StoredF32>} hashRate1mSma
 * @property {MetricPattern1<StoredF32>} hashRate2mSma
 * @property {MetricPattern1<StoredF32>} hashRate1ySma
 * @property {MetricPattern1<StoredF64>} hashRateAth
 * @property {MetricPattern1<StoredF32>} hashRateDrawdown
 * @property {MetricPattern1<StoredF32>} hashPriceThs
 * @property {MetricPattern1<StoredF32>} hashPriceThsMin
 * @property {MetricPattern1<StoredF32>} hashPricePhs
 * @property {MetricPattern1<StoredF32>} hashPricePhsMin
 * @property {MetricPattern1<StoredF32>} hashPriceRebound
 * @property {MetricPattern1<StoredF32>} hashValueThs
 * @property {MetricPattern1<StoredF32>} hashValueThsMin
 * @property {MetricPattern1<StoredF32>} hashValuePhs
 * @property {MetricPattern1<StoredF32>} hashValuePhsMin
 * @property {MetricPattern1<StoredF32>} hashValueRebound
 */

/**
 * @typedef {Object} MetricsTree_Positions
 * @property {MetricPattern20<BlkPosition>} blockPosition
 * @property {MetricPattern21<BlkPosition>} txPosition
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
 * @property {BtcSatsUsdPattern} vaultedSupply
 * @property {BtcSatsUsdPattern} activeSupply
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
 * @property {MetricPattern1<Dollars>} thermoCap
 * @property {MetricPattern1<Dollars>} investorCap
 * @property {MetricPattern1<Dollars>} vaultedCap
 * @property {MetricPattern1<Dollars>} activeCap
 * @property {MetricPattern1<Dollars>} cointimeCap
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Pricing
 * @property {SatsUsdPattern} vaultedPrice
 * @property {RatioPattern} vaultedPriceRatio
 * @property {SatsUsdPattern} activePrice
 * @property {RatioPattern} activePriceRatio
 * @property {SatsUsdPattern} trueMarketMean
 * @property {RatioPattern} trueMarketMeanRatio
 * @property {SatsUsdPattern} cointimePrice
 * @property {RatioPattern} cointimePriceRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Adjusted
 * @property {MetricPattern1<StoredF32>} cointimeAdjInflationRate
 * @property {MetricPattern1<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {MetricPattern1<StoredF64>} cointimeAdjTxUsdVelocity
 */

/**
 * @typedef {Object} MetricsTree_Cointime_ReserveRisk
 * @property {MetricPattern20<StoredF64>} vocdd365dMedian
 * @property {MetricPattern20<StoredF64>} hodlBank
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
 * @property {MetricsTree_Indexes_Minute1} minute1
 * @property {MetricsTree_Indexes_Minute5} minute5
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
 * @property {MetricPattern28<P2PK33AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pk65
 * @property {MetricPattern29<P2PK65AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pkh
 * @property {MetricPattern30<P2PKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2sh
 * @property {MetricPattern31<P2SHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2tr
 * @property {MetricPattern32<P2TRAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wpkh
 * @property {MetricPattern33<P2WPKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wsh
 * @property {MetricPattern34<P2WSHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2a
 * @property {MetricPattern26<P2AAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2ms
 * @property {MetricPattern27<P2MSOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Empty
 * @property {MetricPattern24<EmptyOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Unknown
 * @property {MetricPattern35<UnknownOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Opreturn
 * @property {MetricPattern25<OpReturnIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Height
 * @property {MetricPattern20<Height>} identity
 * @property {MetricPattern20<Minute1>} minute1
 * @property {MetricPattern20<Minute5>} minute5
 * @property {MetricPattern20<Minute10>} minute10
 * @property {MetricPattern20<Minute30>} minute30
 * @property {MetricPattern20<Hour1>} hour1
 * @property {MetricPattern20<Hour4>} hour4
 * @property {MetricPattern20<Hour12>} hour12
 * @property {MetricPattern20<Day1>} day1
 * @property {MetricPattern20<Day3>} day3
 * @property {MetricPattern20<DifficultyEpoch>} difficultyepoch
 * @property {MetricPattern20<HalvingEpoch>} halvingepoch
 * @property {MetricPattern20<Week1>} week1
 * @property {MetricPattern20<Month1>} month1
 * @property {MetricPattern20<Month3>} month3
 * @property {MetricPattern20<Month6>} month6
 * @property {MetricPattern20<Year1>} year1
 * @property {MetricPattern20<Year10>} year10
 * @property {MetricPattern20<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Difficultyepoch
 * @property {MetricPattern19<DifficultyEpoch>} identity
 * @property {MetricPattern19<Height>} firstHeight
 * @property {MetricPattern19<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Halvingepoch
 * @property {MetricPattern18<HalvingEpoch>} identity
 * @property {MetricPattern18<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute1
 * @property {MetricPattern3<Minute1>} identity
 * @property {MetricPattern3<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute5
 * @property {MetricPattern4<Minute5>} identity
 * @property {MetricPattern4<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute10
 * @property {MetricPattern5<Minute10>} identity
 * @property {MetricPattern5<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Minute30
 * @property {MetricPattern6<Minute30>} identity
 * @property {MetricPattern6<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour1
 * @property {MetricPattern7<Hour1>} identity
 * @property {MetricPattern7<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour4
 * @property {MetricPattern8<Hour4>} identity
 * @property {MetricPattern8<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Hour12
 * @property {MetricPattern9<Hour12>} identity
 * @property {MetricPattern9<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Day1
 * @property {MetricPattern10<Day1>} identity
 * @property {MetricPattern10<Date>} date
 * @property {MetricPattern10<Height>} firstHeight
 * @property {MetricPattern10<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Day3
 * @property {MetricPattern11<Day3>} identity
 * @property {MetricPattern11<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Week1
 * @property {MetricPattern12<Week1>} identity
 * @property {MetricPattern12<Date>} date
 * @property {MetricPattern12<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month1
 * @property {MetricPattern13<Month1>} identity
 * @property {MetricPattern13<Date>} date
 * @property {MetricPattern13<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month3
 * @property {MetricPattern14<Month3>} identity
 * @property {MetricPattern14<Date>} date
 * @property {MetricPattern14<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Month6
 * @property {MetricPattern15<Month6>} identity
 * @property {MetricPattern15<Date>} date
 * @property {MetricPattern15<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Year1
 * @property {MetricPattern16<Year1>} identity
 * @property {MetricPattern16<Date>} date
 * @property {MetricPattern16<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Year10
 * @property {MetricPattern17<Year10>} identity
 * @property {MetricPattern17<Date>} date
 * @property {MetricPattern17<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txindex
 * @property {MetricPattern21<TxIndex>} identity
 * @property {MetricPattern21<StoredU64>} inputCount
 * @property {MetricPattern21<StoredU64>} outputCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txinindex
 * @property {MetricPattern22<TxInIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txoutindex
 * @property {MetricPattern23<TxOutIndex>} identity
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
 * @property {SatsUsdPattern} priceAth
 * @property {MetricPattern1<StoredF32>} priceDrawdown
 * @property {MetricPattern1<StoredU16>} daysSincePriceAth
 * @property {MetricPattern2<StoredF32>} yearsSincePriceAth
 * @property {MetricPattern1<StoredU16>} maxDaysBetweenPriceAths
 * @property {MetricPattern2<StoredF32>} maxYearsBetweenPriceAths
 */

/**
 * @typedef {Object} MetricsTree_Market_Lookback
 * @property {SatsUsdPattern} _1d
 * @property {SatsUsdPattern} _1w
 * @property {SatsUsdPattern} _1m
 * @property {SatsUsdPattern} _3m
 * @property {SatsUsdPattern} _6m
 * @property {SatsUsdPattern} _1y
 * @property {SatsUsdPattern} _2y
 * @property {SatsUsdPattern} _3y
 * @property {SatsUsdPattern} _4y
 * @property {SatsUsdPattern} _5y
 * @property {SatsUsdPattern} _6y
 * @property {SatsUsdPattern} _8y
 * @property {SatsUsdPattern} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns
 * @property {MetricsTree_Market_Returns_PriceReturns} priceReturns
 * @property {_10y2y3y4y5y6y8yPattern} cagr
 * @property {SdSmaPattern} _1dReturns1wSd
 * @property {SdSmaPattern} _1dReturns1mSd
 * @property {SdSmaPattern} _1dReturns1ySd
 * @property {MetricPattern20<StoredF32>} downsideReturns
 * @property {SdSmaPattern} downside1wSd
 * @property {SdSmaPattern} downside1mSd
 * @property {SdSmaPattern} downside1ySd
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturns
 * @property {MetricPattern1<StoredF32>} _1d
 * @property {MetricPattern1<StoredF32>} _1w
 * @property {MetricPattern1<StoredF32>} _1m
 * @property {MetricPattern1<StoredF32>} _3m
 * @property {MetricPattern1<StoredF32>} _6m
 * @property {MetricPattern1<StoredF32>} _1y
 * @property {MetricPattern1<StoredF32>} _2y
 * @property {MetricPattern1<StoredF32>} _3y
 * @property {MetricPattern1<StoredF32>} _4y
 * @property {MetricPattern1<StoredF32>} _5y
 * @property {MetricPattern1<StoredF32>} _6y
 * @property {MetricPattern1<StoredF32>} _8y
 * @property {MetricPattern1<StoredF32>} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Volatility
 * @property {MetricPattern1<StoredF32>} price1wVolatility
 * @property {MetricPattern1<StoredF32>} price1mVolatility
 * @property {MetricPattern1<StoredF32>} price1yVolatility
 * @property {MetricPattern1<StoredF32>} sharpe1w
 * @property {MetricPattern1<StoredF32>} sharpe1m
 * @property {MetricPattern1<StoredF32>} sharpe1y
 * @property {MetricPattern1<StoredF32>} sortino1w
 * @property {MetricPattern1<StoredF32>} sortino1m
 * @property {MetricPattern1<StoredF32>} sortino1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Range
 * @property {SatsUsdPattern} price1wMin
 * @property {SatsUsdPattern} price1wMax
 * @property {SatsUsdPattern} price2wMin
 * @property {SatsUsdPattern} price2wMax
 * @property {SatsUsdPattern} price1mMin
 * @property {SatsUsdPattern} price1mMax
 * @property {SatsUsdPattern} price1yMin
 * @property {SatsUsdPattern} price1yMax
 * @property {MetricPattern1<StoredF32>} priceTrueRange
 * @property {MetricPattern1<StoredF32>} priceTrueRange2wSum
 * @property {MetricPattern1<StoredF32>} price2wChoppinessIndex
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage
 * @property {PriceRatioPattern} price1wSma
 * @property {PriceRatioPattern} price8dSma
 * @property {PriceRatioPattern} price13dSma
 * @property {PriceRatioPattern} price21dSma
 * @property {PriceRatioPattern} price1mSma
 * @property {PriceRatioPattern} price34dSma
 * @property {PriceRatioPattern} price55dSma
 * @property {PriceRatioPattern} price89dSma
 * @property {PriceRatioPattern} price111dSma
 * @property {PriceRatioPattern} price144dSma
 * @property {PriceRatioPattern} price200dSma
 * @property {PriceRatioPattern} price350dSma
 * @property {PriceRatioPattern} price1ySma
 * @property {PriceRatioPattern} price2ySma
 * @property {PriceRatioPattern} price200wSma
 * @property {PriceRatioPattern} price4ySma
 * @property {PriceRatioPattern} price1wEma
 * @property {PriceRatioPattern} price8dEma
 * @property {PriceRatioPattern} price12dEma
 * @property {PriceRatioPattern} price13dEma
 * @property {PriceRatioPattern} price21dEma
 * @property {PriceRatioPattern} price26dEma
 * @property {PriceRatioPattern} price1mEma
 * @property {PriceRatioPattern} price34dEma
 * @property {PriceRatioPattern} price55dEma
 * @property {PriceRatioPattern} price89dEma
 * @property {PriceRatioPattern} price144dEma
 * @property {PriceRatioPattern} price200dEma
 * @property {PriceRatioPattern} price1yEma
 * @property {PriceRatioPattern} price2yEma
 * @property {PriceRatioPattern} price200wEma
 * @property {PriceRatioPattern} price4yEma
 * @property {SatsUsdPattern} price200dSmaX24
 * @property {SatsUsdPattern} price200dSmaX08
 * @property {SatsUsdPattern} price350dSmaX2
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca
 * @property {MetricPattern20<Sats>} dcaSatsPerDay
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} periodStack
 * @property {MetricsTree_Market_Dca_PeriodAveragePrice} periodAveragePrice
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodReturns
 * @property {_10y2y3y4y5y6y8yPattern} periodCagr
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>} periodDaysInProfit
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>} periodDaysInLoss
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodMinReturn
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodMaxReturn
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} periodLumpSumStack
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodLumpSumReturns
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>} periodLumpSumDaysInProfit
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>} periodLumpSumDaysInLoss
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodLumpSumMinReturn
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>} periodLumpSumMaxReturn
 * @property {MetricsTree_Market_Dca_ClassStack} classStack
 * @property {MetricsTree_Market_Dca_ClassAveragePrice} classAveragePrice
 * @property {_201520162017201820192020202120222023202420252026Pattern2<StoredF32>} classReturns
 * @property {MetricsTree_Market_Dca_ClassDaysInProfit} classDaysInProfit
 * @property {MetricsTree_Market_Dca_ClassDaysInLoss} classDaysInLoss
 * @property {MetricsTree_Market_Dca_ClassMinReturn} classMinReturn
 * @property {MetricsTree_Market_Dca_ClassMaxReturn} classMaxReturn
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_PeriodAveragePrice
 * @property {SatsUsdPattern} _1w
 * @property {SatsUsdPattern} _1m
 * @property {SatsUsdPattern} _3m
 * @property {SatsUsdPattern} _6m
 * @property {SatsUsdPattern} _1y
 * @property {SatsUsdPattern} _2y
 * @property {SatsUsdPattern} _3y
 * @property {SatsUsdPattern} _4y
 * @property {SatsUsdPattern} _5y
 * @property {SatsUsdPattern} _6y
 * @property {SatsUsdPattern} _8y
 * @property {SatsUsdPattern} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassStack
 * @property {BtcSatsUsdPattern} _2015
 * @property {BtcSatsUsdPattern} _2016
 * @property {BtcSatsUsdPattern} _2017
 * @property {BtcSatsUsdPattern} _2018
 * @property {BtcSatsUsdPattern} _2019
 * @property {BtcSatsUsdPattern} _2020
 * @property {BtcSatsUsdPattern} _2021
 * @property {BtcSatsUsdPattern} _2022
 * @property {BtcSatsUsdPattern} _2023
 * @property {BtcSatsUsdPattern} _2024
 * @property {BtcSatsUsdPattern} _2025
 * @property {BtcSatsUsdPattern} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassAveragePrice
 * @property {SatsUsdPattern} _2015
 * @property {SatsUsdPattern} _2016
 * @property {SatsUsdPattern} _2017
 * @property {SatsUsdPattern} _2018
 * @property {SatsUsdPattern} _2019
 * @property {SatsUsdPattern} _2020
 * @property {SatsUsdPattern} _2021
 * @property {SatsUsdPattern} _2022
 * @property {SatsUsdPattern} _2023
 * @property {SatsUsdPattern} _2024
 * @property {SatsUsdPattern} _2025
 * @property {SatsUsdPattern} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassDaysInProfit
 * @property {MetricPattern1<StoredU32>} _2015
 * @property {MetricPattern1<StoredU32>} _2016
 * @property {MetricPattern1<StoredU32>} _2017
 * @property {MetricPattern1<StoredU32>} _2018
 * @property {MetricPattern1<StoredU32>} _2019
 * @property {MetricPattern1<StoredU32>} _2020
 * @property {MetricPattern1<StoredU32>} _2021
 * @property {MetricPattern1<StoredU32>} _2022
 * @property {MetricPattern1<StoredU32>} _2023
 * @property {MetricPattern1<StoredU32>} _2024
 * @property {MetricPattern1<StoredU32>} _2025
 * @property {MetricPattern1<StoredU32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassDaysInLoss
 * @property {MetricPattern1<StoredU32>} _2015
 * @property {MetricPattern1<StoredU32>} _2016
 * @property {MetricPattern1<StoredU32>} _2017
 * @property {MetricPattern1<StoredU32>} _2018
 * @property {MetricPattern1<StoredU32>} _2019
 * @property {MetricPattern1<StoredU32>} _2020
 * @property {MetricPattern1<StoredU32>} _2021
 * @property {MetricPattern1<StoredU32>} _2022
 * @property {MetricPattern1<StoredU32>} _2023
 * @property {MetricPattern1<StoredU32>} _2024
 * @property {MetricPattern1<StoredU32>} _2025
 * @property {MetricPattern1<StoredU32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassMinReturn
 * @property {MetricPattern1<StoredF32>} _2015
 * @property {MetricPattern1<StoredF32>} _2016
 * @property {MetricPattern1<StoredF32>} _2017
 * @property {MetricPattern1<StoredF32>} _2018
 * @property {MetricPattern1<StoredF32>} _2019
 * @property {MetricPattern1<StoredF32>} _2020
 * @property {MetricPattern1<StoredF32>} _2021
 * @property {MetricPattern1<StoredF32>} _2022
 * @property {MetricPattern1<StoredF32>} _2023
 * @property {MetricPattern1<StoredF32>} _2024
 * @property {MetricPattern1<StoredF32>} _2025
 * @property {MetricPattern1<StoredF32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassMaxReturn
 * @property {MetricPattern1<StoredF32>} _2015
 * @property {MetricPattern1<StoredF32>} _2016
 * @property {MetricPattern1<StoredF32>} _2017
 * @property {MetricPattern1<StoredF32>} _2018
 * @property {MetricPattern1<StoredF32>} _2019
 * @property {MetricPattern1<StoredF32>} _2020
 * @property {MetricPattern1<StoredF32>} _2021
 * @property {MetricPattern1<StoredF32>} _2022
 * @property {MetricPattern1<StoredF32>} _2023
 * @property {MetricPattern1<StoredF32>} _2024
 * @property {MetricPattern1<StoredF32>} _2025
 * @property {MetricPattern1<StoredF32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators
 * @property {MetricPattern1<StoredF32>} puellMultiple
 * @property {MetricPattern1<StoredF32>} nvt
 * @property {MetricsTree_Market_Indicators_Rsi} rsi
 * @property {MetricPattern1<StoredF32>} stochK
 * @property {MetricPattern1<StoredF32>} stochD
 * @property {MetricPattern1<StoredF32>} piCycle
 * @property {MetricsTree_Market_Indicators_Macd} macd
 * @property {MetricPattern1<StoredF32>} gini
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi
 * @property {MetricsTree_Market_Indicators_Rsi_1d} _1d
 * @property {MetricsTree_Market_Indicators_Rsi_1w} _1w
 * @property {MetricsTree_Market_Indicators_Rsi_1m} _1m
 * @property {AverageGainsLossesRsiStochPattern} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1d
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} rsi
 * @property {MetricPattern1<StoredF32>} rsiMin
 * @property {MetricPattern1<StoredF32>} rsiMax
 * @property {MetricPattern1<StoredF32>} stochRsi
 * @property {MetricPattern1<StoredF32>} stochRsiK
 * @property {MetricPattern1<StoredF32>} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1w
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} rsi
 * @property {MetricPattern1<StoredF32>} rsiMin
 * @property {MetricPattern1<StoredF32>} rsiMax
 * @property {MetricPattern1<StoredF32>} stochRsi
 * @property {MetricPattern1<StoredF32>} stochRsiK
 * @property {MetricPattern1<StoredF32>} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Rsi_1m
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} rsi
 * @property {MetricPattern1<StoredF32>} rsiMin
 * @property {MetricPattern1<StoredF32>} rsiMax
 * @property {MetricPattern1<StoredF32>} stochRsi
 * @property {MetricPattern1<StoredF32>} stochRsiK
 * @property {MetricPattern1<StoredF32>} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd
 * @property {MetricsTree_Market_Indicators_Macd_1d} _1d
 * @property {MetricsTree_Market_Indicators_Macd_1w} _1w
 * @property {MetricsTree_Market_Indicators_Macd_1m} _1m
 * @property {HistogramLineSignalPattern} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1d
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1w
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators_Macd_1m
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Pools
 * @property {MetricPattern20<PoolSlug>} heightToPool
 * @property {MetricsTree_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} MetricsTree_Pools_Vecs
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} unknown
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} blockfills
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ultimuspool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} terrapool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} luxor
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} onethash
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btccom
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfarms
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} huobipool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} wayicn
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} canoepool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btctop
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoincom
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} pool175btc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} gbminers
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} axbt
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} asicminer
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitminter
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinrussia
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcserv
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} simplecoinus
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcguild
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} eligius
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ozcoin
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} eclipsemc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} maxbtc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} triplemining
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} coinlab
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} pool50btc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ghashio
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} stminingcorp
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitparking
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} mmpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} polmine
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} kncminer
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitalo
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} f2pool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} hhtt
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} megabigpower
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} mtred
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} nmcbit
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} yourbtcnet
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} givemecoins
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} braiinspool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} antpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} multicoinco
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bcpoolio
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} cointerra
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} kanopool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} solock
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ckpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} nicehash
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitclub
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinaffiliatenetwork
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bwpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} exxbw
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitsolo
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfury
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} twentyoneinc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} digitalbtc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} eightbaochi
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} mybtccoinpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} tbdice
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} hashpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} nexious
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bravomining
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} hotpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} okexpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bcmonster
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} onehash
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bixin
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} tatmaspool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} viabtc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} connectbtc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} batpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} waterhole
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} dcexploration
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} dcex
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} fiftyeightcoin
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinindia
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} shawnp0wers
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} phashio
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} rigpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} haozhuzhu
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} sevenpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningkings
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} hashbx
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} dpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} rawpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} haominer
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} helix
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinukraine
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} poolin
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} secretsuperstar
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} tigerpoolnet
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} sigmapoolcom
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} okpooltop
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} hummerpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} tangpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bytepool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} spiderpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} novablock
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningcity
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} binancepool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} minerium
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} lubiancom
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} okkong
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} aaopool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} emcdpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} foundryusa
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} sbicrypto
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} arkpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} purebtccom
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} marapool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} kucoinpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} entrustcharitypool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} okminer
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} titan
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} pegapool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcnuggets
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} cloudhashing
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} digitalxmintsy
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} telco214
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcpoolparty
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} multipool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} transactioncoinmining
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcdig
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} trickysbtcpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcmp
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} eobot
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} unomp
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} patels
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} gogreenlight
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinindiapool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ekanembtc
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} canoe
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} tiger
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} onem1x
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} zulupool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} secpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} ocean
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} whitepool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} wiz
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} wk057
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} futurebitapollosolo
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} carbonnegative
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} portlandhodl
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} phoenix
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} neopool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} maxipool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfufupool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} gdpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningdutch
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} publicpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningsquared
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} innopolistech
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} btclab
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} parasite
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} redrockpool
 * @property {BlocksCoinbaseDaysDominanceFeeSubsidyPattern} est3lar
 */

/**
 * @typedef {Object} MetricsTree_Prices
 * @property {MetricsTree_Prices_Split} split
 * @property {CentsSatsUsdPattern} ohlc
 * @property {MetricsTree_Prices_Price} price
 */

/**
 * @typedef {Object} MetricsTree_Prices_Split
 * @property {CentsSatsUsdPattern} open
 * @property {CentsSatsUsdPattern} high
 * @property {CentsSatsUsdPattern} low
 * @property {MetricsTree_Prices_Split_Close} close
 */

/**
 * @typedef {Object} MetricsTree_Prices_Split_Close
 * @property {MetricPattern2<Cents>} cents
 * @property {MetricPattern2<Dollars>} usd
 * @property {MetricPattern2<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Prices_Price
 * @property {MetricPattern20<Cents>} cents
 * @property {MetricPattern20<Dollars>} usd
 * @property {MetricPattern20<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Distribution
 * @property {MetricPattern20<SupplyState>} supplyState
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
 * @property {MetricPattern36<FundedAddressIndex>} fundedaddressindex
 * @property {MetricPattern37<EmptyAddressIndex>} emptyaddressindex
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AnyAddressIndexes
 * @property {MetricPattern26<AnyAddressIndex>} p2a
 * @property {MetricPattern28<AnyAddressIndex>} p2pk33
 * @property {MetricPattern29<AnyAddressIndex>} p2pk65
 * @property {MetricPattern30<AnyAddressIndex>} p2pkh
 * @property {MetricPattern31<AnyAddressIndex>} p2sh
 * @property {MetricPattern32<AnyAddressIndex>} p2tr
 * @property {MetricPattern33<AnyAddressIndex>} p2wpkh
 * @property {MetricPattern34<AnyAddressIndex>} p2wsh
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressesData
 * @property {MetricPattern36<FundedAddressData>} funded
 * @property {MetricPattern37<EmptyAddressData>} empty
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
 * @property {MetricsTree_Distribution_UtxoCohorts_Year} year
 * @property {MetricsTree_Distribution_UtxoCohorts_Type} type
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All
 * @property {_30dHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern} realized
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 * @property {MetricsTree_Distribution_UtxoCohorts_All_Relative} relative
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All_Relative
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedPeakRegretRelToMarketCap
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Sth
 * @property {_30dHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern} realized
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
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
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _1d
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _1w
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _1m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _2m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _3m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _4m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _5m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _6m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _1y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _2y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _3y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _4y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _5y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _6y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _7y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _8y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _10y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _12y
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
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Year
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
 * @property {BaseRestPattern} all
 * @property {BaseRestPattern} p2pk65
 * @property {BaseRestPattern} p2pk33
 * @property {BaseRestPattern} p2pkh
 * @property {BaseRestPattern} p2sh
 * @property {BaseRestPattern} p2wpkh
 * @property {BaseRestPattern} p2wsh
 * @property {BaseRestPattern} p2tr
 * @property {BaseRestPattern} p2a
 */

/**
 * @typedef {Object} MetricsTree_Distribution_GrowthRate
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} all
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2pk65
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2pk33
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2pkh
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2sh
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2wpkh
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2wsh
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2tr
 * @property {AverageHeightMaxMedianMinP10P25P75P90Pattern<StoredF32>} p2a
 */

/**
 * @typedef {Object} MetricsTree_Supply
 * @property {BtcSatsUsdPattern} circulating
 * @property {MetricsTree_Supply_Burned} burned
 * @property {MetricPattern1<StoredF32>} inflation
 * @property {MetricsTree_Supply_Velocity} velocity
 * @property {MetricPattern1<Dollars>} marketCap
 * @property {MetricPattern1<StoredF32>} marketCapGrowthRate
 * @property {MetricPattern1<StoredF32>} realizedCapGrowthRate
 * @property {MetricPattern1<StoredF32>} capGrowthRateDiff
 */

/**
 * @typedef {Object} MetricsTree_Supply_Burned
 * @property {BtcSatsUsdPattern4} opreturn
 * @property {BtcSatsUsdPattern4} unspendable
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
    "minute1",
    "minute5",
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

  YEAR_NAMES = /** @type {const} */ ({
    "_2009": {
      "id": "year_2009",
      "short": "2009",
      "long": "Year 2009"
    },
    "_2010": {
      "id": "year_2010",
      "short": "2010",
      "long": "Year 2010"
    },
    "_2011": {
      "id": "year_2011",
      "short": "2011",
      "long": "Year 2011"
    },
    "_2012": {
      "id": "year_2012",
      "short": "2012",
      "long": "Year 2012"
    },
    "_2013": {
      "id": "year_2013",
      "short": "2013",
      "long": "Year 2013"
    },
    "_2014": {
      "id": "year_2014",
      "short": "2014",
      "long": "Year 2014"
    },
    "_2015": {
      "id": "year_2015",
      "short": "2015",
      "long": "Year 2015"
    },
    "_2016": {
      "id": "year_2016",
      "short": "2016",
      "long": "Year 2016"
    },
    "_2017": {
      "id": "year_2017",
      "short": "2017",
      "long": "Year 2017"
    },
    "_2018": {
      "id": "year_2018",
      "short": "2018",
      "long": "Year 2018"
    },
    "_2019": {
      "id": "year_2019",
      "short": "2019",
      "long": "Year 2019"
    },
    "_2020": {
      "id": "year_2020",
      "short": "2020",
      "long": "Year 2020"
    },
    "_2021": {
      "id": "year_2021",
      "short": "2021",
      "long": "Year 2021"
    },
    "_2022": {
      "id": "year_2022",
      "short": "2022",
      "long": "Year 2022"
    },
    "_2023": {
      "id": "year_2023",
      "short": "2023",
      "long": "Year 2023"
    },
    "_2024": {
      "id": "year_2024",
      "short": "2024",
      "long": "Year 2024"
    },
    "_2025": {
      "id": "year_2025",
      "short": "2025",
      "long": "Year 2025"
    },
    "_2026": {
      "id": "year_2026",
      "short": "2026",
      "long": "Year 2026"
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
        blockhash: createMetricPattern20(this, 'blockhash'),
        difficulty: {
          raw: createMetricPattern1(this, 'difficulty'),
          asHash: createMetricPattern1(this, 'difficulty_as_hash'),
          adjustment: createMetricPattern1(this, 'difficulty_adjustment'),
          epoch: createMetricPattern1(this, 'difficulty_epoch'),
          blocksBeforeNextAdjustment: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
          daysBeforeNextAdjustment: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
        },
        time: {
          timestamp: {
            base: createMetricPattern20(this, 'timestamp'),
            minute1: createMetricPattern3(this, 'timestamp_minute1'),
            minute5: createMetricPattern4(this, 'timestamp_minute5'),
            minute10: createMetricPattern5(this, 'timestamp_minute10'),
            minute30: createMetricPattern6(this, 'timestamp_minute30'),
            hour1: createMetricPattern7(this, 'timestamp_hour1'),
            hour4: createMetricPattern8(this, 'timestamp_hour4'),
            hour12: createMetricPattern9(this, 'timestamp_hour12'),
            day1: createMetricPattern10(this, 'timestamp_day1'),
            day3: createMetricPattern11(this, 'timestamp_day3'),
            week1: createMetricPattern12(this, 'timestamp_week1'),
            month1: createMetricPattern13(this, 'timestamp_month1'),
            month3: createMetricPattern14(this, 'timestamp_month3'),
            month6: createMetricPattern15(this, 'timestamp_month6'),
            year1: createMetricPattern16(this, 'timestamp_year1'),
            year10: createMetricPattern17(this, 'timestamp_year10'),
            halvingepoch: createMetricPattern18(this, 'timestamp_halvingepoch'),
            difficultyepoch: createMetricPattern19(this, 'timestamp_difficultyepoch'),
          },
          date: createMetricPattern20(this, 'date'),
          timestampMonotonic: createMetricPattern20(this, 'timestamp_monotonic'),
        },
        totalSize: createMetricPattern20(this, 'total_size'),
        weight: {
          base: createMetricPattern20(this, 'block_weight'),
          cumulative: createMetricPattern1(this, 'block_weight_cumulative'),
          sum: create_1y24h30d7dPattern(this, 'block_weight_sum'),
          average: create_1y24h30d7dPattern(this, 'block_weight_average'),
          min: create_1y24h30d7dPattern(this, 'block_weight_min'),
          max: create_1y24h30d7dPattern(this, 'block_weight_max'),
          p10: create_1y24h30d7dPattern(this, 'block_weight_p10'),
          p25: create_1y24h30d7dPattern(this, 'block_weight_p25'),
          median: create_1y24h30d7dPattern(this, 'block_weight_median'),
          p75: create_1y24h30d7dPattern(this, 'block_weight_p75'),
          p90: create_1y24h30d7dPattern(this, 'block_weight_p90'),
        },
        count: {
          blockCountTarget: createMetricPattern1(this, 'block_count_target'),
          blockCount: createCumulativeHeightSumPattern(this, 'block_count'),
          blockCountSum: create_1y24h30d7dPattern(this, 'block_count_sum'),
          height1hAgo: createMetricPattern20(this, 'height_1h_ago'),
          height24hAgo: createMetricPattern20(this, 'height_24h_ago'),
          height3dAgo: createMetricPattern20(this, 'height_3d_ago'),
          height1wAgo: createMetricPattern20(this, 'height_1w_ago'),
          height8dAgo: createMetricPattern20(this, 'height_8d_ago'),
          height9dAgo: createMetricPattern20(this, 'height_9d_ago'),
          height12dAgo: createMetricPattern20(this, 'height_12d_ago'),
          height13dAgo: createMetricPattern20(this, 'height_13d_ago'),
          height2wAgo: createMetricPattern20(this, 'height_2w_ago'),
          height21dAgo: createMetricPattern20(this, 'height_21d_ago'),
          height26dAgo: createMetricPattern20(this, 'height_26d_ago'),
          height1mAgo: createMetricPattern20(this, 'height_1m_ago'),
          height34dAgo: createMetricPattern20(this, 'height_34d_ago'),
          height55dAgo: createMetricPattern20(this, 'height_55d_ago'),
          height2mAgo: createMetricPattern20(this, 'height_2m_ago'),
          height89dAgo: createMetricPattern20(this, 'height_89d_ago'),
          height111dAgo: createMetricPattern20(this, 'height_111d_ago'),
          height144dAgo: createMetricPattern20(this, 'height_144d_ago'),
          height3mAgo: createMetricPattern20(this, 'height_3m_ago'),
          height6mAgo: createMetricPattern20(this, 'height_6m_ago'),
          height200dAgo: createMetricPattern20(this, 'height_200d_ago'),
          height350dAgo: createMetricPattern20(this, 'height_350d_ago'),
          height1yAgo: createMetricPattern20(this, 'height_1y_ago'),
          height2yAgo: createMetricPattern20(this, 'height_2y_ago'),
          height200wAgo: createMetricPattern20(this, 'height_200w_ago'),
          height3yAgo: createMetricPattern20(this, 'height_3y_ago'),
          height4yAgo: createMetricPattern20(this, 'height_4y_ago'),
          height5yAgo: createMetricPattern20(this, 'height_5y_ago'),
          height6yAgo: createMetricPattern20(this, 'height_6y_ago'),
          height8yAgo: createMetricPattern20(this, 'height_8y_ago'),
          height10yAgo: createMetricPattern20(this, 'height_10y_ago'),
        },
        interval: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'block_interval'),
        halving: {
          epoch: createMetricPattern1(this, 'halving_epoch'),
          blocksBeforeNextHalving: createMetricPattern1(this, 'blocks_before_next_halving'),
          daysBeforeNextHalving: createMetricPattern1(this, 'days_before_next_halving'),
        },
        vbytes: createCumulativeHeightRollingPattern(this, 'block_vbytes'),
        size: createAverageCumulativeMaxMedianMinP10P25P75P90SumPattern(this, 'block_size'),
        fullness: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'block_fullness'),
      },
      transactions: {
        firstTxindex: createMetricPattern20(this, 'first_txindex'),
        height: createMetricPattern21(this, 'height'),
        txid: createMetricPattern21(this, 'txid'),
        txversion: createMetricPattern21(this, 'txversion'),
        rawlocktime: createMetricPattern21(this, 'rawlocktime'),
        baseSize: createMetricPattern21(this, 'base_size'),
        totalSize: createMetricPattern21(this, 'total_size'),
        isExplicitlyRbf: createMetricPattern21(this, 'is_explicitly_rbf'),
        firstTxinindex: createMetricPattern21(this, 'first_txinindex'),
        firstTxoutindex: createMetricPattern21(this, 'first_txoutindex'),
        count: {
          txCount: createCumulativeHeightRollingPattern(this, 'tx_count'),
          isCoinbase: createMetricPattern21(this, 'is_coinbase'),
        },
        size: {
          vsize: create_1h24hBlockTxindexPattern(this, 'tx_vsize'),
          weight: create_1h24hBlockTxindexPattern(this, 'tx_weight'),
        },
        fees: {
          inputValue: createMetricPattern21(this, 'input_value'),
          outputValue: createMetricPattern21(this, 'output_value'),
          fee: create_1h24hBlockTxindexPattern(this, 'fee'),
          feeRate: create_1h24hBlockTxindexPattern(this, 'fee_rate'),
        },
        versions: {
          v1: createCumulativeHeightSumPattern(this, 'tx_v1'),
          v2: createCumulativeHeightSumPattern(this, 'tx_v2'),
          v3: createCumulativeHeightSumPattern(this, 'tx_v3'),
        },
        volume: {
          sentSum: createBtcRollingSatsUsdPattern(this, 'sent_sum'),
          receivedSum: createBtcRollingSatsUsdPattern(this, 'received_sum'),
          annualizedVolume: createBtcSatsUsdPattern(this, 'annualized_volume'),
          txPerSec: createMetricPattern1(this, 'tx_per_sec'),
          outputsPerSec: createMetricPattern1(this, 'outputs_per_sec'),
          inputsPerSec: createMetricPattern1(this, 'inputs_per_sec'),
        },
      },
      inputs: {
        firstTxinindex: createMetricPattern20(this, 'first_txinindex'),
        outpoint: createMetricPattern22(this, 'outpoint'),
        txindex: createMetricPattern22(this, 'txindex'),
        outputtype: createMetricPattern22(this, 'outputtype'),
        typeindex: createMetricPattern22(this, 'typeindex'),
        spent: {
          txoutindex: createMetricPattern22(this, 'txoutindex'),
          value: createMetricPattern22(this, 'value'),
        },
        count: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'input_count'),
      },
      outputs: {
        firstTxoutindex: createMetricPattern20(this, 'first_txoutindex'),
        value: createMetricPattern23(this, 'value'),
        outputtype: createMetricPattern23(this, 'outputtype'),
        typeindex: createMetricPattern23(this, 'typeindex'),
        txindex: createMetricPattern23(this, 'txindex'),
        spent: {
          txinindex: createMetricPattern23(this, 'txinindex'),
        },
        count: {
          totalCount: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'output_count'),
          utxoCount: createMetricPattern1(this, 'exact_utxo_count'),
        },
      },
      addresses: {
        firstP2pk65addressindex: createMetricPattern20(this, 'first_p2pk65addressindex'),
        firstP2pk33addressindex: createMetricPattern20(this, 'first_p2pk33addressindex'),
        firstP2pkhaddressindex: createMetricPattern20(this, 'first_p2pkhaddressindex'),
        firstP2shaddressindex: createMetricPattern20(this, 'first_p2shaddressindex'),
        firstP2wpkhaddressindex: createMetricPattern20(this, 'first_p2wpkhaddressindex'),
        firstP2wshaddressindex: createMetricPattern20(this, 'first_p2wshaddressindex'),
        firstP2traddressindex: createMetricPattern20(this, 'first_p2traddressindex'),
        firstP2aaddressindex: createMetricPattern20(this, 'first_p2aaddressindex'),
        p2pk65bytes: createMetricPattern29(this, 'p2pk65bytes'),
        p2pk33bytes: createMetricPattern28(this, 'p2pk33bytes'),
        p2pkhbytes: createMetricPattern30(this, 'p2pkhbytes'),
        p2shbytes: createMetricPattern31(this, 'p2shbytes'),
        p2wpkhbytes: createMetricPattern33(this, 'p2wpkhbytes'),
        p2wshbytes: createMetricPattern34(this, 'p2wshbytes'),
        p2trbytes: createMetricPattern32(this, 'p2trbytes'),
        p2abytes: createMetricPattern26(this, 'p2abytes'),
      },
      scripts: {
        firstEmptyoutputindex: createMetricPattern20(this, 'first_emptyoutputindex'),
        firstOpreturnindex: createMetricPattern20(this, 'first_opreturnindex'),
        firstP2msoutputindex: createMetricPattern20(this, 'first_p2msoutputindex'),
        firstUnknownoutputindex: createMetricPattern20(this, 'first_unknownoutputindex'),
        emptyToTxindex: createMetricPattern24(this, 'txindex'),
        opreturnToTxindex: createMetricPattern25(this, 'txindex'),
        p2msToTxindex: createMetricPattern27(this, 'txindex'),
        unknownToTxindex: createMetricPattern35(this, 'txindex'),
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
          taprootAdoption: createMetricPattern1(this, 'taproot_adoption'),
          segwitAdoption: createMetricPattern1(this, 'segwit_adoption'),
        },
        value: {
          opreturn: createBtcSatsUsdPattern3(this, 'opreturn_value'),
        },
      },
      mining: {
        rewards: {
          coinbase: createBtcSatsUsdPattern3(this, 'coinbase'),
          subsidy: createBtcSatsUsdPattern3(this, 'subsidy'),
          fees: createBtcSatsUsdPattern3(this, 'fees'),
          unclaimedRewards: createBtcSatsUsdPattern4(this, 'unclaimed_rewards'),
          feeDominance: createMetricPattern1(this, 'fee_dominance'),
          feeDominance24h: createMetricPattern1(this, 'fee_dominance_24h'),
          feeDominance7d: createMetricPattern1(this, 'fee_dominance_7d'),
          feeDominance30d: createMetricPattern1(this, 'fee_dominance_30d'),
          feeDominance1y: createMetricPattern1(this, 'fee_dominance_1y'),
          subsidyDominance: createMetricPattern1(this, 'subsidy_dominance'),
          subsidyDominance24h: createMetricPattern1(this, 'subsidy_dominance_24h'),
          subsidyDominance7d: createMetricPattern1(this, 'subsidy_dominance_7d'),
          subsidyDominance30d: createMetricPattern1(this, 'subsidy_dominance_30d'),
          subsidyDominance1y: createMetricPattern1(this, 'subsidy_dominance_1y'),
          subsidyUsd1ySma: createMetricPattern1(this, 'subsidy_usd_1y_sma'),
        },
        hashrate: {
          hashRate: createMetricPattern1(this, 'hash_rate'),
          hashRate1wSma: createMetricPattern1(this, 'hash_rate_1w_sma'),
          hashRate1mSma: createMetricPattern1(this, 'hash_rate_1m_sma'),
          hashRate2mSma: createMetricPattern1(this, 'hash_rate_2m_sma'),
          hashRate1ySma: createMetricPattern1(this, 'hash_rate_1y_sma'),
          hashRateAth: createMetricPattern1(this, 'hash_rate_ath'),
          hashRateDrawdown: createMetricPattern1(this, 'hash_rate_drawdown'),
          hashPriceThs: createMetricPattern1(this, 'hash_price_ths'),
          hashPriceThsMin: createMetricPattern1(this, 'hash_price_ths_min'),
          hashPricePhs: createMetricPattern1(this, 'hash_price_phs'),
          hashPricePhsMin: createMetricPattern1(this, 'hash_price_phs_min'),
          hashPriceRebound: createMetricPattern1(this, 'hash_price_rebound'),
          hashValueThs: createMetricPattern1(this, 'hash_value_ths'),
          hashValueThsMin: createMetricPattern1(this, 'hash_value_ths_min'),
          hashValuePhs: createMetricPattern1(this, 'hash_value_phs'),
          hashValuePhsMin: createMetricPattern1(this, 'hash_value_phs_min'),
          hashValueRebound: createMetricPattern1(this, 'hash_value_rebound'),
        },
      },
      positions: {
        blockPosition: createMetricPattern20(this, 'position'),
        txPosition: createMetricPattern21(this, 'position'),
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
          vaultedSupply: createBtcSatsUsdPattern(this, 'vaulted_supply'),
          activeSupply: createBtcSatsUsdPattern(this, 'active_supply'),
        },
        value: {
          cointimeValueDestroyed: createCumulativeHeightSumPattern(this, 'cointime_value_destroyed'),
          cointimeValueCreated: createCumulativeHeightSumPattern(this, 'cointime_value_created'),
          cointimeValueStored: createCumulativeHeightSumPattern(this, 'cointime_value_stored'),
          vocdd: createCumulativeHeightSumPattern(this, 'vocdd'),
        },
        cap: {
          thermoCap: createMetricPattern1(this, 'thermo_cap'),
          investorCap: createMetricPattern1(this, 'investor_cap'),
          vaultedCap: createMetricPattern1(this, 'vaulted_cap'),
          activeCap: createMetricPattern1(this, 'active_cap'),
          cointimeCap: createMetricPattern1(this, 'cointime_cap'),
        },
        pricing: {
          vaultedPrice: createSatsUsdPattern(this, 'vaulted_price'),
          vaultedPriceRatio: createRatioPattern(this, 'vaulted_price_ratio'),
          activePrice: createSatsUsdPattern(this, 'active_price'),
          activePriceRatio: createRatioPattern(this, 'active_price_ratio'),
          trueMarketMean: createSatsUsdPattern(this, 'true_market_mean'),
          trueMarketMeanRatio: createRatioPattern(this, 'true_market_mean_ratio'),
          cointimePrice: createSatsUsdPattern(this, 'cointime_price'),
          cointimePriceRatio: createRatioPattern(this, 'cointime_price_ratio'),
        },
        adjusted: {
          cointimeAdjInflationRate: createMetricPattern1(this, 'cointime_adj_inflation_rate'),
          cointimeAdjTxBtcVelocity: createMetricPattern1(this, 'cointime_adj_tx_btc_velocity'),
          cointimeAdjTxUsdVelocity: createMetricPattern1(this, 'cointime_adj_tx_usd_velocity'),
        },
        reserveRisk: {
          vocdd365dMedian: createMetricPattern20(this, 'vocdd_365d_median'),
          hodlBank: createMetricPattern20(this, 'hodl_bank'),
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
            identity: createMetricPattern28(this, 'p2pk33addressindex'),
          },
          p2pk65: {
            identity: createMetricPattern29(this, 'p2pk65addressindex'),
          },
          p2pkh: {
            identity: createMetricPattern30(this, 'p2pkhaddressindex'),
          },
          p2sh: {
            identity: createMetricPattern31(this, 'p2shaddressindex'),
          },
          p2tr: {
            identity: createMetricPattern32(this, 'p2traddressindex'),
          },
          p2wpkh: {
            identity: createMetricPattern33(this, 'p2wpkhaddressindex'),
          },
          p2wsh: {
            identity: createMetricPattern34(this, 'p2wshaddressindex'),
          },
          p2a: {
            identity: createMetricPattern26(this, 'p2aaddressindex'),
          },
          p2ms: {
            identity: createMetricPattern27(this, 'p2msoutputindex'),
          },
          empty: {
            identity: createMetricPattern24(this, 'emptyoutputindex'),
          },
          unknown: {
            identity: createMetricPattern35(this, 'unknownoutputindex'),
          },
          opreturn: {
            identity: createMetricPattern25(this, 'opreturnindex'),
          },
        },
        height: {
          identity: createMetricPattern20(this, 'height'),
          minute1: createMetricPattern20(this, 'minute1'),
          minute5: createMetricPattern20(this, 'minute5'),
          minute10: createMetricPattern20(this, 'minute10'),
          minute30: createMetricPattern20(this, 'minute30'),
          hour1: createMetricPattern20(this, 'hour1'),
          hour4: createMetricPattern20(this, 'hour4'),
          hour12: createMetricPattern20(this, 'hour12'),
          day1: createMetricPattern20(this, 'day1'),
          day3: createMetricPattern20(this, 'day3'),
          difficultyepoch: createMetricPattern20(this, 'difficultyepoch'),
          halvingepoch: createMetricPattern20(this, 'halvingepoch'),
          week1: createMetricPattern20(this, 'week1'),
          month1: createMetricPattern20(this, 'month1'),
          month3: createMetricPattern20(this, 'month3'),
          month6: createMetricPattern20(this, 'month6'),
          year1: createMetricPattern20(this, 'year1'),
          year10: createMetricPattern20(this, 'year10'),
          txindexCount: createMetricPattern20(this, 'txindex_count'),
        },
        difficultyepoch: {
          identity: createMetricPattern19(this, 'difficultyepoch'),
          firstHeight: createMetricPattern19(this, 'first_height'),
          heightCount: createMetricPattern19(this, 'height_count'),
        },
        halvingepoch: {
          identity: createMetricPattern18(this, 'halvingepoch'),
          firstHeight: createMetricPattern18(this, 'first_height'),
        },
        minute1: {
          identity: createMetricPattern3(this, 'minute1'),
          firstHeight: createMetricPattern3(this, 'minute1_first_height'),
        },
        minute5: {
          identity: createMetricPattern4(this, 'minute5'),
          firstHeight: createMetricPattern4(this, 'minute5_first_height'),
        },
        minute10: {
          identity: createMetricPattern5(this, 'minute10'),
          firstHeight: createMetricPattern5(this, 'minute10_first_height'),
        },
        minute30: {
          identity: createMetricPattern6(this, 'minute30'),
          firstHeight: createMetricPattern6(this, 'minute30_first_height'),
        },
        hour1: {
          identity: createMetricPattern7(this, 'hour1'),
          firstHeight: createMetricPattern7(this, 'hour1_first_height'),
        },
        hour4: {
          identity: createMetricPattern8(this, 'hour4'),
          firstHeight: createMetricPattern8(this, 'hour4_first_height'),
        },
        hour12: {
          identity: createMetricPattern9(this, 'hour12'),
          firstHeight: createMetricPattern9(this, 'hour12_first_height'),
        },
        day1: {
          identity: createMetricPattern10(this, 'day1'),
          date: createMetricPattern10(this, 'date'),
          firstHeight: createMetricPattern10(this, 'first_height'),
          heightCount: createMetricPattern10(this, 'height_count'),
        },
        day3: {
          identity: createMetricPattern11(this, 'day3'),
          firstHeight: createMetricPattern11(this, 'day3_first_height'),
        },
        week1: {
          identity: createMetricPattern12(this, 'week1'),
          date: createMetricPattern12(this, 'date'),
          firstHeight: createMetricPattern12(this, 'week1_first_height'),
        },
        month1: {
          identity: createMetricPattern13(this, 'month1'),
          date: createMetricPattern13(this, 'date'),
          firstHeight: createMetricPattern13(this, 'month1_first_height'),
        },
        month3: {
          identity: createMetricPattern14(this, 'month3'),
          date: createMetricPattern14(this, 'date'),
          firstHeight: createMetricPattern14(this, 'month3_first_height'),
        },
        month6: {
          identity: createMetricPattern15(this, 'month6'),
          date: createMetricPattern15(this, 'date'),
          firstHeight: createMetricPattern15(this, 'month6_first_height'),
        },
        year1: {
          identity: createMetricPattern16(this, 'year1'),
          date: createMetricPattern16(this, 'date'),
          firstHeight: createMetricPattern16(this, 'year1_first_height'),
        },
        year10: {
          identity: createMetricPattern17(this, 'year10'),
          date: createMetricPattern17(this, 'date'),
          firstHeight: createMetricPattern17(this, 'year10_first_height'),
        },
        txindex: {
          identity: createMetricPattern21(this, 'txindex'),
          inputCount: createMetricPattern21(this, 'input_count'),
          outputCount: createMetricPattern21(this, 'output_count'),
        },
        txinindex: {
          identity: createMetricPattern22(this, 'txinindex'),
        },
        txoutindex: {
          identity: createMetricPattern23(this, 'txoutindex'),
        },
      },
      market: {
        ath: {
          priceAth: createSatsUsdPattern(this, 'price_ath'),
          priceDrawdown: createMetricPattern1(this, 'price_drawdown'),
          daysSincePriceAth: createMetricPattern1(this, 'days_since_price_ath'),
          yearsSincePriceAth: createMetricPattern2(this, 'years_since_price_ath'),
          maxDaysBetweenPriceAths: createMetricPattern1(this, 'max_days_between_price_aths'),
          maxYearsBetweenPriceAths: createMetricPattern2(this, 'max_years_between_price_aths'),
        },
        lookback: {
          _1d: createSatsUsdPattern(this, 'price_1d_ago'),
          _1w: createSatsUsdPattern(this, 'price_1w_ago'),
          _1m: createSatsUsdPattern(this, 'price_1m_ago'),
          _3m: createSatsUsdPattern(this, 'price_3m_ago'),
          _6m: createSatsUsdPattern(this, 'price_6m_ago'),
          _1y: createSatsUsdPattern(this, 'price_1y_ago'),
          _2y: createSatsUsdPattern(this, 'price_2y_ago'),
          _3y: createSatsUsdPattern(this, 'price_3y_ago'),
          _4y: createSatsUsdPattern(this, 'price_4y_ago'),
          _5y: createSatsUsdPattern(this, 'price_5y_ago'),
          _6y: createSatsUsdPattern(this, 'price_6y_ago'),
          _8y: createSatsUsdPattern(this, 'price_8y_ago'),
          _10y: createSatsUsdPattern(this, 'price_10y_ago'),
        },
        returns: {
          priceReturns: {
            _1d: createMetricPattern1(this, '1d_price_returns'),
            _1w: createMetricPattern1(this, '1w_price_returns'),
            _1m: createMetricPattern1(this, '1m_price_returns'),
            _3m: createMetricPattern1(this, '3m_price_returns'),
            _6m: createMetricPattern1(this, '6m_price_returns'),
            _1y: createMetricPattern1(this, '1y_price_returns'),
            _2y: createMetricPattern1(this, '2y_price_returns'),
            _3y: createMetricPattern1(this, '3y_price_returns'),
            _4y: createMetricPattern1(this, '4y_price_returns'),
            _5y: createMetricPattern1(this, '5y_price_returns'),
            _6y: createMetricPattern1(this, '6y_price_returns'),
            _8y: createMetricPattern1(this, '8y_price_returns'),
            _10y: createMetricPattern1(this, '10y_price_returns'),
          },
          cagr: create_10y2y3y4y5y6y8yPattern(this, 'cagr'),
          _1dReturns1wSd: createSdSmaPattern(this, '1d_returns_1w_sd'),
          _1dReturns1mSd: createSdSmaPattern(this, '1d_returns_1m_sd'),
          _1dReturns1ySd: createSdSmaPattern(this, '1d_returns_1y_sd'),
          downsideReturns: createMetricPattern20(this, 'downside_returns'),
          downside1wSd: createSdSmaPattern(this, 'downside_1w_sd'),
          downside1mSd: createSdSmaPattern(this, 'downside_1m_sd'),
          downside1ySd: createSdSmaPattern(this, 'downside_1y_sd'),
        },
        volatility: {
          price1wVolatility: createMetricPattern1(this, 'price_1w_volatility'),
          price1mVolatility: createMetricPattern1(this, 'price_1m_volatility'),
          price1yVolatility: createMetricPattern1(this, 'price_1y_volatility'),
          sharpe1w: createMetricPattern1(this, 'sharpe_1w'),
          sharpe1m: createMetricPattern1(this, 'sharpe_1m'),
          sharpe1y: createMetricPattern1(this, 'sharpe_1y'),
          sortino1w: createMetricPattern1(this, 'sortino_1w'),
          sortino1m: createMetricPattern1(this, 'sortino_1m'),
          sortino1y: createMetricPattern1(this, 'sortino_1y'),
        },
        range: {
          price1wMin: createSatsUsdPattern(this, 'price_1w_min'),
          price1wMax: createSatsUsdPattern(this, 'price_1w_max'),
          price2wMin: createSatsUsdPattern(this, 'price_2w_min'),
          price2wMax: createSatsUsdPattern(this, 'price_2w_max'),
          price1mMin: createSatsUsdPattern(this, 'price_1m_min'),
          price1mMax: createSatsUsdPattern(this, 'price_1m_max'),
          price1yMin: createSatsUsdPattern(this, 'price_1y_min'),
          price1yMax: createSatsUsdPattern(this, 'price_1y_max'),
          priceTrueRange: createMetricPattern1(this, 'price_true_range'),
          priceTrueRange2wSum: createMetricPattern1(this, 'price_true_range_2w_sum'),
          price2wChoppinessIndex: createMetricPattern1(this, 'price_2w_choppiness_index'),
        },
        movingAverage: {
          price1wSma: createPriceRatioPattern(this, 'price_1w_sma'),
          price8dSma: createPriceRatioPattern(this, 'price_8d_sma'),
          price13dSma: createPriceRatioPattern(this, 'price_13d_sma'),
          price21dSma: createPriceRatioPattern(this, 'price_21d_sma'),
          price1mSma: createPriceRatioPattern(this, 'price_1m_sma'),
          price34dSma: createPriceRatioPattern(this, 'price_34d_sma'),
          price55dSma: createPriceRatioPattern(this, 'price_55d_sma'),
          price89dSma: createPriceRatioPattern(this, 'price_89d_sma'),
          price111dSma: createPriceRatioPattern(this, 'price_111d_sma'),
          price144dSma: createPriceRatioPattern(this, 'price_144d_sma'),
          price200dSma: createPriceRatioPattern(this, 'price_200d_sma'),
          price350dSma: createPriceRatioPattern(this, 'price_350d_sma'),
          price1ySma: createPriceRatioPattern(this, 'price_1y_sma'),
          price2ySma: createPriceRatioPattern(this, 'price_2y_sma'),
          price200wSma: createPriceRatioPattern(this, 'price_200w_sma'),
          price4ySma: createPriceRatioPattern(this, 'price_4y_sma'),
          price1wEma: createPriceRatioPattern(this, 'price_1w_ema'),
          price8dEma: createPriceRatioPattern(this, 'price_8d_ema'),
          price12dEma: createPriceRatioPattern(this, 'price_12d_ema'),
          price13dEma: createPriceRatioPattern(this, 'price_13d_ema'),
          price21dEma: createPriceRatioPattern(this, 'price_21d_ema'),
          price26dEma: createPriceRatioPattern(this, 'price_26d_ema'),
          price1mEma: createPriceRatioPattern(this, 'price_1m_ema'),
          price34dEma: createPriceRatioPattern(this, 'price_34d_ema'),
          price55dEma: createPriceRatioPattern(this, 'price_55d_ema'),
          price89dEma: createPriceRatioPattern(this, 'price_89d_ema'),
          price144dEma: createPriceRatioPattern(this, 'price_144d_ema'),
          price200dEma: createPriceRatioPattern(this, 'price_200d_ema'),
          price1yEma: createPriceRatioPattern(this, 'price_1y_ema'),
          price2yEma: createPriceRatioPattern(this, 'price_2y_ema'),
          price200wEma: createPriceRatioPattern(this, 'price_200w_ema'),
          price4yEma: createPriceRatioPattern(this, 'price_4y_ema'),
          price200dSmaX24: createSatsUsdPattern(this, 'price_200d_sma_x2_4'),
          price200dSmaX08: createSatsUsdPattern(this, 'price_200d_sma_x0_8'),
          price350dSmaX2: createSatsUsdPattern(this, 'price_350d_sma_x2'),
        },
        dca: {
          dcaSatsPerDay: createMetricPattern20(this, 'dca_sats_per_day'),
          periodStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'dca_stack'),
          periodAveragePrice: {
            _1w: createSatsUsdPattern(this, '1w_dca_average_price'),
            _1m: createSatsUsdPattern(this, '1m_dca_average_price'),
            _3m: createSatsUsdPattern(this, '3m_dca_average_price'),
            _6m: createSatsUsdPattern(this, '6m_dca_average_price'),
            _1y: createSatsUsdPattern(this, '1y_dca_average_price'),
            _2y: createSatsUsdPattern(this, '2y_dca_average_price'),
            _3y: createSatsUsdPattern(this, '3y_dca_average_price'),
            _4y: createSatsUsdPattern(this, '4y_dca_average_price'),
            _5y: createSatsUsdPattern(this, '5y_dca_average_price'),
            _6y: createSatsUsdPattern(this, '6y_dca_average_price'),
            _8y: createSatsUsdPattern(this, '8y_dca_average_price'),
            _10y: createSatsUsdPattern(this, '10y_dca_average_price'),
          },
          periodReturns: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_returns'),
          periodCagr: create_10y2y3y4y5y6y8yPattern(this, 'dca_cagr'),
          periodDaysInProfit: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_days_in_profit'),
          periodDaysInLoss: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_days_in_loss'),
          periodMinReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_min_return'),
          periodMaxReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_max_return'),
          periodLumpSumStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'lump_sum_stack'),
          periodLumpSumReturns: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_returns'),
          periodLumpSumDaysInProfit: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_days_in_profit'),
          periodLumpSumDaysInLoss: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_days_in_loss'),
          periodLumpSumMinReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_min_return'),
          periodLumpSumMaxReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_max_return'),
          classStack: {
            _2015: createBtcSatsUsdPattern(this, 'dca_class_2015_stack'),
            _2016: createBtcSatsUsdPattern(this, 'dca_class_2016_stack'),
            _2017: createBtcSatsUsdPattern(this, 'dca_class_2017_stack'),
            _2018: createBtcSatsUsdPattern(this, 'dca_class_2018_stack'),
            _2019: createBtcSatsUsdPattern(this, 'dca_class_2019_stack'),
            _2020: createBtcSatsUsdPattern(this, 'dca_class_2020_stack'),
            _2021: createBtcSatsUsdPattern(this, 'dca_class_2021_stack'),
            _2022: createBtcSatsUsdPattern(this, 'dca_class_2022_stack'),
            _2023: createBtcSatsUsdPattern(this, 'dca_class_2023_stack'),
            _2024: createBtcSatsUsdPattern(this, 'dca_class_2024_stack'),
            _2025: createBtcSatsUsdPattern(this, 'dca_class_2025_stack'),
            _2026: createBtcSatsUsdPattern(this, 'dca_class_2026_stack'),
          },
          classAveragePrice: {
            _2015: createSatsUsdPattern(this, 'dca_class_2015_average_price'),
            _2016: createSatsUsdPattern(this, 'dca_class_2016_average_price'),
            _2017: createSatsUsdPattern(this, 'dca_class_2017_average_price'),
            _2018: createSatsUsdPattern(this, 'dca_class_2018_average_price'),
            _2019: createSatsUsdPattern(this, 'dca_class_2019_average_price'),
            _2020: createSatsUsdPattern(this, 'dca_class_2020_average_price'),
            _2021: createSatsUsdPattern(this, 'dca_class_2021_average_price'),
            _2022: createSatsUsdPattern(this, 'dca_class_2022_average_price'),
            _2023: createSatsUsdPattern(this, 'dca_class_2023_average_price'),
            _2024: createSatsUsdPattern(this, 'dca_class_2024_average_price'),
            _2025: createSatsUsdPattern(this, 'dca_class_2025_average_price'),
            _2026: createSatsUsdPattern(this, 'dca_class_2026_average_price'),
          },
          classReturns: create_201520162017201820192020202120222023202420252026Pattern2(this, 'dca_class'),
          classDaysInProfit: {
            _2015: createMetricPattern1(this, 'dca_class_2015_days_in_profit'),
            _2016: createMetricPattern1(this, 'dca_class_2016_days_in_profit'),
            _2017: createMetricPattern1(this, 'dca_class_2017_days_in_profit'),
            _2018: createMetricPattern1(this, 'dca_class_2018_days_in_profit'),
            _2019: createMetricPattern1(this, 'dca_class_2019_days_in_profit'),
            _2020: createMetricPattern1(this, 'dca_class_2020_days_in_profit'),
            _2021: createMetricPattern1(this, 'dca_class_2021_days_in_profit'),
            _2022: createMetricPattern1(this, 'dca_class_2022_days_in_profit'),
            _2023: createMetricPattern1(this, 'dca_class_2023_days_in_profit'),
            _2024: createMetricPattern1(this, 'dca_class_2024_days_in_profit'),
            _2025: createMetricPattern1(this, 'dca_class_2025_days_in_profit'),
            _2026: createMetricPattern1(this, 'dca_class_2026_days_in_profit'),
          },
          classDaysInLoss: {
            _2015: createMetricPattern1(this, 'dca_class_2015_days_in_loss'),
            _2016: createMetricPattern1(this, 'dca_class_2016_days_in_loss'),
            _2017: createMetricPattern1(this, 'dca_class_2017_days_in_loss'),
            _2018: createMetricPattern1(this, 'dca_class_2018_days_in_loss'),
            _2019: createMetricPattern1(this, 'dca_class_2019_days_in_loss'),
            _2020: createMetricPattern1(this, 'dca_class_2020_days_in_loss'),
            _2021: createMetricPattern1(this, 'dca_class_2021_days_in_loss'),
            _2022: createMetricPattern1(this, 'dca_class_2022_days_in_loss'),
            _2023: createMetricPattern1(this, 'dca_class_2023_days_in_loss'),
            _2024: createMetricPattern1(this, 'dca_class_2024_days_in_loss'),
            _2025: createMetricPattern1(this, 'dca_class_2025_days_in_loss'),
            _2026: createMetricPattern1(this, 'dca_class_2026_days_in_loss'),
          },
          classMinReturn: {
            _2015: createMetricPattern1(this, 'dca_class_2015_min_return'),
            _2016: createMetricPattern1(this, 'dca_class_2016_min_return'),
            _2017: createMetricPattern1(this, 'dca_class_2017_min_return'),
            _2018: createMetricPattern1(this, 'dca_class_2018_min_return'),
            _2019: createMetricPattern1(this, 'dca_class_2019_min_return'),
            _2020: createMetricPattern1(this, 'dca_class_2020_min_return'),
            _2021: createMetricPattern1(this, 'dca_class_2021_min_return'),
            _2022: createMetricPattern1(this, 'dca_class_2022_min_return'),
            _2023: createMetricPattern1(this, 'dca_class_2023_min_return'),
            _2024: createMetricPattern1(this, 'dca_class_2024_min_return'),
            _2025: createMetricPattern1(this, 'dca_class_2025_min_return'),
            _2026: createMetricPattern1(this, 'dca_class_2026_min_return'),
          },
          classMaxReturn: {
            _2015: createMetricPattern1(this, 'dca_class_2015_max_return'),
            _2016: createMetricPattern1(this, 'dca_class_2016_max_return'),
            _2017: createMetricPattern1(this, 'dca_class_2017_max_return'),
            _2018: createMetricPattern1(this, 'dca_class_2018_max_return'),
            _2019: createMetricPattern1(this, 'dca_class_2019_max_return'),
            _2020: createMetricPattern1(this, 'dca_class_2020_max_return'),
            _2021: createMetricPattern1(this, 'dca_class_2021_max_return'),
            _2022: createMetricPattern1(this, 'dca_class_2022_max_return'),
            _2023: createMetricPattern1(this, 'dca_class_2023_max_return'),
            _2024: createMetricPattern1(this, 'dca_class_2024_max_return'),
            _2025: createMetricPattern1(this, 'dca_class_2025_max_return'),
            _2026: createMetricPattern1(this, 'dca_class_2026_max_return'),
          },
        },
        indicators: {
          puellMultiple: createMetricPattern1(this, 'puell_multiple'),
          nvt: createMetricPattern1(this, 'nvt'),
          rsi: {
            _1d: {
              gains: createMetricPattern1(this, 'rsi_gains_1d'),
              losses: createMetricPattern1(this, 'rsi_losses_1d'),
              averageGain: createMetricPattern1(this, 'rsi_avg_gain_1d'),
              averageLoss: createMetricPattern1(this, 'rsi_avg_loss_1d'),
              rsi: createMetricPattern1(this, 'rsi_1d'),
              rsiMin: createMetricPattern1(this, 'rsi_rsi_min_1d'),
              rsiMax: createMetricPattern1(this, 'rsi_rsi_max_1d'),
              stochRsi: createMetricPattern1(this, 'rsi_stoch_rsi_1d'),
              stochRsiK: createMetricPattern1(this, 'rsi_stoch_rsi_k_1d'),
              stochRsiD: createMetricPattern1(this, 'rsi_stoch_rsi_d_1d'),
            },
            _1w: {
              gains: createMetricPattern1(this, 'rsi_gains_1w'),
              losses: createMetricPattern1(this, 'rsi_losses_1w'),
              averageGain: createMetricPattern1(this, 'rsi_avg_gain_1w'),
              averageLoss: createMetricPattern1(this, 'rsi_avg_loss_1w'),
              rsi: createMetricPattern1(this, 'rsi_1w'),
              rsiMin: createMetricPattern1(this, 'rsi_rsi_min_1w'),
              rsiMax: createMetricPattern1(this, 'rsi_rsi_max_1w'),
              stochRsi: createMetricPattern1(this, 'rsi_stoch_rsi_1w'),
              stochRsiK: createMetricPattern1(this, 'rsi_stoch_rsi_k_1w'),
              stochRsiD: createMetricPattern1(this, 'rsi_stoch_rsi_d_1w'),
            },
            _1m: {
              gains: createMetricPattern1(this, 'rsi_gains_1m'),
              losses: createMetricPattern1(this, 'rsi_losses_1m'),
              averageGain: createMetricPattern1(this, 'rsi_avg_gain_1m'),
              averageLoss: createMetricPattern1(this, 'rsi_avg_loss_1m'),
              rsi: createMetricPattern1(this, 'rsi_1m'),
              rsiMin: createMetricPattern1(this, 'rsi_rsi_min_1m'),
              rsiMax: createMetricPattern1(this, 'rsi_rsi_max_1m'),
              stochRsi: createMetricPattern1(this, 'rsi_stoch_rsi_1m'),
              stochRsiK: createMetricPattern1(this, 'rsi_stoch_rsi_k_1m'),
              stochRsiD: createMetricPattern1(this, 'rsi_stoch_rsi_d_1m'),
            },
            _1y: createAverageGainsLossesRsiStochPattern(this, 'rsi'),
          },
          stochK: createMetricPattern1(this, 'stoch_k'),
          stochD: createMetricPattern1(this, 'stoch_d'),
          piCycle: createMetricPattern1(this, 'pi_cycle'),
          macd: {
            _1d: {
              line: createMetricPattern1(this, 'macd_line_1d'),
              signal: createMetricPattern1(this, 'macd_signal_1d'),
              histogram: createMetricPattern1(this, 'macd_histogram_1d'),
            },
            _1w: {
              line: createMetricPattern1(this, 'macd_line_1w'),
              signal: createMetricPattern1(this, 'macd_signal_1w'),
              histogram: createMetricPattern1(this, 'macd_histogram_1w'),
            },
            _1m: {
              line: createMetricPattern1(this, 'macd_line_1m'),
              signal: createMetricPattern1(this, 'macd_signal_1m'),
              histogram: createMetricPattern1(this, 'macd_histogram_1m'),
            },
            _1y: createHistogramLineSignalPattern(this, 'macd'),
          },
          gini: createMetricPattern1(this, 'gini'),
        },
      },
      pools: {
        heightToPool: createMetricPattern20(this, 'pool'),
        vecs: {
          unknown: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'unknown'),
          blockfills: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'blockfills'),
          ultimuspool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ultimuspool'),
          terrapool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'terrapool'),
          luxor: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'luxor'),
          onethash: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onethash'),
          btccom: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btccom'),
          bitfarms: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfarms'),
          huobipool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'huobipool'),
          wayicn: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'wayicn'),
          canoepool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'canoepool'),
          btctop: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btctop'),
          bitcoincom: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoincom'),
          pool175btc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pool175btc'),
          gbminers: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'gbminers'),
          axbt: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'axbt'),
          asicminer: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'asicminer'),
          bitminter: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitminter'),
          bitcoinrussia: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinrussia'),
          btcserv: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcserv'),
          simplecoinus: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'simplecoinus'),
          btcguild: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcguild'),
          eligius: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eligius'),
          ozcoin: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ozcoin'),
          eclipsemc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eclipsemc'),
          maxbtc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'maxbtc'),
          triplemining: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'triplemining'),
          coinlab: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'coinlab'),
          pool50btc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pool50btc'),
          ghashio: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ghashio'),
          stminingcorp: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'stminingcorp'),
          bitparking: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitparking'),
          mmpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mmpool'),
          polmine: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'polmine'),
          kncminer: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kncminer'),
          bitalo: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitalo'),
          f2pool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'f2pool'),
          hhtt: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hhtt'),
          megabigpower: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'megabigpower'),
          mtred: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mtred'),
          nmcbit: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nmcbit'),
          yourbtcnet: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'yourbtcnet'),
          givemecoins: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'givemecoins'),
          braiinspool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'braiinspool'),
          antpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'antpool'),
          multicoinco: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'multicoinco'),
          bcpoolio: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bcpoolio'),
          cointerra: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'cointerra'),
          kanopool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kanopool'),
          solock: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'solock'),
          ckpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ckpool'),
          nicehash: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nicehash'),
          bitclub: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitclub'),
          bitcoinaffiliatenetwork: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinaffiliatenetwork'),
          btcc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcc'),
          bwpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bwpool'),
          exxbw: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'exxbw'),
          bitsolo: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitsolo'),
          bitfury: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfury'),
          twentyoneinc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'twentyoneinc'),
          digitalbtc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'digitalbtc'),
          eightbaochi: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eightbaochi'),
          mybtccoinpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mybtccoinpool'),
          tbdice: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tbdice'),
          hashpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hashpool'),
          nexious: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nexious'),
          bravomining: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bravomining'),
          hotpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hotpool'),
          okexpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okexpool'),
          bcmonster: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bcmonster'),
          onehash: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onehash'),
          bixin: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bixin'),
          tatmaspool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tatmaspool'),
          viabtc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'viabtc'),
          connectbtc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'connectbtc'),
          batpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'batpool'),
          waterhole: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'waterhole'),
          dcexploration: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dcexploration'),
          dcex: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dcex'),
          btpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btpool'),
          fiftyeightcoin: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'fiftyeightcoin'),
          bitcoinindia: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinindia'),
          shawnp0wers: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'shawnp0wers'),
          phashio: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'phashio'),
          rigpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'rigpool'),
          haozhuzhu: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'haozhuzhu'),
          sevenpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sevenpool'),
          miningkings: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningkings'),
          hashbx: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hashbx'),
          dpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dpool'),
          rawpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'rawpool'),
          haominer: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'haominer'),
          helix: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'helix'),
          bitcoinukraine: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinukraine'),
          poolin: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'poolin'),
          secretsuperstar: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'secretsuperstar'),
          tigerpoolnet: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tigerpoolnet'),
          sigmapoolcom: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sigmapoolcom'),
          okpooltop: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okpooltop'),
          hummerpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hummerpool'),
          tangpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tangpool'),
          bytepool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bytepool'),
          spiderpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'spiderpool'),
          novablock: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'novablock'),
          miningcity: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningcity'),
          binancepool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'binancepool'),
          minerium: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'minerium'),
          lubiancom: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'lubiancom'),
          okkong: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okkong'),
          aaopool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'aaopool'),
          emcdpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'emcdpool'),
          foundryusa: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'foundryusa'),
          sbicrypto: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sbicrypto'),
          arkpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'arkpool'),
          purebtccom: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'purebtccom'),
          marapool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'marapool'),
          kucoinpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kucoinpool'),
          entrustcharitypool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'entrustcharitypool'),
          okminer: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okminer'),
          titan: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'titan'),
          pegapool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pegapool'),
          btcnuggets: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcnuggets'),
          cloudhashing: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'cloudhashing'),
          digitalxmintsy: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'digitalxmintsy'),
          telco214: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'telco214'),
          btcpoolparty: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcpoolparty'),
          multipool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'multipool'),
          transactioncoinmining: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'transactioncoinmining'),
          btcdig: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcdig'),
          trickysbtcpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'trickysbtcpool'),
          btcmp: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcmp'),
          eobot: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eobot'),
          unomp: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'unomp'),
          patels: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'patels'),
          gogreenlight: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'gogreenlight'),
          bitcoinindiapool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinindiapool'),
          ekanembtc: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ekanembtc'),
          canoe: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'canoe'),
          tiger: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tiger'),
          onem1x: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onem1x'),
          zulupool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'zulupool'),
          secpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'secpool'),
          ocean: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ocean'),
          whitepool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'whitepool'),
          wiz: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'wiz'),
          wk057: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'wk057'),
          futurebitapollosolo: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'futurebitapollosolo'),
          carbonnegative: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'carbonnegative'),
          portlandhodl: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'portlandhodl'),
          phoenix: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'phoenix'),
          neopool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'neopool'),
          maxipool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'maxipool'),
          bitfufupool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfufupool'),
          gdpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'gdpool'),
          miningdutch: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningdutch'),
          publicpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'publicpool'),
          miningsquared: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningsquared'),
          innopolistech: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'innopolistech'),
          btclab: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btclab'),
          parasite: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'parasite'),
          redrockpool: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'redrockpool'),
          est3lar: createBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'est3lar'),
        },
      },
      prices: {
        split: {
          open: createCentsSatsUsdPattern(this, 'price_open'),
          high: createCentsSatsUsdPattern(this, 'price_high'),
          low: createCentsSatsUsdPattern(this, 'price_low'),
          close: {
            cents: createMetricPattern2(this, 'price_close_cents'),
            usd: createMetricPattern2(this, 'price_close'),
            sats: createMetricPattern2(this, 'price_close_sats'),
          },
        },
        ohlc: createCentsSatsUsdPattern(this, 'price_ohlc'),
        price: {
          cents: createMetricPattern20(this, 'price_cents'),
          usd: createMetricPattern20(this, 'price'),
          sats: createMetricPattern20(this, 'price_sats'),
        },
      },
      distribution: {
        supplyState: createMetricPattern20(this, 'supply_state'),
        anyAddressIndexes: {
          p2a: createMetricPattern26(this, 'anyaddressindex'),
          p2pk33: createMetricPattern28(this, 'anyaddressindex'),
          p2pk65: createMetricPattern29(this, 'anyaddressindex'),
          p2pkh: createMetricPattern30(this, 'anyaddressindex'),
          p2sh: createMetricPattern31(this, 'anyaddressindex'),
          p2tr: createMetricPattern32(this, 'anyaddressindex'),
          p2wpkh: createMetricPattern33(this, 'anyaddressindex'),
          p2wsh: createMetricPattern34(this, 'anyaddressindex'),
        },
        addressesData: {
          funded: createMetricPattern36(this, 'fundedaddressdata'),
          empty: createMetricPattern37(this, 'emptyaddressdata'),
        },
        utxoCohorts: {
          all: {
            supply: create_30dHalvedTotalPattern(this, ''),
            outputs: createUtxoPattern(this, 'utxo_count'),
            activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(this, ''),
            realized: createAdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(this, ''),
            costBasis: createInvestedMaxMinPercentilesSpotPattern(this, ''),
            unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(this, ''),
            relative: {
              supplyInProfitRelToOwnSupply: createMetricPattern1(this, 'supply_in_profit_rel_to_own_supply'),
              supplyInLossRelToOwnSupply: createMetricPattern1(this, 'supply_in_loss_rel_to_own_supply'),
              unrealizedProfitRelToMarketCap: createMetricPattern1(this, 'unrealized_profit_rel_to_market_cap'),
              unrealizedLossRelToMarketCap: createMetricPattern1(this, 'unrealized_loss_rel_to_market_cap'),
              negUnrealizedLossRelToMarketCap: createMetricPattern1(this, 'neg_unrealized_loss_rel_to_market_cap'),
              netUnrealizedPnlRelToMarketCap: createMetricPattern1(this, 'net_unrealized_pnl_rel_to_market_cap'),
              nupl: createMetricPattern1(this, 'nupl'),
              investedCapitalInProfitPct: createMetricPattern1(this, 'invested_capital_in_profit_pct'),
              investedCapitalInLossPct: createMetricPattern1(this, 'invested_capital_in_loss_pct'),
              unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_profit_rel_to_own_total_unrealized_pnl'),
              unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_loss_rel_to_own_total_unrealized_pnl'),
              negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
              netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
              unrealizedPeakRegretRelToMarketCap: createMetricPattern1(this, 'unrealized_peak_regret_rel_to_market_cap'),
            },
          },
          sth: {
            supply: create_30dHalvedTotalPattern(this, 'sth'),
            outputs: createUtxoPattern(this, 'sth_utxo_count'),
            activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(this, 'sth'),
            realized: createAdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(this, 'sth'),
            costBasis: createInvestedMaxMinPercentilesSpotPattern(this, 'sth'),
            unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(this, 'sth'),
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
            _1d: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_1d_old'),
            _1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_1w_old'),
            _1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_1m_old'),
            _2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_2m_old'),
            _3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_3m_old'),
            _4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_4m_old'),
            _5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_5m_old'),
            _6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_6m_old'),
            _1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_1y_old'),
            _2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_2y_old'),
            _3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_3y_old'),
            _4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_4y_old'),
            _5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_5y_old'),
            _6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_6y_old'),
            _7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_7y_old'),
            _8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_8y_old'),
            _10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_10y_old'),
            _12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_over_12y_old'),
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
          year: {
            _2009: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2009'),
            _2010: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2010'),
            _2011: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2011'),
            _2012: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2012'),
            _2013: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2013'),
            _2014: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2014'),
            _2015: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2015'),
            _2016: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2016'),
            _2017: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2017'),
            _2018: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2018'),
            _2019: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2019'),
            _2020: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2020'),
            _2021: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2021'),
            _2022: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2022'),
            _2023: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2023'),
            _2024: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2024'),
            _2025: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2025'),
            _2026: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'year_2026'),
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
          all: createBaseRestPattern(this, 'new_addr_count'),
          p2pk65: createBaseRestPattern(this, 'p2pk65_new_addr_count'),
          p2pk33: createBaseRestPattern(this, 'p2pk33_new_addr_count'),
          p2pkh: createBaseRestPattern(this, 'p2pkh_new_addr_count'),
          p2sh: createBaseRestPattern(this, 'p2sh_new_addr_count'),
          p2wpkh: createBaseRestPattern(this, 'p2wpkh_new_addr_count'),
          p2wsh: createBaseRestPattern(this, 'p2wsh_new_addr_count'),
          p2tr: createBaseRestPattern(this, 'p2tr_new_addr_count'),
          p2a: createBaseRestPattern(this, 'p2a_new_addr_count'),
        },
        growthRate: {
          all: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'growth_rate'),
          p2pk65: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2pk65_growth_rate'),
          p2pk33: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2pk33_growth_rate'),
          p2pkh: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2pkh_growth_rate'),
          p2sh: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2sh_growth_rate'),
          p2wpkh: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2wpkh_growth_rate'),
          p2wsh: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2wsh_growth_rate'),
          p2tr: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2tr_growth_rate'),
          p2a: createAverageHeightMaxMedianMinP10P25P75P90Pattern(this, 'p2a_growth_rate'),
        },
        fundedaddressindex: createMetricPattern36(this, 'fundedaddressindex'),
        emptyaddressindex: createMetricPattern37(this, 'emptyaddressindex'),
      },
      supply: {
        circulating: createBtcSatsUsdPattern(this, 'circulating_supply'),
        burned: {
          opreturn: createBtcSatsUsdPattern4(this, 'opreturn_supply'),
          unspendable: createBtcSatsUsdPattern4(this, 'unspendable_supply'),
        },
        inflation: createMetricPattern1(this, 'inflation_rate'),
        velocity: {
          btc: createMetricPattern1(this, 'btc_velocity'),
          usd: createMetricPattern1(this, 'usd_velocity'),
        },
        marketCap: createMetricPattern1(this, 'market_cap'),
        marketCapGrowthRate: createMetricPattern1(this, 'market_cap_growth_rate'),
        realizedCapGrowthRate: createMetricPattern1(this, 'realized_cap_growth_rate'),
        capGrowthRateDiff: createMetricPattern1(this, 'cap_growth_rate_diff'),
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
   * @param {number=} [start] - Inclusive starting index, if negative counts from end
   * @param {number=} [end] - Exclusive ending index, if negative counts from end
   * @param {string=} [limit] - Maximum number of values to return (ignored if `end` is set)
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
   * @param {number=} [start] - Inclusive starting index, if negative counts from end
   * @param {number=} [end] - Exclusive ending index, if negative counts from end
   * @param {string=} [limit] - Maximum number of values to return (ignored if `end` is set)
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
