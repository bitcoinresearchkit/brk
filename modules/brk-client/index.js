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
 * Unified index for any address type (loaded or empty)
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
/** @typedef {number} Cents */
/**
 * Closing price value for a time period
 *
 * @typedef {Cents} Close
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
/** @typedef {number} DateIndex */
/** @typedef {number} DecadeIndex */
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
/**
 * Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
 * block-based (height, txindex), and address/output type indexes.
 *
 * @typedef {("dateindex"|"decadeindex"|"difficultyepoch"|"emptyoutputindex"|"halvingepoch"|"height"|"txinindex"|"monthindex"|"opreturnindex"|"txoutindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"quarterindex"|"semesterindex"|"txindex"|"unknownoutputindex"|"weekindex"|"yearindex"|"loadedaddressindex"|"emptyaddressindex"|"pairoutputindex")} Index
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
 * Data for a loaded (non-empty) address with current balance
 *
 * @typedef {Object} LoadedAddressData
 * @property {number} txCount - Total transaction count
 * @property {number} fundedTxoCount - Number of transaction outputs funded to this address
 * @property {number} spentTxoCount - Number of transaction outputs spent by this address
 * @property {Sats} received - Satoshis received by this address
 * @property {Sats} sent - Satoshis sent by this address
 * @property {Dollars} realizedCap - The realized capitalization of this address
 */
/** @typedef {TypeIndex} LoadedAddressIndex */
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
/** @typedef {number} MonthIndex */
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
/** @typedef {number[]} OracleBins */
/** @typedef {number[]} OracleBinsV2 */
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
 * Index for 2-output transactions (oracle pair candidates)
 *
 * This indexes all transactions with exactly 2 outputs, which are
 * candidates for the UTXOracle algorithm (payment + change pattern).
 *
 * @typedef {number} PairOutputIndex
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
/** @typedef {("unknown"|"blockfills"|"ultimuspool"|"terrapool"|"luxor"|"onethash"|"btccom"|"bitfarms"|"huobipool"|"wayicn"|"canoepool"|"btctop"|"bitcoincom"|"pool175btc"|"gbminers"|"axbt"|"asicminer"|"bitminter"|"bitcoinrussia"|"btcserv"|"simplecoinus"|"btcguild"|"eligius"|"ozcoin"|"eclipsemc"|"maxbtc"|"triplemining"|"coinlab"|"pool50btc"|"ghashio"|"stminingcorp"|"bitparking"|"mmpool"|"polmine"|"kncminer"|"bitalo"|"f2pool"|"hhtt"|"megabigpower"|"mtred"|"nmcbit"|"yourbtcnet"|"givemecoins"|"braiinspool"|"antpool"|"multicoinco"|"bcpoolio"|"cointerra"|"kanopool"|"solock"|"ckpool"|"nicehash"|"bitclub"|"bitcoinaffiliatenetwork"|"btcc"|"bwpool"|"exxbw"|"bitsolo"|"bitfury"|"twentyoneinc"|"digitalbtc"|"eightbaochi"|"mybtccoinpool"|"tbdice"|"hashpool"|"nexious"|"bravomining"|"hotpool"|"okexpool"|"bcmonster"|"onehash"|"bixin"|"tatmaspool"|"viabtc"|"connectbtc"|"batpool"|"waterhole"|"dcexploration"|"dcex"|"btpool"|"fiftyeightcoin"|"bitcoinindia"|"shawnp0wers"|"phashio"|"rigpool"|"haozhuzhu"|"sevenpool"|"miningkings"|"hashbx"|"dpool"|"rawpool"|"haominer"|"helix"|"bitcoinukraine"|"poolin"|"secretsuperstar"|"tigerpoolnet"|"sigmapoolcom"|"okpooltop"|"hummerpool"|"tangpool"|"bytepool"|"spiderpool"|"novablock"|"miningcity"|"binancepool"|"minerium"|"lubiancom"|"okkong"|"aaopool"|"emcdpool"|"foundryusa"|"sbicrypto"|"arkpool"|"purebtccom"|"marapool"|"kucoinpool"|"entrustcharitypool"|"okminer"|"titan"|"pegapool"|"btcnuggets"|"cloudhashing"|"digitalxmintsy"|"telco214"|"btcpoolparty"|"multipool"|"transactioncoinmining"|"btcdig"|"trickysbtcpool"|"btcmp"|"eobot"|"unomp"|"patels"|"gogreenlight"|"ekanembtc"|"canoe"|"tiger"|"onem1x"|"zulupool"|"secpool"|"ocean"|"whitepool"|"wk057"|"futurebitapollosolo"|"carbonnegative"|"portlandhodl"|"phoenix"|"neopool"|"maxipool"|"bitfufupool"|"luckypool"|"miningdutch"|"publicpool"|"miningsquared"|"innopolistech"|"btclab"|"parasite")} PoolSlug */
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
/** @typedef {number} QuarterIndex */
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
/** @typedef {number} SemesterIndex */
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
 * Input index in the spending transaction
 *
 * @typedef {number} Vin
 */
/**
 * Index of the output being spent in the previous transaction
 *
 * @typedef {number} Vout
 */
/** @typedef {number} WeekIndex */
/**
 * Transaction or block weight in weight units (WU)
 *
 * @typedef {number} Weight
 */
/** @typedef {number} YearIndex */

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

/**
 * @template T
 * @typedef {Object} MetricData
 * @property {number} version - Version of the metric data
 * @property {number} total - Total number of data points
 * @property {number} start - Start index (inclusive)
 * @property {number} end - End index (exclusive)
 * @property {string} stamp - ISO 8601 timestamp of when the response was generated
 * @property {T[]} data - The metric data
 */
/** @typedef {MetricData<any>} AnyMetricData */

/**
 * Thenable interface for await support.
 * @template T
 * @typedef {(onfulfilled?: (value: MetricData<T>) => MetricData<T>, onrejected?: (reason: Error) => never) => Promise<MetricData<T>>} Thenable
 */

/**
 * Metric endpoint builder. Callable (returns itself) so both .by.dateindex and .by.dateindex() work.
 * @template T
 * @typedef {Object} MetricEndpointBuilder
 * @property {(index: number) => SingleItemBuilder<T>} get - Get single item at index
 * @property {(start?: number, end?: number) => RangeBuilder<T>} slice - Slice like Array.slice
 * @property {(n: number) => RangeBuilder<T>} first - Get first n items
 * @property {(n: number) => RangeBuilder<T>} last - Get last n items
 * @property {(n: number) => SkippedBuilder<T>} skip - Skip first n items, chain with take()
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch all data
 * @property {() => Promise<string>} fetchCsv - Fetch all data as CSV
 * @property {Thenable<T>} then - Thenable (await endpoint)
 * @property {string} path - The endpoint path
 */
/** @typedef {MetricEndpointBuilder<any>} AnyMetricEndpointBuilder */

/**
 * @template T
 * @typedef {Object} SingleItemBuilder
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch the item
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/**
 * @template T
 * @typedef {Object} SkippedBuilder
 * @property {(n: number) => RangeBuilder<T>} take - Take n items after skipped position
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch from skipped position to end
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/**
 * @template T
 * @typedef {Object} RangeBuilder
 * @property {(onUpdate?: (value: MetricData<T>) => void) => Promise<MetricData<T>>} fetch - Fetch the range
 * @property {() => Promise<string>} fetchCsv - Fetch as CSV
 * @property {Thenable<T>} then - Thenable
 */

