# Auto-generated BRK Python client
# Do not edit manually

from __future__ import annotations
from dataclasses import dataclass
from typing import TypeVar, Generic, Any, Dict, Optional, List, Iterator, Literal, TypedDict, Union, Protocol, overload, Tuple, TYPE_CHECKING
from http.client import HTTPSConnection, HTTPConnection
from urllib.parse import urlparse
from datetime import date, datetime, timedelta, timezone
import json

if TYPE_CHECKING:
    import pandas as pd  # type: ignore[import-not-found]
    import polars as pl  # type: ignore[import-not-found]

T = TypeVar('T')

# Type definitions

# Bitcoin address string
Address = str
# Satoshis
Sats = int
# Index within its type (e.g., 0 for first P2WPKH address)
TypeIndex = int
# Transaction ID (hash)
Txid = str
# Unified index for any address type (funded or empty)
AnyAddressIndex = TypeIndex
# Unsigned basis points stored as u16.
# 1 bp = 0.0001. Range: 0–6.5535.
# Use for bounded 0–1 ratios (dominance, adoption, liveliness, etc.).
BasisPoints16 = int
# Unsigned basis points stored as u32.
# 1 bp = 0.0001. Range: 0–429,496.7295.
# Use for unbounded unsigned ratios (MVRV, NVT, SOPR, etc.).
BasisPoints32 = int
# Signed basis points stored as i16.
# 1 bp = 0.0001. Range: -3.2767 to +3.2767.
# Use for signed bounded ratios (NUPL, net PnL ratios, etc.).
BasisPointsSigned16 = int
# Signed basis points stored as i32.
# 1 bp = 0.0001. Range: -214,748.3647 to +214,748.3647.
# Use for unbounded signed values (returns, growth rates, volatility, z-scores, etc.).
BasisPointsSigned32 = int
# Bitcoin amount as floating point (1 BTC = 100,000,000 satoshis)
Bitcoin = float
# Block height
Height = int
# UNIX timestamp in seconds
Timestamp = int
# Block hash
BlockHash = str
TxIndex = int
# Transaction or block weight in weight units (WU)
Weight = int
# Unsigned cents (u64) - for values that should never be negative.
# Used for invested capital, realized cap, etc.
Cents = int
# Cents × Sats (u128) - price in cents multiplied by amount in sats.
# Uses u128 because large amounts at any price can overflow u64.
CentsSats = int
# Signed cents (i64) - for values that can be negative.
# Used for profit/loss calculations, deltas, etc.
CentsSigned = int
# Raw cents squared (u128) - stores cents² × sats without division.
# Used for precise accumulation of investor cap values: Σ(price² × sats).
# investor_price = investor_cap_raw / realized_cap_raw
CentsSquaredSats = int
# US Dollar amount as floating point
Dollars = float
# Closing price value for a time period
Close = Dollars
# Cohort identifier for cost basis distribution.
Cohort = str
# Bucket type for cost basis aggregation.
# Options: raw (no aggregation), lin200/lin500/lin1000 (linear $200/$500/$1000),
# log10/log50/log100/log200 (logarithmic with 10/50/100/200 buckets per decade).
CostBasisBucket = Literal["raw", "lin200", "lin500", "lin1000", "log10", "log50", "log100", "log200"]
# Value type for cost basis distribution.
# Options: supply (BTC), realized (USD, price × supply), unrealized (USD, spot × supply).
CostBasisValue = Literal["supply", "realized", "unrealized"]
# Date in YYYYMMDD format stored as u32
Date = int
# Output format for API responses
Format = Literal["json", "csv"]
# Maximum number of results to return. Defaults to 100 if not specified.
Limit = int
# A range boundary: integer index, date, or timestamp.
RangeIndex = Union[int, Date, Timestamp]
Day1 = int
Day3 = int
EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
Epoch = int
# Fee rate in sats/vB
FeeRate = float
FundedAddressIndex = TypeIndex
Halving = int
# Hex-encoded string
Hex = str
# Highest price value for a time period
High = Dollars
Hour1 = int
Hour12 = int
Hour4 = int
# Lowest price value for a time period
Low = Dollars
# Virtual size in vbytes (weight / 4, rounded up)
VSize = int
# Metric name
Metric = str
# Version tracking for data schema and computed values.
# 
# Used to detect when stored data needs to be recomputed due to changes
# in computation logic or source data versions. Supports validation
# against persisted versions to ensure compatibility.
Version = int
# Comma-separated list of metric names
Metrics = str
Minute10 = int
Minute30 = int
Month1 = int
Month3 = int
Month6 = int
# Opening price value for a time period
Open = Dollars
OpReturnIndex = TypeIndex
OutPoint = int
# Type (P2PKH, P2WPKH, P2SH, P2TR, etc.)
OutputType = Literal["p2pk65", "p2pk33", "p2pkh", "p2ms", "p2sh", "opreturn", "p2wpkh", "p2wsh", "p2tr", "p2a", "empty", "unknown"]
P2AAddressIndex = TypeIndex
U8x2 = List[int]
P2ABytes = U8x2
P2MSOutputIndex = TypeIndex
P2PK33AddressIndex = TypeIndex
U8x33 = List[int]
P2PK33Bytes = U8x33
P2PK65AddressIndex = TypeIndex
U8x65 = List[int]
P2PK65Bytes = U8x65
P2PKHAddressIndex = TypeIndex
U8x20 = List[int]
P2PKHBytes = U8x20
P2SHAddressIndex = TypeIndex
P2SHBytes = U8x20
P2TRAddressIndex = TypeIndex
U8x32 = List[int]
P2TRBytes = U8x32
P2WPKHAddressIndex = TypeIndex
P2WPKHBytes = U8x20
P2WSHAddressIndex = TypeIndex
P2WSHBytes = U8x32
PoolSlug = Literal["unknown", "blockfills", "ultimuspool", "terrapool", "luxor", "onethash", "btccom", "bitfarms", "huobipool", "wayicn", "canoepool", "btctop", "bitcoincom", "pool175btc", "gbminers", "axbt", "asicminer", "bitminter", "bitcoinrussia", "btcserv", "simplecoinus", "btcguild", "eligius", "ozcoin", "eclipsemc", "maxbtc", "triplemining", "coinlab", "pool50btc", "ghashio", "stminingcorp", "bitparking", "mmpool", "polmine", "kncminer", "bitalo", "f2pool", "hhtt", "megabigpower", "mtred", "nmcbit", "yourbtcnet", "givemecoins", "braiinspool", "antpool", "multicoinco", "bcpoolio", "cointerra", "kanopool", "solock", "ckpool", "nicehash", "bitclub", "bitcoinaffiliatenetwork", "btcc", "bwpool", "exxbw", "bitsolo", "bitfury", "twentyoneinc", "digitalbtc", "eightbaochi", "mybtccoinpool", "tbdice", "hashpool", "nexious", "bravomining", "hotpool", "okexpool", "bcmonster", "onehash", "bixin", "tatmaspool", "viabtc", "connectbtc", "batpool", "waterhole", "dcexploration", "dcex", "btpool", "fiftyeightcoin", "bitcoinindia", "shawnp0wers", "phashio", "rigpool", "haozhuzhu", "sevenpool", "miningkings", "hashbx", "dpool", "rawpool", "haominer", "helix", "bitcoinukraine", "poolin", "secretsuperstar", "tigerpoolnet", "sigmapoolcom", "okpooltop", "hummerpool", "tangpool", "bytepool", "spiderpool", "novablock", "miningcity", "binancepool", "minerium", "lubiancom", "okkong", "aaopool", "emcdpool", "foundryusa", "sbicrypto", "arkpool", "purebtccom", "marapool", "kucoinpool", "entrustcharitypool", "okminer", "titan", "pegapool", "btcnuggets", "cloudhashing", "digitalxmintsy", "telco214", "btcpoolparty", "multipool", "transactioncoinmining", "btcdig", "trickysbtcpool", "btcmp", "eobot", "unomp", "patels", "gogreenlight", "bitcoinindiapool", "ekanembtc", "canoe", "tiger", "onem1x", "zulupool", "secpool", "ocean", "whitepool", "wiz", "wk057", "futurebitapollosolo", "carbonnegative", "portlandhodl", "phoenix", "neopool", "maxipool", "bitfufupool", "gdpool", "miningdutch", "publicpool", "miningsquared", "innopolistech", "btclab", "parasite", "redrockpool", "est3lar"]
# Transaction locktime
RawLockTime = int
# Fractional satoshis (f64) - for representing USD prices in sats
# 
# Formula: `sats_fract = usd_value * 100_000_000 / btc_price`
# 
# When BTC is $100,000:
# - $1 = 1,000 sats
# - $0.001 = 1 sat
# - $0.0001 = 0.1 sats (fractional)
SatsFract = float
# Signed satoshis (i64) - for values that can be negative.
# Used for changes, deltas, profit/loss calculations, etc.
SatsSigned = int
StoredBool = bool
# Stored 32-bit floating point value
StoredF32 = float
# Fixed-size 64-bit floating point value optimized for on-disk storage
StoredF64 = float
# Fixed-size 64-bit signed integer optimized for on-disk storage
StoredI64 = int
StoredI8 = int
StoredU16 = int
# Fixed-size 32-bit unsigned integer optimized for on-disk storage
StoredU32 = int
# Fixed-size 64-bit unsigned integer optimized for on-disk storage
StoredU64 = int
# Time period for mining statistics.
# 
# Used to specify the lookback window for pool statistics, hashrate calculations,
# and other time-based mining metrics.
TimePeriod = Literal["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"]
# Index of the output being spent in the previous transaction
Vout = int
# Transaction version number
TxVersion = int
TxInIndex = int
TxOutIndex = int
# Input index in the spending transaction
Vin = int
UnknownOutputIndex = TypeIndex
Week1 = int
Year1 = int
Year10 = int
# Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
# block-based (height, tx_index), and address/output type indexes.
Index = Literal["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halving", "epoch", "height", "tx_index", "txin_index", "txout_index", "empty_output_index", "op_return_index", "p2a_address_index", "p2ms_output_index", "p2pk33_address_index", "p2pk65_address_index", "p2pkh_address_index", "p2sh_address_index", "p2tr_address_index", "p2wpkh_address_index", "p2wsh_address_index", "unknown_output_index", "funded_address_index", "empty_address_index"]
# Hierarchical tree node for organizing metrics into categories
TreeNode = Union[dict[str, "TreeNode"], "MetricLeafWithSchema"]
class AddressChainStats(TypedDict):
    """
    Address statistics on the blockchain (confirmed transactions only)
    
    Based on mempool.space's format with type_index extension.

    Attributes:
        funded_txo_count: Total number of transaction outputs that funded this address
        funded_txo_sum: Total amount in satoshis received by this address across all funded outputs
        spent_txo_count: Total number of transaction outputs spent from this address
        spent_txo_sum: Total amount in satoshis spent from this address
        tx_count: Total number of confirmed transactions involving this address
        type_index: Index of this address within its type on the blockchain
    """
    funded_txo_count: int
    funded_txo_sum: Sats
    spent_txo_count: int
    spent_txo_sum: Sats
    tx_count: int
    type_index: TypeIndex

class AddressMempoolStats(TypedDict):
    """
    Address statistics in the mempool (unconfirmed transactions only)
    
    Based on mempool.space's format.

    Attributes:
        funded_txo_count: Number of unconfirmed transaction outputs funding this address
        funded_txo_sum: Total amount in satoshis being received in unconfirmed transactions
        spent_txo_count: Number of unconfirmed transaction inputs spending from this address
        spent_txo_sum: Total amount in satoshis being spent in unconfirmed transactions
        tx_count: Number of unconfirmed transactions involving this address
    """
    funded_txo_count: int
    funded_txo_sum: Sats
    spent_txo_count: int
    spent_txo_sum: Sats
    tx_count: int

class AddressParam(TypedDict):
    address: Address

class AddressStats(TypedDict):
    """
    Address information compatible with mempool.space API format

    Attributes:
        address: Bitcoin address string
        chain_stats: Statistics for confirmed transactions on the blockchain
        mempool_stats: Statistics for unconfirmed transactions in the mempool
    """
    address: Address
    chain_stats: AddressChainStats
    mempool_stats: Union[AddressMempoolStats, None]

class AddressTxidsParam(TypedDict):
    """
    Attributes:
        after_txid: Txid to paginate from (return transactions before this one)
    """
    after_txid: Union[Txid, None]

class AddressValidation(TypedDict):
    """
    Address validation result

    Attributes:
        isvalid: Whether the address is valid
        address: The validated address
        scriptPubKey: The scriptPubKey in hex
        isscript: Whether this is a script address (P2SH)
        iswitness: Whether this is a witness address
        witness_version: Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
        witness_program: Witness program in hex
    """
    isvalid: bool
    address: Optional[str]
    scriptPubKey: Optional[str]
    isscript: Optional[bool]
    iswitness: Optional[bool]
    witness_version: Optional[int]
    witness_program: Optional[str]

class BlockCountParam(TypedDict):
    """
    Attributes:
        block_count: Number of recent blocks to include
    """
    block_count: int

class BlockFeesEntry(TypedDict):
    """
    A single block fees data point.
    """
    avgHeight: Height
    timestamp: Timestamp
    avgFees: Sats

class BlockHashParam(TypedDict):
    hash: BlockHash

class BlockHashStartIndex(TypedDict):
    """
    Attributes:
        hash: Bitcoin block hash
        start_index: Starting transaction index within the block (0-based)
    """
    hash: BlockHash
    start_index: TxIndex

class BlockHashTxIndex(TypedDict):
    """
    Attributes:
        hash: Bitcoin block hash
        index: Transaction index within the block (0-based)
    """
    hash: BlockHash
    index: TxIndex

class BlockInfo(TypedDict):
    """
    Block information returned by the API

    Attributes:
        id: Block hash
        height: Block height
        tx_count: Number of transactions in the block
        size: Block size in bytes
        weight: Block weight in weight units
        timestamp: Block timestamp (Unix time)
        difficulty: Block difficulty as a floating point number
    """
    id: BlockHash
    height: Height
    tx_count: int
    size: int
    weight: Weight
    timestamp: Timestamp
    difficulty: float

class BlockRewardsEntry(TypedDict):
    """
    A single block rewards data point.
    """
    avgHeight: int
    timestamp: int
    avgRewards: int

class BlockSizeEntry(TypedDict):
    """
    A single block size data point.
    """
    avgHeight: int
    timestamp: int
    avgSize: int

class BlockWeightEntry(TypedDict):
    """
    A single block weight data point.
    """
    avgHeight: int
    timestamp: int
    avgWeight: int

class BlockSizesWeights(TypedDict):
    """
    Combined block sizes and weights response.
    """
    sizes: List[BlockSizeEntry]
    weights: List[BlockWeightEntry]

class BlockStatus(TypedDict):
    """
    Block status indicating whether block is in the best chain

    Attributes:
        in_best_chain: Whether this block is in the best chain
        height: Block height (only if in best chain)
        next_best: Hash of the next block in the best chain (only if in best chain and not tip)
    """
    in_best_chain: bool
    height: Union[Height, None]
    next_best: Union[BlockHash, None]

class BlockTimestamp(TypedDict):
    """
    Block information returned for timestamp queries

    Attributes:
        height: Block height
        hash: Block hash
        timestamp: Block timestamp in ISO 8601 format
    """
    height: Height
    hash: BlockHash
    timestamp: str

class CostBasisCohortParam(TypedDict):
    """
    Path parameters for cost basis dates endpoint.
    """
    cohort: Cohort

class CostBasisParams(TypedDict):
    """
    Path parameters for cost basis distribution endpoint.
    """
    cohort: Cohort
    date: str

class CostBasisQuery(TypedDict):
    """
    Query parameters for cost basis distribution endpoint.

    Attributes:
        bucket: Bucket type for aggregation. Default: raw (no aggregation).
        value: Value type to return. Default: supply.
    """
    bucket: CostBasisBucket
    value: CostBasisValue

class DataRangeFormat(TypedDict):
    """
    Data range with output format for API query parameters

    Attributes:
        start: Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
        end: Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    start: Union[RangeIndex, None]
    end: Union[RangeIndex, None]
    limit: Union[Limit, None]
    format: Format

class DifficultyAdjustment(TypedDict):
    """
    Difficulty adjustment information.

    Attributes:
        progressPercent: Progress through current difficulty epoch (0-100%)
        difficultyChange: Estimated difficulty change at next retarget (%)
        estimatedRetargetDate: Estimated Unix timestamp of next retarget
        remainingBlocks: Blocks remaining until retarget
        remainingTime: Estimated seconds until retarget
        previousRetarget: Previous difficulty adjustment (%)
        nextRetargetHeight: Height of next retarget
        timeAvg: Average block time in current epoch (seconds)
        adjustedTimeAvg: Time-adjusted average (accounting for timestamp manipulation)
        timeOffset: Time offset from expected schedule (seconds)
    """
    progressPercent: float
    difficultyChange: float
    estimatedRetargetDate: int
    remainingBlocks: int
    remainingTime: int
    previousRetarget: float
    nextRetargetHeight: Height
    timeAvg: int
    adjustedTimeAvg: int
    timeOffset: int

class DifficultyAdjustmentEntry(TypedDict):
    """
    A single difficulty adjustment entry.
    Serializes as array: [timestamp, height, difficulty, change_percent]
    """
    timestamp: Timestamp
    height: Height
    difficulty: float
    change_percent: float

class DifficultyEntry(TypedDict):
    """
    A single difficulty data point.

    Attributes:
        timestamp: Unix timestamp of the difficulty adjustment.
        difficulty: Difficulty value.
        height: Block height of the adjustment.
    """
    timestamp: Timestamp
    difficulty: float
    height: Height

class DiskUsage(TypedDict):
    """
    Disk usage of the indexed data

    Attributes:
        brk: Human-readable brk data size (e.g., "48.8 GiB")
        brk_bytes: brk data size in bytes
        bitcoin: Human-readable Bitcoin blocks directory size
        bitcoin_bytes: Bitcoin blocks directory size in bytes
        ratio: brk as percentage of Bitcoin data
    """
    brk: str
    brk_bytes: int
    bitcoin: str
    bitcoin_bytes: int
    ratio: float

class EmptyAddressData(TypedDict):
    """
    Data of an empty address

    Attributes:
        tx_count: Total transaction count
        funded_txo_count: Total funded/spent transaction output count (equal since address is empty)
        transfered: Total satoshis transferred
    """
    tx_count: int
    funded_txo_count: int
    transfered: Sats

class ErrorDetail(TypedDict):
    """
    Attributes:
        type: Error category: "invalid_request", "forbidden", "not_found", "unavailable", or "internal"
        code: Machine-readable error code (e.g. "invalid_address", "metric_not_found")
        message: Human-readable description
        doc_url: Link to API documentation
    """
    type: str
    code: str
    message: str
    doc_url: str

class ErrorBody(TypedDict):
    error: ErrorDetail

class FundedAddressData(TypedDict):
    """
    Data for a funded (non-empty) address with current balance

    Attributes:
        tx_count: Total transaction count
        funded_txo_count: Number of transaction outputs funded to this address
        spent_txo_count: Number of transaction outputs spent by this address
        received: Satoshis received by this address
        sent: Satoshis sent by this address
        realized_cap_raw: The realized capitalization: Σ(price × sats)
        investor_cap_raw: The investor capitalization: Σ(price² × sats)
    """
    tx_count: int
    funded_txo_count: int
    spent_txo_count: int
    received: Sats
    sent: Sats
    realized_cap_raw: CentsSats
    investor_cap_raw: CentsSquaredSats

class HashrateEntry(TypedDict):
    """
    A single hashrate data point.

    Attributes:
        timestamp: Unix timestamp.
        avgHashrate: Average hashrate (H/s).
    """
    timestamp: Timestamp
    avgHashrate: int

class HashrateSummary(TypedDict):
    """
    Summary of network hashrate and difficulty data.

    Attributes:
        hashrates: Historical hashrate data points.
        difficulty: Historical difficulty adjustments.
        currentHashrate: Current network hashrate (H/s).
        currentDifficulty: Current network difficulty.
    """
    hashrates: List[HashrateEntry]
    difficulty: List[DifficultyEntry]
    currentHashrate: int
    currentDifficulty: float

class Health(TypedDict):
    """
    Server health status

    Attributes:
        started_at: Server start time (ISO 8601)
        uptime_seconds: Uptime in seconds
        indexed_height: Height of the last indexed block
        computed_height: Height of the last computed block (metrics)
        tip_height: Height of the chain tip (from Bitcoin node)
        blocks_behind: Number of blocks behind the tip
        last_indexed_at: Human-readable timestamp of the last indexed block (ISO 8601)
        last_indexed_at_unix: Unix timestamp of the last indexed block
    """
    status: str
    service: str
    version: str
    timestamp: str
    started_at: str
    uptime_seconds: int
    indexed_height: Height
    computed_height: Height
    tip_height: Height
    blocks_behind: Height
    last_indexed_at: str
    last_indexed_at_unix: Timestamp

class HeightParam(TypedDict):
    height: Height

class IndexInfo(TypedDict):
    """
    Information about an available index and its query aliases

    Attributes:
        index: The canonical index name
        aliases: All Accepted query aliases
    """
    index: Index
    aliases: List[str]

class MempoolBlock(TypedDict):
    """
    Block info in a mempool.space like format for fee estimation.

    Attributes:
        blockSize: Total block size in weight units
        blockVSize: Total block virtual size in vbytes
        nTx: Number of transactions in the projected block
        totalFees: Total fees in satoshis
        medianFee: Median fee rate in sat/vB
        feeRange: Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]
    """
    blockSize: int
    blockVSize: float
    nTx: int
    totalFees: Sats
    medianFee: FeeRate
    feeRange: List[FeeRate]

class MempoolInfo(TypedDict):
    """
    Mempool statistics

    Attributes:
        count: Number of transactions in the mempool
        vsize: Total virtual size of all transactions in the mempool (vbytes)
        total_fee: Total fees of all transactions in the mempool (satoshis)
    """
    count: int
    vsize: VSize
    total_fee: Sats

class MetricCount(TypedDict):
    """
    Metric count statistics - distinct metrics and total metric-index combinations

    Attributes:
        distinct_metrics: Number of unique metrics available (e.g., realized_price, market_cap)
        total_endpoints: Total number of metric-index combinations across all timeframes
        lazy_endpoints: Number of lazy (computed on-the-fly) metric-index combinations
        stored_endpoints: Number of eager (stored on disk) metric-index combinations
    """
    distinct_metrics: int
    total_endpoints: int
    lazy_endpoints: int
    stored_endpoints: int

class MetricInfo(TypedDict):
    """
    Metadata about a metric

    Attributes:
        indexes: Available indexes
        type: Value type (e.g. "f32", "u64", "Sats")
    """
    indexes: List[Index]
    type: str

class MetricParam(TypedDict):
    metric: Metric

class MetricSelection(TypedDict):
    """
    Selection of metrics to query

    Attributes:
        metrics: Requested metrics
        index: Index to query
        start: Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
        end: Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    metrics: Metrics
    index: Index
    start: Union[RangeIndex, None]
    end: Union[RangeIndex, None]
    limit: Union[Limit, None]
    format: Format

class MetricSelectionLegacy(TypedDict):
    """
    Legacy metric selection parameters (deprecated)

    Attributes:
        start: Inclusive start: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `from`, `f`, `s`
        end: Exclusive end: integer index, date (YYYY-MM-DD), or timestamp (ISO 8601). Negative integers count from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    index: Index
    ids: Metrics
    start: Union[RangeIndex, None]
    end: Union[RangeIndex, None]
    limit: Union[Limit, None]
    format: Format

class MetricWithIndex(TypedDict):
    """
    Attributes:
        metric: Metric name
        index: Aggregation index
    """
    metric: Metric
    index: Index

class OHLCCents(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in cents
    """
    open: Open
    high: High
    low: Low
    close: Close

