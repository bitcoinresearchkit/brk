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
 * @property {(RangeIndex|null)=} start - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
 * @property {(RangeIndex|null)=} end - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
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
/** @typedef {number} Epoch */
/**
 * @typedef {Object} ErrorBody
 * @property {ErrorDetail} error
 */
/**
 * @typedef {Object} ErrorDetail
 * @property {string} type - Error category: "invalid_request", "forbidden", "not_found", "unavailable", or "internal"
 * @property {string} code - Machine-readable error code (e.g. "invalid_address", "metric_not_found")
 * @property {string} message - Human-readable description
 * @property {string} docUrl - Link to API documentation
 */
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
/** @typedef {number} Halving */
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
 * @property {string} version
 * @property {string} timestamp
 * @property {string} startedAt - Server start time (ISO 8601)
 * @property {number} uptimeSeconds - Uptime in seconds
 * @property {Height} indexedHeight - Height of the last indexed block
 * @property {Height} computedHeight - Height of the last computed block (metrics)
 * @property {Height} tipHeight - Height of the chain tip (from Bitcoin node)
 * @property {Height} blocksBehind - Number of blocks behind the tip
 * @property {string} lastIndexedAt - Human-readable timestamp of the last indexed block (ISO 8601)
 * @property {Timestamp} lastIndexedAtUnix - Unix timestamp of the last indexed block
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
 * @typedef {("minute10"|"minute30"|"hour1"|"hour4"|"hour12"|"day1"|"day3"|"week1"|"month1"|"month3"|"month6"|"year1"|"year10"|"halving"|"epoch"|"height"|"txindex"|"txinindex"|"txoutindex"|"emptyoutputindex"|"opreturnindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"unknownoutputindex"|"fundedaddressindex"|"emptyaddressindex"|"pairoutputindex")} Index
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
 * Metadata about a metric
 *
 * @typedef {Object} MetricInfo
 * @property {Index[]} indexes - Available indexes
 * @property {string} type - Value type (e.g. "f32", "u64", "Sats")
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
 * @property {(RangeIndex|null)=} start - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
 * @property {(RangeIndex|null)=} end - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
 * @property {(Limit|null)=} limit - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
 * @property {Format=} format - Format of the output
 */
/**
 * Legacy metric selection parameters (deprecated)
 *
 * @typedef {Object} MetricSelectionLegacy
 * @property {Index} index
 * @property {Metrics} ids
 * @property {(RangeIndex|null)=} start - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
 * @property {(RangeIndex|null)=} end - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
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
 * @property {number} totalCount - Total number of metrics
 * @property {number} perPage - Results per page
 * @property {boolean} hasMore - Whether more pages are available after the current one
 * @property {string[]} metrics - List of metric names
 */
/**
 * Pagination parameters for paginated API endpoints
 *
 * @typedef {Object} Pagination
 * @property {?number=} page - Pagination index
 * @property {?number=} perPage - Results per page (default: 1000, max: 1000)
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
 * A range boundary: integer index, date, or timestamp.
 *
 * @typedef {(number|Date|Timestamp)} RangeIndex
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
 * @typedef {Object} SearchQuery
 * @property {Metric} q - Search query string
 * @property {Limit=} limit - Maximum number of results
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
/**
 * Fixed-size 64-bit signed integer optimized for on-disk storage
 *
 * @typedef {number} StoredI64
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
 * @property {Height} computedHeight - Height of the last computed block (metrics)
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

const _i1 = /** @type {const} */ (["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halving", "epoch", "height"]);
const _i2 = /** @type {const} */ (["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halving", "epoch"]);
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
const _i16 = /** @type {const} */ (["halving"]);
const _i17 = /** @type {const} */ (["epoch"]);
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

/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halving: MetricEndpointBuilder<T>, readonly epoch: MetricEndpointBuilder<T>, readonly height: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern1 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern1<T>} */
function createMetricPattern1(client, name) { return /** @type {MetricPattern1<T>} */ (_mp(client, name, _i1)); }
/** @template T @typedef {{ name: string, by: { readonly minute10: DateMetricEndpointBuilder<T>, readonly minute30: DateMetricEndpointBuilder<T>, readonly hour1: DateMetricEndpointBuilder<T>, readonly hour4: DateMetricEndpointBuilder<T>, readonly hour12: DateMetricEndpointBuilder<T>, readonly day1: DateMetricEndpointBuilder<T>, readonly day3: DateMetricEndpointBuilder<T>, readonly week1: DateMetricEndpointBuilder<T>, readonly month1: DateMetricEndpointBuilder<T>, readonly month3: DateMetricEndpointBuilder<T>, readonly month6: DateMetricEndpointBuilder<T>, readonly year1: DateMetricEndpointBuilder<T>, readonly year10: DateMetricEndpointBuilder<T>, readonly halving: MetricEndpointBuilder<T>, readonly epoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern2 */
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
/** @template T @typedef {{ name: string, by: { readonly halving: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern16 */
/** @template T @param {BrkClientBase} client @param {string} name @returns {MetricPattern16<T>} */
function createMetricPattern16(client, name) { return /** @type {MetricPattern16<T>} */ (_mp(client, name, _i16)); }
/** @template T @typedef {{ name: string, by: { readonly epoch: MetricEndpointBuilder<T> }, indexes: () => readonly Index[], get: (index: Index) => MetricEndpointBuilder<T>|undefined }} MetricPattern17 */
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
 * @typedef {Object} _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern
 * @property {CentsSatsUsdPattern} _0sd
 * @property {PriceValuePattern} m05sd
 * @property {PriceValuePattern} m15sd
 * @property {PriceValuePattern} m1sd
 * @property {PriceValuePattern} m25sd
 * @property {PriceValuePattern} m2sd
 * @property {PriceValuePattern} m3sd
 * @property {PriceValuePattern} p05sd
 * @property {PriceValuePattern} p15sd
 * @property {PriceValuePattern} p1sd
 * @property {PriceValuePattern} p25sd
 * @property {PriceValuePattern} p2sd
 * @property {PriceValuePattern} p3sd
 * @property {MetricPattern1<StoredF32>} sd
 * @property {MetricPattern1<StoredF32>} zscore
 */

/**
 * Create a _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern}
 */
function create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc) {
  return {
    _0sd: createCentsSatsUsdPattern(client, _m(acc, '0sd_4y')),
    m05sd: createPriceValuePattern(client, acc),
    m15sd: createPriceValuePattern(client, acc),
    m1sd: createPriceValuePattern(client, acc),
    m25sd: createPriceValuePattern(client, acc),
    m2sd: createPriceValuePattern(client, acc),
    m3sd: createPriceValuePattern(client, acc),
    p05sd: createPriceValuePattern(client, acc),
    p15sd: createPriceValuePattern(client, acc),
    p1sd: createPriceValuePattern(client, acc),
    p25sd: createPriceValuePattern(client, acc),
    p2sd: createPriceValuePattern(client, acc),
    p3sd: createPriceValuePattern(client, acc),
    sd: createMetricPattern1(client, _m(acc, 'sd_4y')),
    zscore: createMetricPattern1(client, _m(acc, 'zscore_4y')),
  };
}

/**
 * @typedef {Object} _10y1m1w1y2y3m3y4y5y6m6y8yPattern2
 * @property {BpsPercentRatioPattern2} _10y
 * @property {BpsPercentRatioPattern2} _1m
 * @property {BpsPercentRatioPattern2} _1w
 * @property {BpsPercentRatioPattern2} _1y
 * @property {BpsPercentRatioPattern2} _2y
 * @property {BpsPercentRatioPattern2} _3m
 * @property {BpsPercentRatioPattern2} _3y
 * @property {BpsPercentRatioPattern2} _4y
 * @property {BpsPercentRatioPattern2} _5y
 * @property {BpsPercentRatioPattern2} _6m
 * @property {BpsPercentRatioPattern2} _6y
 * @property {BpsPercentRatioPattern2} _8y
 */

/**
 * Create a _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2}
 */
function create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, acc) {
  return {
    _10y: createBpsPercentRatioPattern2(client, _m(acc, '10y')),
    _1m: createBpsPercentRatioPattern2(client, _m(acc, '1m')),
    _1w: createBpsPercentRatioPattern2(client, _m(acc, '1w')),
    _1y: createBpsPercentRatioPattern2(client, _m(acc, '1y')),
    _2y: createBpsPercentRatioPattern2(client, _m(acc, '2y')),
    _3m: createBpsPercentRatioPattern2(client, _m(acc, '3m')),
    _3y: createBpsPercentRatioPattern2(client, _m(acc, '3y')),
    _4y: createBpsPercentRatioPattern2(client, _m(acc, '4y')),
    _5y: createBpsPercentRatioPattern2(client, _m(acc, '5y')),
    _6m: createBpsPercentRatioPattern2(client, _m(acc, '6m')),
    _6y: createBpsPercentRatioPattern2(client, _m(acc, '6y')),
    _8y: createBpsPercentRatioPattern2(client, _m(acc, '8y')),
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
 * @typedef {Object} CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern
 * @property {CentsDeltaRelUsdPattern} cap
 * @property {BaseCumulativeSumPattern3} grossPnl
 * @property {LowerPriceUpperPattern} investor
 * @property {BaseCapitulationCumulativeNegativeRelSumValuePattern} loss
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {BaseChangeCumulativeDeltaRelSumPattern} netPnl
 * @property {BaseCumulativeRelPattern} peakRegret
 * @property {BpsCentsPercentilesRatioSatsSmaStdUsdPattern} price
 * @property {BaseCumulativeDistributionRelSumValuePattern} profit
 * @property {_1m1w1y24hPattern<StoredF64>} profitToLossRatio
 * @property {_1m1w1y24hPattern2} sellSideRiskRatio
 * @property {AdjustedRatioValuePattern} sopr
 */

/**
 * Create a CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern}
 */
function createCapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern(client, acc) {
  return {
    cap: createCentsDeltaRelUsdPattern(client, _m(acc, 'realized_cap')),
    grossPnl: createBaseCumulativeSumPattern3(client, _m(acc, 'realized_gross_pnl')),
    investor: createLowerPriceUpperPattern(client, acc),
    loss: createBaseCapitulationCumulativeNegativeRelSumValuePattern(client, acc),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    netPnl: createBaseChangeCumulativeDeltaRelSumPattern(client, _m(acc, 'net')),
    peakRegret: createBaseCumulativeRelPattern(client, _m(acc, 'realized_peak_regret')),
    price: createBpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, _m(acc, 'realized_price')),
    profit: createBaseCumulativeDistributionRelSumValuePattern(client, acc),
    profitToLossRatio: create_1m1w1y24hPattern(client, _m(acc, 'realized_profit_to_loss_ratio')),
    sellSideRiskRatio: create_1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio')),
    sopr: createAdjustedRatioValuePattern(client, acc),
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
    pct10: createMetricPattern18(client, _m(acc, 'pct10')),
    pct25: createMetricPattern18(client, _m(acc, 'pct25')),
    pct75: createMetricPattern18(client, _m(acc, 'pct75')),
    pct90: createMetricPattern18(client, _m(acc, 'pct90')),
    rolling: createAverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc),
    sum: createMetricPattern18(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern
 * @property {_1m1w1y24hPattern<StoredU64>} average
 * @property {MetricPattern1<StoredU64>} base
 * @property {MetricPattern1<StoredU64>} cumulative
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
 * Create a AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern}
 */
function createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc) {
  return {
    average: create_1m1w1y24hPattern(client, _m(acc, 'average')),
    base: createMetricPattern1(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    max: create_1m1w1y24hPattern(client, _m(acc, 'max')),
    median: create_1m1w1y24hPattern(client, _m(acc, 'median')),
    min: create_1m1w1y24hPattern(client, _m(acc, 'min')),
    pct10: create_1m1w1y24hPattern(client, _m(acc, 'pct10')),
    pct25: create_1m1w1y24hPattern(client, _m(acc, 'pct25')),
    pct75: create_1m1w1y24hPattern(client, _m(acc, 'pct75')),
    pct90: create_1m1w1y24hPattern(client, _m(acc, 'pct90')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageGainsLossesRsiStochPattern
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {BpsPercentRatioPattern4} rsi
 * @property {BpsPercentRatioPattern4} rsiMax
 * @property {BpsPercentRatioPattern4} rsiMin
 * @property {BpsPercentRatioPattern4} stochRsi
 * @property {BpsPercentRatioPattern4} stochRsiD
 * @property {BpsPercentRatioPattern4} stochRsiK
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
    rsi: createBpsPercentRatioPattern4(client, _m(acc, '24h')),
    rsiMax: createBpsPercentRatioPattern4(client, _m(acc, 'max_24h')),
    rsiMin: createBpsPercentRatioPattern4(client, _m(acc, 'min_24h')),
    stochRsi: createBpsPercentRatioPattern4(client, _m(acc, 'stoch_24h')),
    stochRsiD: createBpsPercentRatioPattern4(client, _m(acc, 'stoch_d_24h')),
    stochRsiK: createBpsPercentRatioPattern4(client, _m(acc, 'stoch_k_24h')),
  };
}

/**
 * @typedef {Object} AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3
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
 * Create a AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3}
 */
function createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(client, acc) {
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
    pct10: create_1m1w1y24hPattern(client, _m(acc, 'pct10')),
    pct25: create_1m1w1y24hPattern(client, _m(acc, 'pct25')),
    pct75: create_1m1w1y24hPattern(client, _m(acc, 'pct75')),
    pct90: create_1m1w1y24hPattern(client, _m(acc, 'pct90')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern
 * @property {BtcCentsSatsUsdPattern} average
 * @property {BtcCentsSatsUsdPattern} max
 * @property {BtcCentsSatsUsdPattern} median
 * @property {BtcCentsSatsUsdPattern} min
 * @property {BtcCentsSatsUsdPattern} pct10
 * @property {BtcCentsSatsUsdPattern} pct25
 * @property {BtcCentsSatsUsdPattern} pct75
 * @property {BtcCentsSatsUsdPattern} pct90
 */

/**
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc) {
  return {
    average: createBtcCentsSatsUsdPattern(client, _m(acc, 'average')),
    max: createBtcCentsSatsUsdPattern(client, _m(acc, 'max')),
    median: createBtcCentsSatsUsdPattern(client, _m(acc, 'median')),
    min: createBtcCentsSatsUsdPattern(client, _m(acc, 'min')),
    pct10: createBtcCentsSatsUsdPattern(client, _m(acc, 'pct10')),
    pct25: createBtcCentsSatsUsdPattern(client, _m(acc, 'pct25')),
    pct75: createBtcCentsSatsUsdPattern(client, _m(acc, 'pct75')),
    pct90: createBtcCentsSatsUsdPattern(client, _m(acc, 'pct90')),
  };
}

/**
 * @typedef {Object} BaseCapitulationCumulativeNegativeRelSumValuePattern
 * @property {CentsUsdPattern2} base
 * @property {MetricPattern1<Dollars>} capitulationFlow
 * @property {CentsUsdPattern2} cumulative
 * @property {MetricPattern1<Dollars>} negative
 * @property {BpsPercentRatioPattern} relToRcap
 * @property {_1m1w1y24hPattern5} sum
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a BaseCapitulationCumulativeNegativeRelSumValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCapitulationCumulativeNegativeRelSumValuePattern}
 */
function createBaseCapitulationCumulativeNegativeRelSumValuePattern(client, acc) {
  return {
    base: createCentsUsdPattern2(client, _m(acc, 'realized_loss')),
    capitulationFlow: createMetricPattern1(client, _m(acc, 'capitulation_flow')),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'realized_loss_cumulative')),
    negative: createMetricPattern1(client, _m(acc, 'neg_realized_loss')),
    relToRcap: createBpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_rcap')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'realized_loss_sum')),
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'loss_value_created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'loss_value_destroyed')),
  };
}

/**
 * @typedef {Object} BpsCentsPercentilesRatioSatsSmaStdUsdPattern
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<Cents>} cents
 * @property {Pct1Pct2Pct5Pct95Pct98Pct99Pattern} percentiles
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<SatsFract>} sats
 * @property {_1m1w1y2y4yAllPattern} sma
 * @property {_1y2y4yAllPattern} stdDev
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BpsCentsPercentilesRatioSatsSmaStdUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsCentsPercentilesRatioSatsSmaStdUsdPattern}
 */
function createBpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'ratio_bps')),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    percentiles: createPct1Pct2Pct5Pct95Pct98Pct99Pattern(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    sma: create_1m1w1y2y4yAllPattern(client, _m(acc, 'ratio_sma')),
    stdDev: create_1y2y4yAllPattern(client, _m(acc, 'ratio')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2
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
 * Create a AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2<T>}
 */
function createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(client, acc) {
  return {
    average: createMetricPattern18(client, _m(acc, 'average')),
    max: createMetricPattern18(client, _m(acc, 'max')),
    median: createMetricPattern18(client, _m(acc, 'median')),
    min: createMetricPattern18(client, _m(acc, 'min')),
    pct10: createMetricPattern18(client, _m(acc, 'pct10')),
    pct25: createMetricPattern18(client, _m(acc, 'pct25')),
    pct75: createMetricPattern18(client, _m(acc, 'pct75')),
    pct90: createMetricPattern18(client, _m(acc, 'pct90')),
  };
}

/**
 * @typedef {Object} _10y2y3y4y5y6y8yPattern
 * @property {BpsPercentRatioPattern2} _10y
 * @property {BpsPercentRatioPattern2} _2y
 * @property {BpsPercentRatioPattern2} _3y
 * @property {BpsPercentRatioPattern2} _4y
 * @property {BpsPercentRatioPattern2} _5y
 * @property {BpsPercentRatioPattern2} _6y
 * @property {BpsPercentRatioPattern2} _8y
 */

/**
 * Create a _10y2y3y4y5y6y8yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_10y2y3y4y5y6y8yPattern}
 */
function create_10y2y3y4y5y6y8yPattern(client, acc) {
  return {
    _10y: createBpsPercentRatioPattern2(client, _m(acc, '10y')),
    _2y: createBpsPercentRatioPattern2(client, _m(acc, '2y')),
    _3y: createBpsPercentRatioPattern2(client, _m(acc, '3y')),
    _4y: createBpsPercentRatioPattern2(client, _m(acc, '4y')),
    _5y: createBpsPercentRatioPattern2(client, _m(acc, '5y')),
    _6y: createBpsPercentRatioPattern2(client, _m(acc, '6y')),
    _8y: createBpsPercentRatioPattern2(client, _m(acc, '8y')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hBpsPercentRatioPattern
 * @property {BpsPercentRatioPattern4} _1m
 * @property {BpsPercentRatioPattern4} _1w
 * @property {BpsPercentRatioPattern4} _1y
 * @property {BpsPercentRatioPattern4} _24h
 * @property {MetricPattern1<BasisPoints16>} bps
 * @property {MetricPattern1<StoredF32>} percent
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a _1m1w1y24hBpsPercentRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hBpsPercentRatioPattern}
 */
function create_1m1w1y24hBpsPercentRatioPattern(client, acc) {
  return {
    _1m: createBpsPercentRatioPattern4(client, _m(acc, '1m')),
    _1w: createBpsPercentRatioPattern4(client, _m(acc, '1w')),
    _1y: createBpsPercentRatioPattern4(client, _m(acc, '1y')),
    _24h: createBpsPercentRatioPattern4(client, _m(acc, '24h')),
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} BaseCumulativeDistributionRelSumValuePattern
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {MetricPattern1<Dollars>} distributionFlow
 * @property {BpsPercentRatioPattern} relToRcap
 * @property {_1m1w1y24hPattern5} sum
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a BaseCumulativeDistributionRelSumValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeDistributionRelSumValuePattern}
 */
function createBaseCumulativeDistributionRelSumValuePattern(client, acc) {
  return {
    base: createCentsUsdPattern2(client, _m(acc, 'realized_profit')),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'realized_profit_cumulative')),
    distributionFlow: createMetricPattern1(client, _m(acc, 'distribution_flow')),
    relToRcap: createBpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_rcap')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'realized_profit_sum')),
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'profit_value_created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'profit_value_destroyed')),
  };
}

/**
 * @typedef {Object} BaseCumulativeNegativeRelSumPattern2
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {MetricPattern1<Dollars>} negative
 * @property {BpsPercentRatioPattern4} relToMcap
 * @property {BpsPercentRatioPattern4} relToOwnGross
 * @property {BpsPercentRatioPattern} relToOwnMcap
 * @property {_1m1w1y24hPattern5} sum
 */

/**
 * Create a BaseCumulativeNegativeRelSumPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeNegativeRelSumPattern2}
 */
function createBaseCumulativeNegativeRelSumPattern2(client, acc) {
  return {
    base: createCentsUsdPattern2(client, _m(acc, 'unrealized_loss')),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'unrealized_loss_cumulative')),
    negative: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    relToMcap: createBpsPercentRatioPattern4(client, _m(acc, 'unrealized_loss_rel_to_mcap')),
    relToOwnGross: createBpsPercentRatioPattern4(client, _m(acc, 'unrealized_loss_rel_to_own_gross_pnl')),
    relToOwnMcap: createBpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_own_mcap')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'unrealized_loss_sum')),
  };
}

/**
 * @typedef {Object} CapLossMvrvNetPriceProfitSoprPattern
 * @property {CentsDeltaUsdPattern} cap
 * @property {BaseCumulativeNegativeSumPattern} loss
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {BaseCumulativeDeltaSumPattern} netPnl
 * @property {BpsCentsRatioSatsUsdPattern} price
 * @property {BaseCumulativeSumPattern3} profit
 * @property {RatioValuePattern} sopr
 */

/**
 * Create a CapLossMvrvNetPriceProfitSoprPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapLossMvrvNetPriceProfitSoprPattern}
 */
function createCapLossMvrvNetPriceProfitSoprPattern(client, acc) {
  return {
    cap: createCentsDeltaUsdPattern(client, _m(acc, 'realized_cap')),
    loss: createBaseCumulativeNegativeSumPattern(client, acc),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    netPnl: createBaseCumulativeDeltaSumPattern(client, _m(acc, 'net_realized_pnl')),
    price: createBpsCentsRatioSatsUsdPattern(client, _m(acc, 'realized_price')),
    profit: createBaseCumulativeSumPattern3(client, _m(acc, 'realized_profit')),
    sopr: createRatioValuePattern(client, acc),
  };
}

/**
 * @typedef {Object} GrossInvestedLossNetNuplProfitSentimentPattern2
 * @property {CentsUsdPattern2} grossPnl
 * @property {InPattern} investedCapital
 * @property {BaseCumulativeNegativeRelSumPattern2} loss
 * @property {CentsRelUsdPattern2} netPnl
 * @property {BpsRatioPattern} nupl
 * @property {BaseCumulativeRelSumPattern2} profit
 * @property {GreedNetPainPattern} sentiment
 */

/**
 * Create a GrossInvestedLossNetNuplProfitSentimentPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {GrossInvestedLossNetNuplProfitSentimentPattern2}
 */
function createGrossInvestedLossNetNuplProfitSentimentPattern2(client, acc) {
  return {
    grossPnl: createCentsUsdPattern2(client, _m(acc, 'unrealized_gross_pnl')),
    investedCapital: createInPattern(client, _m(acc, 'invested_capital_in')),
    loss: createBaseCumulativeNegativeRelSumPattern2(client, acc),
    netPnl: createCentsRelUsdPattern2(client, _m(acc, 'net_unrealized_pnl')),
    nupl: createBpsRatioPattern(client, _m(acc, 'nupl')),
    profit: createBaseCumulativeRelSumPattern2(client, _m(acc, 'unrealized_profit')),
    sentiment: createGreedNetPainPattern(client, acc),
  };
}

/**
 * @typedef {Object} _1m1w1y2y4yAllPattern
 * @property {BpsRatioPattern2} _1m
 * @property {BpsRatioPattern2} _1w
 * @property {BpsRatioPattern2} _1y
 * @property {BpsRatioPattern2} _2y
 * @property {BpsRatioPattern2} _4y
 * @property {BpsRatioPattern2} all
 */

/**
 * Create a _1m1w1y2y4yAllPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y2y4yAllPattern}
 */
function create_1m1w1y2y4yAllPattern(client, acc) {
  return {
    _1m: createBpsRatioPattern2(client, _m(acc, '1m')),
    _1w: createBpsRatioPattern2(client, _m(acc, '1w')),
    _1y: createBpsRatioPattern2(client, _m(acc, '1y')),
    _2y: createBpsRatioPattern2(client, _m(acc, '2y')),
    _4y: createBpsRatioPattern2(client, _m(acc, '4y')),
    all: createBpsRatioPattern2(client, _m(acc, 'all')),
  };
}

