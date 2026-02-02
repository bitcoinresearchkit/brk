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
 * Unsigned cents (u64) - for values that should never be negative.
 * Used for invested capital, realized cap, etc.
 *
 * @typedef {number} CentsUnsigned
 */
/**
 * Closing price value for a time period
 *
 * @typedef {CentsUnsigned} Close
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
 * @typedef {CentsUnsigned} High
 */
/**
 * Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
 * block-based (height, txindex), and address/output type indexes.
 *
 * @typedef {("dateindex"|"decadeindex"|"difficultyepoch"|"emptyoutputindex"|"halvingepoch"|"height"|"txinindex"|"monthindex"|"opreturnindex"|"txoutindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"quarterindex"|"semesterindex"|"txindex"|"unknownoutputindex"|"weekindex"|"yearindex"|"fundedaddressindex"|"emptyaddressindex"|"pairoutputindex")} Index
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
 * @typedef {CentsUnsigned} Low
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
 * @typedef {Object} OHLCCentsUnsigned
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
 * @typedef {CentsUnsigned} Open
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

// Date conversion constants and helpers
const _GENESIS = new Date(2009, 0, 3);  // dateindex 0, weekindex 0
const _DAY_ONE = new Date(2009, 0, 9);  // dateindex 1 (6 day gap after genesis)
const _MS_PER_DAY = 24 * 60 * 60 * 1000;
const _MS_PER_WEEK = 7 * _MS_PER_DAY;
const _DATE_INDEXES = new Set(['dateindex', 'weekindex', 'monthindex', 'yearindex', 'quarterindex', 'semesterindex', 'decadeindex']);

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
    case 'dateindex': return i === 0 ? _GENESIS : new Date(_DAY_ONE.getTime() + (i - 1) * _MS_PER_DAY);
    case 'weekindex': return new Date(_GENESIS.getTime() + i * _MS_PER_WEEK);
    case 'monthindex': return _addMonths(i);
    case 'yearindex': return new Date(2009 + i, 0, 1);
    case 'quarterindex': return _addMonths(i * 3);
    case 'semesterindex': return _addMonths(i * 6);
    case 'decadeindex': return new Date(2009 + i * 10, 0, 1);
    default: throw new Error(`${index} is not a date-based index`);
  }
}

/**
 * Check if an index type is date-based.
 * @param {Index} index
 * @returns {boolean}
 */
function isDateIndex(index) {
  return _DATE_INDEXES.has(index);
}

/**
 * Wrap raw metric data with helper methods.
 * @template T
 * @param {MetricData<T>} raw - Raw JSON response
 * @returns {MetricData<T>}
 */
function _wrapMetricData(raw) {
  const { index, start, end, data } = raw;
  return /** @type {MetricData<T>} */ ({
    ...raw,
    dates() {
      /** @type {globalThis.Date[]} */
      const result = [];
      for (let i = start; i < end; i++) result.push(indexToDate(index, i));
      return result;
    },
    indexes() {
      /** @type {number[]} */
      const result = [];
      for (let i = start; i < end; i++) result.push(i);
      return result;
    },
    toDateMap() {
      /** @type {Map<globalThis.Date, T>} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(indexToDate(index, start + i), data[i]);
      return map;
    },
    toIndexMap() {
      /** @type {Map<number, T>} */
      const map = new Map();
      for (let i = 0; i < data.length; i++) map.set(start + i, data[i]);
      return map;
    },
    dateEntries() {
      /** @type {Array<[globalThis.Date, T]>} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([indexToDate(index, start + i), data[i]]);
      return result;
    },
    indexEntries() {
      /** @type {Array<[number, T]>} */
      const result = [];
      for (let i = 0; i < data.length; i++) result.push([start + i, data[i]]);
      return result;
    },
    *iter() {
      for (let i = 0; i < data.length; i++) yield [start + i, data[i]];
    },
    *iterDates() {
      for (let i = 0; i < data.length; i++) yield [indexToDate(index, start + i), data[i]];
    },
    [Symbol.iterator]() {
      return this.iter();
    },
  });
}