class OHLCDollars(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in dollars
    """
    open: Open
    high: High
    low: Low
    close: Close

class OHLCSats(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in satoshis
    """
    open: Open
    high: High
    low: Low
    close: Close

class PaginatedMetrics(TypedDict):
    """
    A paginated list of available metric names (1000 per page)

    Attributes:
        current_page: Current page number (0-indexed)
        max_page: Maximum valid page index (0-indexed)
        total_count: Total number of metrics
        per_page: Results per page
        has_more: Whether more pages are available after the current one
        metrics: List of metric names
    """
    current_page: int
    max_page: int
    total_count: int
    per_page: int
    has_more: bool
    metrics: List[str]

class Pagination(TypedDict):
    """
    Pagination parameters for paginated API endpoints

    Attributes:
        page: Pagination index
        per_page: Results per page (default: 1000, max: 1000)
    """
    page: Optional[int]
    per_page: Optional[int]

class PoolBlockCounts(TypedDict):
    """
    Block counts for different time periods

    Attributes:
        all: Total blocks mined (all time)
        _24h: Blocks mined in last 24 hours
        _1w: Blocks mined in last week
    """
    all: int
    _24h: int
    _1w: int

class PoolBlockShares(TypedDict):
    """
    Pool's share of total blocks for different time periods

    Attributes:
        all: Share of all blocks (0.0 - 1.0)
        _24h: Share of blocks in last 24 hours
        _1w: Share of blocks in last week
    """
    all: float
    _24h: float
    _1w: float

class PoolDetailInfo(TypedDict):
    """
    Pool information for detail view

    Attributes:
        id: Unique pool identifier
        name: Pool name
        link: Pool website URL
        addresses: Known payout addresses
        regexes: Coinbase tag patterns (regexes)
        slug: URL-friendly pool identifier
    """
    id: int
    name: str
    link: str
    addresses: List[str]
    regexes: List[str]
    slug: PoolSlug

class PoolDetail(TypedDict):
    """
    Detailed pool information with statistics across time periods

    Attributes:
        pool: Pool information
        blockCount: Block counts for different time periods
        blockShare: Pool's share of total blocks for different time periods
        estimatedHashrate: Estimated hashrate based on blocks mined
        reportedHashrate: Self-reported hashrate (if available)
    """
    pool: PoolDetailInfo
    blockCount: PoolBlockCounts
    blockShare: PoolBlockShares
    estimatedHashrate: int
    reportedHashrate: Optional[int]

class PoolInfo(TypedDict):
    """
    Basic pool information for listing all pools

    Attributes:
        name: Pool name
        slug: URL-friendly pool identifier
        unique_id: Unique numeric pool identifier
    """
    name: str
    slug: PoolSlug
    unique_id: int

class PoolSlugParam(TypedDict):
    slug: PoolSlug

class PoolStats(TypedDict):
    """
    Mining pool with block statistics for a time period

    Attributes:
        poolId: Unique pool identifier
        name: Pool name
        link: Pool website URL
        blockCount: Number of blocks mined in the time period
        rank: Pool ranking by block count (1 = most blocks)
        emptyBlocks: Number of empty blocks mined
        slug: URL-friendly pool identifier
        share: Pool's share of total blocks (0.0 - 1.0)
    """
    poolId: int
    name: str
    link: str
    blockCount: int
    rank: int
    emptyBlocks: int
    slug: PoolSlug
    share: float

class PoolsSummary(TypedDict):
    """
    Mining pools response for a time period

    Attributes:
        pools: List of pools sorted by block count descending
        blockCount: Total blocks in the time period
        lastEstimatedHashrate: Estimated network hashrate (hashes per second)
    """
    pools: List[PoolStats]
    blockCount: int
    lastEstimatedHashrate: int

class RecommendedFees(TypedDict):
    """
    Recommended fee rates in sat/vB

    Attributes:
        fastestFee: Fee rate for fastest confirmation (next block)
        halfHourFee: Fee rate for confirmation within ~30 minutes (3 blocks)
        hourFee: Fee rate for confirmation within ~1 hour (6 blocks)
        economyFee: Fee rate for economical confirmation
        minimumFee: Minimum relay fee rate
    """
    fastestFee: FeeRate
    halfHourFee: FeeRate
    hourFee: FeeRate
    economyFee: FeeRate
    minimumFee: FeeRate

class RewardStats(TypedDict):
    """
    Block reward statistics over a range of blocks

    Attributes:
        startBlock: First block in the range
        endBlock: Last block in the range
    """
    startBlock: Height
    endBlock: Height
    totalReward: Sats
    totalFee: Sats
    totalTx: int

class SearchQuery(TypedDict):
    """
    Attributes:
        q: Search query string
        limit: Maximum number of results
    """
    q: Metric
    limit: Limit

class SupplyState(TypedDict):
    """
    Current supply state tracking UTXO count and total value

    Attributes:
        utxo_count: Number of unspent transaction outputs
        value: Total value in satoshis
    """
    utxo_count: int
    value: Sats

class SyncStatus(TypedDict):
    """
    Sync status of the indexer

    Attributes:
        indexed_height: Height of the last indexed block
        computed_height: Height of the last computed block (metrics)
        tip_height: Height of the chain tip (from Bitcoin node)
        blocks_behind: Number of blocks behind the tip
        last_indexed_at: Human-readable timestamp of the last indexed block (ISO 8601)
        last_indexed_at_unix: Unix timestamp of the last indexed block
    """
    indexed_height: Height
    computed_height: Height
    tip_height: Height
    blocks_behind: Height
    last_indexed_at: str
    last_indexed_at_unix: Timestamp

class TimePeriodParam(TypedDict):
    time_period: TimePeriod

class TimestampParam(TypedDict):
    timestamp: Timestamp

class TxOut(TypedDict):
    """
    Transaction output

    Attributes:
        scriptpubkey: Script pubkey (locking script)
        value: Value of the output in satoshis
    """
    scriptpubkey: str
    value: Sats

class TxIn(TypedDict):
    """
    Transaction input

    Attributes:
        txid: Transaction ID of the output being spent
        prevout: Information about the previous output being spent
        scriptsig: Signature script (for non-SegWit inputs)
        scriptsig_asm: Signature script in assembly format
        is_coinbase: Whether this input is a coinbase (block reward) input
        sequence: Input sequence number
        inner_redeemscript_asm: Inner redeemscript in assembly format (for P2SH-wrapped SegWit)
    """
    txid: Txid
    vout: Vout
    prevout: Union[TxOut, None]
    scriptsig: str
    scriptsig_asm: str
    is_coinbase: bool
    sequence: int
    inner_redeemscript_asm: Optional[str]

class TxStatus(TypedDict):
    """
    Transaction confirmation status

    Attributes:
        confirmed: Whether the transaction is confirmed
        block_height: Block height (only present if confirmed)
        block_hash: Block hash (only present if confirmed)
        block_time: Block timestamp (only present if confirmed)
    """
    confirmed: bool
    block_height: Union[Height, None]
    block_hash: Union[BlockHash, None]
    block_time: Union[Timestamp, None]

class Transaction(TypedDict):
    """
    Transaction information compatible with mempool.space API format

    Attributes:
        size: Transaction size in bytes
        weight: Transaction weight
        sigops: Number of signature operations
        fee: Transaction fee in satoshis
        vin: Transaction inputs
        vout: Transaction outputs
    """
    index: Union[TxIndex, None]
    txid: Txid
    version: TxVersion
    locktime: RawLockTime
    size: int
    weight: Weight
    sigops: int
    fee: Sats
    vin: List[TxIn]
    vout: List[TxOut]
    status: TxStatus

class TxOutspend(TypedDict):
    """
    Status of an output indicating whether it has been spent

    Attributes:
        spent: Whether the output has been spent
        txid: Transaction ID of the spending transaction (only present if spent)
        vin: Input index in the spending transaction (only present if spent)
        status: Status of the spending transaction (only present if spent)
    """
    spent: bool
    txid: Union[Txid, None]
    vin: Union[Vin, None]
    status: Union[TxStatus, None]

class TxidParam(TypedDict):
    txid: Txid

class TxidVout(TypedDict):
    """
    Transaction output reference (txid + output index)

    Attributes:
        txid: Transaction ID
        vout: Output index
    """
    txid: Txid
    vout: Vout

class Utxo(TypedDict):
    """
    Unspent transaction output
    """
    txid: Txid
    vout: Vout
    status: TxStatus
    value: Sats

class ValidateAddressParam(TypedDict):
    """
    Attributes:
        address: Bitcoin address to validate (can be any string)
    """
    address: str

class MetricLeafWithSchema(TypedDict):
    """
    MetricLeaf with JSON Schema for client generation

    Attributes:
        name: The metric name/identifier
        kind: The Rust type (e.g., "Sats", "StoredF64")
        indexes: Available indexes for this metric
        type: JSON Schema type (e.g., "integer", "number", "string", "boolean", "array", "object")
    """
    name: str
    kind: str
    indexes: List[Index]
    type: str


class BrkError(Exception):
    """Custom error class for BRK client errors."""

    def __init__(self, message: str, status: Optional[int] = None):
        super().__init__(message)
        self.status = status


class BrkClientBase:
    """Base HTTP client for making requests."""

    def __init__(self, base_url: str, timeout: float = 30.0):
        parsed = urlparse(base_url)
        self._host = parsed.netloc
        self._secure = parsed.scheme == 'https'
        self._timeout = timeout
        self._conn: Optional[Union[HTTPSConnection, HTTPConnection]] = None

    def _connect(self) -> Union[HTTPSConnection, HTTPConnection]:
        """Get or create HTTP connection."""
        if self._conn is None:
            if self._secure:
                self._conn = HTTPSConnection(self._host, timeout=self._timeout)
            else:
                self._conn = HTTPConnection(self._host, timeout=self._timeout)
        return self._conn

    def get(self, path: str) -> bytes:
        """Make a GET request and return raw bytes."""
        try:
            conn = self._connect()
            conn.request("GET", path)
            res = conn.getresponse()
            data = res.read()
            if res.status >= 400:
                raise BrkError(f"HTTP error: {res.status}", res.status)
            return data
        except (ConnectionError, OSError, TimeoutError) as e:
            self._conn = None
            raise BrkError(str(e))

    def get_json(self, path: str) -> Any:
        """Make a GET request and return JSON."""
        return json.loads(self.get(path))

    def get_text(self, path: str) -> str:
        """Make a GET request and return text."""
        return self.get(path).decode()

    def close(self) -> None:
        """Close the HTTP client."""
        if self._conn:
            self._conn.close()
            self._conn = None

    def __enter__(self) -> BrkClientBase:
        return self

    def __exit__(self, exc_type: Optional[type], exc_val: Optional[BaseException], exc_tb: Optional[Any]) -> None:
        self.close()


def _m(acc: str, s: str) -> str:
    """Build metric name with suffix."""
    if not s: return acc
    return f"{acc}_{s}" if acc else s


def _p(prefix: str, acc: str) -> str:
    """Build metric name with prefix."""
    return f"{prefix}_{acc}" if acc else prefix


# Date conversion constants
_GENESIS = date(2009, 1, 3)  # day1 0, week1 0
_DAY_ONE = date(2009, 1, 9)  # day1 1 (6 day gap after genesis)
_EPOCH = datetime(2009, 1, 1, tzinfo=timezone.utc)
_DATE_INDEXES = frozenset([
    'minute10', 'minute30',
    'hour1', 'hour4', 'hour12',
    'day1', 'day3', 'week1',
    'month1', 'month3', 'month6',
    'year1', 'year10',
])

def _index_to_date(index: str, i: int) -> Union[date, datetime]:
    """Convert an index value to a date/datetime for date-based indexes."""
    if index == 'minute10':
        return _EPOCH + timedelta(minutes=i * 10)
    elif index == 'minute30':
        return _EPOCH + timedelta(minutes=i * 30)
    elif index == 'hour1':
        return _EPOCH + timedelta(hours=i)
    elif index == 'hour4':
        return _EPOCH + timedelta(hours=i * 4)
    elif index == 'hour12':
        return _EPOCH + timedelta(hours=i * 12)
    elif index == 'day1':
        return _GENESIS if i == 0 else _DAY_ONE + timedelta(days=i - 1)
    elif index == 'day3':
        return _EPOCH.date() - timedelta(days=1) + timedelta(days=i * 3)
    elif index == 'week1':
        return _GENESIS + timedelta(weeks=i)
    elif index == 'month1':
        return date(2009 + i // 12, i % 12 + 1, 1)
    elif index == 'month3':
        m = i * 3
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'month6':
        m = i * 6
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'year1':
        return date(2009 + i, 1, 1)
    elif index == 'year10':
        return date(2009 + i * 10, 1, 1)
    else:
        raise ValueError(f"{index} is not a date-based index")


def _date_to_index(index: str, d: Union[date, datetime]) -> int:
    """Convert a date/datetime to an index value for date-based indexes.

    Returns the floor index (latest index whose date is <= the given date).
    For sub-day indexes (minute*, hour*), a plain date is treated as midnight UTC.
    """
    if index in ('minute10', 'minute30', 'hour1', 'hour4', 'hour12'):
        if isinstance(d, datetime):
            dt = d if d.tzinfo else d.replace(tzinfo=timezone.utc)
        else:
            dt = datetime(d.year, d.month, d.day, tzinfo=timezone.utc)
        secs = int((dt - _EPOCH).total_seconds())
        div = {'minute10': 600, 'minute30': 1800,
               'hour1': 3600, 'hour4': 14400, 'hour12': 43200}
        return secs // div[index]
    dd = d.date() if isinstance(d, datetime) else d
    if index == 'day1':
        if dd < _DAY_ONE:
            return 0
        return 1 + (dd - _DAY_ONE).days
    elif index == 'day3':
        return (dd - date(2008, 12, 31)).days // 3
    elif index == 'week1':
        return (dd - _GENESIS).days // 7
    elif index == 'month1':
        return (dd.year - 2009) * 12 + (dd.month - 1)
    elif index == 'month3':
        return (dd.year - 2009) * 4 + (dd.month - 1) // 3
    elif index == 'month6':
        return (dd.year - 2009) * 2 + (dd.month - 1) // 6
    elif index == 'year1':
        return dd.year - 2009
    elif index == 'year10':
        return (dd.year - 2009) // 10
    else:
        raise ValueError(f"{index} is not a date-based index")


@dataclass
class MetricData(Generic[T]):
    """Metric data with range information. Always int-indexed."""
    version: int
    index: Index
    total: int
    start: int
    end: int
    stamp: str
    data: List[T]

    @property
    def is_date_based(self) -> bool:
        """Whether this metric uses a date-based index."""
        return self.index in _DATE_INDEXES

    def indexes(self) -> List[int]:
        """Get raw index numbers."""
        return list(range(self.start, self.end))

    def keys(self) -> List[int]:
        """Get keys as index numbers."""
        return self.indexes()

    def items(self) -> List[Tuple[int, T]]:
        """Get (index, value) pairs."""
        return list(zip(self.indexes(), self.data))

    def to_dict(self) -> Dict[int, T]:
        """Return {index: value} dict."""
        return dict(zip(self.indexes(), self.data))

    def __iter__(self) -> Iterator[Tuple[int, T]]:
        """Iterate over (index, value) pairs."""
        return iter(zip(self.indexes(), self.data))

    def __len__(self) -> int:
        return len(self.data)

    def to_polars(self) -> pl.DataFrame:
        """Convert to Polars DataFrame with 'index' and 'value' columns."""
        try:
            import polars as pl  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("polars is required: pip install polars")
        return pl.DataFrame({"index": self.indexes(), "value": self.data})

    def to_pandas(self) -> pd.DataFrame:
        """Convert to Pandas DataFrame with 'index' and 'value' columns."""
        try:
            import pandas as pd  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("pandas is required: pip install pandas")
        return pd.DataFrame({"index": self.indexes(), "value": self.data})


@dataclass
class DateMetricData(MetricData[T]):
    """Metric data with date-based index. Extends MetricData with date methods."""

    def dates(self) -> List[Union[date, datetime]]:
        """Get dates for the index range. Returns datetime for sub-daily indexes, date for daily+."""
        return [_index_to_date(self.index, i) for i in range(self.start, self.end)]

    def date_items(self) -> List[Tuple[Union[date, datetime], T]]:
        """Get (date, value) pairs."""
        return list(zip(self.dates(), self.data))

    def to_date_dict(self) -> Dict[Union[date, datetime], T]:
        """Return {date: value} dict."""
        return dict(zip(self.dates(), self.data))

    def to_polars(self, with_dates: bool = True) -> pl.DataFrame:
        """Convert to Polars DataFrame.

        Returns a DataFrame with columns:
        - 'date' and 'value' if with_dates=True (default)
        - 'index' and 'value' otherwise
        """
        try:
            import polars as pl  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("polars is required: pip install polars")
        if with_dates:
            return pl.DataFrame({"date": self.dates(), "value": self.data})
        return pl.DataFrame({"index": self.indexes(), "value": self.data})

    def to_pandas(self, with_dates: bool = True) -> pd.DataFrame:
        """Convert to Pandas DataFrame.

        Returns a DataFrame with columns:
        - 'date' and 'value' if with_dates=True (default)
        - 'index' and 'value' otherwise
        """
        try:
            import pandas as pd  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("pandas is required: pip install pandas")
        if with_dates:
            return pd.DataFrame({"date": self.dates(), "value": self.data})
        return pd.DataFrame({"index": self.indexes(), "value": self.data})


# Type aliases for non-generic usage
AnyMetricData = MetricData[Any]
AnyDateMetricData = DateMetricData[Any]


class _EndpointConfig:
    """Shared endpoint configuration."""
    client: BrkClientBase
    name: str
    index: Index
    start: Optional[int]
    end: Optional[int]

    def __init__(self, client: BrkClientBase, name: str, index: Index,
                 start: Optional[int] = None, end: Optional[int] = None):
        self.client = client
        self.name = name
        self.index = index
        self.start = start
        self.end = end

    def path(self) -> str:
        return f"/api/metric/{self.name}/{self.index}"

    def _build_path(self, format: Optional[str] = None) -> str:
        params = []
        if self.start is not None:
            params.append(f"start={self.start}")
        if self.end is not None:
            params.append(f"end={self.end}")
        if format is not None:
            params.append(f"format={format}")
        query = "&".join(params)
        p = self.path()
        return f"{p}?{query}" if query else p

    def _new(self, start: Optional[int] = None, end: Optional[int] = None) -> _EndpointConfig:
        return _EndpointConfig(self.client, self.name, self.index, start, end)

    def get_metric(self) -> MetricData[Any]:
        return MetricData(**self.client.get_json(self._build_path()))

    def get_date_metric(self) -> DateMetricData[Any]:
        return DateMetricData(**self.client.get_json(self._build_path()))

    def get_csv(self) -> str:
        return self.client.get_text(self._build_path(format='csv'))


class RangeBuilder(Generic[T]):
    """Builder with range specified."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the range as parsed JSON."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch the range as CSV string."""
        return self._config.get_csv()


class SingleItemBuilder(Generic[T]):
    """Builder for single item access."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def fetch(self) -> MetricData[T]:
        """Fetch the single item."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class SkippedBuilder(Generic[T]):
    """Builder after calling skip(n). Chain with take() to specify count."""

    def __init__(self, config: _EndpointConfig):
        self._config = config

    def take(self, n: int) -> RangeBuilder[T]:
        """Take n items after the skipped position."""
        start = self._config.start or 0
        return RangeBuilder(self._config._new(start, start + n))

    def fetch(self) -> MetricData[T]:
        """Fetch from skipped position to end."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class DateRangeBuilder(RangeBuilder[T]):
    """Range builder that returns DateMetricData."""
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class DateSingleItemBuilder(SingleItemBuilder[T]):
    """Single item builder that returns DateMetricData."""
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class DateSkippedBuilder(SkippedBuilder[T]):
    """Skipped builder that returns DateMetricData."""
    def take(self, n: int) -> DateRangeBuilder[T]:
        start = self._config.start or 0
        return DateRangeBuilder(self._config._new(start, start + n))
    def fetch(self) -> DateMetricData[T]:
        return self._config.get_date_metric()


class MetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries with int-based indexing.

    Examples:
        data = endpoint.fetch()
        data = endpoint[5].fetch()
        data = endpoint[:10].fetch()
        data = endpoint.head(20).fetch()
        data = endpoint.skip(100).take(10).fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> SingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> RangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice]) -> Union[SingleItemBuilder[T], RangeBuilder[T]]:
        """Access single item or slice by integer index."""
        if isinstance(key, int):
            return SingleItemBuilder(self._config._new(key, key + 1))
        return RangeBuilder(self._config._new(key.start, key.stop))

    def head(self, n: int = 10) -> RangeBuilder[T]:
        """Get the first n items."""
        return RangeBuilder(self._config._new(end=n))

    def tail(self, n: int = 10) -> RangeBuilder[T]:
        """Get the last n items."""
        return RangeBuilder(self._config._new(end=0) if n == 0 else self._config._new(start=-n))

    def skip(self, n: int) -> SkippedBuilder[T]:
        """Skip the first n items."""
        return SkippedBuilder(self._config._new(start=n))

    def fetch(self) -> MetricData[T]:
        """Fetch all data."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


class DateMetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries with date-based indexing.

    Accepts dates in __getitem__ and returns DateMetricData from fetch().

    Examples:
        data = endpoint.fetch()
        data = endpoint[date(2020, 1, 1)].fetch()
        data = endpoint[date(2020, 1, 1):date(2023, 1, 1)].fetch()
        data = endpoint[:10].fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: datetime) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: date) -> DateSingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> DateRangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice, date, datetime]) -> Union[DateSingleItemBuilder[T], DateRangeBuilder[T]]:
        """Access single item or slice. Accepts int, date, or datetime."""
        if isinstance(key, (date, datetime)):
            idx = _date_to_index(self._config.index, key)
            return DateSingleItemBuilder(self._config._new(idx, idx + 1))
        if isinstance(key, int):
            return DateSingleItemBuilder(self._config._new(key, key + 1))
        start, stop = key.start, key.stop
        if isinstance(start, (date, datetime)):
            start = _date_to_index(self._config.index, start)
        if isinstance(stop, (date, datetime)):
            stop = _date_to_index(self._config.index, stop)
        return DateRangeBuilder(self._config._new(start, stop))

    def head(self, n: int = 10) -> DateRangeBuilder[T]:
        """Get the first n items."""
        return DateRangeBuilder(self._config._new(end=n))

    def tail(self, n: int = 10) -> DateRangeBuilder[T]:
        """Get the last n items."""
        return DateRangeBuilder(self._config._new(end=0) if n == 0 else self._config._new(start=-n))

    def skip(self, n: int) -> DateSkippedBuilder[T]:
        """Skip the first n items."""
        return DateSkippedBuilder(self._config._new(start=n))

    def fetch(self) -> DateMetricData[T]:
        """Fetch all data."""
        return self._config.get_date_metric()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


# Type aliases for non-generic usage
AnyMetricEndpointBuilder = MetricEndpointBuilder[Any]
AnyDateMetricEndpointBuilder = DateMetricEndpointBuilder[Any]


class MetricPattern(Protocol[T]):
    """Protocol for metric patterns with different index sets."""

    @property
    def name(self) -> str:
        """Get the metric name."""
        ...

    def indexes(self) -> List[str]:
        """Get the list of available indexes for this metric."""
        ...

    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]:
        """Get an endpoint builder for a specific index, if supported."""
        ...


# Static index tuples
_i1 = ('minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halving', 'epoch', 'height')
_i2 = ('minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halving', 'epoch')
_i3 = ('minute10',)
_i4 = ('minute30',)
_i5 = ('hour1',)
_i6 = ('hour4',)
_i7 = ('hour12',)
_i8 = ('day1',)
_i9 = ('day3',)
_i10 = ('week1',)
_i11 = ('month1',)
_i12 = ('month3',)
_i13 = ('month6',)
_i14 = ('year1',)
_i15 = ('year10',)
_i16 = ('halving',)
_i17 = ('epoch',)
_i18 = ('height',)
_i19 = ('tx_index',)
_i20 = ('txin_index',)
_i21 = ('txout_index',)
_i22 = ('empty_output_index',)
_i23 = ('op_return_index',)
_i24 = ('p2a_address_index',)
_i25 = ('p2ms_output_index',)
_i26 = ('p2pk33_address_index',)
_i27 = ('p2pk65_address_index',)
_i28 = ('p2pkh_address_index',)
_i29 = ('p2sh_address_index',)
_i30 = ('p2tr_address_index',)
_i31 = ('p2wpkh_address_index',)
_i32 = ('p2wsh_address_index',)
_i33 = ('unknown_output_index',)
_i34 = ('funded_address_index',)
_i35 = ('empty_address_index',)

def _ep(c: BrkClientBase, n: str, i: Index) -> MetricEndpointBuilder[Any]:
    return MetricEndpointBuilder(c, n, i)

def _dep(c: BrkClientBase, n: str, i: Index) -> DateMetricEndpointBuilder[Any]:
    return DateMetricEndpointBuilder(c, n, i)

# Index accessor classes

class _MetricPattern1By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute10')
    def minute30(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute30')
    def hour1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour1')
    def hour4(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour4')
    def hour12(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour12')
    def day1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day1')
    def day3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day3')
    def week1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'week1')
    def month1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month1')
    def month3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month3')
    def month6(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month6')
    def year1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year1')
    def year10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year10')
    def halving(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halving')
    def epoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'epoch')
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')

class MetricPattern1(Generic[T]):
    by: _MetricPattern1By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern1By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i1)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i1 else None

class _MetricPattern2By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute10')
    def minute30(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute30')
    def hour1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour1')
    def hour4(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour4')
    def hour12(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour12')
    def day1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day1')
    def day3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day3')
    def week1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'week1')
    def month1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month1')
    def month3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month3')
    def month6(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month6')
    def year1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year1')
    def year10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year10')
    def halving(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halving')
    def epoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'epoch')

class MetricPattern2(Generic[T]):
    by: _MetricPattern2By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern2By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i2)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i2 else None

class _MetricPattern3By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute10')

class MetricPattern3(Generic[T]):
    by: _MetricPattern3By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern3By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i3)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i3 else None

class _MetricPattern4By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute30(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute30')

class MetricPattern4(Generic[T]):
    by: _MetricPattern4By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern4By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i4)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i4 else None

class _MetricPattern5By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour1')

class MetricPattern5(Generic[T]):
    by: _MetricPattern5By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern5By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i5)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i5 else None

class _MetricPattern6By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour4(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour4')

class MetricPattern6(Generic[T]):
    by: _MetricPattern6By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern6By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i6)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i6 else None

class _MetricPattern7By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour12(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour12')

class MetricPattern7(Generic[T]):
    by: _MetricPattern7By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern7By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i7)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i7 else None

class _MetricPattern8By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def day1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day1')

class MetricPattern8(Generic[T]):
    by: _MetricPattern8By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern8By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i8)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i8 else None

class _MetricPattern9By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def day3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day3')

class MetricPattern9(Generic[T]):
    by: _MetricPattern9By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern9By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i9)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i9 else None

class _MetricPattern10By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def week1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'week1')

class MetricPattern10(Generic[T]):
    by: _MetricPattern10By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern10By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i10)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i10 else None

class _MetricPattern11By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month1')

class MetricPattern11(Generic[T]):
    by: _MetricPattern11By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern11By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i11)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i11 else None

class _MetricPattern12By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month3')

class MetricPattern12(Generic[T]):
    by: _MetricPattern12By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern12By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i12)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i12 else None

class _MetricPattern13By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month6(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month6')

class MetricPattern13(Generic[T]):
    by: _MetricPattern13By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern13By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i13)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i13 else None

class _MetricPattern14By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def year1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year1')

class MetricPattern14(Generic[T]):
    by: _MetricPattern14By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern14By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i14)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i14 else None

class _MetricPattern15By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def year10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year10')

class MetricPattern15(Generic[T]):
    by: _MetricPattern15By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern15By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i15)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i15 else None

class _MetricPattern16By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def halving(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halving')

class MetricPattern16(Generic[T]):
    by: _MetricPattern16By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern16By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i16)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i16 else None

class _MetricPattern17By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def epoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'epoch')

class MetricPattern17(Generic[T]):
    by: _MetricPattern17By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern17By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i17)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i17 else None

class _MetricPattern18By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')

class MetricPattern18(Generic[T]):
    by: _MetricPattern18By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern18By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i18)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i18 else None

class _MetricPattern19By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def tx_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'tx_index')

class MetricPattern19(Generic[T]):
    by: _MetricPattern19By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern19By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i19)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i19 else None

class _MetricPattern20By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txin_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txin_index')

class MetricPattern20(Generic[T]):
    by: _MetricPattern20By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern20By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i20)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i20 else None

class _MetricPattern21By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txout_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txout_index')

class MetricPattern21(Generic[T]):
    by: _MetricPattern21By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern21By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i21)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i21 else None

class _MetricPattern22By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def empty_output_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'empty_output_index')

class MetricPattern22(Generic[T]):
    by: _MetricPattern22By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern22By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i22)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i22 else None

class _MetricPattern23By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def op_return_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'op_return_index')

class MetricPattern23(Generic[T]):
    by: _MetricPattern23By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern23By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i23)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i23 else None

class _MetricPattern24By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2a_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2a_address_index')

class MetricPattern24(Generic[T]):
    by: _MetricPattern24By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern24By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i24)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i24 else None

class _MetricPattern25By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2ms_output_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2ms_output_index')

class MetricPattern25(Generic[T]):
    by: _MetricPattern25By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern25By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i25)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i25 else None

class _MetricPattern26By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk33_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk33_address_index')

class MetricPattern26(Generic[T]):
    by: _MetricPattern26By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern26By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i26)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i26 else None

class _MetricPattern27By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk65_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk65_address_index')

class MetricPattern27(Generic[T]):
    by: _MetricPattern27By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern27By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i27)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i27 else None

class _MetricPattern28By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pkh_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pkh_address_index')

class MetricPattern28(Generic[T]):
    by: _MetricPattern28By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern28By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i28)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i28 else None