/**
 * @typedef {Object} BaseChangeCumulativeDeltaRelSumPattern
 * @property {CentsUsdPattern} base
 * @property {RelPattern} change1m
 * @property {CentsUsdPattern} cumulative
 * @property {ChangeRatePattern3} delta
 * @property {BpsPercentRatioPattern2} relToRcap
 * @property {_1m1w1y24hPattern4} sum
 */

/**
 * Create a BaseChangeCumulativeDeltaRelSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseChangeCumulativeDeltaRelSumPattern}
 */
function createBaseChangeCumulativeDeltaRelSumPattern(client, acc) {
  return {
    base: createCentsUsdPattern(client, _m(acc, 'realized_pnl')),
    change1m: createRelPattern(client, _m(acc, 'pnl_change_1m_rel_to')),
    cumulative: createCentsUsdPattern(client, _m(acc, 'realized_pnl_cumulative')),
    delta: createChangeRatePattern3(client, _m(acc, 'realized_pnl_delta')),
    relToRcap: createBpsPercentRatioPattern2(client, _m(acc, 'realized_pnl_rel_to_rcap')),
    sum: create_1m1w1y24hPattern4(client, _m(acc, 'realized_pnl_sum')),
  };
}

/**
 * @typedef {Object} BaseCumulativeRelSumPattern2
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {BpsPercentRatioPattern4} relToMcap
 * @property {BpsPercentRatioPattern4} relToOwnGross
 * @property {BpsPercentRatioPattern4} relToOwnMcap
 * @property {_1m1w1y24hPattern5} sum
 */

/**
 * Create a BaseCumulativeRelSumPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeRelSumPattern2}
 */
function createBaseCumulativeRelSumPattern2(client, acc) {
  return {
    base: createCentsUsdPattern2(client, acc),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'cumulative')),
    relToMcap: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_mcap')),
    relToOwnGross: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_own_gross_pnl')),
    relToOwnMcap: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_own_mcap')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BpsCentsPercentilesRatioSatsUsdPattern
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<Cents>} cents
 * @property {Pct1Pct2Pct5Pct95Pct98Pct99Pattern} percentiles
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BpsCentsPercentilesRatioSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsCentsPercentilesRatioSatsUsdPattern}
 */
function createBpsCentsPercentilesRatioSatsUsdPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'ratio_bps')),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    percentiles: createPct1Pct2Pct5Pct95Pct98Pct99Pattern(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} BtcCentsRelSatsUsdPattern3
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {MetricPattern1<Cents>} cents
 * @property {BpsPercentRatioPattern4} relToCirculating
 * @property {BpsPercentRatioPattern4} relToOwn
 * @property {MetricPattern1<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcCentsRelSatsUsdPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcCentsRelSatsUsdPattern3}
 */
