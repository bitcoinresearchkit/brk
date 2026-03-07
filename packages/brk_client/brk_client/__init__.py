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
# Position within a .blk file, encoding file index and byte offset
BlkPosition = int
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
# Closing price value for a time period
Close = Cents
# Cohort identifier for cost basis distribution.
Cohort = str
# Bucket type for cost basis aggregation.
# Options: raw (no aggregation), lin200/lin500/lin1000 (linear $200/$500/$1000),
# log10/log50/log100/log200 (logarithmic with 10/50/100/200 buckets per decade).
CostBasisBucket = Literal["raw", "lin200", "lin500", "lin1000", "log10", "log50", "log100", "log200"]
# Value type for cost basis distribution.
# Options: supply (BTC), realized (USD, price × supply), unrealized (USD, spot × supply).
CostBasisValue = Literal["supply", "realized", "unrealized"]
# Output format for API responses
Format = Literal["json", "csv"]
# Maximum number of results to return. Defaults to 100 if not specified.
Limit = int
# Date in YYYYMMDD format stored as u32
Date = int
Day1 = int
Day3 = int
DifficultyEpoch = int
# US Dollar amount as floating point
Dollars = float
EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
# Fee rate in sats/vB
FeeRate = float
FundedAddressIndex = TypeIndex
HalvingEpoch = int
# Hex-encoded string
Hex = str
# Highest price value for a time period
High = Cents
Hour1 = int
Hour12 = int
Hour4 = int
# Lowest price value for a time period
Low = Cents
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
Open = Cents
OpReturnIndex = TypeIndex
OutPoint = int
# Type (P2PKH, P2WPKH, P2SH, P2TR, etc.)
OutputType = Literal["p2pk65", "p2pk33", "p2pkh", "p2ms", "p2sh", "opreturn", "p2wpkh", "p2wsh", "p2tr", "p2a", "empty", "unknown"]
P2AAddressIndex = TypeIndex
U8x2 = List[int]
P2ABytes = U8x2
P2MSOutputIndex = TypeIndex
P2PK33AddressIndex = TypeIndex
U8x33 = str
P2PK33Bytes = U8x33
P2PK65AddressIndex = TypeIndex
U8x65 = str
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
# Fixed-size boolean value optimized for on-disk storage (stored as u8)
StoredBool = int
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
# block-based (height, txindex), and address/output type indexes.
Index = Literal["minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch", "height", "txindex", "txinindex", "txoutindex", "emptyoutputindex", "opreturnindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "unknownoutputindex", "fundedaddressindex", "emptyaddressindex", "pairoutputindex"]
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
        limit: Maximum number of results to return. Defaults to 25 if not specified.
    """
    after_txid: Union[Txid, None]
    limit: int

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
        start: Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
        end: Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    start: Optional[int]
    end: Optional[int]
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
    """
    status: str
    service: str
    timestamp: str
    started_at: str
    uptime_seconds: int

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

class LimitParam(TypedDict):
    limit: Limit

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

class MetricParam(TypedDict):
    metric: Metric

class MetricSelection(TypedDict):
    """
    Selection of metrics to query

    Attributes:
        metrics: Requested metrics
        index: Index to query
        start: Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
        end: Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    metrics: Metrics
    index: Index
    start: Optional[int]
    end: Optional[int]
    limit: Union[Limit, None]
    format: Format

class MetricSelectionLegacy(TypedDict):
    """
    Legacy metric selection parameters (deprecated)

    Attributes:
        start: Inclusive starting index, if negative counts from end. Aliases: `from`, `f`, `s`
        end: Exclusive ending index, if negative counts from end. Aliases: `to`, `t`, `e`
        limit: Maximum number of values to return (ignored if `end` is set). Aliases: `count`, `c`, `l`
        format: Format of the output
    """
    index: Index
    ids: Metrics
    start: Optional[int]
    end: Optional[int]
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
        metrics: List of metric names (max 1000 per page)
    """
    current_page: int
    max_page: int
    metrics: List[str]

class Pagination(TypedDict):
    """
    Pagination parameters for paginated API endpoints

    Attributes:
        page: Pagination index
    """
    page: Optional[int]

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
        tip_height: Height of the chain tip (from Bitcoin node)
        blocks_behind: Number of blocks behind the tip
        last_indexed_at: Human-readable timestamp of the last indexed block (ISO 8601)
        last_indexed_at_unix: Unix timestamp of the last indexed block
    """
    indexed_height: Height
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
_i1 = ('minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halvingepoch', 'difficultyepoch', 'height')
_i2 = ('minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halvingepoch', 'difficultyepoch')
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
_i16 = ('halvingepoch',)
_i17 = ('difficultyepoch',)
_i18 = ('height',)
_i19 = ('txindex',)
_i20 = ('txinindex',)
_i21 = ('txoutindex',)
_i22 = ('emptyoutputindex',)
_i23 = ('opreturnindex',)
_i24 = ('p2aaddressindex',)
_i25 = ('p2msoutputindex',)
_i26 = ('p2pk33addressindex',)
_i27 = ('p2pk65addressindex',)
_i28 = ('p2pkhaddressindex',)
_i29 = ('p2shaddressindex',)
_i30 = ('p2traddressindex',)
_i31 = ('p2wpkhaddressindex',)
_i32 = ('p2wshaddressindex',)
_i33 = ('unknownoutputindex',)
_i34 = ('fundedaddressindex',)
_i35 = ('emptyaddressindex',)

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
    def halvingepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halvingepoch')
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')
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
    def halvingepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halvingepoch')
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')

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
    def halvingepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halvingepoch')

class MetricPattern16(Generic[T]):
    by: _MetricPattern16By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern16By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i16)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i16 else None

class _MetricPattern17By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')

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
    def txindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txindex')

class MetricPattern19(Generic[T]):
    by: _MetricPattern19By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern19By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i19)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i19 else None

class _MetricPattern20By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txinindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txinindex')

class MetricPattern20(Generic[T]):
    by: _MetricPattern20By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern20By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i20)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i20 else None

class _MetricPattern21By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txoutindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txoutindex')

class MetricPattern21(Generic[T]):
    by: _MetricPattern21By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern21By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i21)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i21 else None

class _MetricPattern22By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyoutputindex')

class MetricPattern22(Generic[T]):
    by: _MetricPattern22By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern22By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i22)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i22 else None

class _MetricPattern23By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def opreturnindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'opreturnindex')

class MetricPattern23(Generic[T]):
    by: _MetricPattern23By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern23By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i23)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i23 else None

class _MetricPattern24By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2aaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2aaddressindex')

class MetricPattern24(Generic[T]):
    by: _MetricPattern24By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern24By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i24)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i24 else None

class _MetricPattern25By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2msoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2msoutputindex')

class MetricPattern25(Generic[T]):
    by: _MetricPattern25By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern25By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i25)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i25 else None

class _MetricPattern26By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk33addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk33addressindex')

class MetricPattern26(Generic[T]):
    by: _MetricPattern26By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern26By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i26)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i26 else None

class _MetricPattern27By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk65addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk65addressindex')

class MetricPattern27(Generic[T]):
    by: _MetricPattern27By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern27By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i27)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i27 else None

class _MetricPattern28By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pkhaddressindex')

class MetricPattern28(Generic[T]):
    by: _MetricPattern28By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern28By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i28)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i28 else None

class _MetricPattern29By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2shaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2shaddressindex')

class MetricPattern29(Generic[T]):
    by: _MetricPattern29By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern29By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i29)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i29 else None

class _MetricPattern30By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2traddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2traddressindex')

class MetricPattern30(Generic[T]):
    by: _MetricPattern30By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern30By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i30)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i30 else None

class _MetricPattern31By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wpkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wpkhaddressindex')

class MetricPattern31(Generic[T]):
    by: _MetricPattern31By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern31By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i31)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i31 else None

class _MetricPattern32By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wshaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wshaddressindex')

class MetricPattern32(Generic[T]):
    by: _MetricPattern32By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern32By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i32)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i32 else None

class _MetricPattern33By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def unknownoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'unknownoutputindex')

class MetricPattern33(Generic[T]):
    by: _MetricPattern33By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern33By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i33)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i33 else None

class _MetricPattern34By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def fundedaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'fundedaddressindex')

class MetricPattern34(Generic[T]):
    by: _MetricPattern34By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern34By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i34)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i34 else None

class _MetricPattern35By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyaddressindex')

class MetricPattern35(Generic[T]):
    by: _MetricPattern35By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern35By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i35)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i35 else None

# Reusable structural pattern classes

class CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap_raw: MetricPattern18[CentsSats] = MetricPattern18(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.gross_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'realized_gross_pnl'))
        self.gross_pnl_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, _m(acc, 'gross_pnl_sum'))
        self.investor_cap_raw: MetricPattern18[CentsSquaredSats] = MetricPattern18(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'investor_price'))
        self.investor_price_ratio: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'investor_price_ratio'))
        self.investor_price_ratio_percentiles: RatioPattern = RatioPattern(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.lower_price_band: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_pnl_change_1m: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'net_pnl_change_1m'))
        self.net_pnl_change_1m_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_market_cap'))
        self.net_pnl_change_1m_rel_to_realized_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_pnl_change_1m_rel_to_realized_cap'))
        self.net_realized_pnl: CumulativeHeightPattern[CentsSigned] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_ema_1w: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'net_realized_pnl_ema_1w'))
        self.net_realized_pnl_rel_to_realized_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'realized_peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_change_1m: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'realized_cap_change_1m'))
        self.realized_cap_rel_to_own_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_ema_1w: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_loss_ema_1w'))
        self.realized_loss_rel_to_realized_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_loss_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, _m(acc, 'realized_loss'))
        self.realized_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_ratio: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_price_ratio_percentiles: RatioPattern = RatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_price_ratio_std_dev: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_ema_1w: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_profit_ema_1w'))
        self.realized_profit_rel_to_realized_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_to_loss_ratio: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.sell_side_risk_ratio: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_24h_ema: _1m1wPattern2 = _1m1wPattern2(client, _m(acc, 'sell_side_risk_ratio_24h_ema'))
        self.sent_in_loss: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_sum: _1m1w1y24hPattern[Sats] = _1m1w1y24hPattern(client, _m(acc, 'sent_in_loss'))
        self.sent_in_profit: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_sum: _1m1w1y24hPattern[Sats] = _1m1w1y24hPattern(client, _m(acc, 'sent_in_profit'))
        self.sopr: _24hPattern[StoredF64] = _24hPattern(client, _m(acc, 'sopr_24h'))
        self.sopr_24h_ema: _1m1wPattern = _1m1wPattern(client, _m(acc, 'sopr_24h_ema'))
        self.sopr_extended: _1m1w1yPattern[StoredF64] = _1m1w1yPattern(client, _m(acc, 'sopr'))
        self.upper_price_band: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'upper_price_band'))
        self.value_created: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_created_24h'))
        self.value_created_sum_extended: _1m1w1yPattern[Cents] = _1m1w1yPattern(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_destroyed_24h'))
        self.value_destroyed_sum_extended: _1m1w1yPattern[Cents] = _1m1w1yPattern(client, _m(acc, 'value_destroyed'))

class _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._0sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, '0sd_4y'))
        self.m0_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm0_5sd_4y'))
        self.m0_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm0_5sd_4y'))
        self.m1_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm1_5sd_4y'))
        self.m1_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm1_5sd_4y'))
        self.m1sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm1sd_4y'))
        self.m1sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm1sd_4y'))
        self.m2_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm2_5sd_4y'))
        self.m2_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm2_5sd_4y'))
        self.m2sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm2sd_4y'))
        self.m2sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm2sd_4y'))
        self.m3sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm3sd_4y'))
        self.m3sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'm3sd_4y'))
        self.p0_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p0_5sd_4y'))
        self.p0_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p0_5sd_4y'))
        self.p1_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p1_5sd_4y'))
        self.p1_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p1_5sd_4y'))
        self.p1sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p1sd_4y'))
        self.p1sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p1sd_4y'))
        self.p2_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p2_5sd_4y'))
        self.p2_5sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p2_5sd_4y'))
        self.p2sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p2sd_4y'))
        self.p2sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p2sd_4y'))
        self.p3sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p3sd_4y'))
        self.p3sd_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'p3sd_4y'))
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sd_4y'))
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sma_4y'))
        self.zscore: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'zscore_4y'))

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

class MvrvNegNetRealizedSentSoprValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[CentsSigned] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_change_1m: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'realized_cap_change_1m'))
        self.realized_loss: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_ratio: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.sent_in_loss: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sent_in_loss'))
        self.sent_in_profit: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sent_in_profit'))
        self.sopr: _24hPattern[StoredF64] = _24hPattern(client, _m(acc, 'sopr_24h'))
        self.value_created: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_created_24h'))
        self.value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_destroyed_24h'))

class BpsRatioPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'bps'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio_pct1: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct1'))
        self.ratio_pct1_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct1'))
        self.ratio_pct2: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct2'))
        self.ratio_pct2_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct2'))
        self.ratio_pct5: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct5'))
        self.ratio_pct5_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct5'))
        self.ratio_pct95: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct95'))
        self.ratio_pct95_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct95'))
        self.ratio_pct98: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct98'))
        self.ratio_pct98_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct98'))
        self.ratio_pct99: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct99'))
        self.ratio_pct99_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct99'))
        self.ratio_sma_1m: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'sma_1m'))
        self.ratio_sma_1w: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'sma_1w'))

class GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'greed_index'))
        self.gross_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_gross_pnl'))
        self.invested_capital_in_loss: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'invested_capital_in_loss'))
        self.invested_capital_in_loss_raw: MetricPattern18[CentsSats] = MetricPattern18(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'invested_capital_in_profit'))
        self.invested_capital_in_profit_raw: MetricPattern18[CentsSats] = MetricPattern18(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern18[CentsSquaredSats] = MetricPattern18(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern18[CentsSquaredSats] = MetricPattern18(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_sentiment: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_sentiment'))
        self.net_unrealized_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_unrealized_pnl'))
        self.pain_index: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'pain_index'))
        self.supply_in_loss: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_profit'))
        self.unrealized_loss: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_profit'))

class MvrvNegNetRealizedSoprValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[CentsSigned] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_change_1m: MetricPattern1[CentsSigned] = MetricPattern1(client, _m(acc, 'realized_cap_change_1m'))
        self.realized_loss: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_ratio: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.sopr: _24hPattern[StoredF64] = _24hPattern(client, _m(acc, 'sopr_24h'))
        self.value_created: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_created_24h'))
        self.value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_sum: _24hPattern[Cents] = _24hPattern(client, _m(acc, 'value_destroyed_24h'))

class NetNuplSupplyUnrealizedPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_own_gross_pnl'))
        self.net_unrealized_pnl_rel_to_own_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap'))
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_own_gross_pnl'))
        self.unrealized_loss_rel_to_own_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))
        self.unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_own_gross_pnl'))
        self.unrealized_profit_rel_to_own_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))

class RatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio_pct1: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct1'))
        self.ratio_pct1_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct1'))
        self.ratio_pct2: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct2'))
        self.ratio_pct2_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct2'))
        self.ratio_pct5: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct5'))
        self.ratio_pct5_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct5'))
        self.ratio_pct95: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct95'))
        self.ratio_pct95_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct95'))
        self.ratio_pct98: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct98'))
        self.ratio_pct98_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct98'))
        self.ratio_pct99: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'pct99'))
        self.ratio_pct99_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'pct99'))
        self.ratio_sma_1m: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'sma_1m'))
        self.ratio_sma_1w: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'sma_1w'))

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

class _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'dominance_1m'))
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'dominance_1w'))
        self._1y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'dominance_1y'))
        self._24h: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'dominance_24h'))
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'rewards'))
        self.bps: MetricPattern1[BasisPoints16] = MetricPattern1(client, _m(acc, 'dominance_bps'))
        self.cumulative: BaseBtcCentsSatsUsdPattern = BaseBtcCentsSatsUsdPattern(client, acc)
        self.height: MetricPattern18[StoredU32] = MetricPattern18(client, _m(acc, 'blocks_mined'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance_ratio'))
        self.sum: _1m1w1y24hPattern6 = _1m1w1y24hPattern6(client, acc)

class AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'average'))
        self.cumulative: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'cumulative'))
        self.max: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'max'))
        self.median: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'median'))
        self.min: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'min'))
        self.pct10: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'p10'))
        self.pct25: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'p25'))
        self.pct75: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'p75'))
        self.pct90: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'p90'))
        self.rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, acc)
        self.sum: MetricPattern18[StoredU64] = MetricPattern18(client, _m(acc, 'sum'))

class AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'average'))
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern18[StoredU64] = MetricPattern18(client, acc)
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'max'))
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'median'))
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'min'))
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p10'))
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p25'))
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p75'))
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p90'))
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class AverageGainsLossesRsiStochPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'average_gain_24h'))
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'average_loss_24h'))
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'gains_24h'))
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'losses_24h'))
        self.rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '24h'))
        self.rsi_max: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'max_24h'))
        self.rsi_min: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'min_24h'))
        self.stoch_rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'stoch_24h'))
        self.stoch_rsi_d: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'stoch_d_24h'))
        self.stoch_rsi_k: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'stoch_k_24h'))

class InvestedInvestorNegNetSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_raw: MetricPattern18[CentsSats] = MetricPattern18(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit_raw: MetricPattern18[CentsSats] = MetricPattern18(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern18[CentsSquaredSats] = MetricPattern18(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern18[CentsSquaredSats] = MetricPattern18(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_unrealized_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_unrealized_pnl'))
        self.supply_in_loss: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_profit'))
        self.unrealized_loss: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_profit'))

class AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern:
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

class AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'average'))
        self.max: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'max'))
        self.median: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'median'))
        self.min: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'min'))
        self.pct10: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'p10'))
        self.pct25: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'p25'))
        self.pct75: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'p75'))
        self.pct90: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'p90'))
        self.sum: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'sum'))

class AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'average'))
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'max'))
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'median'))
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'min'))
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p10'))
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p25'))
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p75'))
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'p90'))
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, _m(acc, 'sum'))

class _1m1w1y24hBtcCentsSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1m'))
        self._1w: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1w'))
        self._1y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1y'))
        self._24h: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '24h'))
        self.btc: MetricPattern18[Bitcoin] = MetricPattern18(client, _m(acc, 'btc'))
        self.cents: MetricPattern18[Cents] = MetricPattern18(client, _m(acc, 'cents'))
        self.sats: MetricPattern18[Sats] = MetricPattern18(client, acc)
        self.usd: MetricPattern18[Dollars] = MetricPattern18(client, _m(acc, 'usd'))

class AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'average'))
        self.max: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'max'))
        self.median: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'median'))
        self.min: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'min'))
        self.pct10: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'p10'))
        self.pct25: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'p25'))
        self.pct75: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'p75'))
        self.pct90: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'p90'))

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

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSentPattern = CoinblocksCoindaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, _m(acc, 'cost_basis'))
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: MvrvNegNetRealizedSentSoprValuePattern = MvrvNegNetRealizedSentSoprValuePattern(client, acc)
        self.relative: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: InvestedInvestorNegNetSupplyUnrealizedPattern = InvestedInvestorNegNetSupplyUnrealizedPattern(client, acc)

class ActivityAddrOutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: SentPattern = SentPattern(client, _m(acc, 'sent'))
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'addr_count'))
        self.addr_count_change_1m: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'addr_count_change_1m'))
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: MvrvRealizedPattern = MvrvRealizedPattern(client, acc)
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: SupplyPattern = SupplyPattern(client, _m(acc, 'supply_in'))

class MvrvRealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_loss: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_ratio: BpsRatioPattern = BpsRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Cents] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))

class ActivityOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: SentPattern = SentPattern(client, _m(acc, 'sent'))
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: MvrvNegNetRealizedSoprValuePattern = MvrvNegNetRealizedSoprValuePattern(client, acc)
        self.relative: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: NegNetSupplyUnrealizedPattern = NegNetSupplyUnrealizedPattern(client, acc)

class NegNetSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_unrealized_pnl: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'net_unrealized_pnl'))
        self.supply_in_loss: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'supply_in_profit'))
        self.unrealized_loss: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: CentsUsdPattern = CentsUsdPattern(client, _m(acc, 'unrealized_profit'))

class ActivityOutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: SentPattern = SentPattern(client, _m(acc, 'sent'))
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: MvrvRealizedPattern = MvrvRealizedPattern(client, acc)
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, _m(acc, 'supply'))
        self.unrealized: SupplyPattern = SupplyPattern(client, _m(acc, 'supply_in'))

class BaseBtcCentsSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_mined_cumulative'))
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'rewards_cumulative_btc'))
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'rewards_cumulative_cents'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'rewards_cumulative'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'rewards_cumulative_usd'))

class CoinblocksCoindaysSentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: CumulativeHeightPattern[StoredF64] = CumulativeHeightPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, _m(acc, 'coindays_destroyed'))
        self.sent: MetricPattern1[Sats] = MetricPattern1(client, _m(acc, 'sent'))
        self.sent_sum: _24hPattern[Sats] = _24hPattern(client, _m(acc, 'sent_24h'))
        self.sent_sum_extended: _1m1w1yPattern[Sats] = _1m1w1yPattern(client, _m(acc, 'sent'))

class EmaHistogramLineSignalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ema_fast_24h'))
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ema_slow_24h'))
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'histogram_24h'))
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'line_24h'))
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'signal_24h'))

class _1m1w1y24hHeightPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1m'))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1w'))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_1y'))
        self._24h: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average_24h'))
        self.height: MetricPattern18[T] = MetricPattern18(client, acc)

class _1m1w1y24hPattern6:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BaseBtcCentsSatsUsdPattern = BaseBtcCentsSatsUsdPattern(client, acc)
        self._1w: BaseBtcCentsSatsUsdPattern = BaseBtcCentsSatsUsdPattern(client, acc)
        self._1y: BaseBtcCentsSatsUsdPattern = BaseBtcCentsSatsUsdPattern(client, acc)
        self._24h: BaseBtcCentsSatsUsdPattern = BaseBtcCentsSatsUsdPattern(client, acc)

class _1m1w1y24hPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1m'))
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1w'))
        self._1y: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1y'))
        self._24h: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '24h'))

class _1m1w1y24hPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1m'))
        self._1w: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1w'))
        self._1y: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '1y'))
        self._24h: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, '24h'))

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
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, acc)
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class InvestedMaxMinPercentilesPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital'))
        self.max: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'cost_basis_max'))
        self.min: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'cost_basis_min'))
        self.percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis'))

class RatioPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)
        self.ratio_sd_1y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)
        self.ratio_sd_2y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)
        self.ratio_sd_4y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)

class _1m1w1y24hPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1m'))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1w'))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1y'))
        self._24h: MetricPattern1[T] = MetricPattern1(client, _m(acc, '24h'))

class BaseCumulativeSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'cumulative'))
        self.sum: _1m1w1y24hPattern5 = _1m1w1y24hPattern5(client, _m(acc, 'sum'))

class BpsPercentRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints16] = MetricPattern1(client, _m(acc, 'bps'))
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class BpsPriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'ratio_bps'))
        self.price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))

class CentsSatsUsdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern2[Cents] = MetricPattern2(client, _m(acc, 'cents'))
        self.sats: MetricPattern2[Sats] = MetricPattern2(client, _m(acc, 'sats'))
        self.usd: MetricPattern2[Dollars] = MetricPattern2(client, acc)

class CentsSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class ChangeHalvedTotalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.change_1m: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'change_1m'))
        self.halved: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'halved'))
        self.total: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)

class SupplyPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'in_loss_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'in_profit_rel_to_circulating_supply'))
        self.supply_rel_to_circulating_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'rel_to_circulating_supply'))

class _1m1w1yPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1m'))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1w'))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1y'))

class _6bBlockTxindexPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._6b: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '6b'))
        self.block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc)
        self.txindex: MetricPattern19[T] = MetricPattern19(client, acc)

class CumulativeHeightSumPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern18[T] = MetricPattern18(client, acc)
        self.sum: _1m1w1y24hPattern[T] = _1m1w1y24hPattern(client, acc)

class _1m1wPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1m'))
        self._1w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, '1w'))

class _1m1wPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, '1m'))
        self._1w: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, '1w'))

class BaseCumulativePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, acc)
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'cumulative'))

class BlocksDominancePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.blocks_mined: CumulativeHeightPattern[StoredU32] = CumulativeHeightPattern(client, _m(acc, 'blocks_mined'))
        self.dominance: BpsPercentRatioPattern = BpsPercentRatioPattern(client, _m(acc, 'dominance'))

class BpsRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bps: MetricPattern1[BasisPoints32] = MetricPattern1(client, _m(acc, 'bps'))
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)

class CentsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'cents'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class ChangeRatePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.change: _1m1w1y24hPattern[StoredI64] = _1m1w1y24hPattern(client, _m(acc, 'change'))
        self.rate: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, _m(acc, 'rate'))

class MaxMinPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'max'))
        self.min: CentsSatsUsdPattern = CentsSatsUsdPattern(client, _m(acc, 'min'))

class SdSmaPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sd_1y'))
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sma_1y'))

class SentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sent: MetricPattern1[Sats] = MetricPattern1(client, acc)
        self.sent_sum: _24hPattern[Sats] = _24hPattern(client, _m(acc, '24h'))

class SupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.supply_in_loss: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'loss'))
        self.supply_in_profit: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, _m(acc, 'profit'))

class UtxoPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, acc)
        self.utxo_count_change_1m: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'change_1m'))

class CumulativeHeightPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern18[T] = MetricPattern18(client, acc)

class _24hPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._24h: MetricPattern1[T] = MetricPattern1(client, acc)

# Metrics tree classes

class MetricsTree_Blocks_Difficulty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty')
        self.as_hash: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty_as_hash')
        self.adjustment: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'difficulty_adjustment')
        self.epoch: MetricPattern1[DifficultyEpoch] = MetricPattern1(client, 'difficulty_epoch')
        self.blocks_before_next_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_difficulty_adjustment')
        self.days_before_next_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_difficulty_adjustment')

class MetricsTree_Blocks_Time:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.timestamp: MetricPattern1[Timestamp] = MetricPattern1(client, 'timestamp')
        self.date: MetricPattern18[Date] = MetricPattern18(client, 'date')
        self.timestamp_monotonic: MetricPattern18[Timestamp] = MetricPattern18(client, 'timestamp_monotonic')

class MetricsTree_Blocks_Weight:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern18[Weight] = MetricPattern18(client, 'block_weight')
        self.cumulative: MetricPattern1[Weight] = MetricPattern1(client, 'block_weight_cumulative')
        self.sum: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_sum')
        self.average: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_average')
        self.min: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_min')
        self.max: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_max')
        self.pct10: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_p10')
        self.pct25: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_p25')
        self.median: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_median')
        self.pct75: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_p75')
        self.pct90: _1m1w1y24hPattern[Weight] = _1m1w1y24hPattern(client, 'block_weight_p90')

class MetricsTree_Blocks_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_count_target: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_count_target')
        self.block_count: CumulativeHeightSumPattern[StoredU32] = CumulativeHeightSumPattern(client, 'block_count')
        self.block_count_sum: _1m1w1y24hPattern[StoredU32] = _1m1w1y24hPattern(client, 'block_count_sum')
        self.height_1h_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_1h_ago')
        self.height_24h_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_24h_ago')
        self.height_3d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_3d_ago')
        self.height_1w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_1w_ago')
        self.height_8d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_8d_ago')
        self.height_9d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_9d_ago')
        self.height_12d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_12d_ago')
        self.height_13d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_13d_ago')
        self.height_2w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_2w_ago')
        self.height_21d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_21d_ago')
        self.height_26d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_26d_ago')
        self.height_1m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_1m_ago')
        self.height_34d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_34d_ago')
        self.height_55d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_55d_ago')
        self.height_2m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_2m_ago')
        self.height_9w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_9w_ago')
        self.height_12w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_12w_ago')
        self.height_89d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_89d_ago')
        self.height_3m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_3m_ago')
        self.height_14w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_14w_ago')
        self.height_111d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_111d_ago')
        self.height_144d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_144d_ago')
        self.height_6m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_6m_ago')
        self.height_26w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_26w_ago')
        self.height_200d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_200d_ago')
        self.height_9m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_9m_ago')
        self.height_350d_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_350d_ago')
        self.height_12m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_12m_ago')
        self.height_1y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_1y_ago')
        self.height_14m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_14m_ago')
        self.height_2y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_2y_ago')
        self.height_26m_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_26m_ago')
        self.height_3y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_3y_ago')
        self.height_200w_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_200w_ago')
        self.height_4y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_4y_ago')
        self.height_5y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_5y_ago')
        self.height_6y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_6y_ago')
        self.height_8y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_8y_ago')
        self.height_9y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_9y_ago')
        self.height_10y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_10y_ago')
        self.height_12y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_12y_ago')
        self.height_14y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_14y_ago')
        self.height_26y_ago: MetricPattern18[Height] = MetricPattern18(client, 'height_26y_ago')

class MetricsTree_Blocks_Halving:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.epoch: MetricPattern1[HalvingEpoch] = MetricPattern1(client, 'halving_epoch')
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_halving')

class MetricsTree_Blocks_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_size_cumulative')
        self.sum: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_sum')
        self.average: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_average')
        self.min: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_min')
        self.max: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_max')
        self.pct10: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_p10')
        self.pct25: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_p25')
        self.median: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_median')
        self.pct75: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_p75')
        self.pct90: _1m1w1y24hPattern[StoredU64] = _1m1w1y24hPattern(client, 'block_size_p90')

class MetricsTree_Blocks_Fullness:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bps: _1m1w1y24hHeightPattern[BasisPoints16] = _1m1w1y24hHeightPattern(client, 'block_fullness_bps')
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, 'block_fullness_ratio')
        self.percent: MetricPattern1[StoredF32] = MetricPattern1(client, 'block_fullness')

class MetricsTree_Blocks:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern18[BlockHash] = MetricPattern18(client, 'blockhash')
        self.difficulty: MetricsTree_Blocks_Difficulty = MetricsTree_Blocks_Difficulty(client)
        self.time: MetricsTree_Blocks_Time = MetricsTree_Blocks_Time(client)
        self.total_size: MetricPattern18[StoredU64] = MetricPattern18(client, 'total_size')
        self.weight: MetricsTree_Blocks_Weight = MetricsTree_Blocks_Weight(client)
        self.count: MetricsTree_Blocks_Count = MetricsTree_Blocks_Count(client)
        self.interval: _1m1w1y24hHeightPattern[Timestamp] = _1m1w1y24hHeightPattern(client, 'block_interval')
        self.halving: MetricsTree_Blocks_Halving = MetricsTree_Blocks_Halving(client)
        self.vbytes: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'block_vbytes')
        self.size: MetricsTree_Blocks_Size = MetricsTree_Blocks_Size(client)
        self.fullness: MetricsTree_Blocks_Fullness = MetricsTree_Blocks_Fullness(client)

class MetricsTree_Transactions_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.tx_count: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'tx_count')
        self.is_coinbase: MetricPattern19[StoredBool] = MetricPattern19(client, 'is_coinbase')

class MetricsTree_Transactions_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: _6bBlockTxindexPattern[VSize] = _6bBlockTxindexPattern(client, 'tx_vsize')
        self.weight: _6bBlockTxindexPattern[Weight] = _6bBlockTxindexPattern(client, 'tx_weight')

class MetricsTree_Transactions_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.input_value: MetricPattern19[Sats] = MetricPattern19(client, 'input_value')
        self.output_value: MetricPattern19[Sats] = MetricPattern19(client, 'output_value')
        self.fee: _6bBlockTxindexPattern[Sats] = _6bBlockTxindexPattern(client, 'fee')
        self.fee_rate: _6bBlockTxindexPattern[FeeRate] = _6bBlockTxindexPattern(client, 'fee_rate')

class MetricsTree_Transactions_Versions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v1')
        self.v2: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v2')
        self.v3: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v3')

class MetricsTree_Transactions_Volume:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sent_sum: _1m1w1y24hBtcCentsSatsUsdPattern = _1m1w1y24hBtcCentsSatsUsdPattern(client, 'sent_sum')
        self.received_sum: _1m1w1y24hBtcCentsSatsUsdPattern = _1m1w1y24hBtcCentsSatsUsdPattern(client, 'received_sum')
        self.annualized_volume: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'annualized_volume')
        self.tx_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'tx_per_sec')
        self.outputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'outputs_per_sec')
        self.inputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'inputs_per_sec')

class MetricsTree_Transactions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txindex: MetricPattern18[TxIndex] = MetricPattern18(client, 'first_txindex')
        self.height: MetricPattern19[Height] = MetricPattern19(client, 'height')
        self.txid: MetricPattern19[Txid] = MetricPattern19(client, 'txid')
        self.txversion: MetricPattern19[TxVersion] = MetricPattern19(client, 'txversion')
        self.rawlocktime: MetricPattern19[RawLockTime] = MetricPattern19(client, 'rawlocktime')
        self.base_size: MetricPattern19[StoredU32] = MetricPattern19(client, 'base_size')
        self.total_size: MetricPattern19[StoredU32] = MetricPattern19(client, 'total_size')
        self.is_explicitly_rbf: MetricPattern19[StoredBool] = MetricPattern19(client, 'is_explicitly_rbf')
        self.first_txinindex: MetricPattern19[TxInIndex] = MetricPattern19(client, 'first_txinindex')
        self.first_txoutindex: MetricPattern19[TxOutIndex] = MetricPattern19(client, 'first_txoutindex')
        self.count: MetricsTree_Transactions_Count = MetricsTree_Transactions_Count(client)
        self.size: MetricsTree_Transactions_Size = MetricsTree_Transactions_Size(client)
        self.fees: MetricsTree_Transactions_Fees = MetricsTree_Transactions_Fees(client)
        self.versions: MetricsTree_Transactions_Versions = MetricsTree_Transactions_Versions(client)
        self.volume: MetricsTree_Transactions_Volume = MetricsTree_Transactions_Volume(client)

class MetricsTree_Inputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern20[TxOutIndex] = MetricPattern20(client, 'txoutindex')
        self.value: MetricPattern20[Sats] = MetricPattern20(client, 'value')

class MetricsTree_Inputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txinindex: MetricPattern18[TxInIndex] = MetricPattern18(client, 'first_txinindex')
        self.outpoint: MetricPattern20[OutPoint] = MetricPattern20(client, 'outpoint')
        self.txindex: MetricPattern20[TxIndex] = MetricPattern20(client, 'txindex')
        self.outputtype: MetricPattern20[OutputType] = MetricPattern20(client, 'outputtype')
        self.typeindex: MetricPattern20[TypeIndex] = MetricPattern20(client, 'typeindex')
        self.spent: MetricsTree_Inputs_Spent = MetricsTree_Inputs_Spent(client)
        self.count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'input_count')

class MetricsTree_Outputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern21[TxInIndex] = MetricPattern21(client, 'txinindex')

class MetricsTree_Outputs_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'output_count')
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, 'exact_utxo_count')

class MetricsTree_Outputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txoutindex: MetricPattern18[TxOutIndex] = MetricPattern18(client, 'first_txoutindex')
        self.value: MetricPattern21[Sats] = MetricPattern21(client, 'value')
        self.outputtype: MetricPattern21[OutputType] = MetricPattern21(client, 'outputtype')
        self.typeindex: MetricPattern21[TypeIndex] = MetricPattern21(client, 'typeindex')
        self.txindex: MetricPattern21[TxIndex] = MetricPattern21(client, 'txindex')
        self.spent: MetricsTree_Outputs_Spent = MetricsTree_Outputs_Spent(client)
        self.count: MetricsTree_Outputs_Count = MetricsTree_Outputs_Count(client)

class MetricsTree_Addresses:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2pk65addressindex: MetricPattern18[P2PK65AddressIndex] = MetricPattern18(client, 'first_p2pk65addressindex')
        self.first_p2pk33addressindex: MetricPattern18[P2PK33AddressIndex] = MetricPattern18(client, 'first_p2pk33addressindex')
        self.first_p2pkhaddressindex: MetricPattern18[P2PKHAddressIndex] = MetricPattern18(client, 'first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern18[P2SHAddressIndex] = MetricPattern18(client, 'first_p2shaddressindex')
        self.first_p2wpkhaddressindex: MetricPattern18[P2WPKHAddressIndex] = MetricPattern18(client, 'first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern18[P2WSHAddressIndex] = MetricPattern18(client, 'first_p2wshaddressindex')
        self.first_p2traddressindex: MetricPattern18[P2TRAddressIndex] = MetricPattern18(client, 'first_p2traddressindex')
        self.first_p2aaddressindex: MetricPattern18[P2AAddressIndex] = MetricPattern18(client, 'first_p2aaddressindex')
        self.p2pk65bytes: MetricPattern27[P2PK65Bytes] = MetricPattern27(client, 'p2pk65bytes')
        self.p2pk33bytes: MetricPattern26[P2PK33Bytes] = MetricPattern26(client, 'p2pk33bytes')
        self.p2pkhbytes: MetricPattern28[P2PKHBytes] = MetricPattern28(client, 'p2pkhbytes')
        self.p2shbytes: MetricPattern29[P2SHBytes] = MetricPattern29(client, 'p2shbytes')
        self.p2wpkhbytes: MetricPattern31[P2WPKHBytes] = MetricPattern31(client, 'p2wpkhbytes')
        self.p2wshbytes: MetricPattern32[P2WSHBytes] = MetricPattern32(client, 'p2wshbytes')
        self.p2trbytes: MetricPattern30[P2TRBytes] = MetricPattern30(client, 'p2trbytes')
        self.p2abytes: MetricPattern24[P2ABytes] = MetricPattern24(client, 'p2abytes')

class MetricsTree_Scripts_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2a_count')
        self.p2ms: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2ms_count')
        self.p2pk33: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2pk33_count')
        self.p2pk65: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2pk65_count')
        self.p2pkh: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2pkh_count')
        self.p2sh: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2sh_count')
        self.p2tr: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2tr_count')
        self.p2wpkh: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2wpkh_count')
        self.p2wsh: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'p2wsh_count')
        self.opreturn: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'opreturn_count')
        self.emptyoutput: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'emptyoutput_count')
        self.unknownoutput: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'unknownoutput_count')
        self.segwit: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'segwit_count')

class MetricsTree_Scripts_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BaseCumulativePattern = BaseCumulativePattern(client, 'opreturn_value')

class MetricsTree_Scripts_Adoption:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.taproot: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'taproot_adoption')
        self.segwit: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'segwit_adoption')

class MetricsTree_Scripts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_emptyoutputindex: MetricPattern18[EmptyOutputIndex] = MetricPattern18(client, 'first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern18[OpReturnIndex] = MetricPattern18(client, 'first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern18[P2MSOutputIndex] = MetricPattern18(client, 'first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern18[UnknownOutputIndex] = MetricPattern18(client, 'first_unknownoutputindex')
        self.empty_to_txindex: MetricPattern22[TxIndex] = MetricPattern22(client, 'txindex')
        self.opreturn_to_txindex: MetricPattern23[TxIndex] = MetricPattern23(client, 'txindex')
        self.p2ms_to_txindex: MetricPattern25[TxIndex] = MetricPattern25(client, 'txindex')
        self.unknown_to_txindex: MetricPattern33[TxIndex] = MetricPattern33(client, 'txindex')
        self.count: MetricsTree_Scripts_Count = MetricsTree_Scripts_Count(client)
        self.value: MetricsTree_Scripts_Value = MetricsTree_Scripts_Value(client)
        self.adoption: MetricsTree_Scripts_Adoption = MetricsTree_Scripts_Adoption(client)

class MetricsTree_Mining_Rewards_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'fees')
        self.cumulative: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'fees_cumulative')
        self._24h: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fees_24h')
        self._1w: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fees_1w')
        self._1m: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fees_1m')
        self._1y: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 = AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fees_1y')

class MetricsTree_Mining_Rewards:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinbase: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'coinbase')
        self.subsidy: BaseCumulativePattern = BaseCumulativePattern(client, 'subsidy')
        self.fees: MetricsTree_Mining_Rewards_Fees = MetricsTree_Mining_Rewards_Fees(client)
        self.unclaimed_rewards: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'unclaimed_rewards')
        self.fee_dominance: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'fee_dominance')
        self.fee_dominance_rolling: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, 'fee_dominance')
        self.subsidy_dominance: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'subsidy_dominance')
        self.subsidy_dominance_rolling: _1m1w1y24hPattern2 = _1m1w1y24hPattern2(client, 'subsidy_dominance')
        self.subsidy_sma_1y: CentsUsdPattern = CentsUsdPattern(client, 'subsidy_sma_1y')

class MetricsTree_Mining_Hashrate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.hash_rate: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate')
        self.hash_rate_sma_1w: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1w')
        self.hash_rate_sma_1m: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1m')
        self.hash_rate_sma_2m: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_2m')
        self.hash_rate_sma_1y: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_sma_1y')
        self.hash_rate_ath: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_ath')
        self.hash_rate_drawdown: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'hash_rate_drawdown')
        self.hash_price_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths')
        self.hash_price_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths_min')
        self.hash_price_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs')
        self.hash_price_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs_min')
        self.hash_price_rebound: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'hash_price_rebound')
        self.hash_value_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths')
        self.hash_value_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths_min')
        self.hash_value_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs')
        self.hash_value_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs_min')
        self.hash_value_rebound: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'hash_value_rebound')

class MetricsTree_Mining:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.rewards: MetricsTree_Mining_Rewards = MetricsTree_Mining_Rewards(client)
        self.hashrate: MetricsTree_Mining_Hashrate = MetricsTree_Mining_Hashrate(client)

class MetricsTree_Positions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_position: MetricPattern18[BlkPosition] = MetricPattern18(client, 'position')
        self.tx_position: MetricPattern19[BlkPosition] = MetricPattern19(client, 'position')

class MetricsTree_Cointime_Activity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinblocks_created: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'coinblocks_created')
        self.coinblocks_stored: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, 'liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, 'vaultedness')
        self.activity_to_vaultedness_ratio: MetricPattern1[StoredF64] = MetricPattern1(client, 'activity_to_vaultedness_ratio')

class MetricsTree_Cointime_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted_supply: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'vaulted_supply')
        self.active_supply: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'active_supply')

class MetricsTree_Cointime_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_value_destroyed: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'cointime_value_destroyed')
        self.cointime_value_created: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'cointime_value_created')
        self.cointime_value_stored: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'cointime_value_stored')
        self.vocdd: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, 'vocdd')

class MetricsTree_Cointime_Cap:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.thermo_cap: CentsUsdPattern = CentsUsdPattern(client, 'thermo_cap')
        self.investor_cap: CentsUsdPattern = CentsUsdPattern(client, 'investor_cap')
        self.vaulted_cap: CentsUsdPattern = CentsUsdPattern(client, 'vaulted_cap')
        self.active_cap: CentsUsdPattern = CentsUsdPattern(client, 'active_cap')
        self.cointime_cap: CentsUsdPattern = CentsUsdPattern(client, 'cointime_cap')

class MetricsTree_Cointime_Pricing:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'vaulted_price')
        self.vaulted_price_ratio: BpsRatioPattern2 = BpsRatioPattern2(client, 'vaulted_price_ratio')
        self.active_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'active_price')
        self.active_price_ratio: BpsRatioPattern2 = BpsRatioPattern2(client, 'active_price_ratio')
        self.true_market_mean: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'true_market_mean')
        self.true_market_mean_ratio: BpsRatioPattern2 = BpsRatioPattern2(client, 'true_market_mean_ratio')
        self.cointime_price: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'cointime_price')
        self.cointime_price_ratio: BpsRatioPattern2 = BpsRatioPattern2(client, 'cointime_price_ratio')

class MetricsTree_Cointime_Adjusted:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'cointime_adj_inflation_rate')
        self.cointime_adj_tx_velocity_btc: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_velocity_btc')
        self.cointime_adj_tx_velocity_usd: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_velocity_usd')

class MetricsTree_Cointime_ReserveRisk:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vocdd_median_1y: MetricPattern18[StoredF64] = MetricPattern18(client, 'vocdd_median_1y')
        self.hodl_bank: MetricPattern18[StoredF64] = MetricPattern18(client, 'hodl_bank')
        self.reserve_risk: MetricPattern1[StoredF64] = MetricPattern1(client, 'reserve_risk')

class MetricsTree_Cointime:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: MetricsTree_Cointime_Activity = MetricsTree_Cointime_Activity(client)
        self.supply: MetricsTree_Cointime_Supply = MetricsTree_Cointime_Supply(client)
        self.value: MetricsTree_Cointime_Value = MetricsTree_Cointime_Value(client)
        self.cap: MetricsTree_Cointime_Cap = MetricsTree_Cointime_Cap(client)
        self.pricing: MetricsTree_Cointime_Pricing = MetricsTree_Cointime_Pricing(client)
        self.adjusted: MetricsTree_Cointime_Adjusted = MetricsTree_Cointime_Adjusted(client)
        self.reserve_risk: MetricsTree_Cointime_ReserveRisk = MetricsTree_Cointime_ReserveRisk(client)

class MetricsTree_Constants:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.constant_0: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_0')
        self.constant_1: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_1')
        self.constant_2: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_2')
        self.constant_3: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_3')
        self.constant_4: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_4')
        self.constant_20: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_20')
        self.constant_30: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_30')
        self.constant_38_2: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_38_2')
        self.constant_50: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_50')
        self.constant_61_8: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_61_8')
        self.constant_70: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_70')
        self.constant_80: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_80')
        self.constant_100: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_100')
        self.constant_600: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_600')
        self.constant_minus_1: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_1')
        self.constant_minus_2: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_2')
        self.constant_minus_3: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_3')
        self.constant_minus_4: MetricPattern1[StoredI8] = MetricPattern1(client, 'constant_minus_4')

class MetricsTree_Indexes_Address_P2pk33:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern26[P2PK33AddressIndex] = MetricPattern26(client, 'p2pk33addressindex')

class MetricsTree_Indexes_Address_P2pk65:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern27[P2PK65AddressIndex] = MetricPattern27(client, 'p2pk65addressindex')

class MetricsTree_Indexes_Address_P2pkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern28[P2PKHAddressIndex] = MetricPattern28(client, 'p2pkhaddressindex')

class MetricsTree_Indexes_Address_P2sh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern29[P2SHAddressIndex] = MetricPattern29(client, 'p2shaddressindex')

class MetricsTree_Indexes_Address_P2tr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern30[P2TRAddressIndex] = MetricPattern30(client, 'p2traddressindex')

class MetricsTree_Indexes_Address_P2wpkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern31[P2WPKHAddressIndex] = MetricPattern31(client, 'p2wpkhaddressindex')

class MetricsTree_Indexes_Address_P2wsh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern32[P2WSHAddressIndex] = MetricPattern32(client, 'p2wshaddressindex')

class MetricsTree_Indexes_Address_P2a:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern24[P2AAddressIndex] = MetricPattern24(client, 'p2aaddressindex')

class MetricsTree_Indexes_Address_P2ms:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern25[P2MSOutputIndex] = MetricPattern25(client, 'p2msoutputindex')

class MetricsTree_Indexes_Address_Empty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern22[EmptyOutputIndex] = MetricPattern22(client, 'emptyoutputindex')

class MetricsTree_Indexes_Address_Unknown:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern33[UnknownOutputIndex] = MetricPattern33(client, 'unknownoutputindex')

class MetricsTree_Indexes_Address_Opreturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern23[OpReturnIndex] = MetricPattern23(client, 'opreturnindex')

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
        self.opreturn: MetricsTree_Indexes_Address_Opreturn = MetricsTree_Indexes_Address_Opreturn(client)

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
        self.difficultyepoch: MetricPattern18[DifficultyEpoch] = MetricPattern18(client, 'difficultyepoch')
        self.halvingepoch: MetricPattern18[HalvingEpoch] = MetricPattern18(client, 'halvingepoch')
        self.week1: MetricPattern18[Week1] = MetricPattern18(client, 'week1')
        self.month1: MetricPattern18[Month1] = MetricPattern18(client, 'month1')
        self.month3: MetricPattern18[Month3] = MetricPattern18(client, 'month3')
        self.month6: MetricPattern18[Month6] = MetricPattern18(client, 'month6')
        self.year1: MetricPattern18[Year1] = MetricPattern18(client, 'year1')
        self.year10: MetricPattern18[Year10] = MetricPattern18(client, 'year10')
        self.txindex_count: MetricPattern18[StoredU64] = MetricPattern18(client, 'txindex_count')

class MetricsTree_Indexes_Difficultyepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern17[DifficultyEpoch] = MetricPattern17(client, 'difficultyepoch')
        self.first_height: MetricPattern17[Height] = MetricPattern17(client, 'first_height')
        self.height_count: MetricPattern17[StoredU64] = MetricPattern17(client, 'height_count')

class MetricsTree_Indexes_Halvingepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern16[HalvingEpoch] = MetricPattern16(client, 'halvingepoch')
        self.first_height: MetricPattern16[Height] = MetricPattern16(client, 'first_height')

class MetricsTree_Indexes_Minute10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern3[Minute10] = MetricPattern3(client, 'minute10')
        self.first_height: MetricPattern3[Height] = MetricPattern3(client, 'minute10_first_height')

class MetricsTree_Indexes_Minute30:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern4[Minute30] = MetricPattern4(client, 'minute30')
        self.first_height: MetricPattern4[Height] = MetricPattern4(client, 'minute30_first_height')

class MetricsTree_Indexes_Hour1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern5[Hour1] = MetricPattern5(client, 'hour1')
        self.first_height: MetricPattern5[Height] = MetricPattern5(client, 'hour1_first_height')

class MetricsTree_Indexes_Hour4:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern6[Hour4] = MetricPattern6(client, 'hour4')
        self.first_height: MetricPattern6[Height] = MetricPattern6(client, 'hour4_first_height')

class MetricsTree_Indexes_Hour12:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern7[Hour12] = MetricPattern7(client, 'hour12')
        self.first_height: MetricPattern7[Height] = MetricPattern7(client, 'hour12_first_height')

class MetricsTree_Indexes_Day1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern8[Day1] = MetricPattern8(client, 'day1')
        self.date: MetricPattern8[Date] = MetricPattern8(client, 'date')
        self.first_height: MetricPattern8[Height] = MetricPattern8(client, 'first_height')
        self.height_count: MetricPattern8[StoredU64] = MetricPattern8(client, 'height_count')

class MetricsTree_Indexes_Day3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern9[Day3] = MetricPattern9(client, 'day3')
        self.first_height: MetricPattern9[Height] = MetricPattern9(client, 'day3_first_height')

class MetricsTree_Indexes_Week1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern10[Week1] = MetricPattern10(client, 'week1')
        self.date: MetricPattern10[Date] = MetricPattern10(client, 'date')
        self.first_height: MetricPattern10[Height] = MetricPattern10(client, 'week1_first_height')

class MetricsTree_Indexes_Month1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern11[Month1] = MetricPattern11(client, 'month1')
        self.date: MetricPattern11[Date] = MetricPattern11(client, 'date')
        self.first_height: MetricPattern11[Height] = MetricPattern11(client, 'month1_first_height')

class MetricsTree_Indexes_Month3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern12[Month3] = MetricPattern12(client, 'month3')
        self.date: MetricPattern12[Date] = MetricPattern12(client, 'date')
        self.first_height: MetricPattern12[Height] = MetricPattern12(client, 'month3_first_height')

class MetricsTree_Indexes_Month6:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern13[Month6] = MetricPattern13(client, 'month6')
        self.date: MetricPattern13[Date] = MetricPattern13(client, 'date')
        self.first_height: MetricPattern13[Height] = MetricPattern13(client, 'month6_first_height')

class MetricsTree_Indexes_Year1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern14[Year1] = MetricPattern14(client, 'year1')
        self.date: MetricPattern14[Date] = MetricPattern14(client, 'date')
        self.first_height: MetricPattern14[Height] = MetricPattern14(client, 'year1_first_height')

class MetricsTree_Indexes_Year10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern15[Year10] = MetricPattern15(client, 'year10')
        self.date: MetricPattern15[Date] = MetricPattern15(client, 'date')
        self.first_height: MetricPattern15[Height] = MetricPattern15(client, 'year10_first_height')

class MetricsTree_Indexes_Txindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern19[TxIndex] = MetricPattern19(client, 'txindex')
        self.input_count: MetricPattern19[StoredU64] = MetricPattern19(client, 'input_count')
        self.output_count: MetricPattern19[StoredU64] = MetricPattern19(client, 'output_count')

class MetricsTree_Indexes_Txinindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern20[TxInIndex] = MetricPattern20(client, 'txinindex')

class MetricsTree_Indexes_Txoutindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern21[TxOutIndex] = MetricPattern21(client, 'txoutindex')

class MetricsTree_Indexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: MetricsTree_Indexes_Address = MetricsTree_Indexes_Address(client)
        self.height: MetricsTree_Indexes_Height = MetricsTree_Indexes_Height(client)
        self.difficultyepoch: MetricsTree_Indexes_Difficultyepoch = MetricsTree_Indexes_Difficultyepoch(client)
        self.halvingepoch: MetricsTree_Indexes_Halvingepoch = MetricsTree_Indexes_Halvingepoch(client)
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
        self.txindex: MetricsTree_Indexes_Txindex = MetricsTree_Indexes_Txindex(client)
        self.txinindex: MetricsTree_Indexes_Txinindex = MetricsTree_Indexes_Txinindex(client)
        self.txoutindex: MetricsTree_Indexes_Txoutindex = MetricsTree_Indexes_Txoutindex(client)

class MetricsTree_Market_Ath:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ath: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_ath')
        self.price_drawdown: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_drawdown')
        self.days_since_price_ath: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_since_price_ath')
        self.years_since_price_ath: MetricPattern2[StoredF32] = MetricPattern2(client, 'years_since_price_ath')
        self.max_days_between_price_ath: MetricPattern1[StoredF32] = MetricPattern1(client, 'max_days_between_price_ath')
        self.max_years_between_price_ath: MetricPattern2[StoredF32] = MetricPattern2(client, 'max_years_between_price_ath')

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

class MetricsTree_Market_Returns_PriceReturn:
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

class MetricsTree_Market_Returns_PriceReturn24hSd1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sma_1w')
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sd_1w')

class MetricsTree_Market_Returns_PriceReturn24hSd1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sma_1m')
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_return_24h_sd_1m')

class MetricsTree_Market_Returns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_return: MetricsTree_Market_Returns_PriceReturn = MetricsTree_Market_Returns_PriceReturn(client)
        self.price_cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'price_cagr')
        self.price_return_24h_sd_1w: MetricsTree_Market_Returns_PriceReturn24hSd1w = MetricsTree_Market_Returns_PriceReturn24hSd1w(client)
        self.price_return_24h_sd_1m: MetricsTree_Market_Returns_PriceReturn24hSd1m = MetricsTree_Market_Returns_PriceReturn24hSd1m(client)
        self.price_return_24h_sd_1y: SdSmaPattern = SdSmaPattern(client, 'price_return_24h')

class MetricsTree_Market_Volatility:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_volatility_1w: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1w')
        self.price_volatility_1m: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1m')
        self.price_volatility_1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_volatility_1y')

class MetricsTree_Market_Range:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_min_1w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_min_1w')
        self.price_max_1w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_max_1w')
        self.price_min_2w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_min_2w')
        self.price_max_2w: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_max_2w')
        self.price_min_1m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_min_1m')
        self.price_max_1m: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_max_1m')
        self.price_min_1y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_min_1y')
        self.price_max_1y: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_max_1y')
        self.price_true_range: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range')
        self.price_true_range_sum_2w: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range_sum_2w')
        self.price_choppiness_index_2w: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'price_choppiness_index_2w')

class MetricsTree_Market_MovingAverage:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_sma_1w: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_1w')
        self.price_sma_8d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_8d')
        self.price_sma_13d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_13d')
        self.price_sma_21d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_21d')
        self.price_sma_1m: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_1m')
        self.price_sma_34d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_34d')
        self.price_sma_55d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_55d')
        self.price_sma_89d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_89d')
        self.price_sma_111d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_111d')
        self.price_sma_144d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_144d')
        self.price_sma_200d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_200d')
        self.price_sma_350d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_350d')
        self.price_sma_1y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_1y')
        self.price_sma_2y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_2y')
        self.price_sma_200w: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_200w')
        self.price_sma_4y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_sma_4y')
        self.price_ema_1w: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_1w')
        self.price_ema_8d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_8d')
        self.price_ema_12d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_12d')
        self.price_ema_13d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_13d')
        self.price_ema_21d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_21d')
        self.price_ema_26d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_26d')
        self.price_ema_1m: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_1m')
        self.price_ema_34d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_34d')
        self.price_ema_55d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_55d')
        self.price_ema_89d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_89d')
        self.price_ema_144d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_144d')
        self.price_ema_200d: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_200d')
        self.price_ema_1y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_1y')
        self.price_ema_2y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_2y')
        self.price_ema_200w: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_200w')
        self.price_ema_4y: BpsPriceRatioPattern = BpsPriceRatioPattern(client, 'price_ema_4y')
        self.price_sma_200d_x2_4: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_200d_x2_4')
        self.price_sma_200d_x0_8: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_200d_x0_8')
        self.price_sma_350d_x2: CentsSatsUsdPattern = CentsSatsUsdPattern(client, 'price_sma_350d_x2')

class MetricsTree_Market_Dca_PeriodCostBasis:
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

class MetricsTree_Market_Dca_ClassStack:
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

class MetricsTree_Market_Dca_ClassCostBasis:
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

class MetricsTree_Market_Dca_ClassReturn:
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

class MetricsTree_Market_Dca:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dca_sats_per_day: MetricPattern18[Sats] = MetricPattern18(client, 'dca_sats_per_day')
        self.period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'dca_stack')
        self.period_cost_basis: MetricsTree_Market_Dca_PeriodCostBasis = MetricsTree_Market_Dca_PeriodCostBasis(client)
        self.period_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_return')
        self.period_cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'dca_cagr')
        self.period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'lump_sum_stack')
        self.period_lump_sum_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_return')
        self.class_stack: MetricsTree_Market_Dca_ClassStack = MetricsTree_Market_Dca_ClassStack(client)
        self.class_cost_basis: MetricsTree_Market_Dca_ClassCostBasis = MetricsTree_Market_Dca_ClassCostBasis(client)
        self.class_return: MetricsTree_Market_Dca_ClassReturn = MetricsTree_Market_Dca_ClassReturn(client)

class MetricsTree_Market_Indicators_Rsi_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1w')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1w')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_gain_1w')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_loss_1w')
        self.rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_1w')
        self.rsi_min: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_min_1w')
        self.rsi_max: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_max_1w')
        self.stoch_rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_1w')
        self.stoch_rsi_k: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_k_1w')
        self.stoch_rsi_d: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_d_1w')

class MetricsTree_Market_Indicators_Rsi_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1m')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1m')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_gain_1m')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_loss_1m')
        self.rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_1m')
        self.rsi_min: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_min_1m')
        self.rsi_max: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_max_1m')
        self.stoch_rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_1m')
        self.stoch_rsi_k: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_k_1m')
        self.stoch_rsi_d: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_d_1m')

class MetricsTree_Market_Indicators_Rsi_1y:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1y')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1y')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_gain_1y')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_average_loss_1y')
        self.rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_1y')
        self.rsi_min: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_min_1y')
        self.rsi_max: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_max_1y')
        self.stoch_rsi: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_1y')
        self.stoch_rsi_k: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_k_1y')
        self.stoch_rsi_d: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'rsi_stoch_d_1y')

class MetricsTree_Market_Indicators_Rsi:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')
        self._1w: MetricsTree_Market_Indicators_Rsi_1w = MetricsTree_Market_Indicators_Rsi_1w(client)
        self._1m: MetricsTree_Market_Indicators_Rsi_1m = MetricsTree_Market_Indicators_Rsi_1m(client)
        self._1y: MetricsTree_Market_Indicators_Rsi_1y = MetricsTree_Market_Indicators_Rsi_1y(client)

class MetricsTree_Market_Indicators_Macd_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1w')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1w')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1w')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1w')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1w')

class MetricsTree_Market_Indicators_Macd_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1m')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1m')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1m')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1m')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1m')

class MetricsTree_Market_Indicators_Macd_1y:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ema_fast: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_fast_1y')
        self.ema_slow: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_ema_slow_1y')
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1y')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1y')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1y')

class MetricsTree_Market_Indicators_Macd:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h: EmaHistogramLineSignalPattern = EmaHistogramLineSignalPattern(client, 'macd')
        self._1w: MetricsTree_Market_Indicators_Macd_1w = MetricsTree_Market_Indicators_Macd_1w(client)
        self._1m: MetricsTree_Market_Indicators_Macd_1m = MetricsTree_Market_Indicators_Macd_1m(client)
        self._1y: MetricsTree_Market_Indicators_Macd_1y = MetricsTree_Market_Indicators_Macd_1y(client)

class MetricsTree_Market_Indicators:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.puell_multiple: BpsRatioPattern = BpsRatioPattern(client, 'puell_multiple')
        self.nvt: BpsRatioPattern = BpsRatioPattern(client, 'nvt')
        self.rsi: MetricsTree_Market_Indicators_Rsi = MetricsTree_Market_Indicators_Rsi(client)
        self.stoch_k: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'stoch_k')
        self.stoch_d: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'stoch_d')
        self.pi_cycle: BpsRatioPattern = BpsRatioPattern(client, 'pi_cycle')
        self.macd: MetricsTree_Market_Indicators_Macd = MetricsTree_Market_Indicators_Macd(client)
        self.gini: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'gini')

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
        self.indicators: MetricsTree_Market_Indicators = MetricsTree_Market_Indicators(client)

class MetricsTree_Pools_Major:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.unknown: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'unknown')
        self.luxor: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'luxor')
        self.btccom: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'btccom')
        self.btctop: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'btctop')
        self.btcguild: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'btcguild')
        self.eligius: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'eligius')
        self.f2pool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'f2pool')
        self.braiinspool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'braiinspool')
        self.antpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'antpool')
        self.btcc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'btcc')
        self.bwpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'bwpool')
        self.bitfury: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'bitfury')
        self.viabtc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'viabtc')
        self.poolin: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'poolin')
        self.spiderpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'spiderpool')
        self.binancepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'binancepool')
        self.foundryusa: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'foundryusa')
        self.sbicrypto: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'sbicrypto')
        self.marapool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'marapool')
        self.secpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'secpool')
        self.ocean: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'ocean')
        self.whitepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern = _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern(client, 'whitepool')

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

class MetricsTree_Prices_Split_Close:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: MetricPattern2[Cents] = MetricPattern2(client, 'price_close_cents')
        self.usd: MetricPattern2[Dollars] = MetricPattern2(client, 'price_close')
        self.sats: MetricPattern2[Sats] = MetricPattern2(client, 'price_close_sats')

class MetricsTree_Prices_Split:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.open: CentsSatsUsdPattern2 = CentsSatsUsdPattern2(client, 'price_open')
        self.high: CentsSatsUsdPattern2 = CentsSatsUsdPattern2(client, 'price_high')
        self.low: CentsSatsUsdPattern2 = CentsSatsUsdPattern2(client, 'price_low')
        self.close: MetricsTree_Prices_Split_Close = MetricsTree_Prices_Split_Close(client)

class MetricsTree_Prices_Ohlc:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: MetricPattern2[OHLCCents] = MetricPattern2(client, 'price_ohlc_cents')
        self.usd: MetricPattern2[OHLCDollars] = MetricPattern2(client, 'price_ohlc')
        self.sats: MetricPattern2[OHLCSats] = MetricPattern2(client, 'price_ohlc_sats')

class MetricsTree_Prices_Price:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: MetricPattern1[Cents] = MetricPattern1(client, 'price_cents')
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, 'price')
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, 'price_sats')

class MetricsTree_Prices:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.split: MetricsTree_Prices_Split = MetricsTree_Prices_Split(client)
        self.ohlc: MetricsTree_Prices_Ohlc = MetricsTree_Prices_Ohlc(client)
        self.price: MetricsTree_Prices_Price = MetricsTree_Prices_Price(client)

class MetricsTree_Distribution_AnyAddressIndexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern24[AnyAddressIndex] = MetricPattern24(client, 'anyaddressindex')
        self.p2pk33: MetricPattern26[AnyAddressIndex] = MetricPattern26(client, 'anyaddressindex')
        self.p2pk65: MetricPattern27[AnyAddressIndex] = MetricPattern27(client, 'anyaddressindex')
        self.p2pkh: MetricPattern28[AnyAddressIndex] = MetricPattern28(client, 'anyaddressindex')
        self.p2sh: MetricPattern29[AnyAddressIndex] = MetricPattern29(client, 'anyaddressindex')
        self.p2tr: MetricPattern30[AnyAddressIndex] = MetricPattern30(client, 'anyaddressindex')
        self.p2wpkh: MetricPattern31[AnyAddressIndex] = MetricPattern31(client, 'anyaddressindex')
        self.p2wsh: MetricPattern32[AnyAddressIndex] = MetricPattern32(client, 'anyaddressindex')

class MetricsTree_Distribution_AddressesData:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.funded: MetricPattern34[FundedAddressData] = MetricPattern34(client, 'fundedaddressdata')
        self.empty: MetricPattern35[EmptyAddressData] = MetricPattern35(client, 'emptyaddressdata')

class MetricsTree_Distribution_UtxoCohorts_All_Adjusted:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.adjusted_value_created: MetricPattern1[Cents] = MetricPattern1(client, 'adjusted_value_created')
        self.adjusted_value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, 'adjusted_value_destroyed')
        self.adjusted_value_created_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, 'adjusted_value_created')
        self.adjusted_value_destroyed_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, 'adjusted_value_destroyed')
        self.adjusted_sopr: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'adjusted_sopr')
        self.adjusted_sopr_ema: _1m1wPattern = _1m1wPattern(client, 'adjusted_sopr_24h_ema')

class MetricsTree_Distribution_UtxoCohorts_All_Relative:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'supply_in_profit_rel_to_own_supply')
        self.supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'supply_in_loss_rel_to_own_supply')
        self.unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'unrealized_profit_rel_to_market_cap')
        self.unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'unrealized_loss_rel_to_market_cap')
        self.net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'net_unrealized_pnl_rel_to_market_cap')
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, 'nupl')
        self.unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'unrealized_profit_rel_to_own_gross_pnl')
        self.unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'unrealized_loss_rel_to_own_gross_pnl')
        self.net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'net_unrealized_pnl_rel_to_own_gross_pnl')

class MetricsTree_Distribution_UtxoCohorts_All:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, 'supply')
        self.outputs: UtxoPattern = UtxoPattern(client, 'utxo_count')
        self.activity: CoinblocksCoindaysSentPattern = CoinblocksCoindaysSentPattern(client, '')
        self.realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern = CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, '')
        self.cost_basis: InvestedMaxMinPercentilesPattern = InvestedMaxMinPercentilesPattern(client, '')
        self.unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern = GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, '')
        self.adjusted: MetricsTree_Distribution_UtxoCohorts_All_Adjusted = MetricsTree_Distribution_UtxoCohorts_All_Adjusted(client)
        self.relative: MetricsTree_Distribution_UtxoCohorts_All_Relative = MetricsTree_Distribution_UtxoCohorts_All_Relative(client)
        self.dormancy: MetricPattern1[StoredF32] = MetricPattern1(client, 'dormancy')
        self.velocity: MetricPattern1[StoredF32] = MetricPattern1(client, 'velocity')

class MetricsTree_Distribution_UtxoCohorts_Sth:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, 'sth_supply')
        self.outputs: UtxoPattern = UtxoPattern(client, 'sth_utxo_count')
        self.activity: CoinblocksCoindaysSentPattern = CoinblocksCoindaysSentPattern(client, 'sth')
        self.realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern = CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, 'sth')
        self.cost_basis: InvestedMaxMinPercentilesPattern = InvestedMaxMinPercentilesPattern(client, 'sth')
        self.unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern = GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, 'sth')
        self.relative: NetNuplSupplyUnrealizedPattern2 = NetNuplSupplyUnrealizedPattern2(client, 'sth')
        self.dormancy: MetricPattern1[StoredF32] = MetricPattern1(client, 'sth_dormancy')
        self.velocity: MetricPattern1[StoredF32] = MetricPattern1(client, 'sth_velocity')
        self.adjusted_value_created: MetricPattern1[Cents] = MetricPattern1(client, 'sth_adjusted_value_created')
        self.adjusted_value_destroyed: MetricPattern1[Cents] = MetricPattern1(client, 'sth_adjusted_value_destroyed')
        self.adjusted_value_created_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, 'sth_adjusted_value_created')
        self.adjusted_value_destroyed_sum: _1m1w1y24hPattern[Cents] = _1m1w1y24hPattern(client, 'sth_adjusted_value_destroyed')
        self.adjusted_sopr: _1m1w1y24hPattern[StoredF64] = _1m1w1y24hPattern(client, 'sth_adjusted_sopr')
        self.adjusted_sopr_ema: _1m1wPattern = _1m1wPattern(client, 'sth_adjusted_sopr_24h_ema')

class MetricsTree_Distribution_UtxoCohorts_Lth:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: ChangeHalvedTotalPattern = ChangeHalvedTotalPattern(client, 'lth_supply')
        self.outputs: UtxoPattern = UtxoPattern(client, 'lth_utxo_count')
        self.activity: CoinblocksCoindaysSentPattern = CoinblocksCoindaysSentPattern(client, 'lth')
        self.realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern = CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern(client, 'lth')
        self.cost_basis: InvestedMaxMinPercentilesPattern = InvestedMaxMinPercentilesPattern(client, 'lth')
        self.unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern = GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern(client, 'lth')
        self.relative: NetNuplSupplyUnrealizedPattern2 = NetNuplSupplyUnrealizedPattern2(client, 'lth')
        self.dormancy: MetricPattern1[StoredF32] = MetricPattern1(client, 'lth_dormancy')
        self.velocity: MetricPattern1[StoredF32] = MetricPattern1(client, 'lth_velocity')

class MetricsTree_Distribution_UtxoCohorts_AgeRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.up_to_1h: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_1h_old')
        self._1h_to_1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_1h_to_1d_old')
        self._1d_to_1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_1d_to_1w_old')
        self._1w_to_1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_1w_to_1m_old')
        self._1m_to_2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_1m_to_2m_old')
        self._2m_to_3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_2m_to_3m_old')
        self._3m_to_4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_3m_to_4m_old')
        self._4m_to_5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_4m_to_5m_old')
        self._5m_to_6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_5m_to_6m_old')
        self._6m_to_1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_6m_to_1y_old')
        self._1y_to_2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_1y_to_2y_old')
        self._2y_to_3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_2y_to_3y_old')
        self._3y_to_4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_3y_to_4y_old')
        self._4y_to_5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_4y_to_5y_old')
        self._5y_to_6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_5y_to_6y_old')
        self._6y_to_7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_6y_to_7y_old')
        self._7y_to_8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_7y_to_8y_old')
        self._8y_to_10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_8y_to_10y_old')
        self._10y_to_12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_10y_to_12y_old')
        self._12y_to_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_12y_to_15y_old')
        self.from_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_15y_old')

class MetricsTree_Distribution_UtxoCohorts_MaxAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_1w_old')
        self._1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_1m_old')
        self._2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_2m_old')
        self._3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_3m_old')
        self._4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_4m_old')
        self._5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_5m_old')
        self._6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_6m_old')
        self._1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_1y_old')
        self._2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_2y_old')
        self._3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_3y_old')
        self._4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_4y_old')
        self._5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_5y_old')
        self._6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_6y_old')
        self._7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_7y_old')
        self._8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_8y_old')
        self._10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_10y_old')
        self._12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_12y_old')
        self._15y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_under_15y_old')

class MetricsTree_Distribution_UtxoCohorts_MinAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_1d_old')
        self._1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_1w_old')
        self._1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_1m_old')
        self._2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_2m_old')
        self._3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_3m_old')
        self._4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_4m_old')
        self._5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_5m_old')
        self._6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_6m_old')
        self._1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_1y_old')
        self._2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_2y_old')
        self._3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_3y_old')
        self._4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_4y_old')
        self._5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_5y_old')
        self._6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_6y_old')
        self._7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_7y_old')
        self._8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_8y_old')
        self._10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_10y_old')
        self._12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'utxos_over_12y_old')

class MetricsTree_Distribution_UtxoCohorts_Epoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'epoch_0')
        self._1: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'epoch_1')
        self._2: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'epoch_2')
        self._3: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'epoch_3')
        self._4: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'epoch_4')

class MetricsTree_Distribution_UtxoCohorts_Class:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2009: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2009')
        self._2010: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2010')
        self._2011: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2011')
        self._2012: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2012')
        self._2013: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2013')
        self._2014: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2014')
        self._2015: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2015')
        self._2016: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2016')
        self._2017: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2017')
        self._2018: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2018')
        self._2019: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2019')
        self._2020: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2020')
        self._2021: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2021')
        self._2022: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2022')
        self._2023: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2023')
        self._2024: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2024')
        self._2025: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2025')
        self._2026: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'class_2026')

class MetricsTree_Distribution_UtxoCohorts_GeAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1sat')
        self._10sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10sats')
        self._100sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100sats')
        self._1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1k_sats')
        self._10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10k_sats')
        self._100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100k_sats')
        self._1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1m_sats')
        self._10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10m_sats')
        self._1btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1btc')
        self._10btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10btc')
        self._100btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_100btc')
        self._1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_1k_btc')
        self._10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_over_10k_btc')

class MetricsTree_Distribution_UtxoCohorts_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_with_0sats')
        self._1sat_to_10sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_1sat_under_10sats')
        self._10sats_to_100sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_10sats_under_100sats')
        self._100sats_to_1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_100sats_under_1k_sats')
        self._1k_sats_to_10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_1k_sats_under_10k_sats')
        self._10k_sats_to_100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_10k_sats_under_100k_sats')
        self._100k_sats_to_1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_100k_sats_under_1m_sats')
        self._1m_sats_to_10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_1m_sats_under_10m_sats')
        self._10m_sats_to_1btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_10m_sats_under_1btc')
        self._1btc_to_10btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_1btc_under_10btc')
        self._10btc_to_100btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_10btc_under_100btc')
        self._100btc_to_1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_100btc_under_1k_btc')
        self._1k_btc_to_10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_1k_btc_under_10k_btc')
        self._10k_btc_to_100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_10k_btc_under_100k_btc')
        self._100k_btc_or_more: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_above_100k_btc')

class MetricsTree_Distribution_UtxoCohorts_LtAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10sats')
        self._100sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100sats')
        self._1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1k_sats')
        self._10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10k_sats')
        self._100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100k_sats')
        self._1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1m_sats')
        self._10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10m_sats')
        self._1btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1btc')
        self._10btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10btc')
        self._100btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100btc')
        self._1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_1k_btc')
        self._10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_10k_btc')
        self._100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'utxos_under_100k_btc')

class MetricsTree_Distribution_UtxoCohorts_Type:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk65: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2pk65')
        self.p2pk33: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2pk33')
        self.p2pkh: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2pkh')
        self.p2ms: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2ms')
        self.p2sh: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2sh')
        self.p2wpkh: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2wpkh')
        self.p2wsh: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2wsh')
        self.p2tr: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2tr')
        self.p2a: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'p2a')
        self.unknown: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'unknown_outputs')
        self.empty: ActivityOutputsRealizedSupplyUnrealizedPattern = ActivityOutputsRealizedSupplyUnrealizedPattern(client, 'empty_outputs')

class MetricsTree_Distribution_UtxoCohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricsTree_Distribution_UtxoCohorts_All = MetricsTree_Distribution_UtxoCohorts_All(client)
        self.sth: MetricsTree_Distribution_UtxoCohorts_Sth = MetricsTree_Distribution_UtxoCohorts_Sth(client)
        self.lth: MetricsTree_Distribution_UtxoCohorts_Lth = MetricsTree_Distribution_UtxoCohorts_Lth(client)
        self.age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange = MetricsTree_Distribution_UtxoCohorts_AgeRange(client)
        self.max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge = MetricsTree_Distribution_UtxoCohorts_MaxAge(client)
        self.min_age: MetricsTree_Distribution_UtxoCohorts_MinAge = MetricsTree_Distribution_UtxoCohorts_MinAge(client)
        self.epoch: MetricsTree_Distribution_UtxoCohorts_Epoch = MetricsTree_Distribution_UtxoCohorts_Epoch(client)
        self.class: MetricsTree_Distribution_UtxoCohorts_Class = MetricsTree_Distribution_UtxoCohorts_Class(client)
        self.ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount = MetricsTree_Distribution_UtxoCohorts_GeAmount(client)
        self.amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange = MetricsTree_Distribution_UtxoCohorts_AmountRange(client)
        self.lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount = MetricsTree_Distribution_UtxoCohorts_LtAmount(client)
        self.type_: MetricsTree_Distribution_UtxoCohorts_Type = MetricsTree_Distribution_UtxoCohorts_Type(client)

class MetricsTree_Distribution_AddressCohorts_GeAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1sat')
        self._10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10sats')
        self._100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100sats')
        self._1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1k_sats')
        self._10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10k_sats')
        self._100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100k_sats')
        self._1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1m_sats')
        self._10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10m_sats')
        self._1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1btc')
        self._10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10btc')
        self._100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_100btc')
        self._1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_1k_btc')
        self._10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_over_10k_btc')

class MetricsTree_Distribution_AddressCohorts_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_with_0sats')
        self._1sat_to_10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_1sat_under_10sats')
        self._10sats_to_100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_10sats_under_100sats')
        self._100sats_to_1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_100sats_under_1k_sats')
        self._1k_sats_to_10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_1k_sats_under_10k_sats')
        self._10k_sats_to_100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_10k_sats_under_100k_sats')
        self._100k_sats_to_1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_100k_sats_under_1m_sats')
        self._1m_sats_to_10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_1m_sats_under_10m_sats')
        self._10m_sats_to_1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_10m_sats_under_1btc')
        self._1btc_to_10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_1btc_under_10btc')
        self._10btc_to_100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_10btc_under_100btc')
        self._100btc_to_1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_100btc_under_1k_btc')
        self._1k_btc_to_10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_1k_btc_under_10k_btc')
        self._10k_btc_to_100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_10k_btc_under_100k_btc')
        self._100k_btc_or_more: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_above_100k_btc')

class MetricsTree_Distribution_AddressCohorts_LtAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10sats')
        self._100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100sats')
        self._1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1k_sats')
        self._10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10k_sats')
        self._100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100k_sats')
        self._1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1m_sats')
        self._10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10m_sats')
        self._1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1btc')
        self._10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10btc')
        self._100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100btc')
        self._1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_1k_btc')
        self._10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_10k_btc')
        self._100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern = ActivityAddrOutputsRealizedSupplyUnrealizedPattern(client, 'addrs_under_100k_btc')

class MetricsTree_Distribution_AddressCohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount = MetricsTree_Distribution_AddressCohorts_GeAmount(client)
        self.amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange = MetricsTree_Distribution_AddressCohorts_AmountRange(client)
        self.lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount = MetricsTree_Distribution_AddressCohorts_LtAmount(client)

class MetricsTree_Distribution_AddressActivity:
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

class MetricsTree_Distribution_NewAddrCount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'new_addr_count')
        self.p2pk65: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2pk65_new_addr_count')
        self.p2pk33: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2pk33_new_addr_count')
        self.p2pkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2pkh_new_addr_count')
        self.p2sh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2sh_new_addr_count')
        self.p2wpkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2wpkh_new_addr_count')
        self.p2wsh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2wsh_new_addr_count')
        self.p2tr: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2tr_new_addr_count')
        self.p2a: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, 'p2a_new_addr_count')

class MetricsTree_Distribution_Delta:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: ChangeRatePattern = ChangeRatePattern(client, 'addr_count')
        self.p2pk65: ChangeRatePattern = ChangeRatePattern(client, 'p2pk65_addr_count')
        self.p2pk33: ChangeRatePattern = ChangeRatePattern(client, 'p2pk33_addr_count')
        self.p2pkh: ChangeRatePattern = ChangeRatePattern(client, 'p2pkh_addr_count')
        self.p2sh: ChangeRatePattern = ChangeRatePattern(client, 'p2sh_addr_count')
        self.p2wpkh: ChangeRatePattern = ChangeRatePattern(client, 'p2wpkh_addr_count')
        self.p2wsh: ChangeRatePattern = ChangeRatePattern(client, 'p2wsh_addr_count')
        self.p2tr: ChangeRatePattern = ChangeRatePattern(client, 'p2tr_addr_count')
        self.p2a: ChangeRatePattern = ChangeRatePattern(client, 'p2a_addr_count')

class MetricsTree_Distribution:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_state: MetricPattern18[SupplyState] = MetricPattern18(client, 'supply_state')
        self.any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes = MetricsTree_Distribution_AnyAddressIndexes(client)
        self.addresses_data: MetricsTree_Distribution_AddressesData = MetricsTree_Distribution_AddressesData(client)
        self.utxo_cohorts: MetricsTree_Distribution_UtxoCohorts = MetricsTree_Distribution_UtxoCohorts(client)
        self.address_cohorts: MetricsTree_Distribution_AddressCohorts = MetricsTree_Distribution_AddressCohorts(client)
        self.addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'addr_count')
        self.empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'empty_addr_count')
        self.address_activity: MetricsTree_Distribution_AddressActivity = MetricsTree_Distribution_AddressActivity(client)
        self.total_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'total_addr_count')
        self.new_addr_count: MetricsTree_Distribution_NewAddrCount = MetricsTree_Distribution_NewAddrCount(client)
        self.delta: MetricsTree_Distribution_Delta = MetricsTree_Distribution_Delta(client)
        self.fundedaddressindex: MetricPattern34[FundedAddressIndex] = MetricPattern34(client, 'fundedaddressindex')
        self.emptyaddressindex: MetricPattern35[EmptyAddressIndex] = MetricPattern35(client, 'emptyaddressindex')

class MetricsTree_Supply_Burned:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'opreturn_supply')
        self.unspendable: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'unspendable_supply')

class MetricsTree_Supply_Velocity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern1[StoredF64] = MetricPattern1(client, 'velocity_btc')
        self.usd: MetricPattern1[StoredF64] = MetricPattern1(client, 'velocity_usd')

class MetricsTree_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.circulating: BtcCentsSatsUsdPattern = BtcCentsSatsUsdPattern(client, 'circulating_supply')
        self.burned: MetricsTree_Supply_Burned = MetricsTree_Supply_Burned(client)
        self.inflation_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'inflation_rate')
        self.velocity: MetricsTree_Supply_Velocity = MetricsTree_Supply_Velocity(client)
        self.market_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'market_cap')
        self.market_cap_growth_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'market_cap_growth_rate')
        self.realized_cap_growth_rate: BpsPercentRatioPattern = BpsPercentRatioPattern(client, 'realized_cap_growth_rate')
        self.market_minus_realized_cap_growth_rate: MetricPattern1[BasisPointsSigned32] = MetricPattern1(client, 'market_minus_realized_cap_growth_rate')

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
        self.positions: MetricsTree_Positions = MetricsTree_Positions(client)
        self.cointime: MetricsTree_Cointime = MetricsTree_Cointime(client)
        self.constants: MetricsTree_Constants = MetricsTree_Constants(client)
        self.indexes: MetricsTree_Indexes = MetricsTree_Indexes(client)
        self.market: MetricsTree_Market = MetricsTree_Market(client)
        self.pools: MetricsTree_Pools = MetricsTree_Pools(client)
        self.prices: MetricsTree_Prices = MetricsTree_Prices(client)
        self.distribution: MetricsTree_Distribution = MetricsTree_Distribution(client)
        self.supply: MetricsTree_Supply = MetricsTree_Supply(client)

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
      "up_to_1h": {
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
      "from_15y": {
        "id": "over_15y_old",
        "short": "15y+",
        "long": "15+ Years Old"
      }
    }

    MAX_AGE_NAMES = {
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

    MIN_AGE_NAMES = {
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
        "id": "with_0sats",
        "short": "0 sats",
        "long": "0 Sats"
      },
      "_1sat_to_10sats": {
        "id": "above_1sat_under_10sats",
        "short": "1-10 sats",
        "long": "1-10 Sats"
      },
      "_10sats_to_100sats": {
        "id": "above_10sats_under_100sats",
        "short": "10-100 sats",
        "long": "10-100 Sats"
      },
      "_100sats_to_1k_sats": {
        "id": "above_100sats_under_1k_sats",
        "short": "100-1k sats",
        "long": "100-1K Sats"
      },
      "_1k_sats_to_10k_sats": {
        "id": "above_1k_sats_under_10k_sats",
        "short": "1k-10k sats",
        "long": "1K-10K Sats"
      },
      "_10k_sats_to_100k_sats": {
        "id": "above_10k_sats_under_100k_sats",
        "short": "10k-100k sats",
        "long": "10K-100K Sats"
      },
      "_100k_sats_to_1m_sats": {
        "id": "above_100k_sats_under_1m_sats",
        "short": "100k-1M sats",
        "long": "100K-1M Sats"
      },
      "_1m_sats_to_10m_sats": {
        "id": "above_1m_sats_under_10m_sats",
        "short": "1M-10M sats",
        "long": "1M-10M Sats"
      },
      "_10m_sats_to_1btc": {
        "id": "above_10m_sats_under_1btc",
        "short": "0.1-1 BTC",
        "long": "0.1-1 BTC"
      },
      "_1btc_to_10btc": {
        "id": "above_1btc_under_10btc",
        "short": "1-10 BTC",
        "long": "1-10 BTC"
      },
      "_10btc_to_100btc": {
        "id": "above_10btc_under_100btc",
        "short": "10-100 BTC",
        "long": "10-100 BTC"
      },
      "_100btc_to_1k_btc": {
        "id": "above_100btc_under_1k_btc",
        "short": "100-1k BTC",
        "long": "100-1K BTC"
      },
      "_1k_btc_to_10k_btc": {
        "id": "above_1k_btc_under_10k_btc",
        "short": "1k-10k BTC",
        "long": "1K-10K BTC"
      },
      "_10k_btc_to_100k_btc": {
        "id": "above_10k_btc_under_100k_btc",
        "short": "10k-100k BTC",
        "long": "10K-100K BTC"
      },
      "_100k_btc_or_more": {
        "id": "above_100k_btc",
        "short": "100k+ BTC",
        "long": "100K+ BTC"
      }
    }

    GE_AMOUNT_NAMES = {
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

    LT_AMOUNT_NAMES = {
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

    def get_address_txs(self, address: Address, after_txid: Optional[str] = None, limit: Optional[float] = None) -> List[Txid]:
        """Address transaction IDs.

        Get transaction IDs for an address, newest first. Use after_txid for pagination.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*

        Endpoint: `GET /api/address/{address}/txs`"""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        path = f'/api/address/{address}/txs{"?" + query if query else ""}'
        return self.get_json(path)

    def get_address_confirmed_txs(self, address: Address, after_txid: Optional[str] = None, limit: Optional[float] = None) -> List[Txid]:
        """Address confirmed transactions.

        Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*

        Endpoint: `GET /api/address/{address}/txs/chain`"""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
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

    def get_metric_info(self, metric: Metric) -> List[Index]:
        """Get supported indexes for a metric.

        Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on day1, week1, and month1.

        Endpoint: `GET /api/metric/{metric}`"""
        return self.get_json(f'/api/metric/{metric}')

    def get_metric(self, metric: Metric, index: Index, start: Optional[float] = None, end: Optional[float] = None, limit: Optional[str] = None, format: Optional[Format] = None) -> Union[AnyMetricData, str]:
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

    def get_metrics_tree(self) -> TreeNode:
        """Metrics catalog.

        Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.

        Endpoint: `GET /api/metrics`"""
        return self.get_json('/api/metrics')

    def get_metrics(self, metrics: Metrics, index: Index, start: Optional[float] = None, end: Optional[float] = None, limit: Optional[str] = None, format: Optional[Format] = None) -> Union[List[AnyMetricData], str]:
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

    def list_metrics(self, page: Optional[float] = None) -> PaginatedMetrics:
        """Metrics list.

        Paginated flat list of all available metric names. Use `page` query param for pagination.

        Endpoint: `GET /api/metrics/list`"""
        params = []
        if page is not None: params.append(f'page={page}')
        query = '&'.join(params)
        path = f'/api/metrics/list{"?" + query if query else ""}'
        return self.get_json(path)

    def search_metrics(self, metric: Metric, limit: Optional[Limit] = None) -> List[Metric]:
        """Search metrics.

        Fuzzy search for metrics by name. Supports partial matches and typos.

        Endpoint: `GET /api/metrics/search/{metric}`"""
        params = []
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        path = f'/api/metrics/search/{metric}{"?" + query if query else ""}'
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