class _MetricPattern29By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2sh_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2sh_address_index')

class MetricPattern29(Generic[T]):
    by: _MetricPattern29By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern29By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i29)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i29 else None

class _MetricPattern30By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2tr_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2tr_address_index')

class MetricPattern30(Generic[T]):
    by: _MetricPattern30By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern30By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i30)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i30 else None

class _MetricPattern31By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wpkh_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wpkh_address_index')

class MetricPattern31(Generic[T]):
    by: _MetricPattern31By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern31By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i31)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i31 else None

class _MetricPattern32By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wsh_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wsh_address_index')

class MetricPattern32(Generic[T]):
    by: _MetricPattern32By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern32By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i32)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i32 else None

class _MetricPattern33By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def unknown_output_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'unknown_output_index')

class MetricPattern33(Generic[T]):
    by: _MetricPattern33By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern33By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i33)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i33 else None

class _MetricPattern34By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def funded_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'funded_address_index')

class MetricPattern34(Generic[T]):
    by: _MetricPattern34By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern34By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i34)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i34 else None

class _MetricPattern35By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def empty_address_index(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'empty_address_index')

class MetricPattern35(Generic[T]):
    by: _MetricPattern35By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern35By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i35)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i35 else None

# Reusable structural pattern classes

class Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.pct05: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct05'))
        self.pct10: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct10'))
        self.pct15: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct15'))
        self.pct20: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct20'))
        self.pct25: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct25'))
        self.pct30: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct30'))
        self.pct35: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct35'))
        self.pct40: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct40'))
        self.pct45: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct45'))
        self.pct50: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct50'))
        self.pct55: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct55'))
        self.pct60: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct60'))
        self.pct65: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct65'))
        self.pct70: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct70'))
        self.pct75: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct75'))
        self.pct80: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct80'))
        self.pct85: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct85'))
        self.pct90: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct90'))
        self.pct95: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct95'))

class _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._0sd: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, f'0sd{disc}'))
        self.m0_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m0_5sd', disc))
        self.m1_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m1_5sd', disc))
        self.m1sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m1sd', disc))
        self.m2_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m2_5sd', disc))
        self.m2sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m2sd', disc))
        self.m3sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('m3sd', disc))
        self.p0_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p0_5sd', disc))
        self.p1_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p1_5sd', disc))
        self.p1sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p1sd', disc))
        self.p2_5sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p2_5sd', disc))
        self.p2sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p2sd', disc))
        self.p3sd: PriceRatioPattern = PriceRatioPattern(client, acc, _m('p3sd', disc))
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'ratio_sd{disc}'))
        self.zscore: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'ratio_zscore{disc}'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '10y'))
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1m'))
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1w'))
        self._1y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1y'))
        self._2y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '2y'))
        self._3m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '3m'))
        self._3y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '3y'))
        self._4y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '4y'))
        self._5y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '5y'))
        self._6m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '6m'))
        self._6y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '6y'))
        self._8y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '8y'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '10y'))
        self._1m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1m'))
        self._1w: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1w'))
        self._1y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1y'))
        self._2y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '2y'))
        self._3m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '3m'))
        self._3y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '3y'))
        self._4y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '4y'))
        self._5y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '5y'))
        self._6m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '6m'))
        self._6y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '6y'))
        self._8y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '8y'))

class CapGrossInvestorLossMvrvNetPeakPriceProfitSellSoprPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap: CentsDeltaRelUsdPattern = CentsDeltaRelUsdPattern(client, f'{acc}_cap')
        self.gross_pnl: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, f'{acc}_gross_pnl')
        self.investor: LowerPriceUpperPattern = LowerPriceUpperPattern(client, f'{acc}_investor')
        self.loss: BaseCapitulationCumulativeNegativeRelSumValuePattern = BaseCapitulationCumulativeNegativeRelSumValuePattern(client, f'{acc}_loss')
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_mvrv')
        self.net_pnl: BaseChangeCumulativeDeltaRelSumPattern = BaseChangeCumulativeDeltaRelSumPattern(client, f'{acc}_net_pnl')
        self.peak_regret: BaseCumulativeRelPattern = BaseCumulativeRelPattern(client, f'{acc}_peak_regret')
        self.price: BpsCentsPercentilesRatioSatsSmaStdUsdPattern = BpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, f'{acc}_price')
        self.profit: BaseCumulativeDistributionRelSumValuePattern = BaseCumulativeDistributionRelSumValuePattern(client, f'{acc}_profit')
        self.profit_to_loss_ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, f'{acc}_profit_to_loss_ratio')
        self.sell_side_risk_ratio: _1m1w1y24hPattern6 = _1m1w1y24hPattern6(client, f'{acc}_sell_side_risk_ratio')
        self.sopr: AdjustedRatioValuePattern = AdjustedRatioValuePattern(client, f'{acc}_sopr')

class AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'average'))
        self.cumulative: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'cumulative'))
        self.max: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'max'))
        self.median: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'median'))
        self.min: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'min'))
        self.pct10: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'pct90'))
        self.rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc)
        self.sum: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'sum'))

class AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'average'))
        self.base: MetricPattern1[StoredU64] = MetricPattern1(client, acc)
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'max'))
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'median'))
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'min'))
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct10'))
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct25'))
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct75'))
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct90'))
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class AverageGainsLossesRsiStochPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'average_gain_{disc}'))
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'average_loss_{disc}'))
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'gains_{disc}'))
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'losses_{disc}'))
        self.rsi: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, disc))
        self.rsi_max: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, f'max_{disc}'))
        self.rsi_min: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, f'min_{disc}'))
        self.stoch_rsi: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, f'stoch_{disc}'))
        self.stoch_rsi_d: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, f'stoch_d_{disc}'))
        self.stoch_rsi_k: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, f'stoch_k_{disc}'))

class AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, acc)
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2a', acc))
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2pk33', acc))
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2pk65', acc))
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2pkh', acc))
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2sh', acc))
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2tr', acc))
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2wpkh', acc))
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, _p('p2wsh', acc))

class AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'average'))
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'max'))
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'median'))
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'min'))
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct10'))
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct25'))
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct75'))
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'pct90'))
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'average'))
        self.max: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'max'))
        self.median: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'median'))
        self.min: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'min'))
        self.pct10: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'pct10'))
        self.pct25: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'pct25'))
        self.pct75: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'pct75'))
        self.pct90: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'pct90'))

class BaseCapitulationCumulativeNegativeRelSumValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'realized_loss'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'realized_loss_cumulative'))
        self.negative: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.rel_to_rcap: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, 'realized_loss_rel_to_rcap'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, 'realized_loss_sum'))
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'loss_value_created'))
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'loss_value_destroyed'))

class BpsCentsPercentilesRatioSatsSmaStdUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'ratio_bps'))
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.percentiles: Pct1Pct2Pct5Pct95Pct98Pct99Pattern = Pct1Pct2Pct5Pct95Pct98Pct99Pattern(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.sma: _1m1w1y2y4yAllPattern = _1m1w1y2y4yAllPattern(client, _m(acc, 'ratio_sma'))
        self.std_dev: _1y2y4yAllPattern = _1y2y4yAllPattern(client, acc)
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'average'))
        self.max: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'max'))
        self.median: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'median'))
        self.min: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'min'))
        self.pct10: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'pct90'))

class _10y2y3y4y5y6y8yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '10y'))
        self._2y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '2y'))
        self._3y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '3y'))
        self._4y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '4y'))
        self._5y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '5y'))
        self._6y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '6y'))
        self._8y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '8y'))

class _1m1w1y24hBpsPercentRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, '1m'))
        self._1w: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, '1w'))
        self._1y: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, '1y'))
        self._24h: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, '24h'))
        self.bps: MetricPattern1[BasisPoints16] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class BaseCumulativeDistributionRelSumValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'realized_profit'))
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'realized_profit_cumulative'))
        self.distribution_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'distribution_flow'))
        self.rel_to_rcap: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, 'realized_profit_rel_to_rcap'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, 'realized_profit_sum'))
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'profit_value_created'))
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'profit_value_destroyed'))

class BaseCumulativeNegativeRelSumPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'unrealized_loss'))
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'unrealized_loss_cumulative'))
        self.negative: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.rel_to_mcap: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'unrealized_loss_rel_to_mcap'))
        self.rel_to_own_gross: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'unrealized_loss_rel_to_own_gross_pnl'))
        self.rel_to_own_mcap: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, 'unrealized_loss_rel_to_own_mcap'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, 'unrealized_loss_sum'))

class CapLossMvrvNetPriceProfitSoprPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap: CentsDeltaUsdPattern = CentsDeltaUsdPattern(client, _m(acc, 'realized_cap'))
        self.loss: BaseCumulativeNegativeSumPattern = BaseCumulativeNegativeSumPattern(client, acc, '')
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.net_pnl: BaseCumulativeDeltaSumPattern = BaseCumulativeDeltaSumPattern(client, _m(acc, 'net_realized_pnl'))
        self.price: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.profit: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, _m(acc, 'realized_profit'))
        self.sopr: RatioValuePattern = RatioValuePattern(client, _m(acc, 'sopr_24h'))

class GrossInvestedLossNetNuplProfitSentimentPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.gross_pnl: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'unrealized_gross_pnl'))
        self.invested_capital: InPattern = InPattern(client, _m(acc, 'invested_capital_in'))
        self.loss: BaseCumulativeNegativeRelSumPattern2 = BaseCumulativeNegativeRelSumPattern2(client, acc)
        self.net_pnl: CentsRelUsdPattern2 = CentsRelUsdPattern2(client, _m(acc, 'net_unrealized_pnl'))
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'nupl'))
        self.profit: BaseCumulativeRelSumPattern2 = BaseCumulativeRelSumPattern2(client, _m(acc, 'unrealized_profit'))
        self.sentiment: GreedNetPainPattern = GreedNetPainPattern(client, acc)

class _1m1w1y2y4yAllPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, '1m'))
        self._1w: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, '1w'))
        self._1y: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, '1y'))
        self._2y: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, '2y'))
        self._4y: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, '4y'))
        self.all: BpsRatioPattern2 = BpsRatioPattern2(client, _m(acc, 'all'))

class BaseChangeCumulativeDeltaRelSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'realized_pnl'))
        self.change_1m: RelPattern = RelPattern(client, _m(acc, 'pnl_change_1m_rel_to'))
        self.cumulative: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'realized_pnl_cumulative'))
        self.delta: AbsoluteRatePattern2 = AbsoluteRatePattern2(client, _m(acc, 'realized_pnl_delta'))
        self.rel_to_rcap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'realized_pnl_rel_to_rcap'))
        self.sum: _1m1w1y24hPattern3 = _1m1w1y24hPattern3(client, _m(acc, 'realized_pnl_sum'))

class BaseCumulativeRelSumPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, acc)
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'cumulative'))
        self.rel_to_mcap: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_mcap'))
        self.rel_to_own_gross: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_own_gross_pnl'))
        self.rel_to_own_mcap: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_own_mcap'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, 'sum'))

class BpsCentsPercentilesRatioSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'ratio_bps'))
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.percentiles: Pct1Pct2Pct5Pct95Pct98Pct99Pattern = Pct1Pct2Pct5Pct95Pct98Pct99Pattern(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class BtcCentsRelSatsUsdPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, acc)
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.rel_to_circulating: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_circulating'))
        self.rel_to_own: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_own'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class CapLossMvrvPriceProfitSoprPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap: CentsDeltaUsdPattern = CentsDeltaUsdPattern(client, _m(acc, 'realized_cap'))
        self.loss: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, _m(acc, 'realized_loss'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.price: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.profit: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, _m(acc, 'realized_profit'))
        self.sopr: ValuePattern = ValuePattern(client, _m(acc, 'value'))

class DeltaHalfInRelTotalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.half: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'half'))
        self.in_loss: BtcCentsRelSatsUsdPattern = BtcCentsRelSatsUsdPattern(client, _m(acc, 'in_loss'))
        self.in_profit: BtcCentsRelSatsUsdPattern = BtcCentsRelSatsUsdPattern(client, _m(acc, 'in_profit'))
        self.rel_to_circulating: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_circulating'))
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)

class DeltaHalfInRelTotalPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.half: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'half'))
        self.in_loss: BtcCentsRelSatsUsdPattern3 = BtcCentsRelSatsUsdPattern3(client, _m(acc, 'in_loss'))
        self.in_profit: BtcCentsRelSatsUsdPattern3 = BtcCentsRelSatsUsdPattern3(client, _m(acc, 'in_profit'))
        self.rel_to_circulating: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_circulating'))
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)

class Pct1Pct2Pct5Pct95Pct98Pct99Pattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.pct1: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct1')
        self.pct2: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct2')
        self.pct5: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct5')
        self.pct95: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct95')
        self.pct98: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct98')
        self.pct99: BpsPriceRatioPattern = BpsPriceRatioPattern(client, acc, 'pct99')

class ActivityOutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoindaysSentPattern = CoindaysSentPattern(client, acc)
        self.outputs: UnspentPattern = UnspentPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapLossMvrvNetPriceProfitSoprPattern = CapLossMvrvNetPriceProfitSoprPattern(client, acc)
        self.supply: DeltaHalfInRelTotalPattern = DeltaHalfInRelTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: LossNetNuplProfitPattern = LossNetNuplProfitPattern(client, acc)

class AddressOutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.address_count: DeltaInnerPattern = DeltaInnerPattern(client, _m(acc, 'address_count'))
        self.outputs: UnspentPattern = UnspentPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapLossMvrvPriceProfitSoprPattern = CapLossMvrvPriceProfitSoprPattern(client, acc)
        self.supply: DeltaHalfTotalPattern = DeltaHalfTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: NuplPattern = NuplPattern(client, _m(acc, 'nupl'))

class BaseCumulativeInSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern1[Sats] = MetricPattern1(client, acc)
        self.cumulative: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.in_loss: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, _m(acc, 'in_loss'))
        self.in_profit: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, _m(acc, 'in_profit'))
        self.sum: _1m1w1y24hPattern[Sats] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class BpsCentsRatioSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'ratio_bps'))
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class BtcCentsDeltaSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, acc)
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class BtcCentsRelSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, acc)
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.rel_to_circulating: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_circulating'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class BtcCentsRelSatsUsdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, acc)
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.rel_to_own: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'rel_to_own'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class DeltaHalfInTotalPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.half: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'half'))
        self.in_loss: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'in_loss'))
        self.in_profit: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'in_profit'))
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)

class EmaHistogramLineSignalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_ema_fast')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_ema_slow')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_histogram')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_line')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_signal')

class InvestedMaxMinPercentilesSupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital'))
        self.max: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'cost_basis_max'))
        self.min: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'cost_basis_min'))
        self.percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis'))
        self.supply_density: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'supply_density'))

class MvrvNuplRealizedSupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'nupl'))
        self.realized_cap: AllSthPattern = AllSthPattern(client, _m(acc, 'realized_cap'))
        self.realized_price: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.supply: AllSthPattern2 = AllSthPattern2(client, _m(acc, 'supply'))

class PhsReboundThsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.phs: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'phs'))
        self.phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'phs_min'))
        self.rebound: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'rebound'))
        self.ths: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ths'))
        self.ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ths_min'))

class _1m1w1y24hHeightPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1m'))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1w'))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1y'))
        self._24h: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_24h'))
        self.height: MetricPattern18[T] = MetricPattern18(client, acc)

class _1m1w1y24hPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1m_rate'))
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1w_rate'))
        self._1y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1y_rate'))
        self._24h: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '24h_rate'))

class _1m1w1y24hPattern6:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, '1m'))
        self._1w: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, '1w'))
        self._1y: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, '1y'))
        self._24h: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, '24h'))

class _1m1w1y24hPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1m'))
        self._1w: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1w'))
        self._1y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1y'))
        self._24h: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '24h'))

class _1m1w1y2wPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, '1m'))
        self._1w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, '1w'))
        self._1y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, '1y'))
        self._2w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, '2w'))

class _1m1w1y24hPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: CentsUsdPattern = CentsUsdPattern(client, _m(acc, '1m'))
        self._1w: CentsUsdPattern = CentsUsdPattern(client, _m(acc, '1w'))
        self._1y: CentsUsdPattern = CentsUsdPattern(client, _m(acc, '1y'))
        self._24h: CentsUsdPattern = CentsUsdPattern(client, _m(acc, '24h'))

class _1m1w1y24hPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, '1m'))
        self._1w: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, '1w'))
        self._1y: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, '1y'))
        self._24h: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, '24h'))

class _1y2y4yAllPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc, '1y')
        self._2y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc, '2y')
        self._4y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc, '4y')
        self.all: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdZscorePattern(client, acc, '')

class AdjustedRatioValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted: RatioValuePattern2 = RatioValuePattern2(client, acc)
        self.ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, _m(acc, 'sopr'))
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'value_created'))
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'value_destroyed'))

class BaseCumulativeDeltaSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern = CentsUsdPattern(client, acc)
        self.cumulative: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'cumulative'))
        self.delta: AbsoluteRatePattern2 = AbsoluteRatePattern2(client, _m(acc, 'delta'))
        self.sum: _1m1w1y24hPattern3 = _1m1w1y24hPattern3(client, _m(acc, 'sum'))

class BaseCumulativeNegativeSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, disc))
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, f'{disc}_cumulative'))
        self.negative: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, f'neg_{disc}'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, f'{disc}_sum'))

class BothReactivatedReceivingSendingPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.both: _1m1w1y24hHeightPattern[StoredU32] = _1m1w1y24hHeightPattern(client, _m(acc, 'both'))
        self.reactivated: _1m1w1y24hHeightPattern[StoredU32] = _1m1w1y24hHeightPattern(client, _m(acc, 'reactivated'))
        self.receiving: _1m1w1y24hHeightPattern[StoredU32] = _1m1w1y24hHeightPattern(client, _m(acc, 'receiving'))
        self.sending: _1m1w1y24hHeightPattern[StoredU32] = _1m1w1y24hHeightPattern(client, _m(acc, 'sending'))

class BtcCentsSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, acc)
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class CentsDeltaRelUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.delta: AbsoluteRatePattern2 = AbsoluteRatePattern2(client, _m(acc, 'delta'))
        self.rel_to_own_mcap: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, 'rel_to_own_mcap'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class CentsRelUsdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'cents'))
        self.rel_to_own_gross: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'rel_to_own_gross_pnl'))
        self.rel_to_own_mcap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'rel_to_own_mcap'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class CoindaysCoinyearsDormancySentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coindays_destroyed: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, _m(acc, 'coindays_destroyed'))
        self.coinyears_destroyed: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'coinyears_destroyed'))
        self.dormancy: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dormancy'))
        self.sent: BaseCumulativeInSumPattern = BaseCumulativeInSumPattern(client, _m(acc, 'sent'))

class LossNetNuplProfitPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.loss: BaseCumulativeNegativeSumPattern = BaseCumulativeNegativeSumPattern(client, acc, '')
        self.net_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_unrealized_pnl'))
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'nupl'))
        self.profit: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, _m(acc, 'unrealized_profit'))

class OutputsRealizedSupplyUnrealizedPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.outputs: UnspentPattern = UnspentPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapLossMvrvPriceProfitSoprPattern = CapLossMvrvPriceProfitSoprPattern(client, acc)
        self.supply: DeltaHalfInTotalPattern2 = DeltaHalfInTotalPattern2(client, _m(acc, 'supply'))
        self.unrealized: LossNuplProfitPattern = LossNuplProfitPattern(client, acc)

class OutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.outputs: UnspentPattern = UnspentPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapLossMvrvPriceProfitSoprPattern = CapLossMvrvPriceProfitSoprPattern(client, acc)
        self.supply: DeltaHalfTotalPattern = DeltaHalfTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: NuplPattern = NuplPattern(client, _m(acc, 'nupl'))

class _1m1w1y24hPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1m'))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1w'))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1y'))
        self._24h: MetricPattern1[T] = MetricPattern1(client, _m(acc, '24h'))

class BaseCumulativeSumPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'cumulative'))
        self.sum: _1m1w1y24hPattern5 = _1m1w1y24hPattern5(client, _m(acc, 'sum'))

class BaseCumulativeRelPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern1[Cents] = MetricPattern1(client, acc)
        self.cumulative: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.rel_to_rcap: BpsPercentRatioPattern4 = BpsPercentRatioPattern4(client, _m(acc, 'rel_to_rcap'))

class BaseCumulativeSumPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, acc)
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'cumulative'))
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, _m(acc, 'sum'))

class BaseCumulativeSumPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern1[StoredU32] = MetricPattern1(client, acc)
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class BlocksDominanceRewardsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.blocks_mined: BaseCumulativeSumPattern2 = BaseCumulativeSumPattern2(client, _m(acc, 'blocks_mined'))
        self.dominance: _1m1w1y24hBpsPercentRatioPattern = _1m1w1y24hBpsPercentRatioPattern(client, _m(acc, 'dominance'))
        self.rewards: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, _m(acc, 'rewards'))

class BpsPercentRatioPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints16] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class BpsPercentRatioPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class BpsPriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, f'ratio_{disc}_bps'))
        self.price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, disc))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'ratio_{disc}'))

class BpsPercentRatioPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPointsSigned16] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class BpsPercentRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPointsSigned32] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class CentsSatsUsdPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern2[Cents] = MetricPattern2(client, _m(acc, 'cents'))
        self.sats: MetricPattern2[Sats] = MetricPattern2(client, _m(acc, 'sats'))
        self.usd: MetricPattern2[Dollars] = MetricPattern2(client, acc)

class CentsDeltaUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.delta: AbsoluteRatePattern2 = AbsoluteRatePattern2(client, _m(acc, 'delta'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class CentsSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class DeltaHalfTotalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.half: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'half'))
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)

class GreedNetPainPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'greed_index'))
        self.net: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_sentiment'))
        self.pain_index: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'pain_index'))

class LossNuplProfitPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.loss: BaseCumulativeNegativeSumPattern = BaseCumulativeNegativeSumPattern(client, acc, '')
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'nupl'))
        self.profit: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, _m(acc, 'unrealized_profit'))

class LowerPriceUpperPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.lower_price_band: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.price: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, _m(acc, 'investor_price'))
        self.upper_price_band: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'upper_price_band'))

class RatioValuePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, f'{acc}_ratio')
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, f'{acc}_value_created')
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, f'{acc}_value_destroyed')

class RatioValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: _24hPattern = _24hPattern(client, _m(acc, 'sopr_24h'))
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'value_created'))
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'value_destroyed'))

class _6bBlockTxPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._6b: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(client, _m(acc, '6b'))
        self.block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern2(client, acc)
        self.tx_index: MetricPattern19[T] = MetricPattern19(client, acc)

class BaseCumulativeSumPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern1[T] = MetricPattern1(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: _1m1w1y24hPattern[T] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class AbsoluteRatePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.absolute: _1m1w1y24hPattern[StoredI64] = _1m1w1y24hPattern(client, acc)
        self.rate: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, _m(acc, 'rate'))

class AbsoluteRatePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.absolute: _1m1w1y24hPattern3 = _1m1w1y24hPattern3(client, acc)
        self.rate: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, _m(acc, 'rate'))

class AllSthPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.all: BtcCentsDeltaSatsUsdPattern = BtcCentsDeltaSatsUsdPattern(client, _m(acc, 'supply'))
        self.sth: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'sth_supply'))

class AllSthPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.all: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.sth: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'sth_realized_cap'))

class BlocksDominancePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.blocks_mined: BaseCumulativeSumPattern2 = BaseCumulativeSumPattern2(client, _m(acc, 'blocks_mined'))
        self.dominance: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, _m(acc, 'dominance'))

class BpsRatioPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'bps'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)

class BpsRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPointsSigned32] = MetricPattern1(client, _m(acc, 'bps'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)

class CentsUsdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class CentsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'cents'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class CoindaysSentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coindays_destroyed: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, _m(acc, 'coindays_destroyed'))
        self.sent: BaseCumulativeInSumPattern = BaseCumulativeInSumPattern(client, _m(acc, 'sent'))

class DeltaInnerPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, _m(acc, 'delta'))
        self.inner: MetricPattern1[StoredU64] = MetricPattern1(client, acc)

class InPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.in_loss: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'loss'))
        self.in_profit: CentsUsdPattern2 = CentsUsdPattern2(client, _m(acc, 'profit'))

class PriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, disc))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, f'ratio_{disc}'))

class RelPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.rel_to_mcap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'mcap'))
        self.rel_to_rcap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'rcap'))

class SdSmaPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_sd')
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, f'{acc}_sma')

class ValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'created'))
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, _m(acc, 'destroyed'))

class _24hPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._24h: MetricPattern1[StoredF64] = MetricPattern1(client, acc)

class NuplPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, acc)

class UnspentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.unspent_count: DeltaInnerPattern = DeltaInnerPattern(client, acc)

# Metrics tree classes

class MetricsTree_Blocks_Difficulty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.value: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty')
        self.as_hash: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty_as_hash')
        self.adjustment: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'difficulty_adjustment')
        self.epoch: MetricPattern1[Epoch] = MetricPattern1(client, 'difficulty_epoch')
        self.blocks_before_next: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_difficulty_adjustment')
        self.days_before_next: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_difficulty_adjustment')

class MetricsTree_Blocks_Time:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.timestamp: MetricPattern1[Timestamp] = MetricPattern1(client, 'timestamp')
        self.date: MetricPattern18[Date] = MetricPattern18(client, 'date')
        self.timestamp_monotonic: MetricPattern18[Timestamp] = MetricPattern18(client, 'timestamp_monotonic')

class MetricsTree_Blocks_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total: MetricPattern18[StoredU64] = MetricPattern18(client, 'total_size')
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_size_cumulative')
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_sum')
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_average')
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_min')
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_max')
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_pct10')
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_pct25')
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_median')
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_pct75')
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_pct90')

class MetricsTree_Blocks_Weight:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricPattern18[Weight] = MetricPattern18(client, 'block_weight')
        self.cumulative: MetricPattern1[Weight] = MetricPattern1(client, 'block_weight_cumulative')
        self.sum: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_sum')
        self.average: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_average')
        self.min: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_min')
        self.max: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_max')
        self.pct10: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_pct10')
        self.pct25: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_pct25')
        self.median: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_median')
        self.pct75: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_pct75')
        self.pct90: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_pct90')

class MetricsTree_Blocks_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.target: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_count_target')
        self.total: BaseCumulativeSumPattern2 = BaseCumulativeSumPattern2(client, 'block_count')