/**
 * @template T
 * @typedef {Object} MetricData
 * @property {number} version - Version of the metric data
 * @property {Index} index - The index type used for this query
 * @property {number} total - Total number of data points
 * @property {number} start - Start index (inclusive)
 * @property {number} end - End index (exclusive)
 * @property {string} stamp - ISO 8601 timestamp of when the response was generated
 * @property {T[]} data - The metric data
 * @property {() => globalThis.Date[]} dates - Convert index range to dates (date-based indexes only)
 * @property {() => number[]} indexes - Get index range as array
 * @property {() => Map<globalThis.Date, T>} toDateMap - Return data as Map keyed by date (date-based only)
 * @property {() => Map<number, T>} toIndexMap - Return data as Map keyed by index
 * @property {() => Array<[globalThis.Date, T]>} dateEntries - Return data as [date, value] pairs (date-based only)
 * @property {() => Array<[number, T]>} indexEntries - Return data as [index, value] pairs
 * @property {() => IterableIterator<[number, T]>} iter - Iterate over [index, value] pairs
 * @property {() => IterableIterator<[globalThis.Date, T]>} iterDates - Iterate over [date, value] pairs (date-based only)
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
    fetch(onUpdate) { return client._fetchMetricData(buildPath(start, end), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, end, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} idx
   * @returns {SingleItemBuilder<T>}
   */
  const singleItemBuilder = (idx) => ({
    fetch(onUpdate) { return client._fetchMetricData(buildPath(idx, idx + 1), onUpdate); },
    fetchCsv() { return client.getText(buildPath(idx, idx + 1, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /**
   * @param {number} start
   * @returns {SkippedBuilder<T>}
   */
  const skippedBuilder = (start) => ({
    take(n) { return rangeBuilder(start, start + n); },
    fetch(onUpdate) { return client._fetchMetricData(buildPath(start, undefined), onUpdate); },
    fetchCsv() { return client.getText(buildPath(start, undefined, 'csv')); },
    then(resolve, reject) { return this.fetch().then(resolve, reject); },
  });

  /** @type {MetricEndpointBuilder<T>} */
  const endpoint = {
    get(idx) { return singleItemBuilder(idx); },
    slice(start, end) { return rangeBuilder(start, end); },
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

  /**
   * Fetch metric data and wrap with helper methods (internal)
   * @template T
   * @param {string} path
   * @param {(value: MetricData<T>) => void} [onUpdate]
   * @returns {Promise<MetricData<T>>}
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
const _i31 = /** @type {const} */ (["fundedaddressindex"]);
const _i32 = /** @type {const} */ (["emptyaddressindex"]);

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
/** @template T @typedef {{ name: string, by: { readonly fundedaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern31 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern31<T>} */
function createMetricPattern31(client, name) { return _mp(client, name, _i31); }
/** @template T @typedef {{ name: string, by: { readonly emptyaddressindex: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern32 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern32<T>} */
function createMetricPattern32(client, name) { return _mp(client, name, _i32); }

// Reusable structural pattern factories

/**
 * @typedef {Object} AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern
 * @property {MetricPattern6<StoredF64>} adjustedSopr
 * @property {MetricPattern6<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern6<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern11<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern11<CentsSquaredSats>} investorCapRaw
 * @property {DollarsSatsPattern} investorPrice
 * @property {MetricPattern1<CentsUnsigned>} investorPriceCents
 * @property {RatioPattern} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {CumulativeSumPattern2<Dollars>} negRealizedLoss
 * @property {CumulativeSumPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {CumulativeSumPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeSumPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<CentsUnsigned>} realizedCapCents
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {CumulativeSumPattern<Dollars>} realizedLoss
 * @property {MetricPattern4<Dollars>} realizedLoss7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {DollarsSatsPattern} realizedPrice
 * @property {RatioPattern} realizedPriceExtra
 * @property {CumulativeSumPattern<Dollars>} realizedProfit
 * @property {MetricPattern4<Dollars>} realizedProfit7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern6<StoredF64>} realizedProfitToLossRatio
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {BitcoinDollarsSatsPattern3} sentInLoss
 * @property {BitcoinDollarsSatsPattern5} sentInLoss14dEma
 * @property {BitcoinDollarsSatsPattern3} sentInProfit
 * @property {BitcoinDollarsSatsPattern5} sentInProfit14dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern}
 */
function createAdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc) {
  return {
    adjustedSopr: createMetricPattern6(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    capRaw: createMetricPattern11(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern11(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createDollarsSatsPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createCumulativeSumPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeSumPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeSumPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern4(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createDollarsSatsPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeSumPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern4(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2
 * @property {MetricPattern6<StoredF64>} adjustedSopr
 * @property {MetricPattern6<StoredF64>} adjustedSopr30dEma
 * @property {MetricPattern6<StoredF64>} adjustedSopr7dEma
 * @property {MetricPattern1<Dollars>} adjustedValueCreated
 * @property {MetricPattern1<Dollars>} adjustedValueDestroyed
 * @property {MetricPattern11<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern11<CentsSquaredSats>} investorCapRaw
 * @property {DollarsSatsPattern} investorPrice
 * @property {MetricPattern1<CentsUnsigned>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {CumulativeSumPattern2<Dollars>} negRealizedLoss
 * @property {CumulativeSumPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {CumulativeSumPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeSumPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<CentsUnsigned>} realizedCapCents
 * @property {CumulativeSumPattern<Dollars>} realizedLoss
 * @property {MetricPattern4<Dollars>} realizedLoss7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {DollarsSatsPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {CumulativeSumPattern<Dollars>} realizedProfit
 * @property {MetricPattern4<Dollars>} realizedProfit7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {BitcoinDollarsSatsPattern3} sentInLoss
 * @property {BitcoinDollarsSatsPattern5} sentInLoss14dEma
 * @property {BitcoinDollarsSatsPattern3} sentInProfit
 * @property {BitcoinDollarsSatsPattern5} sentInProfit14dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2}
 */
function createAdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc) {
  return {
    adjustedSopr: createMetricPattern6(client, _m(acc, 'adjusted_sopr')),
    adjustedSopr30dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema')),
    adjustedSopr7dEma: createMetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema')),
    adjustedValueCreated: createMetricPattern1(client, _m(acc, 'adjusted_value_created')),
    adjustedValueDestroyed: createMetricPattern1(client, _m(acc, 'adjusted_value_destroyed')),
    capRaw: createMetricPattern11(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern11(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createDollarsSatsPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createCumulativeSumPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeSumPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedLoss: createCumulativeSumPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern4(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createDollarsSatsPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeSumPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern4(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2
 * @property {MetricPattern11<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern11<CentsSquaredSats>} investorCapRaw
 * @property {DollarsSatsPattern} investorPrice
 * @property {MetricPattern1<CentsUnsigned>} investorPriceCents
 * @property {RatioPattern} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {CumulativeSumPattern2<Dollars>} negRealizedLoss
 * @property {CumulativeSumPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {CumulativeSumPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeSumPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<CentsUnsigned>} realizedCapCents
 * @property {MetricPattern1<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {CumulativeSumPattern<Dollars>} realizedLoss
 * @property {MetricPattern4<Dollars>} realizedLoss7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {DollarsSatsPattern} realizedPrice
 * @property {RatioPattern} realizedPriceExtra
 * @property {CumulativeSumPattern<Dollars>} realizedProfit
 * @property {MetricPattern4<Dollars>} realizedProfit7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern6<StoredF64>} realizedProfitToLossRatio
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {BitcoinDollarsSatsPattern3} sentInLoss
 * @property {BitcoinDollarsSatsPattern5} sentInLoss14dEma
 * @property {BitcoinDollarsSatsPattern3} sentInProfit
 * @property {BitcoinDollarsSatsPattern5} sentInProfit14dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2}
 */
function createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc) {
  return {
    capRaw: createMetricPattern11(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern11(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createDollarsSatsPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createCumulativeSumPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeSumPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedCapRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap')),
    realizedLoss: createCumulativeSumPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern4(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createDollarsSatsPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeSumPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern4(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedProfitToLossRatio: createMetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern
 * @property {MetricPattern11<CentsSats>} capRaw
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {MetricPattern11<CentsSquaredSats>} investorCapRaw
 * @property {DollarsSatsPattern} investorPrice
 * @property {MetricPattern1<CentsUnsigned>} investorPriceCents
 * @property {RatioPattern2} investorPriceExtra
 * @property {MetricPattern1<Dollars>} lossValueCreated
 * @property {MetricPattern1<Dollars>} lossValueDestroyed
 * @property {MetricPattern4<StoredF32>} mvrv
 * @property {CumulativeSumPattern2<Dollars>} negRealizedLoss
 * @property {CumulativeSumPattern<Dollars>} netRealizedPnl
 * @property {MetricPattern4<Dollars>} netRealizedPnl7dEma
 * @property {MetricPattern4<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {MetricPattern4<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {CumulativeSumPattern<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {CumulativeSumPattern<Dollars>} peakRegret
 * @property {MetricPattern1<StoredF32>} peakRegretRelToRealizedCap
 * @property {MetricPattern1<Dollars>} profitFlow
 * @property {MetricPattern1<Dollars>} profitValueCreated
 * @property {MetricPattern1<Dollars>} profitValueDestroyed
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern4<Dollars>} realizedCap30dDelta
 * @property {MetricPattern1<CentsUnsigned>} realizedCapCents
 * @property {CumulativeSumPattern<Dollars>} realizedLoss
 * @property {MetricPattern4<Dollars>} realizedLoss7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedLossRelToRealizedCap
 * @property {DollarsSatsPattern} realizedPrice
 * @property {RatioPattern2} realizedPriceExtra
 * @property {CumulativeSumPattern<Dollars>} realizedProfit
 * @property {MetricPattern4<Dollars>} realizedProfit7dEma
 * @property {CumulativeSumPattern<StoredF32>} realizedProfitRelToRealizedCap
 * @property {MetricPattern1<Dollars>} realizedValue
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio30dEma
 * @property {MetricPattern6<StoredF32>} sellSideRiskRatio7dEma
 * @property {BitcoinDollarsSatsPattern3} sentInLoss
 * @property {BitcoinDollarsSatsPattern5} sentInLoss14dEma
 * @property {BitcoinDollarsSatsPattern3} sentInProfit
 * @property {BitcoinDollarsSatsPattern5} sentInProfit14dEma
 * @property {MetricPattern6<StoredF64>} sopr
 * @property {MetricPattern6<StoredF64>} sopr30dEma
 * @property {MetricPattern6<StoredF64>} sopr7dEma
 * @property {MetricPattern1<Dollars>} totalRealizedPnl
 * @property {MetricPattern1<Dollars>} valueCreated
 * @property {MetricPattern1<Dollars>} valueDestroyed
 */

/**
 * Create a CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern}
 */
function createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc) {
  return {
    capRaw: createMetricPattern11(client, _m(acc, 'cap_raw')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    investorCapRaw: createMetricPattern11(client, _m(acc, 'investor_cap_raw')),
    investorPrice: createDollarsSatsPattern(client, _m(acc, 'investor_price')),
    investorPriceCents: createMetricPattern1(client, _m(acc, 'investor_price_cents')),
    investorPriceExtra: createRatioPattern2(client, _m(acc, 'investor_price_ratio')),
    lossValueCreated: createMetricPattern1(client, _m(acc, 'loss_value_created')),
    lossValueDestroyed: createMetricPattern1(client, _m(acc, 'loss_value_destroyed')),
    mvrv: createMetricPattern4(client, _m(acc, 'mvrv')),
    negRealizedLoss: createCumulativeSumPattern2(client, _m(acc, 'neg_realized_loss')),
    netRealizedPnl: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl')),
    netRealizedPnl7dEma: createMetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema')),
    netRealizedPnlCumulative30dDelta: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta')),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap')),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createMetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap')),
    netRealizedPnlRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap')),
    peakRegret: createCumulativeSumPattern(client, _m(acc, 'realized_peak_regret')),
    peakRegretRelToRealizedCap: createMetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap')),
    profitFlow: createMetricPattern1(client, _m(acc, 'profit_flow')),
    profitValueCreated: createMetricPattern1(client, _m(acc, 'profit_value_created')),
    profitValueDestroyed: createMetricPattern1(client, _m(acc, 'profit_value_destroyed')),
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    realizedCap30dDelta: createMetricPattern4(client, _m(acc, 'realized_cap_30d_delta')),
    realizedCapCents: createMetricPattern1(client, _m(acc, 'realized_cap_cents')),
    realizedLoss: createCumulativeSumPattern(client, _m(acc, 'realized_loss')),
    realizedLoss7dEma: createMetricPattern4(client, _m(acc, 'realized_loss_7d_ema')),
    realizedLossRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap')),
    realizedPrice: createDollarsSatsPattern(client, _m(acc, 'realized_price')),
    realizedPriceExtra: createRatioPattern2(client, _m(acc, 'realized_price_ratio')),
    realizedProfit: createCumulativeSumPattern(client, _m(acc, 'realized_profit')),
    realizedProfit7dEma: createMetricPattern4(client, _m(acc, 'realized_profit_7d_ema')),
    realizedProfitRelToRealizedCap: createCumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap')),
    realizedValue: createMetricPattern1(client, _m(acc, 'realized_value')),
    sellSideRiskRatio: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio')),
    sellSideRiskRatio30dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema')),
    sellSideRiskRatio7dEma: createMetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema')),
    sentInLoss: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss')),
    sentInLoss14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema')),
    sentInProfit: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit')),
    sentInProfit14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema')),
    sopr: createMetricPattern6(client, _m(acc, 'sopr')),
    sopr30dEma: createMetricPattern6(client, _m(acc, 'sopr_30d_ema')),
    sopr7dEma: createMetricPattern6(client, _m(acc, 'sopr_7d_ema')),
    totalRealizedPnl: createMetricPattern1(client, _m(acc, 'total_realized_pnl')),
    valueCreated: createMetricPattern1(client, _m(acc, 'value_created')),
    valueDestroyed: createMetricPattern1(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern
 * @property {DollarsSatsPattern2} _0sdUsd
 * @property {MetricPattern4<StoredF32>} m05sd
 * @property {DollarsSatsPattern2} m05sdUsd
 * @property {MetricPattern4<StoredF32>} m15sd
 * @property {DollarsSatsPattern2} m15sdUsd
 * @property {MetricPattern4<StoredF32>} m1sd
 * @property {DollarsSatsPattern2} m1sdUsd
 * @property {MetricPattern4<StoredF32>} m25sd
 * @property {DollarsSatsPattern2} m25sdUsd
 * @property {MetricPattern4<StoredF32>} m2sd
 * @property {DollarsSatsPattern2} m2sdUsd
 * @property {MetricPattern4<StoredF32>} m3sd
 * @property {DollarsSatsPattern2} m3sdUsd
 * @property {MetricPattern4<StoredF32>} p05sd
 * @property {DollarsSatsPattern2} p05sdUsd
 * @property {MetricPattern4<StoredF32>} p15sd
 * @property {DollarsSatsPattern2} p15sdUsd
 * @property {MetricPattern4<StoredF32>} p1sd
 * @property {DollarsSatsPattern2} p1sdUsd
 * @property {MetricPattern4<StoredF32>} p25sd
 * @property {DollarsSatsPattern2} p25sdUsd
 * @property {MetricPattern4<StoredF32>} p2sd
 * @property {DollarsSatsPattern2} p2sdUsd
 * @property {MetricPattern4<StoredF32>} p3sd
 * @property {DollarsSatsPattern2} p3sdUsd
 * @property {MetricPattern4<StoredF32>} sd
 * @property {MetricPattern4<StoredF32>} sma
 * @property {MetricPattern4<StoredF32>} zscore
 */

/**
 * Create a _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern}
 */
function create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc) {
  return {
    _0sdUsd: createDollarsSatsPattern2(client, _m(acc, '0sd_usd')),
    m05sd: createMetricPattern4(client, _m(acc, 'm0_5sd')),
    m05sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm0_5sd_usd')),
    m15sd: createMetricPattern4(client, _m(acc, 'm1_5sd')),
    m15sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm1_5sd_usd')),
    m1sd: createMetricPattern4(client, _m(acc, 'm1sd')),
    m1sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm1sd_usd')),
    m25sd: createMetricPattern4(client, _m(acc, 'm2_5sd')),
    m25sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm2_5sd_usd')),
    m2sd: createMetricPattern4(client, _m(acc, 'm2sd')),
    m2sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm2sd_usd')),
    m3sd: createMetricPattern4(client, _m(acc, 'm3sd')),
    m3sdUsd: createDollarsSatsPattern2(client, _m(acc, 'm3sd_usd')),
    p05sd: createMetricPattern4(client, _m(acc, 'p0_5sd')),
    p05sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p0_5sd_usd')),
    p15sd: createMetricPattern4(client, _m(acc, 'p1_5sd')),
    p15sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p1_5sd_usd')),
    p1sd: createMetricPattern4(client, _m(acc, 'p1sd')),
    p1sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p1sd_usd')),
    p25sd: createMetricPattern4(client, _m(acc, 'p2_5sd')),
    p25sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p2_5sd_usd')),
    p2sd: createMetricPattern4(client, _m(acc, 'p2sd')),
    p2sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p2sd_usd')),
    p3sd: createMetricPattern4(client, _m(acc, 'p3sd')),
    p3sdUsd: createDollarsSatsPattern2(client, _m(acc, 'p3sd_usd')),
    sd: createMetricPattern4(client, _m(acc, 'sd')),
    sma: createMetricPattern4(client, _m(acc, 'sma')),
    zscore: createMetricPattern4(client, _m(acc, 'zscore')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern4
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
 * @property {MetricPattern4<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern4<StoredF32>} unrealizedPeakRegretRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
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
    unrealizedPeakRegretRelToMarketCap: createMetricPattern4(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
    unrealizedProfitRelToOwnMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap')),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl')),
  };
}

/**
 * @typedef {Object} PriceRatioPattern
 * @property {DollarsSatsPattern2} price
 * @property {MetricPattern4<StoredF32>} ratio
 * @property {MetricPattern4<StoredF32>} ratio1mSma
 * @property {MetricPattern4<StoredF32>} ratio1wSma
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio1ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio2ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio4ySd
 * @property {MetricPattern4<StoredF32>} ratioPct1
 * @property {DollarsSatsPattern2} ratioPct1Usd
 * @property {MetricPattern4<StoredF32>} ratioPct2
 * @property {DollarsSatsPattern2} ratioPct2Usd
 * @property {MetricPattern4<StoredF32>} ratioPct5
 * @property {DollarsSatsPattern2} ratioPct5Usd
 * @property {MetricPattern4<StoredF32>} ratioPct95
 * @property {DollarsSatsPattern2} ratioPct95Usd
 * @property {MetricPattern4<StoredF32>} ratioPct98
 * @property {DollarsSatsPattern2} ratioPct98Usd
 * @property {MetricPattern4<StoredF32>} ratioPct99
 * @property {DollarsSatsPattern2} ratioPct99Usd
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
    price: createDollarsSatsPattern2(client, acc),
    ratio: createMetricPattern4(client, _m(acc, 'ratio')),
    ratio1mSma: createMetricPattern4(client, _m(acc, 'ratio_1m_sma')),
    ratio1wSma: createMetricPattern4(client, _m(acc, 'ratio_1w_sma')),
    ratio1ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_1y')),
    ratio2ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_2y')),
    ratio4ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_4y')),
    ratioPct1: createMetricPattern4(client, _m(acc, 'ratio_pct1')),
    ratioPct1Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct1_usd')),
    ratioPct2: createMetricPattern4(client, _m(acc, 'ratio_pct2')),
    ratioPct2Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct2_usd')),
    ratioPct5: createMetricPattern4(client, _m(acc, 'ratio_pct5')),
    ratioPct5Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct5_usd')),
    ratioPct95: createMetricPattern4(client, _m(acc, 'ratio_pct95')),
    ratioPct95Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct95_usd')),
    ratioPct98: createMetricPattern4(client, _m(acc, 'ratio_pct98')),
    ratioPct98Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct98_usd')),
    ratioPct99: createMetricPattern4(client, _m(acc, 'ratio_pct99')),
    ratioPct99Usd: createDollarsSatsPattern2(client, _m(acc, 'ratio_pct99_usd')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern
 * @property {DollarsSatsPattern2} pct05
 * @property {DollarsSatsPattern2} pct10
 * @property {DollarsSatsPattern2} pct15
 * @property {DollarsSatsPattern2} pct20
 * @property {DollarsSatsPattern2} pct25
 * @property {DollarsSatsPattern2} pct30
 * @property {DollarsSatsPattern2} pct35
 * @property {DollarsSatsPattern2} pct40
 * @property {DollarsSatsPattern2} pct45
 * @property {DollarsSatsPattern2} pct50
 * @property {DollarsSatsPattern2} pct55
 * @property {DollarsSatsPattern2} pct60
 * @property {DollarsSatsPattern2} pct65
 * @property {DollarsSatsPattern2} pct70
 * @property {DollarsSatsPattern2} pct75
 * @property {DollarsSatsPattern2} pct80
 * @property {DollarsSatsPattern2} pct85
 * @property {DollarsSatsPattern2} pct90
 * @property {DollarsSatsPattern2} pct95
 */

/**
 * Create a Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern}
 */
function createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, acc) {
  return {
    pct05: createDollarsSatsPattern2(client, _m(acc, 'pct05')),
    pct10: createDollarsSatsPattern2(client, _m(acc, 'pct10')),
    pct15: createDollarsSatsPattern2(client, _m(acc, 'pct15')),
    pct20: createDollarsSatsPattern2(client, _m(acc, 'pct20')),
    pct25: createDollarsSatsPattern2(client, _m(acc, 'pct25')),
    pct30: createDollarsSatsPattern2(client, _m(acc, 'pct30')),
    pct35: createDollarsSatsPattern2(client, _m(acc, 'pct35')),
    pct40: createDollarsSatsPattern2(client, _m(acc, 'pct40')),
    pct45: createDollarsSatsPattern2(client, _m(acc, 'pct45')),
    pct50: createDollarsSatsPattern2(client, _m(acc, 'pct50')),
    pct55: createDollarsSatsPattern2(client, _m(acc, 'pct55')),
    pct60: createDollarsSatsPattern2(client, _m(acc, 'pct60')),
    pct65: createDollarsSatsPattern2(client, _m(acc, 'pct65')),
    pct70: createDollarsSatsPattern2(client, _m(acc, 'pct70')),
    pct75: createDollarsSatsPattern2(client, _m(acc, 'pct75')),
    pct80: createDollarsSatsPattern2(client, _m(acc, 'pct80')),
    pct85: createDollarsSatsPattern2(client, _m(acc, 'pct85')),
    pct90: createDollarsSatsPattern2(client, _m(acc, 'pct90')),
    pct95: createDollarsSatsPattern2(client, _m(acc, 'pct95')),
  };
}

/**
 * @typedef {Object} RatioPattern
 * @property {MetricPattern4<StoredF32>} ratio
 * @property {MetricPattern4<StoredF32>} ratio1mSma
 * @property {MetricPattern4<StoredF32>} ratio1wSma
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio1ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio2ySd
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern} ratio4ySd
 * @property {MetricPattern4<StoredF32>} ratioPct1
 * @property {DollarsSatsPattern2} ratioPct1Usd
 * @property {MetricPattern4<StoredF32>} ratioPct2
 * @property {DollarsSatsPattern2} ratioPct2Usd
 * @property {MetricPattern4<StoredF32>} ratioPct5
 * @property {DollarsSatsPattern2} ratioPct5Usd
 * @property {MetricPattern4<StoredF32>} ratioPct95
 * @property {DollarsSatsPattern2} ratioPct95Usd
 * @property {MetricPattern4<StoredF32>} ratioPct98
 * @property {DollarsSatsPattern2} ratioPct98Usd
 * @property {MetricPattern4<StoredF32>} ratioPct99
 * @property {DollarsSatsPattern2} ratioPct99Usd
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
    ratio: createMetricPattern4(client, acc),
    ratio1mSma: createMetricPattern4(client, _m(acc, '1m_sma')),
    ratio1wSma: createMetricPattern4(client, _m(acc, '1w_sma')),
    ratio1ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y')),
    ratio2ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y')),
    ratio4ySd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y')),
    ratioPct1: createMetricPattern4(client, _m(acc, 'pct1')),
    ratioPct1Usd: createDollarsSatsPattern2(client, _m(acc, 'pct1_usd')),
    ratioPct2: createMetricPattern4(client, _m(acc, 'pct2')),
    ratioPct2Usd: createDollarsSatsPattern2(client, _m(acc, 'pct2_usd')),
    ratioPct5: createMetricPattern4(client, _m(acc, 'pct5')),
    ratioPct5Usd: createDollarsSatsPattern2(client, _m(acc, 'pct5_usd')),
    ratioPct95: createMetricPattern4(client, _m(acc, 'pct95')),
    ratioPct95Usd: createDollarsSatsPattern2(client, _m(acc, 'pct95_usd')),
    ratioPct98: createMetricPattern4(client, _m(acc, 'pct98')),
    ratioPct98Usd: createDollarsSatsPattern2(client, _m(acc, 'pct98_usd')),
    ratioPct99: createMetricPattern4(client, _m(acc, 'pct99')),
    ratioPct99Usd: createDollarsSatsPattern2(client, _m(acc, 'pct99_usd')),
    ratioSd: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc),
  };
}

/**
 * @typedef {Object} GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern
 * @property {MetricPattern1<Dollars>} greedIndex
 * @property {MetricPattern1<Dollars>} investedCapitalInLoss
 * @property {MetricPattern11<CentsSats>} investedCapitalInLossRaw
 * @property {MetricPattern1<Dollars>} investedCapitalInProfit
 * @property {MetricPattern11<CentsSats>} investedCapitalInProfitRaw
 * @property {MetricPattern11<CentsSquaredSats>} investorCapInLossRaw
 * @property {MetricPattern11<CentsSquaredSats>} investorCapInProfitRaw
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {MetricPattern1<Dollars>} netSentiment
 * @property {MetricPattern1<Dollars>} netUnrealizedPnl
 * @property {MetricPattern1<Dollars>} painIndex
 * @property {MetricPattern4<Dollars>} peakRegret
 * @property {BitcoinDollarsSatsPattern4} supplyInLoss
 * @property {BitcoinDollarsSatsPattern4} supplyInProfit
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
    investedCapitalInLossRaw: createMetricPattern11(client, _m(acc, 'invested_capital_in_loss_raw')),
    investedCapitalInProfit: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit')),
    investedCapitalInProfitRaw: createMetricPattern11(client, _m(acc, 'invested_capital_in_profit_raw')),
    investorCapInLossRaw: createMetricPattern11(client, _m(acc, 'investor_cap_in_loss_raw')),
    investorCapInProfitRaw: createMetricPattern11(client, _m(acc, 'investor_cap_in_profit_raw')),
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netSentiment: createMetricPattern1(client, _m(acc, 'net_sentiment')),
    netUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl')),
    painIndex: createMetricPattern1(client, _m(acc, 'pain_index')),
    peakRegret: createMetricPattern4(client, _m(acc, 'unrealized_peak_regret')),
    supplyInLoss: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern1(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern1(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern
 * @property {MetricPattern1<Dollars>} greedIndex
 * @property {MetricPattern1<Dollars>} investedCapitalInLoss
 * @property {MetricPattern11<CentsSats>} investedCapitalInLossRaw
 * @property {MetricPattern1<Dollars>} investedCapitalInProfit
 * @property {MetricPattern11<CentsSats>} investedCapitalInProfitRaw
 * @property {MetricPattern11<CentsSquaredSats>} investorCapInLossRaw
 * @property {MetricPattern11<CentsSquaredSats>} investorCapInProfitRaw
 * @property {MetricPattern1<Dollars>} negUnrealizedLoss
 * @property {MetricPattern1<Dollars>} netSentiment
 * @property {MetricPattern1<Dollars>} netUnrealizedPnl
 * @property {MetricPattern1<Dollars>} painIndex
 * @property {BitcoinDollarsSatsPattern4} supplyInLoss
 * @property {BitcoinDollarsSatsPattern4} supplyInProfit
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
    investedCapitalInLossRaw: createMetricPattern11(client, _m(acc, 'invested_capital_in_loss_raw')),
    investedCapitalInProfit: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit')),
    investedCapitalInProfitRaw: createMetricPattern11(client, _m(acc, 'invested_capital_in_profit_raw')),
    investorCapInLossRaw: createMetricPattern11(client, _m(acc, 'investor_cap_in_loss_raw')),
    investorCapInProfitRaw: createMetricPattern11(client, _m(acc, 'investor_cap_in_profit_raw')),
    negUnrealizedLoss: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    netSentiment: createMetricPattern1(client, _m(acc, 'net_sentiment')),
    netUnrealizedPnl: createMetricPattern1(client, _m(acc, 'net_unrealized_pnl')),
    painIndex: createMetricPattern1(client, _m(acc, 'pain_index')),
    supplyInLoss: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_loss')),
    supplyInProfit: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_profit')),
    totalUnrealizedPnl: createMetricPattern1(client, _m(acc, 'total_unrealized_pnl')),
    unrealizedLoss: createMetricPattern1(client, _m(acc, 'unrealized_loss')),
    unrealizedProfit: createMetricPattern1(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern
 * @property {MetricPattern1<StoredU32>} _1mBlocksMined
 * @property {MetricPattern1<StoredF32>} _1mDominance
 * @property {MetricPattern1<StoredU32>} _1wBlocksMined
 * @property {MetricPattern1<StoredF32>} _1wDominance
 * @property {MetricPattern1<StoredU32>} _1yBlocksMined
 * @property {MetricPattern1<StoredF32>} _1yDominance
 * @property {MetricPattern1<StoredU32>} _24hBlocksMined
 * @property {MetricPattern1<StoredF32>} _24hDominance
 * @property {CumulativeSumPattern<StoredU32>} blocksMined
 * @property {MetricPattern1<StoredU32>} blocksSinceBlock
 * @property {BitcoinDollarsSatsPattern6} coinbase
 * @property {MetricPattern4<StoredU16>} daysSinceBlock
 * @property {MetricPattern1<StoredF32>} dominance
 * @property {BitcoinDollarsSatsPattern3} fee
 * @property {BitcoinDollarsSatsPattern3} subsidy
 */

/**
 * Create a _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern}
 */
function create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, acc) {
  return {
    _1mBlocksMined: createMetricPattern1(client, _m(acc, '1m_blocks_mined')),
    _1mDominance: createMetricPattern1(client, _m(acc, '1m_dominance')),
    _1wBlocksMined: createMetricPattern1(client, _m(acc, '1w_blocks_mined')),
    _1wDominance: createMetricPattern1(client, _m(acc, '1w_dominance')),
    _1yBlocksMined: createMetricPattern1(client, _m(acc, '1y_blocks_mined')),
    _1yDominance: createMetricPattern1(client, _m(acc, '1y_dominance')),
    _24hBlocksMined: createMetricPattern1(client, _m(acc, '24h_blocks_mined')),
    _24hDominance: createMetricPattern1(client, _m(acc, '24h_dominance')),
    blocksMined: createCumulativeSumPattern(client, _m(acc, 'blocks_mined')),
    blocksSinceBlock: createMetricPattern1(client, _m(acc, 'blocks_since_block')),
    coinbase: createBitcoinDollarsSatsPattern6(client, _m(acc, 'coinbase')),
    daysSinceBlock: createMetricPattern4(client, _m(acc, 'days_since_block')),
    dominance: createMetricPattern1(client, _m(acc, 'dominance')),
    fee: createBitcoinDollarsSatsPattern3(client, _m(acc, 'fee')),
    subsidy: createBitcoinDollarsSatsPattern3(client, _m(acc, 'subsidy')),
  };
}

/**
 * @typedef {Object} InvestedNegNetNuplSupplyUnrealizedPattern3
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {MetricPattern1<StoredF32>} nupl
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {MetricPattern4<StoredF64>} supplyRelToCirculatingSupply
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToMarketCap
 * @property {MetricPattern4<StoredF32>} unrealizedPeakRegretRelToMarketCap
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a InvestedNegNetNuplSupplyUnrealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetNuplSupplyUnrealizedPattern3}
 */
function createInvestedNegNetNuplSupplyUnrealizedPattern3(client, acc) {
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
    supplyRelToCirculatingSupply: createMetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedPeakRegretRelToMarketCap: createMetricPattern4(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern3
 * @property {BitcoinDollarsSatsPattern5} _10y
 * @property {BitcoinDollarsSatsPattern5} _1m
 * @property {BitcoinDollarsSatsPattern5} _1w
 * @property {BitcoinDollarsSatsPattern5} _1y
 * @property {BitcoinDollarsSatsPattern5} _2y
 * @property {BitcoinDollarsSatsPattern5} _3m
 * @property {BitcoinDollarsSatsPattern5} _3y
 * @property {BitcoinDollarsSatsPattern5} _4y
 * @property {BitcoinDollarsSatsPattern5} _5y
 * @property {BitcoinDollarsSatsPattern5} _6m
 * @property {BitcoinDollarsSatsPattern5} _6y
 * @property {BitcoinDollarsSatsPattern5} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, acc) {
  return {
    _10y: createBitcoinDollarsSatsPattern5(client, _p('10y', acc)),
    _1m: createBitcoinDollarsSatsPattern5(client, _p('1m', acc)),
    _1w: createBitcoinDollarsSatsPattern5(client, _p('1w', acc)),
    _1y: createBitcoinDollarsSatsPattern5(client, _p('1y', acc)),
    _2y: createBitcoinDollarsSatsPattern5(client, _p('2y', acc)),
    _3m: createBitcoinDollarsSatsPattern5(client, _p('3m', acc)),
    _3y: createBitcoinDollarsSatsPattern5(client, _p('3y', acc)),
    _4y: createBitcoinDollarsSatsPattern5(client, _p('4y', acc)),
    _5y: createBitcoinDollarsSatsPattern5(client, _p('5y', acc)),
    _6m: createBitcoinDollarsSatsPattern5(client, _p('6m', acc)),
    _6y: createBitcoinDollarsSatsPattern5(client, _p('6y', acc)),
    _8y: createBitcoinDollarsSatsPattern5(client, _p('8y', acc)),
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
 * @property {MetricPattern4<StoredF64>} supplyRelToCirculatingSupply
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
    supplyRelToCirculatingSupply: createMetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply')),
    unrealizedLossRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap')),
    unrealizedProfitRelToMarketCap: createMetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap')),
  };
}

/**
 * @typedef {Object} InvestedNegNetSupplyUnrealizedPattern
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
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
 * Create a InvestedNegNetSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedNegNetSupplyUnrealizedPattern}
 */
function createInvestedNegNetSupplyUnrealizedPattern(client, acc) {
  return {
    investedCapitalInLossPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct')),
    investedCapitalInProfitPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct')),
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
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern2
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
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T>}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, acc) {
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
 * @template T
 * @typedef {Object} _201520162017201820192020202120222023202420252026Pattern2
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
 * @property {MetricPattern4<T>} _2026
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
    _2015: createMetricPattern4(client, _m(acc, '2015_returns')),
    _2016: createMetricPattern4(client, _m(acc, '2016_returns')),
    _2017: createMetricPattern4(client, _m(acc, '2017_returns')),
    _2018: createMetricPattern4(client, _m(acc, '2018_returns')),
    _2019: createMetricPattern4(client, _m(acc, '2019_returns')),
    _2020: createMetricPattern4(client, _m(acc, '2020_returns')),
    _2021: createMetricPattern4(client, _m(acc, '2021_returns')),
    _2022: createMetricPattern4(client, _m(acc, '2022_returns')),
    _2023: createMetricPattern4(client, _m(acc, '2023_returns')),
    _2024: createMetricPattern4(client, _m(acc, '2024_returns')),
    _2025: createMetricPattern4(client, _m(acc, '2025_returns')),
    _2026: createMetricPattern4(client, _m(acc, '2026_returns')),
  };
}

/**
 * @typedef {Object} AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern
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
 * Create a AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern}
 */
function createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc) {
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
 * @typedef {Object} AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2
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
 * Create a AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T>}
 */
function createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, acc) {
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
 * @template T
 * @typedef {Object} AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2
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
 * Create a AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T>}
 */
function createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, acc) {
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
 * @typedef {Object} AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern
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
 * Create a AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern}
 */
function createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, acc) {
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
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern
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
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<T>}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(client, acc) {
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
 * @template T
 * @typedef {Object} AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern
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
 * Create a AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>}
 */
function createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc) {
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
 * @typedef {Object} ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MetricPattern1<StoredU64>} addrCount
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
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
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} _10y2y3y4y5y6y8yPattern
 * @property {MetricPattern4<StoredF32>} _10y
 * @property {MetricPattern4<StoredF32>} _2y
 * @property {MetricPattern4<StoredF32>} _3y
 * @property {MetricPattern4<StoredF32>} _4y
 * @property {MetricPattern4<StoredF32>} _5y
 * @property {MetricPattern4<StoredF32>} _6y
 * @property {MetricPattern4<StoredF32>} _8y
 */

/**
 * Create a _10y2y3y4y5y6y8yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y2y3y4y5y6y8yPattern}
 */
function create_10y2y3y4y5y6y8yPattern(client, acc) {
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
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2} realized
 * @property {InvestedNegNetSupplyUnrealizedPattern} relative
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
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc),
    relative: createInvestedNegNetSupplyUnrealizedPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern3} relative
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
    realized: createAdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern3(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern} unrealized
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
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern3} relative
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6}
 */
function createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc),
    relative: createInvestedNegNetNuplSupplyUnrealizedPattern3(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {InvestedSupplyPattern} relative
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
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc),
    relative: createInvestedSupplyPattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityCostOutputsRealizedSupplyUnrealizedPattern
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {MaxMinPattern} costBasis
 * @property {UtxoPattern} outputs
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {_30dHalvedTotalPattern} supply
 * @property {GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern} unrealized
 */

/**
 * Create a ActivityCostOutputsRealizedSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityCostOutputsRealizedSupplyUnrealizedPattern}
 */
function createActivityCostOutputsRealizedSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc),
    costBasis: createMaxMinPattern(client, acc),
    outputs: createUtxoPattern(client, _m(acc, 'utxo_count')),
    realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc),
    supply: create_30dHalvedTotalPattern(client, acc),
    unrealized: createGreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc),
  };
}

/**
 * @typedef {Object} BalanceBothReactivatedReceivingSendingPattern
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} balanceDecreased
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} balanceIncreased
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} both
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} reactivated
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} receiving
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>} sending
 */

/**
 * Create a BalanceBothReactivatedReceivingSendingPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BalanceBothReactivatedReceivingSendingPattern}
 */
function createBalanceBothReactivatedReceivingSendingPattern(client, acc) {
  return {
    balanceDecreased: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_decreased')),
    balanceIncreased: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_increased')),
    both: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'both')),
    reactivated: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'reactivated')),
    receiving: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'receiving')),
    sending: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'sending')),
  };
}