/**
 * @template T
 * @typedef {Object} MetricPattern
 * @property {string} name - The metric name
 * @property {Readonly<Partial<Record<Index, MetricEndpointBuilder<T>>>>} by - Index endpoints as lazy getters. Access via .by.dateindex or .by['dateindex']
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
 * @returns {MetricEndpointBuilder<T>}
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
   * @returns {RangeBuilder<T>}
   */
  const rangeBuilder = (start, end) => ({
    fetch(onUpdate) { return client.getJson(buildPath(start, end), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, end, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} index
   * @returns {SingleItemBuilder<T>}
   */
  const singleItemBuilder = (index) => ({
    fetch(onUpdate) { return client.getJson(buildPath(index, index + 1), onUpdate); },
    fetchCsv() { return client.getText(buildPath(index, index + 1, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} start
   * @returns {SkippedBuilder<T>}
   */
  const skippedBuilder = (start) => ({
    take(n) { return rangeBuilder(start, start + n); },
    fetch(onUpdate) { return client.getJson(buildPath(start, undefined), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, undefined, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /** @type {MetricEndpointBuilder<T>} */
  const endpoint = {
    get(index) { return singleItemBuilder(index); },
    slice(start, end) { return rangeBuilder(start, end); },
    first(n) { return rangeBuilder(undefined, n); },
    last(n) { return n === 0 ? rangeBuilder(undefined, 0) : rangeBuilder(-n, undefined); },
    skip(n) { return skippedBuilder(n); },
    fetch(onUpdate) { return client.getJson(buildPath(), onUpdate); },
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
    this.baseUrl = isString ? options : options.baseUrl;
    this.timeout = isString ? 5000 : (options.timeout ?? 5000);
    /** @type {Promise<Cache | null>} */
    this._cachePromise = _openCache(isString ? undefined : options.cache);
  }

  /**
   * @param {string} path
   * @returns {Promise<Response>}
   */
  async get(path) {
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${base}${path}`;
    const res = await fetch(url, { signal: AbortSignal.timeout(this.timeout) });
    if (!res.ok) throw new BrkError(`HTTP ${res.status}: ${url}`, res.status);
    return res;
  }

  /**
   * Make a GET request with stale-while-revalidate caching
   * @template T
   * @param {string} path
   * @param {(value: T) => void} [onUpdate] - Called when data is available
   * @returns {Promise<T>}
   */
  async getJson(path, onUpdate) {
    const base = this.baseUrl.endsWith('/') ? this.baseUrl.slice(0, -1) : this.baseUrl;
    const url = `${base}${path}`;
    const cache = await this._cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (globalThis.navigator?.onLine === false) {
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data available');
    }

    try {
      const res = await this.get(path);
      if (cachedRes?.headers.get('ETag') === res.headers.get('ETag')) return cachedJson;

      const cloned = res.clone();
      const json = await res.json();
      onUpdate?.(json);
      if (cache) _runIdle(() => cache.put(url, cloned));
      return json;
    } catch (e) {
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

const _i1 = /** @type {const} */ (["dateindex", "decadeindex", "difficultyepoch", "height", "monthindex", "quarterindex", "semesterindex", "weekindex", "yearindex"]);
const _i2 = /** @type {const} */ (["dateindex", "decadeindex", "difficultyepoch", "monthindex", "quarterindex", "semesterindex", "weekindex", "yearindex"]);
const _i3 = /** @type {const} */ (["dateindex", "decadeindex", "height", "monthindex", "quarterindex", "semesterindex", "weekindex", "yearindex"]);
const _i4 = /** @type {const} */ (["dateindex", "decadeindex", "monthindex", "quarterindex", "semesterindex", "weekindex", "yearindex"]);
const _i5 = /** @type {const} */ (["dateindex", "height"]);
const _i6 = /** @type {const} */ (["dateindex"]);
const _i7 = /** @type {const} */ (["decadeindex"]);
const _i8 = /** @type {const} */ (["difficultyepoch"]);
const _i9 = /** @type {const} */ (["emptyoutputindex"]);
const _i10 = /** @type {const} */ (["halvingepoch"]);
const _i11 = /** @type {const} */ (["height"]);
const _i12 = /** @type {const} */ (["txinindex"]);
const _i13 = /** @type {const} */ (["monthindex"]);
const _i14 = /** @type {const} */ (["opreturnindex"]);
const _i15 = /** @type {const} */ (["txoutindex"]);
const _i16 = /** @type {const} */ (["p2aaddressindex"]);
const _i17 = /** @type {const} */ (["p2msoutputindex"]);
const _i18 = /** @type {const} */ (["p2pk33addressindex"]);
const _i19 = /** @type {const} */ (["p2pk65addressindex"]);
const _i20 = /** @type {const} */ (["p2pkhaddressindex"]);
const _i21 = /** @type {const} */ (["p2shaddressindex"]);
const _i22 = /** @type {const} */ (["p2traddressindex"]);
const _i23 = /** @type {const} */ (["p2wpkhaddressindex"]);
const _i24 = /** @type {const} */ (["p2wshaddressindex"]);
const _i25 = /** @type {const} */ (["quarterindex"]);
const _i26 = /** @type {const} */ (["semesterindex"]);
const _i27 = /** @type {const} */ (["txindex"]);
const _i28 = /** @type {const} */ (["unknownoutputindex"]);
const _i29 = /** @type {const} */ (["weekindex"]);
const _i30 = /** @type {const} */ (["yearindex"]);
const _i31 = /** @type {const} */ (["loadedaddressindex"]);
const _i32 = /** @type {const} */ (["emptyaddressindex"]);
const _i33 = /** @type {const} */ (["pairoutputindex"]);

/**
 * Generic metric pattern factory.
 * @template T
 * @param {BrkClientBase} client
 * @param {string} name - The metric vec name
 * @param {readonly Index[]} indexes - The supported indexes
 */
function _mp(client, name, indexes) {
  const by = /** @type {any} */ ({});
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
    indexes() { return indexes; },
    /** @param {Index} index */
    get(index) { return indexes.includes(index) ? _endpoint(client, name, index) : undefined; }
  };
}

/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T>, readonly decadeindex: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T>, readonly monthindex: MetricEndpointBuilder<T>, readonly quarterindex: MetricEndpointBuilder<T>, readonly semesterindex: MetricEndpointBuilder<T>, readonly weekindex: MetricEndpointBuilder<T>, readonly yearindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern1 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern1<T>} */
function createMetricPattern1(client, name) { return _mp(client, name, _i1); }
/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T>, readonly decadeindex: MetricEndpointBuilder<T>, readonly difficultyepoch: MetricEndpointBuilder<T>, readonly monthindex: MetricEndpointBuilder<T>, readonly quarterindex: MetricEndpointBuilder<T>, readonly semesterindex: MetricEndpointBuilder<T>, readonly weekindex: MetricEndpointBuilder<T>, readonly yearindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern2 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern2<T>} */
function createMetricPattern2(client, name) { return _mp(client, name, _i2); }
/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T>, readonly decadeindex: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T>, readonly monthindex: MetricEndpointBuilder<T>, readonly quarterindex: MetricEndpointBuilder<T>, readonly semesterindex: MetricEndpointBuilder<T>, readonly weekindex: MetricEndpointBuilder<T>, readonly yearindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern3 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern3<T>} */
function createMetricPattern3(client, name) { return _mp(client, name, _i3); }
/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T>, readonly decadeindex: MetricEndpointBuilder<T>, readonly monthindex: MetricEndpointBuilder<T>, readonly quarterindex: MetricEndpointBuilder<T>, readonly semesterindex: MetricEndpointBuilder<T>, readonly weekindex: MetricEndpointBuilder<T>, readonly yearindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern4 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern4<T>} */
function createMetricPattern4(client, name) { return _mp(client, name, _i4); }
/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern5 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern5<T>} */
function createMetricPattern5(client, name) { return _mp(client, name, _i5); }
/** @template T @typedef {{ name: string, by: { readonly dateindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern6 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern6<T>} */
function createMetricPattern6(client, name) { return _mp(client, name, _i6); }
/** @template T @typedef {{ name: string, by: { readonly decadeindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern7 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern7<T>} */
function createMetricPattern7(client, name) { return _mp(client, name, _i7); }
/** @template T @typedef {{ name: string, by: { readonly difficultyepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern8 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern8<T>} */
function createMetricPattern8(client, name) { return _mp(client, name, _i8); }
/** @template T @typedef {{ name: string, by: { readonly emptyoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern9 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern9<T>} */
function createMetricPattern9(client, name) { return _mp(client, name, _i9); }
/** @template T @typedef {{ name: string, by: { readonly halvingepoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern10 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern10<T>} */
function createMetricPattern10(client, name) { return _mp(client, name, _i10); }
/** @template T @typedef {{ name: string, by: { readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern11 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern11<T>} */
function createMetricPattern11(client, name) { return _mp(client, name, _i11); }
/** @template T @typedef {{ name: string, by: { readonly txinindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern12 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern12<T>} */
function createMetricPattern12(client, name) { return _mp(client, name, _i12); }
/** @template T @typedef {{ name: string, by: { readonly monthindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern13 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern13<T>} */
function createMetricPattern13(client, name) { return _mp(client, name, _i13); }
/** @template T @typedef {{ name: string, by: { readonly opreturnindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern14 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern14<T>} */
function createMetricPattern14(client, name) { return _mp(client, name, _i14); }
/** @template T @typedef {{ name: string, by: { readonly txoutindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern15 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern15<T>} */
function createMetricPattern15(client, name) { return _mp(client, name, _i15); }
/** @template T @typedef {{ name: string, by: { readonly p2aaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern16 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern16<T>} */
function createMetricPattern16(client, name) { return _mp(client, name, _i16); }
/** @template T @typedef {{ name: string, by: { readonly p2msoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern17 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern17<T>} */
function createMetricPattern17(client, name) { return _mp(client, name, _i17); }
/** @template T @typedef {{ name: string, by: { readonly p2pk33addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern18 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern18<T>} */
function createMetricPattern18(client, name) { return _mp(client, name, _i18); }
/** @template T @typedef {{ name: string, by: { readonly p2pk65addressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern19 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern19<T>} */
function createMetricPattern19(client, name) { return _mp(client, name, _i19); }
/** @template T @typedef {{ name: string, by: { readonly p2pkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern20 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern20<T>} */
function createMetricPattern20(client, name) { return _mp(client, name, _i20); }
/** @template T @typedef {{ name: string, by: { readonly p2shaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern21 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern21<T>} */
function createMetricPattern21(client, name) { return _mp(client, name, _i21); }
/** @template T @typedef {{ name: string, by: { readonly p2traddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern22 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern22<T>} */
function createMetricPattern22(client, name) { return _mp(client, name, _i22); }
/** @template T @typedef {{ name: string, by: { readonly p2wpkhaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern23 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern23<T>} */
function createMetricPattern23(client, name) { return _mp(client, name, _i23); }
/** @template T @typedef {{ name: string, by: { readonly p2wshaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern24 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern24<T>} */
function createMetricPattern24(client, name) { return _mp(client, name, _i24); }
/** @template T @typedef {{ name: string, by: { readonly quarterindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern25 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern25<T>} */
function createMetricPattern25(client, name) { return _mp(client, name, _i25); }
/** @template T @typedef {{ name: string, by: { readonly semesterindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern26 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern26<T>} */
function createMetricPattern26(client, name) { return _mp(client, name, _i26); }
/** @template T @typedef {{ name: string, by: { readonly txindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern27 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern27<T>} */
function createMetricPattern27(client, name) { return _mp(client, name, _i27); }
/** @template T @typedef {{ name: string, by: { readonly unknownoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern28 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern28<T>} */
function createMetricPattern28(client, name) { return _mp(client, name, _i28); }
/** @template T @typedef {{ name: string, by: { readonly weekindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern29 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern29<T>} */
function createMetricPattern29(client, name) { return _mp(client, name, _i29); }
/** @template T @typedef {{ name: string, by: { readonly yearindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern30 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern30<T>} */
function createMetricPattern30(client, name) { return _mp(client, name, _i30); }
/** @template T @typedef {{ name: string, by: { readonly loadedaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern31 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern31<T>} */
function createMetricPattern31(client, name) { return _mp(client, name, _i31); }
/** @template T @typedef {{ name: string, by: { readonly emptyaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern32 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern32<T>} */
function createMetricPattern32(client, name) { return _mp(client, name, _i32); }
/** @template T @typedef {{ name: string, by: { readonly pairoutputindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern33 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern33<T>} */
function createMetricPattern33(client, name) { return _mp(client, name, _i33); }

// Reusable structural pattern factories

/**
 * @typedef {Object} RealizedPattern3
 * @property {MetricPattern6<StoredF64>} adjustedSopr
 * @property {MetricPattern6<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern6<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {BitcoinPattern2<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern6<StoredF64>} realizedProfitToLossRatio
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern3}
 */
function createRealizedPattern3(client, acc) {
  return {
    adjustedSopr: createMetricPattern6(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBitcoinPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} RealizedPattern4
 * @property {MetricPattern6<StoredF64>} adjustedSopr
 * @property {MetricPattern6<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern6<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {BitcoinPattern2<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern4}
 */
function createRealizedPattern4(client, acc) {
  return {
    adjustedSopr: createMetricPattern6(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBitcoinPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} Ratio1ySdPattern
 * @property {MetricPattern4<Dollars>} _0sdUsd
 * @property {MetricPattern4<StoredF32>} m05sd
 * @property {MetricPattern4<Dollars>} m05sdUsd
 * @property {MetricPattern4<StoredF32>} m15sd
 * @property {MetricPattern4<Dollars>} m15sdUsd
 * @property {MetricPattern4<StoredF32>} m1sd
 * @property {MetricPattern4<Dollars>} m1sdUsd
 * @property {MetricPattern4<StoredF32>} m25sd
 * @property {MetricPattern4<Dollars>} m25sdUsd
 * @property {MetricPattern4<StoredF32>} m2sd
 * @property {MetricPattern4<Dollars>} m2sdUsd
 * @property {MetricPattern4<StoredF32>} m3sd
 * @property {MetricPattern4<Dollars>} m3sdUsd
 * @property {MetricPattern4<StoredF32>} p05sd
 * @property {MetricPattern4<Dollars>} p05sdUsd
 * @property {MetricPattern4<StoredF32>} p15sd
 * @property {MetricPattern4<Dollars>} p15sdUsd
 * @property {MetricPattern4<StoredF32>} p1sd
 * @property {MetricPattern4<Dollars>} p1sdUsd
 * @property {MetricPattern4<StoredF32>} p25sd
 * @property {MetricPattern4<Dollars>} p25sdUsd
 * @property {MetricPattern4<StoredF32>} p2sd
 * @property {MetricPattern4<Dollars>} p2sdUsd
 * @property {MetricPattern4<StoredF32>} p3sd
 * @property {MetricPattern4<Dollars>} p3sdUsd
 * @property {MetricPattern4<StoredF32>} sd
 * @property {MetricPattern4<StoredF32>} sma
 * @property {MetricPattern4<StoredF32>} zscore
 */

/**
 * Create a Ratio1ySdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Ratio1ySdPattern}
 */
function createRatio1ySdPattern(client, acc) {
  return {
    _0sdUsd: createMetricPattern4(client, _m(acc, '0sd_usd')),
    m05sd: createMetricPattern4(client, _m(acc, 'm0_5sd')),
    m05sdUsd: createMetricPattern4(client, _m(acc, 'm0_5sd_usd')),
    m15sd: createMetricPattern4(client, _m(acc, 'm1_5sd')),
    m15sdUsd: createMetricPattern4(client, _m(acc, 'm1_5sd_usd')),
    m1sd: createMetricPattern4(client, _m(acc, 'm1sd')),
    m1sdUsd: createMetricPattern4(client, _m(acc, 'm1sd_usd')),
    m25sd: createMetricPattern4(client, _m(acc, 'm2_5sd')),
    m25sdUsd: createMetricPattern4(client, _m(acc, 'm2_5sd_usd')),
    m2sd: createMetricPattern4(client, _m(acc, 'm2sd')),
    m2sdUsd: createMetricPattern4(client, _m(acc, 'm2sd_usd')),
    m3sd: createMetricPattern4(client, _m(acc, 'm3sd')),
    m3sdUsd: createMetricPattern4(client, _m(acc, 'm3sd_usd')),
    p05sd: createMetricPattern4(client, _m(acc, 'p0_5sd')),
    p05sdUsd: createMetricPattern4(client, _m(acc, 'p0_5sd_usd')),
    p15sd: createMetricPattern4(client, _m(acc, 'p1_5sd')),
    p15sdUsd: createMetricPattern4(client, _m(acc, 'p1_5sd_usd')),
    p1sd: createMetricPattern4(client, _m(acc, 'p1sd')),
    p1sdUsd: createMetricPattern4(client, _m(acc, 'p1sd_usd')),
    p25sd: createMetricPattern4(client, _m(acc, 'p2_5sd')),
    p25sdUsd: createMetricPattern4(client, _m(acc, 'p2_5sd_usd')),
    p2sd: createMetricPattern4(client, _m(acc, 'p2sd')),
    p2sdUsd: createMetricPattern4(client, _m(acc, 'p2sd_usd')),
    p3sd: createMetricPattern4(client, _m(acc, 'p3sd')),
    p3sdUsd: createMetricPattern4(client, _m(acc, 'p3sd_usd')),
    sd: createMetricPattern4(client, _m(acc, 'sd')),
    sma: createMetricPattern4(client, _m(acc, 'sma')),
    zscore: createMetricPattern4(client, _m(acc, 'zscore')),
  };
}

/**
 * @typedef {Object} RealizedPattern2
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {BitcoinPattern2<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern6<StoredF64>} realizedProfitToLossRatio
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern2}
 */
function createRealizedPattern2(client, acc) {
  return {
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBitcoinPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} RealizedPattern
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {BitcoinPattern2<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {BlockCountPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {BlockCountPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {BlockCountPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPattern}
 */
function createRealizedPattern(client, acc) {
  return {
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createBitcoinPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createBlockCountPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedLoss: createBlockCountPattern(client, _m(acc, 'realized_loss')),
    realizedLossRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createMetricPattern1(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createBlockCountPattern(client, _m(acc, 'realized_profit')),
    realizedProfitRelToRealizedCap: createBlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} Price111dSmaPattern
 * @property {MetricPattern4<Dollars>} price
 * @property {MetricPattern4<StoredF32>} ratio
 * @property {MetricPattern4<StoredF32>} ratio1mSma
 * @property {MetricPattern4<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {MetricPattern4<StoredF32>} ratioPct1
 * @property {MetricPattern4<Dollars>} ratioPct1Usd
 * @property {MetricPattern4<StoredF32>} ratioPct2
 * @property {MetricPattern4<Dollars>} ratioPct2Usd
 * @property {MetricPattern4<StoredF32>} ratioPct5
 * @property {MetricPattern4<Dollars>} ratioPct5Usd
 * @property {MetricPattern4<StoredF32>} ratioPct95
 * @property {MetricPattern4<Dollars>} ratioPct95Usd
 * @property {MetricPattern4<StoredF32>} ratioPct98
 * @property {MetricPattern4<Dollars>} ratioPct98Usd
 * @property {MetricPattern4<StoredF32>} ratioPct99
 * @property {MetricPattern4<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a Price111dSmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Price111dSmaPattern}
 */
function createPrice111dSmaPattern(client, acc) {
  return {
    price: createMetricPattern4(client, acc),
    ratio: createMetricPattern4(client, _m(acc, 'ratio')),
    ratio1mSma: createMetricPattern4(client, _m(acc, 'ratio_1m_sma')),
    ratio1wSma: createMetricPattern4(client, _m(acc, 'ratio_1w_sma')),
    ratio1ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_1y')),
    ratio2ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_2y')),
    ratio4ySd: createRatio1ySdPattern(client, _m(acc, 'ratio_4y')),
    ratioPct1: createMetricPattern4(client, _m(acc, 'ratio_pct1')),
    ratioPct1Usd: createMetricPattern4(client, _m(acc, 'ratio_pct1_usd')),
    ratioPct2: createMetricPattern4(client, _m(acc, 'ratio_pct2')),
    ratioPct2Usd: createMetricPattern4(client, _m(acc, 'ratio_pct2_usd')),
    ratioPct5: createMetricPattern4(client, _m(acc, 'ratio_pct5')),
    ratioPct5Usd: createMetricPattern4(client, _m(acc, 'ratio_pct5_usd')),
    ratioPct95: createMetricPattern4(client, _m(acc, 'ratio_pct95')),
    ratioPct95Usd: createMetricPattern4(client, _m(acc, 'ratio_pct95_usd')),
    ratioPct98: createMetricPattern4(client, _m(acc, 'ratio_pct98')),
    ratioPct98Usd: createMetricPattern4(client, _m(acc, 'ratio_pct98_usd')),
    ratioPct99: createMetricPattern4(client, _m(acc, 'ratio_pct99')),
    ratioPct99Usd: createMetricPattern4(client, _m(acc, 'ratio_pct99_usd')),
    ratioSd: createRatio1ySdPattern(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} ActivePriceRatioPattern
 * @property {MetricPattern4<StoredF32>} ratio
 * @property {MetricPattern4<StoredF32>} ratio1mSma
 * @property {MetricPattern4<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {MetricPattern4<StoredF32>} ratioPct1
 * @property {MetricPattern4<Dollars>} ratioPct1Usd
 * @property {MetricPattern4<StoredF32>} ratioPct2
 * @property {MetricPattern4<Dollars>} ratioPct2Usd
 * @property {MetricPattern4<StoredF32>} ratioPct5
 * @property {MetricPattern4<Dollars>} ratioPct5Usd
 * @property {MetricPattern4<StoredF32>} ratioPct95
 * @property {MetricPattern4<Dollars>} ratioPct95Usd
 * @property {MetricPattern4<StoredF32>} ratioPct98
 * @property {MetricPattern4<Dollars>} ratioPct98Usd
 * @property {MetricPattern4<StoredF32>} ratioPct99
 * @property {MetricPattern4<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a ActivePriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivePriceRatioPattern}
 */
function createActivePriceRatioPattern(client, acc) {
  return {
    ratio: createMetricPattern4(client, acc),
    ratio1mSma: createMetricPattern4(client, _m(acc, '1m_sma')),
    ratio1wSma: createMetricPattern4(client, _m(acc, '1w_sma')),
    ratio1ySd: createRatio1ySdPattern(client, _m(acc, '1y')),
    ratio2ySd: createRatio1ySdPattern(client, _m(acc, '2y')),
    ratio4ySd: createRatio1ySdPattern(client, _m(acc, '4y')),
    ratioPct1: createMetricPattern4(client, _m(acc, 'pct1')),
    ratioPct1Usd: createMetricPattern4(client, _m(acc, 'pct1_usd')),
    ratioPct2: createMetricPattern4(client, _m(acc, 'pct2')),
    ratioPct2Usd: createMetricPattern4(client, _m(acc, 'pct2_usd')),
    ratioPct5: createMetricPattern4(client, _m(acc, 'pct5')),
    ratioPct5Usd: createMetricPattern4(client, _m(acc, 'pct5_usd')),
    ratioPct95: createMetricPattern4(client, _m(acc, 'pct95')),
    ratioPct95Usd: createMetricPattern4(client, _m(acc, 'pct95_usd')),
    ratioPct98: createMetricPattern4(client, _m(acc, 'pct98')),
    ratioPct98Usd: createMetricPattern4(client, _m(acc, 'pct98_usd')),
    ratioPct99: createMetricPattern4(client, _m(acc, 'pct99')),
    ratioPct99Usd: createMetricPattern4(client, _m(acc, 'pct99_usd')),
    ratioSd: createRatio1ySdPattern(client, acc),
  };
}

/**
 * @typedef {Object} PercentilesPattern
 * @property {MetricPattern4<Dollars>} pct05
 * @property {MetricPattern4<Dollars>} pct10
 * @property {MetricPattern4<Dollars>} pct15
 * @property {MetricPattern4<Dollars>} pct20
 * @property {MetricPattern4<Dollars>} pct25
 * @property {MetricPattern4<Dollars>} pct30
 * @property {MetricPattern4<Dollars>} pct35
 * @property {MetricPattern4<Dollars>} pct40
 * @property {MetricPattern4<Dollars>} pct45
 * @property {MetricPattern4<Dollars>} pct50
 * @property {MetricPattern4<Dollars>} pct55
 * @property {MetricPattern4<Dollars>} pct60
 * @property {MetricPattern4<Dollars>} pct65
 * @property {MetricPattern4<Dollars>} pct70
 * @property {MetricPattern4<Dollars>} pct75
 * @property {MetricPattern4<Dollars>} pct80
 * @property {MetricPattern4<Dollars>} pct85
 * @property {MetricPattern4<Dollars>} pct90
 * @property {MetricPattern4<Dollars>} pct95
 */

/**
 * Create a PercentilesPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PercentilesPattern}
 */
function createPercentilesPattern(client, acc) {
  return {
    pct05: createMetricPattern4(client, _m(acc, 'pct05')),
    pct10: createMetricPattern4(client, _m(acc, 'pct10')),
    pct15: createMetricPattern4(client, _m(acc, 'pct15')),
    pct20: createMetricPattern4(client, _m(acc, 'pct20')),
    pct25: createMetricPattern4(client, _m(acc, 'pct25')),
    pct30: createMetricPattern4(client, _m(acc, 'pct30')),
    pct35: createMetricPattern4(client, _m(acc, 'pct35')),
    pct40: createMetricPattern4(client, _m(acc, 'pct40')),
    pct45: createMetricPattern4(client, _m(acc, 'pct45')),
    pct50: createMetricPattern4(client, _m(acc, 'pct50')),
    pct55: createMetricPattern4(client, _m(acc, 'pct55')),
    pct60: createMetricPattern4(client, _m(acc, 'pct60')),
    pct65: createMetricPattern4(client, _m(acc, 'pct65')),
    pct70: createMetricPattern4(client, _m(acc, 'pct70')),
    pct75: createMetricPattern4(client, _m(acc, 'pct75')),
    pct80: createMetricPattern4(client, _m(acc, 'pct80')),
    pct85: createMetricPattern4(client, _m(acc, 'pct85')),
    pct90: createMetricPattern4(client, _m(acc, 'pct90')),
    pct95: createMetricPattern4(client, _m(acc, 'pct95')),
  };
}

/**
 * @typedef {Object} RelativePattern5
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
 * @property {MetricPattern4<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern5}
 */
function createRelativePattern5(client, acc) {
  return {
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
    supplyRelToCirculatingSupply: createMetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedLossRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @typedef {Object} AaopoolPattern
 * @property {MetricPattern1<StoredU32>} _1mBlocksMined
 * @property {MetricPattern1<StoredF32>} _1mDominance
 * @property {MetricPattern1<StoredU32>} _1wBlocksMined
 * @property {MetricPattern1<StoredF32>} _1wDominance
 * @property {MetricPattern1<StoredU32>} _1yBlocksMined
 * @property {MetricPattern1<StoredF32>} _1yDominance
 * @property {MetricPattern1<StoredU32>} _24hBlocksMined
 * @property {MetricPattern1<StoredF32>} _24hDominance
 * @property {BlockCountPattern<StoredU32>} blocksMined
 * @property {CoinbasePattern2} coinbase
 * @property {MetricPattern4<StoredU16>} daysSinceBlock
 * @property {MetricPattern1<StoredF32>} dominance
 * @property {UnclaimedRewardsPattern} fee
 * @property {UnclaimedRewardsPattern} subsidy
 */

/**
 * Create a AaopoolPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AaopoolPattern}
 */
function createAaopoolPattern(client, acc) {
  return {
    _1mBlocksMined: createMetricPattern1(client, _m(acc, '1m_blocks_mined')),
    _1mDominance: createMetricPattern1(client, _m(acc, '1m_dominance')),
    _1wBlocksMined: createMetricPattern1(client, _m(acc, '1w_blocks_mined')),
    _1wDominance: createMetricPattern1(client, _m(acc, '1w_dominance')),
    _1yBlocksMined: createMetricPattern1(client, _m(acc, '1y_blocks_mined')),
    _1yDominance: createMetricPattern1(client, _m(acc, '1y_dominance')),
    _24hBlocksMined: createMetricPattern1(client, _m(acc, '24h_blocks_mined')),
    _24hDominance: createMetricPattern1(client, _m(acc, '24h_dominance')),
    blocksMined: createBlockCountPattern(client, _m(acc, 'blocks_mined')),
    coinbase: createCoinbasePattern2(client, _m(acc, 'coinbase')),
    daysSinceBlock: createMetricPattern4(client, _m(acc, 'days_since_block')),
    dominance: createMetricPattern1(client, _m(acc, 'dominance')),
    fee: createUnclaimedRewardsPattern(client, _m(acc, 'fee')),
    subsidy: createUnclaimedRewardsPattern(client, _m(acc, 'subsidy')),
  };
}

/**
 * @template T
 * @typedef {Object} LookbackPattern
 * @property {MetricPattern4<T>} _10y
 * @property {MetricPattern4<T>} _1d
 * @property {MetricPattern4<T>} _1m
 * @property {MetricPattern4<T>} _1w
 * @property {MetricPattern4<T>} _1y
 * @property {MetricPattern4<T>} _2y
 * @property {MetricPattern4<T>} _3m
 * @property {MetricPattern4<T>} _3y
 * @property {MetricPattern4<T>} _4y
 * @property {MetricPattern4<T>} _5y
 * @property {MetricPattern4<T>} _6m
 * @property {MetricPattern4<T>} _6y
 * @property {MetricPattern4<T>} _8y
 */

/**
 * Create a LookbackPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {LookbackPattern<T>}
 */
function createLookbackPattern(client, acc) {
  return {
    _10y: createMetricPattern4(client, _m(acc, '10y_ago')),
    _1d: createMetricPattern4(client, _m(acc, '1d_ago')),
    _1m: createMetricPattern4(client, _m(acc, '1m_ago')),
    _1w: createMetricPattern4(client, _m(acc, '1w_ago')),
    _1y: createMetricPattern4(client, _m(acc, '1y_ago')),
    _2y: createMetricPattern4(client, _m(acc, '2y_ago')),
    _3m: createMetricPattern4(client, _m(acc, '3m_ago')),
    _3y: createMetricPattern4(client, _m(acc, '3y_ago')),
    _4y: createMetricPattern4(client, _m(acc, '4y_ago')),
    _5y: createMetricPattern4(client, _m(acc, '5y_ago')),
    _6m: createMetricPattern4(client, _m(acc, '6m_ago')),
    _6y: createMetricPattern4(client, _m(acc, '6y_ago')),
    _8y: createMetricPattern4(client, _m(acc, '8y_ago')),
  };
}

/**
 * @typedef {Object} PeriodLumpSumStackPattern
 * @property {_2015Pattern} _10y
 * @property {_2015Pattern} _1m
 * @property {_2015Pattern} _1w
 * @property {_2015Pattern} _1y
 * @property {_2015Pattern} _2y
 * @property {_2015Pattern} _3m
 * @property {_2015Pattern} _3y
 * @property {_2015Pattern} _4y
 * @property {_2015Pattern} _5y
 * @property {_2015Pattern} _6m
 * @property {_2015Pattern} _6y
 * @property {_2015Pattern} _8y
 */

/**
 * Create a PeriodLumpSumStackPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodLumpSumStackPattern}
 */
function createPeriodLumpSumStackPattern(client, acc) {
  return {
    _10y: create_2015Pattern(client, _p('10y', acc)),
    _1m: create_2015Pattern(client, _p('1m', acc)),
    _1w: create_2015Pattern(client, _p('1w', acc)),
    _1y: create_2015Pattern(client, _p('1y', acc)),
    _2y: create_2015Pattern(client, _p('2y', acc)),
    _3m: create_2015Pattern(client, _p('3m', acc)),
    _3y: create_2015Pattern(client, _p('3y', acc)),
    _4y: create_2015Pattern(client, _p('4y', acc)),
    _5y: create_2015Pattern(client, _p('5y', acc)),
    _6m: create_2015Pattern(client, _p('6m', acc)),
    _6y: create_2015Pattern(client, _p('6y', acc)),
    _8y: create_2015Pattern(client, _p('8y', acc)),
  };
}

/**
 * @template T
 * @typedef {Object} PeriodAveragePricePattern
 * @property {MetricPattern4<T>} _10y
 * @property {MetricPattern4<T>} _1m
 * @property {MetricPattern4<T>} _1w
 * @property {MetricPattern4<T>} _1y
 * @property {MetricPattern4<T>} _2y
 * @property {MetricPattern4<T>} _3m
 * @property {MetricPattern4<T>} _3y
 * @property {MetricPattern4<T>} _4y
 * @property {MetricPattern4<T>} _5y
 * @property {MetricPattern4<T>} _6m
 * @property {MetricPattern4<T>} _6y
 * @property {MetricPattern4<T>} _8y
 */

/**
 * Create a PeriodAveragePricePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodAveragePricePattern<T>}
 */
function createPeriodAveragePricePattern(client, acc) {
  return {
    _10y: createMetricPattern4(client, _p('10y', acc)),
    _1m: createMetricPattern4(client, _p('1m', acc)),
    _1w: createMetricPattern4(client, _p('1w', acc)),
    _1y: createMetricPattern4(client, _p('1y', acc)),
    _2y: createMetricPattern4(client, _p('2y', acc)),
    _3m: createMetricPattern4(client, _p('3m', acc)),
    _3y: createMetricPattern4(client, _p('3y', acc)),
    _4y: createMetricPattern4(client, _p('4y', acc)),
    _5y: createMetricPattern4(client, _p('5y', acc)),
    _6m: createMetricPattern4(client, _p('6m', acc)),
    _6y: createMetricPattern4(client, _p('6y', acc)),
    _8y: createMetricPattern4(client, _p('8y', acc)),
  };
}

/**
 * @typedef {Object} BitcoinPattern
 * @property {MetricPattern2<Bitcoin>} average
 * @property {MetricPattern11<Bitcoin>} base
 * @property {MetricPattern2<Bitcoin>} cumulative
 * @property {MetricPattern2<Bitcoin>} max
 * @property {MetricPattern6<Bitcoin>} median
 * @property {MetricPattern2<Bitcoin>} min
 * @property {MetricPattern6<Bitcoin>} pct10
 * @property {MetricPattern6<Bitcoin>} pct25
 * @property {MetricPattern6<Bitcoin>} pct75
 * @property {MetricPattern6<Bitcoin>} pct90
 * @property {MetricPattern2<Bitcoin>} sum
 */

/**
 * Create a BitcoinPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinPattern}
 */
function createBitcoinPattern(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    base: createMetricPattern11(client, acc),
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    max: createMetricPattern2(client, _m(acc, 'max')),
    median: createMetricPattern6(client, _m(acc, 'median')),
    min: createMetricPattern2(client, _m(acc, 'min')),
    pct10: createMetricPattern6(client, _m(acc, 'pct10')),
    pct25: createMetricPattern6(client, _m(acc, 'pct25')),
    pct75: createMetricPattern6(client, _m(acc, 'pct75')),
    pct90: createMetricPattern6(client, _m(acc, 'pct90')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @template T
 * @typedef {Object} ClassAveragePricePattern
 * @property {MetricPattern4<T>} _2015
 * @property {MetricPattern4<T>} _2016
 * @property {MetricPattern4<T>} _2017
 * @property {MetricPattern4<T>} _2018
 * @property {MetricPattern4<T>} _2019
 * @property {MetricPattern4<T>} _2020
 * @property {MetricPattern4<T>} _2021
 * @property {MetricPattern4<T>} _2022
 * @property {MetricPattern4<T>} _2023
 * @property {MetricPattern4<T>} _2024
 * @property {MetricPattern4<T>} _2025
 */

/**
 * Create a ClassAveragePricePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ClassAveragePricePattern<T>}
 */
function createClassAveragePricePattern(client, acc) {
  return {
    _2015: createMetricPattern4(client, _m(acc, '2015_average_price')),
    _2016: createMetricPattern4(client, _m(acc, '2016_average_price')),
    _2017: createMetricPattern4(client, _m(acc, '2017_average_price')),
    _2018: createMetricPattern4(client, _m(acc, '2018_average_price')),
    _2019: createMetricPattern4(client, _m(acc, '2019_average_price')),
    _2020: createMetricPattern4(client, _m(acc, '2020_average_price')),
    _2021: createMetricPattern4(client, _m(acc, '2021_average_price')),
    _2022: createMetricPattern4(client, _m(acc, '2022_average_price')),
    _2023: createMetricPattern4(client, _m(acc, '2023_average_price')),
    _2024: createMetricPattern4(client, _m(acc, '2024_average_price')),
    _2025: createMetricPattern4(client, _m(acc, '2025_average_price')),
  };
}

/**
 * @template T
 * @typedef {Object} DollarsPattern
 * @property {MetricPattern2<T>} average
 * @property {MetricPattern11<T>} base
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern2<T>} max
 * @property {MetricPattern6<T>} median
 * @property {MetricPattern2<T>} min
 * @property {MetricPattern6<T>} pct10
 * @property {MetricPattern6<T>} pct25
 * @property {MetricPattern6<T>} pct75
 * @property {MetricPattern6<T>} pct90
 * @property {MetricPattern2<T>} sum
 */

/**
 * Create a DollarsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DollarsPattern<T>}
 */
function createDollarsPattern(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    base: createMetricPattern11(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    max: createMetricPattern2(client, _m(acc, 'max')),
    median: createMetricPattern6(client, _m(acc, 'median')),
    min: createMetricPattern2(client, _m(acc, 'min')),
    pct10: createMetricPattern6(client, _m(acc, 'pct10')),
    pct25: createMetricPattern6(client, _m(acc, 'pct25')),
    pct75: createMetricPattern6(client, _m(acc, 'pct75')),
    pct90: createMetricPattern6(client, _m(acc, 'pct90')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} RelativePattern
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern4<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a RelativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern}
 */
function createRelativePattern(client, acc) {
  return {
    negUnrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap')),
    netUnrealizedPnlRelToMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap')),
    nupl: createMetricPattern1(client, _m(acc, 'nupl')),
    supplyInLossRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToCirculatingSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    supplyRelToCirculatingSupply: createMetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} RelativePattern2
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern2}
 */
function createRelativePattern2(client, acc) {
  return {
    negUnrealizedLossRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap')),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')),
    netUnrealizedPnlRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap')),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
    unrealizedLossRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap')),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @template T
 * @typedef {Object} CountPattern2
 * @property {MetricPattern1<T>} average
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern1<T>} max
 * @property {MetricPattern11<T>} median
 * @property {MetricPattern1<T>} min
 * @property {MetricPattern11<T>} pct10
 * @property {MetricPattern11<T>} pct25
 * @property {MetricPattern11<T>} pct75
 * @property {MetricPattern11<T>} pct90
 * @property {MetricPattern1<T>} sum
 */

/**
 * Create a CountPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CountPattern2<T>}
 */
function createCountPattern2(client, acc) {
  return {
    average: createMetricPattern1(client, _m(acc, 'average')),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    max: createMetricPattern1(client, _m(acc, 'max')),
    median: createMetricPattern11(client, _m(acc, 'median')),
    min: createMetricPattern1(client, _m(acc, 'min')),
    pct10: createMetricPattern11(client, _m(acc, 'pct10')),
    pct25: createMetricPattern11(client, _m(acc, 'pct25')),
    pct75: createMetricPattern11(client, _m(acc, 'pct75')),
    pct90: createMetricPattern11(client, _m(acc, 'pct90')),
    sum: createMetricPattern1(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AddrCountPattern
 * @property {MetricPattern1<StoredU64>} all
 * @property {MetricPattern1<StoredU64>} p2a
 * @property {MetricPattern1<StoredU64>} p2pk33
 * @property {MetricPattern1<StoredU64>} p2pk65
 * @property {MetricPattern1<StoredU64>} p2pkh
 * @property {MetricPattern1<StoredU64>} p2sh
 * @property {MetricPattern1<StoredU64>} p2tr
 * @property {MetricPattern1<StoredU64>} p2wpkh
 * @property {MetricPattern1<StoredU64>} p2wsh
 */

/**
 * Create a AddrCountPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AddrCountPattern}
 */
function createAddrCountPattern(client, acc) {
  return {
    all: createMetricPattern1(client, acc),
    p2a: createMetricPattern1(client, _p('p2a', acc)),
    p2pk33: createMetricPattern1(client, _p('p2pk33', acc)),
    p2pk65: createMetricPattern1(client, _p('p2pk65', acc)),
    p2pkh: createMetricPattern1(client, _p('p2pkh', acc)),
    p2sh: createMetricPattern1(client, _p('p2sh', acc)),
    p2tr: createMetricPattern1(client, _p('p2tr', acc)),
    p2wpkh: createMetricPattern1(client, _p('p2wpkh', acc)),
    p2wsh: createMetricPattern1(client, _p('p2wsh', acc)),
  };
}

/**
 * @template T
 * @typedef {Object} FullnessPattern
 * @property {MetricPattern2<T>} average
 * @property {MetricPattern11<T>} base
 * @property {MetricPattern2<T>} max
 * @property {MetricPattern6<T>} median
 * @property {MetricPattern2<T>} min
 * @property {MetricPattern6<T>} pct10
 * @property {MetricPattern6<T>} pct25
 * @property {MetricPattern6<T>} pct75
 * @property {MetricPattern6<T>} pct90
 */

/**
 * Create a FullnessPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {FullnessPattern<T>}
 */
function createFullnessPattern(client, acc) {
  return {
    average: createMetricPattern2(client, _m(acc, 'average')),
    base: createMetricPattern11(client, acc),
    max: createMetricPattern2(client, _m(acc, 'max')),
    median: createMetricPattern6(client, _m(acc, 'median')),
    min: createMetricPattern2(client, _m(acc, 'min')),
    pct10: createMetricPattern6(client, _m(acc, 'pct10')),
    pct25: createMetricPattern6(client, _m(acc, 'pct25')),
    pct75: createMetricPattern6(client, _m(acc, 'pct75')),
    pct90: createMetricPattern6(client, _m(acc, 'pct90')),
  };
}

/**
 * @template T
 * @typedef {Object} FeeRatePattern
 * @property {MetricPattern1<T>} average
 * @property {MetricPattern1<T>} max
 * @property {MetricPattern11<T>} median
 * @property {MetricPattern1<T>} min
 * @property {MetricPattern11<T>} pct10
 * @property {MetricPattern11<T>} pct25
 * @property {MetricPattern11<T>} pct75
 * @property {MetricPattern11<T>} pct90
 * @property {MetricPattern27<T>} txindex
 */

/**
 * Create a FeeRatePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {FeeRatePattern<T>}
 */
function createFeeRatePattern(client, acc) {
  return {
    average: createMetricPattern1(client, _m(acc, 'average')),
    max: createMetricPattern1(client, _m(acc, 'max')),
    median: createMetricPattern11(client, _m(acc, 'median')),
    min: createMetricPattern1(client, _m(acc, 'min')),
    pct10: createMetricPattern11(client, _m(acc, 'pct10')),
    pct25: createMetricPattern11(client, _m(acc, 'pct25')),
    pct75: createMetricPattern11(client, _m(acc, 'pct75')),
    pct90: createMetricPattern11(client, _m(acc, 'pct90')),
    txindex: createMetricPattern27(client, acc),
  };
}

/**
 * @typedef {Object} _0satsPattern
 * @property {ActivityPattern2} activity
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {CostBasisPattern} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0satsPattern}
 */
function create_0satsPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    addrCount: createMetricPattern1(client, _m(acc, 'addr_count')),
    costBasis: createCostBasisPattern(client, acc),
    outputs: createOutputsPattern(client, _m(acc, 'utxo_count')),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} PhaseDailyCentsPattern
 * @property {MetricPattern6<T>} average
 * @property {MetricPattern6<T>} max
 * @property {MetricPattern6<T>} median
 * @property {MetricPattern6<T>} min
 * @property {MetricPattern6<T>} pct10
 * @property {MetricPattern6<T>} pct25
 * @property {MetricPattern6<T>} pct75
 * @property {MetricPattern6<T>} pct90
 */

/**
 * Create a PhaseDailyCentsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PhaseDailyCentsPattern<T>}
 */
function createPhaseDailyCentsPattern(client, acc) {
  return {
    average: createMetricPattern6(client, _m(acc, 'average')),
    max: createMetricPattern6(client, _m(acc, 'max')),
    median: createMetricPattern6(client, _m(acc, 'median')),
    min: createMetricPattern6(client, _m(acc, 'min')),
    pct10: createMetricPattern6(client, _m(acc, 'pct10')),
    pct25: createMetricPattern6(client, _m(acc, 'pct25')),
    pct75: createMetricPattern6(client, _m(acc, 'pct75')),
    pct90: createMetricPattern6(client, _m(acc, 'pct90')),
  };
}

/**
 * @typedef {Object} _10yTo12yPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern2} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yTo12yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10yTo12yPattern}
 */
function create_10yTo12yPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern2(client, acc),
    outputs: createOutputsPattern(client, _m(acc, 'utxo_count')),
    realized: createRealizedPattern2(client, acc),
    relative: createRelativePattern2(client, acc),
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} UnrealizedPattern
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {MetricPattern1<Dollars>} netUnrealizedPnl
 * @property {ActiveSupplyPattern} supplyInLoss
 * @property {ActiveSupplyPattern} supplyInProfit
 * @property {MetricPattern1<Dollars>} totalUnrealizedPnl
 * @property {MetricPattern1<Dollars>} unrealizedLoss
 * @property {MetricPattern1<Dollars>} unrealizedProfit
 */

/**
 * Create a UnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UnrealizedPattern}
 */
function createUnrealizedPattern(client, acc) {
  return {
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl')),
    supplyInLoss: createActiveSupplyPattern(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createActiveSupplyPattern(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern1(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern1(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} _10yPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern4} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10yPattern}
 */
function create_10yPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    outputs: createOutputsPattern(client, _m(acc, 'utxo_count')),
    realized: createRealizedPattern4(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _100btcPattern
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _100btcPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_100btcPattern}
 */
function create_100btcPattern(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    outputs: createOutputsPattern(client, _m(acc, 'utxo_count')),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern(client, acc),
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} PeriodCagrPattern
 * @property {MetricPattern4<StoredF32>} _10y
 * @property {MetricPattern4<StoredF32>} _2y
 * @property {MetricPattern4<StoredF32>} _3y
 * @property {MetricPattern4<StoredF32>} _4y
 * @property {MetricPattern4<StoredF32>} _5y
 * @property {MetricPattern4<StoredF32>} _6y
 * @property {MetricPattern4<StoredF32>} _8y
 */

/**
 * Create a PeriodCagrPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PeriodCagrPattern}
 */
function createPeriodCagrPattern(client, acc) {
  return {
    _10y: createMetricPattern4(client, _p('10y', acc)),
    _2y: createMetricPattern4(client, _p('2y', acc)),
    _3y: createMetricPattern4(client, _p('3y', acc)),
    _4y: createMetricPattern4(client, _p('4y', acc)),
    _5y: createMetricPattern4(client, _p('5y', acc)),
    _6y: createMetricPattern4(client, _p('6y', acc)),
    _8y: createMetricPattern4(client, _p('8y', acc)),
  };
}

/**
 * @typedef {Object} _0satsPattern2
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern} realized
 * @property {RelativePattern4} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0satsPattern2}
 */
function create_0satsPattern2(client, acc) {
  return {
    activity: createActivityPattern2(client, acc),
    costBasis: createCostBasisPattern(client, acc),
    outputs: createOutputsPattern(client, _m(acc, 'utxo_count')),
    realized: createRealizedPattern(client, acc),
    relative: createRelativePattern4(client, _m(acc, 'supply_in')),
    supply: createSupplyPattern2(client, _m(acc, 'supply')),
    unrealized: createUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityPattern2
 * @property {BlockCountPattern<StoredF64>} coinblocksDestroyed
 * @property {BlockCountPattern<StoredF64>} coindaysDestroyed
 * @property {MetricPattern11<Sats>} satblocksDestroyed
 * @property {MetricPattern11<Sats>} satdaysDestroyed
 * @property {UnclaimedRewardsPattern} sent
 */

/**
 * Create a ActivityPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityPattern2}
 */
function createActivityPattern2(client, acc) {
  return {
    coinblocksDestroyed: createBlockCountPattern(client, _m(acc, 'coinblocks_destroyed')),
    coindaysDestroyed: createBlockCountPattern(client, _m(acc, 'coindays_destroyed')),
    satblocksDestroyed: createMetricPattern11(client, _m(acc, 'satblocks_destroyed')),
    satdaysDestroyed: createMetricPattern11(client, _m(acc, 'satdays_destroyed')),
    sent: createUnclaimedRewardsPattern(client, _m(acc, 'sent')),
  };
}

/**
 * @template T
 * @typedef {Object} SplitPattern2
 * @property {MetricPattern1<T>} close
 * @property {MetricPattern1<T>} high
 * @property {MetricPattern1<T>} low
 * @property {MetricPattern1<T>} open
 */

/**
 * Create a SplitPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SplitPattern2<T>}
 */
function createSplitPattern2(client, acc) {
  return {
    close: createMetricPattern1(client, _m(acc, 'close')),
    high: createMetricPattern1(client, _m(acc, 'high')),
    low: createMetricPattern1(client, _m(acc, 'low')),
    open: createMetricPattern1(client, _m(acc, 'open')),
  };
}

/**
 * @typedef {Object} CostBasisPattern2
 * @property {MetricPattern1<Dollars>} max
 * @property {MetricPattern1<Dollars>} min
 * @property {PercentilesPattern} percentiles
 */

/**
 * Create a CostBasisPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CostBasisPattern2}
 */
function createCostBasisPattern2(client, acc) {
  return {
    max: createMetricPattern1(client, _m(acc, 'max_cost_basis')),
    min: createMetricPattern1(client, _m(acc, 'min_cost_basis')),
    percentiles: createPercentilesPattern(client, _m(acc, 'cost_basis')),
  };
}

/**
 * @typedef {Object} CoinbasePattern2
 * @property {BlockCountPattern<Bitcoin>} bitcoin
 * @property {BlockCountPattern<Dollars>} dollars
 * @property {BlockCountPattern<Sats>} sats
 */

/**
 * Create a CoinbasePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinbasePattern2}
 */
function createCoinbasePattern2(client, acc) {
  return {
    bitcoin: createBlockCountPattern(client, _m(acc, 'btc')),
    dollars: createBlockCountPattern(client, _m(acc, 'usd')),
    sats: createBlockCountPattern(client, acc),
  };
}

/**
 * @typedef {Object} CoinbasePattern
 * @property {BitcoinPattern} bitcoin
 * @property {DollarsPattern<Dollars>} dollars
 * @property {DollarsPattern<Sats>} sats
 */

/**
 * Create a CoinbasePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinbasePattern}
 */
function createCoinbasePattern(client, acc) {
  return {
    bitcoin: createBitcoinPattern(client, _m(acc, 'btc')),
    dollars: createDollarsPattern(client, _m(acc, 'usd')),
    sats: createDollarsPattern(client, acc),
  };
}

/**
 * @typedef {Object} UnclaimedRewardsPattern
 * @property {BitcoinPattern2<Bitcoin>} bitcoin
 * @property {BlockCountPattern<Dollars>} dollars
 * @property {BlockCountPattern<Sats>} sats
 */

/**
 * Create a UnclaimedRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UnclaimedRewardsPattern}
 */
function createUnclaimedRewardsPattern(client, acc) {
  return {
    bitcoin: createBitcoinPattern2(client, _m(acc, 'btc')),
    dollars: createBlockCountPattern(client, _m(acc, 'usd')),
    sats: createBlockCountPattern(client, acc),
  };
}

/**
 * @typedef {Object} _2015Pattern
 * @property {MetricPattern4<Bitcoin>} bitcoin
 * @property {MetricPattern4<Dollars>} dollars
 * @property {MetricPattern4<Sats>} sats
 */

/**
 * Create a _2015Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_2015Pattern}
 */
function create_2015Pattern(client, acc) {
  return {
    bitcoin: createMetricPattern4(client, _m(acc, 'btc')),
    dollars: createMetricPattern4(client, _m(acc, 'usd')),
    sats: createMetricPattern4(client, acc),
  };
}

/**
 * @typedef {Object} SegwitAdoptionPattern
 * @property {MetricPattern11<StoredF32>} base
 * @property {MetricPattern2<StoredF32>} cumulative
 * @property {MetricPattern2<StoredF32>} sum
 */

/**
 * Create a SegwitAdoptionPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SegwitAdoptionPattern}
 */
function createSegwitAdoptionPattern(client, acc) {
  return {
    base: createMetricPattern11(client, acc),
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} ActiveSupplyPattern
 * @property {MetricPattern1<Bitcoin>} bitcoin
 * @property {MetricPattern1<Dollars>} dollars
 * @property {MetricPattern1<Sats>} sats
 */

/**
 * Create a ActiveSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActiveSupplyPattern}
 */
function createActiveSupplyPattern(client, acc) {
  return {
    bitcoin: createMetricPattern1(client, _m(acc, 'btc')),
    dollars: createMetricPattern1(client, _m(acc, 'usd')),
    sats: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} SupplyPattern2
 * @property {ActiveSupplyPattern} halved
 * @property {ActiveSupplyPattern} total
 */

/**
 * Create a SupplyPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SupplyPattern2}
 */
function createSupplyPattern2(client, acc) {
  return {
    halved: createActiveSupplyPattern(client, _m(acc, 'halved')),
    total: createActiveSupplyPattern(client, acc),
  };
}

/**
 * @typedef {Object} _1dReturns1mSdPattern
 * @property {MetricPattern4<StoredF32>} sd
 * @property {MetricPattern4<StoredF32>} sma
 */

/**
 * Create a _1dReturns1mSdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1dReturns1mSdPattern}
 */
function create_1dReturns1mSdPattern(client, acc) {
  return {
    sd: createMetricPattern4(client, _m(acc, 'sd')),
    sma: createMetricPattern4(client, _m(acc, 'sma')),
  };
}

/**
 * @typedef {Object} CostBasisPattern
 * @property {MetricPattern1<Dollars>} max
 * @property {MetricPattern1<Dollars>} min
 */

/**
 * Create a CostBasisPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CostBasisPattern}
 */
function createCostBasisPattern(client, acc) {
  return {
    max: createMetricPattern1(client, _m(acc, 'max_cost_basis')),
    min: createMetricPattern1(client, _m(acc, 'min_cost_basis')),
  };
}

/**
 * @typedef {Object} RelativePattern4
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 */

/**
 * Create a RelativePattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelativePattern4}
 */
function createRelativePattern4(client, acc) {
  return {
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'loss_rel_to_own_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'profit_rel_to_own_supply')),
  };
}

/**
 * @template T
 * @typedef {Object} BitcoinPattern2
 * @property {MetricPattern2<T>} cumulative
 * @property {MetricPattern1<T>} sum
 */

/**
 * Create a BitcoinPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinPattern2<T>}
 */
function createBitcoinPattern2(client, acc) {
  return {
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    sum: createMetricPattern1(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} SatsPattern
 * @property {MetricPattern1<T>} ohlc
 * @property {SplitPattern2<T>} split
 */

/**
 * Create a SatsPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SatsPattern<T>}
 */
function createSatsPattern(client, acc) {
  return {
    ohlc: createMetricPattern1(client, _m(acc, 'ohlc_sats')),
    split: createSplitPattern2(client, _m(acc, 'sats')),
  };
}

/**
 * @template T
 * @typedef {Object} BlockCountPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern1<T>} sum
 */

/**
 * Create a BlockCountPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockCountPattern<T>}
 */
function createBlockCountPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    sum: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} RealizedPriceExtraPattern
 * @property {MetricPattern4<StoredF32>} ratio
 */

/**
 * Create a RealizedPriceExtraPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedPriceExtraPattern}
 */
function createRealizedPriceExtraPattern(client, acc) {
  return {
    ratio: createMetricPattern4(client, acc),
  };
}

/**
 * @typedef {Object} OutputsPattern
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * Create a OutputsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {OutputsPattern}
 */
function createOutputsPattern(client, acc) {
  return {
    utxoCount: createMetricPattern1(client, acc),
  };
}

// Catalog tree typedefs

/**
 * @typedef {Object} MetricsTree
 * @property {MetricsTree_Addresses} addresses
 * @property {MetricsTree_Blocks} blocks
 * @property {MetricsTree_Cointime} cointime
 * @property {MetricsTree_Constants} constants
 * @property {MetricsTree_Distribution} distribution
 * @property {MetricsTree_Indexes} indexes
 * @property {MetricsTree_Inputs} inputs
 * @property {MetricsTree_Market} market
 * @property {MetricsTree_Outputs} outputs
 * @property {MetricsTree_Pools} pools
 * @property {MetricsTree_Positions} positions
 * @property {MetricsTree_Price} price
 * @property {MetricsTree_Scripts} scripts
 * @property {MetricsTree_Supply} supply
 * @property {MetricsTree_Transactions} transactions
 */

/**
 * @typedef {Object} MetricsTree_Addresses
 * @property {MetricPattern11<P2AAddressIndex>} firstP2aaddressindex
 * @property {MetricPattern11<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {MetricPattern11<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {MetricPattern11<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {MetricPattern11<P2SHAddressIndex>} firstP2shaddressindex
 * @property {MetricPattern11<P2TRAddressIndex>} firstP2traddressindex
 * @property {MetricPattern11<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {MetricPattern11<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {MetricPattern16<P2ABytes>} p2abytes
 * @property {MetricPattern18<P2PK33Bytes>} p2pk33bytes
 * @property {MetricPattern19<P2PK65Bytes>} p2pk65bytes
 * @property {MetricPattern20<P2PKHBytes>} p2pkhbytes
 * @property {MetricPattern21<P2SHBytes>} p2shbytes
 * @property {MetricPattern22<P2TRBytes>} p2trbytes
 * @property {MetricPattern23<P2WPKHBytes>} p2wpkhbytes
 * @property {MetricPattern24<P2WSHBytes>} p2wshbytes
 */

/**
 * @typedef {Object} MetricsTree_Blocks
 * @property {MetricPattern11<BlockHash>} blockhash
 * @property {MetricsTree_Blocks_Count} count
 * @property {MetricsTree_Blocks_Difficulty} difficulty
 * @property {FullnessPattern<StoredF32>} fullness
 * @property {MetricsTree_Blocks_Halving} halving
 * @property {FullnessPattern<Timestamp>} interval
 * @property {MetricsTree_Blocks_Mining} mining
 * @property {MetricsTree_Blocks_Rewards} rewards
 * @property {MetricsTree_Blocks_Size} size
 * @property {MetricsTree_Blocks_Time} time
 * @property {MetricPattern11<StoredU64>} totalSize
 * @property {DollarsPattern<StoredU64>} vbytes
 * @property {DollarsPattern<Weight>} weight
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Count
 * @property {MetricPattern1<StoredU32>} _1mBlockCount
 * @property {MetricPattern11<Height>} _1mStart
 * @property {MetricPattern1<StoredU32>} _1wBlockCount
 * @property {MetricPattern11<Height>} _1wStart
 * @property {MetricPattern1<StoredU32>} _1yBlockCount
 * @property {MetricPattern11<Height>} _1yStart
 * @property {MetricPattern1<StoredU32>} _24hBlockCount
 * @property {MetricPattern11<Height>} _24hStart
 * @property {BlockCountPattern<StoredU32>} blockCount
 * @property {MetricPattern4<StoredU64>} blockCountTarget
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Difficulty
 * @property {MetricPattern1<StoredF32>} adjustment
 * @property {MetricPattern1<StoredF32>} asHash
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextAdjustment
 * @property {MetricPattern1<StoredF32>} daysBeforeNextAdjustment
 * @property {MetricPattern4<DifficultyEpoch>} epoch
 * @property {MetricPattern1<StoredF64>} raw
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Halving
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextHalving
 * @property {MetricPattern1<StoredF32>} daysBeforeNextHalving
 * @property {MetricPattern4<HalvingEpoch>} epoch
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Mining
 * @property {MetricPattern1<StoredF32>} hashPricePhs
 * @property {MetricPattern1<StoredF32>} hashPricePhsMin
 * @property {MetricPattern1<StoredF32>} hashPriceRebound
 * @property {MetricPattern1<StoredF32>} hashPriceThs
 * @property {MetricPattern1<StoredF32>} hashPriceThsMin
 * @property {MetricPattern1<StoredF64>} hashRate
 * @property {MetricPattern4<StoredF32>} hashRate1mSma
 * @property {MetricPattern4<StoredF64>} hashRate1wSma
 * @property {MetricPattern4<StoredF32>} hashRate1ySma
 * @property {MetricPattern4<StoredF32>} hashRate2mSma
 * @property {MetricPattern1<StoredF32>} hashValuePhs
 * @property {MetricPattern1<StoredF32>} hashValuePhsMin
 * @property {MetricPattern1<StoredF32>} hashValueRebound
 * @property {MetricPattern1<StoredF32>} hashValueThs
 * @property {MetricPattern1<StoredF32>} hashValueThsMin
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Rewards
 * @property {MetricsTree_Blocks_Rewards_24hCoinbaseSum} _24hCoinbaseSum
 * @property {CoinbasePattern} coinbase
 * @property {MetricPattern6<StoredF32>} feeDominance
 * @property {CoinbasePattern} subsidy
 * @property {MetricPattern6<StoredF32>} subsidyDominance
 * @property {MetricPattern4<Dollars>} subsidyUsd1ySma
 * @property {UnclaimedRewardsPattern} unclaimedRewards
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Rewards_24hCoinbaseSum
 * @property {MetricPattern11<Bitcoin>} bitcoin
 * @property {MetricPattern11<Dollars>} dollars
 * @property {MetricPattern11<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Size
 * @property {MetricPattern2<StoredU64>} average
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {MetricPattern2<StoredU64>} max
 * @property {MetricPattern6<StoredU64>} median
 * @property {MetricPattern2<StoredU64>} min
 * @property {MetricPattern6<StoredU64>} pct10
 * @property {MetricPattern6<StoredU64>} pct25
 * @property {MetricPattern6<StoredU64>} pct75
 * @property {MetricPattern6<StoredU64>} pct90
 * @property {MetricPattern2<StoredU64>} sum
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time
 * @property {MetricPattern11<Date>} date
 * @property {MetricPattern1<Timestamp>} timestamp
 * @property {MetricPattern11<Timestamp>} timestampMonotonic
 */

/**
 * @typedef {Object} MetricsTree_Cointime
 * @property {MetricsTree_Cointime_Activity} activity
 * @property {MetricsTree_Cointime_Adjusted} adjusted
 * @property {MetricsTree_Cointime_Cap} cap
 * @property {MetricsTree_Cointime_Pricing} pricing
 * @property {MetricsTree_Cointime_Supply} supply
 * @property {MetricsTree_Cointime_Value} value
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Activity
 * @property {MetricPattern1<StoredF64>} activityToVaultednessRatio
 * @property {BlockCountPattern<StoredF64>} coinblocksCreated
 * @property {BlockCountPattern<StoredF64>} coinblocksStored
 * @property {MetricPattern1<StoredF64>} liveliness
 * @property {MetricPattern1<StoredF64>} vaultedness
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Adjusted
 * @property {MetricPattern4<StoredF32>} cointimeAdjInflationRate
 * @property {MetricPattern4<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {MetricPattern4<StoredF64>} cointimeAdjTxUsdVelocity
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Cap
 * @property {MetricPattern1<Dollars>} activeCap
 * @property {MetricPattern1<Dollars>} cointimeCap
 * @property {MetricPattern1<Dollars>} investorCap
 * @property {MetricPattern1<Dollars>} thermoCap
 * @property {MetricPattern1<Dollars>} vaultedCap
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Pricing
 * @property {MetricPattern1<Dollars>} activePrice
 * @property {ActivePriceRatioPattern} activePriceRatio
 * @property {MetricPattern1<Dollars>} cointimePrice
 * @property {ActivePriceRatioPattern} cointimePriceRatio
 * @property {MetricPattern1<Dollars>} trueMarketMean
 * @property {ActivePriceRatioPattern} trueMarketMeanRatio
 * @property {MetricPattern1<Dollars>} vaultedPrice
 * @property {ActivePriceRatioPattern} vaultedPriceRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Supply
 * @property {ActiveSupplyPattern} activeSupply
 * @property {ActiveSupplyPattern} vaultedSupply
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Value
 * @property {BlockCountPattern<StoredF64>} cointimeValueCreated
 * @property {BlockCountPattern<StoredF64>} cointimeValueDestroyed
 * @property {BlockCountPattern<StoredF64>} cointimeValueStored
 */

/**
 * @typedef {Object} MetricsTree_Constants
 * @property {MetricPattern1<StoredU16>} constant0
 * @property {MetricPattern1<StoredU16>} constant1
 * @property {MetricPattern1<StoredU16>} constant100
 * @property {MetricPattern1<StoredU16>} constant2
 * @property {MetricPattern1<StoredU16>} constant20
 * @property {MetricPattern1<StoredU16>} constant3
 * @property {MetricPattern1<StoredU16>} constant30
 * @property {MetricPattern1<StoredF32>} constant382
 * @property {MetricPattern1<StoredU16>} constant4
 * @property {MetricPattern1<StoredU16>} constant50
 * @property {MetricPattern1<StoredU16>} constant600
 * @property {MetricPattern1<StoredF32>} constant618
 * @property {MetricPattern1<StoredU16>} constant70
 * @property {MetricPattern1<StoredU16>} constant80
 * @property {MetricPattern1<StoredI8>} constantMinus1
 * @property {MetricPattern1<StoredI8>} constantMinus2
 * @property {MetricPattern1<StoredI8>} constantMinus3
 * @property {MetricPattern1<StoredI8>} constantMinus4
 */

/**
 * @typedef {Object} MetricsTree_Distribution
 * @property {AddrCountPattern} addrCount
 * @property {MetricsTree_Distribution_AddressCohorts} addressCohorts
 * @property {MetricsTree_Distribution_AddressesData} addressesData
 * @property {MetricsTree_Distribution_AnyAddressIndexes} anyAddressIndexes
 * @property {MetricPattern11<SupplyState>} chainState
 * @property {AddrCountPattern} emptyAddrCount
 * @property {MetricPattern32<EmptyAddressIndex>} emptyaddressindex
 * @property {MetricPattern31<LoadedAddressIndex>} loadedaddressindex
 * @property {MetricsTree_Distribution_UtxoCohorts} utxoCohorts
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts
 * @property {MetricsTree_Distribution_AddressCohorts_AmountRange} amountRange
 * @property {MetricsTree_Distribution_AddressCohorts_GeAmount} geAmount
 * @property {MetricsTree_Distribution_AddressCohorts_LtAmount} ltAmount
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_AmountRange
 * @property {_0satsPattern} _0sats
 * @property {_0satsPattern} _100btcTo1kBtc
 * @property {_0satsPattern} _100kBtcOrMore
 * @property {_0satsPattern} _100kSatsTo1mSats
 * @property {_0satsPattern} _100satsTo1kSats
 * @property {_0satsPattern} _10btcTo100btc
 * @property {_0satsPattern} _10kBtcTo100kBtc
 * @property {_0satsPattern} _10kSatsTo100kSats
 * @property {_0satsPattern} _10mSatsTo1btc
 * @property {_0satsPattern} _10satsTo100sats
 * @property {_0satsPattern} _1btcTo10btc
 * @property {_0satsPattern} _1kBtcTo10kBtc
 * @property {_0satsPattern} _1kSatsTo10kSats
 * @property {_0satsPattern} _1mSatsTo10mSats
 * @property {_0satsPattern} _1satTo10sats
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_GeAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 * @property {_0satsPattern} _1sat
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressCohorts_LtAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kBtc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AddressesData
 * @property {MetricPattern32<EmptyAddressData>} empty
 * @property {MetricPattern31<LoadedAddressData>} loaded
 */

/**
 * @typedef {Object} MetricsTree_Distribution_AnyAddressIndexes
 * @property {MetricPattern16<AnyAddressIndex>} p2a
 * @property {MetricPattern18<AnyAddressIndex>} p2pk33
 * @property {MetricPattern19<AnyAddressIndex>} p2pk65
 * @property {MetricPattern20<AnyAddressIndex>} p2pkh
 * @property {MetricPattern21<AnyAddressIndex>} p2sh
 * @property {MetricPattern22<AnyAddressIndex>} p2tr
 * @property {MetricPattern23<AnyAddressIndex>} p2wpkh
 * @property {MetricPattern24<AnyAddressIndex>} p2wsh
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts
 * @property {MetricsTree_Distribution_UtxoCohorts_AgeRange} ageRange
 * @property {MetricsTree_Distribution_UtxoCohorts_All} all
 * @property {MetricsTree_Distribution_UtxoCohorts_AmountRange} amountRange
 * @property {MetricsTree_Distribution_UtxoCohorts_Epoch} epoch
 * @property {MetricsTree_Distribution_UtxoCohorts_GeAmount} geAmount
 * @property {MetricsTree_Distribution_UtxoCohorts_LtAmount} ltAmount
 * @property {MetricsTree_Distribution_UtxoCohorts_MaxAge} maxAge
 * @property {MetricsTree_Distribution_UtxoCohorts_MinAge} minAge
 * @property {MetricsTree_Distribution_UtxoCohorts_Term} term
 * @property {MetricsTree_Distribution_UtxoCohorts_Type} type
 * @property {MetricsTree_Distribution_UtxoCohorts_Year} year
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_AgeRange
 * @property {_10yTo12yPattern} _10yTo12y
 * @property {_10yTo12yPattern} _12yTo15y
 * @property {_10yTo12yPattern} _1dTo1w
 * @property {_10yTo12yPattern} _1hTo1d
 * @property {_10yTo12yPattern} _1mTo2m
 * @property {_10yTo12yPattern} _1wTo1m
 * @property {_10yTo12yPattern} _1yTo2y
 * @property {_10yTo12yPattern} _2mTo3m
 * @property {_10yTo12yPattern} _2yTo3y
 * @property {_10yTo12yPattern} _3mTo4m
 * @property {_10yTo12yPattern} _3yTo4y
 * @property {_10yTo12yPattern} _4mTo5m
 * @property {_10yTo12yPattern} _4yTo5y
 * @property {_10yTo12yPattern} _5mTo6m
 * @property {_10yTo12yPattern} _5yTo6y
 * @property {_10yTo12yPattern} _6mTo1y
 * @property {_10yTo12yPattern} _6yTo7y
 * @property {_10yTo12yPattern} _7yTo8y
 * @property {_10yTo12yPattern} _8yTo10y
 * @property {_10yTo12yPattern} from15y
 * @property {_10yTo12yPattern} upTo1h
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All
 * @property {ActivityPattern2} activity
 * @property {MetricsTree_Distribution_UtxoCohorts_All_CostBasis} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern3} realized
 * @property {MetricsTree_Distribution_UtxoCohorts_All_Relative} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All_CostBasis
 * @property {MetricPattern1<Dollars>} max
 * @property {MetricPattern1<Dollars>} min
 * @property {PercentilesPattern} percentiles
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All_Relative
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_AmountRange
 * @property {_0satsPattern2} _0sats
 * @property {_0satsPattern2} _100btcTo1kBtc
 * @property {_0satsPattern2} _100kBtcOrMore
 * @property {_0satsPattern2} _100kSatsTo1mSats
 * @property {_0satsPattern2} _100satsTo1kSats
 * @property {_0satsPattern2} _10btcTo100btc
 * @property {_0satsPattern2} _10kBtcTo100kBtc
 * @property {_0satsPattern2} _10kSatsTo100kSats
 * @property {_0satsPattern2} _10mSatsTo1btc
 * @property {_0satsPattern2} _10satsTo100sats
 * @property {_0satsPattern2} _1btcTo10btc
 * @property {_0satsPattern2} _1kBtcTo10kBtc
 * @property {_0satsPattern2} _1kSatsTo10kSats
 * @property {_0satsPattern2} _1mSatsTo10mSats
 * @property {_0satsPattern2} _1satTo10sats
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Epoch
 * @property {_0satsPattern2} _0
 * @property {_0satsPattern2} _1
 * @property {_0satsPattern2} _2
 * @property {_0satsPattern2} _3
 * @property {_0satsPattern2} _4
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_GeAmount
 * @property {_100btcPattern} _100btc
 * @property {_100btcPattern} _100kSats
 * @property {_100btcPattern} _100sats
 * @property {_100btcPattern} _10btc
 * @property {_100btcPattern} _10kBtc
 * @property {_100btcPattern} _10kSats
 * @property {_100btcPattern} _10mSats
 * @property {_100btcPattern} _10sats
 * @property {_100btcPattern} _1btc
 * @property {_100btcPattern} _1kBtc
 * @property {_100btcPattern} _1kSats
 * @property {_100btcPattern} _1mSats
 * @property {_100btcPattern} _1sat
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_LtAmount
 * @property {_100btcPattern} _100btc
 * @property {_100btcPattern} _100kBtc
 * @property {_100btcPattern} _100kSats
 * @property {_100btcPattern} _100sats
 * @property {_100btcPattern} _10btc
 * @property {_100btcPattern} _10kBtc
 * @property {_100btcPattern} _10kSats
 * @property {_100btcPattern} _10mSats
 * @property {_100btcPattern} _10sats
 * @property {_100btcPattern} _1btc
 * @property {_100btcPattern} _1kBtc
 * @property {_100btcPattern} _1kSats
 * @property {_100btcPattern} _1mSats
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MaxAge
 * @property {_10yPattern} _10y
 * @property {_10yPattern} _12y
 * @property {_10yPattern} _15y
 * @property {_10yPattern} _1m
 * @property {_10yPattern} _1w
 * @property {_10yPattern} _1y
 * @property {_10yPattern} _2m
 * @property {_10yPattern} _2y
 * @property {_10yPattern} _3m
 * @property {_10yPattern} _3y
 * @property {_10yPattern} _4m
 * @property {_10yPattern} _4y
 * @property {_10yPattern} _5m
 * @property {_10yPattern} _5y
 * @property {_10yPattern} _6m
 * @property {_10yPattern} _6y
 * @property {_10yPattern} _7y
 * @property {_10yPattern} _8y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MinAge
 * @property {_100btcPattern} _10y
 * @property {_100btcPattern} _12y
 * @property {_100btcPattern} _1d
 * @property {_100btcPattern} _1m
 * @property {_100btcPattern} _1w
 * @property {_100btcPattern} _1y
 * @property {_100btcPattern} _2m
 * @property {_100btcPattern} _2y
 * @property {_100btcPattern} _3m
 * @property {_100btcPattern} _3y
 * @property {_100btcPattern} _4m
 * @property {_100btcPattern} _4y
 * @property {_100btcPattern} _5m
 * @property {_100btcPattern} _5y
 * @property {_100btcPattern} _6m
 * @property {_100btcPattern} _6y
 * @property {_100btcPattern} _7y
 * @property {_100btcPattern} _8y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term
 * @property {MetricsTree_Distribution_UtxoCohorts_Term_Long} long
 * @property {MetricsTree_Distribution_UtxoCohorts_Term_Short} short
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term_Long
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern2} realized
 * @property {RelativePattern5} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term_Short
 * @property {ActivityPattern2} activity
 * @property {CostBasisPattern2} costBasis
 * @property {OutputsPattern} outputs
 * @property {RealizedPattern3} realized
 * @property {RelativePattern5} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Type
 * @property {_0satsPattern2} empty
 * @property {_0satsPattern2} p2a
 * @property {_0satsPattern2} p2ms
 * @property {_0satsPattern2} p2pk33
 * @property {_0satsPattern2} p2pk65
 * @property {_0satsPattern2} p2pkh
 * @property {_0satsPattern2} p2sh
 * @property {_0satsPattern2} p2tr
 * @property {_0satsPattern2} p2wpkh
 * @property {_0satsPattern2} p2wsh
 * @property {_0satsPattern2} unknown
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Year
 * @property {_0satsPattern2} _2009
 * @property {_0satsPattern2} _2010
 * @property {_0satsPattern2} _2011
 * @property {_0satsPattern2} _2012
 * @property {_0satsPattern2} _2013
 * @property {_0satsPattern2} _2014
 * @property {_0satsPattern2} _2015
 * @property {_0satsPattern2} _2016
 * @property {_0satsPattern2} _2017
 * @property {_0satsPattern2} _2018
 * @property {_0satsPattern2} _2019
 * @property {_0satsPattern2} _2020
 * @property {_0satsPattern2} _2021
 * @property {_0satsPattern2} _2022
 * @property {_0satsPattern2} _2023
 * @property {_0satsPattern2} _2024
 * @property {_0satsPattern2} _2025
 * @property {_0satsPattern2} _2026
 */

/**
 * @typedef {Object} MetricsTree_Indexes
 * @property {MetricsTree_Indexes_Address} address
 * @property {MetricsTree_Indexes_Dateindex} dateindex
 * @property {MetricsTree_Indexes_Decadeindex} decadeindex
 * @property {MetricsTree_Indexes_Difficultyepoch} difficultyepoch
 * @property {MetricsTree_Indexes_Halvingepoch} halvingepoch
 * @property {MetricsTree_Indexes_Height} height
 * @property {MetricsTree_Indexes_Monthindex} monthindex
 * @property {MetricsTree_Indexes_Quarterindex} quarterindex
 * @property {MetricsTree_Indexes_Semesterindex} semesterindex
 * @property {MetricsTree_Indexes_Txindex} txindex
 * @property {MetricsTree_Indexes_Txinindex} txinindex
 * @property {MetricsTree_Indexes_Txoutindex} txoutindex
 * @property {MetricsTree_Indexes_Weekindex} weekindex
 * @property {MetricsTree_Indexes_Yearindex} yearindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address
 * @property {MetricsTree_Indexes_Address_Empty} empty
 * @property {MetricsTree_Indexes_Address_Opreturn} opreturn
 * @property {MetricsTree_Indexes_Address_P2a} p2a
 * @property {MetricsTree_Indexes_Address_P2ms} p2ms
 * @property {MetricsTree_Indexes_Address_P2pk33} p2pk33
 * @property {MetricsTree_Indexes_Address_P2pk65} p2pk65
 * @property {MetricsTree_Indexes_Address_P2pkh} p2pkh
 * @property {MetricsTree_Indexes_Address_P2sh} p2sh
 * @property {MetricsTree_Indexes_Address_P2tr} p2tr
 * @property {MetricsTree_Indexes_Address_P2wpkh} p2wpkh
 * @property {MetricsTree_Indexes_Address_P2wsh} p2wsh
 * @property {MetricsTree_Indexes_Address_Unknown} unknown
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Empty
 * @property {MetricPattern9<EmptyOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Opreturn
 * @property {MetricPattern14<OpReturnIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2a
 * @property {MetricPattern16<P2AAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2ms
 * @property {MetricPattern17<P2MSOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pk33
 * @property {MetricPattern18<P2PK33AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pk65
 * @property {MetricPattern19<P2PK65AddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2pkh
 * @property {MetricPattern20<P2PKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2sh
 * @property {MetricPattern21<P2SHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2tr
 * @property {MetricPattern22<P2TRAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wpkh
 * @property {MetricPattern23<P2WPKHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2wsh
 * @property {MetricPattern24<P2WSHAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Unknown
 * @property {MetricPattern28<UnknownOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Dateindex
 * @property {MetricPattern6<Date>} date
 * @property {MetricPattern6<Height>} firstHeight
 * @property {MetricPattern6<StoredU64>} heightCount
 * @property {MetricPattern6<DateIndex>} identity
 * @property {MetricPattern6<MonthIndex>} monthindex
 * @property {MetricPattern6<WeekIndex>} weekindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Decadeindex
 * @property {MetricPattern7<Date>} date
 * @property {MetricPattern7<YearIndex>} firstYearindex
 * @property {MetricPattern7<DecadeIndex>} identity
 * @property {MetricPattern7<StoredU64>} yearindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Difficultyepoch
 * @property {MetricPattern8<Height>} firstHeight
 * @property {MetricPattern8<StoredU64>} heightCount
 * @property {MetricPattern8<DifficultyEpoch>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Halvingepoch
 * @property {MetricPattern10<Height>} firstHeight
 * @property {MetricPattern10<HalvingEpoch>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Height
 * @property {MetricPattern11<DateIndex>} dateindex
 * @property {MetricPattern11<DifficultyEpoch>} difficultyepoch
 * @property {MetricPattern11<HalvingEpoch>} halvingepoch
 * @property {MetricPattern11<Height>} identity
 * @property {MetricPattern11<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Monthindex
 * @property {MetricPattern13<Date>} date
 * @property {MetricPattern13<StoredU64>} dateindexCount
 * @property {MetricPattern13<DateIndex>} firstDateindex
 * @property {MetricPattern13<MonthIndex>} identity
 * @property {MetricPattern13<QuarterIndex>} quarterindex
 * @property {MetricPattern13<SemesterIndex>} semesterindex
 * @property {MetricPattern13<YearIndex>} yearindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Quarterindex
 * @property {MetricPattern25<Date>} date
 * @property {MetricPattern25<MonthIndex>} firstMonthindex
 * @property {MetricPattern25<QuarterIndex>} identity
 * @property {MetricPattern25<StoredU64>} monthindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Semesterindex
 * @property {MetricPattern26<Date>} date
 * @property {MetricPattern26<MonthIndex>} firstMonthindex
 * @property {MetricPattern26<SemesterIndex>} identity
 * @property {MetricPattern26<StoredU64>} monthindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txindex
 * @property {MetricPattern27<TxIndex>} identity
 * @property {MetricPattern27<StoredU64>} inputCount
 * @property {MetricPattern27<StoredU64>} outputCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txinindex
 * @property {MetricPattern12<TxInIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Txoutindex
 * @property {MetricPattern15<TxOutIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Weekindex
 * @property {MetricPattern29<Date>} date
 * @property {MetricPattern29<StoredU64>} dateindexCount
 * @property {MetricPattern29<DateIndex>} firstDateindex
 * @property {MetricPattern29<WeekIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Yearindex
 * @property {MetricPattern30<Date>} date
 * @property {MetricPattern30<DecadeIndex>} decadeindex
 * @property {MetricPattern30<MonthIndex>} firstMonthindex
 * @property {MetricPattern30<YearIndex>} identity
 * @property {MetricPattern30<StoredU64>} monthindexCount
 */

/**
 * @typedef {Object} MetricsTree_Inputs
 * @property {CountPattern2<StoredU64>} count
 * @property {MetricPattern11<TxInIndex>} firstTxinindex
 * @property {MetricPattern12<OutPoint>} outpoint
 * @property {MetricPattern12<OutputType>} outputtype
 * @property {MetricsTree_Inputs_Spent} spent
 * @property {MetricPattern12<TxIndex>} txindex
 * @property {MetricPattern12<TypeIndex>} typeindex
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Spent
 * @property {MetricPattern12<TxOutIndex>} txoutindex
 * @property {MetricPattern12<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Market
 * @property {MetricsTree_Market_Ath} ath
 * @property {MetricsTree_Market_Dca} dca
 * @property {MetricsTree_Market_Indicators} indicators
 * @property {LookbackPattern<Dollars>} lookback
 * @property {MetricsTree_Market_MovingAverage} movingAverage
 * @property {MetricsTree_Market_Range} range
 * @property {MetricsTree_Market_Returns} returns
 * @property {MetricsTree_Market_Volatility} volatility
 */

/**
 * @typedef {Object} MetricsTree_Market_Ath
 * @property {MetricPattern4<StoredU16>} daysSincePriceAth
 * @property {MetricPattern4<StoredU16>} maxDaysBetweenPriceAths
 * @property {MetricPattern4<StoredF32>} maxYearsBetweenPriceAths
 * @property {MetricPattern1<Dollars>} priceAth
 * @property {MetricPattern3<StoredF32>} priceDrawdown
 * @property {MetricPattern4<StoredF32>} yearsSincePriceAth
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca
 * @property {ClassAveragePricePattern<Dollars>} classAveragePrice
 * @property {MetricsTree_Market_Dca_ClassReturns} classReturns
 * @property {MetricsTree_Market_Dca_ClassStack} classStack
 * @property {PeriodAveragePricePattern<Dollars>} periodAveragePrice
 * @property {PeriodCagrPattern} periodCagr
 * @property {PeriodLumpSumStackPattern} periodLumpSumStack
 * @property {PeriodAveragePricePattern<StoredF32>} periodReturns
 * @property {PeriodLumpSumStackPattern} periodStack
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassReturns
 * @property {MetricPattern4<StoredF32>} _2015
 * @property {MetricPattern4<StoredF32>} _2016
 * @property {MetricPattern4<StoredF32>} _2017
 * @property {MetricPattern4<StoredF32>} _2018
 * @property {MetricPattern4<StoredF32>} _2019
 * @property {MetricPattern4<StoredF32>} _2020
 * @property {MetricPattern4<StoredF32>} _2021
 * @property {MetricPattern4<StoredF32>} _2022
 * @property {MetricPattern4<StoredF32>} _2023
 * @property {MetricPattern4<StoredF32>} _2024
 * @property {MetricPattern4<StoredF32>} _2025
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassStack
 * @property {_2015Pattern} _2015
 * @property {_2015Pattern} _2016
 * @property {_2015Pattern} _2017
 * @property {_2015Pattern} _2018
 * @property {_2015Pattern} _2019
 * @property {_2015Pattern} _2020
 * @property {_2015Pattern} _2021
 * @property {_2015Pattern} _2022
 * @property {_2015Pattern} _2023
 * @property {_2015Pattern} _2024
 * @property {_2015Pattern} _2025
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators
 * @property {MetricPattern6<StoredF32>} gini
 * @property {MetricPattern6<StoredF32>} macdHistogram
 * @property {MetricPattern6<StoredF32>} macdLine
 * @property {MetricPattern6<StoredF32>} macdSignal
 * @property {MetricPattern4<StoredF32>} nvt
 * @property {MetricPattern6<StoredF32>} piCycle
 * @property {MetricPattern4<StoredF32>} puellMultiple
 * @property {MetricPattern6<StoredF32>} rsi14d
 * @property {MetricPattern6<StoredF32>} rsi14dMax
 * @property {MetricPattern6<StoredF32>} rsi14dMin
 * @property {MetricPattern6<StoredF32>} rsiAverageGain14d
 * @property {MetricPattern6<StoredF32>} rsiAverageLoss14d
 * @property {MetricPattern6<StoredF32>} rsiGains
 * @property {MetricPattern6<StoredF32>} rsiLosses
 * @property {MetricPattern6<StoredF32>} stochD
 * @property {MetricPattern6<StoredF32>} stochK
 * @property {MetricPattern6<StoredF32>} stochRsi
 * @property {MetricPattern6<StoredF32>} stochRsiD
 * @property {MetricPattern6<StoredF32>} stochRsiK
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage
 * @property {Price111dSmaPattern} price111dSma
 * @property {Price111dSmaPattern} price12dEma
 * @property {Price111dSmaPattern} price13dEma
 * @property {Price111dSmaPattern} price13dSma
 * @property {Price111dSmaPattern} price144dEma
 * @property {Price111dSmaPattern} price144dSma
 * @property {Price111dSmaPattern} price1mEma
 * @property {Price111dSmaPattern} price1mSma
 * @property {Price111dSmaPattern} price1wEma
 * @property {Price111dSmaPattern} price1wSma
 * @property {Price111dSmaPattern} price1yEma
 * @property {Price111dSmaPattern} price1ySma
 * @property {Price111dSmaPattern} price200dEma
 * @property {Price111dSmaPattern} price200dSma
 * @property {MetricPattern4<Dollars>} price200dSmaX08
 * @property {MetricPattern4<Dollars>} price200dSmaX24
 * @property {Price111dSmaPattern} price200wEma
 * @property {Price111dSmaPattern} price200wSma
 * @property {Price111dSmaPattern} price21dEma
 * @property {Price111dSmaPattern} price21dSma
 * @property {Price111dSmaPattern} price26dEma
 * @property {Price111dSmaPattern} price2yEma
 * @property {Price111dSmaPattern} price2ySma
 * @property {Price111dSmaPattern} price34dEma
 * @property {Price111dSmaPattern} price34dSma
 * @property {Price111dSmaPattern} price350dSma
 * @property {MetricPattern4<Dollars>} price350dSmaX2
 * @property {Price111dSmaPattern} price4yEma
 * @property {Price111dSmaPattern} price4ySma
 * @property {Price111dSmaPattern} price55dEma
 * @property {Price111dSmaPattern} price55dSma
 * @property {Price111dSmaPattern} price89dEma
 * @property {Price111dSmaPattern} price89dSma
 * @property {Price111dSmaPattern} price8dEma
 * @property {Price111dSmaPattern} price8dSma
 */

/**
 * @typedef {Object} MetricsTree_Market_Range
 * @property {MetricPattern4<Dollars>} price1mMax
 * @property {MetricPattern4<Dollars>} price1mMin
 * @property {MetricPattern4<Dollars>} price1wMax
 * @property {MetricPattern4<Dollars>} price1wMin
 * @property {MetricPattern4<Dollars>} price1yMax
 * @property {MetricPattern4<Dollars>} price1yMin
 * @property {MetricPattern4<StoredF32>} price2wChoppinessIndex
 * @property {MetricPattern4<Dollars>} price2wMax
 * @property {MetricPattern4<Dollars>} price2wMin
 * @property {MetricPattern6<StoredF32>} priceTrueRange
 * @property {MetricPattern6<StoredF32>} priceTrueRange2wSum
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns
 * @property {_1dReturns1mSdPattern} _1dReturns1mSd
 * @property {_1dReturns1mSdPattern} _1dReturns1wSd
 * @property {_1dReturns1mSdPattern} _1dReturns1ySd
 * @property {PeriodCagrPattern} cagr
 * @property {_1dReturns1mSdPattern} downside1mSd
 * @property {_1dReturns1mSdPattern} downside1wSd
 * @property {_1dReturns1mSdPattern} downside1ySd
 * @property {MetricPattern6<StoredF32>} downsideReturns
 * @property {MetricsTree_Market_Returns_PriceReturns} priceReturns
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturns
 * @property {MetricPattern4<StoredF32>} _10y
 * @property {MetricPattern4<StoredF32>} _1d
 * @property {MetricPattern4<StoredF32>} _1m
 * @property {MetricPattern4<StoredF32>} _1w
 * @property {MetricPattern4<StoredF32>} _1y
 * @property {MetricPattern4<StoredF32>} _2y
 * @property {MetricPattern4<StoredF32>} _3m
 * @property {MetricPattern4<StoredF32>} _3y
 * @property {MetricPattern4<StoredF32>} _4y
 * @property {MetricPattern4<StoredF32>} _5y
 * @property {MetricPattern4<StoredF32>} _6m
 * @property {MetricPattern4<StoredF32>} _6y
 * @property {MetricPattern4<StoredF32>} _8y
 */

/**
 * @typedef {Object} MetricsTree_Market_Volatility
 * @property {MetricPattern4<StoredF32>} price1mVolatility
 * @property {MetricPattern4<StoredF32>} price1wVolatility
 * @property {MetricPattern4<StoredF32>} price1yVolatility
 * @property {MetricPattern6<StoredF32>} sharpe1m
 * @property {MetricPattern6<StoredF32>} sharpe1w
 * @property {MetricPattern6<StoredF32>} sharpe1y
 * @property {MetricPattern6<StoredF32>} sortino1m
 * @property {MetricPattern6<StoredF32>} sortino1w
 * @property {MetricPattern6<StoredF32>} sortino1y
 */

/**
 * @typedef {Object} MetricsTree_Outputs
 * @property {MetricsTree_Outputs_Count} count
 * @property {MetricPattern11<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern15<OutputType>} outputtype
 * @property {MetricsTree_Outputs_Spent} spent
 * @property {MetricPattern15<TxIndex>} txindex
 * @property {MetricPattern15<TypeIndex>} typeindex
 * @property {MetricPattern15<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Count
 * @property {CountPattern2<StoredU64>} totalCount
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Spent
 * @property {MetricPattern15<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} MetricsTree_Pools
 * @property {MetricPattern11<PoolSlug>} heightToPool
 * @property {MetricsTree_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} MetricsTree_Pools_Vecs
 * @property {AaopoolPattern} aaopool
 * @property {AaopoolPattern} antpool
 * @property {AaopoolPattern} arkpool
 * @property {AaopoolPattern} asicminer
 * @property {AaopoolPattern} axbt
 * @property {AaopoolPattern} batpool
 * @property {AaopoolPattern} bcmonster
 * @property {AaopoolPattern} bcpoolio
 * @property {AaopoolPattern} binancepool
 * @property {AaopoolPattern} bitalo
 * @property {AaopoolPattern} bitclub
 * @property {AaopoolPattern} bitcoinaffiliatenetwork
 * @property {AaopoolPattern} bitcoincom
 * @property {AaopoolPattern} bitcoinindia
 * @property {AaopoolPattern} bitcoinrussia
 * @property {AaopoolPattern} bitcoinukraine
 * @property {AaopoolPattern} bitfarms
 * @property {AaopoolPattern} bitfufupool
 * @property {AaopoolPattern} bitfury
 * @property {AaopoolPattern} bitminter
 * @property {AaopoolPattern} bitparking
 * @property {AaopoolPattern} bitsolo
 * @property {AaopoolPattern} bixin
 * @property {AaopoolPattern} blockfills
 * @property {AaopoolPattern} braiinspool
 * @property {AaopoolPattern} bravomining
 * @property {AaopoolPattern} btcc
 * @property {AaopoolPattern} btccom
 * @property {AaopoolPattern} btcdig
 * @property {AaopoolPattern} btcguild
 * @property {AaopoolPattern} btclab
 * @property {AaopoolPattern} btcmp
 * @property {AaopoolPattern} btcnuggets
 * @property {AaopoolPattern} btcpoolparty
 * @property {AaopoolPattern} btcserv
 * @property {AaopoolPattern} btctop
 * @property {AaopoolPattern} btpool
 * @property {AaopoolPattern} bwpool
 * @property {AaopoolPattern} bytepool
 * @property {AaopoolPattern} canoe
 * @property {AaopoolPattern} canoepool
 * @property {AaopoolPattern} carbonnegative
 * @property {AaopoolPattern} ckpool
 * @property {AaopoolPattern} cloudhashing
 * @property {AaopoolPattern} coinlab
 * @property {AaopoolPattern} cointerra
 * @property {AaopoolPattern} connectbtc
 * @property {AaopoolPattern} dcex
 * @property {AaopoolPattern} dcexploration
 * @property {AaopoolPattern} digitalbtc
 * @property {AaopoolPattern} digitalxmintsy
 * @property {AaopoolPattern} dpool
 * @property {AaopoolPattern} eclipsemc
 * @property {AaopoolPattern} eightbaochi
 * @property {AaopoolPattern} ekanembtc
 * @property {AaopoolPattern} eligius
 * @property {AaopoolPattern} emcdpool
 * @property {AaopoolPattern} entrustcharitypool
 * @property {AaopoolPattern} eobot
 * @property {AaopoolPattern} exxbw
 * @property {AaopoolPattern} f2pool
 * @property {AaopoolPattern} fiftyeightcoin
 * @property {AaopoolPattern} foundryusa
 * @property {AaopoolPattern} futurebitapollosolo
 * @property {AaopoolPattern} gbminers
 * @property {AaopoolPattern} ghashio
 * @property {AaopoolPattern} givemecoins
 * @property {AaopoolPattern} gogreenlight
 * @property {AaopoolPattern} haominer
 * @property {AaopoolPattern} haozhuzhu
 * @property {AaopoolPattern} hashbx
 * @property {AaopoolPattern} hashpool
 * @property {AaopoolPattern} helix
 * @property {AaopoolPattern} hhtt
 * @property {AaopoolPattern} hotpool
 * @property {AaopoolPattern} hummerpool
 * @property {AaopoolPattern} huobipool
 * @property {AaopoolPattern} innopolistech
 * @property {AaopoolPattern} kanopool
 * @property {AaopoolPattern} kncminer
 * @property {AaopoolPattern} kucoinpool
 * @property {AaopoolPattern} lubiancom
 * @property {AaopoolPattern} luckypool
 * @property {AaopoolPattern} luxor
 * @property {AaopoolPattern} marapool
 * @property {AaopoolPattern} maxbtc
 * @property {AaopoolPattern} maxipool
 * @property {AaopoolPattern} megabigpower
 * @property {AaopoolPattern} minerium
 * @property {AaopoolPattern} miningcity
 * @property {AaopoolPattern} miningdutch
 * @property {AaopoolPattern} miningkings
 * @property {AaopoolPattern} miningsquared
 * @property {AaopoolPattern} mmpool
 * @property {AaopoolPattern} mtred
 * @property {AaopoolPattern} multicoinco
 * @property {AaopoolPattern} multipool
 * @property {AaopoolPattern} mybtccoinpool
 * @property {AaopoolPattern} neopool
 * @property {AaopoolPattern} nexious
 * @property {AaopoolPattern} nicehash
 * @property {AaopoolPattern} nmcbit
 * @property {AaopoolPattern} novablock
 * @property {AaopoolPattern} ocean
 * @property {AaopoolPattern} okexpool
 * @property {AaopoolPattern} okkong
 * @property {AaopoolPattern} okminer
 * @property {AaopoolPattern} okpooltop
 * @property {AaopoolPattern} onehash
 * @property {AaopoolPattern} onem1x
 * @property {AaopoolPattern} onethash
 * @property {AaopoolPattern} ozcoin
 * @property {AaopoolPattern} parasite
 * @property {AaopoolPattern} patels
 * @property {AaopoolPattern} pegapool
 * @property {AaopoolPattern} phashio
 * @property {AaopoolPattern} phoenix
 * @property {AaopoolPattern} polmine
 * @property {AaopoolPattern} pool175btc
 * @property {AaopoolPattern} pool50btc
 * @property {AaopoolPattern} poolin
 * @property {AaopoolPattern} portlandhodl
 * @property {AaopoolPattern} publicpool
 * @property {AaopoolPattern} purebtccom
 * @property {AaopoolPattern} rawpool
 * @property {AaopoolPattern} rigpool
 * @property {AaopoolPattern} sbicrypto
 * @property {AaopoolPattern} secpool
 * @property {AaopoolPattern} secretsuperstar
 * @property {AaopoolPattern} sevenpool
 * @property {AaopoolPattern} shawnp0wers
 * @property {AaopoolPattern} sigmapoolcom
 * @property {AaopoolPattern} simplecoinus
 * @property {AaopoolPattern} solock
 * @property {AaopoolPattern} spiderpool
 * @property {AaopoolPattern} stminingcorp
 * @property {AaopoolPattern} tangpool
 * @property {AaopoolPattern} tatmaspool
 * @property {AaopoolPattern} tbdice
 * @property {AaopoolPattern} telco214
 * @property {AaopoolPattern} terrapool
 * @property {AaopoolPattern} tiger
 * @property {AaopoolPattern} tigerpoolnet
 * @property {AaopoolPattern} titan
 * @property {AaopoolPattern} transactioncoinmining
 * @property {AaopoolPattern} trickysbtcpool
 * @property {AaopoolPattern} triplemining
 * @property {AaopoolPattern} twentyoneinc
 * @property {AaopoolPattern} ultimuspool
 * @property {AaopoolPattern} unknown
 * @property {AaopoolPattern} unomp
 * @property {AaopoolPattern} viabtc
 * @property {AaopoolPattern} waterhole
 * @property {AaopoolPattern} wayicn
 * @property {AaopoolPattern} whitepool
 * @property {AaopoolPattern} wk057
 * @property {AaopoolPattern} yourbtcnet
 * @property {AaopoolPattern} zulupool
 */

/**
 * @typedef {Object} MetricsTree_Positions
 * @property {MetricPattern11<BlkPosition>} blockPosition
 * @property {MetricPattern27<BlkPosition>} txPosition
 */

/**
 * @typedef {Object} MetricsTree_Price
 * @property {MetricsTree_Price_Cents} cents
 * @property {MetricsTree_Price_Oracle} oracle
 * @property {SatsPattern<OHLCSats>} sats
 * @property {MetricsTree_Price_Usd} usd
 */

/**
 * @typedef {Object} MetricsTree_Price_Cents
 * @property {MetricPattern5<OHLCCents>} ohlc
 * @property {MetricsTree_Price_Cents_Split} split
 */

/**
 * @typedef {Object} MetricsTree_Price_Cents_Split
 * @property {MetricPattern5<Cents>} close
 * @property {MetricPattern5<Cents>} high
 * @property {MetricPattern5<Cents>} low
 * @property {MetricPattern5<Cents>} open
 */

/**
 * @typedef {Object} MetricsTree_Price_Oracle
 * @property {MetricPattern11<PairOutputIndex>} heightToFirstPairoutputindex
 * @property {MetricPattern6<OHLCCents>} ohlcCents
 * @property {MetricPattern6<OHLCDollars>} ohlcDollars
 * @property {MetricPattern33<Sats>} output0Value
 * @property {MetricPattern33<Sats>} output1Value
 * @property {MetricPattern33<TxIndex>} pairoutputindexToTxindex
 * @property {PhaseDailyCentsPattern<Cents>} phaseDailyCents
 * @property {PhaseDailyCentsPattern<Dollars>} phaseDailyDollars
 * @property {MetricPattern11<OracleBins>} phaseHistogram
 * @property {MetricPattern11<Cents>} phasePriceCents
 * @property {PhaseDailyCentsPattern<Cents>} phaseV2DailyCents
 * @property {PhaseDailyCentsPattern<Dollars>} phaseV2DailyDollars
 * @property {MetricPattern11<OracleBinsV2>} phaseV2Histogram
 * @property {PhaseDailyCentsPattern<Cents>} phaseV2PeakDailyCents
 * @property {PhaseDailyCentsPattern<Dollars>} phaseV2PeakDailyDollars
 * @property {MetricPattern11<Cents>} phaseV2PeakPriceCents
 * @property {MetricPattern11<Cents>} phaseV2PriceCents
 * @property {PhaseDailyCentsPattern<Cents>} phaseV3DailyCents
 * @property {PhaseDailyCentsPattern<Dollars>} phaseV3DailyDollars
 * @property {MetricPattern11<OracleBinsV2>} phaseV3Histogram
 * @property {PhaseDailyCentsPattern<Cents>} phaseV3PeakDailyCents
 * @property {PhaseDailyCentsPattern<Dollars>} phaseV3PeakDailyDollars
 * @property {MetricPattern11<Cents>} phaseV3PeakPriceCents
 * @property {MetricPattern11<Cents>} phaseV3PriceCents
 * @property {MetricPattern11<Cents>} priceCents
 * @property {MetricPattern6<StoredU32>} txCount
 */

/**
 * @typedef {Object} MetricsTree_Price_Usd
 * @property {MetricPattern1<OHLCDollars>} ohlc
 * @property {SplitPattern2<Dollars>} split
 */

/**
 * @typedef {Object} MetricsTree_Scripts
 * @property {MetricsTree_Scripts_Count} count
 * @property {MetricPattern9<TxIndex>} emptyToTxindex
 * @property {MetricPattern11<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {MetricPattern11<OpReturnIndex>} firstOpreturnindex
 * @property {MetricPattern11<P2MSOutputIndex>} firstP2msoutputindex
 * @property {MetricPattern11<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricPattern14<TxIndex>} opreturnToTxindex
 * @property {MetricPattern17<TxIndex>} p2msToTxindex
 * @property {MetricPattern28<TxIndex>} unknownToTxindex
 * @property {MetricsTree_Scripts_Value} value
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Count
 * @property {DollarsPattern<StoredU64>} emptyoutput
 * @property {DollarsPattern<StoredU64>} opreturn
 * @property {DollarsPattern<StoredU64>} p2a
 * @property {DollarsPattern<StoredU64>} p2ms
 * @property {DollarsPattern<StoredU64>} p2pk33
 * @property {DollarsPattern<StoredU64>} p2pk65
 * @property {DollarsPattern<StoredU64>} p2pkh
 * @property {DollarsPattern<StoredU64>} p2sh
 * @property {DollarsPattern<StoredU64>} p2tr
 * @property {DollarsPattern<StoredU64>} p2wpkh
 * @property {DollarsPattern<StoredU64>} p2wsh
 * @property {DollarsPattern<StoredU64>} segwit
 * @property {SegwitAdoptionPattern} segwitAdoption
 * @property {SegwitAdoptionPattern} taprootAdoption
 * @property {DollarsPattern<StoredU64>} unknownoutput
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value
 * @property {CoinbasePattern} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Supply
 * @property {MetricsTree_Supply_Burned} burned
 * @property {MetricsTree_Supply_Circulating} circulating
 * @property {MetricPattern4<StoredF32>} inflation
 * @property {MetricPattern1<Dollars>} marketCap
 * @property {MetricsTree_Supply_Velocity} velocity
 */

/**
 * @typedef {Object} MetricsTree_Supply_Burned
 * @property {UnclaimedRewardsPattern} opreturn
 * @property {UnclaimedRewardsPattern} unspendable
 */

/**
 * @typedef {Object} MetricsTree_Supply_Circulating
 * @property {MetricPattern3<Bitcoin>} bitcoin
 * @property {MetricPattern3<Dollars>} dollars
 * @property {MetricPattern3<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Supply_Velocity
 * @property {MetricPattern4<StoredF64>} btc
 * @property {MetricPattern4<StoredF64>} usd
 */

/**
 * @typedef {Object} MetricsTree_Transactions
 * @property {MetricPattern27<StoredU32>} baseSize
 * @property {MetricsTree_Transactions_Count} count
 * @property {MetricsTree_Transactions_Fees} fees
 * @property {MetricPattern11<TxIndex>} firstTxindex
 * @property {MetricPattern27<TxInIndex>} firstTxinindex
 * @property {MetricPattern27<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern27<Height>} height
 * @property {MetricPattern27<StoredBool>} isExplicitlyRbf
 * @property {MetricPattern27<RawLockTime>} rawlocktime
 * @property {MetricsTree_Transactions_Size} size
 * @property {MetricPattern27<StoredU32>} totalSize
 * @property {MetricPattern27<Txid>} txid
 * @property {MetricPattern27<TxVersion>} txversion
 * @property {MetricsTree_Transactions_Versions} versions
 * @property {MetricsTree_Transactions_Volume} volume
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Count
 * @property {MetricPattern27<StoredBool>} isCoinbase
 * @property {DollarsPattern<StoredU64>} txCount
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees
 * @property {MetricsTree_Transactions_Fees_Fee} fee
 * @property {FeeRatePattern<FeeRate>} feeRate
 * @property {MetricPattern27<Sats>} inputValue
 * @property {MetricPattern27<Sats>} outputValue
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees_Fee
 * @property {CountPattern2<Bitcoin>} bitcoin
 * @property {MetricsTree_Transactions_Fees_Fee_Dollars} dollars
 * @property {CountPattern2<Sats>} sats
 * @property {MetricPattern27<Sats>} txindex
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees_Fee_Dollars
 * @property {MetricPattern1<Dollars>} average
 * @property {MetricPattern2<Dollars>} cumulative
 * @property {MetricPattern11<Dollars>} heightCumulative
 * @property {MetricPattern1<Dollars>} max
 * @property {MetricPattern11<Dollars>} median
 * @property {MetricPattern1<Dollars>} min
 * @property {MetricPattern11<Dollars>} pct10
 * @property {MetricPattern11<Dollars>} pct25
 * @property {MetricPattern11<Dollars>} pct75
 * @property {MetricPattern11<Dollars>} pct90
 * @property {MetricPattern1<Dollars>} sum
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Size
 * @property {FeeRatePattern<VSize>} vsize
 * @property {FeeRatePattern<Weight>} weight
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Versions
 * @property {BlockCountPattern<StoredU64>} v1
 * @property {BlockCountPattern<StoredU64>} v2
 * @property {BlockCountPattern<StoredU64>} v3
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Volume
 * @property {_2015Pattern} annualizedVolume
 * @property {MetricPattern4<StoredF32>} inputsPerSec
 * @property {MetricPattern4<StoredF32>} outputsPerSec
 * @property {ActiveSupplyPattern} sentSum
 * @property {MetricPattern4<StoredF32>} txPerSec
 */

/**
 * Main BRK client with metrics tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
  VERSION = "v0.1.0-alpha.6";

  INDEXES = /** @type {const} */ ([
    "dateindex",
    "decadeindex",
    "difficultyepoch",
    "emptyoutputindex",
    "halvingepoch",
    "height",
    "txinindex",
    "monthindex",
    "opreturnindex",
    "txoutindex",
    "p2aaddressindex",
    "p2msoutputindex",
    "p2pk33addressindex",
    "p2pk65addressindex",
    "p2pkhaddressindex",
    "p2shaddressindex",
    "p2traddressindex",
    "p2wpkhaddressindex",
    "p2wshaddressindex",
    "quarterindex",
    "semesterindex",
    "txindex",
    "unknownoutputindex",
    "weekindex",
    "yearindex",
    "loadedaddressindex",
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
    "ekanembtc": "EkanemBTC",
    "canoe": "CANOE",
    "tiger": "tiger",
    "onem1x": "1M1X",
    "zulupool": "Zulupool",
    "secpool": "SECPOOL",
    "ocean": "OCEAN",
    "whitepool": "WhitePool",
    "wk057": "wk057",
    "futurebitapollosolo": "FutureBit Apollo Solo",
    "carbonnegative": "Carbon Negative",
    "portlandhodl": "Portland.HODL",
    "phoenix": "Phoenix",
    "neopool": "Neopool",
    "maxipool": "MaxiPool",
    "bitfufupool": "BitFuFuPool",
    "luckypool": "luckyPool",
    "miningdutch": "Mining-Dutch",
    "publicpool": "Public Pool",
    "miningsquared": "Mining Squared",
    "innopolistech": "Innopolis Tech",
    "btclab": "BTCLab",
    "parasite": "Parasite"
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
      "short": "Epoch 0",
      "long": "Epoch 0"
    },
    "_1": {
      "id": "epoch_1",
      "short": "Epoch 1",
      "long": "Epoch 1"
    },
    "_2": {
      "id": "epoch_2",
      "short": "Epoch 2",
      "long": "Epoch 2"
    },
    "_3": {
      "id": "epoch_3",
      "short": "Epoch 3",
      "long": "Epoch 3"
    },
    "_4": {
      "id": "epoch_4",
      "short": "Epoch 4",
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
      "id": "up_to_1h_old",
      "short": "<1h",
      "long": "Up to 1 Hour Old"
    },
    "_1hTo1d": {
      "id": "at_least_1h_up_to_1d_old",
      "short": "1h-1d",
      "long": "1 Hour to 1 Day Old"
    },
    "_1dTo1w": {
      "id": "at_least_1d_up_to_1w_old",
      "short": "1d-1w",
      "long": "1 Day to 1 Week Old"
    },
    "_1wTo1m": {
      "id": "at_least_1w_up_to_1m_old",
      "short": "1w-1m",
      "long": "1 Week to 1 Month Old"
    },
    "_1mTo2m": {
      "id": "at_least_1m_up_to_2m_old",
      "short": "1m-2m",
      "long": "1 to 2 Months Old"
    },
    "_2mTo3m": {
      "id": "at_least_2m_up_to_3m_old",
      "short": "2m-3m",
      "long": "2 to 3 Months Old"
    },
    "_3mTo4m": {
      "id": "at_least_3m_up_to_4m_old",
      "short": "3m-4m",
      "long": "3 to 4 Months Old"
    },
    "_4mTo5m": {
      "id": "at_least_4m_up_to_5m_old",
      "short": "4m-5m",
      "long": "4 to 5 Months Old"
    },
    "_5mTo6m": {
      "id": "at_least_5m_up_to_6m_old",
      "short": "5m-6m",
      "long": "5 to 6 Months Old"
    },
    "_6mTo1y": {
      "id": "at_least_6m_up_to_1y_old",
      "short": "6m-1y",
      "long": "6 Months to 1 Year Old"
    },
    "_1yTo2y": {
      "id": "at_least_1y_up_to_2y_old",
      "short": "1y-2y",
      "long": "1 to 2 Years Old"
    },
    "_2yTo3y": {
      "id": "at_least_2y_up_to_3y_old",
      "short": "2y-3y",
      "long": "2 to 3 Years Old"
    },
    "_3yTo4y": {
      "id": "at_least_3y_up_to_4y_old",
      "short": "3y-4y",
      "long": "3 to 4 Years Old"
    },
    "_4yTo5y": {
      "id": "at_least_4y_up_to_5y_old",
      "short": "4y-5y",
      "long": "4 to 5 Years Old"
    },
    "_5yTo6y": {
      "id": "at_least_5y_up_to_6y_old",
      "short": "5y-6y",
      "long": "5 to 6 Years Old"
    },
    "_6yTo7y": {
      "id": "at_least_6y_up_to_7y_old",
      "short": "6y-7y",
      "long": "6 to 7 Years Old"
    },
    "_7yTo8y": {
      "id": "at_least_7y_up_to_8y_old",
      "short": "7y-8y",
      "long": "7 to 8 Years Old"
    },
    "_8yTo10y": {
      "id": "at_least_8y_up_to_10y_old",
      "short": "8y-10y",
      "long": "8 to 10 Years Old"
    },
    "_10yTo12y": {
      "id": "at_least_10y_up_to_12y_old",
      "short": "10y-12y",
      "long": "10 to 12 Years Old"
    },
    "_12yTo15y": {
      "id": "at_least_12y_up_to_15y_old",
      "short": "12y-15y",
      "long": "12 to 15 Years Old"
    },
    "from15y": {
      "id": "at_least_15y_old",
      "short": "15y+",
      "long": "15+ Years Old"
    }
  });

  MAX_AGE_NAMES = /** @type {const} */ ({
    "_1w": {
      "id": "up_to_1w_old",
      "short": "<1w",
      "long": "Up to 1 Week Old"
    },
    "_1m": {
      "id": "up_to_1m_old",
      "short": "<1m",
      "long": "Up to 1 Month Old"
    },
    "_2m": {
      "id": "up_to_2m_old",
      "short": "<2m",
      "long": "Up to 2 Months Old"
    },
    "_3m": {
      "id": "up_to_3m_old",
      "short": "<3m",
      "long": "Up to 3 Months Old"
    },
    "_4m": {
      "id": "up_to_4m_old",
      "short": "<4m",
      "long": "Up to 4 Months Old"
    },
    "_5m": {
      "id": "up_to_5m_old",
      "short": "<5m",
      "long": "Up to 5 Months Old"
    },
    "_6m": {
      "id": "up_to_6m_old",
      "short": "<6m",
      "long": "Up to 6 Months Old"
    },
    "_1y": {
      "id": "up_to_1y_old",
      "short": "<1y",
      "long": "Up to 1 Year Old"
    },
    "_2y": {
      "id": "up_to_2y_old",
      "short": "<2y",
      "long": "Up to 2 Years Old"
    },
    "_3y": {
      "id": "up_to_3y_old",
      "short": "<3y",
      "long": "Up to 3 Years Old"
    },
    "_4y": {
      "id": "up_to_4y_old",
      "short": "<4y",
      "long": "Up to 4 Years Old"
    },
    "_5y": {
      "id": "up_to_5y_old",
      "short": "<5y",
      "long": "Up to 5 Years Old"
    },
    "_6y": {
      "id": "up_to_6y_old",
      "short": "<6y",
      "long": "Up to 6 Years Old"
    },
    "_7y": {
      "id": "up_to_7y_old",
      "short": "<7y",
      "long": "Up to 7 Years Old"
    },
    "_8y": {
      "id": "up_to_8y_old",
      "short": "<8y",
      "long": "Up to 8 Years Old"
    },
    "_10y": {
      "id": "up_to_10y_old",
      "short": "<10y",
      "long": "Up to 10 Years Old"
    },
    "_12y": {
      "id": "up_to_12y_old",
      "short": "<12y",
      "long": "Up to 12 Years Old"
    },
    "_15y": {
      "id": "up_to_15y_old",
      "short": "<15y",
      "long": "Up to 15 Years Old"
    }
  });

  MIN_AGE_NAMES = /** @type {const} */ ({
    "_1d": {
      "id": "at_least_1d_old",
      "short": "1d+",
      "long": "At Least 1 Day Old"
    },
    "_1w": {
      "id": "at_least_1w_old",
      "short": "1w+",
      "long": "At Least 1 Week Old"
    },
    "_1m": {
      "id": "at_least_1m_old",
      "short": "1m+",
      "long": "At Least 1 Month Old"
    },
    "_2m": {
      "id": "at_least_2m_old",
      "short": "2m+",
      "long": "At Least 2 Months Old"
    },
    "_3m": {
      "id": "at_least_3m_old",
      "short": "3m+",
      "long": "At Least 3 Months Old"
    },
    "_4m": {
      "id": "at_least_4m_old",
      "short": "4m+",
      "long": "At Least 4 Months Old"
    },
    "_5m": {
      "id": "at_least_5m_old",
      "short": "5m+",
      "long": "At Least 5 Months Old"
    },
    "_6m": {
      "id": "at_least_6m_old",
      "short": "6m+",
      "long": "At Least 6 Months Old"
    },
    "_1y": {
      "id": "at_least_1y_old",
      "short": "1y+",
      "long": "At Least 1 Year Old"
    },
    "_2y": {
      "id": "at_least_2y_old",
      "short": "2y+",
      "long": "At Least 2 Years Old"
    },
    "_3y": {
      "id": "at_least_3y_old",
      "short": "3y+",
      "long": "At Least 3 Years Old"
    },
    "_4y": {
      "id": "at_least_4y_old",
      "short": "4y+",
      "long": "At Least 4 Years Old"
    },
    "_5y": {
      "id": "at_least_5y_old",
      "short": "5y+",
      "long": "At Least 5 Years Old"
    },
    "_6y": {
      "id": "at_least_6y_old",
      "short": "6y+",
      "long": "At Least 6 Years Old"
    },
    "_7y": {
      "id": "at_least_7y_old",
      "short": "7y+",
      "long": "At Least 7 Years Old"
    },
    "_8y": {
      "id": "at_least_8y_old",
      "short": "8y+",
      "long": "At Least 8 Years Old"
    },
    "_10y": {
      "id": "at_least_10y_old",
      "short": "10y+",
      "long": "At Least 10 Years Old"
    },
    "_12y": {
      "id": "at_least_12y_old",
      "short": "12y+",
      "long": "At Least 12 Years Old"
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
      "long": "1 to 10 Sats"
    },
    "_10satsTo100sats": {
      "id": "above_10sats_under_100sats",
      "short": "10-100 sats",
      "long": "10 to 100 Sats"
    },
    "_100satsTo1kSats": {
      "id": "above_100sats_under_1k_sats",
      "short": "100-1k sats",
      "long": "100 to 1K Sats"
    },
    "_1kSatsTo10kSats": {
      "id": "above_1k_sats_under_10k_sats",
      "short": "1k-10k sats",
      "long": "1K to 10K Sats"
    },
    "_10kSatsTo100kSats": {
      "id": "above_10k_sats_under_100k_sats",
      "short": "10k-100k sats",
      "long": "10K to 100K Sats"
    },
    "_100kSatsTo1mSats": {
      "id": "above_100k_sats_under_1m_sats",
      "short": "100k-1M sats",
      "long": "100K to 1M Sats"
    },
    "_1mSatsTo10mSats": {
      "id": "above_1m_sats_under_10m_sats",
      "short": "1M-10M sats",
      "long": "1M to 10M Sats"
    },
    "_10mSatsTo1btc": {
      "id": "above_10m_sats_under_1btc",
      "short": "0.1-1 BTC",
      "long": "0.1 to 1 BTC"
    },
    "_1btcTo10btc": {
      "id": "above_1btc_under_10btc",
      "short": "1-10 BTC",
      "long": "1 to 10 BTC"
    },
    "_10btcTo100btc": {
      "id": "above_10btc_under_100btc",
      "short": "10-100 BTC",
      "long": "10 to 100 BTC"
    },
    "_100btcTo1kBtc": {
      "id": "above_100btc_under_1k_btc",
      "short": "100-1k BTC",
      "long": "100 to 1K BTC"
    },
    "_1kBtcTo10kBtc": {
      "id": "above_1k_btc_under_10k_btc",
      "short": "1k-10k BTC",
      "long": "1K to 10K BTC"
    },
    "_10kBtcTo100kBtc": {
      "id": "above_10k_btc_under_100k_btc",
      "short": "10k-100k BTC",
      "long": "10K to 100K BTC"
    },
    "_100kBtcOrMore": {
      "id": "above_100k_btc",
      "short": "100k+ BTC",
      "long": "100K+ BTC"
    }
  });

  GE_AMOUNT_NAMES = /** @type {const} */ ({
    "_1sat": {
      "id": "above_1sat",
      "short": "1+ sats",
      "long": "Above 1 Sat"
    },
    "_10sats": {
      "id": "above_10sats",
      "short": "10+ sats",
      "long": "Above 10 Sats"
    },
    "_100sats": {
      "id": "above_100sats",
      "short": "100+ sats",
      "long": "Above 100 Sats"
    },
    "_1kSats": {
      "id": "above_1k_sats",
      "short": "1k+ sats",
      "long": "Above 1K Sats"
    },
    "_10kSats": {
      "id": "above_10k_sats",
      "short": "10k+ sats",
      "long": "Above 10K Sats"
    },
    "_100kSats": {
      "id": "above_100k_sats",
      "short": "100k+ sats",
      "long": "Above 100K Sats"
    },
    "_1mSats": {
      "id": "above_1m_sats",
      "short": "1M+ sats",
      "long": "Above 1M Sats"
    },
    "_10mSats": {
      "id": "above_10m_sats",
      "short": "0.1+ BTC",
      "long": "Above 0.1 BTC"
    },
    "_1btc": {
      "id": "above_1btc",
      "short": "1+ BTC",
      "long": "Above 1 BTC"
    },
    "_10btc": {
      "id": "above_10btc",
      "short": "10+ BTC",
      "long": "Above 10 BTC"
    },
    "_100btc": {
      "id": "above_100btc",
      "short": "100+ BTC",
      "long": "Above 100 BTC"
    },
    "_1kBtc": {
      "id": "above_1k_btc",
      "short": "1k+ BTC",
      "long": "Above 1K BTC"
    },
    "_10kBtc": {
      "id": "above_10k_btc",
      "short": "10k+ BTC",
      "long": "Above 10K BTC"
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
      addresses: {
        firstP2aaddressindex: createMetricPattern11(this, 'first_p2aaddressindex'),
        firstP2pk33addressindex: createMetricPattern11(this, 'first_p2pk33addressindex'),
        firstP2pk65addressindex: createMetricPattern11(this, 'first_p2pk65addressindex'),
        firstP2pkhaddressindex: createMetricPattern11(this, 'first_p2pkhaddressindex'),
        firstP2shaddressindex: createMetricPattern11(this, 'first_p2shaddressindex'),
        firstP2traddressindex: createMetricPattern11(this, 'first_p2traddressindex'),
        firstP2wpkhaddressindex: createMetricPattern11(this, 'first_p2wpkhaddressindex'),
        firstP2wshaddressindex: createMetricPattern11(this, 'first_p2wshaddressindex'),
        p2abytes: createMetricPattern16(this, 'p2abytes'),
        p2pk33bytes: createMetricPattern18(this, 'p2pk33bytes'),
        p2pk65bytes: createMetricPattern19(this, 'p2pk65bytes'),
        p2pkhbytes: createMetricPattern20(this, 'p2pkhbytes'),
        p2shbytes: createMetricPattern21(this, 'p2shbytes'),
        p2trbytes: createMetricPattern22(this, 'p2trbytes'),
        p2wpkhbytes: createMetricPattern23(this, 'p2wpkhbytes'),
        p2wshbytes: createMetricPattern24(this, 'p2wshbytes'),
      },
      blocks: {
        blockhash: createMetricPattern11(this, 'blockhash'),
        count: {
          _1mBlockCount: createMetricPattern1(this, '1m_block_count'),
          _1mStart: createMetricPattern11(this, '1m_start'),
          _1wBlockCount: createMetricPattern1(this, '1w_block_count'),
          _1wStart: createMetricPattern11(this, '1w_start'),
          _1yBlockCount: createMetricPattern1(this, '1y_block_count'),
          _1yStart: createMetricPattern11(this, '1y_start'),
          _24hBlockCount: createMetricPattern1(this, '24h_block_count'),
          _24hStart: createMetricPattern11(this, '24h_start'),
          blockCount: createBlockCountPattern(this, 'block_count'),
          blockCountTarget: createMetricPattern4(this, 'block_count_target'),
        },
        difficulty: {
          adjustment: createMetricPattern1(this, 'difficulty_adjustment'),
          asHash: createMetricPattern1(this, 'difficulty_as_hash'),
          blocksBeforeNextAdjustment: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
          daysBeforeNextAdjustment: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
          epoch: createMetricPattern4(this, 'difficultyepoch'),
          raw: createMetricPattern1(this, 'difficulty'),
        },
        fullness: createFullnessPattern(this, 'block_fullness'),
        halving: {
          blocksBeforeNextHalving: createMetricPattern1(this, 'blocks_before_next_halving'),
          daysBeforeNextHalving: createMetricPattern1(this, 'days_before_next_halving'),
          epoch: createMetricPattern4(this, 'halvingepoch'),
        },
        interval: createFullnessPattern(this, 'block_interval'),
        mining: {
          hashPricePhs: createMetricPattern1(this, 'hash_price_phs'),
          hashPricePhsMin: createMetricPattern1(this, 'hash_price_phs_min'),
          hashPriceRebound: createMetricPattern1(this, 'hash_price_rebound'),
          hashPriceThs: createMetricPattern1(this, 'hash_price_ths'),
          hashPriceThsMin: createMetricPattern1(this, 'hash_price_ths_min'),
          hashRate: createMetricPattern1(this, 'hash_rate'),
          hashRate1mSma: createMetricPattern4(this, 'hash_rate_1m_sma'),
          hashRate1wSma: createMetricPattern4(this, 'hash_rate_1w_sma'),
          hashRate1ySma: createMetricPattern4(this, 'hash_rate_1y_sma'),
          hashRate2mSma: createMetricPattern4(this, 'hash_rate_2m_sma'),
          hashValuePhs: createMetricPattern1(this, 'hash_value_phs'),
          hashValuePhsMin: createMetricPattern1(this, 'hash_value_phs_min'),
          hashValueRebound: createMetricPattern1(this, 'hash_value_rebound'),
          hashValueThs: createMetricPattern1(this, 'hash_value_ths'),
          hashValueThsMin: createMetricPattern1(this, 'hash_value_ths_min'),
        },
        rewards: {
          _24hCoinbaseSum: {
            bitcoin: createMetricPattern11(this, '24h_coinbase_sum_btc'),
            dollars: createMetricPattern11(this, '24h_coinbase_sum_usd'),
            sats: createMetricPattern11(this, '24h_coinbase_sum'),
          },
          coinbase: createCoinbasePattern(this, 'coinbase'),
          feeDominance: createMetricPattern6(this, 'fee_dominance'),
          subsidy: createCoinbasePattern(this, 'subsidy'),
          subsidyDominance: createMetricPattern6(this, 'subsidy_dominance'),
          subsidyUsd1ySma: createMetricPattern4(this, 'subsidy_usd_1y_sma'),
          unclaimedRewards: createUnclaimedRewardsPattern(this, 'unclaimed_rewards'),
        },
        size: {
          average: createMetricPattern2(this, 'block_size_average'),
          cumulative: createMetricPattern1(this, 'block_size_cumulative'),
          max: createMetricPattern2(this, 'block_size_max'),
          median: createMetricPattern6(this, 'block_size_median'),
          min: createMetricPattern2(this, 'block_size_min'),
          pct10: createMetricPattern6(this, 'block_size_pct10'),
          pct25: createMetricPattern6(this, 'block_size_pct25'),
          pct75: createMetricPattern6(this, 'block_size_pct75'),
          pct90: createMetricPattern6(this, 'block_size_pct90'),
          sum: createMetricPattern2(this, 'block_size_sum'),
        },
        time: {
          date: createMetricPattern11(this, 'date'),
          timestamp: createMetricPattern1(this, 'timestamp'),
          timestampMonotonic: createMetricPattern11(this, 'timestamp_monotonic'),
        },
        totalSize: createMetricPattern11(this, 'total_size'),
        vbytes: createDollarsPattern(this, 'block_vbytes'),
        weight: createDollarsPattern(this, 'block_weight'),
      },
      cointime: {
        activity: {
          activityToVaultednessRatio: createMetricPattern1(this, 'activity_to_vaultedness_ratio'),
          coinblocksCreated: createBlockCountPattern(this, 'coinblocks_created'),
          coinblocksStored: createBlockCountPattern(this, 'coinblocks_stored'),
          liveliness: createMetricPattern1(this, 'liveliness'),
          vaultedness: createMetricPattern1(this, 'vaultedness'),
        },
        adjusted: {
          cointimeAdjInflationRate: createMetricPattern4(this, 'cointime_adj_inflation_rate'),
          cointimeAdjTxBtcVelocity: createMetricPattern4(this, 'cointime_adj_tx_btc_velocity'),
          cointimeAdjTxUsdVelocity: createMetricPattern4(this, 'cointime_adj_tx_usd_velocity'),
        },
        cap: {
          activeCap: createMetricPattern1(this, 'active_cap'),
          cointimeCap: createMetricPattern1(this, 'cointime_cap'),
          investorCap: createMetricPattern1(this, 'investor_cap'),
          thermoCap: createMetricPattern1(this, 'thermo_cap'),
          vaultedCap: createMetricPattern1(this, 'vaulted_cap'),
        },
        pricing: {
          activePrice: createMetricPattern1(this, 'active_price'),
          activePriceRatio: createActivePriceRatioPattern(this, 'active_price_ratio'),
          cointimePrice: createMetricPattern1(this, 'cointime_price'),
          cointimePriceRatio: createActivePriceRatioPattern(this, 'cointime_price_ratio'),
          trueMarketMean: createMetricPattern1(this, 'true_market_mean'),
          trueMarketMeanRatio: createActivePriceRatioPattern(this, 'true_market_mean_ratio'),
          vaultedPrice: createMetricPattern1(this, 'vaulted_price'),
          vaultedPriceRatio: createActivePriceRatioPattern(this, 'vaulted_price_ratio'),
        },
        supply: {
          activeSupply: createActiveSupplyPattern(this, 'active_supply'),
          vaultedSupply: createActiveSupplyPattern(this, 'vaulted_supply'),
        },
        value: {
          cointimeValueCreated: createBlockCountPattern(this, 'cointime_value_created'),
          cointimeValueDestroyed: createBlockCountPattern(this, 'cointime_value_destroyed'),
          cointimeValueStored: createBlockCountPattern(this, 'cointime_value_stored'),
        },
      },
      constants: {
        constant0: createMetricPattern1(this, 'constant_0'),
        constant1: createMetricPattern1(this, 'constant_1'),
        constant100: createMetricPattern1(this, 'constant_100'),
        constant2: createMetricPattern1(this, 'constant_2'),
        constant20: createMetricPattern1(this, 'constant_20'),
        constant3: createMetricPattern1(this, 'constant_3'),
        constant30: createMetricPattern1(this, 'constant_30'),
        constant382: createMetricPattern1(this, 'constant_38_2'),
        constant4: createMetricPattern1(this, 'constant_4'),
        constant50: createMetricPattern1(this, 'constant_50'),
        constant600: createMetricPattern1(this, 'constant_600'),
        constant618: createMetricPattern1(this, 'constant_61_8'),
        constant70: createMetricPattern1(this, 'constant_70'),
        constant80: createMetricPattern1(this, 'constant_80'),
        constantMinus1: createMetricPattern1(this, 'constant_minus_1'),
        constantMinus2: createMetricPattern1(this, 'constant_minus_2'),
        constantMinus3: createMetricPattern1(this, 'constant_minus_3'),
        constantMinus4: createMetricPattern1(this, 'constant_minus_4'),
      },
      distribution: {
        addrCount: createAddrCountPattern(this, 'addr_count'),
        addressCohorts: {
          amountRange: {
            _0sats: create_0satsPattern(this, 'addrs_with_0sats'),
            _100btcTo1kBtc: create_0satsPattern(this, 'addrs_above_100btc_under_1k_btc'),
            _100kBtcOrMore: create_0satsPattern(this, 'addrs_above_100k_btc'),
            _100kSatsTo1mSats: create_0satsPattern(this, 'addrs_above_100k_sats_under_1m_sats'),
            _100satsTo1kSats: create_0satsPattern(this, 'addrs_above_100sats_under_1k_sats'),
            _10btcTo100btc: create_0satsPattern(this, 'addrs_above_10btc_under_100btc'),
            _10kBtcTo100kBtc: create_0satsPattern(this, 'addrs_above_10k_btc_under_100k_btc'),
            _10kSatsTo100kSats: create_0satsPattern(this, 'addrs_above_10k_sats_under_100k_sats'),
            _10mSatsTo1btc: create_0satsPattern(this, 'addrs_above_10m_sats_under_1btc'),
            _10satsTo100sats: create_0satsPattern(this, 'addrs_above_10sats_under_100sats'),
            _1btcTo10btc: create_0satsPattern(this, 'addrs_above_1btc_under_10btc'),
            _1kBtcTo10kBtc: create_0satsPattern(this, 'addrs_above_1k_btc_under_10k_btc'),
            _1kSatsTo10kSats: create_0satsPattern(this, 'addrs_above_1k_sats_under_10k_sats'),
            _1mSatsTo10mSats: create_0satsPattern(this, 'addrs_above_1m_sats_under_10m_sats'),
            _1satTo10sats: create_0satsPattern(this, 'addrs_above_1sat_under_10sats'),
          },
          geAmount: {
            _100btc: create_0satsPattern(this, 'addrs_above_100btc'),
            _100kSats: create_0satsPattern(this, 'addrs_above_100k_sats'),
            _100sats: create_0satsPattern(this, 'addrs_above_100sats'),
            _10btc: create_0satsPattern(this, 'addrs_above_10btc'),
            _10kBtc: create_0satsPattern(this, 'addrs_above_10k_btc'),
            _10kSats: create_0satsPattern(this, 'addrs_above_10k_sats'),
            _10mSats: create_0satsPattern(this, 'addrs_above_10m_sats'),
            _10sats: create_0satsPattern(this, 'addrs_above_10sats'),
            _1btc: create_0satsPattern(this, 'addrs_above_1btc'),
            _1kBtc: create_0satsPattern(this, 'addrs_above_1k_btc'),
            _1kSats: create_0satsPattern(this, 'addrs_above_1k_sats'),
            _1mSats: create_0satsPattern(this, 'addrs_above_1m_sats'),
            _1sat: create_0satsPattern(this, 'addrs_above_1sat'),
          },
          ltAmount: {
            _100btc: create_0satsPattern(this, 'addrs_under_100btc'),
            _100kBtc: create_0satsPattern(this, 'addrs_under_100k_btc'),
            _100kSats: create_0satsPattern(this, 'addrs_under_100k_sats'),
            _100sats: create_0satsPattern(this, 'addrs_under_100sats'),
            _10btc: create_0satsPattern(this, 'addrs_under_10btc'),
            _10kBtc: create_0satsPattern(this, 'addrs_under_10k_btc'),
            _10kSats: create_0satsPattern(this, 'addrs_under_10k_sats'),
            _10mSats: create_0satsPattern(this, 'addrs_under_10m_sats'),
            _10sats: create_0satsPattern(this, 'addrs_under_10sats'),
            _1btc: create_0satsPattern(this, 'addrs_under_1btc'),
            _1kBtc: create_0satsPattern(this, 'addrs_under_1k_btc'),
            _1kSats: create_0satsPattern(this, 'addrs_under_1k_sats'),
            _1mSats: create_0satsPattern(this, 'addrs_under_1m_sats'),
          },
        },
        addressesData: {
          empty: createMetricPattern32(this, 'emptyaddressdata'),
          loaded: createMetricPattern31(this, 'loadedaddressdata'),
        },
        anyAddressIndexes: {
          p2a: createMetricPattern16(this, 'anyaddressindex'),
          p2pk33: createMetricPattern18(this, 'anyaddressindex'),
          p2pk65: createMetricPattern19(this, 'anyaddressindex'),
          p2pkh: createMetricPattern20(this, 'anyaddressindex'),
          p2sh: createMetricPattern21(this, 'anyaddressindex'),
          p2tr: createMetricPattern22(this, 'anyaddressindex'),
          p2wpkh: createMetricPattern23(this, 'anyaddressindex'),
          p2wsh: createMetricPattern24(this, 'anyaddressindex'),
        },
        chainState: createMetricPattern11(this, 'chain'),
        emptyAddrCount: createAddrCountPattern(this, 'empty_addr_count'),
        emptyaddressindex: createMetricPattern32(this, 'emptyaddressindex'),
        loadedaddressindex: createMetricPattern31(this, 'loadedaddressindex'),
        utxoCohorts: {
          ageRange: {
            _10yTo12y: create_10yTo12yPattern(this, 'utxos_at_least_10y_up_to_12y_old'),
            _12yTo15y: create_10yTo12yPattern(this, 'utxos_at_least_12y_up_to_15y_old'),
            _1dTo1w: create_10yTo12yPattern(this, 'utxos_at_least_1d_up_to_1w_old'),
            _1hTo1d: create_10yTo12yPattern(this, 'utxos_at_least_1h_up_to_1d_old'),
            _1mTo2m: create_10yTo12yPattern(this, 'utxos_at_least_1m_up_to_2m_old'),
            _1wTo1m: create_10yTo12yPattern(this, 'utxos_at_least_1w_up_to_1m_old'),
            _1yTo2y: create_10yTo12yPattern(this, 'utxos_at_least_1y_up_to_2y_old'),
            _2mTo3m: create_10yTo12yPattern(this, 'utxos_at_least_2m_up_to_3m_old'),
            _2yTo3y: create_10yTo12yPattern(this, 'utxos_at_least_2y_up_to_3y_old'),
            _3mTo4m: create_10yTo12yPattern(this, 'utxos_at_least_3m_up_to_4m_old'),
            _3yTo4y: create_10yTo12yPattern(this, 'utxos_at_least_3y_up_to_4y_old'),
            _4mTo5m: create_10yTo12yPattern(this, 'utxos_at_least_4m_up_to_5m_old'),
            _4yTo5y: create_10yTo12yPattern(this, 'utxos_at_least_4y_up_to_5y_old'),
            _5mTo6m: create_10yTo12yPattern(this, 'utxos_at_least_5m_up_to_6m_old'),
            _5yTo6y: create_10yTo12yPattern(this, 'utxos_at_least_5y_up_to_6y_old'),
            _6mTo1y: create_10yTo12yPattern(this, 'utxos_at_least_6m_up_to_1y_old'),
            _6yTo7y: create_10yTo12yPattern(this, 'utxos_at_least_6y_up_to_7y_old'),
            _7yTo8y: create_10yTo12yPattern(this, 'utxos_at_least_7y_up_to_8y_old'),
            _8yTo10y: create_10yTo12yPattern(this, 'utxos_at_least_8y_up_to_10y_old'),
            from15y: create_10yTo12yPattern(this, 'utxos_at_least_15y_old'),
            upTo1h: create_10yTo12yPattern(this, 'utxos_up_to_1h_old'),
          },
          all: {
            activity: createActivityPattern2(this, ''),
            costBasis: {
              max: createMetricPattern1(this, 'max_cost_basis'),
              min: createMetricPattern1(this, 'min_cost_basis'),
              percentiles: createPercentilesPattern(this, 'cost_basis'),
            },
            outputs: createOutputsPattern(this, 'utxo_count'),
            realized: createRealizedPattern3(this, ''),
            relative: {
              negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
              netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
              supplyInLossRelToOwnSupply: createMetricPattern1(this, 'supply_in_loss_rel_to_own_supply'),
              supplyInProfitRelToOwnSupply: createMetricPattern1(this, 'supply_in_profit_rel_to_own_supply'),
              unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_loss_rel_to_own_total_unrealized_pnl'),
              unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_profit_rel_to_own_total_unrealized_pnl'),
            },
            supply: createSupplyPattern2(this, 'supply'),
            unrealized: createUnrealizedPattern(this, ''),
          },
          amountRange: {
            _0sats: create_0satsPattern2(this, 'utxos_with_0sats'),
            _100btcTo1kBtc: create_0satsPattern2(this, 'utxos_above_100btc_under_1k_btc'),
            _100kBtcOrMore: create_0satsPattern2(this, 'utxos_above_100k_btc'),
            _100kSatsTo1mSats: create_0satsPattern2(this, 'utxos_above_100k_sats_under_1m_sats'),
            _100satsTo1kSats: create_0satsPattern2(this, 'utxos_above_100sats_under_1k_sats'),
            _10btcTo100btc: create_0satsPattern2(this, 'utxos_above_10btc_under_100btc'),
            _10kBtcTo100kBtc: create_0satsPattern2(this, 'utxos_above_10k_btc_under_100k_btc'),
            _10kSatsTo100kSats: create_0satsPattern2(this, 'utxos_above_10k_sats_under_100k_sats'),
            _10mSatsTo1btc: create_0satsPattern2(this, 'utxos_above_10m_sats_under_1btc'),
            _10satsTo100sats: create_0satsPattern2(this, 'utxos_above_10sats_under_100sats'),
            _1btcTo10btc: create_0satsPattern2(this, 'utxos_above_1btc_under_10btc'),
            _1kBtcTo10kBtc: create_0satsPattern2(this, 'utxos_above_1k_btc_under_10k_btc'),
            _1kSatsTo10kSats: create_0satsPattern2(this, 'utxos_above_1k_sats_under_10k_sats'),
            _1mSatsTo10mSats: create_0satsPattern2(this, 'utxos_above_1m_sats_under_10m_sats'),
            _1satTo10sats: create_0satsPattern2(this, 'utxos_above_1sat_under_10sats'),
          },
          epoch: {
            _0: create_0satsPattern2(this, 'epoch_0'),
            _1: create_0satsPattern2(this, 'epoch_1'),
            _2: create_0satsPattern2(this, 'epoch_2'),
            _3: create_0satsPattern2(this, 'epoch_3'),
            _4: create_0satsPattern2(this, 'epoch_4'),
          },
          geAmount: {
            _100btc: create_100btcPattern(this, 'utxos_above_100btc'),
            _100kSats: create_100btcPattern(this, 'utxos_above_100k_sats'),
            _100sats: create_100btcPattern(this, 'utxos_above_100sats'),
            _10btc: create_100btcPattern(this, 'utxos_above_10btc'),
            _10kBtc: create_100btcPattern(this, 'utxos_above_10k_btc'),
            _10kSats: create_100btcPattern(this, 'utxos_above_10k_sats'),
            _10mSats: create_100btcPattern(this, 'utxos_above_10m_sats'),
            _10sats: create_100btcPattern(this, 'utxos_above_10sats'),
            _1btc: create_100btcPattern(this, 'utxos_above_1btc'),
            _1kBtc: create_100btcPattern(this, 'utxos_above_1k_btc'),
            _1kSats: create_100btcPattern(this, 'utxos_above_1k_sats'),
            _1mSats: create_100btcPattern(this, 'utxos_above_1m_sats'),
            _1sat: create_100btcPattern(this, 'utxos_above_1sat'),
          },
          ltAmount: {
            _100btc: create_100btcPattern(this, 'utxos_under_100btc'),
            _100kBtc: create_100btcPattern(this, 'utxos_under_100k_btc'),
            _100kSats: create_100btcPattern(this, 'utxos_under_100k_sats'),
            _100sats: create_100btcPattern(this, 'utxos_under_100sats'),
            _10btc: create_100btcPattern(this, 'utxos_under_10btc'),
            _10kBtc: create_100btcPattern(this, 'utxos_under_10k_btc'),
            _10kSats: create_100btcPattern(this, 'utxos_under_10k_sats'),
            _10mSats: create_100btcPattern(this, 'utxos_under_10m_sats'),
            _10sats: create_100btcPattern(this, 'utxos_under_10sats'),
            _1btc: create_100btcPattern(this, 'utxos_under_1btc'),
            _1kBtc: create_100btcPattern(this, 'utxos_under_1k_btc'),
            _1kSats: create_100btcPattern(this, 'utxos_under_1k_sats'),
            _1mSats: create_100btcPattern(this, 'utxos_under_1m_sats'),
          },
          maxAge: {
            _10y: create_10yPattern(this, 'utxos_up_to_10y_old'),
            _12y: create_10yPattern(this, 'utxos_up_to_12y_old'),
            _15y: create_10yPattern(this, 'utxos_up_to_15y_old'),
            _1m: create_10yPattern(this, 'utxos_up_to_1m_old'),
            _1w: create_10yPattern(this, 'utxos_up_to_1w_old'),
            _1y: create_10yPattern(this, 'utxos_up_to_1y_old'),
            _2m: create_10yPattern(this, 'utxos_up_to_2m_old'),
            _2y: create_10yPattern(this, 'utxos_up_to_2y_old'),
            _3m: create_10yPattern(this, 'utxos_up_to_3m_old'),
            _3y: create_10yPattern(this, 'utxos_up_to_3y_old'),
            _4m: create_10yPattern(this, 'utxos_up_to_4m_old'),
            _4y: create_10yPattern(this, 'utxos_up_to_4y_old'),
            _5m: create_10yPattern(this, 'utxos_up_to_5m_old'),
            _5y: create_10yPattern(this, 'utxos_up_to_5y_old'),
            _6m: create_10yPattern(this, 'utxos_up_to_6m_old'),
            _6y: create_10yPattern(this, 'utxos_up_to_6y_old'),
            _7y: create_10yPattern(this, 'utxos_up_to_7y_old'),
            _8y: create_10yPattern(this, 'utxos_up_to_8y_old'),
          },
          minAge: {
            _10y: create_100btcPattern(this, 'utxos_at_least_10y_old'),
            _12y: create_100btcPattern(this, 'utxos_at_least_12y_old'),
            _1d: create_100btcPattern(this, 'utxos_at_least_1d_old'),
            _1m: create_100btcPattern(this, 'utxos_at_least_1m_old'),
            _1w: create_100btcPattern(this, 'utxos_at_least_1w_old'),
            _1y: create_100btcPattern(this, 'utxos_at_least_1y_old'),
            _2m: create_100btcPattern(this, 'utxos_at_least_2m_old'),
            _2y: create_100btcPattern(this, 'utxos_at_least_2y_old'),
            _3m: create_100btcPattern(this, 'utxos_at_least_3m_old'),
            _3y: create_100btcPattern(this, 'utxos_at_least_3y_old'),
            _4m: create_100btcPattern(this, 'utxos_at_least_4m_old'),
            _4y: create_100btcPattern(this, 'utxos_at_least_4y_old'),
            _5m: create_100btcPattern(this, 'utxos_at_least_5m_old'),
            _5y: create_100btcPattern(this, 'utxos_at_least_5y_old'),
            _6m: create_100btcPattern(this, 'utxos_at_least_6m_old'),
            _6y: create_100btcPattern(this, 'utxos_at_least_6y_old'),
            _7y: create_100btcPattern(this, 'utxos_at_least_7y_old'),
            _8y: create_100btcPattern(this, 'utxos_at_least_8y_old'),
          },
          term: {
            long: {
              activity: createActivityPattern2(this, 'lth'),
              costBasis: createCostBasisPattern2(this, 'lth'),
              outputs: createOutputsPattern(this, 'lth_utxo_count'),
              realized: createRealizedPattern2(this, 'lth'),
              relative: createRelativePattern5(this, 'lth'),
              supply: createSupplyPattern2(this, 'lth_supply'),
              unrealized: createUnrealizedPattern(this, 'lth'),
            },
            short: {
              activity: createActivityPattern2(this, 'sth'),
              costBasis: createCostBasisPattern2(this, 'sth'),
              outputs: createOutputsPattern(this, 'sth_utxo_count'),
              realized: createRealizedPattern3(this, 'sth'),
              relative: createRelativePattern5(this, 'sth'),
              supply: createSupplyPattern2(this, 'sth_supply'),
              unrealized: createUnrealizedPattern(this, 'sth'),
            },
          },
          type: {
            empty: create_0satsPattern2(this, 'empty_outputs'),
            p2a: create_0satsPattern2(this, 'p2a'),
            p2ms: create_0satsPattern2(this, 'p2ms'),
            p2pk33: create_0satsPattern2(this, 'p2pk33'),
            p2pk65: create_0satsPattern2(this, 'p2pk65'),
            p2pkh: create_0satsPattern2(this, 'p2pkh'),
            p2sh: create_0satsPattern2(this, 'p2sh'),
            p2tr: create_0satsPattern2(this, 'p2tr'),
            p2wpkh: create_0satsPattern2(this, 'p2wpkh'),
            p2wsh: create_0satsPattern2(this, 'p2wsh'),
            unknown: create_0satsPattern2(this, 'unknown_outputs'),
          },
          year: {
            _2009: create_0satsPattern2(this, 'year_2009'),
            _2010: create_0satsPattern2(this, 'year_2010'),
            _2011: create_0satsPattern2(this, 'year_2011'),
            _2012: create_0satsPattern2(this, 'year_2012'),
            _2013: create_0satsPattern2(this, 'year_2013'),
            _2014: create_0satsPattern2(this, 'year_2014'),
            _2015: create_0satsPattern2(this, 'year_2015'),
            _2016: create_0satsPattern2(this, 'year_2016'),
            _2017: create_0satsPattern2(this, 'year_2017'),
            _2018: create_0satsPattern2(this, 'year_2018'),
            _2019: create_0satsPattern2(this, 'year_2019'),
            _2020: create_0satsPattern2(this, 'year_2020'),
            _2021: create_0satsPattern2(this, 'year_2021'),
            _2022: create_0satsPattern2(this, 'year_2022'),
            _2023: create_0satsPattern2(this, 'year_2023'),
            _2024: create_0satsPattern2(this, 'year_2024'),
            _2025: create_0satsPattern2(this, 'year_2025'),
            _2026: create_0satsPattern2(this, 'year_2026'),
          },
        },
      },
      indexes: {
        address: {
          empty: {
            identity: createMetricPattern9(this, 'emptyoutputindex'),
          },
          opreturn: {
            identity: createMetricPattern14(this, 'opreturnindex'),
          },
          p2a: {
            identity: createMetricPattern16(this, 'p2aaddressindex'),
          },
          p2ms: {
            identity: createMetricPattern17(this, 'p2msoutputindex'),
          },
          p2pk33: {
            identity: createMetricPattern18(this, 'p2pk33addressindex'),
          },
          p2pk65: {
            identity: createMetricPattern19(this, 'p2pk65addressindex'),
          },
          p2pkh: {
            identity: createMetricPattern20(this, 'p2pkhaddressindex'),
          },
          p2sh: {
            identity: createMetricPattern21(this, 'p2shaddressindex'),
          },
          p2tr: {
            identity: createMetricPattern22(this, 'p2traddressindex'),
          },
          p2wpkh: {
            identity: createMetricPattern23(this, 'p2wpkhaddressindex'),
          },
          p2wsh: {
            identity: createMetricPattern24(this, 'p2wshaddressindex'),
          },
          unknown: {
            identity: createMetricPattern28(this, 'unknownoutputindex'),
          },
        },
        dateindex: {
          date: createMetricPattern6(this, 'date'),
          firstHeight: createMetricPattern6(this, 'first_height'),
          heightCount: createMetricPattern6(this, 'height_count'),
          identity: createMetricPattern6(this, 'dateindex'),
          monthindex: createMetricPattern6(this, 'monthindex'),
          weekindex: createMetricPattern6(this, 'weekindex'),
        },
        decadeindex: {
          date: createMetricPattern7(this, 'date'),
          firstYearindex: createMetricPattern7(this, 'first_yearindex'),
          identity: createMetricPattern7(this, 'decadeindex'),
          yearindexCount: createMetricPattern7(this, 'yearindex_count'),
        },
        difficultyepoch: {
          firstHeight: createMetricPattern8(this, 'first_height'),
          heightCount: createMetricPattern8(this, 'height_count'),
          identity: createMetricPattern8(this, 'difficultyepoch'),
        },
        halvingepoch: {
          firstHeight: createMetricPattern10(this, 'first_height'),
          identity: createMetricPattern10(this, 'halvingepoch'),
        },
        height: {
          dateindex: createMetricPattern11(this, 'dateindex'),
          difficultyepoch: createMetricPattern11(this, 'difficultyepoch'),
          halvingepoch: createMetricPattern11(this, 'halvingepoch'),
          identity: createMetricPattern11(this, 'height'),
          txindexCount: createMetricPattern11(this, 'txindex_count'),
        },
        monthindex: {
          date: createMetricPattern13(this, 'date'),
          dateindexCount: createMetricPattern13(this, 'dateindex_count'),
          firstDateindex: createMetricPattern13(this, 'first_dateindex'),
          identity: createMetricPattern13(this, 'monthindex'),
          quarterindex: createMetricPattern13(this, 'quarterindex'),
          semesterindex: createMetricPattern13(this, 'semesterindex'),
          yearindex: createMetricPattern13(this, 'yearindex'),
        },
        quarterindex: {
          date: createMetricPattern25(this, 'date'),
          firstMonthindex: createMetricPattern25(this, 'first_monthindex'),
          identity: createMetricPattern25(this, 'quarterindex'),
          monthindexCount: createMetricPattern25(this, 'monthindex_count'),
        },
        semesterindex: {
          date: createMetricPattern26(this, 'date'),
          firstMonthindex: createMetricPattern26(this, 'first_monthindex'),
          identity: createMetricPattern26(this, 'semesterindex'),
          monthindexCount: createMetricPattern26(this, 'monthindex_count'),
        },
        txindex: {
          identity: createMetricPattern27(this, 'txindex'),
          inputCount: createMetricPattern27(this, 'input_count'),
          outputCount: createMetricPattern27(this, 'output_count'),
        },
        txinindex: {
          identity: createMetricPattern12(this, 'txinindex'),
        },
        txoutindex: {
          identity: createMetricPattern15(this, 'txoutindex'),
        },
        weekindex: {
          date: createMetricPattern29(this, 'date'),
          dateindexCount: createMetricPattern29(this, 'dateindex_count'),
          firstDateindex: createMetricPattern29(this, 'first_dateindex'),
          identity: createMetricPattern29(this, 'weekindex'),
        },
        yearindex: {
          date: createMetricPattern30(this, 'date'),
          decadeindex: createMetricPattern30(this, 'decadeindex'),
          firstMonthindex: createMetricPattern30(this, 'first_monthindex'),
          identity: createMetricPattern30(this, 'yearindex'),
          monthindexCount: createMetricPattern30(this, 'monthindex_count'),
        },
      },
      inputs: {
        count: createCountPattern2(this, 'input_count'),
        firstTxinindex: createMetricPattern11(this, 'first_txinindex'),
        outpoint: createMetricPattern12(this, 'outpoint'),
        outputtype: createMetricPattern12(this, 'outputtype'),
        spent: {
          txoutindex: createMetricPattern12(this, 'txoutindex'),
          value: createMetricPattern12(this, 'value'),
        },
        txindex: createMetricPattern12(this, 'txindex'),
        typeindex: createMetricPattern12(this, 'typeindex'),
      },
      market: {
        ath: {
          daysSincePriceAth: createMetricPattern4(this, 'days_since_price_ath'),
          maxDaysBetweenPriceAths: createMetricPattern4(this, 'max_days_between_price_aths'),
          maxYearsBetweenPriceAths: createMetricPattern4(this, 'max_years_between_price_aths'),
          priceAth: createMetricPattern1(this, 'price_ath'),
          priceDrawdown: createMetricPattern3(this, 'price_drawdown'),
          yearsSincePriceAth: createMetricPattern4(this, 'years_since_price_ath'),
        },
        dca: {
          classAveragePrice: createClassAveragePricePattern(this, 'dca_class'),
          classReturns: {
            _2015: createMetricPattern4(this, 'dca_class_2015_returns'),
            _2016: createMetricPattern4(this, 'dca_class_2016_returns'),
            _2017: createMetricPattern4(this, 'dca_class_2017_returns'),
            _2018: createMetricPattern4(this, 'dca_class_2018_returns'),
            _2019: createMetricPattern4(this, 'dca_class_2019_returns'),
            _2020: createMetricPattern4(this, 'dca_class_2020_returns'),
            _2021: createMetricPattern4(this, 'dca_class_2021_returns'),
            _2022: createMetricPattern4(this, 'dca_class_2022_returns'),
            _2023: createMetricPattern4(this, 'dca_class_2023_returns'),
            _2024: createMetricPattern4(this, 'dca_class_2024_returns'),
            _2025: createMetricPattern4(this, 'dca_class_2025_returns'),
          },
          classStack: {
            _2015: create_2015Pattern(this, 'dca_class_2015_stack'),
            _2016: create_2015Pattern(this, 'dca_class_2016_stack'),
            _2017: create_2015Pattern(this, 'dca_class_2017_stack'),
            _2018: create_2015Pattern(this, 'dca_class_2018_stack'),
            _2019: create_2015Pattern(this, 'dca_class_2019_stack'),
            _2020: create_2015Pattern(this, 'dca_class_2020_stack'),
            _2021: create_2015Pattern(this, 'dca_class_2021_stack'),
            _2022: create_2015Pattern(this, 'dca_class_2022_stack'),
            _2023: create_2015Pattern(this, 'dca_class_2023_stack'),
            _2024: create_2015Pattern(this, 'dca_class_2024_stack'),
            _2025: create_2015Pattern(this, 'dca_class_2025_stack'),
          },
          periodAveragePrice: createPeriodAveragePricePattern(this, 'dca_average_price'),
          periodCagr: createPeriodCagrPattern(this, 'dca_cagr'),
          periodLumpSumStack: createPeriodLumpSumStackPattern(this, 'lump_sum_stack'),
          periodReturns: createPeriodAveragePricePattern(this, 'dca_returns'),
          periodStack: createPeriodLumpSumStackPattern(this, 'dca_stack'),
        },
        indicators: {
          gini: createMetricPattern6(this, 'gini'),
          macdHistogram: createMetricPattern6(this, 'macd_histogram'),
          macdLine: createMetricPattern6(this, 'macd_line'),
          macdSignal: createMetricPattern6(this, 'macd_signal'),
          nvt: createMetricPattern4(this, 'nvt'),
          piCycle: createMetricPattern6(this, 'pi_cycle'),
          puellMultiple: createMetricPattern4(this, 'puell_multiple'),
          rsi14d: createMetricPattern6(this, 'rsi_14d'),
          rsi14dMax: createMetricPattern6(this, 'rsi_14d_max'),
          rsi14dMin: createMetricPattern6(this, 'rsi_14d_min'),
          rsiAverageGain14d: createMetricPattern6(this, 'rsi_average_gain_14d'),
          rsiAverageLoss14d: createMetricPattern6(this, 'rsi_average_loss_14d'),
          rsiGains: createMetricPattern6(this, 'rsi_gains'),
          rsiLosses: createMetricPattern6(this, 'rsi_losses'),
          stochD: createMetricPattern6(this, 'stoch_d'),
          stochK: createMetricPattern6(this, 'stoch_k'),
          stochRsi: createMetricPattern6(this, 'stoch_rsi'),
          stochRsiD: createMetricPattern6(this, 'stoch_rsi_d'),
          stochRsiK: createMetricPattern6(this, 'stoch_rsi_k'),
        },
        lookback: createLookbackPattern(this, 'price'),
        movingAverage: {
          price111dSma: createPrice111dSmaPattern(this, 'price_111d_sma'),
          price12dEma: createPrice111dSmaPattern(this, 'price_12d_ema'),
          price13dEma: createPrice111dSmaPattern(this, 'price_13d_ema'),
          price13dSma: createPrice111dSmaPattern(this, 'price_13d_sma'),
          price144dEma: createPrice111dSmaPattern(this, 'price_144d_ema'),
          price144dSma: createPrice111dSmaPattern(this, 'price_144d_sma'),
          price1mEma: createPrice111dSmaPattern(this, 'price_1m_ema'),
          price1mSma: createPrice111dSmaPattern(this, 'price_1m_sma'),
          price1wEma: createPrice111dSmaPattern(this, 'price_1w_ema'),
          price1wSma: createPrice111dSmaPattern(this, 'price_1w_sma'),
          price1yEma: createPrice111dSmaPattern(this, 'price_1y_ema'),
          price1ySma: createPrice111dSmaPattern(this, 'price_1y_sma'),
          price200dEma: createPrice111dSmaPattern(this, 'price_200d_ema'),
          price200dSma: createPrice111dSmaPattern(this, 'price_200d_sma'),
          price200dSmaX08: createMetricPattern4(this, 'price_200d_sma_x0_8'),
          price200dSmaX24: createMetricPattern4(this, 'price_200d_sma_x2_4'),
          price200wEma: createPrice111dSmaPattern(this, 'price_200w_ema'),
          price200wSma: createPrice111dSmaPattern(this, 'price_200w_sma'),
          price21dEma: createPrice111dSmaPattern(this, 'price_21d_ema'),
          price21dSma: createPrice111dSmaPattern(this, 'price_21d_sma'),
          price26dEma: createPrice111dSmaPattern(this, 'price_26d_ema'),
          price2yEma: createPrice111dSmaPattern(this, 'price_2y_ema'),
          price2ySma: createPrice111dSmaPattern(this, 'price_2y_sma'),
          price34dEma: createPrice111dSmaPattern(this, 'price_34d_ema'),
          price34dSma: createPrice111dSmaPattern(this, 'price_34d_sma'),
          price350dSma: createPrice111dSmaPattern(this, 'price_350d_sma'),
          price350dSmaX2: createMetricPattern4(this, 'price_350d_sma_x2'),
          price4yEma: createPrice111dSmaPattern(this, 'price_4y_ema'),
          price4ySma: createPrice111dSmaPattern(this, 'price_4y_sma'),
          price55dEma: createPrice111dSmaPattern(this, 'price_55d_ema'),
          price55dSma: createPrice111dSmaPattern(this, 'price_55d_sma'),
          price89dEma: createPrice111dSmaPattern(this, 'price_89d_ema'),
          price89dSma: createPrice111dSmaPattern(this, 'price_89d_sma'),
          price8dEma: createPrice111dSmaPattern(this, 'price_8d_ema'),
          price8dSma: createPrice111dSmaPattern(this, 'price_8d_sma'),
        },
        range: {
          price1mMax: createMetricPattern4(this, 'price_1m_max'),
          price1mMin: createMetricPattern4(this, 'price_1m_min'),
          price1wMax: createMetricPattern4(this, 'price_1w_max'),
          price1wMin: createMetricPattern4(this, 'price_1w_min'),
          price1yMax: createMetricPattern4(this, 'price_1y_max'),
          price1yMin: createMetricPattern4(this, 'price_1y_min'),
          price2wChoppinessIndex: createMetricPattern4(this, 'price_2w_choppiness_index'),
          price2wMax: createMetricPattern4(this, 'price_2w_max'),
          price2wMin: createMetricPattern4(this, 'price_2w_min'),
          priceTrueRange: createMetricPattern6(this, 'price_true_range'),
          priceTrueRange2wSum: createMetricPattern6(this, 'price_true_range_2w_sum'),
        },
        returns: {
          _1dReturns1mSd: create_1dReturns1mSdPattern(this, '1d_returns_1m_sd'),
          _1dReturns1wSd: create_1dReturns1mSdPattern(this, '1d_returns_1w_sd'),
          _1dReturns1ySd: create_1dReturns1mSdPattern(this, '1d_returns_1y_sd'),
          cagr: createPeriodCagrPattern(this, 'cagr'),
          downside1mSd: create_1dReturns1mSdPattern(this, 'downside_1m_sd'),
          downside1wSd: create_1dReturns1mSdPattern(this, 'downside_1w_sd'),
          downside1ySd: create_1dReturns1mSdPattern(this, 'downside_1y_sd'),
          downsideReturns: createMetricPattern6(this, 'downside_returns'),
          priceReturns: {
            _10y: createMetricPattern4(this, '10y_price_returns'),
            _1d: createMetricPattern4(this, '1d_price_returns'),
            _1m: createMetricPattern4(this, '1m_price_returns'),
            _1w: createMetricPattern4(this, '1w_price_returns'),
            _1y: createMetricPattern4(this, '1y_price_returns'),
            _2y: createMetricPattern4(this, '2y_price_returns'),
            _3m: createMetricPattern4(this, '3m_price_returns'),
            _3y: createMetricPattern4(this, '3y_price_returns'),
            _4y: createMetricPattern4(this, '4y_price_returns'),
            _5y: createMetricPattern4(this, '5y_price_returns'),
            _6m: createMetricPattern4(this, '6m_price_returns'),
            _6y: createMetricPattern4(this, '6y_price_returns'),
            _8y: createMetricPattern4(this, '8y_price_returns'),
          },
        },
        volatility: {
          price1mVolatility: createMetricPattern4(this, 'price_1m_volatility'),
          price1wVolatility: createMetricPattern4(this, 'price_1w_volatility'),
          price1yVolatility: createMetricPattern4(this, 'price_1y_volatility'),
          sharpe1m: createMetricPattern6(this, 'sharpe_1m'),
          sharpe1w: createMetricPattern6(this, 'sharpe_1w'),
          sharpe1y: createMetricPattern6(this, 'sharpe_1y'),
          sortino1m: createMetricPattern6(this, 'sortino_1m'),
          sortino1w: createMetricPattern6(this, 'sortino_1w'),
          sortino1y: createMetricPattern6(this, 'sortino_1y'),
        },
      },
      outputs: {
        count: {
          totalCount: createCountPattern2(this, 'output_count'),
          utxoCount: createMetricPattern1(this, 'exact_utxo_count'),
        },
        firstTxoutindex: createMetricPattern11(this, 'first_txoutindex'),
        outputtype: createMetricPattern15(this, 'outputtype'),
        spent: {
          txinindex: createMetricPattern15(this, 'txinindex'),
        },
        txindex: createMetricPattern15(this, 'txindex'),
        typeindex: createMetricPattern15(this, 'typeindex'),
        value: createMetricPattern15(this, 'value'),
      },
      pools: {
        heightToPool: createMetricPattern11(this, 'pool'),
        vecs: {
          aaopool: createAaopoolPattern(this, 'aaopool'),
          antpool: createAaopoolPattern(this, 'antpool'),
          arkpool: createAaopoolPattern(this, 'arkpool'),
          asicminer: createAaopoolPattern(this, 'asicminer'),
          axbt: createAaopoolPattern(this, 'axbt'),
          batpool: createAaopoolPattern(this, 'batpool'),
          bcmonster: createAaopoolPattern(this, 'bcmonster'),
          bcpoolio: createAaopoolPattern(this, 'bcpoolio'),
          binancepool: createAaopoolPattern(this, 'binancepool'),
          bitalo: createAaopoolPattern(this, 'bitalo'),
          bitclub: createAaopoolPattern(this, 'bitclub'),
          bitcoinaffiliatenetwork: createAaopoolPattern(this, 'bitcoinaffiliatenetwork'),
          bitcoincom: createAaopoolPattern(this, 'bitcoincom'),
          bitcoinindia: createAaopoolPattern(this, 'bitcoinindia'),
          bitcoinrussia: createAaopoolPattern(this, 'bitcoinrussia'),
          bitcoinukraine: createAaopoolPattern(this, 'bitcoinukraine'),
          bitfarms: createAaopoolPattern(this, 'bitfarms'),
          bitfufupool: createAaopoolPattern(this, 'bitfufupool'),
          bitfury: createAaopoolPattern(this, 'bitfury'),
          bitminter: createAaopoolPattern(this, 'bitminter'),
          bitparking: createAaopoolPattern(this, 'bitparking'),
          bitsolo: createAaopoolPattern(this, 'bitsolo'),
          bixin: createAaopoolPattern(this, 'bixin'),
          blockfills: createAaopoolPattern(this, 'blockfills'),
          braiinspool: createAaopoolPattern(this, 'braiinspool'),
          bravomining: createAaopoolPattern(this, 'bravomining'),
          btcc: createAaopoolPattern(this, 'btcc'),
          btccom: createAaopoolPattern(this, 'btccom'),
          btcdig: createAaopoolPattern(this, 'btcdig'),
          btcguild: createAaopoolPattern(this, 'btcguild'),
          btclab: createAaopoolPattern(this, 'btclab'),
          btcmp: createAaopoolPattern(this, 'btcmp'),
          btcnuggets: createAaopoolPattern(this, 'btcnuggets'),
          btcpoolparty: createAaopoolPattern(this, 'btcpoolparty'),
          btcserv: createAaopoolPattern(this, 'btcserv'),
          btctop: createAaopoolPattern(this, 'btctop'),
          btpool: createAaopoolPattern(this, 'btpool'),
          bwpool: createAaopoolPattern(this, 'bwpool'),
          bytepool: createAaopoolPattern(this, 'bytepool'),
          canoe: createAaopoolPattern(this, 'canoe'),
          canoepool: createAaopoolPattern(this, 'canoepool'),
          carbonnegative: createAaopoolPattern(this, 'carbonnegative'),
          ckpool: createAaopoolPattern(this, 'ckpool'),
          cloudhashing: createAaopoolPattern(this, 'cloudhashing'),
          coinlab: createAaopoolPattern(this, 'coinlab'),
          cointerra: createAaopoolPattern(this, 'cointerra'),
          connectbtc: createAaopoolPattern(this, 'connectbtc'),
          dcex: createAaopoolPattern(this, 'dcex'),
          dcexploration: createAaopoolPattern(this, 'dcexploration'),
          digitalbtc: createAaopoolPattern(this, 'digitalbtc'),
          digitalxmintsy: createAaopoolPattern(this, 'digitalxmintsy'),
          dpool: createAaopoolPattern(this, 'dpool'),
          eclipsemc: createAaopoolPattern(this, 'eclipsemc'),
          eightbaochi: createAaopoolPattern(this, 'eightbaochi'),
          ekanembtc: createAaopoolPattern(this, 'ekanembtc'),
          eligius: createAaopoolPattern(this, 'eligius'),
          emcdpool: createAaopoolPattern(this, 'emcdpool'),
          entrustcharitypool: createAaopoolPattern(this, 'entrustcharitypool'),
          eobot: createAaopoolPattern(this, 'eobot'),
          exxbw: createAaopoolPattern(this, 'exxbw'),
          f2pool: createAaopoolPattern(this, 'f2pool'),
          fiftyeightcoin: createAaopoolPattern(this, 'fiftyeightcoin'),
          foundryusa: createAaopoolPattern(this, 'foundryusa'),
          futurebitapollosolo: createAaopoolPattern(this, 'futurebitapollosolo'),
          gbminers: createAaopoolPattern(this, 'gbminers'),
          ghashio: createAaopoolPattern(this, 'ghashio'),
          givemecoins: createAaopoolPattern(this, 'givemecoins'),
          gogreenlight: createAaopoolPattern(this, 'gogreenlight'),
          haominer: createAaopoolPattern(this, 'haominer'),
          haozhuzhu: createAaopoolPattern(this, 'haozhuzhu'),
          hashbx: createAaopoolPattern(this, 'hashbx'),
          hashpool: createAaopoolPattern(this, 'hashpool'),
          helix: createAaopoolPattern(this, 'helix'),
          hhtt: createAaopoolPattern(this, 'hhtt'),
          hotpool: createAaopoolPattern(this, 'hotpool'),
          hummerpool: createAaopoolPattern(this, 'hummerpool'),
          huobipool: createAaopoolPattern(this, 'huobipool'),
          innopolistech: createAaopoolPattern(this, 'innopolistech'),
          kanopool: createAaopoolPattern(this, 'kanopool'),
          kncminer: createAaopoolPattern(this, 'kncminer'),
          kucoinpool: createAaopoolPattern(this, 'kucoinpool'),
          lubiancom: createAaopoolPattern(this, 'lubiancom'),
          luckypool: createAaopoolPattern(this, 'luckypool'),
          luxor: createAaopoolPattern(this, 'luxor'),
          marapool: createAaopoolPattern(this, 'marapool'),
          maxbtc: createAaopoolPattern(this, 'maxbtc'),
          maxipool: createAaopoolPattern(this, 'maxipool'),
          megabigpower: createAaopoolPattern(this, 'megabigpower'),
          minerium: createAaopoolPattern(this, 'minerium'),
          miningcity: createAaopoolPattern(this, 'miningcity'),
          miningdutch: createAaopoolPattern(this, 'miningdutch'),
          miningkings: createAaopoolPattern(this, 'miningkings'),
          miningsquared: createAaopoolPattern(this, 'miningsquared'),
          mmpool: createAaopoolPattern(this, 'mmpool'),
          mtred: createAaopoolPattern(this, 'mtred'),
          multicoinco: createAaopoolPattern(this, 'multicoinco'),
          multipool: createAaopoolPattern(this, 'multipool'),
          mybtccoinpool: createAaopoolPattern(this, 'mybtccoinpool'),
          neopool: createAaopoolPattern(this, 'neopool'),
          nexious: createAaopoolPattern(this, 'nexious'),
          nicehash: createAaopoolPattern(this, 'nicehash'),
          nmcbit: createAaopoolPattern(this, 'nmcbit'),
          novablock: createAaopoolPattern(this, 'novablock'),
          ocean: createAaopoolPattern(this, 'ocean'),
          okexpool: createAaopoolPattern(this, 'okexpool'),
          okkong: createAaopoolPattern(this, 'okkong'),
          okminer: createAaopoolPattern(this, 'okminer'),
          okpooltop: createAaopoolPattern(this, 'okpooltop'),
          onehash: createAaopoolPattern(this, 'onehash'),
          onem1x: createAaopoolPattern(this, 'onem1x'),
          onethash: createAaopoolPattern(this, 'onethash'),
          ozcoin: createAaopoolPattern(this, 'ozcoin'),
          parasite: createAaopoolPattern(this, 'parasite'),
          patels: createAaopoolPattern(this, 'patels'),
          pegapool: createAaopoolPattern(this, 'pegapool'),
          phashio: createAaopoolPattern(this, 'phashio'),
          phoenix: createAaopoolPattern(this, 'phoenix'),
          polmine: createAaopoolPattern(this, 'polmine'),
          pool175btc: createAaopoolPattern(this, 'pool175btc'),
          pool50btc: createAaopoolPattern(this, 'pool50btc'),
          poolin: createAaopoolPattern(this, 'poolin'),
          portlandhodl: createAaopoolPattern(this, 'portlandhodl'),
          publicpool: createAaopoolPattern(this, 'publicpool'),
          purebtccom: createAaopoolPattern(this, 'purebtccom'),
          rawpool: createAaopoolPattern(this, 'rawpool'),
          rigpool: createAaopoolPattern(this, 'rigpool'),
          sbicrypto: createAaopoolPattern(this, 'sbicrypto'),
          secpool: createAaopoolPattern(this, 'secpool'),
          secretsuperstar: createAaopoolPattern(this, 'secretsuperstar'),
          sevenpool: createAaopoolPattern(this, 'sevenpool'),
          shawnp0wers: createAaopoolPattern(this, 'shawnp0wers'),
          sigmapoolcom: createAaopoolPattern(this, 'sigmapoolcom'),
          simplecoinus: createAaopoolPattern(this, 'simplecoinus'),
          solock: createAaopoolPattern(this, 'solock'),
          spiderpool: createAaopoolPattern(this, 'spiderpool'),
          stminingcorp: createAaopoolPattern(this, 'stminingcorp'),
          tangpool: createAaopoolPattern(this, 'tangpool'),
          tatmaspool: createAaopoolPattern(this, 'tatmaspool'),
          tbdice: createAaopoolPattern(this, 'tbdice'),
          telco214: createAaopoolPattern(this, 'telco214'),
          terrapool: createAaopoolPattern(this, 'terrapool'),
          tiger: createAaopoolPattern(this, 'tiger'),
          tigerpoolnet: createAaopoolPattern(this, 'tigerpoolnet'),
          titan: createAaopoolPattern(this, 'titan'),
          transactioncoinmining: createAaopoolPattern(this, 'transactioncoinmining'),
          trickysbtcpool: createAaopoolPattern(this, 'trickysbtcpool'),
          triplemining: createAaopoolPattern(this, 'triplemining'),
          twentyoneinc: createAaopoolPattern(this, 'twentyoneinc'),
          ultimuspool: createAaopoolPattern(this, 'ultimuspool'),
          unknown: createAaopoolPattern(this, 'unknown'),
          unomp: createAaopoolPattern(this, 'unomp'),
          viabtc: createAaopoolPattern(this, 'viabtc'),
          waterhole: createAaopoolPattern(this, 'waterhole'),
          wayicn: createAaopoolPattern(this, 'wayicn'),
          whitepool: createAaopoolPattern(this, 'whitepool'),
          wk057: createAaopoolPattern(this, 'wk057'),
          yourbtcnet: createAaopoolPattern(this, 'yourbtcnet'),
          zulupool: createAaopoolPattern(this, 'zulupool'),
        },
      },
      positions: {
        blockPosition: createMetricPattern11(this, 'position'),
        txPosition: createMetricPattern27(this, 'position'),
      },
      price: {
        cents: {
          ohlc: createMetricPattern5(this, 'ohlc_cents'),
          split: {
            close: createMetricPattern5(this, 'price_close_cents'),
            high: createMetricPattern5(this, 'price_high_cents'),
            low: createMetricPattern5(this, 'price_low_cents'),
            open: createMetricPattern5(this, 'price_open_cents'),
          },
        },
        oracle: {
          heightToFirstPairoutputindex: createMetricPattern11(this, 'height_to_first_pairoutputindex'),
          ohlcCents: createMetricPattern6(this, 'oracle_ohlc_cents'),
          ohlcDollars: createMetricPattern6(this, 'oracle_ohlc'),
          output0Value: createMetricPattern33(this, 'pair_output0_value'),
          output1Value: createMetricPattern33(this, 'pair_output1_value'),
          pairoutputindexToTxindex: createMetricPattern33(this, 'pairoutputindex_to_txindex'),
          phaseDailyCents: createPhaseDailyCentsPattern(this, 'phase_daily'),
          phaseDailyDollars: createPhaseDailyCentsPattern(this, 'phase_daily_dollars'),
          phaseHistogram: createMetricPattern11(this, 'phase_histogram'),
          phasePriceCents: createMetricPattern11(this, 'phase_price_cents'),
          phaseV2DailyCents: createPhaseDailyCentsPattern(this, 'phase_v2_daily'),
          phaseV2DailyDollars: createPhaseDailyCentsPattern(this, 'phase_v2_daily_dollars'),
          phaseV2Histogram: createMetricPattern11(this, 'phase_v2_histogram'),
          phaseV2PeakDailyCents: createPhaseDailyCentsPattern(this, 'phase_v2_peak_daily'),
          phaseV2PeakDailyDollars: createPhaseDailyCentsPattern(this, 'phase_v2_peak_daily_dollars'),
          phaseV2PeakPriceCents: createMetricPattern11(this, 'phase_v2_peak_price_cents'),
          phaseV2PriceCents: createMetricPattern11(this, 'phase_v2_price_cents'),
          phaseV3DailyCents: createPhaseDailyCentsPattern(this, 'phase_v3_daily'),
          phaseV3DailyDollars: createPhaseDailyCentsPattern(this, 'phase_v3_daily_dollars'),
          phaseV3Histogram: createMetricPattern11(this, 'phase_v3_histogram'),
          phaseV3PeakDailyCents: createPhaseDailyCentsPattern(this, 'phase_v3_peak_daily'),
          phaseV3PeakDailyDollars: createPhaseDailyCentsPattern(this, 'phase_v3_peak_daily_dollars'),
          phaseV3PeakPriceCents: createMetricPattern11(this, 'phase_v3_peak_price_cents'),
          phaseV3PriceCents: createMetricPattern11(this, 'phase_v3_price_cents'),
          priceCents: createMetricPattern11(this, 'oracle_price_cents'),
          txCount: createMetricPattern6(this, 'oracle_tx_count'),
        },
        sats: createSatsPattern(this, 'price'),
        usd: {
          ohlc: createMetricPattern1(this, 'price_ohlc'),
          split: createSplitPattern2(this, 'price'),
        },
      },
      scripts: {
        count: {
          emptyoutput: createDollarsPattern(this, 'emptyoutput_count'),
          opreturn: createDollarsPattern(this, 'opreturn_count'),
          p2a: createDollarsPattern(this, 'p2a_count'),
          p2ms: createDollarsPattern(this, 'p2ms_count'),
          p2pk33: createDollarsPattern(this, 'p2pk33_count'),
          p2pk65: createDollarsPattern(this, 'p2pk65_count'),
          p2pkh: createDollarsPattern(this, 'p2pkh_count'),
          p2sh: createDollarsPattern(this, 'p2sh_count'),
          p2tr: createDollarsPattern(this, 'p2tr_count'),
          p2wpkh: createDollarsPattern(this, 'p2wpkh_count'),
          p2wsh: createDollarsPattern(this, 'p2wsh_count'),
          segwit: createDollarsPattern(this, 'segwit_count'),
          segwitAdoption: createSegwitAdoptionPattern(this, 'segwit_adoption'),
          taprootAdoption: createSegwitAdoptionPattern(this, 'taproot_adoption'),
          unknownoutput: createDollarsPattern(this, 'unknownoutput_count'),
        },
        emptyToTxindex: createMetricPattern9(this, 'txindex'),
        firstEmptyoutputindex: createMetricPattern11(this, 'first_emptyoutputindex'),
        firstOpreturnindex: createMetricPattern11(this, 'first_opreturnindex'),
        firstP2msoutputindex: createMetricPattern11(this, 'first_p2msoutputindex'),
        firstUnknownoutputindex: createMetricPattern11(this, 'first_unknownoutputindex'),
        opreturnToTxindex: createMetricPattern14(this, 'txindex'),
        p2msToTxindex: createMetricPattern17(this, 'txindex'),
        unknownToTxindex: createMetricPattern28(this, 'txindex'),
        value: {
          opreturn: createCoinbasePattern(this, 'opreturn_value'),
        },
      },
      supply: {
        burned: {
          opreturn: createUnclaimedRewardsPattern(this, 'opreturn_supply'),
          unspendable: createUnclaimedRewardsPattern(this, 'unspendable_supply'),
        },
        circulating: {
          bitcoin: createMetricPattern3(this, 'circulating_supply_btc'),
          dollars: createMetricPattern3(this, 'circulating_supply_usd'),
          sats: createMetricPattern3(this, 'circulating_supply'),
        },
        inflation: createMetricPattern4(this, 'inflation_rate'),
        marketCap: createMetricPattern1(this, 'market_cap'),
        velocity: {
          btc: createMetricPattern4(this, 'btc_velocity'),
          usd: createMetricPattern4(this, 'usd_velocity'),
        },
      },
      transactions: {
        baseSize: createMetricPattern27(this, 'base_size'),
        count: {
          isCoinbase: createMetricPattern27(this, 'is_coinbase'),
          txCount: createDollarsPattern(this, 'tx_count'),
        },
        fees: {
          fee: {
            bitcoin: createCountPattern2(this, 'fee_btc'),
            dollars: {
              average: createMetricPattern1(this, 'fee_usd_average'),
              cumulative: createMetricPattern2(this, 'fee_usd_cumulative'),
              heightCumulative: createMetricPattern11(this, 'fee_usd_cumulative'),
              max: createMetricPattern1(this, 'fee_usd_max'),
              median: createMetricPattern11(this, 'fee_usd_median'),
              min: createMetricPattern1(this, 'fee_usd_min'),
              pct10: createMetricPattern11(this, 'fee_usd_pct10'),
              pct25: createMetricPattern11(this, 'fee_usd_pct25'),
              pct75: createMetricPattern11(this, 'fee_usd_pct75'),
              pct90: createMetricPattern11(this, 'fee_usd_pct90'),
              sum: createMetricPattern1(this, 'fee_usd_sum'),
            },
            sats: createCountPattern2(this, 'fee'),
            txindex: createMetricPattern27(this, 'fee'),
          },
          feeRate: createFeeRatePattern(this, 'fee_rate'),
          inputValue: createMetricPattern27(this, 'input_value'),
          outputValue: createMetricPattern27(this, 'output_value'),
        },
        firstTxindex: createMetricPattern11(this, 'first_txindex'),
        firstTxinindex: createMetricPattern27(this, 'first_txinindex'),
        firstTxoutindex: createMetricPattern27(this, 'first_txoutindex'),
        height: createMetricPattern27(this, 'height'),
        isExplicitlyRbf: createMetricPattern27(this, 'is_explicitly_rbf'),
        rawlocktime: createMetricPattern27(this, 'rawlocktime'),
        size: {
          vsize: createFeeRatePattern(this, 'tx_vsize'),
          weight: createFeeRatePattern(this, 'tx_weight'),
        },
        totalSize: createMetricPattern27(this, 'total_size'),
        txid: createMetricPattern27(this, 'txid'),
        txversion: createMetricPattern27(this, 'txversion'),
        versions: {
          v1: createBlockCountPattern(this, 'tx_v1'),
          v2: createBlockCountPattern(this, 'tx_v2'),
          v3: createBlockCountPattern(this, 'tx_v3'),
        },
        volume: {
          annualizedVolume: create_2015Pattern(this, 'annualized_volume'),
          inputsPerSec: createMetricPattern4(this, 'inputs_per_sec'),
          outputsPerSec: createMetricPattern4(this, 'outputs_per_sec'),
          sentSum: createActiveSupplyPattern(this, 'sent_sum'),
          txPerSec: createMetricPattern4(this, 'tx_per_sec'),
        },
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
   * Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.
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