class MetricsTree_Blocks_Lookback:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1h: MetricPattern18[Height] = MetricPattern18(client, 'height_1h_ago')
        self._24h: MetricPattern18[Height] = MetricPattern18(client, 'height_24h_ago')
        self._3d: MetricPattern18[Height] = MetricPattern18(client, 'height_3d_ago')
        self._1w: MetricPattern18[Height] = MetricPattern18(client, 'height_1w_ago')
        self._8d: MetricPattern18[Height] = MetricPattern18(client, 'height_8d_ago')
        self._9d: MetricPattern18[Height] = MetricPattern18(client, 'height_9d_ago')
        self._12d: MetricPattern18[Height] = MetricPattern18(client, 'height_12d_ago')
        self._13d: MetricPattern18[Height] = MetricPattern18(client, 'height_13d_ago')
        self._2w: MetricPattern18[Height] = MetricPattern18(client, 'height_2w_ago')
        self._21d: MetricPattern18[Height] = MetricPattern18(client, 'height_21d_ago')
        self._26d: MetricPattern18[Height] = MetricPattern18(client, 'height_26d_ago')
        self._1m: MetricPattern18[Height] = MetricPattern18(client, 'height_1m_ago')
        self._34d: MetricPattern18[Height] = MetricPattern18(client, 'height_34d_ago')
        self._55d: MetricPattern18[Height] = MetricPattern18(client, 'height_55d_ago')
        self._2m: MetricPattern18[Height] = MetricPattern18(client, 'height_2m_ago')
        self._9w: MetricPattern18[Height] = MetricPattern18(client, 'height_9w_ago')
        self._12w: MetricPattern18[Height] = MetricPattern18(client, 'height_12w_ago')
        self._89d: MetricPattern18[Height] = MetricPattern18(client, 'height_89d_ago')
        self._3m: MetricPattern18[Height] = MetricPattern18(client, 'height_3m_ago')
        self._14w: MetricPattern18[Height] = MetricPattern18(client, 'height_14w_ago')
        self._111d: MetricPattern18[Height] = MetricPattern18(client, 'height_111d_ago')
        self._144d: MetricPattern18[Height] = MetricPattern18(client, 'height_144d_ago')
        self._6m: MetricPattern18[Height] = MetricPattern18(client, 'height_6m_ago')
        self._26w: MetricPattern18[Height] = MetricPattern18(client, 'height_26w_ago')
        self._200d: MetricPattern18[Height] = MetricPattern18(client, 'height_200d_ago')
        self._9m: MetricPattern18[Height] = MetricPattern18(client, 'height_9m_ago')
        self._350d: MetricPattern18[Height] = MetricPattern18(client, 'height_350d_ago')
        self._12m: MetricPattern18[Height] = MetricPattern18(client, 'height_12m_ago')
        self._1y: MetricPattern18[Height] = MetricPattern18(client, 'height_1y_ago')
        self._14m: MetricPattern18[Height] = MetricPattern18(client, 'height_14m_ago')
        self._2y: MetricPattern18[Height] = MetricPattern18(client, 'height_2y_ago')
        self._26m: MetricPattern18[Height] = MetricPattern18(client, 'height_26m_ago')
        self._3y: MetricPattern18[Height] = MetricPattern18(client, 'height_3y_ago')
        self._200w: MetricPattern18[Height] = MetricPattern18(client, 'height_200w_ago')
        self._4y: MetricPattern18[Height] = MetricPattern18(client, 'height_4y_ago')
        self._5y: MetricPattern18[Height] = MetricPattern18(client, 'height_5y_ago')
        self._6y: MetricPattern18[Height] = MetricPattern18(client, 'height_6y_ago')
        self._8y: MetricPattern18[Height] = MetricPattern18(client, 'height_8y_ago')
        self._9y: MetricPattern18[Height] = MetricPattern18(client, 'height_9y_ago')
        self._10y: MetricPattern18[Height] = MetricPattern18(client, 'height_10y_ago')
        self._12y: MetricPattern18[Height] = MetricPattern18(client, 'height_12y_ago')
        self._14y: MetricPattern18[Height] = MetricPattern18(client, 'height_14y_ago')
        self._26y: MetricPattern18[Height] = MetricPattern18(client, 'height_26y_ago')

class MetricsTree_Blocks_Fullness:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bps: _1m1w1y24hHeightPattern[BasisPoints16] = _1m1w1y24hHeightPattern(client, 'block_fullness_bps')
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, 'block_fullness_ratio')
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, 'block_fullness')

class MetricsTree_Blocks_Halving:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.epoch: MetricPattern1[Halving] = MetricPattern1(client, 'halving_epoch')
        self.blocks_before_next: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_halving')
        self.days_before_next: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_halving')

class MetricsTree_Blocks:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern18[BlockHash] = MetricPattern18(client, 'blockhash')
        self.difficulty: MetricsTree_Blocks_Difficulty = MetricsTree_Blocks_Difficulty(client)
        self.time: MetricsTree_Blocks_Time = MetricsTree_Blocks_Time(client)
        self.size: MetricsTree_Blocks_Size = MetricsTree_Blocks_Size(client)
        self.weight: MetricsTree_Blocks_Weight = MetricsTree_Blocks_Weight(client)
        self.count: MetricsTree_Blocks_Count = MetricsTree_Blocks_Count(client)
        self.lookback: MetricsTree_Blocks_Lookback = MetricsTree_Blocks_Lookback(client)
        self.interval: _1m1w1y24hHeightPattern[Timestamp] = _1m1w1y24hHeightPattern(client, 'block_interval')
        self.vbytes: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'block_vbytes')
        self.fullness: MetricsTree_Blocks_Fullness = MetricsTree_Blocks_Fullness(client)
        self.halving: MetricsTree_Blocks_Halving = MetricsTree_Blocks_Halving(client)

class MetricsTree_Transactions_Raw:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_tx_index: MetricPattern18[TxIndex] = MetricPattern18(client, 'first_tx_index')
        self.height: MetricPattern19[Height] = MetricPattern19(client, 'height')
        self.txid: MetricPattern19[Txid] = MetricPattern19(client, 'txid')
        self.tx_version: MetricPattern19[TxVersion] = MetricPattern19(client, 'tx_version')
        self.raw_locktime: MetricPattern19[RawLockTime] = MetricPattern19(client, 'raw_locktime')
        self.base_size: MetricPattern19[StoredU32] = MetricPattern19(client, 'base_size')
        self.total_size: MetricPattern19[StoredU32] = MetricPattern19(client, 'total_size')
        self.is_explicitly_rbf: MetricPattern19[StoredBool] = MetricPattern19(client, 'is_explicitly_rbf')
        self.first_txin_index: MetricPattern19[TxInIndex] = MetricPattern19(client, 'first_txin_index')
        self.first_txout_index: MetricPattern19[TxOutIndex] = MetricPattern19(client, 'first_txout_index')

class MetricsTree_Transactions_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'tx_count')
        self.is_coinbase: MetricPattern19[StoredBool] = MetricPattern19(client, 'is_coinbase')

class MetricsTree_Transactions_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: _6bBlockTxPattern[VSize] = _6bBlockTxPattern(client, 'tx_vsize')
        self.weight: _6bBlockTxPattern[Weight] = _6bBlockTxPattern(client, 'tx_weight')

class MetricsTree_Transactions_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.input_value: MetricPattern19[Sats] = MetricPattern19(client, 'input_value')
        self.output_value: MetricPattern19[Sats] = MetricPattern19(client, 'output_value')
        self.fee: _6bBlockTxPattern[Sats] = _6bBlockTxPattern(client, 'fee')
        self.fee_rate: _6bBlockTxPattern[FeeRate] = _6bBlockTxPattern(client, 'fee_rate')

class MetricsTree_Transactions_Versions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'tx_v1')
        self.v2: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'tx_v2')
        self.v3: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'tx_v3')

class MetricsTree_Transactions_Volume:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sent_sum: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'sent_sum')
        self.received_sum: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'received_sum')
        self.tx_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'tx_per_sec')
        self.outputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'outputs_per_sec')
        self.inputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'inputs_per_sec')

class MetricsTree_Transactions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricsTree_Transactions_Raw = MetricsTree_Transactions_Raw(client)
        self.count: MetricsTree_Transactions_Count = MetricsTree_Transactions_Count(client)
        self.size: MetricsTree_Transactions_Size = MetricsTree_Transactions_Size(client)
        self.fees: MetricsTree_Transactions_Fees = MetricsTree_Transactions_Fees(client)
        self.versions: MetricsTree_Transactions_Versions = MetricsTree_Transactions_Versions(client)
        self.volume: MetricsTree_Transactions_Volume = MetricsTree_Transactions_Volume(client)

class MetricsTree_Inputs_Raw:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txin_index: MetricPattern18[TxInIndex] = MetricPattern18(client, 'first_txin_index')
        self.outpoint: MetricPattern20[OutPoint] = MetricPattern20(client, 'outpoint')
        self.tx_index: MetricPattern20[TxIndex] = MetricPattern20(client, 'tx_index')
        self.output_type: MetricPattern20[OutputType] = MetricPattern20(client, 'output_type')
        self.type_index: MetricPattern20[TypeIndex] = MetricPattern20(client, 'type_index')

class MetricsTree_Inputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txout_index: MetricPattern20[TxOutIndex] = MetricPattern20(client, 'txout_index')
        self.value: MetricPattern20[Sats] = MetricPattern20(client, 'value')

class MetricsTree_Inputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricsTree_Inputs_Raw = MetricsTree_Inputs_Raw(client)
        self.spent: MetricsTree_Inputs_Spent = MetricsTree_Inputs_Spent(client)
        self.count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'input_count')

class MetricsTree_Outputs_Raw:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txout_index: MetricPattern18[TxOutIndex] = MetricPattern18(client, 'first_txout_index')
        self.value: MetricPattern21[Sats] = MetricPattern21(client, 'value')
        self.output_type: MetricPattern21[OutputType] = MetricPattern21(client, 'output_type')
        self.type_index: MetricPattern21[TypeIndex] = MetricPattern21(client, 'type_index')
        self.tx_index: MetricPattern21[TxIndex] = MetricPattern21(client, 'tx_index')

class MetricsTree_Outputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txin_index: MetricPattern21[TxInIndex] = MetricPattern21(client, 'txin_index')

class MetricsTree_Outputs_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'output_count')
        self.unspent: MetricPattern1[StoredU64] = MetricPattern1(client, 'exact_utxo_count')

class MetricsTree_Outputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricsTree_Outputs_Raw = MetricsTree_Outputs_Raw(client)
        self.spent: MetricsTree_Outputs_Spent = MetricsTree_Outputs_Spent(client)
        self.count: MetricsTree_Outputs_Count = MetricsTree_Outputs_Count(client)

class MetricsTree_Addresses_Raw_P2pk65:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2PK65AddressIndex] = MetricPattern18(client, 'first_p2pk65_address_index')
        self.bytes: MetricPattern27[P2PK65Bytes] = MetricPattern27(client, 'p2pk65_bytes')

class MetricsTree_Addresses_Raw_P2pk33:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2PK33AddressIndex] = MetricPattern18(client, 'first_p2pk33_address_index')
        self.bytes: MetricPattern26[P2PK33Bytes] = MetricPattern26(client, 'p2pk33_bytes')

class MetricsTree_Addresses_Raw_P2pkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2PKHAddressIndex] = MetricPattern18(client, 'first_p2pkh_address_index')
        self.bytes: MetricPattern28[P2PKHBytes] = MetricPattern28(client, 'p2pkh_bytes')

class MetricsTree_Addresses_Raw_P2sh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2SHAddressIndex] = MetricPattern18(client, 'first_p2sh_address_index')
        self.bytes: MetricPattern29[P2SHBytes] = MetricPattern29(client, 'p2sh_bytes')

class MetricsTree_Addresses_Raw_P2wpkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2WPKHAddressIndex] = MetricPattern18(client, 'first_p2wpkh_address_index')
        self.bytes: MetricPattern31[P2WPKHBytes] = MetricPattern31(client, 'p2wpkh_bytes')

class MetricsTree_Addresses_Raw_P2wsh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2WSHAddressIndex] = MetricPattern18(client, 'first_p2wsh_address_index')
        self.bytes: MetricPattern32[P2WSHBytes] = MetricPattern32(client, 'p2wsh_bytes')

class MetricsTree_Addresses_Raw_P2tr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2TRAddressIndex] = MetricPattern18(client, 'first_p2tr_address_index')
        self.bytes: MetricPattern30[P2TRBytes] = MetricPattern30(client, 'p2tr_bytes')

class MetricsTree_Addresses_Raw_P2a:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2AAddressIndex] = MetricPattern18(client, 'first_p2a_address_index')
        self.bytes: MetricPattern24[P2ABytes] = MetricPattern24(client, 'p2a_bytes')

class MetricsTree_Addresses_Raw:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk65: MetricsTree_Addresses_Raw_P2pk65 = MetricsTree_Addresses_Raw_P2pk65(client)
        self.p2pk33: MetricsTree_Addresses_Raw_P2pk33 = MetricsTree_Addresses_Raw_P2pk33(client)
        self.p2pkh: MetricsTree_Addresses_Raw_P2pkh = MetricsTree_Addresses_Raw_P2pkh(client)
        self.p2sh: MetricsTree_Addresses_Raw_P2sh = MetricsTree_Addresses_Raw_P2sh(client)
        self.p2wpkh: MetricsTree_Addresses_Raw_P2wpkh = MetricsTree_Addresses_Raw_P2wpkh(client)
        self.p2wsh: MetricsTree_Addresses_Raw_P2wsh = MetricsTree_Addresses_Raw_P2wsh(client)
        self.p2tr: MetricsTree_Addresses_Raw_P2tr = MetricsTree_Addresses_Raw_P2tr(client)
        self.p2a: MetricsTree_Addresses_Raw_P2a = MetricsTree_Addresses_Raw_P2a(client)

class MetricsTree_Addresses_Indexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern24[AnyAddressIndex] = MetricPattern24(client, 'any_address_index')
        self.p2pk33: MetricPattern26[AnyAddressIndex] = MetricPattern26(client, 'any_address_index')
        self.p2pk65: MetricPattern27[AnyAddressIndex] = MetricPattern27(client, 'any_address_index')
        self.p2pkh: MetricPattern28[AnyAddressIndex] = MetricPattern28(client, 'any_address_index')
        self.p2sh: MetricPattern29[AnyAddressIndex] = MetricPattern29(client, 'any_address_index')
        self.p2tr: MetricPattern30[AnyAddressIndex] = MetricPattern30(client, 'any_address_index')
        self.p2wpkh: MetricPattern31[AnyAddressIndex] = MetricPattern31(client, 'any_address_index')
        self.p2wsh: MetricPattern32[AnyAddressIndex] = MetricPattern32(client, 'any_address_index')
        self.funded: MetricPattern34[FundedAddressIndex] = MetricPattern34(client, 'funded_address_index')
        self.empty: MetricPattern35[EmptyAddressIndex] = MetricPattern35(client, 'empty_address_index')

class MetricsTree_Addresses_Data:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.funded: MetricPattern34[FundedAddressData] = MetricPattern34(client, 'funded_address_data')
        self.empty: MetricPattern35[EmptyAddressData] = MetricPattern35(client, 'empty_address_data')

class MetricsTree_Addresses_Activity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'address_activity')
        self.p2pk65: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2pk65_address_activity')
        self.p2pk33: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2pk33_address_activity')
        self.p2pkh: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2pkh_address_activity')
        self.p2sh: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2sh_address_activity')
        self.p2wpkh: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2wpkh_address_activity')
        self.p2wsh: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2wsh_address_activity')
        self.p2tr: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2tr_address_activity')
        self.p2a: BothReactivatedReceivingSendingPattern = BothReactivatedReceivingSendingPattern(client, 'p2a_address_activity')

class MetricsTree_Addresses_New:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'new_address_count')
        self.p2pk65: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pk65_new_address_count')
        self.p2pk33: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pk33_new_address_count')
        self.p2pkh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pkh_new_address_count')
        self.p2sh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2sh_new_address_count')
        self.p2wpkh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2wpkh_new_address_count')
        self.p2wsh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2wsh_new_address_count')
        self.p2tr: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2tr_new_address_count')
        self.p2a: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2a_new_address_count')

class MetricsTree_Addresses_Delta:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: AbsoluteRatePattern = AbsoluteRatePattern(client, 'address_count')
        self.p2pk65: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2pk65_address_count')
        self.p2pk33: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2pk33_address_count')
        self.p2pkh: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2pkh_address_count')
        self.p2sh: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2sh_address_count')
        self.p2wpkh: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2wpkh_address_count')
        self.p2wsh: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2wsh_address_count')
        self.p2tr: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2tr_address_count')
        self.p2a: AbsoluteRatePattern = AbsoluteRatePattern(client, 'p2a_address_count')

class MetricsTree_Addresses:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricsTree_Addresses_Raw = MetricsTree_Addresses_Raw(client)
        self.indexes: MetricsTree_Addresses_Indexes = MetricsTree_Addresses_Indexes(client)
        self.data: MetricsTree_Addresses_Data = MetricsTree_Addresses_Data(client)
        self.funded: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3 = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(client, 'address_count')
        self.empty: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3 = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(client, 'empty_address_count')
        self.activity: MetricsTree_Addresses_Activity = MetricsTree_Addresses_Activity(client)
        self.total: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3 = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern3(client, 'total_address_count')
        self.new: MetricsTree_Addresses_New = MetricsTree_Addresses_New(client)
        self.delta: MetricsTree_Addresses_Delta = MetricsTree_Addresses_Delta(client)

class MetricsTree_Scripts_Raw_Empty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[EmptyOutputIndex] = MetricPattern18(client, 'first_empty_output_index')
        self.to_tx_index: MetricPattern22[TxIndex] = MetricPattern22(client, 'tx_index')

class MetricsTree_Scripts_Raw_OpReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[OpReturnIndex] = MetricPattern18(client, 'first_op_return_index')
        self.to_tx_index: MetricPattern23[TxIndex] = MetricPattern23(client, 'tx_index')

class MetricsTree_Scripts_Raw_P2ms:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[P2MSOutputIndex] = MetricPattern18(client, 'first_p2ms_output_index')
        self.to_tx_index: MetricPattern25[TxIndex] = MetricPattern25(client, 'tx_index')

class MetricsTree_Scripts_Raw_Unknown:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_index: MetricPattern18[UnknownOutputIndex] = MetricPattern18(client, 'first_unknown_output_index')
        self.to_tx_index: MetricPattern33[TxIndex] = MetricPattern33(client, 'tx_index')

class MetricsTree_Scripts_Raw:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: MetricsTree_Scripts_Raw_Empty = MetricsTree_Scripts_Raw_Empty(client)
        self.op_return: MetricsTree_Scripts_Raw_OpReturn = MetricsTree_Scripts_Raw_OpReturn(client)
        self.p2ms: MetricsTree_Scripts_Raw_P2ms = MetricsTree_Scripts_Raw_P2ms(client)
        self.unknown: MetricsTree_Scripts_Raw_Unknown = MetricsTree_Scripts_Raw_Unknown(client)

class MetricsTree_Scripts_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2a_count')
        self.p2ms: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2ms_count')
        self.p2pk33: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pk33_count')
        self.p2pk65: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pk65_count')
        self.p2pkh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2pkh_count')
        self.p2sh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2sh_count')
        self.p2tr: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2tr_count')
        self.p2wpkh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2wpkh_count')
        self.p2wsh: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'p2wsh_count')
        self.op_return: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'op_return_count')
        self.empty_output: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'empty_output_count')
        self.unknown_output: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'unknown_output_count')
        self.segwit: BaseCumulativeSumPattern[StoredU64] = BaseCumulativeSumPattern(client, 'segwit_count')

class MetricsTree_Scripts_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.op_return: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'op_return_value')

class MetricsTree_Scripts_Adoption:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.taproot: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'taproot_adoption')
        self.segwit: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'segwit_adoption')

class MetricsTree_Scripts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricsTree_Scripts_Raw = MetricsTree_Scripts_Raw(client)
        self.count: MetricsTree_Scripts_Count = MetricsTree_Scripts_Count(client)
        self.value: MetricsTree_Scripts_Value = MetricsTree_Scripts_Value(client)
        self.adoption: MetricsTree_Scripts_Adoption = MetricsTree_Scripts_Adoption(client)

class MetricsTree_Mining_Rewards_Subsidy:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'subsidy')
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'subsidy_cumulative')
        self.dominance: _1m1w1y24hBpsPercentRatioPattern = _1m1w1y24hBpsPercentRatioPattern(client, 'subsidy_dominance')
        self.sma_1y: CentsUsdPattern2 = CentsUsdPattern2(client, 'subsidy_sma_1y')

class MetricsTree_Mining_Rewards_Fees_RatioMultiple:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: BpsRatioPattern2 = BpsRatioPattern2(client, 'fee_ratio_multiple_24h')
        self._1w: BpsRatioPattern2 = BpsRatioPattern2(client, 'fee_ratio_multiple_1w')
        self._1m: BpsRatioPattern2 = BpsRatioPattern2(client, 'fee_ratio_multiple_1m')
        self._1y: BpsRatioPattern2 = BpsRatioPattern2(client, 'fee_ratio_multiple_1y')

class MetricsTree_Mining_Rewards_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'fees')
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'fees_cumulative')
        self.sum: _1m1w1y24hPattern5 = _1m1w1y24hPattern5(client, 'fees_sum')
        self._24h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'fees_24h')
        self._1w: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'fees_1w')
        self._1m: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'fees_1m')
        self._1y: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'fees_1y')
        self.dominance: _1m1w1y24hBpsPercentRatioPattern = _1m1w1y24hBpsPercentRatioPattern(client, 'fee_dominance')
        self.ratio_multiple: MetricsTree_Mining_Rewards_Fees_RatioMultiple = MetricsTree_Mining_Rewards_Fees_RatioMultiple(client)

class MetricsTree_Mining_Rewards:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinbase: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'coinbase')
        self.subsidy: MetricsTree_Mining_Rewards_Subsidy = MetricsTree_Mining_Rewards_Subsidy(client)
        self.fees: MetricsTree_Mining_Rewards_Fees = MetricsTree_Mining_Rewards_Fees(client)
        self.unclaimed: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'unclaimed_rewards')

class MetricsTree_Mining_Hashrate_Rate_Sma:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1w')
        self._1m: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1m')
        self._2m: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_2m')
        self._1y: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1y')

class MetricsTree_Mining_Hashrate_Rate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate')
        self.sma: MetricsTree_Mining_Hashrate_Rate_Sma = MetricsTree_Mining_Hashrate_Rate_Sma(client)
        self.ath: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_ath')
        self.drawdown: BpsPercentRatioPattern5 = BpsPercentRatioPattern5(client, 'hash_rate_drawdown')

class MetricsTree_Mining_Hashrate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.rate: MetricsTree_Mining_Hashrate_Rate = MetricsTree_Mining_Hashrate_Rate(client)
        self.price: PhsReboundThsPattern = PhsReboundThsPattern(client, 'hash_price')
        self.value: PhsReboundThsPattern = PhsReboundThsPattern(client, 'hash_value')

class MetricsTree_Mining:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.rewards: MetricsTree_Mining_Rewards = MetricsTree_Mining_Rewards(client)
        self.hashrate: MetricsTree_Mining_Hashrate = MetricsTree_Mining_Hashrate(client)

class MetricsTree_Cointime_Activity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinblocks_created: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'coinblocks_created')
        self.coinblocks_stored: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, 'liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, 'vaultedness')
        self.ratio: MetricPattern1[StoredF64] = MetricPattern1(client, 'activity_to_vaultedness_ratio')

class MetricsTree_Cointime_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'vaulted_supply')
        self.active: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'active_supply')

class MetricsTree_Cointime_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.destroyed: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'cointime_value_destroyed')
        self.created: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'cointime_value_created')
        self.stored: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'cointime_value_stored')
        self.vocdd: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'vocdd')

class MetricsTree_Cointime_Cap:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.thermo: CentsUsdPattern2 = CentsUsdPattern2(client, 'thermo_cap')
        self.investor: CentsUsdPattern2 = CentsUsdPattern2(client, 'investor_cap')
        self.vaulted: CentsUsdPattern2 = CentsUsdPattern2(client, 'vaulted_cap')
        self.active: CentsUsdPattern2 = CentsUsdPattern2(client, 'active_cap')
        self.cointime: CentsUsdPattern2 = CentsUsdPattern2(client, 'cointime_cap')
        self.aviv: BpsRatioPattern2 = BpsRatioPattern2(client, 'aviv_ratio')

class MetricsTree_Cointime_Prices:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'vaulted_price')
        self.active: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'active_price')
        self.true_market_mean: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'true_market_mean')
        self.cointime: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'cointime_price')
        self.transfer: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'transfer_price')
        self.balanced: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'balanced_price')
        self.terminal: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'terminal_price')
        self.delta: BpsCentsPercentilesRatioSatsUsdPattern = BpsCentsPercentilesRatioSatsUsdPattern(client, 'delta_price')
        self.cumulative_market_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'cumulative_market_cap')

class MetricsTree_Cointime_Adjusted:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.inflation_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'cointime_adj_inflation_rate')
        self.tx_velocity_native: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_velocity')
        self.tx_velocity_fiat: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_velocity_fiat')

class MetricsTree_Cointime_ReserveRisk:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.value: MetricPattern1[StoredF64] = MetricPattern1(client, 'reserve_risk')
        self.vocdd_median_1y: MetricPattern18[StoredF64] = MetricPattern18(client, 'vocdd_median_1y')
        self.hodl_bank: MetricPattern18[StoredF64] = MetricPattern18(client, 'hodl_bank')

class MetricsTree_Cointime:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: MetricsTree_Cointime_Activity = MetricsTree_Cointime_Activity(client)
        self.supply: MetricsTree_Cointime_Supply = MetricsTree_Cointime_Supply(client)
        self.value: MetricsTree_Cointime_Value = MetricsTree_Cointime_Value(client)
        self.cap: MetricsTree_Cointime_Cap = MetricsTree_Cointime_Cap(client)
        self.prices: MetricsTree_Cointime_Prices = MetricsTree_Cointime_Prices(client)
        self.adjusted: MetricsTree_Cointime_Adjusted = MetricsTree_Cointime_Adjusted(client)
        self.reserve_risk: MetricsTree_Cointime_ReserveRisk = MetricsTree_Cointime_ReserveRisk(client)
        self.coinblocks_destroyed: BaseCumulativeSumPattern[StoredF64] = BaseCumulativeSumPattern(client, 'coinblocks_destroyed')

class MetricsTree_Constants:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_0')
        self._1: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_1')
        self._2: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_2')
        self._3: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_3')
        self._4: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_4')
        self._20: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_20')
        self._30: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_30')
        self._38_2: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_38_2')
        self._50: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_50')
        self._61_8: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_61_8')
        self._70: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_70')
        self._80: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_80')
        self._100: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_100')
        self._600: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_600')
        self.minus_1: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_1')
        self.minus_2: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_2')
        self.minus_3: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_3')
        self.minus_4: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_4')

class MetricsTree_Indexes_Address_P2pk33:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern26[P2PK33AddressIndex] = MetricPattern26(client, 'p2pk33_address_index')
        self.address: MetricPattern26[Address] = MetricPattern26(client, 'p2pk33_address')

class MetricsTree_Indexes_Address_P2pk65:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern27[P2PK65AddressIndex] = MetricPattern27(client, 'p2pk65_address_index')
        self.address: MetricPattern27[Address] = MetricPattern27(client, 'p2pk65_address')