/**
 * @typedef {Object} CoinblocksCoindaysSatblocksSatdaysSentPattern
 * @property {CumulativeSumPattern<StoredF64>} coinblocksDestroyed
 * @property {CumulativeSumPattern<StoredF64>} coindaysDestroyed
 * @property {MetricPattern11<Sats>} satblocksDestroyed
 * @property {MetricPattern11<Sats>} satdaysDestroyed
 * @property {BitcoinDollarsSatsPattern3} sent
 * @property {BitcoinDollarsSatsPattern5} sent14dEma
 */

/**
 * Create a CoinblocksCoindaysSatblocksSatdaysSentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoinblocksCoindaysSatblocksSatdaysSentPattern}
 */
function createCoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc) {
  return {
    coinblocksDestroyed: createCumulativeSumPattern(client, _m(acc, 'coinblocks_destroyed')),
    coindaysDestroyed: createCumulativeSumPattern(client, _m(acc, 'coindays_destroyed')),
    satblocksDestroyed: createMetricPattern11(client, _m(acc, 'satblocks_destroyed')),
    satdaysDestroyed: createMetricPattern11(client, _m(acc, 'satdays_destroyed')),
    sent: createBitcoinDollarsSatsPattern3(client, _m(acc, 'sent')),
    sent14dEma: createBitcoinDollarsSatsPattern5(client, _m(acc, 'sent_14d_ema')),
  };
}

/**
 * @typedef {Object} InvestedMaxMinPercentilesSpotPattern
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} investedCapital
 * @property {DollarsSatsPattern} max
 * @property {DollarsSatsPattern} min
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} percentiles
 * @property {MetricPattern4<StoredF32>} spotCostBasisPercentile
 * @property {MetricPattern4<StoredF32>} spotInvestedCapitalPercentile
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
    max: createDollarsSatsPattern(client, _m(acc, 'max_cost_basis')),
    min: createDollarsSatsPattern(client, _m(acc, 'min_cost_basis')),
    percentiles: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis')),
    spotCostBasisPercentile: createMetricPattern4(client, _m(acc, 'spot_cost_basis_percentile')),
    spotInvestedCapitalPercentile: createMetricPattern4(client, _m(acc, 'spot_invested_capital_percentile')),
  };
}

/**
 * @typedef {Object} InvestedSupplyPattern
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF64>} supplyInLossRelToOwnSupply
 * @property {MetricPattern1<StoredF64>} supplyInProfitRelToOwnSupply
 */

/**
 * Create a InvestedSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedSupplyPattern}
 */
function createInvestedSupplyPattern(client, acc) {
  return {
    investedCapitalInLossPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct')),
    investedCapitalInProfitPct: createMetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct')),
    supplyInLossRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply')),
    supplyInProfitRelToOwnSupply: createMetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply')),
  };
}

/**
 * @template T
 * @typedef {Object} CloseHighLowOpenPattern2
 * @property {MetricPattern1<T>} close
 * @property {MetricPattern1<T>} high
 * @property {MetricPattern1<T>} low
 * @property {MetricPattern1<T>} open
 */

/**
 * Create a CloseHighLowOpenPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CloseHighLowOpenPattern2<T>}
 */
function createCloseHighLowOpenPattern2(client, acc) {
  return {
    close: createMetricPattern1(client, _m(acc, 'close')),
    high: createMetricPattern1(client, _m(acc, 'high')),
    low: createMetricPattern1(client, _m(acc, 'low')),
    open: createMetricPattern1(client, _m(acc, 'open')),
  };
}

/**
 * @typedef {Object} _30dHalvedTotalPattern
 * @property {BitcoinDollarsSatsPattern5} _30dChange
 * @property {BitcoinDollarsSatsPattern4} halved
 * @property {BitcoinDollarsSatsPattern4} total
 */

/**
 * Create a _30dHalvedTotalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_30dHalvedTotalPattern}
 */
function create_30dHalvedTotalPattern(client, acc) {
  return {
    _30dChange: createBitcoinDollarsSatsPattern5(client, _m(acc, '_30d_change')),
    halved: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply_halved')),
    total: createBitcoinDollarsSatsPattern4(client, _m(acc, 'supply')),
  };
}

/**
 * @typedef {Object} BaseCumulativeSumPattern
 * @property {MetricPattern11<StoredF32>} base
 * @property {MetricPattern2<StoredF32>} cumulative
 * @property {MetricPattern2<StoredF32>} sum
 */

/**
 * Create a BaseCumulativeSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern}
 */
function createBaseCumulativeSumPattern(client, acc) {
  return {
    base: createMetricPattern11(client, acc),
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    sum: createMetricPattern2(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BitcoinDollarsSatsPattern2
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} bitcoin
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Dollars>} dollars
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Sats>} sats
 */

/**
 * Create a BitcoinDollarsSatsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinDollarsSatsPattern2}
 */
function createBitcoinDollarsSatsPattern2(client, acc) {
  return {
    bitcoin: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, _m(acc, 'btc')),
    dollars: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, 'usd')),
    sats: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, acc),
  };
}

/**
 * @typedef {Object} BitcoinDollarsSatsPattern4
 * @property {MetricPattern1<Bitcoin>} bitcoin
 * @property {MetricPattern1<Dollars>} dollars
 * @property {MetricPattern1<Sats>} sats
 */

/**
 * Create a BitcoinDollarsSatsPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinDollarsSatsPattern4}
 */