function createBtcCentsRelSatsUsdPattern3(client, acc) {
  return {
    btc: createMetricPattern1(client, acc),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    relToCirculating: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_circulating')),
    relToOwn: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_own')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} CapLossMvrvPriceProfitSoprPattern
 * @property {CentsDeltaUsdPattern} cap
 * @property {BaseCumulativeSumPattern3} loss
 * @property {MetricPattern1<StoredF32>} mvrv
 * @property {BpsCentsRatioSatsUsdPattern} price
 * @property {BaseCumulativeSumPattern3} profit
 * @property {ValuePattern} sopr
 */

/**
 * Create a CapLossMvrvPriceProfitSoprPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CapLossMvrvPriceProfitSoprPattern}
 */
function createCapLossMvrvPriceProfitSoprPattern(client, acc) {
  return {
    cap: createCentsDeltaUsdPattern(client, _m(acc, 'realized_cap')),
    loss: createBaseCumulativeSumPattern3(client, _m(acc, 'realized_loss')),
    mvrv: createMetricPattern1(client, _m(acc, 'mvrv')),
    price: createBpsCentsRatioSatsUsdPattern(client, _m(acc, 'realized_price')),
    profit: createBaseCumulativeSumPattern3(client, _m(acc, 'realized_profit')),
    sopr: createValuePattern(client, _m(acc, 'value')),
  };
}

/**
 * @typedef {Object} DeltaHalfInRelTotalPattern
 * @property {ChangeRatePattern2} delta
 * @property {BtcCentsSatsUsdPattern} half
 * @property {BtcCentsRelSatsUsdPattern} inLoss
 * @property {BtcCentsRelSatsUsdPattern} inProfit
 * @property {BpsPercentRatioPattern4} relToCirculating
 * @property {BtcCentsSatsUsdPattern} total
 */

/**
 * Create a DeltaHalfInRelTotalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DeltaHalfInRelTotalPattern}
 */
function createDeltaHalfInRelTotalPattern(client, acc) {
  return {
    delta: createChangeRatePattern2(client, _m(acc, 'delta')),
    half: createBtcCentsSatsUsdPattern(client, _m(acc, 'half')),
    inLoss: createBtcCentsRelSatsUsdPattern(client, _m(acc, 'in_loss')),
    inProfit: createBtcCentsRelSatsUsdPattern(client, _m(acc, 'in_profit')),
    relToCirculating: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_circulating')),
    total: createBtcCentsSatsUsdPattern(client, acc),
  };
}

/**
 * @typedef {Object} DeltaHalfInRelTotalPattern2
 * @property {ChangeRatePattern2} delta
 * @property {BtcCentsSatsUsdPattern} half
 * @property {BtcCentsRelSatsUsdPattern3} inLoss
 * @property {BtcCentsRelSatsUsdPattern3} inProfit
 * @property {BpsPercentRatioPattern4} relToCirculating
 * @property {BtcCentsSatsUsdPattern} total
 */

/**
 * Create a DeltaHalfInRelTotalPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DeltaHalfInRelTotalPattern2}
 */
function createDeltaHalfInRelTotalPattern2(client, acc) {
  return {
    delta: createChangeRatePattern2(client, _m(acc, 'delta')),
    half: createBtcCentsSatsUsdPattern(client, _m(acc, 'half')),
    inLoss: createBtcCentsRelSatsUsdPattern3(client, _m(acc, 'in_loss')),
    inProfit: createBtcCentsRelSatsUsdPattern3(client, _m(acc, 'in_profit')),
    relToCirculating: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_circulating')),
    total: createBtcCentsSatsUsdPattern(client, acc),
  };
}

/**
 * @typedef {Object} Pct1Pct2Pct5Pct95Pct98Pct99Pattern
 * @property {BpsPriceRatioPattern} pct1
 * @property {BpsPriceRatioPattern} pct2
 * @property {BpsPriceRatioPattern} pct5
 * @property {BpsPriceRatioPattern} pct95
 * @property {BpsPriceRatioPattern} pct98
 * @property {BpsPriceRatioPattern} pct99
 */

/**
 * Create a Pct1Pct2Pct5Pct95Pct98Pct99Pattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Pct1Pct2Pct5Pct95Pct98Pct99Pattern}
 */
function createPct1Pct2Pct5Pct95Pct98Pct99Pattern(client, acc) {
  return {
    pct1: createBpsPriceRatioPattern(client, acc),
    pct2: createBpsPriceRatioPattern(client, acc),
    pct5: createBpsPriceRatioPattern(client, acc),
    pct95: createBpsPriceRatioPattern(client, acc),
    pct98: createBpsPriceRatioPattern(client, acc),
    pct99: createBpsPriceRatioPattern(client, acc),
  };
}

/**
 * @typedef {Object} ActivityOutputsRealizedSupplyUnrealizedPattern
 * @property {CoindaysSentPattern} activity
 * @property {UnspentPattern} outputs
 * @property {CapLossMvrvNetPriceProfitSoprPattern} realized
 * @property {DeltaHalfInRelTotalPattern} supply
 * @property {LossNetNuplProfitPattern} unrealized
 */

/**
 * Create a ActivityOutputsRealizedSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ActivityOutputsRealizedSupplyUnrealizedPattern}
 */
function createActivityOutputsRealizedSupplyUnrealizedPattern(client, acc) {
  return {
    activity: createCoindaysSentPattern(client, acc),
    outputs: createUnspentPattern(client, _m(acc, 'utxo_count')),
    realized: createCapLossMvrvNetPriceProfitSoprPattern(client, acc),
    supply: createDeltaHalfInRelTotalPattern(client, _m(acc, 'supply')),
    unrealized: createLossNetNuplProfitPattern(client, acc),
  };
}

/**
 * @typedef {Object} AddressOutputsRealizedSupplyUnrealizedPattern
 * @property {DeltaInnerPattern} addressCount
 * @property {UnspentPattern} outputs
 * @property {CapLossMvrvPriceProfitSoprPattern} realized
 * @property {DeltaHalfTotalPattern} supply
 * @property {NuplPattern} unrealized
 */

/**
 * Create a AddressOutputsRealizedSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AddressOutputsRealizedSupplyUnrealizedPattern}
 */
function createAddressOutputsRealizedSupplyUnrealizedPattern(client, acc) {
  return {
    addressCount: createDeltaInnerPattern(client, _m(acc, 'address_count')),
    outputs: createUnspentPattern(client, _m(acc, 'utxo_count')),
    realized: createCapLossMvrvPriceProfitSoprPattern(client, acc),
    supply: createDeltaHalfTotalPattern(client, _m(acc, 'supply')),
    unrealized: createNuplPattern(client, _m(acc, 'nupl')),
  };
}

/**
 * @typedef {Object} BaseCumulativeInSumPattern
 * @property {MetricPattern1<Sats>} base
 * @property {MetricPattern1<Sats>} cumulative
 * @property {BaseCumulativeSumPattern4} inLoss
 * @property {BaseCumulativeSumPattern4} inProfit
 * @property {_1m1w1y24hPattern<Sats>} sum
 */

/**
 * Create a BaseCumulativeInSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeInSumPattern}
 */
function createBaseCumulativeInSumPattern(client, acc) {
  return {
    base: createMetricPattern1(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    inLoss: createBaseCumulativeSumPattern4(client, _m(acc, 'in_loss')),
    inProfit: createBaseCumulativeSumPattern4(client, _m(acc, 'in_profit')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BpsCentsRatioSatsUsdPattern
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BpsCentsRatioSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsCentsRatioSatsUsdPattern}
 */
function createBpsCentsRatioSatsUsdPattern(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'ratio_bps')),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} BtcCentsRelSatsUsdPattern
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {MetricPattern1<Cents>} cents
 * @property {BpsPercentRatioPattern4} relToCirculating
 * @property {MetricPattern1<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcCentsRelSatsUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcCentsRelSatsUsdPattern}
 */
function createBtcCentsRelSatsUsdPattern(client, acc) {
  return {
    btc: createMetricPattern1(client, acc),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    relToCirculating: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_circulating')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} BtcCentsRelSatsUsdPattern2
 * @property {MetricPattern1<Bitcoin>} btc
 * @property {MetricPattern1<Cents>} cents
 * @property {BpsPercentRatioPattern4} relToOwn
 * @property {MetricPattern1<Sats>} sats
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a BtcCentsRelSatsUsdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BtcCentsRelSatsUsdPattern2}
 */
function createBtcCentsRelSatsUsdPattern2(client, acc) {
  return {
    btc: createMetricPattern1(client, acc),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    relToOwn: createBpsPercentRatioPattern4(client, _m(acc, 'rel_to_own')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} CoindaysCoinyearsDormancySentVelocityPattern
 * @property {BaseCumulativeSumPattern<StoredF64>} coindaysDestroyed
 * @property {MetricPattern1<StoredF64>} coinyearsDestroyed
 * @property {MetricPattern1<StoredF32>} dormancy
 * @property {BaseCumulativeInSumPattern} sent
 * @property {MetricPattern1<StoredF32>} velocity
 */

/**
 * Create a CoindaysCoinyearsDormancySentVelocityPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoindaysCoinyearsDormancySentVelocityPattern}
 */
function createCoindaysCoinyearsDormancySentVelocityPattern(client, acc) {
  return {
    coindaysDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'coindays_destroyed')),
    coinyearsDestroyed: createMetricPattern1(client, _m(acc, 'coinyears_destroyed')),
    dormancy: createMetricPattern1(client, _m(acc, 'dormancy')),
    sent: createBaseCumulativeInSumPattern(client, _m(acc, 'sent')),
    velocity: createMetricPattern1(client, _m(acc, 'velocity')),
  };
}

/**
 * @typedef {Object} DeltaHalfInTotalPattern2
 * @property {ChangeRatePattern2} delta
 * @property {BtcCentsSatsUsdPattern} half
 * @property {BtcCentsSatsUsdPattern} inLoss
 * @property {BtcCentsSatsUsdPattern} inProfit
 * @property {BtcCentsSatsUsdPattern} total
 */

/**
 * Create a DeltaHalfInTotalPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DeltaHalfInTotalPattern2}
 */
function createDeltaHalfInTotalPattern2(client, acc) {
  return {
    delta: createChangeRatePattern2(client, _m(acc, 'delta')),
    half: createBtcCentsSatsUsdPattern(client, _m(acc, 'half')),
    inLoss: createBtcCentsSatsUsdPattern(client, _m(acc, 'in_loss')),
    inProfit: createBtcCentsSatsUsdPattern(client, _m(acc, 'in_profit')),
    total: createBtcCentsSatsUsdPattern(client, acc),
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
 * @typedef {Object} InvestedMaxMinPercentilesSupplyPattern
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} investedCapital
 * @property {CentsSatsUsdPattern} max
 * @property {CentsSatsUsdPattern} min
 * @property {Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern} percentiles
 * @property {BpsPercentRatioPattern4} supplyDensity
 */

/**
 * Create a InvestedMaxMinPercentilesSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InvestedMaxMinPercentilesSupplyPattern}
 */
function createInvestedMaxMinPercentilesSupplyPattern(client, acc) {
  return {
    investedCapital: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital')),
    max: createCentsSatsUsdPattern(client, _m(acc, 'cost_basis_max')),
    min: createCentsSatsUsdPattern(client, _m(acc, 'cost_basis_min')),
    percentiles: createPct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis')),
    supplyDensity: createBpsPercentRatioPattern4(client, _m(acc, 'supply_density')),
  };
}

/**
 * @typedef {Object} PhsReboundThsPattern
 * @property {MetricPattern1<StoredF32>} phs
 * @property {MetricPattern1<StoredF32>} phsMin
 * @property {BpsPercentRatioPattern2} rebound
 * @property {MetricPattern1<StoredF32>} ths
 * @property {MetricPattern1<StoredF32>} thsMin
 */

/**
 * Create a PhsReboundThsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PhsReboundThsPattern}
 */
function createPhsReboundThsPattern(client, acc) {
  return {
    phs: createMetricPattern1(client, _m(acc, 'phs')),
    phsMin: createMetricPattern1(client, _m(acc, 'phs_min')),
    rebound: createBpsPercentRatioPattern2(client, _m(acc, 'rebound')),
    ths: createMetricPattern1(client, _m(acc, 'ths')),
    thsMin: createMetricPattern1(client, _m(acc, 'ths_min')),
  };
}

/**
 * @template T
 * @typedef {Object} _1m1w1y24hHeightPattern
 * @property {MetricPattern1<T>} _1m
 * @property {MetricPattern1<T>} _1w
 * @property {MetricPattern1<T>} _1y
 * @property {MetricPattern1<T>} _24h
 * @property {MetricPattern18<T>} height
 */

/**
 * Create a _1m1w1y24hHeightPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hHeightPattern<T>}
 */
function create_1m1w1y24hHeightPattern(client, acc) {
  return {
    _1m: createMetricPattern1(client, _m(acc, 'average_1m')),
    _1w: createMetricPattern1(client, _m(acc, 'average_1w')),
    _1y: createMetricPattern1(client, _m(acc, 'average_1y')),
    _24h: createMetricPattern1(client, _m(acc, 'average_24h')),
    height: createMetricPattern18(client, acc),
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
    _1m: createBpsPercentRatioPattern(client, _m(acc, '1m_rate')),
    _1w: createBpsPercentRatioPattern(client, _m(acc, '1w_rate')),
    _1y: createBpsPercentRatioPattern(client, _m(acc, '1y_rate')),
    _24h: createBpsPercentRatioPattern(client, _m(acc, '24h_rate')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern3
 * @property {BpsPercentRatioPattern2} _1m
 * @property {BpsPercentRatioPattern2} _1w
 * @property {BpsPercentRatioPattern2} _1y
 * @property {BpsPercentRatioPattern2} _24h
 */

/**
 * Create a _1m1w1y24hPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern3}
 */
function create_1m1w1y24hPattern3(client, acc) {
  return {
    _1m: createBpsPercentRatioPattern2(client, _m(acc, '1m_rate')),
    _1w: createBpsPercentRatioPattern2(client, _m(acc, '1w_rate')),
    _1y: createBpsPercentRatioPattern2(client, _m(acc, '1y_rate')),
    _24h: createBpsPercentRatioPattern2(client, _m(acc, '24h_rate')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern6
 * @property {BtcCentsSatsUsdPattern} _1m
 * @property {BtcCentsSatsUsdPattern} _1w
 * @property {BtcCentsSatsUsdPattern} _1y
 * @property {BtcCentsSatsUsdPattern} _24h
 */

/**
 * Create a _1m1w1y24hPattern6 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern6}
 */
function create_1m1w1y24hPattern6(client, acc) {
  return {
    _1m: createBtcCentsSatsUsdPattern(client, _m(acc, '1m')),
    _1w: createBtcCentsSatsUsdPattern(client, _m(acc, '1w')),
    _1y: createBtcCentsSatsUsdPattern(client, _m(acc, '1y')),
    _24h: createBtcCentsSatsUsdPattern(client, _m(acc, '24h')),
  };
}

/**
 * @typedef {Object} _1m1w1y2wPattern
 * @property {CentsSatsUsdPattern} _1m
 * @property {CentsSatsUsdPattern} _1w
 * @property {CentsSatsUsdPattern} _1y
 * @property {CentsSatsUsdPattern} _2w
 */

/**
 * Create a _1m1w1y2wPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y2wPattern}
 */
function create_1m1w1y2wPattern(client, acc) {
  return {
    _1m: createCentsSatsUsdPattern(client, _m(acc, '1m')),
    _1w: createCentsSatsUsdPattern(client, _m(acc, '1w')),
    _1y: createCentsSatsUsdPattern(client, _m(acc, '1y')),
    _2w: createCentsSatsUsdPattern(client, _m(acc, '2w')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern4
 * @property {CentsUsdPattern} _1m
 * @property {CentsUsdPattern} _1w
 * @property {CentsUsdPattern} _1y
 * @property {CentsUsdPattern} _24h
 */

/**
 * Create a _1m1w1y24hPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern4}
 */
function create_1m1w1y24hPattern4(client, acc) {
  return {
    _1m: createCentsUsdPattern(client, _m(acc, '1m_change')),
    _1w: createCentsUsdPattern(client, _m(acc, '1w_change')),
    _1y: createCentsUsdPattern(client, _m(acc, '1y_change')),
    _24h: createCentsUsdPattern(client, _m(acc, '24h_change')),
  };
}

/**
 * @typedef {Object} _1m1w1y24hPattern5
 * @property {CentsUsdPattern2} _1m
 * @property {CentsUsdPattern2} _1w
 * @property {CentsUsdPattern2} _1y
 * @property {CentsUsdPattern2} _24h
 */

/**
 * Create a _1m1w1y24hPattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1m1w1y24hPattern5}
 */
function create_1m1w1y24hPattern5(client, acc) {
  return {
    _1m: createCentsUsdPattern2(client, _m(acc, '1m')),
    _1w: createCentsUsdPattern2(client, _m(acc, '1w')),
    _1y: createCentsUsdPattern2(client, _m(acc, '1y')),
    _24h: createCentsUsdPattern2(client, _m(acc, '24h')),
  };
}

/**
 * @typedef {Object} _1y2y4yAllPattern
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} _1y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} _2y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} _4y
 * @property {_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern} all
 */

/**
 * Create a _1y2y4yAllPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1y2y4yAllPattern}
 */
function create_1y2y4yAllPattern(client, acc) {
  return {
    _1y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc),
    _2y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc),
    _4y: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc),
    all: create_0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc),
  };
}

/**
 * @typedef {Object} AdjustedRatioValuePattern
 * @property {RatioValuePattern2} adjusted
 * @property {_1m1w1y24hPattern<StoredF64>} ratio
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a AdjustedRatioValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {AdjustedRatioValuePattern}
 */
function createAdjustedRatioValuePattern(client, acc) {
  return {
    adjusted: createRatioValuePattern2(client, acc),
    ratio: create_1m1w1y24hPattern(client, _m(acc, 'sopr')),
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @typedef {Object} BaseCumulativeDeltaSumPattern
 * @property {CentsUsdPattern} base
 * @property {CentsUsdPattern} cumulative
 * @property {ChangeRatePattern3} delta
 * @property {_1m1w1y24hPattern4} sum
 */

/**
 * Create a BaseCumulativeDeltaSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeDeltaSumPattern}
 */
function createBaseCumulativeDeltaSumPattern(client, acc) {
  return {
    base: createCentsUsdPattern(client, acc),
    cumulative: createCentsUsdPattern(client, _m(acc, 'cumulative')),
    delta: createChangeRatePattern3(client, _m(acc, 'delta')),
    sum: create_1m1w1y24hPattern4(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BaseCumulativeNegativeSumPattern
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {MetricPattern1<Dollars>} negative
 * @property {_1m1w1y24hPattern5} sum
 */

/**
 * Create a BaseCumulativeNegativeSumPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeNegativeSumPattern}
 */
function createBaseCumulativeNegativeSumPattern(client, acc) {
  return {
    base: createCentsUsdPattern2(client, _m(acc, 'unrealized_loss')),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'unrealized_loss_cumulative')),
    negative: createMetricPattern1(client, _m(acc, 'neg_unrealized_loss')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'unrealized_loss_sum')),
  };
}

/**
 * @typedef {Object} BothReactivatedReceivingSendingPattern
 * @property {_1m1w1y24hHeightPattern<StoredU32>} both
 * @property {_1m1w1y24hHeightPattern<StoredU32>} reactivated
 * @property {_1m1w1y24hHeightPattern<StoredU32>} receiving
 * @property {_1m1w1y24hHeightPattern<StoredU32>} sending
 */

/**
 * Create a BothReactivatedReceivingSendingPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BothReactivatedReceivingSendingPattern}
 */
function createBothReactivatedReceivingSendingPattern(client, acc) {
  return {
    both: create_1m1w1y24hHeightPattern(client, _m(acc, 'both')),
    reactivated: create_1m1w1y24hHeightPattern(client, _m(acc, 'reactivated')),
    receiving: create_1m1w1y24hHeightPattern(client, _m(acc, 'receiving')),
    sending: create_1m1w1y24hHeightPattern(client, _m(acc, 'sending')),
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
    btc: createMetricPattern1(client, acc),
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    sats: createMetricPattern1(client, _m(acc, 'sats')),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} CentsDeltaRelUsdPattern
 * @property {MetricPattern1<Cents>} cents
 * @property {ChangeRatePattern3} delta
 * @property {BpsPercentRatioPattern} relToOwnMcap
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsDeltaRelUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsDeltaRelUsdPattern}
 */
function createCentsDeltaRelUsdPattern(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    delta: createChangeRatePattern3(client, _m(acc, 'delta')),
    relToOwnMcap: createBpsPercentRatioPattern(client, _m(acc, 'rel_to_own_mcap')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} CentsRelUsdPattern2
 * @property {MetricPattern1<CentsSigned>} cents
 * @property {BpsPercentRatioPattern2} relToOwnGross
 * @property {BpsPercentRatioPattern2} relToOwnMcap
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsRelUsdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsRelUsdPattern2}
 */
function createCentsRelUsdPattern2(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    relToOwnGross: createBpsPercentRatioPattern2(client, _m(acc, 'rel_to_own_gross_pnl')),
    relToOwnMcap: createBpsPercentRatioPattern2(client, _m(acc, 'rel_to_own_mcap')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} LossNetNuplProfitPattern
 * @property {BaseCumulativeNegativeSumPattern} loss
 * @property {CentsUsdPattern} netPnl
 * @property {BpsRatioPattern} nupl
 * @property {BaseCumulativeSumPattern3} profit
 */

/**
 * Create a LossNetNuplProfitPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {LossNetNuplProfitPattern}
 */
function createLossNetNuplProfitPattern(client, acc) {
  return {
    loss: createBaseCumulativeNegativeSumPattern(client, acc),
    netPnl: createCentsUsdPattern(client, _m(acc, 'net_unrealized_pnl')),
    nupl: createBpsRatioPattern(client, _m(acc, 'nupl')),
    profit: createBaseCumulativeSumPattern3(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} OutputsRealizedSupplyUnrealizedPattern2
 * @property {UnspentPattern} outputs
 * @property {CapLossMvrvPriceProfitSoprPattern} realized
 * @property {DeltaHalfInTotalPattern2} supply
 * @property {LossNuplProfitPattern} unrealized
 */

/**
 * Create a OutputsRealizedSupplyUnrealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {OutputsRealizedSupplyUnrealizedPattern2}
 */
function createOutputsRealizedSupplyUnrealizedPattern2(client, acc) {
  return {
    outputs: createUnspentPattern(client, _m(acc, 'utxo_count')),
    realized: createCapLossMvrvPriceProfitSoprPattern(client, acc),
    supply: createDeltaHalfInTotalPattern2(client, _m(acc, 'supply')),
    unrealized: createLossNuplProfitPattern(client, acc),
  };
}

/**
 * @typedef {Object} OutputsRealizedSupplyUnrealizedPattern
 * @property {UnspentPattern} outputs
 * @property {CapLossMvrvPriceProfitSoprPattern} realized
 * @property {DeltaHalfTotalPattern} supply
 * @property {NuplPattern} unrealized
 */

/**
 * Create a OutputsRealizedSupplyUnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {OutputsRealizedSupplyUnrealizedPattern}
 */
function createOutputsRealizedSupplyUnrealizedPattern(client, acc) {
  return {
    outputs: createUnspentPattern(client, _m(acc, 'utxo_count')),
    realized: createCapLossMvrvPriceProfitSoprPattern(client, acc),
    supply: createDeltaHalfTotalPattern(client, _m(acc, 'supply')),
    unrealized: createNuplPattern(client, _m(acc, 'nupl')),
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
 * @typedef {Object} BaseCumulativeSumPattern4
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 * @property {_1m1w1y24hPattern6} sum
 */

/**
 * Create a BaseCumulativeSumPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern4}
 */
function createBaseCumulativeSumPattern4(client, acc) {
  return {
    base: createBtcCentsSatsUsdPattern(client, acc),
    cumulative: createBtcCentsSatsUsdPattern(client, _m(acc, 'cumulative')),
    sum: create_1m1w1y24hPattern6(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BaseCumulativeRelPattern
 * @property {MetricPattern1<Cents>} base
 * @property {MetricPattern1<Cents>} cumulative
 * @property {BpsPercentRatioPattern} relToRcap
 */

/**
 * Create a BaseCumulativeRelPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeRelPattern}
 */
function createBaseCumulativeRelPattern(client, acc) {
  return {
    base: createMetricPattern1(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    relToRcap: createBpsPercentRatioPattern(client, _m(acc, 'rel_to_rcap')),
  };
}

/**
 * @typedef {Object} BaseCumulativeSumPattern3
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {_1m1w1y24hPattern5} sum
 */

/**
 * Create a BaseCumulativeSumPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern3}
 */
function createBaseCumulativeSumPattern3(client, acc) {
  return {
    base: createCentsUsdPattern2(client, acc),
    cumulative: createCentsUsdPattern2(client, _m(acc, 'cumulative')),
    sum: create_1m1w1y24hPattern5(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BaseCumulativeSumPattern2
 * @property {MetricPattern1<StoredU32>} base
 * @property {MetricPattern1<StoredU64>} cumulative
 * @property {_1m1w1y24hPattern<StoredU64>} sum
 */

/**
 * Create a BaseCumulativeSumPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern2}
 */
function createBaseCumulativeSumPattern2(client, acc) {
  return {
    base: createMetricPattern1(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BlocksDominanceRewardsPattern
 * @property {BaseCumulativeSumPattern2} blocksMined
 * @property {_1m1w1y24hBpsPercentRatioPattern} dominance
 * @property {BaseCumulativeSumPattern4} rewards
 */

/**
 * Create a BlocksDominanceRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlocksDominanceRewardsPattern}
 */
function createBlocksDominanceRewardsPattern(client, acc) {
  return {
    blocksMined: createBaseCumulativeSumPattern2(client, _m(acc, 'blocks_mined')),
    dominance: create_1m1w1y24hBpsPercentRatioPattern(client, _m(acc, 'dominance')),
    rewards: createBaseCumulativeSumPattern4(client, _m(acc, 'rewards')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern4
 * @property {MetricPattern1<BasisPoints16>} bps
 * @property {MetricPattern1<StoredF32>} percent
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsPercentRatioPattern4 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsPercentRatioPattern4}
 */
function createBpsPercentRatioPattern4(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern
 * @property {MetricPattern1<BasisPoints32>} bps
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
    bps: createMetricPattern1(client, _m(acc, 'ratio_pct99_bps')),
    price: createCentsSatsUsdPattern(client, _m(acc, 'pct99')),
    ratio: createMetricPattern1(client, _m(acc, 'ratio_pct99')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern5
 * @property {MetricPattern1<BasisPointsSigned16>} bps
 * @property {MetricPattern1<StoredF32>} percent
 * @property {MetricPattern1<StoredF32>} ratio
 */

/**
 * Create a BpsPercentRatioPattern5 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BpsPercentRatioPattern5}
 */
function createBpsPercentRatioPattern5(client, acc) {
  return {
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} BpsPercentRatioPattern2
 * @property {MetricPattern1<BasisPointsSigned32>} bps
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
    bps: createMetricPattern1(client, _m(acc, 'bps')),
    percent: createMetricPattern1(client, acc),
    ratio: createMetricPattern1(client, _m(acc, 'ratio')),
  };
}

/**
 * @typedef {Object} CentsSatsUsdPattern3
 * @property {MetricPattern2<Cents>} cents
 * @property {MetricPattern2<Sats>} sats
 * @property {MetricPattern2<Dollars>} usd
 */

/**
 * Create a CentsSatsUsdPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsSatsUsdPattern3}
 */
function createCentsSatsUsdPattern3(client, acc) {
  return {
    cents: createMetricPattern2(client, _m(acc, 'cents')),
    sats: createMetricPattern2(client, _m(acc, 'sats')),
    usd: createMetricPattern2(client, acc),
  };
}

/**
 * @typedef {Object} CentsDeltaUsdPattern
 * @property {MetricPattern1<Cents>} cents
 * @property {ChangeRatePattern3} delta
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsDeltaUsdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsDeltaUsdPattern}
 */
function createCentsDeltaUsdPattern(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    delta: createChangeRatePattern3(client, _m(acc, 'delta')),
    usd: createMetricPattern1(client, acc),
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
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} DeltaHalfTotalPattern
 * @property {ChangeRatePattern2} delta
 * @property {BtcCentsSatsUsdPattern} half
 * @property {BtcCentsSatsUsdPattern} total
 */

/**
 * Create a DeltaHalfTotalPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DeltaHalfTotalPattern}
 */
function createDeltaHalfTotalPattern(client, acc) {
  return {
    delta: createChangeRatePattern2(client, _m(acc, 'delta')),
    half: createBtcCentsSatsUsdPattern(client, _m(acc, 'half')),
    total: createBtcCentsSatsUsdPattern(client, acc),
  };
}

/**
 * @typedef {Object} GreedNetPainPattern
 * @property {CentsUsdPattern2} greedIndex
 * @property {CentsUsdPattern} net
 * @property {CentsUsdPattern2} painIndex
 */

/**
 * Create a GreedNetPainPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {GreedNetPainPattern}
 */
function createGreedNetPainPattern(client, acc) {
  return {
    greedIndex: createCentsUsdPattern2(client, _m(acc, 'greed_index')),
    net: createCentsUsdPattern(client, _m(acc, 'net_sentiment')),
    painIndex: createCentsUsdPattern2(client, _m(acc, 'pain_index')),
  };
}

/**
 * @typedef {Object} LossNuplProfitPattern
 * @property {BaseCumulativeSumPattern3} loss
 * @property {BpsRatioPattern} nupl
 * @property {BaseCumulativeSumPattern3} profit
 */

/**
 * Create a LossNuplProfitPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {LossNuplProfitPattern}
 */
function createLossNuplProfitPattern(client, acc) {
  return {
    loss: createBaseCumulativeSumPattern3(client, _m(acc, 'unrealized_loss')),
    nupl: createBpsRatioPattern(client, _m(acc, 'nupl')),
    profit: createBaseCumulativeSumPattern3(client, _m(acc, 'unrealized_profit')),
  };
}

/**
 * @typedef {Object} LowerPriceUpperPattern
 * @property {CentsSatsUsdPattern} lowerPriceBand
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} price
 * @property {CentsSatsUsdPattern} upperPriceBand
 */

/**
 * Create a LowerPriceUpperPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {LowerPriceUpperPattern}
 */
function createLowerPriceUpperPattern(client, acc) {
  return {
    lowerPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'lower_price_band')),
    price: createBpsCentsPercentilesRatioSatsUsdPattern(client, _m(acc, 'investor_price')),
    upperPriceBand: createCentsSatsUsdPattern(client, _m(acc, 'upper_price_band')),
  };
}

/**
 * @typedef {Object} RatioValuePattern2
 * @property {_1m1w1y24hPattern<StoredF64>} ratio
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a RatioValuePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioValuePattern2}
 */
function createRatioValuePattern2(client, acc) {
  return {
    ratio: create_1m1w1y24hPattern(client, _m(acc, 'asopr')),
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'adj_value_created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'adj_value_destroyed')),
  };
}

/**
 * @typedef {Object} RatioValuePattern
 * @property {_24hPattern} ratio
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a RatioValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RatioValuePattern}
 */
function createRatioValuePattern(client, acc) {
  return {
    ratio: create_24hPattern(client, _m(acc, 'sopr_24h')),
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'value_created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'value_destroyed')),
  };
}

/**
 * @template T
 * @typedef {Object} _6bBlockTxindexPattern
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2<T>} _6b
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2<T>} block
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
    _6b: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(client, _m(acc, '6b')),
    block: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(client, acc),
    txindex: createMetricPattern19(client, acc),
  };
}

/**
 * @template T
 * @typedef {Object} BaseCumulativeSumPattern
 * @property {MetricPattern1<T>} base
 * @property {MetricPattern1<T>} cumulative
 * @property {_1m1w1y24hPattern<T>} sum
 */

/**
 * Create a BaseCumulativeSumPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BaseCumulativeSumPattern<T>}
 */
function createBaseCumulativeSumPattern(client, acc) {
  return {
    base: createMetricPattern1(client, acc),
    cumulative: createMetricPattern1(client, _m(acc, 'cumulative')),
    sum: create_1m1w1y24hPattern(client, _m(acc, 'sum')),
  };
}

/**
 * @typedef {Object} BlocksDominancePattern
 * @property {BaseCumulativeSumPattern2} blocksMined
 * @property {BpsPercentRatioPattern4} dominance
 */

/**
 * Create a BlocksDominancePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlocksDominancePattern}
 */
function createBlocksDominancePattern(client, acc) {
  return {
    blocksMined: createBaseCumulativeSumPattern2(client, _m(acc, 'blocks_mined')),
    dominance: createBpsPercentRatioPattern4(client, _m(acc, 'dominance')),
  };
}

/**
 * @typedef {Object} BpsRatioPattern2
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<StoredF32>} ratio
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
  };
}

/**
 * @typedef {Object} BpsRatioPattern
 * @property {MetricPattern1<BasisPointsSigned32>} bps
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
 * @typedef {Object} CentsUsdPattern2
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 */

/**
 * Create a CentsUsdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CentsUsdPattern2}
 */
function createCentsUsdPattern2(client, acc) {
  return {
    cents: createMetricPattern1(client, _m(acc, 'cents')),
    usd: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} CentsUsdPattern
 * @property {MetricPattern1<CentsSigned>} cents
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
    cents: createMetricPattern1(client, acc),
    usd: createMetricPattern1(client, _m(acc, 'usd')),
  };
}

/**
 * @typedef {Object} ChangeRatePattern
 * @property {_1m1w1y24hPattern<StoredI64>} change
 * @property {_1m1w1y24hPattern2} rate
 */

/**
 * Create a ChangeRatePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ChangeRatePattern}
 */
function createChangeRatePattern(client, acc) {
  return {
    change: create_1m1w1y24hPattern(client, acc),
    rate: create_1m1w1y24hPattern2(client, acc),
  };
}

/**
 * @typedef {Object} ChangeRatePattern2
 * @property {_1m1w1y24hPattern<StoredI64>} change
 * @property {_1m1w1y24hPattern3} rate
 */

/**
 * Create a ChangeRatePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ChangeRatePattern2}
 */
function createChangeRatePattern2(client, acc) {
  return {
    change: create_1m1w1y24hPattern(client, acc),
    rate: create_1m1w1y24hPattern3(client, acc),
  };
}

/**
 * @typedef {Object} ChangeRatePattern3
 * @property {_1m1w1y24hPattern4} change
 * @property {_1m1w1y24hPattern3} rate
 */

/**
 * Create a ChangeRatePattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ChangeRatePattern3}
 */
function createChangeRatePattern3(client, acc) {
  return {
    change: create_1m1w1y24hPattern4(client, acc),
    rate: create_1m1w1y24hPattern3(client, acc),
  };
}

/**
 * @typedef {Object} CoindaysSentPattern
 * @property {BaseCumulativeSumPattern<StoredF64>} coindaysDestroyed
 * @property {BaseCumulativeInSumPattern} sent
 */

/**
 * Create a CoindaysSentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {CoindaysSentPattern}
 */
function createCoindaysSentPattern(client, acc) {
  return {
    coindaysDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'coindays_destroyed')),
    sent: createBaseCumulativeInSumPattern(client, _m(acc, 'sent')),
  };
}

/**
 * @typedef {Object} DeltaInnerPattern
 * @property {ChangeRatePattern2} delta
 * @property {MetricPattern1<StoredU64>} inner
 */

/**
 * Create a DeltaInnerPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {DeltaInnerPattern}
 */
function createDeltaInnerPattern(client, acc) {
  return {
    delta: createChangeRatePattern2(client, _m(acc, 'delta')),
    inner: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} InPattern
 * @property {CentsUsdPattern2} inLoss
 * @property {CentsUsdPattern2} inProfit
 */

/**
 * Create a InPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {InPattern}
 */
function createInPattern(client, acc) {
  return {
    inLoss: createCentsUsdPattern2(client, _m(acc, 'loss')),
    inProfit: createCentsUsdPattern2(client, _m(acc, 'profit')),
  };
}

/**
 * @typedef {Object} PriceValuePattern
 * @property {CentsSatsUsdPattern} price
 * @property {MetricPattern1<StoredF32>} value
 */

/**
 * Create a PriceValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {PriceValuePattern}
 */
function createPriceValuePattern(client, acc) {
  return {
    price: createCentsSatsUsdPattern(client, _m(acc, 'p3sd_4y')),
    value: createMetricPattern1(client, _m(acc, 'ratio_p3sd_4y')),
  };
}

/**
 * @typedef {Object} RealizedSupplyPattern
 * @property {MetricPattern1<Dollars>} realizedCap
 * @property {MetricPattern1<Sats>} supply
 */

/**
 * Create a RealizedSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RealizedSupplyPattern}
 */
function createRealizedSupplyPattern(client, acc) {
  return {
    realizedCap: createMetricPattern1(client, _m(acc, 'realized_cap')),
    supply: createMetricPattern1(client, _m(acc, 'supply')),
  };
}

/**
 * @typedef {Object} RelPattern
 * @property {BpsPercentRatioPattern2} relToMcap
 * @property {BpsPercentRatioPattern2} relToRcap
 */

/**
 * Create a RelPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {RelPattern}
 */
function createRelPattern(client, acc) {
  return {
    relToMcap: createBpsPercentRatioPattern2(client, _m(acc, 'mcap')),
    relToRcap: createBpsPercentRatioPattern2(client, _m(acc, 'rcap')),
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
 * @typedef {Object} ValuePattern
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * Create a ValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {ValuePattern}
 */
function createValuePattern(client, acc) {
  return {
    valueCreated: createBaseCumulativeSumPattern(client, _m(acc, 'created')),
    valueDestroyed: createBaseCumulativeSumPattern(client, _m(acc, 'destroyed')),
  };
}

/**
 * @typedef {Object} _24hPattern
 * @property {MetricPattern1<StoredF64>} _24h
 */

/**
 * Create a _24hPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_24hPattern}
 */
function create_24hPattern(client, acc) {
  return {
    _24h: createMetricPattern1(client, acc),
  };
}

/**
 * @typedef {Object} NuplPattern
 * @property {BpsRatioPattern} nupl
 */

/**
 * Create a NuplPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {NuplPattern}
 */
function createNuplPattern(client, acc) {
  return {
    nupl: createBpsRatioPattern(client, acc),
  };
}

/**
 * @typedef {Object} UnspentPattern
 * @property {DeltaInnerPattern} unspentCount
 */

/**
 * Create a UnspentPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {UnspentPattern}
 */
function createUnspentPattern(client, acc) {
  return {
    unspentCount: createDeltaInnerPattern(client, acc),
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
 * @property {MetricsTree_Cointime} cointime
 * @property {MetricsTree_Constants} constants
 * @property {MetricsTree_Indexes} indexes
 * @property {MetricsTree_Indicators} indicators
 * @property {MetricsTree_Market} market
 * @property {MetricsTree_Pools} pools
 * @property {MetricsTree_Prices} prices
 * @property {MetricsTree_Supply} supply
 * @property {MetricsTree_Cohorts} cohorts
 */

/**
 * @typedef {Object} MetricsTree_Blocks
 * @property {MetricPattern18<BlockHash>} blockhash
 * @property {MetricsTree_Blocks_Difficulty} difficulty
 * @property {MetricsTree_Blocks_Time} time
 * @property {MetricsTree_Blocks_Size} size
 * @property {MetricsTree_Blocks_Weight} weight
 * @property {MetricsTree_Blocks_Count} count
 * @property {MetricsTree_Blocks_Lookback} lookback
 * @property {_1m1w1y24hHeightPattern<Timestamp>} interval
 * @property {MetricsTree_Blocks_Halving} halving
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} vbytes
 * @property {MetricsTree_Blocks_Fullness} fullness
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Difficulty
 * @property {MetricPattern18<StoredF64>} raw
 * @property {MetricPattern2<StoredF64>} base
 * @property {MetricPattern1<StoredF64>} asHash
 * @property {BpsPercentRatioPattern2} adjustment
 * @property {MetricPattern1<Epoch>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNext
 * @property {MetricPattern1<StoredF32>} daysBeforeNext
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Time
 * @property {MetricPattern1<Timestamp>} timestamp
 * @property {MetricPattern18<Date>} date
 * @property {MetricPattern18<Timestamp>} timestampMonotonic
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Size
 * @property {MetricPattern18<StoredU64>} total
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
 * @typedef {Object} MetricsTree_Blocks_Weight
 * @property {MetricPattern18<Weight>} raw
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
 * @property {MetricPattern1<StoredU64>} target
 * @property {BaseCumulativeSumPattern2} total
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Lookback
 * @property {MetricPattern18<Height>} _1h
 * @property {MetricPattern18<Height>} _24h
 * @property {MetricPattern18<Height>} _3d
 * @property {MetricPattern18<Height>} _1w
 * @property {MetricPattern18<Height>} _8d
 * @property {MetricPattern18<Height>} _9d
 * @property {MetricPattern18<Height>} _12d
 * @property {MetricPattern18<Height>} _13d
 * @property {MetricPattern18<Height>} _2w
 * @property {MetricPattern18<Height>} _21d
 * @property {MetricPattern18<Height>} _26d
 * @property {MetricPattern18<Height>} _1m
 * @property {MetricPattern18<Height>} _34d
 * @property {MetricPattern18<Height>} _55d
 * @property {MetricPattern18<Height>} _2m
 * @property {MetricPattern18<Height>} _9w
 * @property {MetricPattern18<Height>} _12w
 * @property {MetricPattern18<Height>} _89d
 * @property {MetricPattern18<Height>} _3m
 * @property {MetricPattern18<Height>} _14w
 * @property {MetricPattern18<Height>} _111d
 * @property {MetricPattern18<Height>} _144d
 * @property {MetricPattern18<Height>} _6m
 * @property {MetricPattern18<Height>} _26w
 * @property {MetricPattern18<Height>} _200d
 * @property {MetricPattern18<Height>} _9m
 * @property {MetricPattern18<Height>} _350d
 * @property {MetricPattern18<Height>} _12m
 * @property {MetricPattern18<Height>} _1y
 * @property {MetricPattern18<Height>} _14m
 * @property {MetricPattern18<Height>} _2y
 * @property {MetricPattern18<Height>} _26m
 * @property {MetricPattern18<Height>} _3y
 * @property {MetricPattern18<Height>} _200w
 * @property {MetricPattern18<Height>} _4y
 * @property {MetricPattern18<Height>} _5y
 * @property {MetricPattern18<Height>} _6y
 * @property {MetricPattern18<Height>} _8y
 * @property {MetricPattern18<Height>} _9y
 * @property {MetricPattern18<Height>} _10y
 * @property {MetricPattern18<Height>} _12y
 * @property {MetricPattern18<Height>} _14y
 * @property {MetricPattern18<Height>} _26y
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Halving
 * @property {MetricPattern1<Halving>} epoch
 * @property {MetricPattern1<StoredU32>} blocksBeforeNext
 * @property {MetricPattern1<StoredF32>} daysBeforeNext
 */

/**
 * @typedef {Object} MetricsTree_Blocks_Fullness
 * @property {_1m1w1y24hHeightPattern<BasisPoints16>} bps
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricPattern1<StoredF32>} percent
 */

/**
 * @typedef {Object} MetricsTree_Transactions
 * @property {MetricsTree_Transactions_Raw} raw
 * @property {MetricsTree_Transactions_Count} count
 * @property {MetricsTree_Transactions_Size} size
 * @property {MetricsTree_Transactions_Fees} fees
 * @property {MetricsTree_Transactions_Versions} versions
 * @property {MetricsTree_Transactions_Volume} volume
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Raw
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
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Count
 * @property {AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern} total
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
 * @property {BaseCumulativeSumPattern<StoredU64>} v1
 * @property {BaseCumulativeSumPattern<StoredU64>} v2
 * @property {BaseCumulativeSumPattern<StoredU64>} v3
 */

/**
 * @typedef {Object} MetricsTree_Transactions_Volume
 * @property {BaseCumulativeSumPattern4} sentSum
 * @property {BaseCumulativeSumPattern4} receivedSum
 * @property {MetricPattern1<StoredF32>} txPerSec
 * @property {MetricPattern1<StoredF32>} outputsPerSec
 * @property {MetricPattern1<StoredF32>} inputsPerSec
 */

/**
 * @typedef {Object} MetricsTree_Inputs
 * @property {MetricsTree_Inputs_Raw} raw
 * @property {MetricsTree_Inputs_Spent} spent
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} count
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Raw
 * @property {MetricPattern18<TxInIndex>} firstTxinindex
 * @property {MetricPattern20<OutPoint>} outpoint
 * @property {MetricPattern20<TxIndex>} txindex
 * @property {MetricPattern20<OutputType>} outputtype
 * @property {MetricPattern20<TypeIndex>} typeindex
 */

/**
 * @typedef {Object} MetricsTree_Inputs_Spent
 * @property {MetricPattern20<TxOutIndex>} txoutindex
 * @property {MetricPattern20<Sats>} value
 */

/**
 * @typedef {Object} MetricsTree_Outputs
 * @property {MetricsTree_Outputs_Raw} raw
 * @property {MetricsTree_Outputs_Spent} spent
 * @property {MetricsTree_Outputs_Count} count
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Raw
 * @property {MetricPattern18<TxOutIndex>} firstTxoutindex
 * @property {MetricPattern21<Sats>} value
 * @property {MetricPattern21<OutputType>} outputtype
 * @property {MetricPattern21<TypeIndex>} typeindex
 * @property {MetricPattern21<TxIndex>} txindex
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Spent
 * @property {MetricPattern21<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} MetricsTree_Outputs_Count
 * @property {AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern} total
 * @property {MetricPattern1<StoredU64>} unspent
 */

/**
 * @typedef {Object} MetricsTree_Addresses
 * @property {MetricsTree_Addresses_Raw} raw
 * @property {MetricsTree_Addresses_Indexes} indexes
 * @property {MetricsTree_Addresses_Data} data
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3} funded
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3} empty
 * @property {MetricsTree_Addresses_Activity} activity
 * @property {AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3} total
 * @property {MetricsTree_Addresses_New} new
 * @property {MetricsTree_Addresses_Delta} delta
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw
 * @property {MetricsTree_Addresses_Raw_P2pk65} p2pk65
 * @property {MetricsTree_Addresses_Raw_P2pk33} p2pk33
 * @property {MetricsTree_Addresses_Raw_P2pkh} p2pkh
 * @property {MetricsTree_Addresses_Raw_P2sh} p2sh
 * @property {MetricsTree_Addresses_Raw_P2wpkh} p2wpkh
 * @property {MetricsTree_Addresses_Raw_P2wsh} p2wsh
 * @property {MetricsTree_Addresses_Raw_P2tr} p2tr
 * @property {MetricsTree_Addresses_Raw_P2a} p2a
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2pk65
 * @property {MetricPattern18<P2PK65AddressIndex>} firstIndex
 * @property {MetricPattern27<P2PK65Bytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2pk33
 * @property {MetricPattern18<P2PK33AddressIndex>} firstIndex
 * @property {MetricPattern26<P2PK33Bytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2pkh
 * @property {MetricPattern18<P2PKHAddressIndex>} firstIndex
 * @property {MetricPattern28<P2PKHBytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2sh
 * @property {MetricPattern18<P2SHAddressIndex>} firstIndex
 * @property {MetricPattern29<P2SHBytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2wpkh
 * @property {MetricPattern18<P2WPKHAddressIndex>} firstIndex
 * @property {MetricPattern31<P2WPKHBytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2wsh
 * @property {MetricPattern18<P2WSHAddressIndex>} firstIndex
 * @property {MetricPattern32<P2WSHBytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2tr
 * @property {MetricPattern18<P2TRAddressIndex>} firstIndex
 * @property {MetricPattern30<P2TRBytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Raw_P2a
 * @property {MetricPattern18<P2AAddressIndex>} firstIndex
 * @property {MetricPattern24<P2ABytes>} bytes
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Indexes
 * @property {MetricPattern24<AnyAddressIndex>} p2a
 * @property {MetricPattern26<AnyAddressIndex>} p2pk33
 * @property {MetricPattern27<AnyAddressIndex>} p2pk65
 * @property {MetricPattern28<AnyAddressIndex>} p2pkh
 * @property {MetricPattern29<AnyAddressIndex>} p2sh
 * @property {MetricPattern30<AnyAddressIndex>} p2tr
 * @property {MetricPattern31<AnyAddressIndex>} p2wpkh
 * @property {MetricPattern32<AnyAddressIndex>} p2wsh
 * @property {MetricPattern34<FundedAddressIndex>} funded
 * @property {MetricPattern35<EmptyAddressIndex>} empty
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Data
 * @property {MetricPattern34<FundedAddressData>} funded
 * @property {MetricPattern35<EmptyAddressData>} empty
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Activity
 * @property {BothReactivatedReceivingSendingPattern} all
 * @property {BothReactivatedReceivingSendingPattern} p2pk65
 * @property {BothReactivatedReceivingSendingPattern} p2pk33
 * @property {BothReactivatedReceivingSendingPattern} p2pkh
 * @property {BothReactivatedReceivingSendingPattern} p2sh
 * @property {BothReactivatedReceivingSendingPattern} p2wpkh
 * @property {BothReactivatedReceivingSendingPattern} p2wsh
 * @property {BothReactivatedReceivingSendingPattern} p2tr
 * @property {BothReactivatedReceivingSendingPattern} p2a
 */

/**
 * @typedef {Object} MetricsTree_Addresses_New
 * @property {BaseCumulativeSumPattern<StoredU64>} all
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pk65
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pk33
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pkh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2sh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2wpkh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2wsh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2tr
 * @property {BaseCumulativeSumPattern<StoredU64>} p2a
 */

/**
 * @typedef {Object} MetricsTree_Addresses_Delta
 * @property {ChangeRatePattern} all
 * @property {ChangeRatePattern} p2pk65
 * @property {ChangeRatePattern} p2pk33
 * @property {ChangeRatePattern} p2pkh
 * @property {ChangeRatePattern} p2sh
 * @property {ChangeRatePattern} p2wpkh
 * @property {ChangeRatePattern} p2wsh
 * @property {ChangeRatePattern} p2tr
 * @property {ChangeRatePattern} p2a
 */

/**
 * @typedef {Object} MetricsTree_Scripts
 * @property {MetricsTree_Scripts_Raw} raw
 * @property {MetricsTree_Scripts_Count} count
 * @property {MetricsTree_Scripts_Value} value
 * @property {MetricsTree_Scripts_Adoption} adoption
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Raw
 * @property {MetricsTree_Scripts_Raw_Empty} empty
 * @property {MetricsTree_Scripts_Raw_Opreturn} opreturn
 * @property {MetricsTree_Scripts_Raw_P2ms} p2ms
 * @property {MetricsTree_Scripts_Raw_Unknown} unknown
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Raw_Empty
 * @property {MetricPattern18<EmptyOutputIndex>} firstIndex
 * @property {MetricPattern22<TxIndex>} toTxindex
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Raw_Opreturn
 * @property {MetricPattern18<OpReturnIndex>} firstIndex
 * @property {MetricPattern23<TxIndex>} toTxindex
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Raw_P2ms
 * @property {MetricPattern18<P2MSOutputIndex>} firstIndex
 * @property {MetricPattern25<TxIndex>} toTxindex
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Raw_Unknown
 * @property {MetricPattern18<UnknownOutputIndex>} firstIndex
 * @property {MetricPattern33<TxIndex>} toTxindex
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Count
 * @property {BaseCumulativeSumPattern<StoredU64>} p2a
 * @property {BaseCumulativeSumPattern<StoredU64>} p2ms
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pk33
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pk65
 * @property {BaseCumulativeSumPattern<StoredU64>} p2pkh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2sh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2tr
 * @property {BaseCumulativeSumPattern<StoredU64>} p2wpkh
 * @property {BaseCumulativeSumPattern<StoredU64>} p2wsh
 * @property {BaseCumulativeSumPattern<StoredU64>} opreturn
 * @property {BaseCumulativeSumPattern<StoredU64>} emptyoutput
 * @property {BaseCumulativeSumPattern<StoredU64>} unknownoutput
 * @property {BaseCumulativeSumPattern<StoredU64>} segwit
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value
 * @property {MetricsTree_Scripts_Value_Opreturn} opreturn
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Value_Opreturn
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 */

/**
 * @typedef {Object} MetricsTree_Scripts_Adoption
 * @property {BpsPercentRatioPattern4} taproot
 * @property {BpsPercentRatioPattern4} segwit
 */

/**
 * @typedef {Object} MetricsTree_Mining
 * @property {MetricsTree_Mining_Rewards} rewards
 * @property {MetricsTree_Mining_Hashrate} hashrate
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards
 * @property {BaseCumulativeSumPattern4} coinbase
 * @property {MetricsTree_Mining_Rewards_Subsidy} subsidy
 * @property {MetricsTree_Mining_Rewards_Fees} fees
 * @property {BaseCumulativeSumPattern4} unclaimed
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards_Subsidy
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 * @property {_1m1w1y24hBpsPercentRatioPattern} dominance
 * @property {CentsUsdPattern2} sma1y
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards_Fees
 * @property {BtcCentsSatsUsdPattern} base
 * @property {BtcCentsSatsUsdPattern} cumulative
 * @property {_1m1w1y24hPattern6} sum
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} _24h
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} _1w
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} _1m
 * @property {AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern} _1y
 * @property {_1m1w1y24hBpsPercentRatioPattern} dominance
 * @property {MetricsTree_Mining_Rewards_Fees_RatioMultiple} ratioMultiple
 */

/**
 * @typedef {Object} MetricsTree_Mining_Rewards_Fees_RatioMultiple
 * @property {BpsRatioPattern2} _24h
 * @property {BpsRatioPattern2} _1w
 * @property {BpsRatioPattern2} _1m
 * @property {BpsRatioPattern2} _1y
 */

/**
 * @typedef {Object} MetricsTree_Mining_Hashrate
 * @property {MetricsTree_Mining_Hashrate_Rate} rate
 * @property {PhsReboundThsPattern} price
 * @property {PhsReboundThsPattern} value
 */

/**
 * @typedef {Object} MetricsTree_Mining_Hashrate_Rate
 * @property {MetricPattern1<StoredF64>} base
 * @property {MetricsTree_Mining_Hashrate_Rate_Sma} sma
 * @property {MetricPattern1<StoredF64>} ath
 * @property {BpsPercentRatioPattern5} drawdown
 */

/**
 * @typedef {Object} MetricsTree_Mining_Hashrate_Rate_Sma
 * @property {MetricPattern1<StoredF64>} _1w
 * @property {MetricPattern1<StoredF64>} _1m
 * @property {MetricPattern1<StoredF64>} _2m
 * @property {MetricPattern1<StoredF64>} _1y
 */

/**
 * @typedef {Object} MetricsTree_Cointime
 * @property {MetricsTree_Cointime_Activity} activity
 * @property {MetricsTree_Cointime_Supply} supply
 * @property {MetricsTree_Cointime_Value} value
 * @property {MetricsTree_Cointime_Cap} cap
 * @property {MetricsTree_Cointime_Prices} prices
 * @property {MetricsTree_Cointime_Adjusted} adjusted
 * @property {MetricsTree_Cointime_ReserveRisk} reserveRisk
 * @property {MetricsTree_Cointime_CoinblocksDestroyed} coinblocksDestroyed
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Activity
 * @property {BaseCumulativeSumPattern<StoredF64>} coinblocksCreated
 * @property {BaseCumulativeSumPattern<StoredF64>} coinblocksStored
 * @property {MetricPattern1<StoredF64>} liveliness
 * @property {MetricPattern1<StoredF64>} vaultedness
 * @property {MetricPattern1<StoredF64>} ratio
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Supply
 * @property {BtcCentsSatsUsdPattern} vaulted
 * @property {BtcCentsSatsUsdPattern} active
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Value
 * @property {BaseCumulativeSumPattern<StoredF64>} destroyed
 * @property {BaseCumulativeSumPattern<StoredF64>} created
 * @property {BaseCumulativeSumPattern<StoredF64>} stored
 * @property {BaseCumulativeSumPattern<StoredF64>} vocdd
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Cap
 * @property {CentsUsdPattern2} thermo
 * @property {CentsUsdPattern2} investor
 * @property {CentsUsdPattern2} vaulted
 * @property {CentsUsdPattern2} active
 * @property {CentsUsdPattern2} cointime
 * @property {BpsRatioPattern2} aviv
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Prices
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} vaulted
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} active
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} trueMarketMean
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} cointime
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} transfer
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} balanced
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} terminal
 * @property {BpsCentsPercentilesRatioSatsUsdPattern} delta
 * @property {MetricPattern1<Dollars>} cumulativeMarketCap
 */

/**
 * @typedef {Object} MetricsTree_Cointime_Adjusted
 * @property {BpsPercentRatioPattern2} inflationRate
 * @property {MetricPattern1<StoredF64>} txVelocityBtc
 * @property {MetricPattern1<StoredF64>} txVelocityUsd
 */

/**
 * @typedef {Object} MetricsTree_Cointime_ReserveRisk
 * @property {MetricPattern1<StoredF64>} value
 * @property {MetricPattern18<StoredF64>} vocddMedian1y
 * @property {MetricPattern18<StoredF64>} hodlBank
 */

/**
 * @typedef {Object} MetricsTree_Cointime_CoinblocksDestroyed
 * @property {MetricPattern1<StoredF64>} base
 * @property {MetricPattern1<StoredF64>} cumulative
 */

/**
 * @typedef {Object} MetricsTree_Constants
 * @property {MetricPattern1<StoredU16>} _0
 * @property {MetricPattern1<StoredU16>} _1
 * @property {MetricPattern1<StoredU16>} _2
 * @property {MetricPattern1<StoredU16>} _3
 * @property {MetricPattern1<StoredU16>} _4
 * @property {MetricPattern1<StoredU16>} _20
 * @property {MetricPattern1<StoredU16>} _30
 * @property {MetricPattern1<StoredF32>} _382
 * @property {MetricPattern1<StoredU16>} _50
 * @property {MetricPattern1<StoredF32>} _618
 * @property {MetricPattern1<StoredU16>} _70
 * @property {MetricPattern1<StoredU16>} _80
 * @property {MetricPattern1<StoredU16>} _100
 * @property {MetricPattern1<StoredU16>} _600
 * @property {MetricPattern1<StoredI8>} minus1
 * @property {MetricPattern1<StoredI8>} minus2
 * @property {MetricPattern1<StoredI8>} minus3
 * @property {MetricPattern1<StoredI8>} minus4
 */

/**
 * @typedef {Object} MetricsTree_Indexes
 * @property {MetricsTree_Indexes_Address} address
 * @property {MetricsTree_Indexes_Height} height
 * @property {MetricsTree_Indexes_Epoch} epoch
 * @property {MetricsTree_Indexes_Halving} halving
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
 * @property {MetricPattern18<Epoch>} epoch
 * @property {MetricPattern18<Halving>} halving
 * @property {MetricPattern18<Week1>} week1
 * @property {MetricPattern18<Month1>} month1
 * @property {MetricPattern18<Month3>} month3
 * @property {MetricPattern18<Month6>} month6
 * @property {MetricPattern18<Year1>} year1
 * @property {MetricPattern18<Year10>} year10
 * @property {MetricPattern18<StoredU64>} txindexCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Epoch
 * @property {MetricPattern17<Epoch>} identity
 * @property {MetricPattern17<Height>} firstHeight
 * @property {MetricPattern17<StoredU64>} heightCount
 */

/**
 * @typedef {Object} MetricsTree_Indexes_Halving
 * @property {MetricPattern16<Halving>} identity
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
 * @typedef {Object} MetricsTree_Indicators
 * @property {BpsRatioPattern2} puellMultiple
 * @property {BpsRatioPattern2} nvt
 * @property {BpsPercentRatioPattern4} gini
 * @property {BpsRatioPattern2} rhodlRatio
 * @property {BpsRatioPattern2} thermocapMultiple
 * @property {MetricPattern1<StoredF32>} coindaysDestroyedSupplyAdjusted
 * @property {MetricPattern1<StoredF32>} coinyearsDestroyedSupplyAdjusted
 * @property {MetricsTree_Indicators_Dormancy} dormancy
 * @property {MetricPattern1<StoredF32>} stockToFlow
 * @property {MetricPattern1<StoredF32>} sellerExhaustionConstant
 */

/**
 * @typedef {Object} MetricsTree_Indicators_Dormancy
 * @property {MetricPattern1<StoredF32>} supplyAdjusted
 * @property {MetricPattern1<StoredF32>} flow
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
 * @property {MetricsTree_Market_Technical} technical
 */

/**
 * @typedef {Object} MetricsTree_Market_Ath
 * @property {CentsSatsUsdPattern} high
 * @property {BpsPercentRatioPattern5} drawdown
 * @property {MetricPattern1<StoredF32>} daysSince
 * @property {MetricPattern2<StoredF32>} yearsSince
 * @property {MetricPattern1<StoredF32>} maxDaysBetween
 * @property {MetricPattern2<StoredF32>} maxYearsBetween
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
 * @property {MetricsTree_Market_Returns_Periods} periods
 * @property {_10y2y3y4y5y6y8yPattern} cagr
 * @property {MetricsTree_Market_Returns_Sd24h} sd24h
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_Periods
 * @property {BpsPercentRatioPattern2} _24h
 * @property {BpsPercentRatioPattern2} _1w
 * @property {BpsPercentRatioPattern2} _1m
 * @property {BpsPercentRatioPattern2} _3m
 * @property {BpsPercentRatioPattern2} _6m
 * @property {BpsPercentRatioPattern2} _1y
 * @property {BpsPercentRatioPattern2} _2y
 * @property {BpsPercentRatioPattern2} _3y
 * @property {BpsPercentRatioPattern2} _4y
 * @property {BpsPercentRatioPattern2} _5y
 * @property {BpsPercentRatioPattern2} _6y
 * @property {BpsPercentRatioPattern2} _8y
 * @property {BpsPercentRatioPattern2} _10y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_Sd24h
 * @property {MetricsTree_Market_Returns_Sd24h_1w} _1w
 * @property {MetricsTree_Market_Returns_Sd24h_1m} _1m
 * @property {SdSmaPattern} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_Sd24h_1w
 * @property {MetricPattern1<StoredF32>} sma
 * @property {MetricPattern1<StoredF32>} sd
 */

/**
 * @typedef {Object} MetricsTree_Market_Returns_Sd24h_1m
 * @property {MetricPattern1<StoredF32>} sma
 * @property {MetricPattern1<StoredF32>} sd
 */

/**
 * @typedef {Object} MetricsTree_Market_Volatility
 * @property {MetricPattern1<StoredF32>} _1w
 * @property {MetricPattern1<StoredF32>} _1m
 * @property {MetricPattern1<StoredF32>} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Range
 * @property {_1m1w1y2wPattern} min
 * @property {_1m1w1y2wPattern} max
 * @property {MetricPattern1<StoredF32>} trueRange
 * @property {MetricPattern1<StoredF32>} trueRangeSum2w
 * @property {BpsPercentRatioPattern4} choppinessIndex2w
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage
 * @property {MetricsTree_Market_MovingAverage_Sma} sma
 * @property {MetricsTree_Market_MovingAverage_Ema} ema
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma
 * @property {BpsCentsRatioSatsUsdPattern} _1w
 * @property {BpsCentsRatioSatsUsdPattern} _8d
 * @property {BpsCentsRatioSatsUsdPattern} _13d
 * @property {BpsCentsRatioSatsUsdPattern} _21d
 * @property {BpsCentsRatioSatsUsdPattern} _1m
 * @property {BpsCentsRatioSatsUsdPattern} _34d
 * @property {BpsCentsRatioSatsUsdPattern} _55d
 * @property {BpsCentsRatioSatsUsdPattern} _89d
 * @property {BpsCentsRatioSatsUsdPattern} _111d
 * @property {BpsCentsRatioSatsUsdPattern} _144d
 * @property {MetricsTree_Market_MovingAverage_Sma_200d} _200d
 * @property {MetricsTree_Market_MovingAverage_Sma_350d} _350d
 * @property {BpsCentsRatioSatsUsdPattern} _1y
 * @property {BpsCentsRatioSatsUsdPattern} _2y
 * @property {BpsCentsRatioSatsUsdPattern} _200w
 * @property {BpsCentsRatioSatsUsdPattern} _4y
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma_200d
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricsTree_Market_MovingAverage_Sma_200d_X24} x24
 * @property {MetricsTree_Market_MovingAverage_Sma_200d_X08} x08
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma_200d_X24
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<SatsFract>} sats
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma_200d_X08
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<SatsFract>} sats
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma_350d
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<SatsFract>} sats
 * @property {MetricPattern1<BasisPoints32>} bps
 * @property {MetricPattern1<StoredF32>} ratio
 * @property {MetricsTree_Market_MovingAverage_Sma_350d_X2} x2
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Sma_350d_X2
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<SatsFract>} sats
 */

/**
 * @typedef {Object} MetricsTree_Market_MovingAverage_Ema
 * @property {BpsCentsRatioSatsUsdPattern} _1w
 * @property {BpsCentsRatioSatsUsdPattern} _8d
 * @property {BpsCentsRatioSatsUsdPattern} _12d
 * @property {BpsCentsRatioSatsUsdPattern} _13d
 * @property {BpsCentsRatioSatsUsdPattern} _21d
 * @property {BpsCentsRatioSatsUsdPattern} _26d
 * @property {BpsCentsRatioSatsUsdPattern} _1m
 * @property {BpsCentsRatioSatsUsdPattern} _34d
 * @property {BpsCentsRatioSatsUsdPattern} _55d
 * @property {BpsCentsRatioSatsUsdPattern} _89d
 * @property {BpsCentsRatioSatsUsdPattern} _144d
 * @property {BpsCentsRatioSatsUsdPattern} _200d
 * @property {BpsCentsRatioSatsUsdPattern} _1y
 * @property {BpsCentsRatioSatsUsdPattern} _2y
 * @property {BpsCentsRatioSatsUsdPattern} _200w
 * @property {BpsCentsRatioSatsUsdPattern} _4y
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca
 * @property {MetricPattern18<Sats>} satsPerDay
 * @property {MetricsTree_Market_Dca_Period} period
 * @property {MetricsTree_Market_Dca_Class} class
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_Period
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} stack
 * @property {MetricsTree_Market_Dca_Period_CostBasis} costBasis
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2} return
 * @property {_10y2y3y4y5y6y8yPattern} cagr
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern3} lumpSumStack
 * @property {_10y1m1w1y2y3m3y4y5y6m6y8yPattern2} lumpSumReturn
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_Period_CostBasis
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
 * @typedef {Object} MetricsTree_Market_Dca_Class
 * @property {MetricsTree_Market_Dca_Class_Stack} stack
 * @property {MetricsTree_Market_Dca_Class_CostBasis} costBasis
 * @property {MetricsTree_Market_Dca_Class_Return} return
 */

/**
 * @typedef {Object} MetricsTree_Market_Dca_Class_Stack
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
 * @typedef {Object} MetricsTree_Market_Dca_Class_CostBasis
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
 * @typedef {Object} MetricsTree_Market_Dca_Class_Return
 * @property {BpsPercentRatioPattern2} from2015
 * @property {BpsPercentRatioPattern2} from2016
 * @property {BpsPercentRatioPattern2} from2017
 * @property {BpsPercentRatioPattern2} from2018
 * @property {BpsPercentRatioPattern2} from2019
 * @property {BpsPercentRatioPattern2} from2020
 * @property {BpsPercentRatioPattern2} from2021
 * @property {BpsPercentRatioPattern2} from2022
 * @property {BpsPercentRatioPattern2} from2023
 * @property {BpsPercentRatioPattern2} from2024
 * @property {BpsPercentRatioPattern2} from2025
 * @property {BpsPercentRatioPattern2} from2026
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical
 * @property {MetricsTree_Market_Technical_Rsi} rsi
 * @property {BpsPercentRatioPattern4} stochK
 * @property {BpsPercentRatioPattern4} stochD
 * @property {BpsRatioPattern2} piCycle
 * @property {MetricsTree_Market_Technical_Macd} macd
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Rsi
 * @property {AverageGainsLossesRsiStochPattern} _24h
 * @property {MetricsTree_Market_Technical_Rsi_1w} _1w
 * @property {MetricsTree_Market_Technical_Rsi_1m} _1m
 * @property {MetricsTree_Market_Technical_Rsi_1y} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Rsi_1w
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern4} rsi
 * @property {BpsPercentRatioPattern4} rsiMin
 * @property {BpsPercentRatioPattern4} rsiMax
 * @property {BpsPercentRatioPattern4} stochRsi
 * @property {BpsPercentRatioPattern4} stochRsiK
 * @property {BpsPercentRatioPattern4} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Rsi_1m
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern4} rsi
 * @property {BpsPercentRatioPattern4} rsiMin
 * @property {BpsPercentRatioPattern4} rsiMax
 * @property {BpsPercentRatioPattern4} stochRsi
 * @property {BpsPercentRatioPattern4} stochRsiK
 * @property {BpsPercentRatioPattern4} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Rsi_1y
 * @property {MetricPattern1<StoredF32>} gains
 * @property {MetricPattern1<StoredF32>} losses
 * @property {MetricPattern1<StoredF32>} averageGain
 * @property {MetricPattern1<StoredF32>} averageLoss
 * @property {BpsPercentRatioPattern4} rsi
 * @property {BpsPercentRatioPattern4} rsiMin
 * @property {BpsPercentRatioPattern4} rsiMax
 * @property {BpsPercentRatioPattern4} stochRsi
 * @property {BpsPercentRatioPattern4} stochRsiK
 * @property {BpsPercentRatioPattern4} stochRsiD
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Macd
 * @property {EmaHistogramLineSignalPattern} _24h
 * @property {MetricsTree_Market_Technical_Macd_1w} _1w
 * @property {MetricsTree_Market_Technical_Macd_1m} _1m
 * @property {MetricsTree_Market_Technical_Macd_1y} _1y
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Macd_1w
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Macd_1m
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Market_Technical_Macd_1y
 * @property {MetricPattern1<StoredF32>} emaFast
 * @property {MetricPattern1<StoredF32>} emaSlow
 * @property {MetricPattern1<StoredF32>} line
 * @property {MetricPattern1<StoredF32>} signal
 * @property {MetricPattern1<StoredF32>} histogram
 */

/**
 * @typedef {Object} MetricsTree_Pools
 * @property {MetricPattern18<PoolSlug>} heightToPool
 * @property {MetricsTree_Pools_Major} major
 * @property {MetricsTree_Pools_Minor} minor
 */

/**
 * @typedef {Object} MetricsTree_Pools_Major
 * @property {BlocksDominanceRewardsPattern} unknown
 * @property {BlocksDominanceRewardsPattern} luxor
 * @property {BlocksDominanceRewardsPattern} btccom
 * @property {BlocksDominanceRewardsPattern} btctop
 * @property {BlocksDominanceRewardsPattern} btcguild
 * @property {BlocksDominanceRewardsPattern} eligius
 * @property {BlocksDominanceRewardsPattern} f2pool
 * @property {BlocksDominanceRewardsPattern} braiinspool
 * @property {BlocksDominanceRewardsPattern} antpool
 * @property {BlocksDominanceRewardsPattern} btcc
 * @property {BlocksDominanceRewardsPattern} bwpool
 * @property {BlocksDominanceRewardsPattern} bitfury
 * @property {BlocksDominanceRewardsPattern} viabtc
 * @property {BlocksDominanceRewardsPattern} poolin
 * @property {BlocksDominanceRewardsPattern} spiderpool
 * @property {BlocksDominanceRewardsPattern} binancepool
 * @property {BlocksDominanceRewardsPattern} foundryusa
 * @property {BlocksDominanceRewardsPattern} sbicrypto
 * @property {BlocksDominanceRewardsPattern} marapool
 * @property {BlocksDominanceRewardsPattern} secpool
 * @property {BlocksDominanceRewardsPattern} ocean
 * @property {BlocksDominanceRewardsPattern} whitepool
 */

/**
 * @typedef {Object} MetricsTree_Pools_Minor
 * @property {BlocksDominancePattern} blockfills
 * @property {BlocksDominancePattern} ultimuspool
 * @property {BlocksDominancePattern} terrapool
 * @property {BlocksDominancePattern} onethash
 * @property {BlocksDominancePattern} bitfarms
 * @property {BlocksDominancePattern} huobipool
 * @property {BlocksDominancePattern} wayicn
 * @property {BlocksDominancePattern} canoepool
 * @property {BlocksDominancePattern} bitcoincom
 * @property {BlocksDominancePattern} pool175btc
 * @property {BlocksDominancePattern} gbminers
 * @property {BlocksDominancePattern} axbt
 * @property {BlocksDominancePattern} asicminer
 * @property {BlocksDominancePattern} bitminter
 * @property {BlocksDominancePattern} bitcoinrussia
 * @property {BlocksDominancePattern} btcserv
 * @property {BlocksDominancePattern} simplecoinus
 * @property {BlocksDominancePattern} ozcoin
 * @property {BlocksDominancePattern} eclipsemc
 * @property {BlocksDominancePattern} maxbtc
 * @property {BlocksDominancePattern} triplemining
 * @property {BlocksDominancePattern} coinlab
 * @property {BlocksDominancePattern} pool50btc
 * @property {BlocksDominancePattern} ghashio
 * @property {BlocksDominancePattern} stminingcorp
 * @property {BlocksDominancePattern} bitparking
 * @property {BlocksDominancePattern} mmpool
 * @property {BlocksDominancePattern} polmine
 * @property {BlocksDominancePattern} kncminer
 * @property {BlocksDominancePattern} bitalo
 * @property {BlocksDominancePattern} hhtt
 * @property {BlocksDominancePattern} megabigpower
 * @property {BlocksDominancePattern} mtred
 * @property {BlocksDominancePattern} nmcbit
 * @property {BlocksDominancePattern} yourbtcnet
 * @property {BlocksDominancePattern} givemecoins
 * @property {BlocksDominancePattern} multicoinco
 * @property {BlocksDominancePattern} bcpoolio
 * @property {BlocksDominancePattern} cointerra
 * @property {BlocksDominancePattern} kanopool
 * @property {BlocksDominancePattern} solock
 * @property {BlocksDominancePattern} ckpool
 * @property {BlocksDominancePattern} nicehash
 * @property {BlocksDominancePattern} bitclub
 * @property {BlocksDominancePattern} bitcoinaffiliatenetwork
 * @property {BlocksDominancePattern} exxbw
 * @property {BlocksDominancePattern} bitsolo
 * @property {BlocksDominancePattern} twentyoneinc
 * @property {BlocksDominancePattern} digitalbtc
 * @property {BlocksDominancePattern} eightbaochi
 * @property {BlocksDominancePattern} mybtccoinpool
 * @property {BlocksDominancePattern} tbdice
 * @property {BlocksDominancePattern} hashpool
 * @property {BlocksDominancePattern} nexious
 * @property {BlocksDominancePattern} bravomining
 * @property {BlocksDominancePattern} hotpool
 * @property {BlocksDominancePattern} okexpool
 * @property {BlocksDominancePattern} bcmonster
 * @property {BlocksDominancePattern} onehash
 * @property {BlocksDominancePattern} bixin
 * @property {BlocksDominancePattern} tatmaspool
 * @property {BlocksDominancePattern} connectbtc
 * @property {BlocksDominancePattern} batpool
 * @property {BlocksDominancePattern} waterhole
 * @property {BlocksDominancePattern} dcexploration
 * @property {BlocksDominancePattern} dcex
 * @property {BlocksDominancePattern} btpool
 * @property {BlocksDominancePattern} fiftyeightcoin
 * @property {BlocksDominancePattern} bitcoinindia
 * @property {BlocksDominancePattern} shawnp0wers
 * @property {BlocksDominancePattern} phashio
 * @property {BlocksDominancePattern} rigpool
 * @property {BlocksDominancePattern} haozhuzhu
 * @property {BlocksDominancePattern} sevenpool
 * @property {BlocksDominancePattern} miningkings
 * @property {BlocksDominancePattern} hashbx
 * @property {BlocksDominancePattern} dpool
 * @property {BlocksDominancePattern} rawpool
 * @property {BlocksDominancePattern} haominer
 * @property {BlocksDominancePattern} helix
 * @property {BlocksDominancePattern} bitcoinukraine
 * @property {BlocksDominancePattern} secretsuperstar
 * @property {BlocksDominancePattern} tigerpoolnet
 * @property {BlocksDominancePattern} sigmapoolcom
 * @property {BlocksDominancePattern} okpooltop
 * @property {BlocksDominancePattern} hummerpool
 * @property {BlocksDominancePattern} tangpool
 * @property {BlocksDominancePattern} bytepool
 * @property {BlocksDominancePattern} novablock
 * @property {BlocksDominancePattern} miningcity
 * @property {BlocksDominancePattern} minerium
 * @property {BlocksDominancePattern} lubiancom
 * @property {BlocksDominancePattern} okkong
 * @property {BlocksDominancePattern} aaopool
 * @property {BlocksDominancePattern} emcdpool
 * @property {BlocksDominancePattern} arkpool
 * @property {BlocksDominancePattern} purebtccom
 * @property {BlocksDominancePattern} kucoinpool
 * @property {BlocksDominancePattern} entrustcharitypool
 * @property {BlocksDominancePattern} okminer
 * @property {BlocksDominancePattern} titan
 * @property {BlocksDominancePattern} pegapool
 * @property {BlocksDominancePattern} btcnuggets
 * @property {BlocksDominancePattern} cloudhashing
 * @property {BlocksDominancePattern} digitalxmintsy
 * @property {BlocksDominancePattern} telco214
 * @property {BlocksDominancePattern} btcpoolparty
 * @property {BlocksDominancePattern} multipool
 * @property {BlocksDominancePattern} transactioncoinmining
 * @property {BlocksDominancePattern} btcdig
 * @property {BlocksDominancePattern} trickysbtcpool
 * @property {BlocksDominancePattern} btcmp
 * @property {BlocksDominancePattern} eobot
 * @property {BlocksDominancePattern} unomp
 * @property {BlocksDominancePattern} patels
 * @property {BlocksDominancePattern} gogreenlight
 * @property {BlocksDominancePattern} bitcoinindiapool
 * @property {BlocksDominancePattern} ekanembtc
 * @property {BlocksDominancePattern} canoe
 * @property {BlocksDominancePattern} tiger
 * @property {BlocksDominancePattern} onem1x
 * @property {BlocksDominancePattern} zulupool
 * @property {BlocksDominancePattern} wiz
 * @property {BlocksDominancePattern} wk057
 * @property {BlocksDominancePattern} futurebitapollosolo
 * @property {BlocksDominancePattern} carbonnegative
 * @property {BlocksDominancePattern} portlandhodl
 * @property {BlocksDominancePattern} phoenix
 * @property {BlocksDominancePattern} neopool
 * @property {BlocksDominancePattern} maxipool
 * @property {BlocksDominancePattern} bitfufupool
 * @property {BlocksDominancePattern} gdpool
 * @property {BlocksDominancePattern} miningdutch
 * @property {BlocksDominancePattern} publicpool
 * @property {BlocksDominancePattern} miningsquared
 * @property {BlocksDominancePattern} innopolistech
 * @property {BlocksDominancePattern} btclab
 * @property {BlocksDominancePattern} parasite
 * @property {BlocksDominancePattern} redrockpool
 * @property {BlocksDominancePattern} est3lar
 */

/**
 * @typedef {Object} MetricsTree_Prices
 * @property {MetricsTree_Prices_Split} split
 * @property {MetricsTree_Prices_Ohlc} ohlc
 * @property {MetricsTree_Prices_Spot} spot
 */

/**
 * @typedef {Object} MetricsTree_Prices_Split
 * @property {CentsSatsUsdPattern3} open
 * @property {CentsSatsUsdPattern3} high
 * @property {CentsSatsUsdPattern3} low
 * @property {CentsSatsUsdPattern3} close
 */

/**
 * @typedef {Object} MetricsTree_Prices_Ohlc
 * @property {MetricPattern2<OHLCCents>} cents
 * @property {MetricPattern2<OHLCDollars>} usd
 * @property {MetricPattern2<OHLCSats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Prices_Spot
 * @property {MetricPattern1<Cents>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {MetricPattern1<Sats>} sats
 */

/**
 * @typedef {Object} MetricsTree_Supply
 * @property {BtcCentsSatsUsdPattern} circulating
 * @property {MetricsTree_Supply_Burned} burned
 * @property {BpsPercentRatioPattern2} inflationRate
 * @property {MetricsTree_Supply_Velocity} velocity
 * @property {CentsDeltaUsdPattern} marketCap
 * @property {_1m1w1y24hPattern<BasisPointsSigned32>} marketMinusRealizedCapGrowthRate
 * @property {BtcCentsSatsUsdPattern} hodledOrLost
 * @property {MetricPattern18<SupplyState>} state
 */

/**
 * @typedef {Object} MetricsTree_Supply_Burned
 * @property {BaseCumulativeSumPattern4} opreturn
 * @property {BaseCumulativeSumPattern4} unspendable
 */

/**
 * @typedef {Object} MetricsTree_Supply_Velocity
 * @property {MetricPattern1<StoredF64>} btc
 * @property {MetricPattern1<StoredF64>} usd
 */

/**
 * @typedef {Object} MetricsTree_Cohorts
 * @property {MetricsTree_Cohorts_Utxo} utxo
 * @property {MetricsTree_Cohorts_Address} address
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo
 * @property {MetricsTree_Cohorts_Utxo_All} all
 * @property {MetricsTree_Cohorts_Utxo_Sth} sth
 * @property {MetricsTree_Cohorts_Utxo_Lth} lth
 * @property {MetricsTree_Cohorts_Utxo_AgeRange} ageRange
 * @property {MetricsTree_Cohorts_Utxo_UnderAge} underAge
 * @property {MetricsTree_Cohorts_Utxo_OverAge} overAge
 * @property {MetricsTree_Cohorts_Utxo_Epoch} epoch
 * @property {MetricsTree_Cohorts_Utxo_Class} class
 * @property {MetricsTree_Cohorts_Utxo_OverAmount} overAmount
 * @property {MetricsTree_Cohorts_Utxo_AmountRange} amountRange
 * @property {MetricsTree_Cohorts_Utxo_UnderAmount} underAmount
 * @property {MetricsTree_Cohorts_Utxo_Type} type
 * @property {MetricsTree_Cohorts_Utxo_Profitability} profitability
 * @property {MetricsTree_Cohorts_Utxo_Matured} matured
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All
 * @property {MetricsTree_Cohorts_Utxo_All_Supply} supply
 * @property {UnspentPattern} outputs
 * @property {CoindaysCoinyearsDormancySentVelocityPattern} activity
 * @property {CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} realized
 * @property {InvestedMaxMinPercentilesSupplyPattern} costBasis
 * @property {MetricsTree_Cohorts_Utxo_All_Unrealized} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All_Supply
 * @property {BtcCentsRelSatsUsdPattern2} inProfit
 * @property {BtcCentsRelSatsUsdPattern2} inLoss
 * @property {BtcCentsSatsUsdPattern} total
 * @property {BtcCentsSatsUsdPattern} half
 * @property {ChangeRatePattern2} delta
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All_Unrealized
 * @property {CentsUsdPattern2} grossPnl
 * @property {InPattern} investedCapital
 * @property {GreedNetPainPattern} sentiment
 * @property {MetricsTree_Cohorts_Utxo_All_Unrealized_Loss} loss
 * @property {MetricsTree_Cohorts_Utxo_All_Unrealized_NetPnl} netPnl
 * @property {MetricsTree_Cohorts_Utxo_All_Unrealized_Profit} profit
 * @property {BpsRatioPattern} nupl
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All_Unrealized_Loss
 * @property {MetricPattern1<Dollars>} negative
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {_1m1w1y24hPattern5} sum
 * @property {BpsPercentRatioPattern4} relToMcap
 * @property {BpsPercentRatioPattern4} relToOwnGross
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All_Unrealized_NetPnl
 * @property {MetricPattern1<CentsSigned>} cents
 * @property {MetricPattern1<Dollars>} usd
 * @property {BpsPercentRatioPattern2} relToOwnGross
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_All_Unrealized_Profit
 * @property {CentsUsdPattern2} base
 * @property {CentsUsdPattern2} cumulative
 * @property {_1m1w1y24hPattern5} sum
 * @property {BpsPercentRatioPattern4} relToMcap
 * @property {BpsPercentRatioPattern4} relToOwnGross
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Sth
 * @property {CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern} realized
 * @property {DeltaHalfInRelTotalPattern2} supply
 * @property {UnspentPattern} outputs
 * @property {CoindaysCoinyearsDormancySentVelocityPattern} activity
 * @property {InvestedMaxMinPercentilesSupplyPattern} costBasis
 * @property {GrossInvestedLossNetNuplProfitSentimentPattern2} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Lth
 * @property {DeltaHalfInRelTotalPattern2} supply
 * @property {UnspentPattern} outputs
 * @property {CoindaysCoinyearsDormancySentVelocityPattern} activity
 * @property {MetricsTree_Cohorts_Utxo_Lth_Realized} realized
 * @property {InvestedMaxMinPercentilesSupplyPattern} costBasis
 * @property {GrossInvestedLossNetNuplProfitSentimentPattern2} unrealized
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Lth_Realized
 * @property {BaseCumulativeDistributionRelSumValuePattern} profit
 * @property {BaseCapitulationCumulativeNegativeRelSumValuePattern} loss
 * @property {BaseCumulativeSumPattern3} grossPnl
 * @property {MetricsTree_Cohorts_Utxo_Lth_Realized_SellSideRiskRatio} sellSideRiskRatio
 * @property {BaseChangeCumulativeDeltaRelSumPattern} netPnl
 * @property {MetricsTree_Cohorts_Utxo_Lth_Realized_Sopr} sopr
 * @property {BaseCumulativeRelPattern} peakRegret
 * @property {LowerPriceUpperPattern} investor
 * @property {_1m1w1y24hPattern<StoredF64>} profitToLossRatio
 * @property {CentsDeltaRelUsdPattern} cap
 * @property {BpsCentsPercentilesRatioSatsSmaStdUsdPattern} price
 * @property {MetricPattern1<StoredF32>} mvrv
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Lth_Realized_SellSideRiskRatio
 * @property {BpsPercentRatioPattern} _24h
 * @property {BpsPercentRatioPattern} _1w
 * @property {BpsPercentRatioPattern} _1m
 * @property {BpsPercentRatioPattern} _1y
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Lth_Realized_Sopr
 * @property {_1m1w1y24hPattern<StoredF64>} ratio
 * @property {BaseCumulativeSumPattern<Cents>} valueCreated
 * @property {BaseCumulativeSumPattern<Cents>} valueDestroyed
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_AgeRange
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} under1h
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1hTo1d
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1dTo1w
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1wTo1m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1mTo2m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2mTo3m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3mTo4m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4mTo5m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5mTo6m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6mTo1y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1yTo2y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2yTo3y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3yTo4y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4yTo5y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5yTo6y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6yTo7y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _7yTo8y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _8yTo10y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _10yTo12y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _12yTo15y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} over15y
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_UnderAge
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1w
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _7y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _8y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _10y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _12y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _15y
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_OverAge
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1d
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1w
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6m
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _5y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _6y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _7y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _8y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _10y
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _12y
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Epoch
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _0
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _1
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _3
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _4
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Class
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2009
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2010
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2011
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2012
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2013
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2014
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2015
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2016
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2017
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2018
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2019
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2020
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2021
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2022
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2023
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2024
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2025
 * @property {ActivityOutputsRealizedSupplyUnrealizedPattern} _2026
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_OverAmount
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1sat
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kBtc
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_AmountRange
 * @property {OutputsRealizedSupplyUnrealizedPattern} _0sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1satTo10sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10satsTo100sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100satsTo1kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kSatsTo10kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kSatsTo100kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100kSatsTo1mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1mSatsTo10mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10mSatsTo1btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1btcTo10btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10btcTo100btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100btcTo1kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kBtcTo10kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kBtcTo100kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} over100kBtc
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_UnderAmount
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100sats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100kSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10mSats
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100btc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _1kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _10kBtc
 * @property {OutputsRealizedSupplyUnrealizedPattern} _100kBtc
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Type
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2pk65
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2pk33
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2pkh
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2ms
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2sh
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2wpkh
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2wsh
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2tr
 * @property {OutputsRealizedSupplyUnrealizedPattern2} p2a
 * @property {OutputsRealizedSupplyUnrealizedPattern2} unknown
 * @property {OutputsRealizedSupplyUnrealizedPattern2} empty
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Profitability
 * @property {MetricsTree_Cohorts_Utxo_Profitability_Range} range
 * @property {MetricsTree_Cohorts_Utxo_Profitability_Profit} profit
 * @property {MetricsTree_Cohorts_Utxo_Profitability_Loss} loss
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Profitability_Range
 * @property {RealizedSupplyPattern} over1000pctInProfit
 * @property {RealizedSupplyPattern} _500pctTo1000pctInProfit
 * @property {RealizedSupplyPattern} _300pctTo500pctInProfit
 * @property {RealizedSupplyPattern} _200pctTo300pctInProfit
 * @property {RealizedSupplyPattern} _100pctTo200pctInProfit
 * @property {RealizedSupplyPattern} _90pctTo100pctInProfit
 * @property {RealizedSupplyPattern} _80pctTo90pctInProfit
 * @property {RealizedSupplyPattern} _70pctTo80pctInProfit
 * @property {RealizedSupplyPattern} _60pctTo70pctInProfit
 * @property {RealizedSupplyPattern} _50pctTo60pctInProfit
 * @property {RealizedSupplyPattern} _40pctTo50pctInProfit
 * @property {RealizedSupplyPattern} _30pctTo40pctInProfit
 * @property {RealizedSupplyPattern} _20pctTo30pctInProfit
 * @property {RealizedSupplyPattern} _10pctTo20pctInProfit
 * @property {RealizedSupplyPattern} _0pctTo10pctInProfit
 * @property {RealizedSupplyPattern} _0pctTo10pctInLoss
 * @property {RealizedSupplyPattern} _10pctTo20pctInLoss
 * @property {RealizedSupplyPattern} _20pctTo30pctInLoss
 * @property {RealizedSupplyPattern} _30pctTo40pctInLoss
 * @property {RealizedSupplyPattern} _40pctTo50pctInLoss
 * @property {RealizedSupplyPattern} _50pctTo60pctInLoss
 * @property {RealizedSupplyPattern} _60pctTo70pctInLoss
 * @property {RealizedSupplyPattern} _70pctTo80pctInLoss
 * @property {RealizedSupplyPattern} _80pctTo90pctInLoss
 * @property {RealizedSupplyPattern} _90pctTo100pctInLoss
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Profitability_Profit
 * @property {RealizedSupplyPattern} breakeven
 * @property {RealizedSupplyPattern} _10pct
 * @property {RealizedSupplyPattern} _20pct
 * @property {RealizedSupplyPattern} _30pct
 * @property {RealizedSupplyPattern} _40pct
 * @property {RealizedSupplyPattern} _50pct
 * @property {RealizedSupplyPattern} _60pct
 * @property {RealizedSupplyPattern} _70pct
 * @property {RealizedSupplyPattern} _80pct
 * @property {RealizedSupplyPattern} _90pct
 * @property {RealizedSupplyPattern} _100pct
 * @property {RealizedSupplyPattern} _200pct
 * @property {RealizedSupplyPattern} _300pct
 * @property {RealizedSupplyPattern} _500pct
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Profitability_Loss
 * @property {RealizedSupplyPattern} breakeven
 * @property {RealizedSupplyPattern} _10pct
 * @property {RealizedSupplyPattern} _20pct
 * @property {RealizedSupplyPattern} _30pct
 * @property {RealizedSupplyPattern} _40pct
 * @property {RealizedSupplyPattern} _50pct
 * @property {RealizedSupplyPattern} _60pct
 * @property {RealizedSupplyPattern} _70pct
 * @property {RealizedSupplyPattern} _80pct
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Utxo_Matured
 * @property {BtcCentsSatsUsdPattern} under1h
 * @property {BtcCentsSatsUsdPattern} _1hTo1d
 * @property {BtcCentsSatsUsdPattern} _1dTo1w
 * @property {BtcCentsSatsUsdPattern} _1wTo1m
 * @property {BtcCentsSatsUsdPattern} _1mTo2m
 * @property {BtcCentsSatsUsdPattern} _2mTo3m
 * @property {BtcCentsSatsUsdPattern} _3mTo4m
 * @property {BtcCentsSatsUsdPattern} _4mTo5m
 * @property {BtcCentsSatsUsdPattern} _5mTo6m
 * @property {BtcCentsSatsUsdPattern} _6mTo1y
 * @property {BtcCentsSatsUsdPattern} _1yTo2y
 * @property {BtcCentsSatsUsdPattern} _2yTo3y
 * @property {BtcCentsSatsUsdPattern} _3yTo4y
 * @property {BtcCentsSatsUsdPattern} _4yTo5y
 * @property {BtcCentsSatsUsdPattern} _5yTo6y
 * @property {BtcCentsSatsUsdPattern} _6yTo7y
 * @property {BtcCentsSatsUsdPattern} _7yTo8y
 * @property {BtcCentsSatsUsdPattern} _8yTo10y
 * @property {BtcCentsSatsUsdPattern} _10yTo12y
 * @property {BtcCentsSatsUsdPattern} _12yTo15y
 * @property {BtcCentsSatsUsdPattern} over15y
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Address
 * @property {MetricsTree_Cohorts_Address_OverAmount} overAmount
 * @property {MetricsTree_Cohorts_Address_AmountRange} amountRange
 * @property {MetricsTree_Cohorts_Address_UnderAmount} underAmount
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Address_OverAmount
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1sat
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kBtc
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Address_AmountRange
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _0sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1satTo10sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10satsTo100sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100satsTo1kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kSatsTo10kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kSatsTo100kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100kSatsTo1mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1mSatsTo10mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10mSatsTo1btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1btcTo10btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10btcTo100btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100btcTo1kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kBtcTo10kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kBtcTo100kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} over100kBtc
 */

/**
 * @typedef {Object} MetricsTree_Cohorts_Address_UnderAmount
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100sats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100kSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10mSats
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100btc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _1kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _10kBtc
 * @property {AddressOutputsRealizedSupplyUnrealizedPattern} _100kBtc
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
    "halving",
    "epoch",
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
    "under1h": {
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
    "over15y": {
      "id": "over_15y_old",
      "short": "15y+",
      "long": "15+ Years Old"
    }
  });

  UNDER_AGE_NAMES = /** @type {const} */ ({
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

  OVER_AGE_NAMES = /** @type {const} */ ({
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
      "id": "0sats",
      "short": "0 sats",
      "long": "0 Sats"
    },
    "_1satTo10sats": {
      "id": "1sat_to_10sats",
      "short": "1-10 sats",
      "long": "1-10 Sats"
    },
    "_10satsTo100sats": {
      "id": "10sats_to_100sats",
      "short": "10-100 sats",
      "long": "10-100 Sats"
    },
    "_100satsTo1kSats": {
      "id": "100sats_to_1k_sats",
      "short": "100-1k sats",
      "long": "100-1K Sats"
    },
    "_1kSatsTo10kSats": {
      "id": "1k_sats_to_10k_sats",
      "short": "1k-10k sats",
      "long": "1K-10K Sats"
    },
    "_10kSatsTo100kSats": {
      "id": "10k_sats_to_100k_sats",
      "short": "10k-100k sats",
      "long": "10K-100K Sats"
    },
    "_100kSatsTo1mSats": {
      "id": "100k_sats_to_1m_sats",
      "short": "100k-1M sats",
      "long": "100K-1M Sats"
    },
    "_1mSatsTo10mSats": {
      "id": "1m_sats_to_10m_sats",
      "short": "1M-10M sats",
      "long": "1M-10M Sats"
    },
    "_10mSatsTo1btc": {
      "id": "10m_sats_to_1btc",
      "short": "0.1-1 BTC",
      "long": "0.1-1 BTC"
    },
    "_1btcTo10btc": {
      "id": "1btc_to_10btc",
      "short": "1-10 BTC",
      "long": "1-10 BTC"
    },
    "_10btcTo100btc": {
      "id": "10btc_to_100btc",
      "short": "10-100 BTC",
      "long": "10-100 BTC"
    },
    "_100btcTo1kBtc": {
      "id": "100btc_to_1k_btc",
      "short": "100-1k BTC",
      "long": "100-1K BTC"
    },
    "_1kBtcTo10kBtc": {
      "id": "1k_btc_to_10k_btc",
      "short": "1k-10k BTC",
      "long": "1K-10K BTC"
    },
    "_10kBtcTo100kBtc": {
      "id": "10k_btc_to_100k_btc",
      "short": "10k-100k BTC",
      "long": "10K-100K BTC"
    },
    "over100kBtc": {
      "id": "over_100k_btc",
      "short": "100k+ BTC",
      "long": "100K+ BTC"
    }
  });

  OVER_AMOUNT_NAMES = /** @type {const} */ ({
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

  UNDER_AMOUNT_NAMES = /** @type {const} */ ({
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
          raw: createMetricPattern18(this, 'difficulty'),
          base: createMetricPattern2(this, 'difficulty'),
          asHash: createMetricPattern1(this, 'difficulty_as_hash'),
          adjustment: createBpsPercentRatioPattern2(this, 'difficulty_adjustment'),
          epoch: createMetricPattern1(this, 'difficulty_epoch'),
          blocksBeforeNext: createMetricPattern1(this, 'blocks_before_next_difficulty_adjustment'),
          daysBeforeNext: createMetricPattern1(this, 'days_before_next_difficulty_adjustment'),
        },
        time: {
          timestamp: createMetricPattern1(this, 'timestamp'),
          date: createMetricPattern18(this, 'date'),
          timestampMonotonic: createMetricPattern18(this, 'timestamp_monotonic'),
        },
        size: {
          total: createMetricPattern18(this, 'total_size'),
          cumulative: createMetricPattern1(this, 'block_size_cumulative'),
          sum: create_1m1w1y24hPattern(this, 'block_size_sum'),
          average: create_1m1w1y24hPattern(this, 'block_size_average'),
          min: create_1m1w1y24hPattern(this, 'block_size_min'),
          max: create_1m1w1y24hPattern(this, 'block_size_max'),
          pct10: create_1m1w1y24hPattern(this, 'block_size_pct10'),
          pct25: create_1m1w1y24hPattern(this, 'block_size_pct25'),
          median: create_1m1w1y24hPattern(this, 'block_size_median'),
          pct75: create_1m1w1y24hPattern(this, 'block_size_pct75'),
          pct90: create_1m1w1y24hPattern(this, 'block_size_pct90'),
        },
        weight: {
          raw: createMetricPattern18(this, 'block_weight'),
          cumulative: createMetricPattern1(this, 'block_weight_cumulative'),
          sum: create_1m1w1y24hPattern(this, 'block_weight_sum'),
          average: create_1m1w1y24hPattern(this, 'block_weight_average'),
          min: create_1m1w1y24hPattern(this, 'block_weight_min'),
          max: create_1m1w1y24hPattern(this, 'block_weight_max'),
          pct10: create_1m1w1y24hPattern(this, 'block_weight_pct10'),
          pct25: create_1m1w1y24hPattern(this, 'block_weight_pct25'),
          median: create_1m1w1y24hPattern(this, 'block_weight_median'),
          pct75: create_1m1w1y24hPattern(this, 'block_weight_pct75'),
          pct90: create_1m1w1y24hPattern(this, 'block_weight_pct90'),
        },
        count: {
          target: createMetricPattern1(this, 'block_count_target'),
          total: createBaseCumulativeSumPattern2(this, 'block_count'),
        },
        lookback: {
          _1h: createMetricPattern18(this, 'height_1h_ago'),
          _24h: createMetricPattern18(this, 'height_24h_ago'),
          _3d: createMetricPattern18(this, 'height_3d_ago'),
          _1w: createMetricPattern18(this, 'height_1w_ago'),
          _8d: createMetricPattern18(this, 'height_8d_ago'),
          _9d: createMetricPattern18(this, 'height_9d_ago'),
          _12d: createMetricPattern18(this, 'height_12d_ago'),
          _13d: createMetricPattern18(this, 'height_13d_ago'),
          _2w: createMetricPattern18(this, 'height_2w_ago'),
          _21d: createMetricPattern18(this, 'height_21d_ago'),
          _26d: createMetricPattern18(this, 'height_26d_ago'),
          _1m: createMetricPattern18(this, 'height_1m_ago'),
          _34d: createMetricPattern18(this, 'height_34d_ago'),
          _55d: createMetricPattern18(this, 'height_55d_ago'),
          _2m: createMetricPattern18(this, 'height_2m_ago'),
          _9w: createMetricPattern18(this, 'height_9w_ago'),
          _12w: createMetricPattern18(this, 'height_12w_ago'),
          _89d: createMetricPattern18(this, 'height_89d_ago'),
          _3m: createMetricPattern18(this, 'height_3m_ago'),
          _14w: createMetricPattern18(this, 'height_14w_ago'),
          _111d: createMetricPattern18(this, 'height_111d_ago'),
          _144d: createMetricPattern18(this, 'height_144d_ago'),
          _6m: createMetricPattern18(this, 'height_6m_ago'),
          _26w: createMetricPattern18(this, 'height_26w_ago'),
          _200d: createMetricPattern18(this, 'height_200d_ago'),
          _9m: createMetricPattern18(this, 'height_9m_ago'),
          _350d: createMetricPattern18(this, 'height_350d_ago'),
          _12m: createMetricPattern18(this, 'height_12m_ago'),
          _1y: createMetricPattern18(this, 'height_1y_ago'),
          _14m: createMetricPattern18(this, 'height_14m_ago'),
          _2y: createMetricPattern18(this, 'height_2y_ago'),
          _26m: createMetricPattern18(this, 'height_26m_ago'),
          _3y: createMetricPattern18(this, 'height_3y_ago'),
          _200w: createMetricPattern18(this, 'height_200w_ago'),
          _4y: createMetricPattern18(this, 'height_4y_ago'),
          _5y: createMetricPattern18(this, 'height_5y_ago'),
          _6y: createMetricPattern18(this, 'height_6y_ago'),
          _8y: createMetricPattern18(this, 'height_8y_ago'),
          _9y: createMetricPattern18(this, 'height_9y_ago'),
          _10y: createMetricPattern18(this, 'height_10y_ago'),
          _12y: createMetricPattern18(this, 'height_12y_ago'),
          _14y: createMetricPattern18(this, 'height_14y_ago'),
          _26y: createMetricPattern18(this, 'height_26y_ago'),
        },
        interval: create_1m1w1y24hHeightPattern(this, 'block_interval'),
        halving: {
          epoch: createMetricPattern1(this, 'halving_epoch'),
          blocksBeforeNext: createMetricPattern1(this, 'blocks_before_next_halving'),
          daysBeforeNext: createMetricPattern1(this, 'days_before_next_halving'),
        },
        vbytes: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'block_vbytes'),
        fullness: {
          bps: create_1m1w1y24hHeightPattern(this, 'block_fullness_bps'),
          ratio: createMetricPattern1(this, 'block_fullness_ratio'),
          percent: createMetricPattern1(this, 'block_fullness'),
        },
      },
      transactions: {
        raw: {
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
        },
        count: {
          total: createAverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(this, 'tx_count'),
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
          v1: createBaseCumulativeSumPattern(this, 'tx_v1'),
          v2: createBaseCumulativeSumPattern(this, 'tx_v2'),
          v3: createBaseCumulativeSumPattern(this, 'tx_v3'),
        },
        volume: {
          sentSum: createBaseCumulativeSumPattern4(this, 'sent_sum'),
          receivedSum: createBaseCumulativeSumPattern4(this, 'received_sum'),
          txPerSec: createMetricPattern1(this, 'tx_per_sec'),
          outputsPerSec: createMetricPattern1(this, 'outputs_per_sec'),
          inputsPerSec: createMetricPattern1(this, 'inputs_per_sec'),
        },
      },
      inputs: {
        raw: {
          firstTxinindex: createMetricPattern18(this, 'first_txinindex'),
          outpoint: createMetricPattern20(this, 'outpoint'),
          txindex: createMetricPattern20(this, 'txindex'),
          outputtype: createMetricPattern20(this, 'outputtype'),
          typeindex: createMetricPattern20(this, 'typeindex'),
        },
        spent: {
          txoutindex: createMetricPattern20(this, 'txoutindex'),
          value: createMetricPattern20(this, 'value'),
        },
        count: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'input_count'),
      },
      outputs: {
        raw: {
          firstTxoutindex: createMetricPattern18(this, 'first_txoutindex'),
          value: createMetricPattern21(this, 'value'),
          outputtype: createMetricPattern21(this, 'outputtype'),
          typeindex: createMetricPattern21(this, 'typeindex'),
          txindex: createMetricPattern21(this, 'txindex'),
        },
        spent: {
          txinindex: createMetricPattern21(this, 'txinindex'),
        },
        count: {
          total: createAverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(this, 'output_count'),
          unspent: createMetricPattern1(this, 'exact_utxo_count'),
        },
      },
      addresses: {
        raw: {
          p2pk65: {
            firstIndex: createMetricPattern18(this, 'first_p2pk65addressindex'),
            bytes: createMetricPattern27(this, 'p2pk65bytes'),
          },
          p2pk33: {
            firstIndex: createMetricPattern18(this, 'first_p2pk33addressindex'),
            bytes: createMetricPattern26(this, 'p2pk33bytes'),
          },
          p2pkh: {
            firstIndex: createMetricPattern18(this, 'first_p2pkhaddressindex'),
            bytes: createMetricPattern28(this, 'p2pkhbytes'),
          },
          p2sh: {
            firstIndex: createMetricPattern18(this, 'first_p2shaddressindex'),
            bytes: createMetricPattern29(this, 'p2shbytes'),
          },
          p2wpkh: {
            firstIndex: createMetricPattern18(this, 'first_p2wpkhaddressindex'),
            bytes: createMetricPattern31(this, 'p2wpkhbytes'),
          },
          p2wsh: {
            firstIndex: createMetricPattern18(this, 'first_p2wshaddressindex'),
            bytes: createMetricPattern32(this, 'p2wshbytes'),
          },
          p2tr: {
            firstIndex: createMetricPattern18(this, 'first_p2traddressindex'),
            bytes: createMetricPattern30(this, 'p2trbytes'),
          },
          p2a: {
            firstIndex: createMetricPattern18(this, 'first_p2aaddressindex'),
            bytes: createMetricPattern24(this, 'p2abytes'),
          },
        },
        indexes: {
          p2a: createMetricPattern24(this, 'anyaddressindex'),
          p2pk33: createMetricPattern26(this, 'anyaddressindex'),
          p2pk65: createMetricPattern27(this, 'anyaddressindex'),
          p2pkh: createMetricPattern28(this, 'anyaddressindex'),
          p2sh: createMetricPattern29(this, 'anyaddressindex'),
          p2tr: createMetricPattern30(this, 'anyaddressindex'),
          p2wpkh: createMetricPattern31(this, 'anyaddressindex'),
          p2wsh: createMetricPattern32(this, 'anyaddressindex'),
          funded: createMetricPattern34(this, 'funded_address_index'),
          empty: createMetricPattern35(this, 'empty_address_index'),
        },
        data: {
          funded: createMetricPattern34(this, 'fundedaddressdata'),
          empty: createMetricPattern35(this, 'emptyaddressdata'),
        },
        funded: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(this, 'address_count'),
        empty: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(this, 'empty_address_count'),
        activity: {
          all: createBothReactivatedReceivingSendingPattern(this, 'address_activity'),
          p2pk65: createBothReactivatedReceivingSendingPattern(this, 'p2pk65_address_activity'),
          p2pk33: createBothReactivatedReceivingSendingPattern(this, 'p2pk33_address_activity'),
          p2pkh: createBothReactivatedReceivingSendingPattern(this, 'p2pkh_address_activity'),
          p2sh: createBothReactivatedReceivingSendingPattern(this, 'p2sh_address_activity'),
          p2wpkh: createBothReactivatedReceivingSendingPattern(this, 'p2wpkh_address_activity'),
          p2wsh: createBothReactivatedReceivingSendingPattern(this, 'p2wsh_address_activity'),
          p2tr: createBothReactivatedReceivingSendingPattern(this, 'p2tr_address_activity'),
          p2a: createBothReactivatedReceivingSendingPattern(this, 'p2a_address_activity'),
        },
        total: createAllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(this, 'total_address_count'),
        new: {
          all: createBaseCumulativeSumPattern(this, 'new_address_count'),
          p2pk65: createBaseCumulativeSumPattern(this, 'p2pk65_new_address_count'),
          p2pk33: createBaseCumulativeSumPattern(this, 'p2pk33_new_address_count'),
          p2pkh: createBaseCumulativeSumPattern(this, 'p2pkh_new_address_count'),
          p2sh: createBaseCumulativeSumPattern(this, 'p2sh_new_address_count'),
          p2wpkh: createBaseCumulativeSumPattern(this, 'p2wpkh_new_address_count'),
          p2wsh: createBaseCumulativeSumPattern(this, 'p2wsh_new_address_count'),
          p2tr: createBaseCumulativeSumPattern(this, 'p2tr_new_address_count'),
          p2a: createBaseCumulativeSumPattern(this, 'p2a_new_address_count'),
        },
        delta: {
          all: createChangeRatePattern(this, 'address_count'),
          p2pk65: createChangeRatePattern(this, 'p2pk65_address_count'),
          p2pk33: createChangeRatePattern(this, 'p2pk33_address_count'),
          p2pkh: createChangeRatePattern(this, 'p2pkh_address_count'),
          p2sh: createChangeRatePattern(this, 'p2sh_address_count'),
          p2wpkh: createChangeRatePattern(this, 'p2wpkh_address_count'),
          p2wsh: createChangeRatePattern(this, 'p2wsh_address_count'),
          p2tr: createChangeRatePattern(this, 'p2tr_address_count'),
          p2a: createChangeRatePattern(this, 'p2a_address_count'),
        },
      },
      scripts: {
        raw: {
          empty: {
            firstIndex: createMetricPattern18(this, 'first_emptyoutputindex'),
            toTxindex: createMetricPattern22(this, 'txindex'),
          },
          opreturn: {
            firstIndex: createMetricPattern18(this, 'first_opreturnindex'),
            toTxindex: createMetricPattern23(this, 'txindex'),
          },
          p2ms: {
            firstIndex: createMetricPattern18(this, 'first_p2msoutputindex'),
            toTxindex: createMetricPattern25(this, 'txindex'),
          },
          unknown: {
            firstIndex: createMetricPattern18(this, 'first_unknownoutputindex'),
            toTxindex: createMetricPattern33(this, 'txindex'),
          },
        },
        count: {
          p2a: createBaseCumulativeSumPattern(this, 'p2a_count'),
          p2ms: createBaseCumulativeSumPattern(this, 'p2ms_count'),
          p2pk33: createBaseCumulativeSumPattern(this, 'p2pk33_count'),
          p2pk65: createBaseCumulativeSumPattern(this, 'p2pk65_count'),
          p2pkh: createBaseCumulativeSumPattern(this, 'p2pkh_count'),
          p2sh: createBaseCumulativeSumPattern(this, 'p2sh_count'),
          p2tr: createBaseCumulativeSumPattern(this, 'p2tr_count'),
          p2wpkh: createBaseCumulativeSumPattern(this, 'p2wpkh_count'),
          p2wsh: createBaseCumulativeSumPattern(this, 'p2wsh_count'),
          opreturn: createBaseCumulativeSumPattern(this, 'opreturn_count'),
          emptyoutput: createBaseCumulativeSumPattern(this, 'emptyoutput_count'),
          unknownoutput: createBaseCumulativeSumPattern(this, 'unknownoutput_count'),
          segwit: createBaseCumulativeSumPattern(this, 'segwit_count'),
        },
        value: {
          opreturn: {
            base: createBtcCentsSatsUsdPattern(this, 'opreturn_value'),
            cumulative: createBtcCentsSatsUsdPattern(this, 'opreturn_value_cumulative'),
          },
        },
        adoption: {
          taproot: createBpsPercentRatioPattern4(this, 'taproot_adoption'),
          segwit: createBpsPercentRatioPattern4(this, 'segwit_adoption'),
        },
      },
      mining: {
        rewards: {
          coinbase: createBaseCumulativeSumPattern4(this, 'coinbase'),
          subsidy: {
            base: createBtcCentsSatsUsdPattern(this, 'subsidy'),
            cumulative: createBtcCentsSatsUsdPattern(this, 'subsidy_cumulative'),
            dominance: create_1m1w1y24hBpsPercentRatioPattern(this, 'subsidy_dominance'),
            sma1y: createCentsUsdPattern2(this, 'subsidy_sma_1y'),
          },
          fees: {
            base: createBtcCentsSatsUsdPattern(this, 'fees'),
            cumulative: createBtcCentsSatsUsdPattern(this, 'fees_cumulative'),
            sum: create_1m1w1y24hPattern6(this, 'fees_sum'),
            _24h: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'fees_24h'),
            _1w: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'fees_1w'),
            _1m: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'fees_1m'),
            _1y: createAverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(this, 'fees_1y'),
            dominance: create_1m1w1y24hBpsPercentRatioPattern(this, 'fee_dominance'),
            ratioMultiple: {
              _24h: createBpsRatioPattern2(this, 'fee_ratio_multiple_24h'),
              _1w: createBpsRatioPattern2(this, 'fee_ratio_multiple_1w'),
              _1m: createBpsRatioPattern2(this, 'fee_ratio_multiple_1m'),
              _1y: createBpsRatioPattern2(this, 'fee_ratio_multiple_1y'),
            },
          },
          unclaimed: createBaseCumulativeSumPattern4(this, 'unclaimed_rewards'),
        },
        hashrate: {
          rate: {
            base: createMetricPattern1(this, 'hash_rate'),
            sma: {
              _1w: createMetricPattern1(this, 'hash_rate_sma_1w'),
              _1m: createMetricPattern1(this, 'hash_rate_sma_1m'),
              _2m: createMetricPattern1(this, 'hash_rate_sma_2m'),
              _1y: createMetricPattern1(this, 'hash_rate_sma_1y'),
            },
            ath: createMetricPattern1(this, 'hash_rate_ath'),
            drawdown: createBpsPercentRatioPattern5(this, 'hash_rate_drawdown'),
          },
          price: createPhsReboundThsPattern(this, 'hash_price'),
          value: createPhsReboundThsPattern(this, 'hash_value'),
        },
      },
      cointime: {
        activity: {
          coinblocksCreated: createBaseCumulativeSumPattern(this, 'coinblocks_created'),
          coinblocksStored: createBaseCumulativeSumPattern(this, 'coinblocks_stored'),
          liveliness: createMetricPattern1(this, 'liveliness'),
          vaultedness: createMetricPattern1(this, 'vaultedness'),
          ratio: createMetricPattern1(this, 'activity_to_vaultedness_ratio'),
        },
        supply: {
          vaulted: createBtcCentsSatsUsdPattern(this, 'vaulted_supply'),
          active: createBtcCentsSatsUsdPattern(this, 'active_supply'),
        },
        value: {
          destroyed: createBaseCumulativeSumPattern(this, 'cointime_value_destroyed'),
          created: createBaseCumulativeSumPattern(this, 'cointime_value_created'),
          stored: createBaseCumulativeSumPattern(this, 'cointime_value_stored'),
          vocdd: createBaseCumulativeSumPattern(this, 'vocdd'),
        },
        cap: {
          thermo: createCentsUsdPattern2(this, 'thermo_cap'),
          investor: createCentsUsdPattern2(this, 'investor_cap'),
          vaulted: createCentsUsdPattern2(this, 'vaulted_cap'),
          active: createCentsUsdPattern2(this, 'active_cap'),
          cointime: createCentsUsdPattern2(this, 'cointime_cap'),
          aviv: createBpsRatioPattern2(this, 'aviv_ratio'),
        },
        prices: {
          vaulted: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'vaulted_price'),
          active: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'active_price'),
          trueMarketMean: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'true_market_mean'),
          cointime: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'cointime_price'),
          transfer: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'transfer_price'),
          balanced: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'balanced_price'),
          terminal: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'terminal_price'),
          delta: createBpsCentsPercentilesRatioSatsUsdPattern(this, 'delta_price'),
          cumulativeMarketCap: createMetricPattern1(this, 'cumulative_market_cap'),
        },
        adjusted: {
          inflationRate: createBpsPercentRatioPattern2(this, 'cointime_adj_inflation_rate'),
          txVelocityBtc: createMetricPattern1(this, 'cointime_adj_tx_velocity_btc'),
          txVelocityUsd: createMetricPattern1(this, 'cointime_adj_tx_velocity_usd'),
        },
        reserveRisk: {
          value: createMetricPattern1(this, 'reserve_risk'),
          vocddMedian1y: createMetricPattern18(this, 'vocdd_median_1y'),
          hodlBank: createMetricPattern18(this, 'hodl_bank'),
        },
        coinblocksDestroyed: {
          base: createMetricPattern1(this, 'coinblocks_destroyed'),
          cumulative: createMetricPattern1(this, 'coinblocks_destroyed_cumulative'),
        },
      },
      constants: {
        _0: createMetricPattern1(this, 'constant_0'),
        _1: createMetricPattern1(this, 'constant_1'),
        _2: createMetricPattern1(this, 'constant_2'),
        _3: createMetricPattern1(this, 'constant_3'),
        _4: createMetricPattern1(this, 'constant_4'),
        _20: createMetricPattern1(this, 'constant_20'),
        _30: createMetricPattern1(this, 'constant_30'),
        _382: createMetricPattern1(this, 'constant_38_2'),
        _50: createMetricPattern1(this, 'constant_50'),
        _618: createMetricPattern1(this, 'constant_61_8'),
        _70: createMetricPattern1(this, 'constant_70'),
        _80: createMetricPattern1(this, 'constant_80'),
        _100: createMetricPattern1(this, 'constant_100'),
        _600: createMetricPattern1(this, 'constant_600'),
        minus1: createMetricPattern1(this, 'constant_minus_1'),
        minus2: createMetricPattern1(this, 'constant_minus_2'),
        minus3: createMetricPattern1(this, 'constant_minus_3'),
        minus4: createMetricPattern1(this, 'constant_minus_4'),
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
          epoch: createMetricPattern18(this, 'epoch'),
          halving: createMetricPattern18(this, 'halving'),
          week1: createMetricPattern18(this, 'week1'),
          month1: createMetricPattern18(this, 'month1'),
          month3: createMetricPattern18(this, 'month3'),
          month6: createMetricPattern18(this, 'month6'),
          year1: createMetricPattern18(this, 'year1'),
          year10: createMetricPattern18(this, 'year10'),
          txindexCount: createMetricPattern18(this, 'txindex_count'),
        },
        epoch: {
          identity: createMetricPattern17(this, 'epoch'),
          firstHeight: createMetricPattern17(this, 'first_height'),
          heightCount: createMetricPattern17(this, 'height_count'),
        },
        halving: {
          identity: createMetricPattern16(this, 'halving'),
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
      indicators: {
        puellMultiple: createBpsRatioPattern2(this, 'puell_multiple'),
        nvt: createBpsRatioPattern2(this, 'nvt'),
        gini: createBpsPercentRatioPattern4(this, 'gini'),
        rhodlRatio: createBpsRatioPattern2(this, 'rhodl_ratio'),
        thermocapMultiple: createBpsRatioPattern2(this, 'thermocap_multiple'),
        coindaysDestroyedSupplyAdjusted: createMetricPattern1(this, 'coindays_destroyed_supply_adjusted'),
        coinyearsDestroyedSupplyAdjusted: createMetricPattern1(this, 'coinyears_destroyed_supply_adjusted'),
        dormancy: {
          supplyAdjusted: createMetricPattern1(this, 'dormancy_supply_adjusted'),
          flow: createMetricPattern1(this, 'dormancy_flow'),
        },
        stockToFlow: createMetricPattern1(this, 'stock_to_flow'),
        sellerExhaustionConstant: createMetricPattern1(this, 'seller_exhaustion_constant'),
      },
      market: {
        ath: {
          high: createCentsSatsUsdPattern(this, 'price_ath'),
          drawdown: createBpsPercentRatioPattern5(this, 'price_drawdown'),
          daysSince: createMetricPattern1(this, 'days_since_price_ath'),
          yearsSince: createMetricPattern2(this, 'years_since_price_ath'),
          maxDaysBetween: createMetricPattern1(this, 'max_days_between_price_ath'),
          maxYearsBetween: createMetricPattern2(this, 'max_years_between_price_ath'),
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
          periods: {
            _24h: createBpsPercentRatioPattern2(this, 'price_return_24h'),
            _1w: createBpsPercentRatioPattern2(this, 'price_return_1w'),
            _1m: createBpsPercentRatioPattern2(this, 'price_return_1m'),
            _3m: createBpsPercentRatioPattern2(this, 'price_return_3m'),
            _6m: createBpsPercentRatioPattern2(this, 'price_return_6m'),
            _1y: createBpsPercentRatioPattern2(this, 'price_return_1y'),
            _2y: createBpsPercentRatioPattern2(this, 'price_return_2y'),
            _3y: createBpsPercentRatioPattern2(this, 'price_return_3y'),
            _4y: createBpsPercentRatioPattern2(this, 'price_return_4y'),
            _5y: createBpsPercentRatioPattern2(this, 'price_return_5y'),
            _6y: createBpsPercentRatioPattern2(this, 'price_return_6y'),
            _8y: createBpsPercentRatioPattern2(this, 'price_return_8y'),
            _10y: createBpsPercentRatioPattern2(this, 'price_return_10y'),
          },
          cagr: create_10y2y3y4y5y6y8yPattern(this, 'price_cagr'),
          sd24h: {
            _1w: {
              sma: createMetricPattern1(this, 'price_return_24h_sma_1w'),
              sd: createMetricPattern1(this, 'price_return_24h_sd_1w'),
            },
            _1m: {
              sma: createMetricPattern1(this, 'price_return_24h_sma_1m'),
              sd: createMetricPattern1(this, 'price_return_24h_sd_1m'),
            },
            _1y: createSdSmaPattern(this, 'price_return_24h'),
          },
        },
        volatility: {
          _1w: createMetricPattern1(this, 'price_volatility_1w'),
          _1m: createMetricPattern1(this, 'price_volatility_1m'),
          _1y: createMetricPattern1(this, 'price_volatility_1y'),
        },
        range: {
          min: create_1m1w1y2wPattern(this, 'price_min'),
          max: create_1m1w1y2wPattern(this, 'price_max'),
          trueRange: createMetricPattern1(this, 'price_true_range'),
          trueRangeSum2w: createMetricPattern1(this, 'price_true_range_sum_2w'),
          choppinessIndex2w: createBpsPercentRatioPattern4(this, 'price_choppiness_index_2w'),
        },
        movingAverage: {
          sma: {
            _1w: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_1w'),
            _8d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_8d'),
            _13d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_13d'),
            _21d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_21d'),
            _1m: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_1m'),
            _34d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_34d'),
            _55d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_55d'),
            _89d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_89d'),
            _111d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_111d'),
            _144d: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_144d'),
            _200d: {
              cents: createMetricPattern1(this, 'price_sma_200d_cents'),
              usd: createMetricPattern1(this, 'price_sma_200d'),
              sats: createMetricPattern1(this, 'price_sma_200d_sats'),
              bps: createMetricPattern1(this, 'price_sma_200d_ratio_bps'),
              ratio: createMetricPattern1(this, 'price_sma_200d_ratio'),
              x24: {
                cents: createMetricPattern1(this, 'price_sma_200d_x2_4_cents'),
                usd: createMetricPattern1(this, 'price_sma_200d_x2_4_usd'),
                sats: createMetricPattern1(this, 'price_sma_200d_x2_4_sats'),
              },
              x08: {
                cents: createMetricPattern1(this, 'price_sma_200d_x0_8_cents'),
                usd: createMetricPattern1(this, 'price_sma_200d_x0_8_usd'),
                sats: createMetricPattern1(this, 'price_sma_200d_x0_8_sats'),
              },
            },
            _350d: {
              cents: createMetricPattern1(this, 'price_sma_350d_cents'),
              usd: createMetricPattern1(this, 'price_sma_350d'),
              sats: createMetricPattern1(this, 'price_sma_350d_sats'),
              bps: createMetricPattern1(this, 'price_sma_350d_ratio_bps'),
              ratio: createMetricPattern1(this, 'price_sma_350d_ratio'),
              x2: {
                cents: createMetricPattern1(this, 'price_sma_350d_x2_cents'),
                usd: createMetricPattern1(this, 'price_sma_350d_x2_usd'),
                sats: createMetricPattern1(this, 'price_sma_350d_x2_sats'),
              },
            },
            _1y: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_1y'),
            _2y: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_2y'),
            _200w: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_200w'),
            _4y: createBpsCentsRatioSatsUsdPattern(this, 'price_sma_4y'),
          },
          ema: {
            _1w: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_1w'),
            _8d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_8d'),
            _12d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_12d'),
            _13d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_13d'),
            _21d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_21d'),
            _26d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_26d'),
            _1m: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_1m'),
            _34d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_34d'),
            _55d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_55d'),
            _89d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_89d'),
            _144d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_144d'),
            _200d: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_200d'),
            _1y: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_1y'),
            _2y: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_2y'),
            _200w: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_200w'),
            _4y: createBpsCentsRatioSatsUsdPattern(this, 'price_ema_4y'),
          },
        },
        dca: {
          satsPerDay: createMetricPattern18(this, 'dca_sats_per_day'),
          period: {
            stack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'dca_stack'),
            costBasis: {
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
            return: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'dca_return'),
            cagr: create_10y2y3y4y5y6y8yPattern(this, 'dca_cagr'),
            lumpSumStack: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern3(this, 'lump_sum_stack'),
            lumpSumReturn: create_10y1m1w1y2y3m3y4y5y6m6y8yPattern2(this, 'lump_sum_return'),
          },
          class: {
            stack: {
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
            costBasis: {
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
            return: {
              from2015: createBpsPercentRatioPattern2(this, 'dca_return_from_2015'),
              from2016: createBpsPercentRatioPattern2(this, 'dca_return_from_2016'),
              from2017: createBpsPercentRatioPattern2(this, 'dca_return_from_2017'),
              from2018: createBpsPercentRatioPattern2(this, 'dca_return_from_2018'),
              from2019: createBpsPercentRatioPattern2(this, 'dca_return_from_2019'),
              from2020: createBpsPercentRatioPattern2(this, 'dca_return_from_2020'),
              from2021: createBpsPercentRatioPattern2(this, 'dca_return_from_2021'),
              from2022: createBpsPercentRatioPattern2(this, 'dca_return_from_2022'),
              from2023: createBpsPercentRatioPattern2(this, 'dca_return_from_2023'),
              from2024: createBpsPercentRatioPattern2(this, 'dca_return_from_2024'),
              from2025: createBpsPercentRatioPattern2(this, 'dca_return_from_2025'),
              from2026: createBpsPercentRatioPattern2(this, 'dca_return_from_2026'),
            },
          },
        },
        technical: {
          rsi: {
            _24h: createAverageGainsLossesRsiStochPattern(this, 'rsi'),
            _1w: {
              gains: createMetricPattern1(this, 'rsi_gains_1w'),
              losses: createMetricPattern1(this, 'rsi_losses_1w'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1w'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1w'),
              rsi: createBpsPercentRatioPattern4(this, 'rsi_1w'),
              rsiMin: createBpsPercentRatioPattern4(this, 'rsi_min_1w'),
              rsiMax: createBpsPercentRatioPattern4(this, 'rsi_max_1w'),
              stochRsi: createBpsPercentRatioPattern4(this, 'rsi_stoch_1w'),
              stochRsiK: createBpsPercentRatioPattern4(this, 'rsi_stoch_k_1w'),
              stochRsiD: createBpsPercentRatioPattern4(this, 'rsi_stoch_d_1w'),
            },
            _1m: {
              gains: createMetricPattern1(this, 'rsi_gains_1m'),
              losses: createMetricPattern1(this, 'rsi_losses_1m'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1m'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1m'),
              rsi: createBpsPercentRatioPattern4(this, 'rsi_1m'),
              rsiMin: createBpsPercentRatioPattern4(this, 'rsi_min_1m'),
              rsiMax: createBpsPercentRatioPattern4(this, 'rsi_max_1m'),
              stochRsi: createBpsPercentRatioPattern4(this, 'rsi_stoch_1m'),
              stochRsiK: createBpsPercentRatioPattern4(this, 'rsi_stoch_k_1m'),
              stochRsiD: createBpsPercentRatioPattern4(this, 'rsi_stoch_d_1m'),
            },
            _1y: {
              gains: createMetricPattern1(this, 'rsi_gains_1y'),
              losses: createMetricPattern1(this, 'rsi_losses_1y'),
              averageGain: createMetricPattern1(this, 'rsi_average_gain_1y'),
              averageLoss: createMetricPattern1(this, 'rsi_average_loss_1y'),
              rsi: createBpsPercentRatioPattern4(this, 'rsi_1y'),
              rsiMin: createBpsPercentRatioPattern4(this, 'rsi_min_1y'),
              rsiMax: createBpsPercentRatioPattern4(this, 'rsi_max_1y'),
              stochRsi: createBpsPercentRatioPattern4(this, 'rsi_stoch_1y'),
              stochRsiK: createBpsPercentRatioPattern4(this, 'rsi_stoch_k_1y'),
              stochRsiD: createBpsPercentRatioPattern4(this, 'rsi_stoch_d_1y'),
            },
          },
          stochK: createBpsPercentRatioPattern4(this, 'stoch_k'),
          stochD: createBpsPercentRatioPattern4(this, 'stoch_d'),
          piCycle: createBpsRatioPattern2(this, 'pi_cycle'),
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
        },
      },
      pools: {
        heightToPool: createMetricPattern18(this, 'pool'),
        major: {
          unknown: createBlocksDominanceRewardsPattern(this, 'unknown'),
          luxor: createBlocksDominanceRewardsPattern(this, 'luxor'),
          btccom: createBlocksDominanceRewardsPattern(this, 'btccom'),
          btctop: createBlocksDominanceRewardsPattern(this, 'btctop'),
          btcguild: createBlocksDominanceRewardsPattern(this, 'btcguild'),
          eligius: createBlocksDominanceRewardsPattern(this, 'eligius'),
          f2pool: createBlocksDominanceRewardsPattern(this, 'f2pool'),
          braiinspool: createBlocksDominanceRewardsPattern(this, 'braiinspool'),
          antpool: createBlocksDominanceRewardsPattern(this, 'antpool'),
          btcc: createBlocksDominanceRewardsPattern(this, 'btcc'),
          bwpool: createBlocksDominanceRewardsPattern(this, 'bwpool'),
          bitfury: createBlocksDominanceRewardsPattern(this, 'bitfury'),
          viabtc: createBlocksDominanceRewardsPattern(this, 'viabtc'),
          poolin: createBlocksDominanceRewardsPattern(this, 'poolin'),
          spiderpool: createBlocksDominanceRewardsPattern(this, 'spiderpool'),
          binancepool: createBlocksDominanceRewardsPattern(this, 'binancepool'),
          foundryusa: createBlocksDominanceRewardsPattern(this, 'foundryusa'),
          sbicrypto: createBlocksDominanceRewardsPattern(this, 'sbicrypto'),
          marapool: createBlocksDominanceRewardsPattern(this, 'marapool'),
          secpool: createBlocksDominanceRewardsPattern(this, 'secpool'),
          ocean: createBlocksDominanceRewardsPattern(this, 'ocean'),
          whitepool: createBlocksDominanceRewardsPattern(this, 'whitepool'),
        },
        minor: {
          blockfills: createBlocksDominancePattern(this, 'blockfills'),
          ultimuspool: createBlocksDominancePattern(this, 'ultimuspool'),
          terrapool: createBlocksDominancePattern(this, 'terrapool'),
          onethash: createBlocksDominancePattern(this, 'onethash'),
          bitfarms: createBlocksDominancePattern(this, 'bitfarms'),
          huobipool: createBlocksDominancePattern(this, 'huobipool'),
          wayicn: createBlocksDominancePattern(this, 'wayicn'),
          canoepool: createBlocksDominancePattern(this, 'canoepool'),
          bitcoincom: createBlocksDominancePattern(this, 'bitcoincom'),
          pool175btc: createBlocksDominancePattern(this, 'pool175btc'),
          gbminers: createBlocksDominancePattern(this, 'gbminers'),
          axbt: createBlocksDominancePattern(this, 'axbt'),
          asicminer: createBlocksDominancePattern(this, 'asicminer'),
          bitminter: createBlocksDominancePattern(this, 'bitminter'),
          bitcoinrussia: createBlocksDominancePattern(this, 'bitcoinrussia'),
          btcserv: createBlocksDominancePattern(this, 'btcserv'),
          simplecoinus: createBlocksDominancePattern(this, 'simplecoinus'),
          ozcoin: createBlocksDominancePattern(this, 'ozcoin'),
          eclipsemc: createBlocksDominancePattern(this, 'eclipsemc'),
          maxbtc: createBlocksDominancePattern(this, 'maxbtc'),
          triplemining: createBlocksDominancePattern(this, 'triplemining'),
          coinlab: createBlocksDominancePattern(this, 'coinlab'),
          pool50btc: createBlocksDominancePattern(this, 'pool50btc'),
          ghashio: createBlocksDominancePattern(this, 'ghashio'),
          stminingcorp: createBlocksDominancePattern(this, 'stminingcorp'),
          bitparking: createBlocksDominancePattern(this, 'bitparking'),
          mmpool: createBlocksDominancePattern(this, 'mmpool'),
          polmine: createBlocksDominancePattern(this, 'polmine'),
          kncminer: createBlocksDominancePattern(this, 'kncminer'),
          bitalo: createBlocksDominancePattern(this, 'bitalo'),
          hhtt: createBlocksDominancePattern(this, 'hhtt'),
          megabigpower: createBlocksDominancePattern(this, 'megabigpower'),
          mtred: createBlocksDominancePattern(this, 'mtred'),
          nmcbit: createBlocksDominancePattern(this, 'nmcbit'),
          yourbtcnet: createBlocksDominancePattern(this, 'yourbtcnet'),
          givemecoins: createBlocksDominancePattern(this, 'givemecoins'),
          multicoinco: createBlocksDominancePattern(this, 'multicoinco'),
          bcpoolio: createBlocksDominancePattern(this, 'bcpoolio'),
          cointerra: createBlocksDominancePattern(this, 'cointerra'),
          kanopool: createBlocksDominancePattern(this, 'kanopool'),
          solock: createBlocksDominancePattern(this, 'solock'),
          ckpool: createBlocksDominancePattern(this, 'ckpool'),
          nicehash: createBlocksDominancePattern(this, 'nicehash'),
          bitclub: createBlocksDominancePattern(this, 'bitclub'),
          bitcoinaffiliatenetwork: createBlocksDominancePattern(this, 'bitcoinaffiliatenetwork'),
          exxbw: createBlocksDominancePattern(this, 'exxbw'),
          bitsolo: createBlocksDominancePattern(this, 'bitsolo'),
          twentyoneinc: createBlocksDominancePattern(this, 'twentyoneinc'),
          digitalbtc: createBlocksDominancePattern(this, 'digitalbtc'),
          eightbaochi: createBlocksDominancePattern(this, 'eightbaochi'),
          mybtccoinpool: createBlocksDominancePattern(this, 'mybtccoinpool'),
          tbdice: createBlocksDominancePattern(this, 'tbdice'),
          hashpool: createBlocksDominancePattern(this, 'hashpool'),
          nexious: createBlocksDominancePattern(this, 'nexious'),
          bravomining: createBlocksDominancePattern(this, 'bravomining'),
          hotpool: createBlocksDominancePattern(this, 'hotpool'),
          okexpool: createBlocksDominancePattern(this, 'okexpool'),
          bcmonster: createBlocksDominancePattern(this, 'bcmonster'),
          onehash: createBlocksDominancePattern(this, 'onehash'),
          bixin: createBlocksDominancePattern(this, 'bixin'),
          tatmaspool: createBlocksDominancePattern(this, 'tatmaspool'),
          connectbtc: createBlocksDominancePattern(this, 'connectbtc'),
          batpool: createBlocksDominancePattern(this, 'batpool'),
          waterhole: createBlocksDominancePattern(this, 'waterhole'),
          dcexploration: createBlocksDominancePattern(this, 'dcexploration'),
          dcex: createBlocksDominancePattern(this, 'dcex'),
          btpool: createBlocksDominancePattern(this, 'btpool'),
          fiftyeightcoin: createBlocksDominancePattern(this, 'fiftyeightcoin'),
          bitcoinindia: createBlocksDominancePattern(this, 'bitcoinindia'),
          shawnp0wers: createBlocksDominancePattern(this, 'shawnp0wers'),
          phashio: createBlocksDominancePattern(this, 'phashio'),
          rigpool: createBlocksDominancePattern(this, 'rigpool'),
          haozhuzhu: createBlocksDominancePattern(this, 'haozhuzhu'),
          sevenpool: createBlocksDominancePattern(this, 'sevenpool'),
          miningkings: createBlocksDominancePattern(this, 'miningkings'),
          hashbx: createBlocksDominancePattern(this, 'hashbx'),
          dpool: createBlocksDominancePattern(this, 'dpool'),
          rawpool: createBlocksDominancePattern(this, 'rawpool'),
          haominer: createBlocksDominancePattern(this, 'haominer'),
          helix: createBlocksDominancePattern(this, 'helix'),
          bitcoinukraine: createBlocksDominancePattern(this, 'bitcoinukraine'),
          secretsuperstar: createBlocksDominancePattern(this, 'secretsuperstar'),
          tigerpoolnet: createBlocksDominancePattern(this, 'tigerpoolnet'),
          sigmapoolcom: createBlocksDominancePattern(this, 'sigmapoolcom'),
          okpooltop: createBlocksDominancePattern(this, 'okpooltop'),
          hummerpool: createBlocksDominancePattern(this, 'hummerpool'),
          tangpool: createBlocksDominancePattern(this, 'tangpool'),
          bytepool: createBlocksDominancePattern(this, 'bytepool'),
          novablock: createBlocksDominancePattern(this, 'novablock'),
          miningcity: createBlocksDominancePattern(this, 'miningcity'),
          minerium: createBlocksDominancePattern(this, 'minerium'),
          lubiancom: createBlocksDominancePattern(this, 'lubiancom'),
          okkong: createBlocksDominancePattern(this, 'okkong'),
          aaopool: createBlocksDominancePattern(this, 'aaopool'),
          emcdpool: createBlocksDominancePattern(this, 'emcdpool'),
          arkpool: createBlocksDominancePattern(this, 'arkpool'),
          purebtccom: createBlocksDominancePattern(this, 'purebtccom'),
          kucoinpool: createBlocksDominancePattern(this, 'kucoinpool'),
          entrustcharitypool: createBlocksDominancePattern(this, 'entrustcharitypool'),
          okminer: createBlocksDominancePattern(this, 'okminer'),
          titan: createBlocksDominancePattern(this, 'titan'),
          pegapool: createBlocksDominancePattern(this, 'pegapool'),
          btcnuggets: createBlocksDominancePattern(this, 'btcnuggets'),
          cloudhashing: createBlocksDominancePattern(this, 'cloudhashing'),
          digitalxmintsy: createBlocksDominancePattern(this, 'digitalxmintsy'),
          telco214: createBlocksDominancePattern(this, 'telco214'),
          btcpoolparty: createBlocksDominancePattern(this, 'btcpoolparty'),
          multipool: createBlocksDominancePattern(this, 'multipool'),
          transactioncoinmining: createBlocksDominancePattern(this, 'transactioncoinmining'),
          btcdig: createBlocksDominancePattern(this, 'btcdig'),
          trickysbtcpool: createBlocksDominancePattern(this, 'trickysbtcpool'),
          btcmp: createBlocksDominancePattern(this, 'btcmp'),
          eobot: createBlocksDominancePattern(this, 'eobot'),
          unomp: createBlocksDominancePattern(this, 'unomp'),
          patels: createBlocksDominancePattern(this, 'patels'),
          gogreenlight: createBlocksDominancePattern(this, 'gogreenlight'),
          bitcoinindiapool: createBlocksDominancePattern(this, 'bitcoinindiapool'),
          ekanembtc: createBlocksDominancePattern(this, 'ekanembtc'),
          canoe: createBlocksDominancePattern(this, 'canoe'),
          tiger: createBlocksDominancePattern(this, 'tiger'),
          onem1x: createBlocksDominancePattern(this, 'onem1x'),
          zulupool: createBlocksDominancePattern(this, 'zulupool'),
          wiz: createBlocksDominancePattern(this, 'wiz'),
          wk057: createBlocksDominancePattern(this, 'wk057'),
          futurebitapollosolo: createBlocksDominancePattern(this, 'futurebitapollosolo'),
          carbonnegative: createBlocksDominancePattern(this, 'carbonnegative'),
          portlandhodl: createBlocksDominancePattern(this, 'portlandhodl'),
          phoenix: createBlocksDominancePattern(this, 'phoenix'),
          neopool: createBlocksDominancePattern(this, 'neopool'),
          maxipool: createBlocksDominancePattern(this, 'maxipool'),
          bitfufupool: createBlocksDominancePattern(this, 'bitfufupool'),
          gdpool: createBlocksDominancePattern(this, 'gdpool'),
          miningdutch: createBlocksDominancePattern(this, 'miningdutch'),
          publicpool: createBlocksDominancePattern(this, 'publicpool'),
          miningsquared: createBlocksDominancePattern(this, 'miningsquared'),
          innopolistech: createBlocksDominancePattern(this, 'innopolistech'),
          btclab: createBlocksDominancePattern(this, 'btclab'),
          parasite: createBlocksDominancePattern(this, 'parasite'),
          redrockpool: createBlocksDominancePattern(this, 'redrockpool'),
          est3lar: createBlocksDominancePattern(this, 'est3lar'),
        },
      },
      prices: {
        split: {
          open: createCentsSatsUsdPattern3(this, 'price_open'),
          high: createCentsSatsUsdPattern3(this, 'price_high'),
          low: createCentsSatsUsdPattern3(this, 'price_low'),
          close: createCentsSatsUsdPattern3(this, 'price_close'),
        },
        ohlc: {
          cents: createMetricPattern2(this, 'price_ohlc_cents'),
          usd: createMetricPattern2(this, 'price_ohlc'),
          sats: createMetricPattern2(this, 'price_ohlc_sats'),
        },
        spot: {
          cents: createMetricPattern1(this, 'price_cents'),
          usd: createMetricPattern1(this, 'price'),
          sats: createMetricPattern1(this, 'price_sats'),
        },
      },
      supply: {
        circulating: createBtcCentsSatsUsdPattern(this, 'circulating_supply'),
        burned: {
          opreturn: createBaseCumulativeSumPattern4(this, 'opreturn_supply'),
          unspendable: createBaseCumulativeSumPattern4(this, 'unspendable_supply'),
        },
        inflationRate: createBpsPercentRatioPattern2(this, 'inflation_rate'),
        velocity: {
          btc: createMetricPattern1(this, 'velocity_btc'),
          usd: createMetricPattern1(this, 'velocity_usd'),
        },
        marketCap: createCentsDeltaUsdPattern(this, 'market_cap'),
        marketMinusRealizedCapGrowthRate: create_1m1w1y24hPattern(this, 'market_minus_realized_cap_growth_rate'),
        hodledOrLost: createBtcCentsSatsUsdPattern(this, 'hodled_or_lost_coins'),
        state: createMetricPattern18(this, 'supply_state'),
      },
      cohorts: {
        utxo: {
          all: {
            supply: {
              inProfit: createBtcCentsRelSatsUsdPattern2(this, 'supply_in_profit'),
              inLoss: createBtcCentsRelSatsUsdPattern2(this, 'supply_in_loss'),
              total: createBtcCentsSatsUsdPattern(this, 'supply'),
              half: createBtcCentsSatsUsdPattern(this, 'supply_half'),
              delta: createChangeRatePattern2(this, 'supply_delta'),
            },
            outputs: createUnspentPattern(this, 'utxo_count'),
            activity: createCoindaysCoinyearsDormancySentVelocityPattern(this, ''),
            realized: createCapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern(this, ''),
            costBasis: createInvestedMaxMinPercentilesSupplyPattern(this, ''),
            unrealized: {
              grossPnl: createCentsUsdPattern2(this, 'unrealized_gross_pnl'),
              investedCapital: createInPattern(this, 'invested_capital_in'),
              sentiment: createGreedNetPainPattern(this, ''),
              loss: {
                negative: createMetricPattern1(this, 'neg_unrealized_loss'),
                base: createCentsUsdPattern2(this, 'unrealized_loss'),
                cumulative: createCentsUsdPattern2(this, 'unrealized_loss_cumulative'),
                sum: create_1m1w1y24hPattern5(this, 'unrealized_loss_sum'),
                relToMcap: createBpsPercentRatioPattern4(this, 'unrealized_loss_rel_to_mcap'),
                relToOwnGross: createBpsPercentRatioPattern4(this, 'unrealized_loss_rel_to_own_gross_pnl'),
              },
              netPnl: {
                cents: createMetricPattern1(this, 'net_unrealized_pnl_cents'),
                usd: createMetricPattern1(this, 'net_unrealized_pnl'),
                relToOwnGross: createBpsPercentRatioPattern2(this, 'net_unrealized_pnl_rel_to_own_gross_pnl'),
              },
              profit: {
                base: createCentsUsdPattern2(this, 'unrealized_profit'),
                cumulative: createCentsUsdPattern2(this, 'unrealized_profit_cumulative'),
                sum: create_1m1w1y24hPattern5(this, 'unrealized_profit_sum'),
                relToMcap: createBpsPercentRatioPattern4(this, 'unrealized_profit_rel_to_mcap'),
                relToOwnGross: createBpsPercentRatioPattern4(this, 'unrealized_profit_rel_to_own_gross_pnl'),
              },
              nupl: createBpsRatioPattern(this, 'nupl'),
            },
          },
          sth: {
            realized: createCapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern(this, 'sth'),
            supply: createDeltaHalfInRelTotalPattern2(this, 'sth_supply'),
            outputs: createUnspentPattern(this, 'sth_utxo_count'),
            activity: createCoindaysCoinyearsDormancySentVelocityPattern(this, 'sth'),
            costBasis: createInvestedMaxMinPercentilesSupplyPattern(this, 'sth'),
            unrealized: createGrossInvestedLossNetNuplProfitSentimentPattern2(this, 'sth'),
          },
          lth: {
            supply: createDeltaHalfInRelTotalPattern2(this, 'lth_supply'),
            outputs: createUnspentPattern(this, 'lth_utxo_count'),
            activity: createCoindaysCoinyearsDormancySentVelocityPattern(this, 'lth'),
            realized: {
              profit: createBaseCumulativeDistributionRelSumValuePattern(this, 'lth'),
              loss: createBaseCapitulationCumulativeNegativeRelSumValuePattern(this, 'lth'),
              grossPnl: createBaseCumulativeSumPattern3(this, 'lth_realized_gross_pnl'),
              sellSideRiskRatio: {
                _24h: createBpsPercentRatioPattern(this, 'lth_sell_side_risk_ratio_24h'),
                _1w: createBpsPercentRatioPattern(this, 'lth_sell_side_risk_ratio_1w'),
                _1m: createBpsPercentRatioPattern(this, 'lth_sell_side_risk_ratio_1m'),
                _1y: createBpsPercentRatioPattern(this, 'lth_sell_side_risk_ratio_1y'),
              },
              netPnl: createBaseChangeCumulativeDeltaRelSumPattern(this, 'lth_net'),
              sopr: {
                ratio: create_1m1w1y24hPattern(this, 'lth_sopr'),
                valueCreated: createBaseCumulativeSumPattern(this, 'lth_value_created'),
                valueDestroyed: createBaseCumulativeSumPattern(this, 'lth_value_destroyed'),
              },
              peakRegret: createBaseCumulativeRelPattern(this, 'lth_realized_peak_regret'),
              investor: createLowerPriceUpperPattern(this, 'lth'),
              profitToLossRatio: create_1m1w1y24hPattern(this, 'lth_realized_profit_to_loss_ratio'),
              cap: createCentsDeltaRelUsdPattern(this, 'lth_realized_cap'),
              price: createBpsCentsPercentilesRatioSatsSmaStdUsdPattern(this, 'lth_realized_price'),
              mvrv: createMetricPattern1(this, 'lth_mvrv'),
            },
            costBasis: createInvestedMaxMinPercentilesSupplyPattern(this, 'lth'),
            unrealized: createGrossInvestedLossNetNuplProfitSentimentPattern2(this, 'lth'),
          },
          ageRange: {
            under1h: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1h_old'),
            _1hTo1d: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1h_to_1d_old'),
            _1dTo1w: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1d_to_1w_old'),
            _1wTo1m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1w_to_1m_old'),
            _1mTo2m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1m_to_2m_old'),
            _2mTo3m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_2m_to_3m_old'),
            _3mTo4m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_3m_to_4m_old'),
            _4mTo5m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_4m_to_5m_old'),
            _5mTo6m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_5m_to_6m_old'),
            _6mTo1y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_6m_to_1y_old'),
            _1yTo2y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1y_to_2y_old'),
            _2yTo3y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_2y_to_3y_old'),
            _3yTo4y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_3y_to_4y_old'),
            _4yTo5y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_4y_to_5y_old'),
            _5yTo6y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_5y_to_6y_old'),
            _6yTo7y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_6y_to_7y_old'),
            _7yTo8y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_7y_to_8y_old'),
            _8yTo10y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_8y_to_10y_old'),
            _10yTo12y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10y_to_12y_old'),
            _12yTo15y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_12y_to_15y_old'),
            over15y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_15y_old'),
          },
          underAge: {
            _1w: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1w_old'),
            _1m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1m_old'),
            _2m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_2m_old'),
            _3m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_3m_old'),
            _4m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_4m_old'),
            _5m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_5m_old'),
            _6m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_6m_old'),
            _1y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1y_old'),
            _2y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_2y_old'),
            _3y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_3y_old'),
            _4y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_4y_old'),
            _5y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_5y_old'),
            _6y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_6y_old'),
            _7y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_7y_old'),
            _8y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_8y_old'),
            _10y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10y_old'),
            _12y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_12y_old'),
            _15y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_15y_old'),
          },
          overAge: {
            _1d: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1d_old'),
            _1w: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1w_old'),
            _1m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1m_old'),
            _2m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_2m_old'),
            _3m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_3m_old'),
            _4m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_4m_old'),
            _5m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_5m_old'),
            _6m: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_6m_old'),
            _1y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1y_old'),
            _2y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_2y_old'),
            _3y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_3y_old'),
            _4y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_4y_old'),
            _5y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_5y_old'),
            _6y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_6y_old'),
            _7y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_7y_old'),
            _8y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_8y_old'),
            _10y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10y_old'),
            _12y: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_12y_old'),
          },
          epoch: {
            _0: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'epoch_0'),
            _1: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'epoch_1'),
            _2: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'epoch_2'),
            _3: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'epoch_3'),
            _4: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'epoch_4'),
          },
          class: {
            _2009: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2009'),
            _2010: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2010'),
            _2011: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2011'),
            _2012: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2012'),
            _2013: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2013'),
            _2014: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2014'),
            _2015: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2015'),
            _2016: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2016'),
            _2017: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2017'),
            _2018: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2018'),
            _2019: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2019'),
            _2020: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2020'),
            _2021: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2021'),
            _2022: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2022'),
            _2023: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2023'),
            _2024: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2024'),
            _2025: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2025'),
            _2026: createActivityOutputsRealizedSupplyUnrealizedPattern(this, 'class_2026'),
          },
          overAmount: {
            _1sat: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1sat'),
            _10sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10sats'),
            _100sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_100sats'),
            _1kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1k_sats'),
            _10kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10k_sats'),
            _100kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_100k_sats'),
            _1mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1m_sats'),
            _10mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10m_sats'),
            _1btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1btc'),
            _10btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10btc'),
            _100btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_100btc'),
            _1kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_1k_btc'),
            _10kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_10k_btc'),
          },
          amountRange: {
            _0sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_0sats'),
            _1satTo10sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1sat_to_10sats'),
            _10satsTo100sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10sats_to_100sats'),
            _100satsTo1kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_100sats_to_1k_sats'),
            _1kSatsTo10kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1k_sats_to_10k_sats'),
            _10kSatsTo100kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10k_sats_to_100k_sats'),
            _100kSatsTo1mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_100k_sats_to_1m_sats'),
            _1mSatsTo10mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1m_sats_to_10m_sats'),
            _10mSatsTo1btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10m_sats_to_1btc'),
            _1btcTo10btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1btc_to_10btc'),
            _10btcTo100btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10btc_to_100btc'),
            _100btcTo1kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_100btc_to_1k_btc'),
            _1kBtcTo10kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_1k_btc_to_10k_btc'),
            _10kBtcTo100kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_10k_btc_to_100k_btc'),
            over100kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_over_100k_btc'),
          },
          underAmount: {
            _10sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10sats'),
            _100sats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_100sats'),
            _1kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1k_sats'),
            _10kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10k_sats'),
            _100kSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_100k_sats'),
            _1mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1m_sats'),
            _10mSats: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10m_sats'),
            _1btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1btc'),
            _10btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10btc'),
            _100btc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_100btc'),
            _1kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_1k_btc'),
            _10kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_10k_btc'),
            _100kBtc: createOutputsRealizedSupplyUnrealizedPattern(this, 'utxos_under_100k_btc'),
          },
          type: {
            p2pk65: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2pk65'),
            p2pk33: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2pk33'),
            p2pkh: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2pkh'),
            p2ms: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2ms'),
            p2sh: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2sh'),
            p2wpkh: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2wpkh'),
            p2wsh: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2wsh'),
            p2tr: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2tr'),
            p2a: createOutputsRealizedSupplyUnrealizedPattern2(this, 'p2a'),
            unknown: createOutputsRealizedSupplyUnrealizedPattern2(this, 'unknown_outputs'),
            empty: createOutputsRealizedSupplyUnrealizedPattern2(this, 'empty_outputs'),
          },
          profitability: {
            range: {
              over1000pctInProfit: createRealizedSupplyPattern(this, 'utxos_over_1000pct_in_profit'),
              _500pctTo1000pctInProfit: createRealizedSupplyPattern(this, 'utxos_500pct_to_1000pct_in_profit'),
              _300pctTo500pctInProfit: createRealizedSupplyPattern(this, 'utxos_300pct_to_500pct_in_profit'),
              _200pctTo300pctInProfit: createRealizedSupplyPattern(this, 'utxos_200pct_to_300pct_in_profit'),
              _100pctTo200pctInProfit: createRealizedSupplyPattern(this, 'utxos_100pct_to_200pct_in_profit'),
              _90pctTo100pctInProfit: createRealizedSupplyPattern(this, 'utxos_90pct_to_100pct_in_profit'),
              _80pctTo90pctInProfit: createRealizedSupplyPattern(this, 'utxos_80pct_to_90pct_in_profit'),
              _70pctTo80pctInProfit: createRealizedSupplyPattern(this, 'utxos_70pct_to_80pct_in_profit'),
              _60pctTo70pctInProfit: createRealizedSupplyPattern(this, 'utxos_60pct_to_70pct_in_profit'),
              _50pctTo60pctInProfit: createRealizedSupplyPattern(this, 'utxos_50pct_to_60pct_in_profit'),
              _40pctTo50pctInProfit: createRealizedSupplyPattern(this, 'utxos_40pct_to_50pct_in_profit'),
              _30pctTo40pctInProfit: createRealizedSupplyPattern(this, 'utxos_30pct_to_40pct_in_profit'),
              _20pctTo30pctInProfit: createRealizedSupplyPattern(this, 'utxos_20pct_to_30pct_in_profit'),
              _10pctTo20pctInProfit: createRealizedSupplyPattern(this, 'utxos_10pct_to_20pct_in_profit'),
              _0pctTo10pctInProfit: createRealizedSupplyPattern(this, 'utxos_0pct_to_10pct_in_profit'),
              _0pctTo10pctInLoss: createRealizedSupplyPattern(this, 'utxos_0pct_to_10pct_in_loss'),
              _10pctTo20pctInLoss: createRealizedSupplyPattern(this, 'utxos_10pct_to_20pct_in_loss'),
              _20pctTo30pctInLoss: createRealizedSupplyPattern(this, 'utxos_20pct_to_30pct_in_loss'),
              _30pctTo40pctInLoss: createRealizedSupplyPattern(this, 'utxos_30pct_to_40pct_in_loss'),
              _40pctTo50pctInLoss: createRealizedSupplyPattern(this, 'utxos_40pct_to_50pct_in_loss'),
              _50pctTo60pctInLoss: createRealizedSupplyPattern(this, 'utxos_50pct_to_60pct_in_loss'),
              _60pctTo70pctInLoss: createRealizedSupplyPattern(this, 'utxos_60pct_to_70pct_in_loss'),
              _70pctTo80pctInLoss: createRealizedSupplyPattern(this, 'utxos_70pct_to_80pct_in_loss'),
              _80pctTo90pctInLoss: createRealizedSupplyPattern(this, 'utxos_80pct_to_90pct_in_loss'),
              _90pctTo100pctInLoss: createRealizedSupplyPattern(this, 'utxos_90pct_to_100pct_in_loss'),
            },
            profit: {
              breakeven: createRealizedSupplyPattern(this, 'utxos_in_profit'),
              _10pct: createRealizedSupplyPattern(this, 'utxos_over_10pct_in_profit'),
              _20pct: createRealizedSupplyPattern(this, 'utxos_over_20pct_in_profit'),
              _30pct: createRealizedSupplyPattern(this, 'utxos_over_30pct_in_profit'),
              _40pct: createRealizedSupplyPattern(this, 'utxos_over_40pct_in_profit'),
              _50pct: createRealizedSupplyPattern(this, 'utxos_over_50pct_in_profit'),
              _60pct: createRealizedSupplyPattern(this, 'utxos_over_60pct_in_profit'),
              _70pct: createRealizedSupplyPattern(this, 'utxos_over_70pct_in_profit'),
              _80pct: createRealizedSupplyPattern(this, 'utxos_over_80pct_in_profit'),
              _90pct: createRealizedSupplyPattern(this, 'utxos_over_90pct_in_profit'),
              _100pct: createRealizedSupplyPattern(this, 'utxos_over_100pct_in_profit'),
              _200pct: createRealizedSupplyPattern(this, 'utxos_over_200pct_in_profit'),
              _300pct: createRealizedSupplyPattern(this, 'utxos_over_300pct_in_profit'),
              _500pct: createRealizedSupplyPattern(this, 'utxos_over_500pct_in_profit'),
            },
            loss: {
              breakeven: createRealizedSupplyPattern(this, 'utxos_in_loss'),
              _10pct: createRealizedSupplyPattern(this, 'utxos_over_10pct_in_loss'),
              _20pct: createRealizedSupplyPattern(this, 'utxos_over_20pct_in_loss'),
              _30pct: createRealizedSupplyPattern(this, 'utxos_over_30pct_in_loss'),
              _40pct: createRealizedSupplyPattern(this, 'utxos_over_40pct_in_loss'),
              _50pct: createRealizedSupplyPattern(this, 'utxos_over_50pct_in_loss'),
              _60pct: createRealizedSupplyPattern(this, 'utxos_over_60pct_in_loss'),
              _70pct: createRealizedSupplyPattern(this, 'utxos_over_70pct_in_loss'),
              _80pct: createRealizedSupplyPattern(this, 'utxos_over_80pct_in_loss'),
            },
          },
          matured: {
            under1h: createBtcCentsSatsUsdPattern(this, 'utxo_under_1h_old_matured'),
            _1hTo1d: createBtcCentsSatsUsdPattern(this, 'utxo_1h_to_1d_old_matured'),
            _1dTo1w: createBtcCentsSatsUsdPattern(this, 'utxo_1d_to_1w_old_matured'),
            _1wTo1m: createBtcCentsSatsUsdPattern(this, 'utxo_1w_to_1m_old_matured'),
            _1mTo2m: createBtcCentsSatsUsdPattern(this, 'utxo_1m_to_2m_old_matured'),
            _2mTo3m: createBtcCentsSatsUsdPattern(this, 'utxo_2m_to_3m_old_matured'),
            _3mTo4m: createBtcCentsSatsUsdPattern(this, 'utxo_3m_to_4m_old_matured'),
            _4mTo5m: createBtcCentsSatsUsdPattern(this, 'utxo_4m_to_5m_old_matured'),
            _5mTo6m: createBtcCentsSatsUsdPattern(this, 'utxo_5m_to_6m_old_matured'),
            _6mTo1y: createBtcCentsSatsUsdPattern(this, 'utxo_6m_to_1y_old_matured'),
            _1yTo2y: createBtcCentsSatsUsdPattern(this, 'utxo_1y_to_2y_old_matured'),
            _2yTo3y: createBtcCentsSatsUsdPattern(this, 'utxo_2y_to_3y_old_matured'),
            _3yTo4y: createBtcCentsSatsUsdPattern(this, 'utxo_3y_to_4y_old_matured'),
            _4yTo5y: createBtcCentsSatsUsdPattern(this, 'utxo_4y_to_5y_old_matured'),
            _5yTo6y: createBtcCentsSatsUsdPattern(this, 'utxo_5y_to_6y_old_matured'),
            _6yTo7y: createBtcCentsSatsUsdPattern(this, 'utxo_6y_to_7y_old_matured'),
            _7yTo8y: createBtcCentsSatsUsdPattern(this, 'utxo_7y_to_8y_old_matured'),
            _8yTo10y: createBtcCentsSatsUsdPattern(this, 'utxo_8y_to_10y_old_matured'),
            _10yTo12y: createBtcCentsSatsUsdPattern(this, 'utxo_10y_to_12y_old_matured'),
            _12yTo15y: createBtcCentsSatsUsdPattern(this, 'utxo_12y_to_15y_old_matured'),
            over15y: createBtcCentsSatsUsdPattern(this, 'utxo_over_15y_old_matured'),
          },
        },
        address: {
          overAmount: {
            _1sat: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_1sat'),
            _10sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_10sats'),
            _100sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_100sats'),
            _1kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_1k_sats'),
            _10kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_10k_sats'),
            _100kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_100k_sats'),
            _1mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_1m_sats'),
            _10mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_10m_sats'),
            _1btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_1btc'),
            _10btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_10btc'),
            _100btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_100btc'),
            _1kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_1k_btc'),
            _10kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_10k_btc'),
          },
          amountRange: {
            _0sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_0sats'),
            _1satTo10sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_1sat_to_10sats'),
            _10satsTo100sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_10sats_to_100sats'),
            _100satsTo1kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_100sats_to_1k_sats'),
            _1kSatsTo10kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_1k_sats_to_10k_sats'),
            _10kSatsTo100kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_10k_sats_to_100k_sats'),
            _100kSatsTo1mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_100k_sats_to_1m_sats'),
            _1mSatsTo10mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_1m_sats_to_10m_sats'),
            _10mSatsTo1btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_10m_sats_to_1btc'),
            _1btcTo10btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_1btc_to_10btc'),
            _10btcTo100btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_10btc_to_100btc'),
            _100btcTo1kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_100btc_to_1k_btc'),
            _1kBtcTo10kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_1k_btc_to_10k_btc'),
            _10kBtcTo100kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_10k_btc_to_100k_btc'),
            over100kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_over_100k_btc'),
          },
          underAmount: {
            _10sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_10sats'),
            _100sats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_100sats'),
            _1kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_1k_sats'),
            _10kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_10k_sats'),
            _100kSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_100k_sats'),
            _1mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_1m_sats'),
            _10mSats: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_10m_sats'),
            _1btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_1btc'),
            _10btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_10btc'),
            _100btc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_100btc'),
            _1kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_1k_btc'),
            _10kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_10k_btc'),
            _100kBtc: createAddressOutputsRealizedSupplyUnrealizedPattern(this, 'addrs_under_100k_btc'),
          },
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
   * Address transactions
   *
   * Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. Use ?after_txid=<txid> for pagination.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*
   *
   * Endpoint: `GET /api/address/{address}/txs`
   *
   * @param {Address} address
   * @param {Txid=} [after_txid] - Txid to paginate from (return transactions before this one)
   * @returns {Promise<Transaction[]>}
   */
  async getAddressTxs(address, after_txid) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    const query = params.toString();
    const path = `/api/address/${address}/txs${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Address confirmed transactions
   *
   * Get confirmed transactions for an address, 25 per page. Use ?after_txid=<txid> for pagination.
   *
   * *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*
   *
   * Endpoint: `GET /api/address/{address}/txs/chain`
   *
   * @param {Address} address
   * @param {Txid=} [after_txid] - Txid to paginate from (return transactions before this one)
   * @returns {Promise<Transaction[]>}
   */
  async getAddressConfirmedTxs(address, after_txid) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
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
   * Get metric info
   *
   * Returns the supported indexes and value type for the specified metric.
   *
   * Endpoint: `GET /api/metric/{metric}`
   *
   * @param {Metric} metric
   * @returns {Promise<MetricInfo>}
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
   * @param {RangeIndex=} [start] - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
   * @param {RangeIndex=} [end] - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
   * @param {Limit=} [limit] - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
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
   * Get raw metric data
   *
   * Returns just the data array without the MetricData wrapper. Supports the same range and format parameters as the standard endpoint.
   *
   * Endpoint: `GET /api/metric/{metric}/{index}/data`
   *
   * @param {Metric} metric - Metric name
   * @param {Index} index - Aggregation index
   * @param {RangeIndex=} [start] - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
   * @param {RangeIndex=} [end] - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
   * @param {Limit=} [limit] - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
   * @param {Format=} [format] - Format of the output
   * @returns {Promise<boolean[] | string>}
   */
  async getMetricData(metric, index, start, end, limit, format) {
    const params = new URLSearchParams();
    if (start !== undefined) params.set('start', String(start));
    if (end !== undefined) params.set('end', String(end));
    if (limit !== undefined) params.set('limit', String(limit));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    const path = `/api/metric/${metric}/${index}/data${query ? '?' + query : ''}`;
    if (format === 'csv') {
      return this.getText(path);
    }
    return this.getJson(path);
  }

  /**
   * Get latest metric value
   *
   * Returns the single most recent value for a metric, unwrapped (not inside a MetricData object).
   *
   * Endpoint: `GET /api/metric/{metric}/{index}/latest`
   *
   * @param {Metric} metric - Metric name
   * @param {Index} index - Aggregation index
   * @returns {Promise<*>}
   */
  async getMetricLatest(metric, index) {
    return this.getJson(`/api/metric/${metric}/${index}/latest`);
  }

  /**
   * Get metric data length
   *
   * Returns the total number of data points for a metric at the given index.
   *
   * Endpoint: `GET /api/metric/{metric}/{index}/len`
   *
   * @param {Metric} metric - Metric name
   * @param {Index} index - Aggregation index
   * @returns {Promise<number>}
   */
  async getMetricLen(metric, index) {
    return this.getJson(`/api/metric/${metric}/${index}/len`);
  }

  /**
   * Get metric version
   *
   * Returns the current version of a metric. Changes when the metric data is updated.
   *
   * Endpoint: `GET /api/metric/{metric}/{index}/version`
   *
   * @param {Metric} metric - Metric name
   * @param {Index} index - Aggregation index
   * @returns {Promise<Version>}
   */
  async getMetricVersion(metric, index) {
    return this.getJson(`/api/metric/${metric}/${index}/version`);
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
   * @param {RangeIndex=} [start] - Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
   * @param {RangeIndex=} [end] - Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
   * @param {Limit=} [limit] - Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
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
   * @param {number=} [per_page] - Results per page (default: 1000, max: 1000)
   * @returns {Promise<PaginatedMetrics>}
   */
  async listMetrics(page, per_page) {
    const params = new URLSearchParams();
    if (page !== undefined) params.set('page', String(page));
    if (per_page !== undefined) params.set('per_page', String(per_page));
    const query = params.toString();
    const path = `/api/metrics/list${query ? '?' + query : ''}`;
    return this.getJson(path);
  }

  /**
   * Search metrics
   *
   * Fuzzy search for metrics by name. Supports partial matches and typos.
   *
   * Endpoint: `GET /api/metrics/search`
   *
   * @param {Metric} [q] - Search query string
   * @param {Limit=} [limit] - Maximum number of results
   * @returns {Promise<Metric[]>}
   */
  async searchMetrics(q, limit) {
    const params = new URLSearchParams();
    params.set('q', String(q));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    const path = `/api/metrics/search${query ? '?' + query : ''}`;
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