class MetricsTree_Indexes_Address_P2pkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern28[P2PKHAddressIndex] = MetricPattern28(client, 'p2pkh_address_index')
        self.address: MetricPattern28[Address] = MetricPattern28(client, 'p2pkh_address')

class MetricsTree_Indexes_Address_P2sh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern29[P2SHAddressIndex] = MetricPattern29(client, 'p2sh_address_index')
        self.address: MetricPattern29[Address] = MetricPattern29(client, 'p2sh_address')

class MetricsTree_Indexes_Address_P2tr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern30[P2TRAddressIndex] = MetricPattern30(client, 'p2tr_address_index')
        self.address: MetricPattern30[Address] = MetricPattern30(client, 'p2tr_address')

class MetricsTree_Indexes_Address_P2wpkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern31[P2WPKHAddressIndex] = MetricPattern31(client, 'p2wpkh_address_index')
        self.address: MetricPattern31[Address] = MetricPattern31(client, 'p2wpkh_address')

class MetricsTree_Indexes_Address_P2wsh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern32[P2WSHAddressIndex] = MetricPattern32(client, 'p2wsh_address_index')
        self.address: MetricPattern32[Address] = MetricPattern32(client, 'p2wsh_address')

class MetricsTree_Indexes_Address_P2a:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern24[P2AAddressIndex] = MetricPattern24(client, 'p2a_address_index')
        self.address: MetricPattern24[Address] = MetricPattern24(client, 'p2a_address')

class MetricsTree_Indexes_Address_P2ms:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern25[P2MSOutputIndex] = MetricPattern25(client, 'p2ms_output_index')

class MetricsTree_Indexes_Address_Empty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern22[EmptyOutputIndex] = MetricPattern22(client, 'empty_output_index')

class MetricsTree_Indexes_Address_Unknown:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern33[UnknownOutputIndex] = MetricPattern33(client, 'unknown_output_index')

class MetricsTree_Indexes_Address_OpReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern23[OpReturnIndex] = MetricPattern23(client, 'op_return_index')

class MetricsTree_Indexes_Address:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk33: MetricsTree_Indexes_Address_P2pk33 = MetricsTree_Indexes_Address_P2pk33(client)
        self.p2pk65: MetricsTree_Indexes_Address_P2pk65 = MetricsTree_Indexes_Address_P2pk65(client)
        self.p2pkh: MetricsTree_Indexes_Address_P2pkh = MetricsTree_Indexes_Address_P2pkh(client)
        self.p2sh: MetricsTree_Indexes_Address_P2sh = MetricsTree_Indexes_Address_P2sh(client)
        self.p2tr: MetricsTree_Indexes_Address_P2tr = MetricsTree_Indexes_Address_P2tr(client)
        self.p2wpkh: MetricsTree_Indexes_Address_P2wpkh = MetricsTree_Indexes_Address_P2wpkh(client)
        self.p2wsh: MetricsTree_Indexes_Address_P2wsh = MetricsTree_Indexes_Address_P2wsh(client)
        self.p2a: MetricsTree_Indexes_Address_P2a = MetricsTree_Indexes_Address_P2a(client)
        self.p2ms: MetricsTree_Indexes_Address_P2ms = MetricsTree_Indexes_Address_P2ms(client)
        self.empty: MetricsTree_Indexes_Address_Empty = MetricsTree_Indexes_Address_Empty(client)
        self.unknown: MetricsTree_Indexes_Address_Unknown = MetricsTree_Indexes_Address_Unknown(client)
        self.op_return: MetricsTree_Indexes_Address_OpReturn = MetricsTree_Indexes_Address_OpReturn(client)

class MetricsTree_Indexes_Height:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern18[Height] = MetricPattern18(client, 'height')
        self.minute10: MetricPattern18[Minute10] = MetricPattern18(client, 'minute10')
        self.minute30: MetricPattern18[Minute30] = MetricPattern18(client, 'minute30')
        self.hour1: MetricPattern18[Hour1] = MetricPattern18(client, 'hour1')
        self.hour4: MetricPattern18[Hour4] = MetricPattern18(client, 'hour4')
        self.hour12: MetricPattern18[Hour12] = MetricPattern18(client, 'hour12')
        self.day1: MetricPattern18[Day1] = MetricPattern18(client, 'day1')
        self.day3: MetricPattern18[Day3] = MetricPattern18(client, 'day3')
        self.epoch: MetricPattern18[Epoch] = MetricPattern18(client, 'epoch')
        self.halving: MetricPattern18[Halving] = MetricPattern18(client, 'halving')
        self.week1: MetricPattern18[Week1] = MetricPattern18(client, 'week1')
        self.month1: MetricPattern18[Month1] = MetricPattern18(client, 'month1')
        self.month3: MetricPattern18[Month3] = MetricPattern18(client, 'month3')
        self.month6: MetricPattern18[Month6] = MetricPattern18(client, 'month6')
        self.year1: MetricPattern18[Year1] = MetricPattern18(client, 'year1')
        self.year10: MetricPattern18[Year10] = MetricPattern18(client, 'year10')
        self.tx_index_count: MetricPattern18[StoredU64] = MetricPattern18(client, 'tx_index_count')

class MetricsTree_Indexes_Epoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern17[Epoch] = MetricPattern17(client, 'epoch')
        self.first_height: MetricPattern17[Height] = MetricPattern17(client, 'first_height')
        self.height_count: MetricPattern17[StoredU64] = MetricPattern17(client, 'height_count')

class MetricsTree_Indexes_Halving:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern16[Halving] = MetricPattern16(client, 'halving')
        self.first_height: MetricPattern16[Height] = MetricPattern16(client, 'first_height')

class MetricsTree_Indexes_Minute10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern3[Minute10] = MetricPattern3(client, 'minute10_index')
        self.first_height: MetricPattern3[Height] = MetricPattern3(client, 'first_height')

class MetricsTree_Indexes_Minute30:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern4[Minute30] = MetricPattern4(client, 'minute30_index')
        self.first_height: MetricPattern4[Height] = MetricPattern4(client, 'first_height')

class MetricsTree_Indexes_Hour1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern5[Hour1] = MetricPattern5(client, 'hour1_index')
        self.first_height: MetricPattern5[Height] = MetricPattern5(client, 'first_height')

class MetricsTree_Indexes_Hour4:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern6[Hour4] = MetricPattern6(client, 'hour4_index')
        self.first_height: MetricPattern6[Height] = MetricPattern6(client, 'first_height')

class MetricsTree_Indexes_Hour12:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern7[Hour12] = MetricPattern7(client, 'hour12_index')
        self.first_height: MetricPattern7[Height] = MetricPattern7(client, 'first_height')

class MetricsTree_Indexes_Day1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern8[Day1] = MetricPattern8(client, 'day1_index')
        self.date: MetricPattern8[Date] = MetricPattern8(client, 'date')
        self.first_height: MetricPattern8[Height] = MetricPattern8(client, 'first_height')
        self.height_count: MetricPattern8[StoredU64] = MetricPattern8(client, 'height_count')

class MetricsTree_Indexes_Day3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern9[Day3] = MetricPattern9(client, 'day3_index')
        self.first_height: MetricPattern9[Height] = MetricPattern9(client, 'first_height')

class MetricsTree_Indexes_Week1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern10[Week1] = MetricPattern10(client, 'week1_index')
        self.date: MetricPattern10[Date] = MetricPattern10(client, 'date')
        self.first_height: MetricPattern10[Height] = MetricPattern10(client, 'first_height')

class MetricsTree_Indexes_Month1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern11[Month1] = MetricPattern11(client, 'month1_index')
        self.date: MetricPattern11[Date] = MetricPattern11(client, 'date')
        self.first_height: MetricPattern11[Height] = MetricPattern11(client, 'first_height')

class MetricsTree_Indexes_Month3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern12[Month3] = MetricPattern12(client, 'month3_index')
        self.date: MetricPattern12[Date] = MetricPattern12(client, 'date')
        self.first_height: MetricPattern12[Height] = MetricPattern12(client, 'first_height')

class MetricsTree_Indexes_Month6:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern13[Month6] = MetricPattern13(client, 'month6_index')
        self.date: MetricPattern13[Date] = MetricPattern13(client, 'date')
        self.first_height: MetricPattern13[Height] = MetricPattern13(client, 'first_height')

class MetricsTree_Indexes_Year1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern14[Year1] = MetricPattern14(client, 'year1_index')
        self.date: MetricPattern14[Date] = MetricPattern14(client, 'date')
        self.first_height: MetricPattern14[Height] = MetricPattern14(client, 'first_height')

class MetricsTree_Indexes_Year10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern15[Year10] = MetricPattern15(client, 'year10_index')
        self.date: MetricPattern15[Date] = MetricPattern15(client, 'date')
        self.first_height: MetricPattern15[Height] = MetricPattern15(client, 'first_height')

class MetricsTree_Indexes_TxIndex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern19[TxIndex] = MetricPattern19(client, 'tx_index')
        self.input_count: MetricPattern19[StoredU64] = MetricPattern19(client, 'input_count')
        self.output_count: MetricPattern19[StoredU64] = MetricPattern19(client, 'output_count')

class MetricsTree_Indexes_TxinIndex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern20[TxInIndex] = MetricPattern20(client, 'txin_index')

class MetricsTree_Indexes_TxoutIndex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern21[TxOutIndex] = MetricPattern21(client, 'txout_index')

class MetricsTree_Indexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: MetricsTree_Indexes_Address = MetricsTree_Indexes_Address(client)
        self.height: MetricsTree_Indexes_Height = MetricsTree_Indexes_Height(client)
        self.epoch: MetricsTree_Indexes_Epoch = MetricsTree_Indexes_Epoch(client)
        self.halving: MetricsTree_Indexes_Halving = MetricsTree_Indexes_Halving(client)
        self.minute10: MetricsTree_Indexes_Minute10 = MetricsTree_Indexes_Minute10(client)
        self.minute30: MetricsTree_Indexes_Minute30 = MetricsTree_Indexes_Minute30(client)
        self.hour1: MetricsTree_Indexes_Hour1 = MetricsTree_Indexes_Hour1(client)
        self.hour4: MetricsTree_Indexes_Hour4 = MetricsTree_Indexes_Hour4(client)
        self.hour12: MetricsTree_Indexes_Hour12 = MetricsTree_Indexes_Hour12(client)
        self.day1: MetricsTree_Indexes_Day1 = MetricsTree_Indexes_Day1(client)
        self.day3: MetricsTree_Indexes_Day3 = MetricsTree_Indexes_Day3(client)
        self.week1: MetricsTree_Indexes_Week1 = MetricsTree_Indexes_Week1(client)
        self.month1: MetricsTree_Indexes_Month1 = MetricsTree_Indexes_Month1(client)
        self.month3: MetricsTree_Indexes_Month3 = MetricsTree_Indexes_Month3(client)
        self.month6: MetricsTree_Indexes_Month6 = MetricsTree_Indexes_Month6(client)
        self.year1: MetricsTree_Indexes_Year1 = MetricsTree_Indexes_Year1(client)
        self.year10: MetricsTree_Indexes_Year10 = MetricsTree_Indexes_Year10(client)
        self.tx_index: MetricsTree_Indexes_TxIndex = MetricsTree_Indexes_TxIndex(client)
        self.txin_index: MetricsTree_Indexes_TxinIndex = MetricsTree_Indexes_TxinIndex(client)
        self.txout_index: MetricsTree_Indexes_TxoutIndex = MetricsTree_Indexes_TxoutIndex(client)

class MetricsTree_Indicators_Dormancy:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_adjusted: MetricPattern1[StoredF32] = MetricPattern1(client, 'dormancy_supply_adjusted')
        self.flow: MetricPattern1[StoredF32] = MetricPattern1(client, 'dormancy_flow')

class MetricsTree_Indicators:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.puell_multiple: BpsRatioPattern2 = BpsRatioPattern2(client, 'puell_multiple')
        self.nvt: BpsRatioPattern2 = BpsRatioPattern2(client, 'nvt')
        self.gini: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'gini')
        self.rhodl_ratio: BpsRatioPattern2 = BpsRatioPattern2(client, 'rhodl_ratio')
        self.thermocap_multiple: BpsRatioPattern2 = BpsRatioPattern2(client, 'thermocap_multiple')
        self.coindays_destroyed_supply_adjusted: MetricPattern1[StoredF32] = MetricPattern1(client, 'coindays_destroyed_supply_adjusted')
        self.coinyears_destroyed_supply_adjusted: MetricPattern1[StoredF32] = MetricPattern1(client, 'coinyears_destroyed_supply_adjusted')
        self.dormancy: MetricsTree_Indicators_Dormancy = MetricsTree_Indicators_Dormancy(client)
        self.stock_to_flow: MetricPattern1[StoredF32] = MetricPattern1(client, 'stock_to_flow')
        self.seller_exhaustion_constant: MetricPattern1[StoredF32] = MetricPattern1(client, 'seller_exhaustion_constant')

class MetricsTree_Market_Ath:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.high: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_ath')
        self.drawdown: BpsPercentRatioPattern5 = BpsPercentRatioPattern5(client, 'price_drawdown')
        self.days_since: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_since_price_ath')
        self.years_since: MetricPattern1[StoredF32] = MetricPattern1(client, 'years_since_price_ath')
        self.max_days_between: MetricPattern1[StoredF32] = MetricPattern1(client, 'max_days_between_price_ath')
        self.max_years_between: MetricPattern1[StoredF32] = MetricPattern1(client, 'max_years_between_price_ath')

class MetricsTree_Market_Lookback:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_24h')
        self._1w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_1w')
        self._1m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_1m')
        self._3m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_3m')
        self._6m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_6m')
        self._1y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_1y')
        self._2y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_2y')
        self._3y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_3y')
        self._4y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_4y')
        self._5y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_5y')
        self._6y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_6y')
        self._8y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_8y')
        self._10y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_lookback_10y')

class MetricsTree_Market_Returns_Periods:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_24h')
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_1w')
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_1m')
        self._3m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_3m')
        self._6m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_6m')
        self._1y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_1y')
        self._2y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_2y')
        self._3y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_3y')
        self._4y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_4y')
        self._5y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_5y')
        self._6y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_6y')
        self._8y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_8y')
        self._10y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_return_10y')

class MetricsTree_Market_Returns_Sd24h_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sma_1w')
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sd_1w')

class MetricsTree_Market_Returns_Sd24h_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sma_1m')
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sd_1m')

class MetricsTree_Market_Returns_Sd24h_1y:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sma_1y')
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sd_1y')

class MetricsTree_Market_Returns_Sd24h:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: MetricsTree_Market_Returns_Sd24h_1w = MetricsTree_Market_Returns_Sd24h_1w(client)
        self._1m: MetricsTree_Market_Returns_Sd24h_1m = MetricsTree_Market_Returns_Sd24h_1m(client)
        self._1y: MetricsTree_Market_Returns_Sd24h_1y = MetricsTree_Market_Returns_Sd24h_1y(client)

class MetricsTree_Market_Returns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.periods: MetricsTree_Market_Returns_Periods = MetricsTree_Market_Returns_Periods(client)
        self.cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'price_cagr')
        self.sd_24h: MetricsTree_Market_Returns_Sd24h = MetricsTree_Market_Returns_Sd24h(client)

class MetricsTree_Market_Volatility:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1w')
        self._1m: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1m')
        self._1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1y')

class MetricsTree_Market_Range:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.min: _1m1w1y2wPattern = _1m1w1y2wPattern(client, 'price_min')
        self.max: _1m1w1y2wPattern = _1m1w1y2wPattern(client, 'price_max')
        self.true_range: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range')
        self.true_range_sum_2w: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range_sum_2w')
        self.choppiness_index_2w: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'price_choppiness_index_2w')

class MetricsTree_Market_MovingAverage_Sma_200d:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, 'price_sma_200d')
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, 'price_sma_200d_cents')
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, 'price_sma_200d_sats')
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, 'price_sma_200d_ratio_bps')
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_sma_200d_ratio')
        self.x2_4: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_200d_x2_4')
        self.x0_8: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_200d_x0_8')

class MetricsTree_Market_MovingAverage_Sma_350d:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, 'price_sma_350d')
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, 'price_sma_350d_cents')
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, 'price_sma_350d_sats')
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, 'price_sma_350d_ratio_bps')
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_sma_350d_ratio')
        self.x2: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_350d_x2')

class MetricsTree_Market_MovingAverage_Sma:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_1w')
        self._8d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_8d')
        self._13d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_13d')
        self._21d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_21d')
        self._1m: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_1m')
        self._34d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_34d')
        self._55d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_55d')
        self._89d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_89d')
        self._111d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_111d')
        self._144d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_144d')
        self._200d: MetricsTree_Market_MovingAverage_Sma_200d = MetricsTree_Market_MovingAverage_Sma_200d(client)
        self._350d: MetricsTree_Market_MovingAverage_Sma_350d = MetricsTree_Market_MovingAverage_Sma_350d(client)
        self._1y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_1y')
        self._2y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_2y')
        self._200w: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_200w')
        self._4y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_sma_4y')

class MetricsTree_Market_MovingAverage_Ema:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_1w')
        self._8d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_8d')
        self._12d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_12d')
        self._13d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_13d')
        self._21d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_21d')
        self._26d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_26d')
        self._1m: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_1m')
        self._34d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_34d')
        self._55d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_55d')
        self._89d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_89d')
        self._144d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_144d')
        self._200d: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_200d')
        self._1y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_1y')
        self._2y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_2y')
        self._200w: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_200w')
        self._4y: BpsCentsRatioSatsUsdPattern = BpsCentsRatioSatsUsdPattern(client, 'price_ema_4y')

class MetricsTree_Market_MovingAverage:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricsTree_Market_MovingAverage_Sma = MetricsTree_Market_MovingAverage_Sma(client)
        self.ema: MetricsTree_Market_MovingAverage_Ema = MetricsTree_Market_MovingAverage_Ema(client)

class MetricsTree_Market_Dca_Period_CostBasis:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_1w')
        self._1m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_1m')
        self._3m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_3m')
        self._6m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_6m')
        self._1y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_1y')
        self._2y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_2y')
        self._3y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_3y')
        self._4y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_4y')
        self._5y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_5y')
        self._6y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_6y')
        self._8y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_8y')
        self._10y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_10y')

class MetricsTree_Market_Dca_Period:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'dca_stack')
        self.cost_basis: MetricsTree_Market_Dca_Period_CostBasis = MetricsTree_Market_Dca_Period_CostBasis(client)
        self.r#return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_return')
        self.cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'dca_cagr')
        self.lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'lump_sum_stack')
        self.lump_sum_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_return')

class MetricsTree_Market_Dca_Class_Stack:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.from_2015: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2015')
        self.from_2016: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2016')
        self.from_2017: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2017')
        self.from_2018: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2018')
        self.from_2019: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2019')
        self.from_2020: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2020')
        self.from_2021: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2021')
        self.from_2022: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2022')
        self.from_2023: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2023')
        self.from_2024: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2024')
        self.from_2025: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2025')
        self.from_2026: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'dca_stack_from_2026')

class MetricsTree_Market_Dca_Class_CostBasis:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.from_2015: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2015')
        self.from_2016: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2016')
        self.from_2017: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2017')
        self.from_2018: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2018')
        self.from_2019: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2019')
        self.from_2020: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2020')
        self.from_2021: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2021')
        self.from_2022: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2022')
        self.from_2023: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2023')
        self.from_2024: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2024')
        self.from_2025: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2025')
        self.from_2026: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'dca_cost_basis_from_2026')

class MetricsTree_Market_Dca_Class_Return:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.from_2015: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2015')
        self.from_2016: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2016')
        self.from_2017: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2017')
        self.from_2018: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2018')
        self.from_2019: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2019')
        self.from_2020: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2020')
        self.from_2021: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2021')
        self.from_2022: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2022')
        self.from_2023: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2023')
        self.from_2024: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2024')
        self.from_2025: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2025')
        self.from_2026: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'dca_return_from_2026')

class MetricsTree_Market_Dca_Class:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.stack: MetricsTree_Market_Dca_Class_Stack = MetricsTree_Market_Dca_Class_Stack(client)
        self.cost_basis: MetricsTree_Market_Dca_Class_CostBasis = MetricsTree_Market_Dca_Class_CostBasis(client)
        self.r#return: MetricsTree_Market_Dca_Class_Return = MetricsTree_Market_Dca_Class_Return(client)

class MetricsTree_Market_Dca:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sats_per_day: MetricPattern18[Sats] = MetricPattern18(client, 'dca_sats_per_day')
        self.period: MetricsTree_Market_Dca_Period = MetricsTree_Market_Dca_Period(client)
        self.class_: MetricsTree_Market_Dca_Class = MetricsTree_Market_Dca_Class(client)

class MetricsTree_Market_Technical_Rsi:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')
        self._1w: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')
        self._1m: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')
        self._1y: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')

class MetricsTree_Market_Technical_Macd_24h:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_24h')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_24h')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_24h')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_24h')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_24h')

class MetricsTree_Market_Technical_Macd_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1w')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1w')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1w')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1w')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1w')

class MetricsTree_Market_Technical_Macd_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1m')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1m')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1m')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1m')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1m')

class MetricsTree_Market_Technical_Macd_1y:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1y')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1y')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1y')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1y')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1y')

class MetricsTree_Market_Technical_Macd:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: MetricsTree_Market_Technical_Macd_24h = MetricsTree_Market_Technical_Macd_24h(client)
        self._1w: MetricsTree_Market_Technical_Macd_1w = MetricsTree_Market_Technical_Macd_1w(client)
        self._1m: MetricsTree_Market_Technical_Macd_1m = MetricsTree_Market_Technical_Macd_1m(client)
        self._1y: MetricsTree_Market_Technical_Macd_1y = MetricsTree_Market_Technical_Macd_1y(client)

class MetricsTree_Market_Technical:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.rsi: MetricsTree_Market_Technical_Rsi = MetricsTree_Market_Technical_Rsi(client)
        self.stoch_k: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'stoch_k')
        self.stoch_d: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'stoch_d')
        self.pi_cycle: BpsRatioPattern2 = BpsRatioPattern2(client, 'pi_cycle')
        self.macd: MetricsTree_Market_Technical_Macd = MetricsTree_Market_Technical_Macd(client)

class MetricsTree_Market:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ath: MetricsTree_Market_Ath = MetricsTree_Market_Ath(client)
        self.lookback: MetricsTree_Market_Lookback = MetricsTree_Market_Lookback(client)
        self.returns: MetricsTree_Market_Returns = MetricsTree_Market_Returns(client)
        self.volatility: MetricsTree_Market_Volatility = MetricsTree_Market_Volatility(client)
        self.range: MetricsTree_Market_Range = MetricsTree_Market_Range(client)
        self.moving_average: MetricsTree_Market_MovingAverage = MetricsTree_Market_MovingAverage(client)
        self.dca: MetricsTree_Market_Dca = MetricsTree_Market_Dca(client)
        self.technical: MetricsTree_Market_Technical = MetricsTree_Market_Technical(client)

class MetricsTree_Pools_Major:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.unknown: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'unknown')
        self.luxor: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'luxor')
        self.btccom: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'btccom')
        self.btctop: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'btctop')
        self.btcguild: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'btcguild')
        self.eligius: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'eligius')
        self.f2pool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'f2pool')
        self.braiinspool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'braiinspool')
        self.antpool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'antpool')
        self.btcc: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'btcc')
        self.bwpool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'bwpool')
        self.bitfury: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'bitfury')
        self.viabtc: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'viabtc')
        self.poolin: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'poolin')
        self.spiderpool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'spiderpool')
        self.binancepool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'binancepool')
        self.foundryusa: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'foundryusa')
        self.sbicrypto: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'sbicrypto')
        self.marapool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'marapool')
        self.secpool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'secpool')
        self.ocean: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'ocean')
        self.whitepool: BlocksDominanceRewardsPattern = BlocksDominanceRewardsPattern(client, 'whitepool')