function createBitcoinDollarsSatsPattern4(client, acc) {
  return {
    bitcoin: createMetricPattern1(client, _m(acc, 'btc')),
    dollars: createMetricPattern1(client, _m(acc, 'usd')),
    sats: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} BitcoinDollarsSatsPattern5
 * @property {MetricPattern4<Bitcoin>} bitcoin
 * @property {MetricPattern4<Dollars>} dollars
 * @property {MetricPattern4<Sats>} sats
 */

/**
 * Create a BitcoinDollarsSatsPattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinDollarsSatsPattern5}
 */
function createBitcoinDollarsSatsPattern5(client, acc) {
  return {
    bitcoin: createMetricPattern4(client, _m(acc, 'btc')),
    dollars: createMetricPattern4(client, _m(acc, 'usd')),
    sats: createMetricPattern4(client, acc),
  };
}

/**
 * @typedef {Object} BitcoinDollarsSatsPattern6
 * @property {CumulativeSumPattern<Bitcoin>} bitcoin
 * @property {CumulativeSumPattern<Dollars>} dollars
 * @property {CumulativeSumPattern<Sats>} sats
 */

/**
 * Create a BitcoinDollarsSatsPattern6 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinDollarsSatsPattern6}
 */
function createBitcoinDollarsSatsPattern6(client, acc) {
  return {
    bitcoin: createCumulativeSumPattern(client, _m(acc, 'btc')),
    dollars: createCumulativeSumPattern(client, _m(acc, 'usd')),
    sats: createCumulativeSumPattern(client, acc),
  };
}

/**
 * @typedef {Object} BitcoinDollarsSatsPattern3
 * @property {CumulativeSumPattern2<Bitcoin>} bitcoin
 * @property {CumulativeSumPattern<Dollars>} dollars
 * @property {CumulativeSumPattern<Sats>} sats
 */

/**
 * Create a BitcoinDollarsSatsPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BitcoinDollarsSatsPattern3}
 */
function createBitcoinDollarsSatsPattern3(client, acc) {
  return {
    bitcoin: createCumulativeSumPattern2(client, _m(acc, 'btc')),
    dollars: createCumulativeSumPattern(client, _m(acc, 'usd')),
    sats: createCumulativeSumPattern(client, acc),
  };
}

/**
 * @typedef {Object} DollarsSatsPattern
 * @property {MetricPattern1<Dollars>} dollars
 * @property {MetricPattern1<SatsFract>} sats
 */

/**
 * Create a DollarsSatsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DollarsSatsPattern}
 */
function createDollarsSatsPattern(client, acc) {
  return {
    dollars: createMetricPattern1(client, acc),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
  };
}

/**
 * @typedef {Object} DollarsSatsPattern2
 * @property {MetricPattern4<Dollars>} dollars
 * @property {MetricPattern4<SatsFract>} sats
 */

/**
 * Create a DollarsSatsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DollarsSatsPattern2}
 */
function createDollarsSatsPattern2(client, acc) {
  return {
    dollars: createMetricPattern4(client, acc),
    sats: createMetricPattern4(client, _m(acc, 'sats')),
  };
}

/**
 * @typedef {Object} MaxMinPattern
 * @property {DollarsSatsPattern} max
 * @property {DollarsSatsPattern} min
 */

/**
 * Create a MaxMinPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {MaxMinPattern}
 */
function createMaxMinPattern(client, acc) {
  return {
    max: createDollarsSatsPattern(client, _m(acc, 'max_cost_basis')),
    min: createDollarsSatsPattern(client, _m(acc, 'min_cost_basis')),
  };
}

/**
 * @typedef {Object} SdSmaPattern
 * @property {MetricPattern4<StoredF32>} sd
 * @property {MetricPattern4<StoredF32>} sma
 */

/**
 * Create a SdSmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {SdSmaPattern}
 */
function createSdSmaPattern(client, acc) {
  return {
    sd: createMetricPattern4(client, _m(acc, 'sd')),
    sma: createMetricPattern4(client, _m(acc, 'sma')),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeSumPattern
 * @property {MetricPattern1<T>} cumulative
 * @property {MetricPattern1<T>} sum
 */

/**
 * Create a CumulativeSumPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CumulativeSumPattern<T>}
 */
function createCumulativeSumPattern(client, acc) {
  return {
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    sum: createMetricPattern1(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} CumulativeSumPattern2
 * @property {MetricPattern2<T>} cumulative
 * @property {MetricPattern1<T>} sum
 */

/**
 * Create a CumulativeSumPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CumulativeSumPattern2<T>}
 */
function createCumulativeSumPattern2(client, acc) {
  return {
    cumulative: createMetricPattern2(client, _m(acc, 'cumulative')),
    sum: createMetricPattern1(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} OhlcSplitPattern2
 * @property {MetricPattern1<T>} ohlc
 * @property {CloseHighLowOpenPattern2<T>} split
 */

/**
 * Create a OhlcSplitPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {OhlcSplitPattern2<T>}
 */
function createOhlcSplitPattern2(client, acc) {
  return {
    ohlc: createMetricPattern1(client, _m(acc, 'ohlc_sats')),
    split: createCloseHighLowOpenPattern2(client, _m(acc, 'sats')),
  };
}

/**
 * @typedef {Object} RatioPattern2
 * @property {MetricPattern4<StoredF32>} ratio
 */

/**
 * Create a RatioPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioPattern2}
 */
function createRatioPattern2(client, acc) {
  return {
    ratio: createMetricPattern4(client, acc),
  };
}

/**
 * @typedef {Object} UtxoPattern
 * @property {MetricPattern1<StoredU64>} utxoCount
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
 * @property {MetricsTree_Positions} positions
 * @property {MetricsTree_Cointime} cointime
 * @property {MetricsTree_Constants} constants
 * @property {MetricsTree_Indexes} indexes
 * @property {MetricsTree_Market} market
 * @property {MetricsTree_Pools} pools
 * @property {MetricsTree_Price} price
 * @property {MetricsTree_Distribution} distribution
 * @property {MetricsTree_Supply} supply
 */

/**
 * @typedef {Object} MetricsTree_Blocks
 * @property {MetricPattern11<BlockHash>} blockhash
 * @property {MetricsTree_Blocks_Difficulty} difficulty
 * @property {MetricsTree_Blocks_Time} time
 * @property {MetricPattern11<StoredU64>} totalSize
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Weight>} weight
 * @property {MetricsTree_Blocks_Count} count
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<Timestamp>} interval
 * @property {MetricsTree_Blocks_Mining} mining
 * @property {MetricsTree_Blocks_Rewards} rewards
 * @property {MetricsTree_Blocks_Halving} halving
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} vbytes
 * @property {MetricsTree_Blocks_Size} size
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} fullness
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Difficulty
 * @property {MetricPattern1<StoredF64>} raw
 * @property {MetricPattern1<StoredF32>} asHash
 * @property {MetricPattern1<StoredF32>} adjustment
 * @property {MetricPattern4<DifficultyEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextAdjustment
 * @property {MetricPattern1<StoredF32>} daysBeforeNextAdjustment
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time
 * @property {MetricPattern1<Timestamp>} timestamp
 * @property {MetricPattern11<Date>} date
 * @property {MetricPattern11<Timestamp>} timestampMonotonic
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Count
 * @property {MetricPattern4<StoredU64>} blockCountTarget
 * @property {CumulativeSumPattern<StoredU32>} blockCount
 * @property {MetricPattern11<Height>} _24hStart
 * @property {MetricPattern11<Height>} _1wStart
 * @property {MetricPattern11<Height>} _1mStart
 * @property {MetricPattern11<Height>} _1yStart
 * @property {MetricPattern1<StoredU32>} _24hBlockCount
 * @property {MetricPattern1<StoredU32>} _1wBlockCount
 * @property {MetricPattern1<StoredU32>} _1mBlockCount
 * @property {MetricPattern1<StoredU32>} _1yBlockCount
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Mining
 * @property {MetricPattern1<StoredF64>} hashRate
 * @property {MetricPattern4<StoredF64>} hashRate1wSma
 * @property {MetricPattern4<StoredF32>} hashRate1mSma
 * @property {MetricPattern4<StoredF32>} hashRate2mSma
 * @property {MetricPattern4<StoredF32>} hashRate1ySma
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
 * @typedef {Object} MetricsTree_Blocks_Rewards
 * @property {MetricsTree_Blocks_Rewards_24hCoinbaseSum} _24hCoinbaseSum
 * @property {BitcoinDollarsSatsPattern2} coinbase
 * @property {BitcoinDollarsSatsPattern2} subsidy
 * @property {BitcoinDollarsSatsPattern3} unclaimedRewards
 * @property {MetricPattern6<StoredF32>} feeDominance
 * @property {MetricPattern6<StoredF32>} subsidyDominance
 * @property {MetricPattern4<Dollars>} subsidyUsd1ySma
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Rewards_24hCoinbaseSum
 * @property {MetricPattern11<Sats>} sats
 * @property {MetricPattern11<Bitcoin>} bitcoin
 * @property {MetricPattern11<Dollars>} dollars
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Halving
 * @property {MetricPattern4<HalvingEpoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNextHalving
 * @property {MetricPattern1<StoredF32>} daysBeforeNextHalving
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Size
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {MetricPattern2<StoredU64>} average
 * @property {MetricPattern2<StoredU64>} min
 * @property {MetricPattern2<StoredU64>} max
 * @property {MetricPattern6<StoredU64>} pct10
 * @property {MetricPattern6<StoredU64>} pct25
 * @property {MetricPattern6<StoredU64>} median
 * @property {MetricPattern6<StoredU64>} pct75
 * @property {MetricPattern6<StoredU64>} pct90
 * @property {MetricPattern2<StoredU64>} sum
 */

/**
 * @typedef {Object} MetricsTree_Transactions
 * @property {MetricPattern11<TxIndex>} firstTxindex
 * @property {MetricPattern27<Height>} height
 * @property {MetricPattern27<Txid>} txid
 * @property {MetricPattern27<TxVersion>} txversion
 * @property {MetricPattern27<RawLockTime>} rawlocktime
 * @property {MetricPattern27<StoredU32>} baseSize
 * @property {MetricPattern27<StoredU32>} totalSize
 * @property {MetricPattern27<StoredBool>} isExplicitlyRbf
 * @property {MetricPattern27<TxInIndex>} firstTxinindex
 * @property {MetricPattern27<TxOutIndex>} firstTxoutindex
 * @property {MetricsTree_Transactions_Count} count
 * @property {MetricsTree_Transactions_Size} size
 * @property {MetricsTree_Transactions_Fees} fees
 * @property {MetricsTree_Transactions_Versions} versions
 * @property {MetricsTree_Transactions_Volume} volume
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Count
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} txCount
 * @property {MetricPattern27<StoredBool>} isCoinbase
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Size
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<VSize>} vsize
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<Weight>} weight
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees
 * @property {MetricPattern27<Sats>} inputValue
 * @property {MetricPattern27<Sats>} outputValue
 * @property {MetricsTree_Transactions_Fees_Fee} fee
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<FeeRate>} feeRate
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Fees_Fee
 * @property {MetricPattern27<Sats>} txindex
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Sats>} sats
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Bitcoin>} bitcoin
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Dollars>} dollars
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Versions
 * @property {CumulativeSumPattern<StoredU64>} v1
 * @property {CumulativeSumPattern<StoredU64>} v2
 * @property {CumulativeSumPattern<StoredU64>} v3
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Volume
 * @property {BitcoinDollarsSatsPattern4} sentSum
 * @property {BitcoinDollarsSatsPattern4} receivedSum
 * @property {BitcoinDollarsSatsPattern5} annualizedVolume
 * @property {MetricPattern4<StoredF32>} txPerSec
 * @property {MetricPattern4<StoredF32>} outputsPerSec
 * @property {MetricPattern4<StoredF32>} inputsPerSec
 */

/**
 * @typedef {Object} MetricsTree_Inputs
 * @property {MetricPattern11<TxInIndex>} firstTxinindex
 * @property {MetricPattern12<OutPoint>} outpoint
 * @property {MetricPattern12<TxIndex>} txindex
 * @property {MetricPattern12<OutputType>} outputtype
 * @property {MetricPattern12<TypeIndex>} typeindex
 * @property {MetricsTree_Inputs_Spent} spent
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} count
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Spent
 * @property {MetricPattern12<TxOutIndex>} txoutindex
 * @property {MetricPattern12<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Outputs
 * @property {MetricPattern11<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern15<Sats>} value
 * @property {MetricPattern15<OutputType>} outputtype
 * @property {MetricPattern15<TypeIndex>} typeindex
 * @property {MetricPattern15<TxIndex>} txindex
 * @property {MetricsTree_Outputs_Spent} spent
 * @property {MetricsTree_Outputs_Count} count
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Spent
 * @property {MetricPattern15<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Count
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} totalCount
 * @property {MetricPattern1<StoredU64>} utxoCount
 */

/**
 * @typedef {Object} MetricsTree_Addresses
 * @property {MetricPattern11<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {MetricPattern11<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {MetricPattern11<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {MetricPattern11<P2SHAddressIndex>} firstP2shaddressindex
 * @property {MetricPattern11<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {MetricPattern11<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {MetricPattern11<P2TRAddressIndex>} firstP2traddressindex
 * @property {MetricPattern11<P2AAddressIndex>} firstP2aaddressindex
 * @property {MetricPattern19<P2PK65Bytes>} p2pk65bytes
 * @property {MetricPattern18<P2PK33Bytes>} p2pk33bytes
 * @property {MetricPattern20<P2PKHBytes>} p2pkhbytes
 * @property {MetricPattern21<P2SHBytes>} p2shbytes
 * @property {MetricPattern23<P2WPKHBytes>} p2wpkhbytes
 * @property {MetricPattern24<P2WSHBytes>} p2wshbytes
 * @property {MetricPattern22<P2TRBytes>} p2trbytes
 * @property {MetricPattern16<P2ABytes>} p2abytes
 */

/**
 * @typedef {Object} MetricsTree_Scripts
 * @property {MetricPattern11<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {MetricPattern11<OpReturnIndex>} firstOpreturnindex
 * @property {MetricPattern11<P2MSOutputIndex>} firstP2msoutputindex
 * @property {MetricPattern11<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricPattern9<TxIndex>} emptyToTxindex
 * @property {MetricPattern14<TxIndex>} opreturnToTxindex
 * @property {MetricPattern17<TxIndex>} p2msToTxindex
 * @property {MetricPattern28<TxIndex>} unknownToTxindex
 * @property {MetricsTree_Scripts_Count} count
 * @property {MetricsTree_Scripts_Value} value
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Count
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2a
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2ms
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pk33
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pk65
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pkh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2sh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2tr
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2wpkh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2wsh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} opreturn
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} emptyoutput
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} unknownoutput
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} segwit
 * @property {BaseCumulativeSumPattern} taprootAdoption
 * @property {BaseCumulativeSumPattern} segwitAdoption
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value
 * @property {BitcoinDollarsSatsPattern2} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Positions
 * @property {MetricPattern11<BlkPosition>} blockPosition
 * @property {MetricPattern27<BlkPosition>} txPosition
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
 * @property {CumulativeSumPattern<StoredF64>} coinblocksCreated
 * @property {CumulativeSumPattern<StoredF64>} coinblocksStored
 * @property {MetricPattern1<StoredF64>} liveliness
 * @property {MetricPattern1<StoredF64>} vaultedness
 * @property {MetricPattern1<StoredF64>} activityToVaultednessRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Supply
 * @property {BitcoinDollarsSatsPattern4} vaultedSupply
 * @property {BitcoinDollarsSatsPattern4} activeSupply
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Value
 * @property {CumulativeSumPattern<StoredF64>} cointimeValueDestroyed
 * @property {CumulativeSumPattern<StoredF64>} cointimeValueCreated
 * @property {CumulativeSumPattern<StoredF64>} cointimeValueStored
 * @property {CumulativeSumPattern<StoredF64>} vocdd
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
 * @property {DollarsSatsPattern} vaultedPrice
 * @property {RatioPattern} vaultedPriceRatio
 * @property {DollarsSatsPattern} activePrice
 * @property {RatioPattern} activePriceRatio
 * @property {DollarsSatsPattern} trueMarketMean
 * @property {RatioPattern} trueMarketMeanRatio
 * @property {DollarsSatsPattern} cointimePrice
 * @property {RatioPattern} cointimePriceRatio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Adjusted
 * @property {MetricPattern4<StoredF32>} cointimeAdjInflationRate
 * @property {MetricPattern4<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {MetricPattern4<StoredF64>} cointimeAdjTxUsdVelocity
 */

/**
 * @typedef {Object} MetricsTree_Cointime_ReserveRisk
 * @property {MetricPattern6<StoredF64>} vocdd365dSma
 * @property {MetricPattern6<StoredF64>} hodlBank
 * @property {MetricPattern4<StoredF64>} reserveRisk
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
 * @property {MetricsTree_Indexes_Dateindex} dateindex
 * @property {MetricsTree_Indexes_Weekindex} weekindex
 * @property {MetricsTree_Indexes_Monthindex} monthindex
 * @property {MetricsTree_Indexes_Quarterindex} quarterindex
 * @property {MetricsTree_Indexes_Semesterindex} semesterindex
 * @property {MetricsTree_Indexes_Yearindex} yearindex
 * @property {MetricsTree_Indexes_Decadeindex} decadeindex
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
 * @typedef {Object} MetricsTree_Indexes_Address_P2a
 * @property {MetricPattern16<P2AAddressIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_P2ms
 * @property {MetricPattern17<P2MSOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Empty
 * @property {MetricPattern9<EmptyOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Unknown
 * @property {MetricPattern28<UnknownOutputIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Address_Opreturn
 * @property {MetricPattern14<OpReturnIndex>} identity
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Height
 * @property {MetricPattern11<Height>} identity
 * @property {MetricPattern11<DateIndex>} dateindex
 * @property {MetricPattern11<DifficultyEpoch>} difficultyepoch
 * @property {MetricPattern11<HalvingEpoch>} halvingepoch
 * @property {MetricPattern11<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Difficultyepoch
 * @property {MetricPattern8<DifficultyEpoch>} identity
 * @property {MetricPattern8<Height>} firstHeight
 * @property {MetricPattern8<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Halvingepoch
 * @property {MetricPattern10<HalvingEpoch>} identity
 * @property {MetricPattern10<Height>} firstHeight
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Dateindex
 * @property {MetricPattern6<DateIndex>} identity
 * @property {MetricPattern6<Date>} date
 * @property {MetricPattern6<Height>} firstHeight
 * @property {MetricPattern6<StoredU64>} heightCount
 * @property {MetricPattern6<WeekIndex>} weekindex
 * @property {MetricPattern6<MonthIndex>} monthindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Weekindex
 * @property {MetricPattern29<WeekIndex>} identity
 * @property {MetricPattern29<Date>} date
 * @property {MetricPattern29<DateIndex>} firstDateindex
 * @property {MetricPattern29<StoredU64>} dateindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Monthindex
 * @property {MetricPattern13<MonthIndex>} identity
 * @property {MetricPattern13<Date>} date
 * @property {MetricPattern13<DateIndex>} firstDateindex
 * @property {MetricPattern13<StoredU64>} dateindexCount
 * @property {MetricPattern13<QuarterIndex>} quarterindex
 * @property {MetricPattern13<SemesterIndex>} semesterindex
 * @property {MetricPattern13<YearIndex>} yearindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Quarterindex
 * @property {MetricPattern25<QuarterIndex>} identity
 * @property {MetricPattern25<Date>} date
 * @property {MetricPattern25<MonthIndex>} firstMonthindex
 * @property {MetricPattern25<StoredU64>} monthindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Semesterindex
 * @property {MetricPattern26<SemesterIndex>} identity
 * @property {MetricPattern26<Date>} date
 * @property {MetricPattern26<MonthIndex>} firstMonthindex
 * @property {MetricPattern26<StoredU64>} monthindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Yearindex
 * @property {MetricPattern30<YearIndex>} identity
 * @property {MetricPattern30<Date>} date
 * @property {MetricPattern30<MonthIndex>} firstMonthindex
 * @property {MetricPattern30<StoredU64>} monthindexCount
 * @property {MetricPattern30<DecadeIndex>} decadeindex
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Decadeindex
 * @property {MetricPattern7<DecadeIndex>} identity
 * @property {MetricPattern7<Date>} date
 * @property {MetricPattern7<YearIndex>} firstYearindex
 * @property {MetricPattern7<StoredU64>} yearindexCount
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
 * @property {DollarsSatsPattern} priceAth
 * @property {MetricPattern3<StoredF32>} priceDrawdown
 * @property {MetricPattern4<StoredU16>} daysSincePriceAth
 * @property {MetricPattern4<StoredF32>} yearsSincePriceAth
 * @property {MetricPattern4<StoredU16>} maxDaysBetweenPriceAths
 * @property {MetricPattern4<StoredF32>} maxYearsBetweenPriceAths
 */

/**
 * @typedef {Object} MetricsTree_Market_Lookback
 * @property {DollarsSatsPattern2} _1d
 * @property {DollarsSatsPattern2} _1w
 * @property {DollarsSatsPattern2} _1m
 * @property {DollarsSatsPattern2} _3m
 * @property {DollarsSatsPattern2} _6m
 * @property {DollarsSatsPattern2} _1y
 * @property {DollarsSatsPattern2} _2y
 * @property {DollarsSatsPattern2} _3y
 * @property {DollarsSatsPattern2} _4y
 * @property {DollarsSatsPattern2} _5y
 * @property {DollarsSatsPattern2} _6y
 * @property {DollarsSatsPattern2} _8y
 * @property {DollarsSatsPattern2} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns
 * @property {MetricsTree_Market_Returns_PriceReturns} priceReturns
 * @property {_10y2y3y4y5y6y8yPattern} cagr
 * @property {SdSmaPattern} _1dReturns1wSd
 * @property {SdSmaPattern} _1dReturns1mSd
 * @property {SdSmaPattern} _1dReturns1ySd
 * @property {MetricPattern6<StoredF32>} downsideReturns
 * @property {SdSmaPattern} downside1wSd
 * @property {SdSmaPattern} downside1mSd
 * @property {SdSmaPattern} downside1ySd
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_PriceReturns
 * @property {MetricPattern4<StoredF32>} _1d
 * @property {MetricPattern4<StoredF32>} _1w
 * @property {MetricPattern4<StoredF32>} _1m
 * @property {MetricPattern4<StoredF32>} _3m
 * @property {MetricPattern4<StoredF32>} _6m
 * @property {MetricPattern4<StoredF32>} _1y
 * @property {MetricPattern4<StoredF32>} _2y
 * @property {MetricPattern4<StoredF32>} _3y
 * @property {MetricPattern4<StoredF32>} _4y
 * @property {MetricPattern4<StoredF32>} _5y
 * @property {MetricPattern4<StoredF32>} _6y
 * @property {MetricPattern4<StoredF32>} _8y
 * @property {MetricPattern4<StoredF32>} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Volatility
 * @property {MetricPattern4<StoredF32>} price1wVolatility
 * @property {MetricPattern4<StoredF32>} price1mVolatility
 * @property {MetricPattern4<StoredF32>} price1yVolatility
 * @property {MetricPattern6<StoredF32>} sharpe1w
 * @property {MetricPattern6<StoredF32>} sharpe1m
 * @property {MetricPattern6<StoredF32>} sharpe1y
 * @property {MetricPattern6<StoredF32>} sortino1w
 * @property {MetricPattern6<StoredF32>} sortino1m
 * @property {MetricPattern6<StoredF32>} sortino1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Range
 * @property {DollarsSatsPattern2} price1wMin
 * @property {DollarsSatsPattern2} price1wMax
 * @property {DollarsSatsPattern2} price2wMin
 * @property {DollarsSatsPattern2} price2wMax
 * @property {DollarsSatsPattern2} price1mMin
 * @property {DollarsSatsPattern2} price1mMax
 * @property {DollarsSatsPattern2} price1yMin
 * @property {DollarsSatsPattern2} price1yMax
 * @property {MetricPattern6<StoredF32>} priceTrueRange
 * @property {MetricPattern6<StoredF32>} priceTrueRange2wSum
 * @property {MetricPattern4<StoredF32>} price2wChoppinessIndex
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
 * @property {DollarsSatsPattern2} price200dSmaX24
 * @property {DollarsSatsPattern2} price200dSmaX08
 * @property {DollarsSatsPattern2} price350dSmaX2
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca
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
 * @property {DollarsSatsPattern2} _1w
 * @property {DollarsSatsPattern2} _1m
 * @property {DollarsSatsPattern2} _3m
 * @property {DollarsSatsPattern2} _6m
 * @property {DollarsSatsPattern2} _1y
 * @property {DollarsSatsPattern2} _2y
 * @property {DollarsSatsPattern2} _3y
 * @property {DollarsSatsPattern2} _4y
 * @property {DollarsSatsPattern2} _5y
 * @property {DollarsSatsPattern2} _6y
 * @property {DollarsSatsPattern2} _8y
 * @property {DollarsSatsPattern2} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassStack
 * @property {BitcoinDollarsSatsPattern5} _2015
 * @property {BitcoinDollarsSatsPattern5} _2016
 * @property {BitcoinDollarsSatsPattern5} _2017
 * @property {BitcoinDollarsSatsPattern5} _2018
 * @property {BitcoinDollarsSatsPattern5} _2019
 * @property {BitcoinDollarsSatsPattern5} _2020
 * @property {BitcoinDollarsSatsPattern5} _2021
 * @property {BitcoinDollarsSatsPattern5} _2022
 * @property {BitcoinDollarsSatsPattern5} _2023
 * @property {BitcoinDollarsSatsPattern5} _2024
 * @property {BitcoinDollarsSatsPattern5} _2025
 * @property {BitcoinDollarsSatsPattern5} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassAveragePrice
 * @property {DollarsSatsPattern2} _2015
 * @property {DollarsSatsPattern2} _2016
 * @property {DollarsSatsPattern2} _2017
 * @property {DollarsSatsPattern2} _2018
 * @property {DollarsSatsPattern2} _2019
 * @property {DollarsSatsPattern2} _2020
 * @property {DollarsSatsPattern2} _2021
 * @property {DollarsSatsPattern2} _2022
 * @property {DollarsSatsPattern2} _2023
 * @property {DollarsSatsPattern2} _2024
 * @property {DollarsSatsPattern2} _2025
 * @property {DollarsSatsPattern2} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassDaysInProfit
 * @property {MetricPattern4<StoredU32>} _2015
 * @property {MetricPattern4<StoredU32>} _2016
 * @property {MetricPattern4<StoredU32>} _2017
 * @property {MetricPattern4<StoredU32>} _2018
 * @property {MetricPattern4<StoredU32>} _2019
 * @property {MetricPattern4<StoredU32>} _2020
 * @property {MetricPattern4<StoredU32>} _2021
 * @property {MetricPattern4<StoredU32>} _2022
 * @property {MetricPattern4<StoredU32>} _2023
 * @property {MetricPattern4<StoredU32>} _2024
 * @property {MetricPattern4<StoredU32>} _2025
 * @property {MetricPattern4<StoredU32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassDaysInLoss
 * @property {MetricPattern4<StoredU32>} _2015
 * @property {MetricPattern4<StoredU32>} _2016
 * @property {MetricPattern4<StoredU32>} _2017
 * @property {MetricPattern4<StoredU32>} _2018
 * @property {MetricPattern4<StoredU32>} _2019
 * @property {MetricPattern4<StoredU32>} _2020
 * @property {MetricPattern4<StoredU32>} _2021
 * @property {MetricPattern4<StoredU32>} _2022
 * @property {MetricPattern4<StoredU32>} _2023
 * @property {MetricPattern4<StoredU32>} _2024
 * @property {MetricPattern4<StoredU32>} _2025
 * @property {MetricPattern4<StoredU32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassMinReturn
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
 * @property {MetricPattern4<StoredF32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_ClassMaxReturn
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
 * @property {MetricPattern4<StoredF32>} _2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Indicators
 * @property {MetricPattern4<StoredF32>} puellMultiple
 * @property {MetricPattern4<StoredF32>} nvt
 * @property {MetricPattern6<StoredF32>} rsiGains
 * @property {MetricPattern6<StoredF32>} rsiLosses
 * @property {MetricPattern6<StoredF32>} rsiAverageGain14d
 * @property {MetricPattern6<StoredF32>} rsiAverageLoss14d
 * @property {MetricPattern6<StoredF32>} rsi14d
 * @property {MetricPattern6<StoredF32>} rsi14dMin
 * @property {MetricPattern6<StoredF32>} rsi14dMax
 * @property {MetricPattern6<StoredF32>} stochRsi
 * @property {MetricPattern6<StoredF32>} stochRsiK
 * @property {MetricPattern6<StoredF32>} stochRsiD
 * @property {MetricPattern6<StoredF32>} stochK
 * @property {MetricPattern6<StoredF32>} stochD
 * @property {MetricPattern6<StoredF32>} piCycle
 * @property {MetricPattern6<StoredF32>} macdLine
 * @property {MetricPattern6<StoredF32>} macdSignal
 * @property {MetricPattern6<StoredF32>} macdHistogram
 * @property {MetricPattern6<StoredF32>} gini
 */

/**
 * @typedef {Object} MetricsTree_Pools
 * @property {MetricPattern11<PoolSlug>} heightToPool
 * @property {MetricsTree_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} MetricsTree_Pools_Vecs
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} unknown
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} blockfills
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ultimuspool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} terrapool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} luxor
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} onethash
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btccom
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfarms
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} huobipool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} wayicn
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} canoepool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btctop
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoincom
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} pool175btc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} gbminers
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} axbt
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} asicminer
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitminter
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinrussia
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcserv
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} simplecoinus
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcguild
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} eligius
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ozcoin
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} eclipsemc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} maxbtc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} triplemining
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} coinlab
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} pool50btc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ghashio
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} stminingcorp
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitparking
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} mmpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} polmine
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} kncminer
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitalo
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} f2pool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} hhtt
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} megabigpower
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} mtred
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} nmcbit
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} yourbtcnet
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} givemecoins
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} braiinspool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} antpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} multicoinco
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bcpoolio
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} cointerra
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} kanopool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} solock
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ckpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} nicehash
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitclub
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinaffiliatenetwork
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bwpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} exxbw
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitsolo
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfury
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} twentyoneinc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} digitalbtc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} eightbaochi
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} mybtccoinpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} tbdice
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} hashpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} nexious
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bravomining
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} hotpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} okexpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bcmonster
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} onehash
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bixin
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} tatmaspool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} viabtc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} connectbtc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} batpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} waterhole
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} dcexploration
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} dcex
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} fiftyeightcoin
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinindia
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} shawnp0wers
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} phashio
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} rigpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} haozhuzhu
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} sevenpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningkings
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} hashbx
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} dpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} rawpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} haominer
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} helix
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitcoinukraine
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} poolin
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} secretsuperstar
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} tigerpoolnet
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} sigmapoolcom
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} okpooltop
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} hummerpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} tangpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bytepool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} spiderpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} novablock
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningcity
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} binancepool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} minerium
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} lubiancom
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} okkong
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} aaopool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} emcdpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} foundryusa
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} sbicrypto
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} arkpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} purebtccom
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} marapool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} kucoinpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} entrustcharitypool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} okminer
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} titan
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} pegapool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcnuggets
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} cloudhashing
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} digitalxmintsy
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} telco214
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcpoolparty
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} multipool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} transactioncoinmining
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcdig
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} trickysbtcpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btcmp
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} eobot
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} unomp
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} patels
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} gogreenlight
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ekanembtc
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} canoe
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} tiger
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} onem1x
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} zulupool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} secpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} ocean
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} whitepool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} wk057
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} futurebitapollosolo
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} carbonnegative
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} portlandhodl
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} phoenix
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} neopool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} maxipool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} bitfufupool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} luckypool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningdutch
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} publicpool
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} miningsquared
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} innopolistech
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} btclab
 * @property {_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern} parasite
 */

/**
 * @typedef {Object} MetricsTree_Price
 * @property {MetricsTree_Price_Cents} cents
 * @property {MetricsTree_Price_Usd} usd
 * @property {OhlcSplitPattern2<OHLCSats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Price_Cents
 * @property {MetricsTree_Price_Cents_Split} split
 * @property {MetricPattern5<OHLCCentsUnsigned>} ohlc
 */

/**
 * @typedef {Object} MetricsTree_Price_Cents_Split
 * @property {MetricPattern5<CentsUnsigned>} open
 * @property {MetricPattern5<CentsUnsigned>} high
 * @property {MetricPattern5<CentsUnsigned>} low
 * @property {MetricPattern5<CentsUnsigned>} close
 */

/**
 * @typedef {Object} MetricsTree_Price_Usd
 * @property {CloseHighLowOpenPattern2<Dollars>} split
 * @property {MetricPattern1<OHLCDollars>} ohlc
 */

/**
 * @typedef {Object} MetricsTree_Distribution
 * @property {MetricPattern11<SupplyState>} supplyState
 * @property {MetricsTree_Distribution_AnyAddressIndexes} anyAddressIndexes
 * @property {MetricsTree_Distribution_AddressesData} addressesData
 * @property {MetricsTree_Distribution_UtxoCohorts} utxoCohorts
 * @property {MetricsTree_Distribution_AddressCohorts} addressCohorts
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} addrCount
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} emptyAddrCount
 * @property {MetricsTree_Distribution_AddressActivity} addressActivity
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern} totalAddrCount
 * @property {MetricsTree_Distribution_NewAddrCount} newAddrCount
 * @property {MetricsTree_Distribution_GrowthRate} growthRate
 * @property {MetricPattern31<FundedAddressIndex>} fundedaddressindex
 * @property {MetricPattern32<EmptyAddressIndex>} emptyaddressindex
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
 * @typedef {Object} MetricsTree_Distribution_AddressesData
 * @property {MetricPattern31<FundedAddressData>} funded
 * @property {MetricPattern32<EmptyAddressData>} empty
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts
 * @property {MetricsTree_Distribution_UtxoCohorts_All} all
 * @property {MetricsTree_Distribution_UtxoCohorts_AgeRange} ageRange
 * @property {MetricsTree_Distribution_UtxoCohorts_Epoch} epoch
 * @property {MetricsTree_Distribution_UtxoCohorts_Year} year
 * @property {MetricsTree_Distribution_UtxoCohorts_MinAge} minAge
 * @property {MetricsTree_Distribution_UtxoCohorts_GeAmount} geAmount
 * @property {MetricsTree_Distribution_UtxoCohorts_AmountRange} amountRange
 * @property {MetricsTree_Distribution_UtxoCohorts_Term} term
 * @property {MetricsTree_Distribution_UtxoCohorts_Type} type
 * @property {MetricsTree_Distribution_UtxoCohorts_MaxAge} maxAge
 * @property {MetricsTree_Distribution_UtxoCohorts_LtAmount} ltAmount
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_All
 * @property {_30dHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
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
 * @property {MetricPattern1<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {MetricPattern1<StoredF32>} investedCapitalInProfitPct
 * @property {MetricPattern1<StoredF32>} investedCapitalInLossPct
 * @property {MetricPattern4<StoredF32>} unrealizedPeakRegretRelToMarketCap
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
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MinAge
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _1d
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _1w
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _1m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _2m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _3m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _4m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _5m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _6m
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _1y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _2y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _3y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _4y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _5y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _6y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _7y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _8y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _10y
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6} _12y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_GeAmount
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1sat
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10kBtc
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
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term
 * @property {MetricsTree_Distribution_UtxoCohorts_Term_Short} short
 * @property {MetricsTree_Distribution_UtxoCohorts_Term_Long} long
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term_Short
 * @property {_30dHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern} realized
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern4} relative
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Term_Long
 * @property {_30dHalvedTotalPattern} supply
 * @property {UtxoPattern} outputs
 * @property {CoinblocksCoindaysSatblocksSatdaysSentPattern} activity
 * @property {CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2} realized
 * @property {GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern} unrealized
 * @property {InvestedMaxMinPercentilesSpotPattern} costBasis
 * @property {InvestedNegNetNuplSupplyUnrealizedPattern4} relative
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_Type
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pk65
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pk33
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2pkh
 * @property {ActivityCostOutputsRealizedSupplyUnrealizedPattern} p2ms
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2sh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2wpkh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2wsh
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2tr
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3} p2a
 * @property {ActivityCostOutputsRealizedSupplyUnrealizedPattern} unknown
 * @property {ActivityCostOutputsRealizedSupplyUnrealizedPattern} empty
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_MaxAge
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
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5} _15y
 */

/**
 * @typedef {Object} MetricsTree_Distribution_UtxoCohorts_LtAmount
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100sats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100kSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10mSats
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100btc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _1kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _10kBtc
 * @property {ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4} _100kBtc
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
 * @typedef {Object} MetricsTree_Distribution_NewAddrCount
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} all
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pk65
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pk33
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2pkh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2sh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2wpkh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2wsh
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2tr
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>} p2a
 */

/**
 * @typedef {Object} MetricsTree_Distribution_GrowthRate
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} all
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2pk65
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2pk33
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2pkh
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2sh
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2wpkh
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2wsh
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2tr
 * @property {AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>} p2a
 */

/**
 * @typedef {Object} MetricsTree_Supply
 * @property {MetricsTree_Supply_Circulating} circulating
 * @property {MetricsTree_Supply_Burned} burned
 * @property {MetricPattern4<StoredF32>} inflation
 * @property {MetricsTree_Supply_Velocity} velocity
 * @property {MetricPattern1<Dollars>} marketCap
 */

/**
 * @typedef {Object} MetricsTree_Supply_Circulating
 * @property {MetricPattern3<Sats>} sats
 * @property {MetricPattern3<Bitcoin>} bitcoin
 * @property {MetricPattern3<Dollars>} dollars
 */

/**
 * @typedef {Object} MetricsTree_Supply_Burned
 * @property {BitcoinDollarsSatsPattern3} opreturn
 * @property {BitcoinDollarsSatsPattern3} unspendable
 */

/**
 * @typedef {Object} MetricsTree_Supply_Velocity
 * @property {MetricPattern4<StoredF64>} btc
 * @property {MetricPattern4<StoredF64>} usd
 */

/**
 * Main BRK client with metrics tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
  VERSION = "v0.1.2";

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
   * Check if an index type is date-based.
   * @param {Index} index
   * @returns {boolean}
   */
  isDateIndex(index) {
    return isDateIndex(index);
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
        blockhash: createMetricPattern11(this, 'blockhash'),
        difficulty: {
          raw: createMetricPattern1(this, 'difficulty'),
          asHash: createMetricPattern1(this, 'difficulty_as_hash'),
          adjustment: createMetricPattern1(this, 'difficulty_adjustment'),
          epoch: createMetricPattern4(this, 'difficultyepoch'),
          blocksBeforeNextAdjustment: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
          daysBeforeNextAdjustment: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
        },
        time: {
          timestamp: createMetricPattern1(this, 'timestamp'),
          date: createMetricPattern11(this, 'date'),
          timestampMonotonic: createMetricPattern11(this, 'timestamp_monotonic'),
        },
        totalSize: createMetricPattern11(this, 'total_size'),
        weight: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'block_weight'),
        count: {
          blockCountTarget: createMetricPattern4(this, 'block_count_target'),
          blockCount: createCumulativeSumPattern(this, 'block_count'),
          _24hStart: createMetricPattern11(this, '24h_start'),
          _1wStart: createMetricPattern11(this, '1w_start'),
          _1mStart: createMetricPattern11(this, '1m_start'),
          _1yStart: createMetricPattern11(this, '1y_start'),
          _24hBlockCount: createMetricPattern1(this, '24h_block_count'),
          _1wBlockCount: createMetricPattern1(this, '1w_block_count'),
          _1mBlockCount: createMetricPattern1(this, '1m_block_count'),
          _1yBlockCount: createMetricPattern1(this, '1y_block_count'),
        },
        interval: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'block_interval'),
        mining: {
          hashRate: createMetricPattern1(this, 'hash_rate'),
          hashRate1wSma: createMetricPattern4(this, 'hash_rate_1w_sma'),
          hashRate1mSma: createMetricPattern4(this, 'hash_rate_1m_sma'),
          hashRate2mSma: createMetricPattern4(this, 'hash_rate_2m_sma'),
          hashRate1ySma: createMetricPattern4(this, 'hash_rate_1y_sma'),
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
        rewards: {
          _24hCoinbaseSum: {
            sats: createMetricPattern11(this, '24h_coinbase_sum'),
            bitcoin: createMetricPattern11(this, '24h_coinbase_sum_btc'),
            dollars: createMetricPattern11(this, '24h_coinbase_sum_usd'),
          },
          coinbase: createBitcoinDollarsSatsPattern2(this, 'coinbase'),
          subsidy: createBitcoinDollarsSatsPattern2(this, 'subsidy'),
          unclaimedRewards: createBitcoinDollarsSatsPattern3(this, 'unclaimed_rewards'),
          feeDominance: createMetricPattern6(this, 'fee_dominance'),
          subsidyDominance: createMetricPattern6(this, 'subsidy_dominance'),
          subsidyUsd1ySma: createMetricPattern4(this, 'subsidy_usd_1y_sma'),
        },
        halving: {
          epoch: createMetricPattern4(this, 'halvingepoch'),
          blocksBeforeNextHalving: createMetricPattern1(this, 'blocks_before_next_halving'),
          daysBeforeNextHalving: createMetricPattern1(this, 'days_before_next_halving'),
        },
        vbytes: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'block_vbytes'),
        size: {
          cumulative: createMetricPattern1(this, 'block_size_cumulative'),
          average: createMetricPattern2(this, 'block_size_average'),
          min: createMetricPattern2(this, 'block_size_min'),
          max: createMetricPattern2(this, 'block_size_max'),
          pct10: createMetricPattern6(this, 'block_size_pct10'),
          pct25: createMetricPattern6(this, 'block_size_pct25'),
          median: createMetricPattern6(this, 'block_size_median'),
          pct75: createMetricPattern6(this, 'block_size_pct75'),
          pct90: createMetricPattern6(this, 'block_size_pct90'),
          sum: createMetricPattern2(this, 'block_size_sum'),
        },
        fullness: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'block_fullness'),
      },
      transactions: {
        firstTxindex: createMetricPattern11(this, 'first_txindex'),
        height: createMetricPattern27(this, 'height'),
        txid: createMetricPattern27(this, 'txid'),
        txversion: createMetricPattern27(this, 'txversion'),
        rawlocktime: createMetricPattern27(this, 'rawlocktime'),
        baseSize: createMetricPattern27(this, 'base_size'),
        totalSize: createMetricPattern27(this, 'total_size'),
        isExplicitlyRbf: createMetricPattern27(this, 'is_explicitly_rbf'),
        firstTxinindex: createMetricPattern27(this, 'first_txinindex'),
        firstTxoutindex: createMetricPattern27(this, 'first_txoutindex'),
        count: {
          txCount: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'tx_count'),
          isCoinbase: createMetricPattern27(this, 'is_coinbase'),
        },
        size: {
          vsize: createAverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(this, 'tx_vsize'),
          weight: createAverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(this, 'tx_weight'),
        },
        fees: {
          inputValue: createMetricPattern27(this, 'input_value'),
          outputValue: createMetricPattern27(this, 'output_value'),
          fee: {
            txindex: createMetricPattern27(this, 'fee'),
            sats: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'fee'),
            bitcoin: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'fee_btc'),
            dollars: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'fee_usd'),
          },
          feeRate: createAverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(this, 'fee_rate'),
        },
        versions: {
          v1: createCumulativeSumPattern(this, 'tx_v1'),
          v2: createCumulativeSumPattern(this, 'tx_v2'),
          v3: createCumulativeSumPattern(this, 'tx_v3'),
        },
        volume: {
          sentSum: createBitcoinDollarsSatsPattern4(this, 'sent_sum'),
          receivedSum: createBitcoinDollarsSatsPattern4(this, 'received_sum'),
          annualizedVolume: createBitcoinDollarsSatsPattern5(this, 'annualized_volume'),
          txPerSec: createMetricPattern4(this, 'tx_per_sec'),
          outputsPerSec: createMetricPattern4(this, 'outputs_per_sec'),
          inputsPerSec: createMetricPattern4(this, 'inputs_per_sec'),
        },
      },
      inputs: {
        firstTxinindex: createMetricPattern11(this, 'first_txinindex'),
        outpoint: createMetricPattern12(this, 'outpoint'),
        txindex: createMetricPattern12(this, 'txindex'),
        outputtype: createMetricPattern12(this, 'outputtype'),
        typeindex: createMetricPattern12(this, 'typeindex'),
        spent: {
          txoutindex: createMetricPattern12(this, 'txoutindex'),
          value: createMetricPattern12(this, 'value'),
        },
        count: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'input_count'),
      },
      outputs: {
        firstTxoutindex: createMetricPattern11(this, 'first_txoutindex'),
        value: createMetricPattern15(this, 'value'),
        outputtype: createMetricPattern15(this, 'outputtype'),
        typeindex: createMetricPattern15(this, 'typeindex'),
        txindex: createMetricPattern15(this, 'txindex'),
        spent: {
          txinindex: createMetricPattern15(this, 'txinindex'),
        },
        count: {
          totalCount: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'output_count'),
          utxoCount: createMetricPattern1(this, 'exact_utxo_count'),
        },
      },
      addresses: {
        firstP2pk65addressindex: createMetricPattern11(this, 'first_p2pk65addressindex'),
        firstP2pk33addressindex: createMetricPattern11(this, 'first_p2pk33addressindex'),
        firstP2pkhaddressindex: createMetricPattern11(this, 'first_p2pkhaddressindex'),
        firstP2shaddressindex: createMetricPattern11(this, 'first_p2shaddressindex'),
        firstP2wpkhaddressindex: createMetricPattern11(this, 'first_p2wpkhaddressindex'),
        firstP2wshaddressindex: createMetricPattern11(this, 'first_p2wshaddressindex'),
        firstP2traddressindex: createMetricPattern11(this, 'first_p2traddressindex'),
        firstP2aaddressindex: createMetricPattern11(this, 'first_p2aaddressindex'),
        p2pk65bytes: createMetricPattern19(this, 'p2pk65bytes'),
        p2pk33bytes: createMetricPattern18(this, 'p2pk33bytes'),
        p2pkhbytes: createMetricPattern20(this, 'p2pkhbytes'),
        p2shbytes: createMetricPattern21(this, 'p2shbytes'),
        p2wpkhbytes: createMetricPattern23(this, 'p2wpkhbytes'),
        p2wshbytes: createMetricPattern24(this, 'p2wshbytes'),
        p2trbytes: createMetricPattern22(this, 'p2trbytes'),
        p2abytes: createMetricPattern16(this, 'p2abytes'),
      },
      scripts: {
        firstEmptyoutputindex: createMetricPattern11(this, 'first_emptyoutputindex'),
        firstOpreturnindex: createMetricPattern11(this, 'first_opreturnindex'),
        firstP2msoutputindex: createMetricPattern11(this, 'first_p2msoutputindex'),
        firstUnknownoutputindex: createMetricPattern11(this, 'first_unknownoutputindex'),
        emptyToTxindex: createMetricPattern9(this, 'txindex'),
        opreturnToTxindex: createMetricPattern14(this, 'txindex'),
        p2msToTxindex: createMetricPattern17(this, 'txindex'),
        unknownToTxindex: createMetricPattern28(this, 'txindex'),
        count: {
          p2a: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2a_count'),
          p2ms: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2ms_count'),
          p2pk33: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pk33_count'),
          p2pk65: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pk65_count'),
          p2pkh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pkh_count'),
          p2sh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2sh_count'),
          p2tr: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2tr_count'),
          p2wpkh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2wpkh_count'),
          p2wsh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2wsh_count'),
          opreturn: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'opreturn_count'),
          emptyoutput: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'emptyoutput_count'),
          unknownoutput: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'unknownoutput_count'),
          segwit: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'segwit_count'),
          taprootAdoption: createBaseCumulativeSumPattern(this, 'taproot_adoption'),
          segwitAdoption: createBaseCumulativeSumPattern(this, 'segwit_adoption'),
        },
        value: {
          opreturn: createBitcoinDollarsSatsPattern2(this, 'opreturn_value'),
        },
      },
      positions: {
        blockPosition: createMetricPattern11(this, 'position'),
        txPosition: createMetricPattern27(this, 'position'),
      },
      cointime: {
        activity: {
          coinblocksCreated: createCumulativeSumPattern(this, 'coinblocks_created'),
          coinblocksStored: createCumulativeSumPattern(this, 'coinblocks_stored'),
          liveliness: createMetricPattern1(this, 'liveliness'),
          vaultedness: createMetricPattern1(this, 'vaultedness'),
          activityToVaultednessRatio: createMetricPattern1(this, 'activity_to_vaultedness_ratio'),
        },
        supply: {
          vaultedSupply: createBitcoinDollarsSatsPattern4(this, 'vaulted_supply'),
          activeSupply: createBitcoinDollarsSatsPattern4(this, 'active_supply'),
        },
        value: {
          cointimeValueDestroyed: createCumulativeSumPattern(this, 'cointime_value_destroyed'),
          cointimeValueCreated: createCumulativeSumPattern(this, 'cointime_value_created'),
          cointimeValueStored: createCumulativeSumPattern(this, 'cointime_value_stored'),
          vocdd: createCumulativeSumPattern(this, 'vocdd'),
        },
        cap: {
          thermoCap: createMetricPattern1(this, 'thermo_cap'),
          investorCap: createMetricPattern1(this, 'investor_cap'),
          vaultedCap: createMetricPattern1(this, 'vaulted_cap'),
          activeCap: createMetricPattern1(this, 'active_cap'),
          cointimeCap: createMetricPattern1(this, 'cointime_cap'),
        },
        pricing: {
          vaultedPrice: createDollarsSatsPattern(this, 'vaulted_price'),
          vaultedPriceRatio: createRatioPattern(this, 'vaulted_price_ratio'),
          activePrice: createDollarsSatsPattern(this, 'active_price'),
          activePriceRatio: createRatioPattern(this, 'active_price_ratio'),
          trueMarketMean: createDollarsSatsPattern(this, 'true_market_mean'),
          trueMarketMeanRatio: createRatioPattern(this, 'true_market_mean_ratio'),
          cointimePrice: createDollarsSatsPattern(this, 'cointime_price'),
          cointimePriceRatio: createRatioPattern(this, 'cointime_price_ratio'),
        },
        adjusted: {
          cointimeAdjInflationRate: createMetricPattern4(this, 'cointime_adj_inflation_rate'),
          cointimeAdjTxBtcVelocity: createMetricPattern4(this, 'cointime_adj_tx_btc_velocity'),
          cointimeAdjTxUsdVelocity: createMetricPattern4(this, 'cointime_adj_tx_usd_velocity'),
        },
        reserveRisk: {
          vocdd365dSma: createMetricPattern6(this, 'vocdd_365d_sma'),
          hodlBank: createMetricPattern6(this, 'hodl_bank'),
          reserveRisk: createMetricPattern4(this, 'reserve_risk'),
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
          p2a: {
            identity: createMetricPattern16(this, 'p2aaddressindex'),
          },
          p2ms: {
            identity: createMetricPattern17(this, 'p2msoutputindex'),
          },
          empty: {
            identity: createMetricPattern9(this, 'emptyoutputindex'),
          },
          unknown: {
            identity: createMetricPattern28(this, 'unknownoutputindex'),
          },
          opreturn: {
            identity: createMetricPattern14(this, 'opreturnindex'),
          },
        },
        height: {
          identity: createMetricPattern11(this, 'height'),
          dateindex: createMetricPattern11(this, 'dateindex'),
          difficultyepoch: createMetricPattern11(this, 'difficultyepoch'),
          halvingepoch: createMetricPattern11(this, 'halvingepoch'),
          txindexCount: createMetricPattern11(this, 'txindex_count'),
        },
        difficultyepoch: {
          identity: createMetricPattern8(this, 'difficultyepoch'),
          firstHeight: createMetricPattern8(this, 'first_height'),
          heightCount: createMetricPattern8(this, 'height_count'),
        },
        halvingepoch: {
          identity: createMetricPattern10(this, 'halvingepoch'),
          firstHeight: createMetricPattern10(this, 'first_height'),
        },
        dateindex: {
          identity: createMetricPattern6(this, 'dateindex'),
          date: createMetricPattern6(this, 'date'),
          firstHeight: createMetricPattern6(this, 'first_height'),
          heightCount: createMetricPattern6(this, 'height_count'),
          weekindex: createMetricPattern6(this, 'weekindex'),
          monthindex: createMetricPattern6(this, 'monthindex'),
        },
        weekindex: {
          identity: createMetricPattern29(this, 'weekindex'),
          date: createMetricPattern29(this, 'date'),
          firstDateindex: createMetricPattern29(this, 'first_dateindex'),
          dateindexCount: createMetricPattern29(this, 'dateindex_count'),
        },
        monthindex: {
          identity: createMetricPattern13(this, 'monthindex'),
          date: createMetricPattern13(this, 'date'),
          firstDateindex: createMetricPattern13(this, 'first_dateindex'),
          dateindexCount: createMetricPattern13(this, 'dateindex_count'),
          quarterindex: createMetricPattern13(this, 'quarterindex'),
          semesterindex: createMetricPattern13(this, 'semesterindex'),
          yearindex: createMetricPattern13(this, 'yearindex'),
        },
        quarterindex: {
          identity: createMetricPattern25(this, 'quarterindex'),
          date: createMetricPattern25(this, 'date'),
          firstMonthindex: createMetricPattern25(this, 'first_monthindex'),
          monthindexCount: createMetricPattern25(this, 'monthindex_count'),
        },
        semesterindex: {
          identity: createMetricPattern26(this, 'semesterindex'),
          date: createMetricPattern26(this, 'date'),
          firstMonthindex: createMetricPattern26(this, 'first_monthindex'),
          monthindexCount: createMetricPattern26(this, 'monthindex_count'),
        },
        yearindex: {
          identity: createMetricPattern30(this, 'yearindex'),
          date: createMetricPattern30(this, 'date'),
          firstMonthindex: createMetricPattern30(this, 'first_monthindex'),
          monthindexCount: createMetricPattern30(this, 'monthindex_count'),
          decadeindex: createMetricPattern30(this, 'decadeindex'),
        },
        decadeindex: {
          identity: createMetricPattern7(this, 'decadeindex'),
          date: createMetricPattern7(this, 'date'),
          firstYearindex: createMetricPattern7(this, 'first_yearindex'),
          yearindexCount: createMetricPattern7(this, 'yearindex_count'),
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
      },
      market: {
        ath: {
          priceAth: createDollarsSatsPattern(this, 'price_ath'),
          priceDrawdown: createMetricPattern3(this, 'price_drawdown'),
          daysSincePriceAth: createMetricPattern4(this, 'days_since_price_ath'),
          yearsSincePriceAth: createMetricPattern4(this, 'years_since_price_ath'),
          maxDaysBetweenPriceAths: createMetricPattern4(this, 'max_days_between_price_aths'),
          maxYearsBetweenPriceAths: createMetricPattern4(this, 'max_years_between_price_aths'),
        },
        lookback: {
          _1d: createDollarsSatsPattern2(this, 'price_1d_ago'),
          _1w: createDollarsSatsPattern2(this, 'price_1w_ago'),
          _1m: createDollarsSatsPattern2(this, 'price_1m_ago'),
          _3m: createDollarsSatsPattern2(this, 'price_3m_ago'),
          _6m: createDollarsSatsPattern2(this, 'price_6m_ago'),
          _1y: createDollarsSatsPattern2(this, 'price_1y_ago'),
          _2y: createDollarsSatsPattern2(this, 'price_2y_ago'),
          _3y: createDollarsSatsPattern2(this, 'price_3y_ago'),
          _4y: createDollarsSatsPattern2(this, 'price_4y_ago'),
          _5y: createDollarsSatsPattern2(this, 'price_5y_ago'),
          _6y: createDollarsSatsPattern2(this, 'price_6y_ago'),
          _8y: createDollarsSatsPattern2(this, 'price_8y_ago'),
          _10y: createDollarsSatsPattern2(this, 'price_10y_ago'),
        },
        returns: {
          priceReturns: {
            _1d: createMetricPattern4(this, '1d_price_returns'),
            _1w: createMetricPattern4(this, '1w_price_returns'),
            _1m: createMetricPattern4(this, '1m_price_returns'),
            _3m: createMetricPattern4(this, '3m_price_returns'),
            _6m: createMetricPattern4(this, '6m_price_returns'),
            _1y: createMetricPattern4(this, '1y_price_returns'),
            _2y: createMetricPattern4(this, '2y_price_returns'),
            _3y: createMetricPattern4(this, '3y_price_returns'),
            _4y: createMetricPattern4(this, '4y_price_returns'),
            _5y: createMetricPattern4(this, '5y_price_returns'),
            _6y: createMetricPattern4(this, '6y_price_returns'),
            _8y: createMetricPattern4(this, '8y_price_returns'),
            _10y: createMetricPattern4(this, '10y_price_returns'),
          },
          cagr: create_10y2y3y4y5y6y8yPattern(this, 'cagr'),
          _1dReturns1wSd: createSdSmaPattern(this, '1d_returns_1w_sd'),
          _1dReturns1mSd: createSdSmaPattern(this, '1d_returns_1m_sd'),
          _1dReturns1ySd: createSdSmaPattern(this, '1d_returns_1y_sd'),
          downsideReturns: createMetricPattern6(this, 'downside_returns'),
          downside1wSd: createSdSmaPattern(this, 'downside_1w_sd'),
          downside1mSd: createSdSmaPattern(this, 'downside_1m_sd'),
          downside1ySd: createSdSmaPattern(this, 'downside_1y_sd'),
        },
        volatility: {
          price1wVolatility: createMetricPattern4(this, 'price_1w_volatility'),
          price1mVolatility: createMetricPattern4(this, 'price_1m_volatility'),
          price1yVolatility: createMetricPattern4(this, 'price_1y_volatility'),
          sharpe1w: createMetricPattern6(this, 'sharpe_1w'),
          sharpe1m: createMetricPattern6(this, 'sharpe_1m'),
          sharpe1y: createMetricPattern6(this, 'sharpe_1y'),
          sortino1w: createMetricPattern6(this, 'sortino_1w'),
          sortino1m: createMetricPattern6(this, 'sortino_1m'),
          sortino1y: createMetricPattern6(this, 'sortino_1y'),
        },
        range: {
          price1wMin: createDollarsSatsPattern2(this, 'price_1w_min'),
          price1wMax: createDollarsSatsPattern2(this, 'price_1w_max'),
          price2wMin: createDollarsSatsPattern2(this, 'price_2w_min'),
          price2wMax: createDollarsSatsPattern2(this, 'price_2w_max'),
          price1mMin: createDollarsSatsPattern2(this, 'price_1m_min'),
          price1mMax: createDollarsSatsPattern2(this, 'price_1m_max'),
          price1yMin: createDollarsSatsPattern2(this, 'price_1y_min'),
          price1yMax: createDollarsSatsPattern2(this, 'price_1y_max'),
          priceTrueRange: createMetricPattern6(this, 'price_true_range'),
          priceTrueRange2wSum: createMetricPattern6(this, 'price_true_range_2w_sum'),
          price2wChoppinessIndex: createMetricPattern4(this, 'price_2w_choppiness_index'),
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
          price200dSmaX24: createDollarsSatsPattern2(this, 'price_200d_sma_x2_4'),
          price200dSmaX08: createDollarsSatsPattern2(this, 'price_200d_sma_x0_8'),
          price350dSmaX2: createDollarsSatsPattern2(this, 'price_350d_sma_x2'),
        },
        dca: {
          periodStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'dca_stack'),
          periodAveragePrice: {
            _1w: createDollarsSatsPattern2(this, '1w_dca_average_price'),
            _1m: createDollarsSatsPattern2(this, '1m_dca_average_price'),
            _3m: createDollarsSatsPattern2(this, '3m_dca_average_price'),
            _6m: createDollarsSatsPattern2(this, '6m_dca_average_price'),
            _1y: createDollarsSatsPattern2(this, '1y_dca_average_price'),
            _2y: createDollarsSatsPattern2(this, '2y_dca_average_price'),
            _3y: createDollarsSatsPattern2(this, '3y_dca_average_price'),
            _4y: createDollarsSatsPattern2(this, '4y_dca_average_price'),
            _5y: createDollarsSatsPattern2(this, '5y_dca_average_price'),
            _6y: createDollarsSatsPattern2(this, '6y_dca_average_price'),
            _8y: createDollarsSatsPattern2(this, '8y_dca_average_price'),
            _10y: createDollarsSatsPattern2(this, '10y_dca_average_price'),
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
            _2015: createBitcoinDollarsSatsPattern5(this, 'dca_class_2015_stack'),
            _2016: createBitcoinDollarsSatsPattern5(this, 'dca_class_2016_stack'),
            _2017: createBitcoinDollarsSatsPattern5(this, 'dca_class_2017_stack'),
            _2018: createBitcoinDollarsSatsPattern5(this, 'dca_class_2018_stack'),
            _2019: createBitcoinDollarsSatsPattern5(this, 'dca_class_2019_stack'),
            _2020: createBitcoinDollarsSatsPattern5(this, 'dca_class_2020_stack'),
            _2021: createBitcoinDollarsSatsPattern5(this, 'dca_class_2021_stack'),
            _2022: createBitcoinDollarsSatsPattern5(this, 'dca_class_2022_stack'),
            _2023: createBitcoinDollarsSatsPattern5(this, 'dca_class_2023_stack'),
            _2024: createBitcoinDollarsSatsPattern5(this, 'dca_class_2024_stack'),
            _2025: createBitcoinDollarsSatsPattern5(this, 'dca_class_2025_stack'),
            _2026: createBitcoinDollarsSatsPattern5(this, 'dca_class_2026_stack'),
          },
          classAveragePrice: {
            _2015: createDollarsSatsPattern2(this, 'dca_class_2015_average_price'),
            _2016: createDollarsSatsPattern2(this, 'dca_class_2016_average_price'),
            _2017: createDollarsSatsPattern2(this, 'dca_class_2017_average_price'),
            _2018: createDollarsSatsPattern2(this, 'dca_class_2018_average_price'),
            _2019: createDollarsSatsPattern2(this, 'dca_class_2019_average_price'),
            _2020: createDollarsSatsPattern2(this, 'dca_class_2020_average_price'),
            _2021: createDollarsSatsPattern2(this, 'dca_class_2021_average_price'),
            _2022: createDollarsSatsPattern2(this, 'dca_class_2022_average_price'),
            _2023: createDollarsSatsPattern2(this, 'dca_class_2023_average_price'),
            _2024: createDollarsSatsPattern2(this, 'dca_class_2024_average_price'),
            _2025: createDollarsSatsPattern2(this, 'dca_class_2025_average_price'),
            _2026: createDollarsSatsPattern2(this, 'dca_class_2026_average_price'),
          },
          classReturns: create_201520162017201820192020202120222023202420252026Pattern2(this, 'dca_class'),
          classDaysInProfit: {
            _2015: createMetricPattern4(this, 'dca_class_2015_days_in_profit'),
            _2016: createMetricPattern4(this, 'dca_class_2016_days_in_profit'),
            _2017: createMetricPattern4(this, 'dca_class_2017_days_in_profit'),
            _2018: createMetricPattern4(this, 'dca_class_2018_days_in_profit'),
            _2019: createMetricPattern4(this, 'dca_class_2019_days_in_profit'),
            _2020: createMetricPattern4(this, 'dca_class_2020_days_in_profit'),
            _2021: createMetricPattern4(this, 'dca_class_2021_days_in_profit'),
            _2022: createMetricPattern4(this, 'dca_class_2022_days_in_profit'),
            _2023: createMetricPattern4(this, 'dca_class_2023_days_in_profit'),
            _2024: createMetricPattern4(this, 'dca_class_2024_days_in_profit'),
            _2025: createMetricPattern4(this, 'dca_class_2025_days_in_profit'),
            _2026: createMetricPattern4(this, 'dca_class_2026_days_in_profit'),
          },
          classDaysInLoss: {
            _2015: createMetricPattern4(this, 'dca_class_2015_days_in_loss'),
            _2016: createMetricPattern4(this, 'dca_class_2016_days_in_loss'),
            _2017: createMetricPattern4(this, 'dca_class_2017_days_in_loss'),
            _2018: createMetricPattern4(this, 'dca_class_2018_days_in_loss'),
            _2019: createMetricPattern4(this, 'dca_class_2019_days_in_loss'),
            _2020: createMetricPattern4(this, 'dca_class_2020_days_in_loss'),
            _2021: createMetricPattern4(this, 'dca_class_2021_days_in_loss'),
            _2022: createMetricPattern4(this, 'dca_class_2022_days_in_loss'),
            _2023: createMetricPattern4(this, 'dca_class_2023_days_in_loss'),
            _2024: createMetricPattern4(this, 'dca_class_2024_days_in_loss'),
            _2025: createMetricPattern4(this, 'dca_class_2025_days_in_loss'),
            _2026: createMetricPattern4(this, 'dca_class_2026_days_in_loss'),
          },
          classMinReturn: {
            _2015: createMetricPattern4(this, 'dca_class_2015_min_return'),
            _2016: createMetricPattern4(this, 'dca_class_2016_min_return'),
            _2017: createMetricPattern4(this, 'dca_class_2017_min_return'),
            _2018: createMetricPattern4(this, 'dca_class_2018_min_return'),
            _2019: createMetricPattern4(this, 'dca_class_2019_min_return'),
            _2020: createMetricPattern4(this, 'dca_class_2020_min_return'),
            _2021: createMetricPattern4(this, 'dca_class_2021_min_return'),
            _2022: createMetricPattern4(this, 'dca_class_2022_min_return'),
            _2023: createMetricPattern4(this, 'dca_class_2023_min_return'),
            _2024: createMetricPattern4(this, 'dca_class_2024_min_return'),
            _2025: createMetricPattern4(this, 'dca_class_2025_min_return'),
            _2026: createMetricPattern4(this, 'dca_class_2026_min_return'),
          },
          classMaxReturn: {
            _2015: createMetricPattern4(this, 'dca_class_2015_max_return'),
            _2016: createMetricPattern4(this, 'dca_class_2016_max_return'),
            _2017: createMetricPattern4(this, 'dca_class_2017_max_return'),
            _2018: createMetricPattern4(this, 'dca_class_2018_max_return'),
            _2019: createMetricPattern4(this, 'dca_class_2019_max_return'),
            _2020: createMetricPattern4(this, 'dca_class_2020_max_return'),
            _2021: createMetricPattern4(this, 'dca_class_2021_max_return'),
            _2022: createMetricPattern4(this, 'dca_class_2022_max_return'),
            _2023: createMetricPattern4(this, 'dca_class_2023_max_return'),
            _2024: createMetricPattern4(this, 'dca_class_2024_max_return'),
            _2025: createMetricPattern4(this, 'dca_class_2025_max_return'),
            _2026: createMetricPattern4(this, 'dca_class_2026_max_return'),
          },
        },
        indicators: {
          puellMultiple: createMetricPattern4(this, 'puell_multiple'),
          nvt: createMetricPattern4(this, 'nvt'),
          rsiGains: createMetricPattern6(this, 'rsi_gains'),
          rsiLosses: createMetricPattern6(this, 'rsi_losses'),
          rsiAverageGain14d: createMetricPattern6(this, 'rsi_average_gain_14d'),
          rsiAverageLoss14d: createMetricPattern6(this, 'rsi_average_loss_14d'),
          rsi14d: createMetricPattern6(this, 'rsi_14d'),
          rsi14dMin: createMetricPattern6(this, 'rsi_14d_min'),
          rsi14dMax: createMetricPattern6(this, 'rsi_14d_max'),
          stochRsi: createMetricPattern6(this, 'stoch_rsi'),
          stochRsiK: createMetricPattern6(this, 'stoch_rsi_k'),
          stochRsiD: createMetricPattern6(this, 'stoch_rsi_d'),
          stochK: createMetricPattern6(this, 'stoch_k'),
          stochD: createMetricPattern6(this, 'stoch_d'),
          piCycle: createMetricPattern6(this, 'pi_cycle'),
          macdLine: createMetricPattern6(this, 'macd_line'),
          macdSignal: createMetricPattern6(this, 'macd_signal'),
          macdHistogram: createMetricPattern6(this, 'macd_histogram'),
          gini: createMetricPattern6(this, 'gini'),
        },
      },
      pools: {
        heightToPool: createMetricPattern11(this, 'pool'),
        vecs: {
          unknown: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'unknown'),
          blockfills: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'blockfills'),
          ultimuspool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ultimuspool'),
          terrapool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'terrapool'),
          luxor: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'luxor'),
          onethash: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onethash'),
          btccom: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btccom'),
          bitfarms: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfarms'),
          huobipool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'huobipool'),
          wayicn: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'wayicn'),
          canoepool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'canoepool'),
          btctop: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btctop'),
          bitcoincom: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoincom'),
          pool175btc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pool175btc'),
          gbminers: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'gbminers'),
          axbt: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'axbt'),
          asicminer: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'asicminer'),
          bitminter: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitminter'),
          bitcoinrussia: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinrussia'),
          btcserv: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcserv'),
          simplecoinus: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'simplecoinus'),
          btcguild: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcguild'),
          eligius: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eligius'),
          ozcoin: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ozcoin'),
          eclipsemc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eclipsemc'),
          maxbtc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'maxbtc'),
          triplemining: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'triplemining'),
          coinlab: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'coinlab'),
          pool50btc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pool50btc'),
          ghashio: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ghashio'),
          stminingcorp: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'stminingcorp'),
          bitparking: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitparking'),
          mmpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mmpool'),
          polmine: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'polmine'),
          kncminer: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kncminer'),
          bitalo: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitalo'),
          f2pool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'f2pool'),
          hhtt: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hhtt'),
          megabigpower: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'megabigpower'),
          mtred: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mtred'),
          nmcbit: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nmcbit'),
          yourbtcnet: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'yourbtcnet'),
          givemecoins: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'givemecoins'),
          braiinspool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'braiinspool'),
          antpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'antpool'),
          multicoinco: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'multicoinco'),
          bcpoolio: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bcpoolio'),
          cointerra: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'cointerra'),
          kanopool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kanopool'),
          solock: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'solock'),
          ckpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ckpool'),
          nicehash: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nicehash'),
          bitclub: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitclub'),
          bitcoinaffiliatenetwork: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinaffiliatenetwork'),
          btcc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcc'),
          bwpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bwpool'),
          exxbw: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'exxbw'),
          bitsolo: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitsolo'),
          bitfury: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfury'),
          twentyoneinc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'twentyoneinc'),
          digitalbtc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'digitalbtc'),
          eightbaochi: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eightbaochi'),
          mybtccoinpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'mybtccoinpool'),
          tbdice: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tbdice'),
          hashpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hashpool'),
          nexious: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'nexious'),
          bravomining: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bravomining'),
          hotpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hotpool'),
          okexpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okexpool'),
          bcmonster: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bcmonster'),
          onehash: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onehash'),
          bixin: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bixin'),
          tatmaspool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tatmaspool'),
          viabtc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'viabtc'),
          connectbtc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'connectbtc'),
          batpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'batpool'),
          waterhole: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'waterhole'),
          dcexploration: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dcexploration'),
          dcex: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dcex'),
          btpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btpool'),
          fiftyeightcoin: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'fiftyeightcoin'),
          bitcoinindia: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinindia'),
          shawnp0wers: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'shawnp0wers'),
          phashio: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'phashio'),
          rigpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'rigpool'),
          haozhuzhu: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'haozhuzhu'),
          sevenpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sevenpool'),
          miningkings: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningkings'),
          hashbx: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hashbx'),
          dpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'dpool'),
          rawpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'rawpool'),
          haominer: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'haominer'),
          helix: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'helix'),
          bitcoinukraine: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitcoinukraine'),
          poolin: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'poolin'),
          secretsuperstar: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'secretsuperstar'),
          tigerpoolnet: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tigerpoolnet'),
          sigmapoolcom: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sigmapoolcom'),
          okpooltop: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okpooltop'),
          hummerpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'hummerpool'),
          tangpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tangpool'),
          bytepool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bytepool'),
          spiderpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'spiderpool'),
          novablock: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'novablock'),
          miningcity: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningcity'),
          binancepool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'binancepool'),
          minerium: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'minerium'),
          lubiancom: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'lubiancom'),
          okkong: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okkong'),
          aaopool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'aaopool'),
          emcdpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'emcdpool'),
          foundryusa: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'foundryusa'),
          sbicrypto: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'sbicrypto'),
          arkpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'arkpool'),
          purebtccom: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'purebtccom'),
          marapool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'marapool'),
          kucoinpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'kucoinpool'),
          entrustcharitypool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'entrustcharitypool'),
          okminer: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'okminer'),
          titan: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'titan'),
          pegapool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'pegapool'),
          btcnuggets: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcnuggets'),
          cloudhashing: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'cloudhashing'),
          digitalxmintsy: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'digitalxmintsy'),
          telco214: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'telco214'),
          btcpoolparty: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcpoolparty'),
          multipool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'multipool'),
          transactioncoinmining: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'transactioncoinmining'),
          btcdig: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcdig'),
          trickysbtcpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'trickysbtcpool'),
          btcmp: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btcmp'),
          eobot: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'eobot'),
          unomp: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'unomp'),
          patels: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'patels'),
          gogreenlight: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'gogreenlight'),
          ekanembtc: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ekanembtc'),
          canoe: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'canoe'),
          tiger: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'tiger'),
          onem1x: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'onem1x'),
          zulupool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'zulupool'),
          secpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'secpool'),
          ocean: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'ocean'),
          whitepool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'whitepool'),
          wk057: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'wk057'),
          futurebitapollosolo: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'futurebitapollosolo'),
          carbonnegative: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'carbonnegative'),
          portlandhodl: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'portlandhodl'),
          phoenix: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'phoenix'),
          neopool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'neopool'),
          maxipool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'maxipool'),
          bitfufupool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'bitfufupool'),
          luckypool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'luckypool'),
          miningdutch: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningdutch'),
          publicpool: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'publicpool'),
          miningsquared: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'miningsquared'),
          innopolistech: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'innopolistech'),
          btclab: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'btclab'),
          parasite: create_1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(this, 'parasite'),
        },
      },
      price: {
        cents: {
          split: {
            open: createMetricPattern5(this, 'price_open_cents'),
            high: createMetricPattern5(this, 'price_high_cents'),
            low: createMetricPattern5(this, 'price_low_cents'),
            close: createMetricPattern5(this, 'price_close_cents'),
          },
          ohlc: createMetricPattern5(this, 'ohlc_cents'),
        },
        usd: {
          split: createCloseHighLowOpenPattern2(this, 'price'),
          ohlc: createMetricPattern1(this, 'price_ohlc'),
        },
        sats: createOhlcSplitPattern2(this, 'price'),
      },
      distribution: {
        supplyState: createMetricPattern11(this, 'supply_state'),
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
        addressesData: {
          funded: createMetricPattern31(this, 'fundedaddressdata'),
          empty: createMetricPattern32(this, 'emptyaddressdata'),
        },
        utxoCohorts: {
          all: {
            supply: create_30dHalvedTotalPattern(this, ''),
            outputs: createUtxoPattern(this, 'utxo_count'),
            activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(this, ''),
            realized: createAdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(this, ''),
            unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(this, ''),
            costBasis: createInvestedMaxMinPercentilesSpotPattern(this, ''),
            relative: {
              supplyInProfitRelToOwnSupply: createMetricPattern1(this, 'supply_in_profit_rel_to_own_supply'),
              supplyInLossRelToOwnSupply: createMetricPattern1(this, 'supply_in_loss_rel_to_own_supply'),
              unrealizedProfitRelToMarketCap: createMetricPattern1(this, 'unrealized_profit_rel_to_market_cap'),
              unrealizedLossRelToMarketCap: createMetricPattern1(this, 'unrealized_loss_rel_to_market_cap'),
              negUnrealizedLossRelToMarketCap: createMetricPattern1(this, 'neg_unrealized_loss_rel_to_market_cap'),
              netUnrealizedPnlRelToMarketCap: createMetricPattern1(this, 'net_unrealized_pnl_rel_to_market_cap'),
              nupl: createMetricPattern1(this, 'nupl'),
              unrealizedProfitRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_profit_rel_to_own_total_unrealized_pnl'),
              unrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'unrealized_loss_rel_to_own_total_unrealized_pnl'),
              negUnrealizedLossRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
              netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createMetricPattern1(this, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
              investedCapitalInProfitPct: createMetricPattern1(this, 'invested_capital_in_profit_pct'),
              investedCapitalInLossPct: createMetricPattern1(this, 'invested_capital_in_loss_pct'),
              unrealizedPeakRegretRelToMarketCap: createMetricPattern4(this, 'unrealized_peak_regret_rel_to_market_cap'),
            },
          },
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
          minAge: {
            _1d: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_1d_old'),
            _1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_1w_old'),
            _1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_1m_old'),
            _2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_2m_old'),
            _3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_3m_old'),
            _4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_4m_old'),
            _5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_5m_old'),
            _6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_6m_old'),
            _1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_1y_old'),
            _2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_2y_old'),
            _3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_3y_old'),
            _4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_4y_old'),
            _5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_5y_old'),
            _6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_6y_old'),
            _7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_7y_old'),
            _8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_8y_old'),
            _10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_10y_old'),
            _12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(this, 'utxos_over_12y_old'),
          },
          geAmount: {
            _1sat: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_1sat'),
            _10sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_10sats'),
            _100sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_100sats'),
            _1kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_1k_sats'),
            _10kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_10k_sats'),
            _100kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_100k_sats'),
            _1mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_1m_sats'),
            _10mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_10m_sats'),
            _1btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_1btc'),
            _10btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_10btc'),
            _100btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_100btc'),
            _1kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_1k_btc'),
            _10kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_over_10k_btc'),
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
          term: {
            short: {
              supply: create_30dHalvedTotalPattern(this, 'sth'),
              outputs: createUtxoPattern(this, 'sth_utxo_count'),
              activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(this, 'sth'),
              realized: createAdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(this, 'sth'),
              unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(this, 'sth'),
              costBasis: createInvestedMaxMinPercentilesSpotPattern(this, 'sth'),
              relative: createInvestedNegNetNuplSupplyUnrealizedPattern4(this, 'sth'),
            },
            long: {
              supply: create_30dHalvedTotalPattern(this, 'lth'),
              outputs: createUtxoPattern(this, 'lth_utxo_count'),
              activity: createCoinblocksCoindaysSatblocksSatdaysSentPattern(this, 'lth'),
              realized: createCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(this, 'lth'),
              unrealized: createGreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(this, 'lth'),
              costBasis: createInvestedMaxMinPercentilesSpotPattern(this, 'lth'),
              relative: createInvestedNegNetNuplSupplyUnrealizedPattern4(this, 'lth'),
            },
          },
          type: {
            p2pk65: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pk65'),
            p2pk33: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pk33'),
            p2pkh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2pkh'),
            p2ms: createActivityCostOutputsRealizedSupplyUnrealizedPattern(this, 'p2ms'),
            p2sh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2sh'),
            p2wpkh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2wpkh'),
            p2wsh: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2wsh'),
            p2tr: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2tr'),
            p2a: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(this, 'p2a'),
            unknown: createActivityCostOutputsRealizedSupplyUnrealizedPattern(this, 'unknown_outputs'),
            empty: createActivityCostOutputsRealizedSupplyUnrealizedPattern(this, 'empty_outputs'),
          },
          maxAge: {
            _1w: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_1w_old'),
            _1m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_1m_old'),
            _2m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_2m_old'),
            _3m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_3m_old'),
            _4m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_4m_old'),
            _5m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_5m_old'),
            _6m: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_6m_old'),
            _1y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_1y_old'),
            _2y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_2y_old'),
            _3y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_3y_old'),
            _4y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_4y_old'),
            _5y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_5y_old'),
            _6y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_6y_old'),
            _7y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_7y_old'),
            _8y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_8y_old'),
            _10y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_10y_old'),
            _12y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_12y_old'),
            _15y: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(this, 'utxos_under_15y_old'),
          },
          ltAmount: {
            _10sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10sats'),
            _100sats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_100sats'),
            _1kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1k_sats'),
            _10kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10k_sats'),
            _100kSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_100k_sats'),
            _1mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1m_sats'),
            _10mSats: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10m_sats'),
            _1btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1btc'),
            _10btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10btc'),
            _100btc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_100btc'),
            _1kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_1k_btc'),
            _10kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_10k_btc'),
            _100kBtc: createActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(this, 'utxos_under_100k_btc'),
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
        totalAddrCount: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(this, 'total_addr_count'),
        newAddrCount: {
          all: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'new_addr_count'),
          p2pk65: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pk65_new_addr_count'),
          p2pk33: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pk33_new_addr_count'),
          p2pkh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2pkh_new_addr_count'),
          p2sh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2sh_new_addr_count'),
          p2wpkh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2wpkh_new_addr_count'),
          p2wsh: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2wsh_new_addr_count'),
          p2tr: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2tr_new_addr_count'),
          p2a: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(this, 'p2a_new_addr_count'),
        },
        growthRate: {
          all: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'growth_rate'),
          p2pk65: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2pk65_growth_rate'),
          p2pk33: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2pk33_growth_rate'),
          p2pkh: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2pkh_growth_rate'),
          p2sh: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2sh_growth_rate'),
          p2wpkh: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2wpkh_growth_rate'),
          p2wsh: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2wsh_growth_rate'),
          p2tr: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2tr_growth_rate'),
          p2a: createAverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'p2a_growth_rate'),
        },
        fundedaddressindex: createMetricPattern31(this, 'fundedaddressindex'),
        emptyaddressindex: createMetricPattern32(this, 'emptyaddressindex'),
      },
      supply: {
        circulating: {
          sats: createMetricPattern3(this, 'circulating_supply'),
          bitcoin: createMetricPattern3(this, 'circulating_supply_btc'),
          dollars: createMetricPattern3(this, 'circulating_supply_usd'),
        },
        burned: {
          opreturn: createBitcoinDollarsSatsPattern3(this, 'opreturn_supply'),
          unspendable: createBitcoinDollarsSatsPattern3(this, 'unspendable_supply'),
        },
        inflation: createMetricPattern4(this, 'inflation_rate'),
        velocity: {
          btc: createMetricPattern4(this, 'btc_velocity'),
          usd: createMetricPattern4(this, 'usd_velocity'),
        },
        marketCap: createMetricPattern1(this, 'market_cap'),
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