class MetricsTree_Pools_Minor:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockfills: BlocksDominancePattern = BlocksDominancePattern(client, 'blockfills')
        self.ultimuspool: BlocksDominancePattern = BlocksDominancePattern(client, 'ultimuspool')
        self.terrapool: BlocksDominancePattern = BlocksDominancePattern(client, 'terrapool')
        self.onethash: BlocksDominancePattern = BlocksDominancePattern(client, 'onethash')
        self.bitfarms: BlocksDominancePattern = BlocksDominancePattern(client, 'bitfarms')
        self.huobipool: BlocksDominancePattern = BlocksDominancePattern(client, 'huobipool')
        self.wayicn: BlocksDominancePattern = BlocksDominancePattern(client, 'wayicn')
        self.canoepool: BlocksDominancePattern = BlocksDominancePattern(client, 'canoepool')
        self.bitcoincom: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoincom')
        self.pool175btc: BlocksDominancePattern = BlocksDominancePattern(client, 'pool175btc')
        self.gbminers: BlocksDominancePattern = BlocksDominancePattern(client, 'gbminers')
        self.axbt: BlocksDominancePattern = BlocksDominancePattern(client, 'axbt')
        self.asicminer: BlocksDominancePattern = BlocksDominancePattern(client, 'asicminer')
        self.bitminter: BlocksDominancePattern = BlocksDominancePattern(client, 'bitminter')
        self.bitcoinrussia: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoinrussia')
        self.btcserv: BlocksDominancePattern = BlocksDominancePattern(client, 'btcserv')
        self.simplecoinus: BlocksDominancePattern = BlocksDominancePattern(client, 'simplecoinus')
        self.ozcoin: BlocksDominancePattern = BlocksDominancePattern(client, 'ozcoin')
        self.eclipsemc: BlocksDominancePattern = BlocksDominancePattern(client, 'eclipsemc')
        self.maxbtc: BlocksDominancePattern = BlocksDominancePattern(client, 'maxbtc')
        self.triplemining: BlocksDominancePattern = BlocksDominancePattern(client, 'triplemining')
        self.coinlab: BlocksDominancePattern = BlocksDominancePattern(client, 'coinlab')
        self.pool50btc: BlocksDominancePattern = BlocksDominancePattern(client, 'pool50btc')
        self.ghashio: BlocksDominancePattern = BlocksDominancePattern(client, 'ghashio')
        self.stminingcorp: BlocksDominancePattern = BlocksDominancePattern(client, 'stminingcorp')
        self.bitparking: BlocksDominancePattern = BlocksDominancePattern(client, 'bitparking')
        self.mmpool: BlocksDominancePattern = BlocksDominancePattern(client, 'mmpool')
        self.polmine: BlocksDominancePattern = BlocksDominancePattern(client, 'polmine')
        self.kncminer: BlocksDominancePattern = BlocksDominancePattern(client, 'kncminer')
        self.bitalo: BlocksDominancePattern = BlocksDominancePattern(client, 'bitalo')
        self.hhtt: BlocksDominancePattern = BlocksDominancePattern(client, 'hhtt')
        self.megabigpower: BlocksDominancePattern = BlocksDominancePattern(client, 'megabigpower')
        self.mtred: BlocksDominancePattern = BlocksDominancePattern(client, 'mtred')
        self.nmcbit: BlocksDominancePattern = BlocksDominancePattern(client, 'nmcbit')
        self.yourbtcnet: BlocksDominancePattern = BlocksDominancePattern(client, 'yourbtcnet')
        self.givemecoins: BlocksDominancePattern = BlocksDominancePattern(client, 'givemecoins')
        self.multicoinco: BlocksDominancePattern = BlocksDominancePattern(client, 'multicoinco')
        self.bcpoolio: BlocksDominancePattern = BlocksDominancePattern(client, 'bcpoolio')
        self.cointerra: BlocksDominancePattern = BlocksDominancePattern(client, 'cointerra')
        self.kanopool: BlocksDominancePattern = BlocksDominancePattern(client, 'kanopool')
        self.solock: BlocksDominancePattern = BlocksDominancePattern(client, 'solock')
        self.ckpool: BlocksDominancePattern = BlocksDominancePattern(client, 'ckpool')
        self.nicehash: BlocksDominancePattern = BlocksDominancePattern(client, 'nicehash')
        self.bitclub: BlocksDominancePattern = BlocksDominancePattern(client, 'bitclub')
        self.bitcoinaffiliatenetwork: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoinaffiliatenetwork')
        self.exxbw: BlocksDominancePattern = BlocksDominancePattern(client, 'exxbw')
        self.bitsolo: BlocksDominancePattern = BlocksDominancePattern(client, 'bitsolo')
        self.twentyoneinc: BlocksDominancePattern = BlocksDominancePattern(client, 'twentyoneinc')
        self.digitalbtc: BlocksDominancePattern = BlocksDominancePattern(client, 'digitalbtc')
        self.eightbaochi: BlocksDominancePattern = BlocksDominancePattern(client, 'eightbaochi')
        self.mybtccoinpool: BlocksDominancePattern = BlocksDominancePattern(client, 'mybtccoinpool')
        self.tbdice: BlocksDominancePattern = BlocksDominancePattern(client, 'tbdice')
        self.hashpool: BlocksDominancePattern = BlocksDominancePattern(client, 'hashpool')
        self.nexious: BlocksDominancePattern = BlocksDominancePattern(client, 'nexious')
        self.bravomining: BlocksDominancePattern = BlocksDominancePattern(client, 'bravomining')
        self.hotpool: BlocksDominancePattern = BlocksDominancePattern(client, 'hotpool')
        self.okexpool: BlocksDominancePattern = BlocksDominancePattern(client, 'okexpool')
        self.bcmonster: BlocksDominancePattern = BlocksDominancePattern(client, 'bcmonster')
        self.onehash: BlocksDominancePattern = BlocksDominancePattern(client, 'onehash')
        self.bixin: BlocksDominancePattern = BlocksDominancePattern(client, 'bixin')
        self.tatmaspool: BlocksDominancePattern = BlocksDominancePattern(client, 'tatmaspool')
        self.connectbtc: BlocksDominancePattern = BlocksDominancePattern(client, 'connectbtc')
        self.batpool: BlocksDominancePattern = BlocksDominancePattern(client, 'batpool')
        self.waterhole: BlocksDominancePattern = BlocksDominancePattern(client, 'waterhole')
        self.dcexploration: BlocksDominancePattern = BlocksDominancePattern(client, 'dcexploration')
        self.dcex: BlocksDominancePattern = BlocksDominancePattern(client, 'dcex')
        self.btpool: BlocksDominancePattern = BlocksDominancePattern(client, 'btpool')
        self.fiftyeightcoin: BlocksDominancePattern = BlocksDominancePattern(client, 'fiftyeightcoin')
        self.bitcoinindia: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoinindia')
        self.shawnp0wers: BlocksDominancePattern = BlocksDominancePattern(client, 'shawnp0wers')
        self.phashio: BlocksDominancePattern = BlocksDominancePattern(client, 'phashio')
        self.rigpool: BlocksDominancePattern = BlocksDominancePattern(client, 'rigpool')
        self.haozhuzhu: BlocksDominancePattern = BlocksDominancePattern(client, 'haozhuzhu')
        self.sevenpool: BlocksDominancePattern = BlocksDominancePattern(client, 'sevenpool')
        self.miningkings: BlocksDominancePattern = BlocksDominancePattern(client, 'miningkings')
        self.hashbx: BlocksDominancePattern = BlocksDominancePattern(client, 'hashbx')
        self.dpool: BlocksDominancePattern = BlocksDominancePattern(client, 'dpool')
        self.rawpool: BlocksDominancePattern = BlocksDominancePattern(client, 'rawpool')
        self.haominer: BlocksDominancePattern = BlocksDominancePattern(client, 'haominer')
        self.helix: BlocksDominancePattern = BlocksDominancePattern(client, 'helix')
        self.bitcoinukraine: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoinukraine')
        self.secretsuperstar: BlocksDominancePattern = BlocksDominancePattern(client, 'secretsuperstar')
        self.tigerpoolnet: BlocksDominancePattern = BlocksDominancePattern(client, 'tigerpoolnet')
        self.sigmapoolcom: BlocksDominancePattern = BlocksDominancePattern(client, 'sigmapoolcom')
        self.okpooltop: BlocksDominancePattern = BlocksDominancePattern(client, 'okpooltop')
        self.hummerpool: BlocksDominancePattern = BlocksDominancePattern(client, 'hummerpool')
        self.tangpool: BlocksDominancePattern = BlocksDominancePattern(client, 'tangpool')
        self.bytepool: BlocksDominancePattern = BlocksDominancePattern(client, 'bytepool')
        self.novablock: BlocksDominancePattern = BlocksDominancePattern(client, 'novablock')
        self.miningcity: BlocksDominancePattern = BlocksDominancePattern(client, 'miningcity')
        self.minerium: BlocksDominancePattern = BlocksDominancePattern(client, 'minerium')
        self.lubiancom: BlocksDominancePattern = BlocksDominancePattern(client, 'lubiancom')
        self.okkong: BlocksDominancePattern = BlocksDominancePattern(client, 'okkong')
        self.aaopool: BlocksDominancePattern = BlocksDominancePattern(client, 'aaopool')
        self.emcdpool: BlocksDominancePattern = BlocksDominancePattern(client, 'emcdpool')
        self.arkpool: BlocksDominancePattern = BlocksDominancePattern(client, 'arkpool')
        self.purebtccom: BlocksDominancePattern = BlocksDominancePattern(client, 'purebtccom')
        self.kucoinpool: BlocksDominancePattern = BlocksDominancePattern(client, 'kucoinpool')
        self.entrustcharitypool: BlocksDominancePattern = BlocksDominancePattern(client, 'entrustcharitypool')
        self.okminer: BlocksDominancePattern = BlocksDominancePattern(client, 'okminer')
        self.titan: BlocksDominancePattern = BlocksDominancePattern(client, 'titan')
        self.pegapool: BlocksDominancePattern = BlocksDominancePattern(client, 'pegapool')
        self.btcnuggets: BlocksDominancePattern = BlocksDominancePattern(client, 'btcnuggets')
        self.cloudhashing: BlocksDominancePattern = BlocksDominancePattern(client, 'cloudhashing')
        self.digitalxmintsy: BlocksDominancePattern = BlocksDominancePattern(client, 'digitalxmintsy')
        self.telco214: BlocksDominancePattern = BlocksDominancePattern(client, 'telco214')
        self.btcpoolparty: BlocksDominancePattern = BlocksDominancePattern(client, 'btcpoolparty')
        self.multipool: BlocksDominancePattern = BlocksDominancePattern(client, 'multipool')
        self.transactioncoinmining: BlocksDominancePattern = BlocksDominancePattern(client, 'transactioncoinmining')
        self.btcdig: BlocksDominancePattern = BlocksDominancePattern(client, 'btcdig')
        self.trickysbtcpool: BlocksDominancePattern = BlocksDominancePattern(client, 'trickysbtcpool')
        self.btcmp: BlocksDominancePattern = BlocksDominancePattern(client, 'btcmp')
        self.eobot: BlocksDominancePattern = BlocksDominancePattern(client, 'eobot')
        self.unomp: BlocksDominancePattern = BlocksDominancePattern(client, 'unomp')
        self.patels: BlocksDominancePattern = BlocksDominancePattern(client, 'patels')
        self.gogreenlight: BlocksDominancePattern = BlocksDominancePattern(client, 'gogreenlight')
        self.bitcoinindiapool: BlocksDominancePattern = BlocksDominancePattern(client, 'bitcoinindiapool')
        self.ekanembtc: BlocksDominancePattern = BlocksDominancePattern(client, 'ekanembtc')
        self.canoe: BlocksDominancePattern = BlocksDominancePattern(client, 'canoe')
        self.tiger: BlocksDominancePattern = BlocksDominancePattern(client, 'tiger')
        self.onem1x: BlocksDominancePattern = BlocksDominancePattern(client, 'onem1x')
        self.zulupool: BlocksDominancePattern = BlocksDominancePattern(client, 'zulupool')
        self.wiz: BlocksDominancePattern = BlocksDominancePattern(client, 'wiz')
        self.wk057: BlocksDominancePattern = BlocksDominancePattern(client, 'wk057')
        self.futurebitapollosolo: BlocksDominancePattern = BlocksDominancePattern(client, 'futurebitapollosolo')
        self.carbonnegative: BlocksDominancePattern = BlocksDominancePattern(client, 'carbonnegative')
        self.portlandhodl: BlocksDominancePattern = BlocksDominancePattern(client, 'portlandhodl')
        self.phoenix: BlocksDominancePattern = BlocksDominancePattern(client, 'phoenix')
        self.neopool: BlocksDominancePattern = BlocksDominancePattern(client, 'neopool')
        self.maxipool: BlocksDominancePattern = BlocksDominancePattern(client, 'maxipool')
        self.bitfufupool: BlocksDominancePattern = BlocksDominancePattern(client, 'bitfufupool')
        self.gdpool: BlocksDominancePattern = BlocksDominancePattern(client, 'gdpool')
        self.miningdutch: BlocksDominancePattern = BlocksDominancePattern(client, 'miningdutch')
        self.publicpool: BlocksDominancePattern = BlocksDominancePattern(client, 'publicpool')
        self.miningsquared: BlocksDominancePattern = BlocksDominancePattern(client, 'miningsquared')
        self.innopolistech: BlocksDominancePattern = BlocksDominancePattern(client, 'innopolistech')
        self.btclab: BlocksDominancePattern = BlocksDominancePattern(client, 'btclab')
        self.parasite: BlocksDominancePattern = BlocksDominancePattern(client, 'parasite')
        self.redrockpool: BlocksDominancePattern = BlocksDominancePattern(client, 'redrockpool')
        self.est3lar: BlocksDominancePattern = BlocksDominancePattern(client, 'est3lar')

class MetricsTree_Pools:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.height_to_pool: MetricPattern18[PoolSlug] = MetricPattern18(client, 'pool')
        self.major: MetricsTree_Pools_Major = MetricsTree_Pools_Major(client)
        self.minor: MetricsTree_Pools_Minor = MetricsTree_Pools_Minor(client)

class MetricsTree_Prices_Split:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.open: CentsSatsUsdPattern3 = CentsSatsUsdPattern3(client, 'price_open')
        self.high: CentsSatsUsdPattern3 = CentsSatsUsdPattern3(client, 'price_high')
        self.low: CentsSatsUsdPattern3 = CentsSatsUsdPattern3(client, 'price_low')
        self.close: CentsSatsUsdPattern3 = CentsSatsUsdPattern3(client, 'price_close')

class MetricsTree_Prices_Ohlc:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.usd: MetricPattern2[OHLCDollars] = MetricPattern2(client, 'price_ohlc')
        self.cents: MetricPattern2[OHLCCents] = MetricPattern2(client, 'price_ohlc_cents')
        self.sats: MetricPattern2[OHLCSats] = MetricPattern2(client, 'price_ohlc_sats')

class MetricsTree_Prices_Spot:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, 'price')
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, 'price_cents')
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, 'price_sats')

class MetricsTree_Prices:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.split: MetricsTree_Prices_Split = MetricsTree_Prices_Split(client)
        self.ohlc: MetricsTree_Prices_Ohlc = MetricsTree_Prices_Ohlc(client)
        self.spot: MetricsTree_Prices_Spot = MetricsTree_Prices_Spot(client)

class MetricsTree_Supply_Burned:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.unspendable: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'unspendable_supply')

class MetricsTree_Supply_Velocity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.native: MetricPattern1[StoredF64] = MetricPattern1(client, 'velocity')
        self.fiat: MetricPattern1[StoredF64] = MetricPattern1(client, 'velocity_fiat')

class MetricsTree_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.state: MetricPattern18[SupplyState] = MetricPattern18(client, 'supply_state')
        self.circulating: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'circulating_supply')
        self.burned: MetricsTree_Supply_Burned = MetricsTree_Supply_Burned(client)
        self.inflation_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'inflation_rate')
        self.velocity: MetricsTree_Supply_Velocity = MetricsTree_Supply_Velocity(client)
        self.market_cap: CentsDeltaUsdPattern = CentsDeltaUsdPattern(client, 'market_cap')
        self.market_minus_realized_cap_growth_rate: _1m1w1y24hPattern[BasisPointsSigned32] = _1m1w1y24hPattern(client, 'market_minus_realized_cap_growth_rate')
        self.hodled_or_lost: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'hodled_or_lost_coins')

class MetricsTree_Cohorts_Utxo_All_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'supply')
        self.half: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'supply_half')
        self.delta: AbsoluteRatePattern = AbsoluteRatePattern(client, 'supply_delta')
        self.in_profit: BtcCentsRelSatsUsdPattern2 = BtcCentsRelSatsUsdPattern2(client, 'supply_in_profit')
        self.in_loss: BtcCentsRelSatsUsdPattern2 = BtcCentsRelSatsUsdPattern2(client, 'supply_in_loss')

class MetricsTree_Cohorts_Utxo_All_Realized:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cap: CentsDeltaRelUsdPattern = CentsDeltaRelUsdPattern(client, 'realized_cap')
        self.profit: BaseCumulativeDistributionRelSumValuePattern = BaseCumulativeDistributionRelSumValuePattern(client, '')
        self.loss: BaseCapitulationCumulativeNegativeRelSumValuePattern = BaseCapitulationCumulativeNegativeRelSumValuePattern(client, '')
        self.price: BpsCentsPercentilesRatioSatsSmaStdUsdPattern = BpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, 'realized_price')
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, 'mvrv')
        self.sopr: AdjustedRatioValuePattern = AdjustedRatioValuePattern(client, '')
        self.net_pnl: BaseChangeCumulativeDeltaRelSumPattern = BaseChangeCumulativeDeltaRelSumPattern(client, 'net')
        self.gross_pnl: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, 'realized_gross_pnl')
        self.sell_side_risk_ratio: _1m1w1y24hPattern6 = _1m1w1y24hPattern6(client, 'sell_side_risk_ratio')
        self.peak_regret: BaseCumulativeRelPattern = BaseCumulativeRelPattern(client, 'realized_peak_regret')
        self.investor: LowerPriceUpperPattern = LowerPriceUpperPattern(client, '')
        self.profit_to_loss_ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'realized_profit_to_loss_ratio')

class MetricsTree_Cohorts_Utxo_All_Unrealized_Profit:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, 'unrealized_profit')
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, 'unrealized_profit_cumulative')
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, 'unrealized_profit_sum')
        self.rel_to_mcap: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'unrealized_profit_rel_to_mcap')
        self.rel_to_own_gross: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'unrealized_profit_rel_to_own_gross_pnl')

class MetricsTree_Cohorts_Utxo_All_Unrealized_Loss:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: CentsUsdPattern2 = CentsUsdPattern2(client, 'unrealized_loss')
        self.cumulative: CentsUsdPattern2 = CentsUsdPattern2(client, 'unrealized_loss_cumulative')
        self.sum: _1m1w1y24hPattern4 = _1m1w1y24hPattern4(client, 'unrealized_loss_sum')
        self.negative: MetricPattern1[Dollars] = MetricPattern1(client, 'neg_unrealized_loss')
        self.rel_to_mcap: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'unrealized_loss_rel_to_mcap')
        self.rel_to_own_gross: BpsPercentRatioPattern3 = BpsPercentRatioPattern3(client, 'unrealized_loss_rel_to_own_gross_pnl')

class MetricsTree_Cohorts_Utxo_All_Unrealized_NetPnl:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, 'net_unrealized_pnl')
        self.cents: MetricPattern1[CentsSigned] = MetricPattern1(client, 'net_unrealized_pnl_cents')
        self.rel_to_own_gross: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'net_unrealized_pnl_rel_to_own_gross_pnl')

class MetricsTree_Cohorts_Utxo_All_Unrealized:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.nupl: BpsRatioPattern = BpsRatioPattern(client, 'nupl')
        self.profit: MetricsTree_Cohorts_Utxo_All_Unrealized_Profit = MetricsTree_Cohorts_Utxo_All_Unrealized_Profit(client)
        self.loss: MetricsTree_Cohorts_Utxo_All_Unrealized_Loss = MetricsTree_Cohorts_Utxo_All_Unrealized_Loss(client)
        self.net_pnl: MetricsTree_Cohorts_Utxo_All_Unrealized_NetPnl = MetricsTree_Cohorts_Utxo_All_Unrealized_NetPnl(client)
        self.gross_pnl: CentsUsdPattern2 = CentsUsdPattern2(client, 'unrealized_gross_pnl')
        self.invested_capital: InPattern = InPattern(client, 'invested_capital_in')
        self.sentiment: GreedNetPainPattern = GreedNetPainPattern(client, '')

class MetricsTree_Cohorts_Utxo_All:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: MetricsTree_Cohorts_Utxo_All_Supply = MetricsTree_Cohorts_Utxo_All_Supply(client)
        self.outputs: UnspentPattern = UnspentPattern(client, 'utxo_count')
        self.activity: CoindaysCoinyearsDormancySentPattern = CoindaysCoinyearsDormancySentPattern(client, '')
        self.realized: MetricsTree_Cohorts_Utxo_All_Realized = MetricsTree_Cohorts_Utxo_All_Realized(client)
        self.cost_basis: InvestedMaxMinPercentilesSupplyPattern = InvestedMaxMinPercentilesSupplyPattern(client, '')
        self.unrealized: MetricsTree_Cohorts_Utxo_All_Unrealized = MetricsTree_Cohorts_Utxo_All_Unrealized(client)

class MetricsTree_Cohorts_Utxo_Sth_Realized:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cap: CentsDeltaRelUsdPattern = CentsDeltaRelUsdPattern(client, 'sth_realized_cap')
        self.profit: BaseCumulativeDistributionRelSumValuePattern = BaseCumulativeDistributionRelSumValuePattern(client, 'sth')
        self.loss: BaseCapitulationCumulativeNegativeRelSumValuePattern = BaseCapitulationCumulativeNegativeRelSumValuePattern(client, 'sth')
        self.price: BpsCentsPercentilesRatioSatsSmaStdUsdPattern = BpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, 'sth_realized_price')
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, 'sth_mvrv')
        self.sopr: AdjustedRatioValuePattern = AdjustedRatioValuePattern(client, 'sth')
        self.net_pnl: BaseChangeCumulativeDeltaRelSumPattern = BaseChangeCumulativeDeltaRelSumPattern(client, 'sth_net')
        self.gross_pnl: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, 'sth_realized_gross_pnl')
        self.sell_side_risk_ratio: _1m1w1y24hPattern6 = _1m1w1y24hPattern6(client, 'sth_sell_side_risk_ratio')
        self.peak_regret: BaseCumulativeRelPattern = BaseCumulativeRelPattern(client, 'sth_realized_peak_regret')
        self.investor: LowerPriceUpperPattern = LowerPriceUpperPattern(client, 'sth')
        self.profit_to_loss_ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'sth_realized_profit_to_loss_ratio')

class MetricsTree_Cohorts_Utxo_Sth:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: DeltaHalfInRelTotalPattern2 = DeltaHalfInRelTotalPattern2(client, 'sth_supply')
        self.outputs: UnspentPattern = UnspentPattern(client, 'sth_utxo_count')
        self.activity: CoindaysCoinyearsDormancySentPattern = CoindaysCoinyearsDormancySentPattern(client, 'sth')
        self.realized: MetricsTree_Cohorts_Utxo_Sth_Realized = MetricsTree_Cohorts_Utxo_Sth_Realized(client)
        self.cost_basis: InvestedMaxMinPercentilesSupplyPattern = InvestedMaxMinPercentilesSupplyPattern(client, 'sth')
        self.unrealized: GrossInvestedLossNetNuplProfitSentimentPattern2 = GrossInvestedLossNetNuplProfitSentimentPattern2(client, 'sth')

class MetricsTree_Cohorts_Utxo_Lth_Realized_Sopr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.value_created: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, 'lth_value_created')
        self.value_destroyed: BaseCumulativeSumPattern[Cents] = BaseCumulativeSumPattern(client, 'lth_value_destroyed')
        self.ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'lth_sopr')

class MetricsTree_Cohorts_Utxo_Lth_Realized:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cap: CentsDeltaRelUsdPattern = CentsDeltaRelUsdPattern(client, 'lth_realized_cap')
        self.profit: BaseCumulativeDistributionRelSumValuePattern = BaseCumulativeDistributionRelSumValuePattern(client, 'lth')
        self.loss: BaseCapitulationCumulativeNegativeRelSumValuePattern = BaseCapitulationCumulativeNegativeRelSumValuePattern(client, 'lth')
        self.price: BpsCentsPercentilesRatioSatsSmaStdUsdPattern = BpsCentsPercentilesRatioSatsSmaStdUsdPattern(client, 'lth_realized_price')
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, 'lth_mvrv')
        self.sopr: MetricsTree_Cohorts_Utxo_Lth_Realized_Sopr = MetricsTree_Cohorts_Utxo_Lth_Realized_Sopr(client)
        self.net_pnl: BaseChangeCumulativeDeltaRelSumPattern = BaseChangeCumulativeDeltaRelSumPattern(client, 'lth_net')
        self.gross_pnl: BaseCumulativeSumPattern3 = BaseCumulativeSumPattern3(client, 'lth_realized_gross_pnl')
        self.sell_side_risk_ratio: _1m1w1y24hPattern6 = _1m1w1y24hPattern6(client, 'lth_sell_side_risk_ratio')
        self.peak_regret: BaseCumulativeRelPattern = BaseCumulativeRelPattern(client, 'lth_realized_peak_regret')
        self.investor: LowerPriceUpperPattern = LowerPriceUpperPattern(client, 'lth')
        self.profit_to_loss_ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'lth_realized_profit_to_loss_ratio')

class MetricsTree_Cohorts_Utxo_Lth:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: DeltaHalfInRelTotalPattern2 = DeltaHalfInRelTotalPattern2(client, 'lth_supply')
        self.outputs: UnspentPattern = UnspentPattern(client, 'lth_utxo_count')
        self.activity: CoindaysCoinyearsDormancySentPattern = CoindaysCoinyearsDormancySentPattern(client, 'lth')
        self.realized: MetricsTree_Cohorts_Utxo_Lth_Realized = MetricsTree_Cohorts_Utxo_Lth_Realized(client)
        self.cost_basis: InvestedMaxMinPercentilesSupplyPattern = InvestedMaxMinPercentilesSupplyPattern(client, 'lth')
        self.unrealized: GrossInvestedLossNetNuplProfitSentimentPattern2 = GrossInvestedLossNetNuplProfitSentimentPattern2(client, 'lth')

class MetricsTree_Cohorts_Utxo_AgeRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.under_1h: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1h_old')
        self._1h_to_1d: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1h_to_1d_old')
        self._1d_to_1w: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1d_to_1w_old')
        self._1w_to_1m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1w_to_1m_old')
        self._1m_to_2m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1m_to_2m_old')
        self._2m_to_3m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_2m_to_3m_old')
        self._3m_to_4m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_3m_to_4m_old')
        self._4m_to_5m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_4m_to_5m_old')
        self._5m_to_6m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_5m_to_6m_old')
        self._6m_to_1y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_6m_to_1y_old')
        self._1y_to_2y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1y_to_2y_old')
        self._2y_to_3y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_2y_to_3y_old')
        self._3y_to_4y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_3y_to_4y_old')
        self._4y_to_5y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_4y_to_5y_old')
        self._5y_to_6y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_5y_to_6y_old')
        self._6y_to_7y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_6y_to_7y_old')
        self._7y_to_8y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_7y_to_8y_old')
        self._8y_to_10y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_8y_to_10y_old')
        self._10y_to_12y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10y_to_12y_old')
        self._12y_to_15y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_12y_to_15y_old')
        self.over_15y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_15y_old')

class MetricsTree_Cohorts_Utxo_UnderAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1w_old')
        self._1m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1m_old')
        self._2m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_2m_old')
        self._3m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_3m_old')
        self._4m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_4m_old')
        self._5m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_5m_old')
        self._6m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_6m_old')
        self._1y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1y_old')
        self._2y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_2y_old')
        self._3y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_3y_old')
        self._4y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_4y_old')
        self._5y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_5y_old')
        self._6y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_6y_old')
        self._7y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_7y_old')
        self._8y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_8y_old')
        self._10y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10y_old')
        self._12y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_12y_old')
        self._15y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_15y_old')

class MetricsTree_Cohorts_Utxo_OverAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1d_old')
        self._1w: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1w_old')
        self._1m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1m_old')
        self._2m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_2m_old')
        self._3m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_3m_old')
        self._4m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_4m_old')
        self._5m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_5m_old')
        self._6m: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_6m_old')
        self._1y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1y_old')
        self._2y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_2y_old')
        self._3y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_3y_old')
        self._4y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_4y_old')
        self._5y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_5y_old')
        self._6y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_6y_old')
        self._7y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_7y_old')
        self._8y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_8y_old')
        self._10y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10y_old')
        self._12y: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_12y_old')

class MetricsTree_Cohorts_Utxo_Epoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'epoch_0')
        self._1: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'epoch_1')
        self._2: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'epoch_2')
        self._3: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'epoch_3')
        self._4: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'epoch_4')

class MetricsTree_Cohorts_Utxo_Class:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2009: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2009')
        self._2010: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2010')
        self._2011: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2011')
        self._2012: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2012')
        self._2013: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2013')
        self._2014: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2014')
        self._2015: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2015')
        self._2016: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2016')
        self._2017: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2017')
        self._2018: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2018')
        self._2019: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2019')
        self._2020: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2020')
        self._2021: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2021')
        self._2022: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2022')
        self._2023: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2023')
        self._2024: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2024')
        self._2025: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2025')
        self._2026: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'class_2026')

class MetricsTree_Cohorts_Utxo_OverAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1sat')
        self._10sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10sats')
        self._100sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100sats')
        self._1k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1k_sats')
        self._10k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10k_sats')
        self._100k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100k_sats')
        self._1m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1m_sats')
        self._10m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10m_sats')
        self._1btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1btc')
        self._10btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10btc')
        self._100btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100btc')
        self._1k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1k_btc')
        self._10k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10k_btc')

class MetricsTree_Cohorts_Utxo_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_0sats')
        self._1sat_to_10sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1sat_to_10sats')
        self._10sats_to_100sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10sats_to_100sats')
        self._100sats_to_1k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_100sats_to_1k_sats')
        self._1k_sats_to_10k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1k_sats_to_10k_sats')
        self._10k_sats_to_100k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10k_sats_to_100k_sats')
        self._100k_sats_to_1m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_100k_sats_to_1m_sats')
        self._1m_sats_to_10m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1m_sats_to_10m_sats')
        self._10m_sats_to_1btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10m_sats_to_1btc')
        self._1btc_to_10btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1btc_to_10btc')
        self._10btc_to_100btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10btc_to_100btc')
        self._100btc_to_1k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_100btc_to_1k_btc')
        self._1k_btc_to_10k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_1k_btc_to_10k_btc')
        self._10k_btc_to_100k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_10k_btc_to_100k_btc')
        self.over_100k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100k_btc')

class MetricsTree_Cohorts_Utxo_UnderAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10sats')
        self._100sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100sats')
        self._1k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1k_sats')
        self._10k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10k_sats')
        self._100k_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100k_sats')
        self._1m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1m_sats')
        self._10m_sats: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10m_sats')
        self._1btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1btc')
        self._10btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10btc')
        self._100btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100btc')
        self._1k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1k_btc')
        self._10k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10k_btc')
        self._100k_btc: OutputsRealizedSupplyUnrealizedPattern = OutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100k_btc')

class MetricsTree_Cohorts_Utxo_Type:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk65: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2pk65')
        self.p2pk33: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2pk33')
        self.p2pkh: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2pkh')
        self.p2ms: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2ms')
        self.p2sh: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2sh')
        self.p2wpkh: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2wpkh')
        self.p2wsh: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2wsh')
        self.p2tr: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2tr')
        self.p2a: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'p2a')
        self.unknown: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'unknown_outputs')
        self.empty: OutputsRealizedSupplyUnrealizedPattern2 = OutputsRealizedSupplyUnrealizedPattern2(client, 'empty_outputs')

class MetricsTree_Cohorts_Utxo_Profitability_Range:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.over_1000pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_1000pct_in_profit')
        self._500pct_to_1000pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_500pct_to_1000pct_in_profit')
        self._300pct_to_500pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_300pct_to_500pct_in_profit')
        self._200pct_to_300pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_200pct_to_300pct_in_profit')
        self._100pct_to_200pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_100pct_to_200pct_in_profit')
        self._90pct_to_100pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_90pct_to_100pct_in_profit')
        self._80pct_to_90pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_80pct_to_90pct_in_profit')
        self._70pct_to_80pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_70pct_to_80pct_in_profit')
        self._60pct_to_70pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_60pct_to_70pct_in_profit')
        self._50pct_to_60pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_50pct_to_60pct_in_profit')
        self._40pct_to_50pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_40pct_to_50pct_in_profit')
        self._30pct_to_40pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_30pct_to_40pct_in_profit')
        self._20pct_to_30pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_20pct_to_30pct_in_profit')
        self._10pct_to_20pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_10pct_to_20pct_in_profit')
        self._0pct_to_10pct_in_profit: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_0pct_to_10pct_in_profit')
        self._0pct_to_10pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_0pct_to_10pct_in_loss')
        self._10pct_to_20pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_10pct_to_20pct_in_loss')
        self._20pct_to_30pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_20pct_to_30pct_in_loss')
        self._30pct_to_40pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_30pct_to_40pct_in_loss')
        self._40pct_to_50pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_40pct_to_50pct_in_loss')
        self._50pct_to_60pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_50pct_to_60pct_in_loss')
        self._60pct_to_70pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_60pct_to_70pct_in_loss')
        self._70pct_to_80pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_70pct_to_80pct_in_loss')
        self._80pct_to_90pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_80pct_to_90pct_in_loss')
        self._90pct_to_100pct_in_loss: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_90pct_to_100pct_in_loss')

class MetricsTree_Cohorts_Utxo_Profitability_Profit:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.breakeven: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_in_profit')
        self._10pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_10pct_in_profit')
        self._20pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_20pct_in_profit')
        self._30pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_30pct_in_profit')
        self._40pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_40pct_in_profit')
        self._50pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_50pct_in_profit')
        self._60pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_60pct_in_profit')
        self._70pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_70pct_in_profit')
        self._80pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_80pct_in_profit')
        self._90pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_90pct_in_profit')
        self._100pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_100pct_in_profit')
        self._200pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_200pct_in_profit')
        self._300pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_300pct_in_profit')
        self._500pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_500pct_in_profit')

class MetricsTree_Cohorts_Utxo_Profitability_Loss:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.breakeven: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_in_loss')
        self._10pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_10pct_in_loss')
        self._20pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_20pct_in_loss')
        self._30pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_30pct_in_loss')
        self._40pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_40pct_in_loss')
        self._50pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_50pct_in_loss')
        self._60pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_60pct_in_loss')
        self._70pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_70pct_in_loss')
        self._80pct: MvrvNuplRealizedSupplyPattern = MvrvNuplRealizedSupplyPattern(client, 'utxos_over_80pct_in_loss')

class MetricsTree_Cohorts_Utxo_Profitability:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.range: MetricsTree_Cohorts_Utxo_Profitability_Range = MetricsTree_Cohorts_Utxo_Profitability_Range(client)
        self.profit: MetricsTree_Cohorts_Utxo_Profitability_Profit = MetricsTree_Cohorts_Utxo_Profitability_Profit(client)
        self.loss: MetricsTree_Cohorts_Utxo_Profitability_Loss = MetricsTree_Cohorts_Utxo_Profitability_Loss(client)

class MetricsTree_Cohorts_Utxo_Matured:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.under_1h: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_under_1h_old_matured_supply')
        self._1h_to_1d: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_1h_to_1d_old_matured_supply')
        self._1d_to_1w: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_1d_to_1w_old_matured_supply')
        self._1w_to_1m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_1w_to_1m_old_matured_supply')
        self._1m_to_2m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_1m_to_2m_old_matured_supply')
        self._2m_to_3m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_2m_to_3m_old_matured_supply')
        self._3m_to_4m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_3m_to_4m_old_matured_supply')
        self._4m_to_5m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_4m_to_5m_old_matured_supply')
        self._5m_to_6m: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_5m_to_6m_old_matured_supply')
        self._6m_to_1y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_6m_to_1y_old_matured_supply')
        self._1y_to_2y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_1y_to_2y_old_matured_supply')
        self._2y_to_3y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_2y_to_3y_old_matured_supply')
        self._3y_to_4y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_3y_to_4y_old_matured_supply')
        self._4y_to_5y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_4y_to_5y_old_matured_supply')
        self._5y_to_6y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_5y_to_6y_old_matured_supply')
        self._6y_to_7y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_6y_to_7y_old_matured_supply')
        self._7y_to_8y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_7y_to_8y_old_matured_supply')
        self._8y_to_10y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_8y_to_10y_old_matured_supply')
        self._10y_to_12y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_10y_to_12y_old_matured_supply')
        self._12y_to_15y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_12y_to_15y_old_matured_supply')
        self.over_15y: BaseCumulativeSumPattern4 = BaseCumulativeSumPattern4(client, 'utxos_over_15y_old_matured_supply')

class MetricsTree_Cohorts_Utxo:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricsTree_Cohorts_Utxo_All = MetricsTree_Cohorts_Utxo_All(client)
        self.sth: MetricsTree_Cohorts_Utxo_Sth = MetricsTree_Cohorts_Utxo_Sth(client)
        self.lth: MetricsTree_Cohorts_Utxo_Lth = MetricsTree_Cohorts_Utxo_Lth(client)
        self.age_range: MetricsTree_Cohorts_Utxo_AgeRange = MetricsTree_Cohorts_Utxo_AgeRange(client)
        self.under_age: MetricsTree_Cohorts_Utxo_UnderAge = MetricsTree_Cohorts_Utxo_UnderAge(client)
        self.over_age: MetricsTree_Cohorts_Utxo_OverAge = MetricsTree_Cohorts_Utxo_OverAge(client)
        self.epoch: MetricsTree_Cohorts_Utxo_Epoch = MetricsTree_Cohorts_Utxo_Epoch(client)
        self.class_: MetricsTree_Cohorts_Utxo_Class = MetricsTree_Cohorts_Utxo_Class(client)
        self.over_amount: MetricsTree_Cohorts_Utxo_OverAmount = MetricsTree_Cohorts_Utxo_OverAmount(client)
        self.amount_range: MetricsTree_Cohorts_Utxo_AmountRange = MetricsTree_Cohorts_Utxo_AmountRange(client)
        self.under_amount: MetricsTree_Cohorts_Utxo_UnderAmount = MetricsTree_Cohorts_Utxo_UnderAmount(client)
        self.r#type: MetricsTree_Cohorts_Utxo_Type = MetricsTree_Cohorts_Utxo_Type(client)
        self.profitability: MetricsTree_Cohorts_Utxo_Profitability = MetricsTree_Cohorts_Utxo_Profitability(client)
        self.matured: MetricsTree_Cohorts_Utxo_Matured = MetricsTree_Cohorts_Utxo_Matured(client)

class MetricsTree_Cohorts_Address_OverAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1sat')
        self._10sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10sats')
        self._100sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100sats')
        self._1k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1k_sats')
        self._10k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10k_sats')
        self._100k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100k_sats')
        self._1m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1m_sats')
        self._10m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10m_sats')
        self._1btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1btc')
        self._10btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10btc')
        self._100btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100btc')
        self._1k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1k_btc')
        self._10k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10k_btc')

class MetricsTree_Cohorts_Address_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_0sats')
        self._1sat_to_10sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_1sat_to_10sats')
        self._10sats_to_100sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_10sats_to_100sats')
        self._100sats_to_1k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_100sats_to_1k_sats')
        self._1k_sats_to_10k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_1k_sats_to_10k_sats')
        self._10k_sats_to_100k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_10k_sats_to_100k_sats')
        self._100k_sats_to_1m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_100k_sats_to_1m_sats')
        self._1m_sats_to_10m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_1m_sats_to_10m_sats')
        self._10m_sats_to_1btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_10m_sats_to_1btc')
        self._1btc_to_10btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_1btc_to_10btc')
        self._10btc_to_100btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_10btc_to_100btc')
        self._100btc_to_1k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_100btc_to_1k_btc')
        self._1k_btc_to_10k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_1k_btc_to_10k_btc')
        self._10k_btc_to_100k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_10k_btc_to_100k_btc')
        self.over_100k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100k_btc')

class MetricsTree_Cohorts_Address_UnderAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10sats')
        self._100sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100sats')
        self._1k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1k_sats')
        self._10k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10k_sats')
        self._100k_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100k_sats')
        self._1m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1m_sats')
        self._10m_sats: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10m_sats')
        self._1btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1btc')
        self._10btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10btc')
        self._100btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100btc')
        self._1k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1k_btc')
        self._10k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10k_btc')
        self._100k_btc: AddressOutputsRealizedSupplyUnrealizedPattern = AddressOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100k_btc')

class MetricsTree_Cohorts_Address:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.over_amount: MetricsTree_Cohorts_Address_OverAmount = MetricsTree_Cohorts_Address_OverAmount(client)
        self.amount_range: MetricsTree_Cohorts_Address_AmountRange = MetricsTree_Cohorts_Address_AmountRange(client)
        self.under_amount: MetricsTree_Cohorts_Address_UnderAmount = MetricsTree_Cohorts_Address_UnderAmount(client)

class MetricsTree_Cohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.utxo: MetricsTree_Cohorts_Utxo = MetricsTree_Cohorts_Utxo(client)
        self.address: MetricsTree_Cohorts_Address = MetricsTree_Cohorts_Address(client)

class MetricsTree:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks: MetricsTree_Blocks = MetricsTree_Blocks(client)
        self.transactions: MetricsTree_Transactions = MetricsTree_Transactions(client)
        self.inputs: MetricsTree_Inputs = MetricsTree_Inputs(client)
        self.outputs: MetricsTree_Outputs = MetricsTree_Outputs(client)
        self.addresses: MetricsTree_Addresses = MetricsTree_Addresses(client)
        self.scripts: MetricsTree_Scripts = MetricsTree_Scripts(client)
        self.mining: MetricsTree_Mining = MetricsTree_Mining(client)
        self.cointime: MetricsTree_Cointime = MetricsTree_Cointime(client)
        self.constants: MetricsTree_Constants = MetricsTree_Constants(client)
        self.indexes: MetricsTree_Indexes = MetricsTree_Indexes(client)
        self.indicators: MetricsTree_Indicators = MetricsTree_Indicators(client)
        self.market: MetricsTree_Market = MetricsTree_Market(client)
        self.pools: MetricsTree_Pools = MetricsTree_Pools(client)
        self.prices: MetricsTree_Prices = MetricsTree_Prices(client)
        self.supply: MetricsTree_Supply = MetricsTree_Supply(client)
        self.cohorts: MetricsTree_Cohorts = MetricsTree_Cohorts(client)

class BrkClient(BrkClientBase):
    """Main BRK client with metrics tree and API methods."""

    VERSION = "v0.1.9"

    INDEXES = [
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
      "tx_index",
      "txin_index",
      "txout_index",
      "empty_output_index",
      "op_return_index",
      "p2a_address_index",
      "p2ms_output_index",
      "p2pk33_address_index",
      "p2pk65_address_index",
      "p2pkh_address_index",
      "p2sh_address_index",
      "p2tr_address_index",
      "p2wpkh_address_index",
      "p2wsh_address_index",
      "unknown_output_index",
      "funded_address_index",
      "empty_address_index"
    ]

    POOL_ID_TO_POOL_NAME = {
      "aaopool": "AAO Pool",
      "antpool": "AntPool",
      "arkpool": "ArkPool",
      "asicminer": "ASICMiner",
      "axbt": "A-XBT",
      "batpool": "BATPOOL",
      "bcmonster": "BCMonster",
      "bcpoolio": "bcpool.io",
      "binancepool": "Binance Pool",
      "bitalo": "Bitalo",
      "bitclub": "BitClub",
      "bitcoinaffiliatenetwork": "Bitcoin Affiliate Network",
      "bitcoincom": "Bitcoin.com",
      "bitcoinindia": "Bitcoin India",
      "bitcoinindiapool": "BitcoinIndia",
      "bitcoinrussia": "BitcoinRussia",
      "bitcoinukraine": "Bitcoin-Ukraine",
      "bitfarms": "Bitfarms",
      "bitfufupool": "BitFuFuPool",
      "bitfury": "BitFury",
      "bitminter": "BitMinter",
      "bitparking": "Bitparking",
      "bitsolo": "Bitsolo",
      "bixin": "Bixin",
      "blockfills": "BlockFills",
      "braiinspool": "Braiins Pool",
      "bravomining": "Bravo Mining",
      "btcc": "BTCC",
      "btccom": "BTC.com",
      "btcdig": "BTCDig",
      "btcguild": "BTC Guild",
      "btclab": "BTCLab",
      "btcmp": "BTCMP",
      "btcnuggets": "BTC Nuggets",
      "btcpoolparty": "BTC Pool Party",
      "btcserv": "BTCServ",
      "btctop": "BTC.TOP",
      "btpool": "BTPOOL",
      "bwpool": "BWPool",
      "bytepool": "BytePool",
      "canoe": "CANOE",
      "canoepool": "CanoePool",
      "carbonnegative": "Carbon Negative",
      "ckpool": "CKPool",
      "cloudhashing": "CloudHashing",
      "coinlab": "CoinLab",
      "cointerra": "Cointerra",
      "connectbtc": "ConnectBTC",
      "dcex": "DCEX",
      "dcexploration": "DCExploration",
      "digitalbtc": "digitalBTC",
      "digitalxmintsy": "digitalX Mintsy",
      "dpool": "DPOOL",
      "eclipsemc": "EclipseMC",
      "eightbaochi": "8baochi",
      "ekanembtc": "EkanemBTC",
      "eligius": "Eligius",
      "emcdpool": "EMCDPool",
      "entrustcharitypool": "Entrust Charity Pool",
      "eobot": "Eobot",
      "est3lar": "Est3lar",
      "exxbw": "EXX&BW",
      "f2pool": "F2Pool",
      "fiftyeightcoin": "58COIN",
      "foundryusa": "Foundry USA",
      "futurebitapollosolo": "FutureBit Apollo Solo",
      "gbminers": "GBMiners",
      "gdpool": "GDPool",
      "ghashio": "GHash.IO",
      "givemecoins": "Give Me Coins",
      "gogreenlight": "GoGreenLight",
      "haominer": "haominer",
      "haozhuzhu": "HAOZHUZHU",
      "hashbx": "HashBX",
      "hashpool": "HASHPOOL",
      "helix": "Helix",
      "hhtt": "HHTT",
      "hotpool": "HotPool",
      "hummerpool": "Hummerpool",
      "huobipool": "Huobi.pool",
      "innopolistech": "Innopolis Tech",
      "kanopool": "KanoPool",
      "kncminer": "KnCMiner",
      "kucoinpool": "KuCoinPool",
      "lubiancom": "Lubian.com",
      "luxor": "Luxor",
      "marapool": "MARA Pool",
      "maxbtc": "MaxBTC",
      "maxipool": "MaxiPool",
      "megabigpower": "MegaBigPower",
      "minerium": "Minerium",
      "miningcity": "MiningCity",
      "miningdutch": "Mining-Dutch",
      "miningkings": "MiningKings",
      "miningsquared": "Mining Squared",
      "mmpool": "mmpool",
      "mtred": "Mt Red",
      "multicoinco": "MultiCoin.co",
      "multipool": "Multipool",
      "mybtccoinpool": "myBTCcoin Pool",
      "neopool": "Neopool",
      "nexious": "Nexious",
      "nicehash": "NiceHash",
      "nmcbit": "NMCbit",
      "novablock": "NovaBlock",
      "ocean": "OCEAN",
      "okexpool": "OKExPool",
      "okkong": "OKKONG",
      "okminer": "OKMINER",
      "okpooltop": "okpool.top",
      "onehash": "1Hash",
      "onem1x": "1M1X",
      "onethash": "1THash",
      "ozcoin": "OzCoin",
      "parasite": "Parasite",
      "patels": "Patels",
      "pegapool": "PEGA Pool",
      "phashio": "PHash.IO",
      "phoenix": "Phoenix",
      "polmine": "Polmine",
      "pool175btc": "175btc",
      "pool50btc": "50BTC",
      "poolin": "Poolin",
      "portlandhodl": "Portland.HODL",
      "publicpool": "Public Pool",
      "purebtccom": "PureBTC.COM",
      "rawpool": "Rawpool",
      "redrockpool": "RedRock Pool",
      "rigpool": "RigPool",
      "sbicrypto": "SBI Crypto",
      "secpool": "SECPOOL",
      "secretsuperstar": "SecretSuperstar",
      "sevenpool": "7pool",
      "shawnp0wers": "shawnp0wers",
      "sigmapoolcom": "Sigmapool.com",
      "simplecoinus": "simplecoin.us",
      "solock": "Solo CK",
      "spiderpool": "SpiderPool",
      "stminingcorp": "ST Mining Corp",
      "tangpool": "Tangpool",
      "tatmaspool": "TATMAS Pool",
      "tbdice": "TBDice",
      "telco214": "Telco 214",
      "terrapool": "Terra Pool",
      "tiger": "tiger",
      "tigerpoolnet": "tigerpool.net",
      "titan": "Titan",
      "transactioncoinmining": "transactioncoinmining",
      "trickysbtcpool": "Tricky's BTC Pool",
      "triplemining": "TripleMining",
      "twentyoneinc": "21 Inc.",
      "ultimuspool": "ULTIMUSPOOL",
      "unknown": "Unknown",
      "unomp": "UNOMP",
      "viabtc": "ViaBTC",
      "waterhole": "Waterhole",
      "wayicn": "WAYI.CN",
      "whitepool": "WhitePool",
      "wiz": "wiz",
      "wk057": "wk057",
      "yourbtcnet": "Yourbtc.net",
      "zulupool": "Zulupool"
    }

    TERM_NAMES = {
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
    }

    EPOCH_NAMES = {
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
    }

    CLASS_NAMES = {
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
    }

    SPENDABLE_TYPE_NAMES = {
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
    }

    AGE_RANGE_NAMES = {
      "under_1h": {
        "id": "under_1h_old",
        "short": "<1h",
        "long": "Under 1 Hour Old"
      },
      "_1h_to_1d": {
        "id": "1h_to_1d_old",
        "short": "1h-1d",
        "long": "1 Hour to 1 Day Old"
      },
      "_1d_to_1w": {
        "id": "1d_to_1w_old",
        "short": "1d-1w",
        "long": "1 Day to 1 Week Old"
      },
      "_1w_to_1m": {
        "id": "1w_to_1m_old",
        "short": "1w-1m",
        "long": "1 Week to 1 Month Old"
      },
      "_1m_to_2m": {
        "id": "1m_to_2m_old",
        "short": "1m-2m",
        "long": "1 to 2 Months Old"
      },
      "_2m_to_3m": {
        "id": "2m_to_3m_old",
        "short": "2m-3m",
        "long": "2 to 3 Months Old"
      },
      "_3m_to_4m": {
        "id": "3m_to_4m_old",
        "short": "3m-4m",
        "long": "3 to 4 Months Old"
      },
      "_4m_to_5m": {
        "id": "4m_to_5m_old",
        "short": "4m-5m",
        "long": "4 to 5 Months Old"
      },
      "_5m_to_6m": {
        "id": "5m_to_6m_old",
        "short": "5m-6m",
        "long": "5 to 6 Months Old"
      },
      "_6m_to_1y": {
        "id": "6m_to_1y_old",
        "short": "6m-1y",
        "long": "6 Months to 1 Year Old"
      },
      "_1y_to_2y": {
        "id": "1y_to_2y_old",
        "short": "1y-2y",
        "long": "1 to 2 Years Old"
      },
      "_2y_to_3y": {
        "id": "2y_to_3y_old",
        "short": "2y-3y",
        "long": "2 to 3 Years Old"
      },
      "_3y_to_4y": {
        "id": "3y_to_4y_old",
        "short": "3y-4y",
        "long": "3 to 4 Years Old"
      },
      "_4y_to_5y": {
        "id": "4y_to_5y_old",
        "short": "4y-5y",
        "long": "4 to 5 Years Old"
      },
      "_5y_to_6y": {
        "id": "5y_to_6y_old",
        "short": "5y-6y",
        "long": "5 to 6 Years Old"
      },
      "_6y_to_7y": {
        "id": "6y_to_7y_old",
        "short": "6y-7y",
        "long": "6 to 7 Years Old"
      },
      "_7y_to_8y": {
        "id": "7y_to_8y_old",
        "short": "7y-8y",
        "long": "7 to 8 Years Old"
      },
      "_8y_to_10y": {
        "id": "8y_to_10y_old",
        "short": "8y-10y",
        "long": "8 to 10 Years Old"
      },
      "_10y_to_12y": {
        "id": "10y_to_12y_old",
        "short": "10y-12y",
        "long": "10 to 12 Years Old"
      },
      "_12y_to_15y": {
        "id": "12y_to_15y_old",
        "short": "12y-15y",
        "long": "12 to 15 Years Old"
      },
      "over_15y": {
        "id": "over_15y_old",
        "short": "15y+",
        "long": "15+ Years Old"
      }
    }

    UNDER_AGE_NAMES = {
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
    }

    OVER_AGE_NAMES = {
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
    }

    AMOUNT_RANGE_NAMES = {
      "_0sats": {
        "id": "0sats",
        "short": "0 sats",
        "long": "0 Sats"
      },
      "_1sat_to_10sats": {
        "id": "1sat_to_10sats",
        "short": "1-10 sats",
        "long": "1-10 Sats"
      },
      "_10sats_to_100sats": {
        "id": "10sats_to_100sats",
        "short": "10-100 sats",
        "long": "10-100 Sats"
      },
      "_100sats_to_1k_sats": {
        "id": "100sats_to_1k_sats",
        "short": "100-1k sats",
        "long": "100-1K Sats"
      },
      "_1k_sats_to_10k_sats": {
        "id": "1k_sats_to_10k_sats",
        "short": "1k-10k sats",
        "long": "1K-10K Sats"
      },
      "_10k_sats_to_100k_sats": {
        "id": "10k_sats_to_100k_sats",
        "short": "10k-100k sats",
        "long": "10K-100K Sats"
      },
      "_100k_sats_to_1m_sats": {
        "id": "100k_sats_to_1m_sats",
        "short": "100k-1M sats",
        "long": "100K-1M Sats"
      },
      "_1m_sats_to_10m_sats": {
        "id": "1m_sats_to_10m_sats",
        "short": "1M-10M sats",
        "long": "1M-10M Sats"
      },
      "_10m_sats_to_1btc": {
        "id": "10m_sats_to_1btc",
        "short": "0.1-1 BTC",
        "long": "0.1-1 BTC"
      },
      "_1btc_to_10btc": {
        "id": "1btc_to_10btc",
        "short": "1-10 BTC",
        "long": "1-10 BTC"
      },
      "_10btc_to_100btc": {
        "id": "10btc_to_100btc",
        "short": "10-100 BTC",
        "long": "10-100 BTC"
      },
      "_100btc_to_1k_btc": {
        "id": "100btc_to_1k_btc",
        "short": "100-1k BTC",
        "long": "100-1K BTC"
      },
      "_1k_btc_to_10k_btc": {
        "id": "1k_btc_to_10k_btc",
        "short": "1k-10k BTC",
        "long": "1K-10K BTC"
      },
      "_10k_btc_to_100k_btc": {
        "id": "10k_btc_to_100k_btc",
        "short": "10k-100k BTC",
        "long": "10K-100K BTC"
      },
      "over_100k_btc": {
        "id": "over_100k_btc",
        "short": "100k+ BTC",
        "long": "100K+ BTC"
      }
    }

    OVER_AMOUNT_NAMES = {
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
      "_1k_sats": {
        "id": "over_1k_sats",
        "short": "1k+ sats",
        "long": "Over 1K Sats"
      },
      "_10k_sats": {
        "id": "over_10k_sats",
        "short": "10k+ sats",
        "long": "Over 10K Sats"
      },
      "_100k_sats": {
        "id": "over_100k_sats",
        "short": "100k+ sats",
        "long": "Over 100K Sats"
      },
      "_1m_sats": {
        "id": "over_1m_sats",
        "short": "1M+ sats",
        "long": "Over 1M Sats"
      },
      "_10m_sats": {
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
      "_1k_btc": {
        "id": "over_1k_btc",
        "short": "1k+ BTC",
        "long": "Over 1K BTC"
      },
      "_10k_btc": {
        "id": "over_10k_btc",
        "short": "10k+ BTC",
        "long": "Over 10K BTC"
      }
    }

    UNDER_AMOUNT_NAMES = {
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
      "_1k_sats": {
        "id": "under_1k_sats",
        "short": "<1k sats",
        "long": "Under 1K Sats"
      },
      "_10k_sats": {
        "id": "under_10k_sats",
        "short": "<10k sats",
        "long": "Under 10K Sats"
      },
      "_100k_sats": {
        "id": "under_100k_sats",
        "short": "<100k sats",
        "long": "Under 100K Sats"
      },
      "_1m_sats": {
        "id": "under_1m_sats",
        "short": "<1M sats",
        "long": "Under 1M Sats"
      },
      "_10m_sats": {
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
      "_1k_btc": {
        "id": "under_1k_btc",
        "short": "<1k BTC",
        "long": "Under 1K BTC"
      },
      "_10k_btc": {
        "id": "under_10k_btc",
        "short": "<10k BTC",
        "long": "Under 10K BTC"
      },
      "_100k_btc": {
        "id": "under_100k_btc",
        "short": "<100k BTC",
        "long": "Under 100K BTC"
      }
    }

    PROFITABILITY_RANGE_NAMES = {
      "over_1000pct_in_profit": {
        "id": "utxos_over_1000pct_in_profit",
        "short": ">1000%",
        "long": "Over 1000% Profit"
      },
      "_500pct_to_1000pct_in_profit": {
        "id": "utxos_500pct_to_1000pct_in_profit",
        "short": "500-1000%",
        "long": "500-1000% Profit"
      },
      "_300pct_to_500pct_in_profit": {
        "id": "utxos_300pct_to_500pct_in_profit",
        "short": "300-500%",
        "long": "300-500% Profit"
      },
      "_200pct_to_300pct_in_profit": {
        "id": "utxos_200pct_to_300pct_in_profit",
        "short": "200-300%",
        "long": "200-300% Profit"
      },
      "_100pct_to_200pct_in_profit": {
        "id": "utxos_100pct_to_200pct_in_profit",
        "short": "100-200%",
        "long": "100-200% Profit"
      },
      "_90pct_to_100pct_in_profit": {
        "id": "utxos_90pct_to_100pct_in_profit",
        "short": "90-100%",
        "long": "90-100% Profit"
      },
      "_80pct_to_90pct_in_profit": {
        "id": "utxos_80pct_to_90pct_in_profit",
        "short": "80-90%",
        "long": "80-90% Profit"
      },
      "_70pct_to_80pct_in_profit": {
        "id": "utxos_70pct_to_80pct_in_profit",
        "short": "70-80%",
        "long": "70-80% Profit"
      },
      "_60pct_to_70pct_in_profit": {
        "id": "utxos_60pct_to_70pct_in_profit",
        "short": "60-70%",
        "long": "60-70% Profit"
      },
      "_50pct_to_60pct_in_profit": {
        "id": "utxos_50pct_to_60pct_in_profit",
        "short": "50-60%",
        "long": "50-60% Profit"
      },
      "_40pct_to_50pct_in_profit": {
        "id": "utxos_40pct_to_50pct_in_profit",
        "short": "40-50%",
        "long": "40-50% Profit"
      },
      "_30pct_to_40pct_in_profit": {
        "id": "utxos_30pct_to_40pct_in_profit",
        "short": "30-40%",
        "long": "30-40% Profit"
      },
      "_20pct_to_30pct_in_profit": {
        "id": "utxos_20pct_to_30pct_in_profit",
        "short": "20-30%",
        "long": "20-30% Profit"
      },
      "_10pct_to_20pct_in_profit": {
        "id": "utxos_10pct_to_20pct_in_profit",
        "short": "10-20%",
        "long": "10-20% Profit"
      },
      "_0pct_to_10pct_in_profit": {
        "id": "utxos_0pct_to_10pct_in_profit",
        "short": "0-10%",
        "long": "0-10% Profit"
      },
      "_0pct_to_10pct_in_loss": {
        "id": "utxos_0pct_to_10pct_in_loss",
        "short": "0-10%L",
        "long": "0-10% Loss"
      },
      "_10pct_to_20pct_in_loss": {
        "id": "utxos_10pct_to_20pct_in_loss",
        "short": "10-20%L",
        "long": "10-20% Loss"
      },
      "_20pct_to_30pct_in_loss": {
        "id": "utxos_20pct_to_30pct_in_loss",
        "short": "20-30%L",
        "long": "20-30% Loss"
      },
      "_30pct_to_40pct_in_loss": {
        "id": "utxos_30pct_to_40pct_in_loss",
        "short": "30-40%L",
        "long": "30-40% Loss"
      },
      "_40pct_to_50pct_in_loss": {
        "id": "utxos_40pct_to_50pct_in_loss",
        "short": "40-50%L",
        "long": "40-50% Loss"
      },
      "_50pct_to_60pct_in_loss": {
        "id": "utxos_50pct_to_60pct_in_loss",
        "short": "50-60%L",
        "long": "50-60% Loss"
      },
      "_60pct_to_70pct_in_loss": {
        "id": "utxos_60pct_to_70pct_in_loss",
        "short": "60-70%L",
        "long": "60-70% Loss"
      },
      "_70pct_to_80pct_in_loss": {
        "id": "utxos_70pct_to_80pct_in_loss",
        "short": "70-80%L",
        "long": "70-80% Loss"
      },
      "_80pct_to_90pct_in_loss": {
        "id": "utxos_80pct_to_90pct_in_loss",
        "short": "80-90%L",
        "long": "80-90% Loss"
      },
      "_90pct_to_100pct_in_loss": {
        "id": "utxos_90pct_to_100pct_in_loss",
        "short": "90-100%L",
        "long": "90-100% Loss"
      }
    }

    PROFIT_NAMES = {
      "breakeven": {
        "id": "utxos_in_profit",
        "short": "≥0%",
        "long": "In Profit (Breakeven+)"
      },
      "_10pct": {
        "id": "utxos_over_10pct_in_profit",
        "short": "≥10%",
        "long": "10%+ Profit"
      },
      "_20pct": {
        "id": "utxos_over_20pct_in_profit",
        "short": "≥20%",
        "long": "20%+ Profit"
      },
      "_30pct": {
        "id": "utxos_over_30pct_in_profit",
        "short": "≥30%",
        "long": "30%+ Profit"
      },
      "_40pct": {
        "id": "utxos_over_40pct_in_profit",
        "short": "≥40%",
        "long": "40%+ Profit"
      },
      "_50pct": {
        "id": "utxos_over_50pct_in_profit",
        "short": "≥50%",
        "long": "50%+ Profit"
      },
      "_60pct": {
        "id": "utxos_over_60pct_in_profit",
        "short": "≥60%",
        "long": "60%+ Profit"
      },
      "_70pct": {
        "id": "utxos_over_70pct_in_profit",
        "short": "≥70%",
        "long": "70%+ Profit"
      },
      "_80pct": {
        "id": "utxos_over_80pct_in_profit",
        "short": "≥80%",
        "long": "80%+ Profit"
      },
      "_90pct": {
        "id": "utxos_over_90pct_in_profit",
        "short": "≥90%",
        "long": "90%+ Profit"
      },
      "_100pct": {
        "id": "utxos_over_100pct_in_profit",
        "short": "≥100%",
        "long": "100%+ Profit"
      },
      "_200pct": {
        "id": "utxos_over_200pct_in_profit",
        "short": "≥200%",
        "long": "200%+ Profit"
      },
      "_300pct": {
        "id": "utxos_over_300pct_in_profit",
        "short": "≥300%",
        "long": "300%+ Profit"
      },
      "_500pct": {
        "id": "utxos_over_500pct_in_profit",
        "short": "≥500%",
        "long": "500%+ Profit"
      }
    }

    LOSS_NAMES = {
      "breakeven": {
        "id": "utxos_in_loss",
        "short": "<0%",
        "long": "In Loss (Below Breakeven)"
      },
      "_10pct": {
        "id": "utxos_over_10pct_in_loss",
        "short": "≥10%L",
        "long": "10%+ Loss"
      },
      "_20pct": {
        "id": "utxos_over_20pct_in_loss",
        "short": "≥20%L",
        "long": "20%+ Loss"
      },
      "_30pct": {
        "id": "utxos_over_30pct_in_loss",
        "short": "≥30%L",
        "long": "30%+ Loss"
      },
      "_40pct": {
        "id": "utxos_over_40pct_in_loss",
        "short": "≥40%L",
        "long": "40%+ Loss"
      },
      "_50pct": {
        "id": "utxos_over_50pct_in_loss",
        "short": "≥50%L",
        "long": "50%+ Loss"
      },
      "_60pct": {
        "id": "utxos_over_60pct_in_loss",
        "short": "≥60%L",
        "long": "60%+ Loss"
      },
      "_70pct": {
        "id": "utxos_over_70pct_in_loss",
        "short": "≥70%L",
        "long": "70%+ Loss"
      },
      "_80pct": {
        "id": "utxos_over_80pct_in_loss",
        "short": "≥80%L",
        "long": "80%+ Loss"
      }
    }

    def __init__(self, base_url: str = 'http://localhost:3000', timeout: float = 30.0):
        super().__init__(base_url, timeout)
        self.metrics = MetricsTree(self)

    def metric(self, metric: str, index: Index) -> MetricEndpointBuilder[Any]:
        """Create a dynamic metric endpoint builder for any metric/index combination.

        Use this for programmatic access when the metric name is determined at runtime.
        For type-safe access, use the `metrics` tree instead.
        """
        return MetricEndpointBuilder(self, metric, index)

    def index_to_date(self, index: Index, i: int) -> Union[date, datetime]:
        """Convert an index value to a date/datetime for date-based indexes."""
        return _index_to_date(index, i)

    def date_to_index(self, index: Index, d: Union[date, datetime]) -> int:
        """Convert a date/datetime to an index value for date-based indexes."""
        return _date_to_index(index, d)

    def get_api(self) -> Any:
        """Compact OpenAPI specification.

        Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.

        Endpoint: `GET /api.json`"""
        return self.get_json('/api.json')

    def get_address(self, address: Address) -> AddressStats:
        """Address information.

        Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*

        Endpoint: `GET /api/address/{address}`"""
        return self.get_json(f'/api/address/{address}')

    def get_address_txs(self, address: Address, after_txid: Optional[Txid] = None) -> List[Transaction]:
        """Address transactions.

        Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. Use ?after_txid=<txid> for pagination.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

        Endpoint: `GET /api/address/{address}/txs`"""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        query = '&'.join(params)
        path = f'/api/address/{address}/txs{"?" + query if query else ""}'
        return self.get_json(path)

    def get_address_confirmed_txs(self, address: Address, after_txid: Optional[Txid] = None) -> List[Transaction]:
        """Address confirmed transactions.

        Get confirmed transactions for an address, 25 per page. Use ?after_txid=<txid> for pagination.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

        Endpoint: `GET /api/address/{address}/txs/chain`"""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        query = '&'.join(params)
        path = f'/api/address/{address}/txs/chain{"?" + query if query else ""}'
        return self.get_json(path)

    def get_address_mempool_txs(self, address: Address) -> List[Txid]:
        """Address mempool transactions.

        Get unconfirmed transaction IDs for an address from the mempool (up to 50).

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*

        Endpoint: `GET /api/address/{address}/txs/mempool`"""
        return self.get_json(f'/api/address/{address}/txs/mempool')

    def get_address_utxos(self, address: Address) -> List[Utxo]:
        """Address UTXOs.

        Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*

        Endpoint: `GET /api/address/{address}/utxo`"""
        return self.get_json(f'/api/address/{address}/utxo')

    def get_block_by_height(self, height: Height) -> BlockInfo:
        """Block by height.

        Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*

        Endpoint: `GET /api/block-height/{height}`"""
        return self.get_json(f'/api/block-height/{height}')

    def get_block(self, hash: BlockHash) -> BlockInfo:
        """Block information.

        Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*

        Endpoint: `GET /api/block/{hash}`"""
        return self.get_json(f'/api/block/{hash}')

    def get_block_raw(self, hash: BlockHash) -> List[float]:
        """Raw block.

        Returns the raw block data in binary format.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*

        Endpoint: `GET /api/block/{hash}/raw`"""
        return self.get_json(f'/api/block/{hash}/raw')

    def get_block_status(self, hash: BlockHash) -> BlockStatus:
        """Block status.

        Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*

        Endpoint: `GET /api/block/{hash}/status`"""
        return self.get_json(f'/api/block/{hash}/status')

    def get_block_txid(self, hash: BlockHash, index: TxIndex) -> Txid:
        """Transaction ID at index.

        Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-id)*

        Endpoint: `GET /api/block/{hash}/txid/{index}`"""
        return self.get_json(f'/api/block/{hash}/txid/{index}')

    def get_block_txids(self, hash: BlockHash) -> List[Txid]:
        """Block transaction IDs.

        Retrieve all transaction IDs in a block. Returns an array of txids in block order.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*

        Endpoint: `GET /api/block/{hash}/txids`"""
        return self.get_json(f'/api/block/{hash}/txids')

    def get_block_txs(self, hash: BlockHash, start_index: TxIndex) -> List[Transaction]:
        """Block transactions (paginated).

        Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*

        Endpoint: `GET /api/block/{hash}/txs/{start_index}`"""
        return self.get_json(f'/api/block/{hash}/txs/{start_index}')

    def get_blocks(self) -> List[BlockInfo]:
        """Recent blocks.

        Retrieve the last 10 blocks. Returns block metadata for each block.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

        Endpoint: `GET /api/blocks`"""
        return self.get_json('/api/blocks')

    def get_blocks_from_height(self, height: Height) -> List[BlockInfo]:
        """Blocks from height.

        Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*

        Endpoint: `GET /api/blocks/{height}`"""
        return self.get_json(f'/api/blocks/{height}')

    def get_mempool(self) -> MempoolInfo:
        """Mempool statistics.

        Get current mempool statistics including transaction count, total vsize, and total fees.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*

        Endpoint: `GET /api/mempool/info`"""
        return self.get_json('/api/mempool/info')

    def get_live_price(self) -> Dollars:
        """Live BTC/USD price.

        Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.

        Endpoint: `GET /api/mempool/price`"""
        return self.get_json('/api/mempool/price')

    def get_mempool_txids(self) -> List[Txid]:
        """Mempool transaction IDs.

        Get all transaction IDs currently in the mempool.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

        Endpoint: `GET /api/mempool/txids`"""
        return self.get_json('/api/mempool/txids')

    def get_metric_info(self, metric: Metric) -> MetricInfo:
        """Get metric info.

        Returns the supported indexes and value type for the specified metric.

        Endpoint: `GET /api/metric/{metric}`"""
        return self.get_json(f'/api/metric/{metric}')

    def get_metric(self, metric: Metric, index: Index, start: Optional[RangeIndex] = None, end: Optional[RangeIndex] = None, limit: Optional[Limit] = None, format: Optional[Format] = None) -> Union[AnyMetricData, str]:
        """Get metric data.

        Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).

        Endpoint: `GET /api/metric/{metric}/{index}`"""
        params = []
        if start is not None: params.append(f'start={start}')
        if end is not None: params.append(f'end={end}')
        if limit is not None: params.append(f'limit={limit}')
        if format is not None: params.append(f'format={format}')
        query = '&'.join(params)
        path = f'/api/metric/{metric}/{index}{"?" + query if query else ""}'
        if format == 'csv':
            return self.get_text(path)
        return self.get_json(path)

    def get_metric_data(self, metric: Metric, index: Index, start: Optional[RangeIndex] = None, end: Optional[RangeIndex] = None, limit: Optional[Limit] = None, format: Optional[Format] = None) -> Union[List[bool], str]:
        """Get raw metric data.

        Returns just the data array without the MetricData wrapper. Supports the same range and format parameters as the standard endpoint.

        Endpoint: `GET /api/metric/{metric}/{index}/data`"""
        params = []
        if start is not None: params.append(f'start={start}')
        if end is not None: params.append(f'end={end}')
        if limit is not None: params.append(f'limit={limit}')
        if format is not None: params.append(f'format={format}')
        query = '&'.join(params)
        path = f'/api/metric/{metric}/{index}/data{"?" + query if query else ""}'
        if format == 'csv':
            return self.get_text(path)
        return self.get_json(path)

    def get_metric_latest(self, metric: Metric, index: Index) -> Any:
        """Get latest metric value.

        Returns the single most recent value for a metric, unwrapped (not inside a MetricData object).

        Endpoint: `GET /api/metric/{metric}/{index}/latest`"""
        return self.get_json(f'/api/metric/{metric}/{index}/latest')

    def get_metric_len(self, metric: Metric, index: Index) -> float:
        """Get metric data length.

        Returns the total number of data points for a metric at the given index.

        Endpoint: `GET /api/metric/{metric}/{index}/len`"""
        return self.get_json(f'/api/metric/{metric}/{index}/len')

    def get_metric_version(self, metric: Metric, index: Index) -> Version:
        """Get metric version.

        Returns the current version of a metric. Changes when the metric data is updated.

        Endpoint: `GET /api/metric/{metric}/{index}/version`"""
        return self.get_json(f'/api/metric/{metric}/{index}/version')

    def get_metrics_tree(self) -> TreeNode:
        """Metrics catalog.

        Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.

        Endpoint: `GET /api/metrics`"""
        return self.get_json('/api/metrics')

    def get_metrics(self, metrics: Metrics, index: Index, start: Optional[RangeIndex] = None, end: Optional[RangeIndex] = None, limit: Optional[Limit] = None, format: Optional[Format] = None) -> Union[List[AnyMetricData], str]:
        """Bulk metric data.

        Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects. For a single metric, use `get_metric` instead.

        Endpoint: `GET /api/metrics/bulk`"""
        params = []
        params.append(f'metrics={metrics}')
        params.append(f'index={index}')
        if start is not None: params.append(f'start={start}')
        if end is not None: params.append(f'end={end}')
        if limit is not None: params.append(f'limit={limit}')
        if format is not None: params.append(f'format={format}')
        query = '&'.join(params)
        path = f'/api/metrics/bulk{"?" + query if query else ""}'
        if format == 'csv':
            return self.get_text(path)
        return self.get_json(path)

    def get_cost_basis_cohorts(self) -> List[str]:
        """Available cost basis cohorts.

        List available cohorts for cost basis distribution.

        Endpoint: `GET /api/metrics/cost-basis`"""
        return self.get_json('/api/metrics/cost-basis')

    def get_cost_basis_dates(self, cohort: Cohort) -> List[Date]:
        """Available cost basis dates.

        List available dates for a cohort's cost basis distribution.

        Endpoint: `GET /api/metrics/cost-basis/{cohort}/dates`"""
        return self.get_json(f'/api/metrics/cost-basis/{cohort}/dates')

    def get_cost_basis(self, cohort: Cohort, date: str, bucket: Optional[CostBasisBucket] = None, value: Optional[CostBasisValue] = None) -> dict:
        """Cost basis distribution.

        Get the cost basis distribution for a cohort on a specific date.

        Query params:
        - `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
        - `value`: supply (default, in BTC), realized (USD), unrealized (USD)

        Endpoint: `GET /api/metrics/cost-basis/{cohort}/{date}`"""
        params = []
        if bucket is not None: params.append(f'bucket={bucket}')
        if value is not None: params.append(f'value={value}')
        query = '&'.join(params)
        path = f'/api/metrics/cost-basis/{cohort}/{date}{"?" + query if query else ""}'
        return self.get_json(path)

    def get_metrics_count(self) -> List[MetricCount]:
        """Metric count.

        Returns the number of metrics available per index type.

        Endpoint: `GET /api/metrics/count`"""
        return self.get_json('/api/metrics/count')

    def get_indexes(self) -> List[IndexInfo]:
        """List available indexes.

        Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.

        Endpoint: `GET /api/metrics/indexes`"""
        return self.get_json('/api/metrics/indexes')

    def list_metrics(self, page: Optional[float] = None, per_page: Optional[float] = None) -> PaginatedMetrics:
        """Metrics list.

        Paginated flat list of all available metric names. Use `page` query param for pagination.

        Endpoint: `GET /api/metrics/list`"""
        params = []
        if page is not None: params.append(f'page={page}')
        if per_page is not None: params.append(f'per_page={per_page}')
        query = '&'.join(params)
        path = f'/api/metrics/list{"?" + query if query else ""}'
        return self.get_json(path)

    def search_metrics(self, q: Metric, limit: Optional[Limit] = None) -> List[Metric]:
        """Search metrics.

        Fuzzy search for metrics by name. Supports partial matches and typos.

        Endpoint: `GET /api/metrics/search`"""
        params = []
        params.append(f'q={q}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        path = f'/api/metrics/search{"?" + query if query else ""}'
        return self.get_json(path)

    def get_disk_usage(self) -> DiskUsage:
        """Disk usage.

        Returns the disk space used by BRK and Bitcoin data.

        Endpoint: `GET /api/server/disk`"""
        return self.get_json('/api/server/disk')

    def get_sync_status(self) -> SyncStatus:
        """Sync status.

        Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.

        Endpoint: `GET /api/server/sync`"""
        return self.get_json('/api/server/sync')

    def get_tx(self, txid: Txid) -> Transaction:
        """Transaction information.

        Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*

        Endpoint: `GET /api/tx/{txid}`"""
        return self.get_json(f'/api/tx/{txid}')

    def get_tx_hex(self, txid: Txid) -> Hex:
        """Transaction hex.

        Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*

        Endpoint: `GET /api/tx/{txid}/hex`"""
        return self.get_json(f'/api/tx/{txid}/hex')

    def get_tx_outspend(self, txid: Txid, vout: Vout) -> TxOutspend:
        """Output spend status.

        Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*

        Endpoint: `GET /api/tx/{txid}/outspend/{vout}`"""
        return self.get_json(f'/api/tx/{txid}/outspend/{vout}')

    def get_tx_outspends(self, txid: Txid) -> List[TxOutspend]:
        """All output spend statuses.

        Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*

        Endpoint: `GET /api/tx/{txid}/outspends`"""
        return self.get_json(f'/api/tx/{txid}/outspends')

    def get_tx_status(self, txid: Txid) -> TxStatus:
        """Transaction status.

        Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*

        Endpoint: `GET /api/tx/{txid}/status`"""
        return self.get_json(f'/api/tx/{txid}/status')

    def get_difficulty_adjustment(self) -> DifficultyAdjustment:
        """Difficulty adjustment.

        Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*

        Endpoint: `GET /api/v1/difficulty-adjustment`"""
        return self.get_json('/api/v1/difficulty-adjustment')

    def get_mempool_blocks(self) -> List[MempoolBlock]:
        """Projected mempool blocks.

        Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*

        Endpoint: `GET /api/v1/fees/mempool-blocks`"""
        return self.get_json('/api/v1/fees/mempool-blocks')

    def get_recommended_fees(self) -> RecommendedFees:
        """Recommended fees.

        Get recommended fee rates for different confirmation targets based on current mempool state.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*

        Endpoint: `GET /api/v1/fees/recommended`"""
        return self.get_json('/api/v1/fees/recommended')

    def get_block_fee_rates(self, time_period: TimePeriod) -> Any:
        """Block fee rates (WIP).

        **Work in progress.** Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*

        Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`"""
        return self.get_json(f'/api/v1/mining/blocks/fee-rates/{time_period}')

    def get_block_fees(self, time_period: TimePeriod) -> List[BlockFeesEntry]:
        """Block fees.

        Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*

        Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`"""
        return self.get_json(f'/api/v1/mining/blocks/fees/{time_period}')

    def get_block_rewards(self, time_period: TimePeriod) -> List[BlockRewardsEntry]:
        """Block rewards.

        Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*

        Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`"""
        return self.get_json(f'/api/v1/mining/blocks/rewards/{time_period}')

    def get_block_sizes_weights(self, time_period: TimePeriod) -> BlockSizesWeights:
        """Block sizes and weights.

        Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*

        Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`"""
        return self.get_json(f'/api/v1/mining/blocks/sizes-weights/{time_period}')

    def get_block_by_timestamp(self, timestamp: Timestamp) -> BlockTimestamp:
        """Block by timestamp.

        Find the block closest to a given UNIX timestamp.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*

        Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`"""
        return self.get_json(f'/api/v1/mining/blocks/timestamp/{timestamp}')

    def get_difficulty_adjustments(self) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments (all time).

        Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

        Endpoint: `GET /api/v1/mining/difficulty-adjustments`"""
        return self.get_json('/api/v1/mining/difficulty-adjustments')

    def get_difficulty_adjustments_by_period(self, time_period: TimePeriod) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments.

        Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*

        Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`"""
        return self.get_json(f'/api/v1/mining/difficulty-adjustments/{time_period}')

    def get_hashrate(self) -> HashrateSummary:
        """Network hashrate (all time).

        Get network hashrate and difficulty data for all time.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

        Endpoint: `GET /api/v1/mining/hashrate`"""
        return self.get_json('/api/v1/mining/hashrate')

    def get_hashrate_by_period(self, time_period: TimePeriod) -> HashrateSummary:
        """Network hashrate.

        Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*

        Endpoint: `GET /api/v1/mining/hashrate/{time_period}`"""
        return self.get_json(f'/api/v1/mining/hashrate/{time_period}')

    def get_pool(self, slug: PoolSlug) -> PoolDetail:
        """Mining pool details.

        Get detailed information about a specific mining pool including block counts and shares for different time periods.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*

        Endpoint: `GET /api/v1/mining/pool/{slug}`"""
        return self.get_json(f'/api/v1/mining/pool/{slug}')

    def get_pools(self) -> List[PoolInfo]:
        """List all mining pools.

        Get list of all known mining pools with their identifiers.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

        Endpoint: `GET /api/v1/mining/pools`"""
        return self.get_json('/api/v1/mining/pools')

    def get_pool_stats(self, time_period: TimePeriod) -> PoolsSummary:
        """Mining pool statistics.

        Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*

        Endpoint: `GET /api/v1/mining/pools/{time_period}`"""
        return self.get_json(f'/api/v1/mining/pools/{time_period}')

    def get_reward_stats(self, block_count: float) -> RewardStats:
        """Mining reward statistics.

        Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*

        Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`"""
        return self.get_json(f'/api/v1/mining/reward-stats/{block_count}')

    def validate_address(self, address: str) -> AddressValidation:
        """Validate address.

        Validate a Bitcoin address and get information about its type and scriptPubKey.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*

        Endpoint: `GET /api/v1/validate-address/{address}`"""
        return self.get_json(f'/api/v1/validate-address/{address}')

    def get_health(self) -> Health:
        """Health check.

        Returns the health status of the API server, including uptime information.

        Endpoint: `GET /health`"""
        return self.get_json('/health')

    def get_openapi(self) -> Any:
        """OpenAPI specification.

        Full OpenAPI 3.1 specification for this API.

        Endpoint: `GET /openapi.json`"""
        return self.get_json('/openapi.json')

    def get_version(self) -> str:
        """API version.

        Returns the current version of the API server

        Endpoint: `GET /version`"""
        return self.get_json('/version')

