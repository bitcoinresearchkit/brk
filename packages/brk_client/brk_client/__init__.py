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
# Raw cents squared (u128) - stores cents² × sats without division.
# Used for precise accumulation of investor cap values: Σ(price² × sats).
# investor_price = investor_cap_raw / realized_cap_raw
CentsSquaredSats = int
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
Hour1 = int
Hour12 = int
Hour4 = int
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
Minute1 = int
Minute10 = int
Minute30 = int
Minute5 = int
Month1 = int
Month3 = int
Month6 = int
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
Index = Literal["minute1", "minute5", "minute10", "minute30", "hour1", "hour4", "hour12", "day1", "day3", "week1", "month1", "month3", "month6", "year1", "year10", "halvingepoch", "difficultyepoch", "height", "txindex", "txinindex", "txoutindex", "emptyoutputindex", "opreturnindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "unknownoutputindex", "fundedaddressindex", "emptyaddressindex", "pairoutputindex"]
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
        start: Inclusive starting index, if negative counts from end
        end: Exclusive ending index, if negative counts from end
        limit: Maximum number of values to return (ignored if `end` is set)
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
        start: Inclusive starting index, if negative counts from end
        end: Exclusive ending index, if negative counts from end
        limit: Maximum number of values to return (ignored if `end` is set)
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
        start: Inclusive starting index, if negative counts from end
        end: Exclusive ending index, if negative counts from end
        limit: Maximum number of values to return (ignored if `end` is set)
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
    'minute1', 'minute5', 'minute10', 'minute30',
    'hour1', 'hour4', 'hour12',
    'day1', 'day3', 'week1',
    'month1', 'month3', 'month6',
    'year1', 'year10',
])

def _index_to_date(index: str, i: int) -> Union[date, datetime]:
    """Convert an index value to a date/datetime for date-based indexes."""
    if index == 'minute1':
        return _EPOCH + timedelta(minutes=i)
    elif index == 'minute5':
        return _EPOCH + timedelta(minutes=i * 5)
    elif index == 'minute10':
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
        return _EPOCH.date() + timedelta(days=i * 3)
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
    if index in ('minute1', 'minute5', 'minute10', 'minute30', 'hour1', 'hour4', 'hour12'):
        if isinstance(d, datetime):
            dt = d if d.tzinfo else d.replace(tzinfo=timezone.utc)
        else:
            dt = datetime(d.year, d.month, d.day, tzinfo=timezone.utc)
        secs = int((dt - _EPOCH).total_seconds())
        div = {'minute1': 60, 'minute5': 300, 'minute10': 600, 'minute30': 1800,
               'hour1': 3600, 'hour4': 14400, 'hour12': 43200}
        return secs // div[index]
    dd = d.date() if isinstance(d, datetime) else d
    if index == 'day1':
        if dd < _DAY_ONE:
            return 0
        return 1 + (dd - _DAY_ONE).days
    elif index == 'day3':
        return (dd - date(2009, 1, 1)).days // 3
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
_i1 = ('minute1', 'minute5', 'minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halvingepoch', 'difficultyepoch', 'height')
_i2 = ('minute1', 'minute5', 'minute10', 'minute30', 'hour1', 'hour4', 'hour12', 'day1', 'day3', 'week1', 'month1', 'month3', 'month6', 'year1', 'year10', 'halvingepoch', 'difficultyepoch')
_i3 = ('minute1',)
_i4 = ('minute5',)
_i5 = ('minute10',)
_i6 = ('minute30',)
_i7 = ('hour1',)
_i8 = ('hour4',)
_i9 = ('hour12',)
_i10 = ('day1',)
_i11 = ('day3',)
_i12 = ('week1',)
_i13 = ('month1',)
_i14 = ('month3',)
_i15 = ('month6',)
_i16 = ('year1',)
_i17 = ('year10',)
_i18 = ('halvingepoch',)
_i19 = ('difficultyepoch',)
_i20 = ('height',)
_i21 = ('txindex',)
_i22 = ('txinindex',)
_i23 = ('txoutindex',)
_i24 = ('emptyoutputindex',)
_i25 = ('opreturnindex',)
_i26 = ('p2aaddressindex',)
_i27 = ('p2msoutputindex',)
_i28 = ('p2pk33addressindex',)
_i29 = ('p2pk65addressindex',)
_i30 = ('p2pkhaddressindex',)
_i31 = ('p2shaddressindex',)
_i32 = ('p2traddressindex',)
_i33 = ('p2wpkhaddressindex',)
_i34 = ('p2wshaddressindex',)
_i35 = ('unknownoutputindex',)
_i36 = ('fundedaddressindex',)
_i37 = ('emptyaddressindex',)

def _ep(c: BrkClientBase, n: str, i: Index) -> MetricEndpointBuilder[Any]:
    return MetricEndpointBuilder(c, n, i)

def _dep(c: BrkClientBase, n: str, i: Index) -> DateMetricEndpointBuilder[Any]:
    return DateMetricEndpointBuilder(c, n, i)

# Index accessor classes

class _MetricPattern1By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute1')
    def minute5(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute5')
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
    def minute1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute1')
    def minute5(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute5')
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
    def minute1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute1')

class MetricPattern3(Generic[T]):
    by: _MetricPattern3By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern3By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i3)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i3 else None

class _MetricPattern4By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute5(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute5')

class MetricPattern4(Generic[T]):
    by: _MetricPattern4By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern4By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i4)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i4 else None

class _MetricPattern5By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute10')

class MetricPattern5(Generic[T]):
    by: _MetricPattern5By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern5By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i5)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i5 else None

class _MetricPattern6By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def minute30(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'minute30')

class MetricPattern6(Generic[T]):
    by: _MetricPattern6By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern6By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i6)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i6 else None

class _MetricPattern7By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour1')

class MetricPattern7(Generic[T]):
    by: _MetricPattern7By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern7By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i7)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i7 else None

class _MetricPattern8By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour4(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour4')

class MetricPattern8(Generic[T]):
    by: _MetricPattern8By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern8By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i8)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i8 else None

class _MetricPattern9By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def hour12(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'hour12')

class MetricPattern9(Generic[T]):
    by: _MetricPattern9By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern9By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i9)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i9 else None

class _MetricPattern10By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def day1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day1')

class MetricPattern10(Generic[T]):
    by: _MetricPattern10By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern10By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i10)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i10 else None

class _MetricPattern11By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def day3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'day3')

class MetricPattern11(Generic[T]):
    by: _MetricPattern11By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern11By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i11)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i11 else None

class _MetricPattern12By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def week1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'week1')

class MetricPattern12(Generic[T]):
    by: _MetricPattern12By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern12By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i12)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i12 else None

class _MetricPattern13By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month1')

class MetricPattern13(Generic[T]):
    by: _MetricPattern13By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern13By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i13)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i13 else None

class _MetricPattern14By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month3(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month3')

class MetricPattern14(Generic[T]):
    by: _MetricPattern14By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern14By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i14)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i14 else None

class _MetricPattern15By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def month6(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'month6')

class MetricPattern15(Generic[T]):
    by: _MetricPattern15By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern15By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i15)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i15 else None

class _MetricPattern16By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def year1(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year1')

class MetricPattern16(Generic[T]):
    by: _MetricPattern16By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern16By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i16)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i16 else None

class _MetricPattern17By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def year10(self) -> DateMetricEndpointBuilder[T]: return _dep(self._c, self._n, 'year10')

class MetricPattern17(Generic[T]):
    by: _MetricPattern17By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern17By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i17)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i17 else None

class _MetricPattern18By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def halvingepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halvingepoch')

class MetricPattern18(Generic[T]):
    by: _MetricPattern18By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern18By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i18)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i18 else None

class _MetricPattern19By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')

class MetricPattern19(Generic[T]):
    by: _MetricPattern19By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern19By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i19)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i19 else None

class _MetricPattern20By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')

class MetricPattern20(Generic[T]):
    by: _MetricPattern20By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern20By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i20)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i20 else None

class _MetricPattern21By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txindex')

class MetricPattern21(Generic[T]):
    by: _MetricPattern21By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern21By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i21)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i21 else None

class _MetricPattern22By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txinindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txinindex')

class MetricPattern22(Generic[T]):
    by: _MetricPattern22By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern22By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i22)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i22 else None

class _MetricPattern23By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txoutindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txoutindex')

class MetricPattern23(Generic[T]):
    by: _MetricPattern23By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern23By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i23)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i23 else None

class _MetricPattern24By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyoutputindex')

class MetricPattern24(Generic[T]):
    by: _MetricPattern24By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern24By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i24)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i24 else None

class _MetricPattern25By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def opreturnindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'opreturnindex')

class MetricPattern25(Generic[T]):
    by: _MetricPattern25By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern25By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i25)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i25 else None

class _MetricPattern26By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2aaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2aaddressindex')

class MetricPattern26(Generic[T]):
    by: _MetricPattern26By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern26By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i26)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i26 else None

class _MetricPattern27By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2msoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2msoutputindex')

class MetricPattern27(Generic[T]):
    by: _MetricPattern27By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern27By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i27)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i27 else None

class _MetricPattern28By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk33addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk33addressindex')

class MetricPattern28(Generic[T]):
    by: _MetricPattern28By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern28By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i28)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i28 else None

class _MetricPattern29By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk65addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk65addressindex')

class MetricPattern29(Generic[T]):
    by: _MetricPattern29By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern29By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i29)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i29 else None

class _MetricPattern30By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pkhaddressindex')

class MetricPattern30(Generic[T]):
    by: _MetricPattern30By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern30By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i30)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i30 else None

class _MetricPattern31By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2shaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2shaddressindex')

class MetricPattern31(Generic[T]):
    by: _MetricPattern31By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern31By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i31)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i31 else None

class _MetricPattern32By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2traddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2traddressindex')

class MetricPattern32(Generic[T]):
    by: _MetricPattern32By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern32By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i32)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i32 else None

class _MetricPattern33By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wpkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wpkhaddressindex')

class MetricPattern33(Generic[T]):
    by: _MetricPattern33By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern33By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i33)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i33 else None

class _MetricPattern34By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wshaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wshaddressindex')

class MetricPattern34(Generic[T]):
    by: _MetricPattern34By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern34By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i34)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i34 else None

class _MetricPattern35By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def unknownoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'unknownoutputindex')

class MetricPattern35(Generic[T]):
    by: _MetricPattern35By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern35By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i35)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i35 else None

class _MetricPattern36By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def fundedaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'fundedaddressindex')

class MetricPattern36(Generic[T]):
    by: _MetricPattern36By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern36By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i36)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i36 else None

class _MetricPattern37By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyaddressindex')

class MetricPattern37(Generic[T]):
    by: _MetricPattern37By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern37By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i37)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i37 else None

# Reusable structural pattern classes

class AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_1y'))
        self.adjusted_sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h'))
        self.adjusted_sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h_30d_ema'))
        self.adjusted_sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h_7d_ema'))
        self.adjusted_sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_30d'))
        self.adjusted_sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_7d'))
        self.adjusted_sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_1y'))
        self.adjusted_value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_24h'))
        self.adjusted_value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_30d'))
        self.adjusted_value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_7d'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.adjusted_value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_1y'))
        self.adjusted_value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_24h'))
        self.adjusted_value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_30d'))
        self.adjusted_value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_7d'))
        self.cap_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.investor_price_ratio_ext: RatioPattern3 = RatioPattern3(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.lower_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_1y'))
        self.realized_loss_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_24h'))
        self.realized_loss_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_30d'))
        self.realized_loss_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d'))
        self.realized_loss_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_price_ratio_ext: RatioPattern3 = RatioPattern3(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_1y'))
        self.realized_profit_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_24h'))
        self.realized_profit_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_30d'))
        self.realized_profit_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d'))
        self.realized_profit_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_1y'))
        self.realized_profit_to_loss_ratio_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_24h'))
        self.realized_profit_to_loss_ratio_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_30d'))
        self.realized_profit_to_loss_ratio_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_7d'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.realized_value_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_1y'))
        self.realized_value_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_24h'))
        self.realized_value_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_30d'))
        self.realized_value_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_7d'))
        self.sell_side_risk_ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_1y: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y'))
        self.sell_side_risk_ratio_24h: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h'))
        self.sell_side_risk_ratio_24h_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema'))
        self.sell_side_risk_ratio_24h_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema'))
        self.sell_side_risk_ratio_30d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr'))
        self.sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_1y'))
        self.sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h'))
        self.sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_30d_ema'))
        self.sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_7d_ema'))
        self.sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d'))
        self.sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d'))
        self.sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.upper_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'upper_price_band'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_1y'))
        self.value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_24h'))
        self.value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_30d'))
        self.value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_7d'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_1y'))
        self.value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_24h'))
        self.value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_30d'))
        self.value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_7d'))

class AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_1y'))
        self.adjusted_sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h'))
        self.adjusted_sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h_30d_ema'))
        self.adjusted_sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_24h_7d_ema'))
        self.adjusted_sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_30d'))
        self.adjusted_sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_7d'))
        self.adjusted_sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_1y'))
        self.adjusted_value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_24h'))
        self.adjusted_value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_30d'))
        self.adjusted_value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created_7d'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.adjusted_value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_1y'))
        self.adjusted_value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_24h'))
        self.adjusted_value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_30d'))
        self.adjusted_value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed_7d'))
        self.cap_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.lower_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_loss: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.realized_value_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_1y'))
        self.realized_value_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_24h'))
        self.realized_value_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_30d'))
        self.realized_value_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_7d'))
        self.sell_side_risk_ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_1y: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y'))
        self.sell_side_risk_ratio_24h: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h'))
        self.sell_side_risk_ratio_24h_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema'))
        self.sell_side_risk_ratio_24h_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema'))
        self.sell_side_risk_ratio_30d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr'))
        self.sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_1y'))
        self.sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h'))
        self.sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_30d_ema'))
        self.sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_7d_ema'))
        self.sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d'))
        self.sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d'))
        self.sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.upper_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'upper_price_band'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_1y'))
        self.value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_24h'))
        self.value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_30d'))
        self.value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_7d'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_1y'))
        self.value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_24h'))
        self.value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_30d'))
        self.value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_7d'))

class CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.investor_price_ratio_ext: RatioPattern3 = RatioPattern3(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.lower_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_1y'))
        self.realized_loss_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_24h'))
        self.realized_loss_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_30d'))
        self.realized_loss_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d'))
        self.realized_loss_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_price_ratio_ext: RatioPattern3 = RatioPattern3(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_1y'))
        self.realized_profit_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_24h'))
        self.realized_profit_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_30d'))
        self.realized_profit_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d'))
        self.realized_profit_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_1y'))
        self.realized_profit_to_loss_ratio_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_24h'))
        self.realized_profit_to_loss_ratio_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_30d'))
        self.realized_profit_to_loss_ratio_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'realized_profit_to_loss_ratio_7d'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.realized_value_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_1y'))
        self.realized_value_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_24h'))
        self.realized_value_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_30d'))
        self.realized_value_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_7d'))
        self.sell_side_risk_ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_1y: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y'))
        self.sell_side_risk_ratio_24h: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h'))
        self.sell_side_risk_ratio_24h_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema'))
        self.sell_side_risk_ratio_24h_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema'))
        self.sell_side_risk_ratio_30d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr'))
        self.sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_1y'))
        self.sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h'))
        self.sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_30d_ema'))
        self.sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_7d_ema'))
        self.sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d'))
        self.sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d'))
        self.sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.upper_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'upper_price_band'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_1y'))
        self.value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_24h'))
        self.value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_30d'))
        self.value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_7d'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_1y'))
        self.value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_24h'))
        self.value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_30d'))
        self.value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_7d'))

class CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.lower_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'lower_price_band'))
        self.mvrv: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[Cents] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_loss: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeHeightPattern[Dollars] = CumulativeHeightPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.realized_value_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_1y'))
        self.realized_value_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_24h'))
        self.realized_value_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_30d'))
        self.realized_value_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value_7d'))
        self.sell_side_risk_ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_1y: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_1y'))
        self.sell_side_risk_ratio_24h: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h'))
        self.sell_side_risk_ratio_24h_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_30d_ema'))
        self.sell_side_risk_ratio_24h_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_24h_7d_ema'))
        self.sell_side_risk_ratio_30d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr'))
        self.sopr_1y: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_1y'))
        self.sopr_24h: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h'))
        self.sopr_24h_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_30d_ema'))
        self.sopr_24h_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_24h_7d_ema'))
        self.sopr_30d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d'))
        self.sopr_30d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d'))
        self.sopr_7d_ema: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.upper_price_band: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'upper_price_band'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_created_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_1y'))
        self.value_created_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_24h'))
        self.value_created_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_30d'))
        self.value_created_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created_7d'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))
        self.value_destroyed_1y: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_1y'))
        self.value_destroyed_24h: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_24h'))
        self.value_destroyed_30d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_30d'))
        self.value_destroyed_7d: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed_7d'))

class _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._0sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, '0sd_usd'))
        self.m0_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm0_5sd'))
        self.m0_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm0_5sd_usd'))
        self.m1_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm1_5sd'))
        self.m1_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm1_5sd_usd'))
        self.m1sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm1sd'))
        self.m1sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm1sd_usd'))
        self.m2_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm2_5sd'))
        self.m2_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm2_5sd_usd'))
        self.m2sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm2sd'))
        self.m2sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm2sd_usd'))
        self.m3sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'm3sd'))
        self.m3sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'm3sd_usd'))
        self.p0_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p0_5sd'))
        self.p0_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p0_5sd_usd'))
        self.p1_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p1_5sd'))
        self.p1_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p1_5sd_usd'))
        self.p1sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p1sd'))
        self.p1sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p1sd_usd'))
        self.p2_5sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p2_5sd'))
        self.p2_5sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p2_5sd_usd'))
        self.p2sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p2sd'))
        self.p2sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p2sd_usd'))
        self.p3sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'p3sd'))
        self.p3sd_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'p3sd_usd'))
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sd'))
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sma'))
        self.zscore: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'zscore'))

class InvestedNegNetNuplSupplyUnrealizedPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct'))
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct'))
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap'))
        self.neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap'))
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap'))
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'))
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_loss_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap'))
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))
        self.unrealized_profit_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl'))

class PriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.price: SatsUsdPattern = SatsUsdPattern(client, acc)
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio'))
        self.ratio_1m_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_1m_sma'))
        self.ratio_1w_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_1w_sma'))
        self.ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_1y'))
        self.ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_2y'))
        self.ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_4y'))
        self.ratio_pct1: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct1'))
        self.ratio_pct1_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct1_usd'))
        self.ratio_pct2: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct2'))
        self.ratio_pct2_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct2_usd'))
        self.ratio_pct5: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct5'))
        self.ratio_pct5_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct5_usd'))
        self.ratio_pct95: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct95'))
        self.ratio_pct95_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct95_usd'))
        self.ratio_pct98: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct98'))
        self.ratio_pct98_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct98_usd'))
        self.ratio_pct99: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'ratio_pct99'))
        self.ratio_pct99_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'ratio_pct99_usd'))
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio'))

class Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.pct05: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct05'))
        self.pct10: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct10'))
        self.pct15: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct15'))
        self.pct20: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct20'))
        self.pct25: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct25'))
        self.pct30: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct30'))
        self.pct35: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct35'))
        self.pct40: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct40'))
        self.pct45: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct45'))
        self.pct50: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct50'))
        self.pct55: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct55'))
        self.pct60: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct60'))
        self.pct65: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct65'))
        self.pct70: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct70'))
        self.pct75: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct75'))
        self.pct80: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct80'))
        self.pct85: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct85'))
        self.pct90: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct90'))
        self.pct95: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct95'))

class RatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)
        self.ratio_1m_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1m_sma'))
        self.ratio_1w_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1w_sma'))
        self.ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y'))
        self.ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y'))
        self.ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y'))
        self.ratio_pct1: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct1'))
        self.ratio_pct1_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct1_usd'))
        self.ratio_pct2: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct2'))
        self.ratio_pct2_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct2_usd'))
        self.ratio_pct5: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct5'))
        self.ratio_pct5_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct5_usd'))
        self.ratio_pct95: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct95'))
        self.ratio_pct95_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct95_usd'))
        self.ratio_pct98: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct98'))
        self.ratio_pct98_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct98_usd'))
        self.ratio_pct99: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct99'))
        self.ratio_pct99_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct99_usd'))
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)

class RatioPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio_1m_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1m_sma'))
        self.ratio_1w_sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1w_sma'))
        self.ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y'))
        self.ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y'))
        self.ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y'))
        self.ratio_pct1: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct1'))
        self.ratio_pct1_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct1_usd'))
        self.ratio_pct2: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct2'))
        self.ratio_pct2_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct2_usd'))
        self.ratio_pct5: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct5'))
        self.ratio_pct5_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct5_usd'))
        self.ratio_pct95: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct95'))
        self.ratio_pct95_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct95_usd'))
        self.ratio_pct98: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct98'))
        self.ratio_pct98_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct98_usd'))
        self.ratio_pct99: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'pct99'))
        self.ratio_pct99_usd: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'pct99_usd'))
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)

class GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'greed_index'))
        self.invested_capital_in_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss'))
        self.invested_capital_in_loss_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit'))
        self.invested_capital_in_profit_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_sentiment: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_sentiment'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.pain_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'pain_index'))
        self.peak_regret: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_peak_regret'))
        self.supply_in_loss: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_profit'))

class Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.day1: MetricPattern10[T] = MetricPattern10(client, _m(acc, 'day1'))
        self.day3: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'day3'))
        self.difficultyepoch: MetricPattern19[T] = MetricPattern19(client, _m(acc, 'difficultyepoch'))
        self.halvingepoch: MetricPattern18[T] = MetricPattern18(client, _m(acc, 'halvingepoch'))
        self.hour1: MetricPattern7[T] = MetricPattern7(client, _m(acc, 'hour1'))
        self.hour12: MetricPattern9[T] = MetricPattern9(client, _m(acc, 'hour12'))
        self.hour4: MetricPattern8[T] = MetricPattern8(client, _m(acc, 'hour4'))
        self.minute1: MetricPattern3[T] = MetricPattern3(client, _m(acc, 'minute1'))
        self.minute10: MetricPattern5[T] = MetricPattern5(client, _m(acc, 'minute10'))
        self.minute30: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'minute30'))
        self.minute5: MetricPattern4[T] = MetricPattern4(client, _m(acc, 'minute5'))
        self.month1: MetricPattern13[T] = MetricPattern13(client, _m(acc, 'month1'))
        self.month3: MetricPattern14[T] = MetricPattern14(client, _m(acc, 'month3'))
        self.month6: MetricPattern15[T] = MetricPattern15(client, _m(acc, 'month6'))
        self.week1: MetricPattern12[T] = MetricPattern12(client, _m(acc, 'week1'))
        self.year1: MetricPattern16[T] = MetricPattern16(client, _m(acc, 'year1'))
        self.year10: MetricPattern17[T] = MetricPattern17(client, _m(acc, 'year10'))

class GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'greed_index'))
        self.invested_capital_in_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss'))
        self.invested_capital_in_loss_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit'))
        self.invested_capital_in_profit_raw: MetricPattern20[CentsSats] = MetricPattern20(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern20[CentsSquaredSats] = MetricPattern20(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_sentiment: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_sentiment'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.pain_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'pain_index'))
        self.supply_in_loss: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_profit'))

class BlocksCoinbaseDaysDominanceFeeSubsidyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.blocks_mined: CumulativeHeightSumPattern[StoredU32] = CumulativeHeightSumPattern(client, _m(acc, 'blocks_mined'))
        self.blocks_mined_1m_sum: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_mined_1m_sum'))
        self.blocks_mined_1w_sum: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_mined_1w_sum'))
        self.blocks_mined_1y_sum: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_mined_1y_sum'))
        self.blocks_mined_24h_sum: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_mined_24h_sum'))
        self.blocks_since_block: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_since_block'))
        self.coinbase: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, _m(acc, 'coinbase'))
        self.days_since_block: MetricPattern1[StoredU16] = MetricPattern1(client, _m(acc, 'days_since_block'))
        self.dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance'))
        self.dominance_1m: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance_1m'))
        self.dominance_1w: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance_1w'))
        self.dominance_1y: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance_1y'))
        self.dominance_24h: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance_24h'))
        self.fee: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, _m(acc, 'fee'))
        self.subsidy: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, _m(acc, 'subsidy'))

class InvestedNegNetNuplSupplyUnrealizedPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct'))
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct'))
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('10y', acc))
        self._1m: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('1m', acc))
        self._1w: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('1w', acc))
        self._1y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('1y', acc))
        self._2y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('2y', acc))
        self._3m: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('3m', acc))
        self._3y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('3y', acc))
        self._4y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('4y', acc))
        self._5y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('5y', acc))
        self._6m: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('6m', acc))
        self._6y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('6y', acc))
        self._8y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _p('8y', acc))

class InvestedNegNetNuplSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct'))
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct'))
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_market_cap'))
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_market_cap'))
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'nupl'))
        self.supply_in_loss_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_circulating_supply'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_circulating_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.supply_rel_to_circulating_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern1[T] = MetricPattern1(client, _p('10y', acc))
        self._1m: MetricPattern1[T] = MetricPattern1(client, _p('1m', acc))
        self._1w: MetricPattern1[T] = MetricPattern1(client, _p('1w', acc))
        self._1y: MetricPattern1[T] = MetricPattern1(client, _p('1y', acc))
        self._2y: MetricPattern1[T] = MetricPattern1(client, _p('2y', acc))
        self._3m: MetricPattern1[T] = MetricPattern1(client, _p('3m', acc))
        self._3y: MetricPattern1[T] = MetricPattern1(client, _p('3y', acc))
        self._4y: MetricPattern1[T] = MetricPattern1(client, _p('4y', acc))
        self._5y: MetricPattern1[T] = MetricPattern1(client, _p('5y', acc))
        self._6m: MetricPattern1[T] = MetricPattern1(client, _p('6m', acc))
        self._6y: MetricPattern1[T] = MetricPattern1(client, _p('6y', acc))
        self._8y: MetricPattern1[T] = MetricPattern1(client, _p('8y', acc))

class _201520162017201820192020202120222023202420252026Pattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._2015: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2015_returns'))
        self._2016: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2016_returns'))
        self._2017: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2017_returns'))
        self._2018: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2018_returns'))
        self._2019: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2019_returns'))
        self._2020: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2020_returns'))
        self._2021: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2021_returns'))
        self._2022: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2022_returns'))
        self._2023: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2023_returns'))
        self._2024: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2024_returns'))
        self._2025: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2025_returns'))
        self._2026: MetricPattern1[T] = MetricPattern1(client, _m(acc, '2026_returns'))

class AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'average'))
        self.cumulative: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'cumulative'))
        self.max: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'max'))
        self.median: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'median'))
        self.min: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'min'))
        self.pct10: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'pct90'))
        self.rolling: AverageMaxMedianMinP10P25P75P90SumPattern = AverageMaxMedianMinP10P25P75P90SumPattern(client, acc)
        self.sum: MetricPattern20[StoredU64] = MetricPattern20(client, _m(acc, 'sum'))

class AverageCumulativeMaxMedianMinP10P25P75P90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'average'))
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'max'))
        self.median: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'median'))
        self.min: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'min'))
        self.p10: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p10'))
        self.p25: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p25'))
        self.p75: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p75'))
        self.p90: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p90'))
        self.sum: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'sum'))

class AverageGainsLossesRsiStochPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'avg_gain_1y'))
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'avg_loss_1y'))
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'gains_1y'))
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'losses_1y'))
        self.rsi: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1y'))
        self.rsi_max: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'rsi_max_1y'))
        self.rsi_min: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'rsi_min_1y'))
        self.stoch_rsi: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'stoch_rsi_1y'))
        self.stoch_rsi_d: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'stoch_rsi_d_1y'))
        self.stoch_rsi_k: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'stoch_rsi_k_1y'))

class ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'addr_count'))
        self.addr_count_30d_change: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'addr_count_30d_change'))
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern = CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern = InvestedNegNetNuplSupplyUnrealizedPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.all: _30dCountPattern = _30dCountPattern(client, acc)
        self.p2a: _30dCountPattern = _30dCountPattern(client, _p('p2a', acc))
        self.p2pk33: _30dCountPattern = _30dCountPattern(client, _p('p2pk33', acc))
        self.p2pk65: _30dCountPattern = _30dCountPattern(client, _p('p2pk65', acc))
        self.p2pkh: _30dCountPattern = _30dCountPattern(client, _p('p2pkh', acc))
        self.p2sh: _30dCountPattern = _30dCountPattern(client, _p('p2sh', acc))
        self.p2tr: _30dCountPattern = _30dCountPattern(client, _p('p2tr', acc))
        self.p2wpkh: _30dCountPattern = _30dCountPattern(client, _p('p2wpkh', acc))
        self.p2wsh: _30dCountPattern = _30dCountPattern(client, _p('p2wsh', acc))

class AverageMaxMedianMinP10P25P75P90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'average'))
        self.max: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'max'))
        self.median: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'median'))
        self.min: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'min'))
        self.p10: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p10'))
        self.p25: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p25'))
        self.p75: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p75'))
        self.p90: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'p90'))
        self.sum: _1y24h30d7dPattern[StoredU64] = _1y24h30d7dPattern(client, _m(acc, 'sum'))

class AverageHeightMaxMedianMinP10P25P75P90Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'average'))
        self.height: MetricPattern20[T] = MetricPattern20(client, acc)
        self.max: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'max'))
        self.median: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'median'))
        self.min: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'min'))
        self.p10: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'p10'))
        self.p25: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'p25'))
        self.p75: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'p75'))
        self.p90: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, _m(acc, 'p90'))

class AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'average'))
        self.max: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'max'))
        self.median: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'median'))
        self.min: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'min'))
        self.pct10: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern20[T] = MetricPattern20(client, _m(acc, 'pct90'))

class _10y2y3y4y5y6y8yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('10y', acc))
        self._2y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('2y', acc))
        self._3y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('3y', acc))
        self._4y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('4y', acc))
        self._5y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('5y', acc))
        self._6y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('6y', acc))
        self._8y: MetricPattern1[StoredF32] = MetricPattern1(client, _p('8y', acc))

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 = CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern2 = InvestedNegNetNuplSupplyUnrealizedPattern2(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 = AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern4 = InvestedNegNetNuplSupplyUnrealizedPattern4(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern = CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern = InvestedNegNetNuplSupplyUnrealizedPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern = CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern4 = InvestedNegNetNuplSupplyUnrealizedPattern4(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class BalanceBothReactivatedReceivingSendingPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.balance_decreased: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'balance_decreased'))
        self.balance_increased: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'balance_increased'))
        self.both: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'both'))
        self.reactivated: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'reactivated'))
        self.receiving: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'receiving'))
        self.sending: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredU32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, _m(acc, 'sending'))

class CoinblocksCoindaysSatblocksSatdaysSentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: CumulativeHeightSumPattern[StoredF64] = CumulativeHeightSumPattern(client, _m(acc, 'coindays_destroyed'))
        self.satblocks_destroyed: MetricPattern20[Sats] = MetricPattern20(client, _m(acc, 'satblocks_destroyed'))
        self.satdays_destroyed: MetricPattern20[Sats] = MetricPattern20(client, _m(acc, 'satdays_destroyed'))
        self.sent: BtcSatsUsdPattern2 = BtcSatsUsdPattern2(client, _m(acc, 'sent'))
        self.sent_14d_ema: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'sent_14d_ema'))

class InvestedMaxMinPercentilesSpotPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital'))
        self.max: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'max_cost_basis'))
        self.min: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'min_cost_basis'))
        self.percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis'))
        self.spot_cost_basis_percentile: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'spot_cost_basis_percentile'))
        self.spot_invested_capital_percentile: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'spot_invested_capital_percentile'))

class CloseHighLowOpenPricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.close: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'close'))
        self.high: Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern[T] = Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, _m(acc, 'high'))
        self.low: Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern[T] = Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, _m(acc, 'low'))
        self.open: Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern[T] = Day1Day3DifficultyepochHalvingepochHour1Hour12Hour4Minute1Minute10Minute30Minute5Month1Month3Month6Week1Year1Year10Pattern(client, _m(acc, 'open'))
        self.price: MetricPattern20[T] = MetricPattern20(client, acc)

class _1y24h30d7dPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1y: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, '1y'))
        self._24h: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, '24h'))
        self._30d: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, '30d'))
        self._7d: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, '7d'))

class BtcRollingSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern20[Bitcoin] = MetricPattern20(client, _m(acc, 'btc'))
        self.rolling: _1y24h30d7dPattern2 = _1y24h30d7dPattern2(client, acc)
        self.sats: MetricPattern20[Sats] = MetricPattern20(client, acc)
        self.usd: MetricPattern20[Dollars] = MetricPattern20(client, _m(acc, 'usd'))

class _1h24hBlockTxindexPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '1h'))
        self._24h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, '24h'))
        self.block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern[T] = AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, acc)
        self.txindex: MetricPattern21[T] = MetricPattern21(client, acc)

class _1y24h30d7dPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1y: MetricPattern1[T] = MetricPattern1(client, _m(acc, '1y'))
        self._24h: MetricPattern1[T] = MetricPattern1(client, _m(acc, '24h'))
        self._30d: MetricPattern1[T] = MetricPattern1(client, _m(acc, '30d'))
        self._7d: MetricPattern1[T] = MetricPattern1(client, _m(acc, '7d'))

class _30dHalvedTotalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._30d_change: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, '_30d_change'))
        self.halved: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply_halved'))
        self.total: BtcSatsUsdPattern = BtcSatsUsdPattern(client, _m(acc, 'supply'))

class BtcSatsUsdPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.sats: CumulativeHeightPattern[Sats] = CumulativeHeightPattern(client, acc)
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class BtcSatsUsdPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.sats: CumulativeHeightRollingPattern[Sats] = CumulativeHeightRollingPattern(client, acc)
        self.usd: CumulativeHeightRollingPattern[Dollars] = CumulativeHeightRollingPattern(client, _m(acc, 'usd'))

class BtcSatsUsdPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.sats: CumulativeHeightSumPattern[Sats] = CumulativeHeightSumPattern(client, acc)
        self.usd: CumulativeHeightSumPattern[Dollars] = CumulativeHeightSumPattern(client, _m(acc, 'usd'))

class BtcSatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.btc: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, acc)
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))

class HistogramLineSignalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'histogram_1y'))
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'line_1y'))
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'signal_1y'))

class CumulativeHeightRollingPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern20[T] = MetricPattern20(client, acc)
        self.rolling: AverageMaxMedianMinP10P25P75P90SumPattern = AverageMaxMedianMinP10P25P75P90SumPattern(client, acc)

class CumulativeHeightSumPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern20[T] = MetricPattern20(client, acc)
        self.sum: _1y24h30d7dPattern[T] = _1y24h30d7dPattern(client, acc)

class _30dCountPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._30d_change: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, '30d_change'))
        self.count: MetricPattern1[StoredU64] = MetricPattern1(client, acc)

class BaseRestPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern20[StoredU64] = MetricPattern20(client, acc)
        self.rest: AverageCumulativeMaxMedianMinP10P25P75P90SumPattern = AverageCumulativeMaxMedianMinP10P25P75P90SumPattern(client, acc)

class MaxMinPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'max_cost_basis'))
        self.min: SatsUsdPattern = SatsUsdPattern(client, _m(acc, 'min_cost_basis'))

class SatsUsdPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))
        self.usd: MetricPattern1[Dollars] = MetricPattern1(client, acc)

class SdSmaPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sd'))
        self.sma: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'sma'))

class UtxoPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, acc)
        self.utxo_count_30d_change: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, '30d_change'))

class CumulativeHeightPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.height: MetricPattern20[T] = MetricPattern20(client, acc)

class RatioPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern1[StoredF32] = MetricPattern1(client, acc)

# Metrics tree classes

class MetricsTree_Blocks_Difficulty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty')
        self.as_hash: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_as_hash')
        self.adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_adjustment')
        self.epoch: MetricPattern1[DifficultyEpoch] = MetricPattern1(client, 'difficulty_epoch')
        self.blocks_before_next_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_difficulty_adjustment')
        self.days_before_next_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_difficulty_adjustment')

class MetricsTree_Blocks_Time_Timestamp:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern20[Timestamp] = MetricPattern20(client, 'timestamp')
        self.minute1: MetricPattern3[Timestamp] = MetricPattern3(client, 'timestamp_minute1')
        self.minute5: MetricPattern4[Timestamp] = MetricPattern4(client, 'timestamp_minute5')
        self.minute10: MetricPattern5[Timestamp] = MetricPattern5(client, 'timestamp_minute10')
        self.minute30: MetricPattern6[Timestamp] = MetricPattern6(client, 'timestamp_minute30')
        self.hour1: MetricPattern7[Timestamp] = MetricPattern7(client, 'timestamp_hour1')
        self.hour4: MetricPattern8[Timestamp] = MetricPattern8(client, 'timestamp_hour4')
        self.hour12: MetricPattern9[Timestamp] = MetricPattern9(client, 'timestamp_hour12')
        self.day1: MetricPattern10[Timestamp] = MetricPattern10(client, 'timestamp_day1')
        self.day3: MetricPattern11[Timestamp] = MetricPattern11(client, 'timestamp_day3')
        self.week1: MetricPattern12[Timestamp] = MetricPattern12(client, 'timestamp_week1')
        self.month1: MetricPattern13[Timestamp] = MetricPattern13(client, 'timestamp_month1')
        self.month3: MetricPattern14[Timestamp] = MetricPattern14(client, 'timestamp_month3')
        self.month6: MetricPattern15[Timestamp] = MetricPattern15(client, 'timestamp_month6')
        self.year1: MetricPattern16[Timestamp] = MetricPattern16(client, 'timestamp_year1')
        self.year10: MetricPattern17[Timestamp] = MetricPattern17(client, 'timestamp_year10')
        self.halvingepoch: MetricPattern18[Timestamp] = MetricPattern18(client, 'timestamp_halvingepoch')
        self.difficultyepoch: MetricPattern19[Timestamp] = MetricPattern19(client, 'timestamp_difficultyepoch')

class MetricsTree_Blocks_Time:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.timestamp: MetricsTree_Blocks_Time_Timestamp = MetricsTree_Blocks_Time_Timestamp(client)
        self.date: MetricPattern20[Date] = MetricPattern20(client, 'date')
        self.timestamp_monotonic: MetricPattern20[Timestamp] = MetricPattern20(client, 'timestamp_monotonic')

class MetricsTree_Blocks_Weight:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base: MetricPattern20[Weight] = MetricPattern20(client, 'block_weight')
        self.cumulative: MetricPattern1[Weight] = MetricPattern1(client, 'block_weight_cumulative')
        self.sum: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_sum')
        self.average: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_average')
        self.min: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_min')
        self.max: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_max')
        self.p10: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_p10')
        self.p25: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_p25')
        self.median: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_median')
        self.p75: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_p75')
        self.p90: _1y24h30d7dPattern[Weight] = _1y24h30d7dPattern(client, 'block_weight_p90')

class MetricsTree_Blocks_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_count_target: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_count_target')
        self.block_count: CumulativeHeightSumPattern[StoredU32] = CumulativeHeightSumPattern(client, 'block_count')
        self.block_count_sum: _1y24h30d7dPattern[StoredU32] = _1y24h30d7dPattern(client, 'block_count_sum')
        self.height_1h_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_1h_ago')
        self.height_24h_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_24h_ago')
        self.height_3d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_3d_ago')
        self.height_1w_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_1w_ago')
        self.height_8d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_8d_ago')
        self.height_9d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_9d_ago')
        self.height_12d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_12d_ago')
        self.height_13d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_13d_ago')
        self.height_2w_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_2w_ago')
        self.height_21d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_21d_ago')
        self.height_26d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_26d_ago')
        self.height_1m_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_1m_ago')
        self.height_34d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_34d_ago')
        self.height_55d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_55d_ago')
        self.height_2m_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_2m_ago')
        self.height_89d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_89d_ago')
        self.height_111d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_111d_ago')
        self.height_144d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_144d_ago')
        self.height_3m_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_3m_ago')
        self.height_6m_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_6m_ago')
        self.height_200d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_200d_ago')
        self.height_350d_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_350d_ago')
        self.height_1y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_1y_ago')
        self.height_2y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_2y_ago')
        self.height_200w_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_200w_ago')
        self.height_3y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_3y_ago')
        self.height_4y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_4y_ago')
        self.height_5y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_5y_ago')
        self.height_6y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_6y_ago')
        self.height_8y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_8y_ago')
        self.height_10y_ago: MetricPattern20[Height] = MetricPattern20(client, 'height_10y_ago')

class MetricsTree_Blocks_Halving:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.epoch: MetricPattern1[HalvingEpoch] = MetricPattern1(client, 'halving_epoch')
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_halving')

class MetricsTree_Blocks:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern20[BlockHash] = MetricPattern20(client, 'blockhash')
        self.difficulty: MetricsTree_Blocks_Difficulty = MetricsTree_Blocks_Difficulty(client)
        self.time: MetricsTree_Blocks_Time = MetricsTree_Blocks_Time(client)
        self.total_size: MetricPattern20[StoredU64] = MetricPattern20(client, 'total_size')
        self.weight: MetricsTree_Blocks_Weight = MetricsTree_Blocks_Weight(client)
        self.count: MetricsTree_Blocks_Count = MetricsTree_Blocks_Count(client)
        self.interval: AverageHeightMaxMedianMinP10P25P75P90Pattern[Timestamp] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'block_interval')
        self.halving: MetricsTree_Blocks_Halving = MetricsTree_Blocks_Halving(client)
        self.vbytes: CumulativeHeightRollingPattern[StoredU64] = CumulativeHeightRollingPattern(client, 'block_vbytes')
        self.size: AverageCumulativeMaxMedianMinP10P25P75P90SumPattern = AverageCumulativeMaxMedianMinP10P25P75P90SumPattern(client, 'block_size')
        self.fullness: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'block_fullness')

class MetricsTree_Transactions_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.tx_count: CumulativeHeightRollingPattern[StoredU64] = CumulativeHeightRollingPattern(client, 'tx_count')
        self.is_coinbase: MetricPattern21[StoredBool] = MetricPattern21(client, 'is_coinbase')

class MetricsTree_Transactions_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: _1h24hBlockTxindexPattern[VSize] = _1h24hBlockTxindexPattern(client, 'tx_vsize')
        self.weight: _1h24hBlockTxindexPattern[Weight] = _1h24hBlockTxindexPattern(client, 'tx_weight')

class MetricsTree_Transactions_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.input_value: MetricPattern21[Sats] = MetricPattern21(client, 'input_value')
        self.output_value: MetricPattern21[Sats] = MetricPattern21(client, 'output_value')
        self.fee: _1h24hBlockTxindexPattern[Sats] = _1h24hBlockTxindexPattern(client, 'fee')
        self.fee_rate: _1h24hBlockTxindexPattern[FeeRate] = _1h24hBlockTxindexPattern(client, 'fee_rate')

class MetricsTree_Transactions_Versions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v1')
        self.v2: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v2')
        self.v3: CumulativeHeightSumPattern[StoredU64] = CumulativeHeightSumPattern(client, 'tx_v3')

class MetricsTree_Transactions_Volume:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sent_sum: BtcRollingSatsUsdPattern = BtcRollingSatsUsdPattern(client, 'sent_sum')
        self.received_sum: BtcRollingSatsUsdPattern = BtcRollingSatsUsdPattern(client, 'received_sum')
        self.annualized_volume: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'annualized_volume')
        self.tx_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'tx_per_sec')
        self.outputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'outputs_per_sec')
        self.inputs_per_sec: MetricPattern1[StoredF32] = MetricPattern1(client, 'inputs_per_sec')

class MetricsTree_Transactions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txindex: MetricPattern20[TxIndex] = MetricPattern20(client, 'first_txindex')
        self.height: MetricPattern21[Height] = MetricPattern21(client, 'height')
        self.txid: MetricPattern21[Txid] = MetricPattern21(client, 'txid')
        self.txversion: MetricPattern21[TxVersion] = MetricPattern21(client, 'txversion')
        self.rawlocktime: MetricPattern21[RawLockTime] = MetricPattern21(client, 'rawlocktime')
        self.base_size: MetricPattern21[StoredU32] = MetricPattern21(client, 'base_size')
        self.total_size: MetricPattern21[StoredU32] = MetricPattern21(client, 'total_size')
        self.is_explicitly_rbf: MetricPattern21[StoredBool] = MetricPattern21(client, 'is_explicitly_rbf')
        self.first_txinindex: MetricPattern21[TxInIndex] = MetricPattern21(client, 'first_txinindex')
        self.first_txoutindex: MetricPattern21[TxOutIndex] = MetricPattern21(client, 'first_txoutindex')
        self.count: MetricsTree_Transactions_Count = MetricsTree_Transactions_Count(client)
        self.size: MetricsTree_Transactions_Size = MetricsTree_Transactions_Size(client)
        self.fees: MetricsTree_Transactions_Fees = MetricsTree_Transactions_Fees(client)
        self.versions: MetricsTree_Transactions_Versions = MetricsTree_Transactions_Versions(client)
        self.volume: MetricsTree_Transactions_Volume = MetricsTree_Transactions_Volume(client)

class MetricsTree_Inputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern22[TxOutIndex] = MetricPattern22(client, 'txoutindex')
        self.value: MetricPattern22[Sats] = MetricPattern22(client, 'value')

class MetricsTree_Inputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txinindex: MetricPattern20[TxInIndex] = MetricPattern20(client, 'first_txinindex')
        self.outpoint: MetricPattern22[OutPoint] = MetricPattern22(client, 'outpoint')
        self.txindex: MetricPattern22[TxIndex] = MetricPattern22(client, 'txindex')
        self.outputtype: MetricPattern22[OutputType] = MetricPattern22(client, 'outputtype')
        self.typeindex: MetricPattern22[TypeIndex] = MetricPattern22(client, 'typeindex')
        self.spent: MetricsTree_Inputs_Spent = MetricsTree_Inputs_Spent(client)
        self.count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'input_count')

class MetricsTree_Outputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern23[TxInIndex] = MetricPattern23(client, 'txinindex')

class MetricsTree_Outputs_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern(client, 'output_count')
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, 'exact_utxo_count')

class MetricsTree_Outputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txoutindex: MetricPattern20[TxOutIndex] = MetricPattern20(client, 'first_txoutindex')
        self.value: MetricPattern23[Sats] = MetricPattern23(client, 'value')
        self.outputtype: MetricPattern23[OutputType] = MetricPattern23(client, 'outputtype')
        self.typeindex: MetricPattern23[TypeIndex] = MetricPattern23(client, 'typeindex')
        self.txindex: MetricPattern23[TxIndex] = MetricPattern23(client, 'txindex')
        self.spent: MetricsTree_Outputs_Spent = MetricsTree_Outputs_Spent(client)
        self.count: MetricsTree_Outputs_Count = MetricsTree_Outputs_Count(client)

class MetricsTree_Addresses:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2pk65addressindex: MetricPattern20[P2PK65AddressIndex] = MetricPattern20(client, 'first_p2pk65addressindex')
        self.first_p2pk33addressindex: MetricPattern20[P2PK33AddressIndex] = MetricPattern20(client, 'first_p2pk33addressindex')
        self.first_p2pkhaddressindex: MetricPattern20[P2PKHAddressIndex] = MetricPattern20(client, 'first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern20[P2SHAddressIndex] = MetricPattern20(client, 'first_p2shaddressindex')
        self.first_p2wpkhaddressindex: MetricPattern20[P2WPKHAddressIndex] = MetricPattern20(client, 'first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern20[P2WSHAddressIndex] = MetricPattern20(client, 'first_p2wshaddressindex')
        self.first_p2traddressindex: MetricPattern20[P2TRAddressIndex] = MetricPattern20(client, 'first_p2traddressindex')
        self.first_p2aaddressindex: MetricPattern20[P2AAddressIndex] = MetricPattern20(client, 'first_p2aaddressindex')
        self.p2pk65bytes: MetricPattern29[P2PK65Bytes] = MetricPattern29(client, 'p2pk65bytes')
        self.p2pk33bytes: MetricPattern28[P2PK33Bytes] = MetricPattern28(client, 'p2pk33bytes')
        self.p2pkhbytes: MetricPattern30[P2PKHBytes] = MetricPattern30(client, 'p2pkhbytes')
        self.p2shbytes: MetricPattern31[P2SHBytes] = MetricPattern31(client, 'p2shbytes')
        self.p2wpkhbytes: MetricPattern33[P2WPKHBytes] = MetricPattern33(client, 'p2wpkhbytes')
        self.p2wshbytes: MetricPattern34[P2WSHBytes] = MetricPattern34(client, 'p2wshbytes')
        self.p2trbytes: MetricPattern32[P2TRBytes] = MetricPattern32(client, 'p2trbytes')
        self.p2abytes: MetricPattern26[P2ABytes] = MetricPattern26(client, 'p2abytes')

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
        self.taproot_adoption: MetricPattern1[StoredF32] = MetricPattern1(client, 'taproot_adoption')
        self.segwit_adoption: MetricPattern1[StoredF32] = MetricPattern1(client, 'segwit_adoption')

class MetricsTree_Scripts_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BtcSatsUsdPattern3 = BtcSatsUsdPattern3(client, 'opreturn_value')

class MetricsTree_Scripts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_emptyoutputindex: MetricPattern20[EmptyOutputIndex] = MetricPattern20(client, 'first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern20[OpReturnIndex] = MetricPattern20(client, 'first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern20[P2MSOutputIndex] = MetricPattern20(client, 'first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern20[UnknownOutputIndex] = MetricPattern20(client, 'first_unknownoutputindex')
        self.empty_to_txindex: MetricPattern24[TxIndex] = MetricPattern24(client, 'txindex')
        self.opreturn_to_txindex: MetricPattern25[TxIndex] = MetricPattern25(client, 'txindex')
        self.p2ms_to_txindex: MetricPattern27[TxIndex] = MetricPattern27(client, 'txindex')
        self.unknown_to_txindex: MetricPattern35[TxIndex] = MetricPattern35(client, 'txindex')
        self.count: MetricsTree_Scripts_Count = MetricsTree_Scripts_Count(client)
        self.value: MetricsTree_Scripts_Value = MetricsTree_Scripts_Value(client)

class MetricsTree_Mining_Rewards:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinbase: BtcSatsUsdPattern3 = BtcSatsUsdPattern3(client, 'coinbase')
        self.subsidy: BtcSatsUsdPattern3 = BtcSatsUsdPattern3(client, 'subsidy')
        self.fees: BtcSatsUsdPattern3 = BtcSatsUsdPattern3(client, 'fees')
        self.unclaimed_rewards: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, 'unclaimed_rewards')
        self.fee_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, 'fee_dominance')
        self.fee_dominance_24h: MetricPattern1[StoredF32] = MetricPattern1(client, 'fee_dominance_24h')
        self.fee_dominance_7d: MetricPattern1[StoredF32] = MetricPattern1(client, 'fee_dominance_7d')
        self.fee_dominance_30d: MetricPattern1[StoredF32] = MetricPattern1(client, 'fee_dominance_30d')
        self.fee_dominance_1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'fee_dominance_1y')
        self.subsidy_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, 'subsidy_dominance')
        self.subsidy_dominance_24h: MetricPattern1[StoredF32] = MetricPattern1(client, 'subsidy_dominance_24h')
        self.subsidy_dominance_7d: MetricPattern1[StoredF32] = MetricPattern1(client, 'subsidy_dominance_7d')
        self.subsidy_dominance_30d: MetricPattern1[StoredF32] = MetricPattern1(client, 'subsidy_dominance_30d')
        self.subsidy_dominance_1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'subsidy_dominance_1y')
        self.subsidy_usd_1y_sma: MetricPattern1[Dollars] = MetricPattern1(client, 'subsidy_usd_1y_sma')

class MetricsTree_Mining_Hashrate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.hash_rate: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate')
        self.hash_rate_1w_sma: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_1w_sma')
        self.hash_rate_1m_sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_rate_1m_sma')
        self.hash_rate_2m_sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_rate_2m_sma')
        self.hash_rate_1y_sma: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_rate_1y_sma')
        self.hash_rate_ath: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate_ath')
        self.hash_rate_drawdown: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_rate_drawdown')
        self.hash_price_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths')
        self.hash_price_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths_min')
        self.hash_price_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs')
        self.hash_price_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs_min')
        self.hash_price_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_rebound')
        self.hash_value_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths')
        self.hash_value_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths_min')
        self.hash_value_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs')
        self.hash_value_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs_min')
        self.hash_value_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_rebound')

class MetricsTree_Mining:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.rewards: MetricsTree_Mining_Rewards = MetricsTree_Mining_Rewards(client)
        self.hashrate: MetricsTree_Mining_Hashrate = MetricsTree_Mining_Hashrate(client)

class MetricsTree_Positions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_position: MetricPattern20[BlkPosition] = MetricPattern20(client, 'position')
        self.tx_position: MetricPattern21[BlkPosition] = MetricPattern21(client, 'position')

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
        self.vaulted_supply: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'vaulted_supply')
        self.active_supply: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'active_supply')

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
        self.thermo_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'thermo_cap')
        self.investor_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'investor_cap')
        self.vaulted_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'vaulted_cap')
        self.active_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'active_cap')
        self.cointime_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'cointime_cap')

class MetricsTree_Cointime_Pricing:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted_price: SatsUsdPattern = SatsUsdPattern(client, 'vaulted_price')
        self.vaulted_price_ratio: RatioPattern = RatioPattern(client, 'vaulted_price_ratio')
        self.active_price: SatsUsdPattern = SatsUsdPattern(client, 'active_price')
        self.active_price_ratio: RatioPattern = RatioPattern(client, 'active_price_ratio')
        self.true_market_mean: SatsUsdPattern = SatsUsdPattern(client, 'true_market_mean')
        self.true_market_mean_ratio: RatioPattern = RatioPattern(client, 'true_market_mean_ratio')
        self.cointime_price: SatsUsdPattern = SatsUsdPattern(client, 'cointime_price')
        self.cointime_price_ratio: RatioPattern = RatioPattern(client, 'cointime_price_ratio')

class MetricsTree_Cointime_Adjusted:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: MetricPattern1[StoredF32] = MetricPattern1(client, 'cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: MetricPattern1[StoredF64] = MetricPattern1(client, 'cointime_adj_tx_usd_velocity')

class MetricsTree_Cointime_ReserveRisk:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vocdd_365d_median: MetricPattern20[StoredF64] = MetricPattern20(client, 'vocdd_365d_median')
        self.hodl_bank: MetricPattern20[StoredF64] = MetricPattern20(client, 'hodl_bank')
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
        self.identity: MetricPattern28[P2PK33AddressIndex] = MetricPattern28(client, 'p2pk33addressindex')

class MetricsTree_Indexes_Address_P2pk65:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern29[P2PK65AddressIndex] = MetricPattern29(client, 'p2pk65addressindex')

class MetricsTree_Indexes_Address_P2pkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern30[P2PKHAddressIndex] = MetricPattern30(client, 'p2pkhaddressindex')

class MetricsTree_Indexes_Address_P2sh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern31[P2SHAddressIndex] = MetricPattern31(client, 'p2shaddressindex')

class MetricsTree_Indexes_Address_P2tr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern32[P2TRAddressIndex] = MetricPattern32(client, 'p2traddressindex')

class MetricsTree_Indexes_Address_P2wpkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern33[P2WPKHAddressIndex] = MetricPattern33(client, 'p2wpkhaddressindex')

class MetricsTree_Indexes_Address_P2wsh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern34[P2WSHAddressIndex] = MetricPattern34(client, 'p2wshaddressindex')

class MetricsTree_Indexes_Address_P2a:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern26[P2AAddressIndex] = MetricPattern26(client, 'p2aaddressindex')

class MetricsTree_Indexes_Address_P2ms:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern27[P2MSOutputIndex] = MetricPattern27(client, 'p2msoutputindex')

class MetricsTree_Indexes_Address_Empty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern24[EmptyOutputIndex] = MetricPattern24(client, 'emptyoutputindex')

class MetricsTree_Indexes_Address_Unknown:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern35[UnknownOutputIndex] = MetricPattern35(client, 'unknownoutputindex')

class MetricsTree_Indexes_Address_Opreturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern25[OpReturnIndex] = MetricPattern25(client, 'opreturnindex')

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
        self.identity: MetricPattern20[Height] = MetricPattern20(client, 'height')
        self.minute1: MetricPattern20[Minute1] = MetricPattern20(client, 'minute1')
        self.minute5: MetricPattern20[Minute5] = MetricPattern20(client, 'minute5')
        self.minute10: MetricPattern20[Minute10] = MetricPattern20(client, 'minute10')
        self.minute30: MetricPattern20[Minute30] = MetricPattern20(client, 'minute30')
        self.hour1: MetricPattern20[Hour1] = MetricPattern20(client, 'hour1')
        self.hour4: MetricPattern20[Hour4] = MetricPattern20(client, 'hour4')
        self.hour12: MetricPattern20[Hour12] = MetricPattern20(client, 'hour12')
        self.day1: MetricPattern20[Day1] = MetricPattern20(client, 'day1')
        self.day3: MetricPattern20[Day3] = MetricPattern20(client, 'day3')
        self.difficultyepoch: MetricPattern20[DifficultyEpoch] = MetricPattern20(client, 'difficultyepoch')
        self.halvingepoch: MetricPattern20[HalvingEpoch] = MetricPattern20(client, 'halvingepoch')
        self.week1: MetricPattern20[Week1] = MetricPattern20(client, 'week1')
        self.month1: MetricPattern20[Month1] = MetricPattern20(client, 'month1')
        self.month3: MetricPattern20[Month3] = MetricPattern20(client, 'month3')
        self.month6: MetricPattern20[Month6] = MetricPattern20(client, 'month6')
        self.year1: MetricPattern20[Year1] = MetricPattern20(client, 'year1')
        self.year10: MetricPattern20[Year10] = MetricPattern20(client, 'year10')
        self.txindex_count: MetricPattern20[StoredU64] = MetricPattern20(client, 'txindex_count')

class MetricsTree_Indexes_Difficultyepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern19[DifficultyEpoch] = MetricPattern19(client, 'difficultyepoch')
        self.first_height: MetricPattern19[Height] = MetricPattern19(client, 'first_height')
        self.height_count: MetricPattern19[StoredU64] = MetricPattern19(client, 'height_count')

class MetricsTree_Indexes_Halvingepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern18[HalvingEpoch] = MetricPattern18(client, 'halvingepoch')
        self.first_height: MetricPattern18[Height] = MetricPattern18(client, 'first_height')

class MetricsTree_Indexes_Minute1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern3[Minute1] = MetricPattern3(client, 'minute1')
        self.first_height: MetricPattern3[Height] = MetricPattern3(client, 'minute1_first_height')

class MetricsTree_Indexes_Minute5:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern4[Minute5] = MetricPattern4(client, 'minute5')
        self.first_height: MetricPattern4[Height] = MetricPattern4(client, 'minute5_first_height')

class MetricsTree_Indexes_Minute10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern5[Minute10] = MetricPattern5(client, 'minute10')
        self.first_height: MetricPattern5[Height] = MetricPattern5(client, 'minute10_first_height')

class MetricsTree_Indexes_Minute30:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern6[Minute30] = MetricPattern6(client, 'minute30')
        self.first_height: MetricPattern6[Height] = MetricPattern6(client, 'minute30_first_height')

class MetricsTree_Indexes_Hour1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern7[Hour1] = MetricPattern7(client, 'hour1')
        self.first_height: MetricPattern7[Height] = MetricPattern7(client, 'hour1_first_height')

class MetricsTree_Indexes_Hour4:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern8[Hour4] = MetricPattern8(client, 'hour4')
        self.first_height: MetricPattern8[Height] = MetricPattern8(client, 'hour4_first_height')

class MetricsTree_Indexes_Hour12:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern9[Hour12] = MetricPattern9(client, 'hour12')
        self.first_height: MetricPattern9[Height] = MetricPattern9(client, 'hour12_first_height')

class MetricsTree_Indexes_Day1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern10[Day1] = MetricPattern10(client, 'day1')
        self.date: MetricPattern10[Date] = MetricPattern10(client, 'date')
        self.first_height: MetricPattern10[Height] = MetricPattern10(client, 'first_height')
        self.height_count: MetricPattern10[StoredU64] = MetricPattern10(client, 'height_count')

class MetricsTree_Indexes_Day3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern11[Day3] = MetricPattern11(client, 'day3')
        self.first_height: MetricPattern11[Height] = MetricPattern11(client, 'day3_first_height')

class MetricsTree_Indexes_Week1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern12[Week1] = MetricPattern12(client, 'week1')
        self.date: MetricPattern12[Date] = MetricPattern12(client, 'date')
        self.first_height: MetricPattern12[Height] = MetricPattern12(client, 'week1_first_height')

class MetricsTree_Indexes_Month1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern13[Month1] = MetricPattern13(client, 'month1')
        self.date: MetricPattern13[Date] = MetricPattern13(client, 'date')
        self.first_height: MetricPattern13[Height] = MetricPattern13(client, 'month1_first_height')

class MetricsTree_Indexes_Month3:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern14[Month3] = MetricPattern14(client, 'month3')
        self.date: MetricPattern14[Date] = MetricPattern14(client, 'date')
        self.first_height: MetricPattern14[Height] = MetricPattern14(client, 'month3_first_height')

class MetricsTree_Indexes_Month6:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern15[Month6] = MetricPattern15(client, 'month6')
        self.date: MetricPattern15[Date] = MetricPattern15(client, 'date')
        self.first_height: MetricPattern15[Height] = MetricPattern15(client, 'month6_first_height')

class MetricsTree_Indexes_Year1:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern16[Year1] = MetricPattern16(client, 'year1')
        self.date: MetricPattern16[Date] = MetricPattern16(client, 'date')
        self.first_height: MetricPattern16[Height] = MetricPattern16(client, 'year1_first_height')

class MetricsTree_Indexes_Year10:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern17[Year10] = MetricPattern17(client, 'year10')
        self.date: MetricPattern17[Date] = MetricPattern17(client, 'date')
        self.first_height: MetricPattern17[Height] = MetricPattern17(client, 'year10_first_height')

class MetricsTree_Indexes_Txindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern21[TxIndex] = MetricPattern21(client, 'txindex')
        self.input_count: MetricPattern21[StoredU64] = MetricPattern21(client, 'input_count')
        self.output_count: MetricPattern21[StoredU64] = MetricPattern21(client, 'output_count')

class MetricsTree_Indexes_Txinindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern22[TxInIndex] = MetricPattern22(client, 'txinindex')

class MetricsTree_Indexes_Txoutindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern23[TxOutIndex] = MetricPattern23(client, 'txoutindex')

class MetricsTree_Indexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: MetricsTree_Indexes_Address = MetricsTree_Indexes_Address(client)
        self.height: MetricsTree_Indexes_Height = MetricsTree_Indexes_Height(client)
        self.difficultyepoch: MetricsTree_Indexes_Difficultyepoch = MetricsTree_Indexes_Difficultyepoch(client)
        self.halvingepoch: MetricsTree_Indexes_Halvingepoch = MetricsTree_Indexes_Halvingepoch(client)
        self.minute1: MetricsTree_Indexes_Minute1 = MetricsTree_Indexes_Minute1(client)
        self.minute5: MetricsTree_Indexes_Minute5 = MetricsTree_Indexes_Minute5(client)
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
        self.price_ath: SatsUsdPattern = SatsUsdPattern(client, 'price_ath')
        self.price_drawdown: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_drawdown')
        self.days_since_price_ath: MetricPattern1[StoredU16] = MetricPattern1(client, 'days_since_price_ath')
        self.years_since_price_ath: MetricPattern2[StoredF32] = MetricPattern2(client, 'years_since_price_ath')
        self.max_days_between_price_aths: MetricPattern1[StoredU16] = MetricPattern1(client, 'max_days_between_price_aths')
        self.max_years_between_price_aths: MetricPattern2[StoredF32] = MetricPattern2(client, 'max_years_between_price_aths')

class MetricsTree_Market_Lookback:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: SatsUsdPattern = SatsUsdPattern(client, 'price_1d_ago')
        self._1w: SatsUsdPattern = SatsUsdPattern(client, 'price_1w_ago')
        self._1m: SatsUsdPattern = SatsUsdPattern(client, 'price_1m_ago')
        self._3m: SatsUsdPattern = SatsUsdPattern(client, 'price_3m_ago')
        self._6m: SatsUsdPattern = SatsUsdPattern(client, 'price_6m_ago')
        self._1y: SatsUsdPattern = SatsUsdPattern(client, 'price_1y_ago')
        self._2y: SatsUsdPattern = SatsUsdPattern(client, 'price_2y_ago')
        self._3y: SatsUsdPattern = SatsUsdPattern(client, 'price_3y_ago')
        self._4y: SatsUsdPattern = SatsUsdPattern(client, 'price_4y_ago')
        self._5y: SatsUsdPattern = SatsUsdPattern(client, 'price_5y_ago')
        self._6y: SatsUsdPattern = SatsUsdPattern(client, 'price_6y_ago')
        self._8y: SatsUsdPattern = SatsUsdPattern(client, 'price_8y_ago')
        self._10y: SatsUsdPattern = SatsUsdPattern(client, 'price_10y_ago')

class MetricsTree_Market_Returns_PriceReturns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: MetricPattern1[StoredF32] = MetricPattern1(client, '1d_price_returns')
        self._1w: MetricPattern1[StoredF32] = MetricPattern1(client, '1w_price_returns')
        self._1m: MetricPattern1[StoredF32] = MetricPattern1(client, '1m_price_returns')
        self._3m: MetricPattern1[StoredF32] = MetricPattern1(client, '3m_price_returns')
        self._6m: MetricPattern1[StoredF32] = MetricPattern1(client, '6m_price_returns')
        self._1y: MetricPattern1[StoredF32] = MetricPattern1(client, '1y_price_returns')
        self._2y: MetricPattern1[StoredF32] = MetricPattern1(client, '2y_price_returns')
        self._3y: MetricPattern1[StoredF32] = MetricPattern1(client, '3y_price_returns')
        self._4y: MetricPattern1[StoredF32] = MetricPattern1(client, '4y_price_returns')
        self._5y: MetricPattern1[StoredF32] = MetricPattern1(client, '5y_price_returns')
        self._6y: MetricPattern1[StoredF32] = MetricPattern1(client, '6y_price_returns')
        self._8y: MetricPattern1[StoredF32] = MetricPattern1(client, '8y_price_returns')
        self._10y: MetricPattern1[StoredF32] = MetricPattern1(client, '10y_price_returns')

class MetricsTree_Market_Returns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_returns: MetricsTree_Market_Returns_PriceReturns = MetricsTree_Market_Returns_PriceReturns(client)
        self.cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'cagr')
        self._1d_returns_1w_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1w_sd')
        self._1d_returns_1m_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1m_sd')
        self._1d_returns_1y_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1y_sd')
        self.downside_returns: MetricPattern20[StoredF32] = MetricPattern20(client, 'downside_returns')
        self.downside_1w_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1w_sd')
        self.downside_1m_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1m_sd')
        self.downside_1y_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1y_sd')

class MetricsTree_Market_Volatility:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1w_volatility: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_1w_volatility')
        self.price_1m_volatility: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_1m_volatility')
        self.price_1y_volatility: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_1y_volatility')
        self.sharpe_1w: MetricPattern1[StoredF32] = MetricPattern1(client, 'sharpe_1w')
        self.sharpe_1m: MetricPattern1[StoredF32] = MetricPattern1(client, 'sharpe_1m')
        self.sharpe_1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'sharpe_1y')
        self.sortino_1w: MetricPattern1[StoredF32] = MetricPattern1(client, 'sortino_1w')
        self.sortino_1m: MetricPattern1[StoredF32] = MetricPattern1(client, 'sortino_1m')
        self.sortino_1y: MetricPattern1[StoredF32] = MetricPattern1(client, 'sortino_1y')

class MetricsTree_Market_Range:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1w_min: SatsUsdPattern = SatsUsdPattern(client, 'price_1w_min')
        self.price_1w_max: SatsUsdPattern = SatsUsdPattern(client, 'price_1w_max')
        self.price_2w_min: SatsUsdPattern = SatsUsdPattern(client, 'price_2w_min')
        self.price_2w_max: SatsUsdPattern = SatsUsdPattern(client, 'price_2w_max')
        self.price_1m_min: SatsUsdPattern = SatsUsdPattern(client, 'price_1m_min')
        self.price_1m_max: SatsUsdPattern = SatsUsdPattern(client, 'price_1m_max')
        self.price_1y_min: SatsUsdPattern = SatsUsdPattern(client, 'price_1y_min')
        self.price_1y_max: SatsUsdPattern = SatsUsdPattern(client, 'price_1y_max')
        self.price_true_range: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range')
        self.price_true_range_2w_sum: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_true_range_2w_sum')
        self.price_2w_choppiness_index: MetricPattern1[StoredF32] = MetricPattern1(client, 'price_2w_choppiness_index')

class MetricsTree_Market_MovingAverage:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1w_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_1w_sma')
        self.price_8d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_8d_sma')
        self.price_13d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_13d_sma')
        self.price_21d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_21d_sma')
        self.price_1m_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_1m_sma')
        self.price_34d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_34d_sma')
        self.price_55d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_55d_sma')
        self.price_89d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_89d_sma')
        self.price_111d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_111d_sma')
        self.price_144d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_144d_sma')
        self.price_200d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_200d_sma')
        self.price_350d_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_350d_sma')
        self.price_1y_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_1y_sma')
        self.price_2y_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_2y_sma')
        self.price_200w_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_200w_sma')
        self.price_4y_sma: PriceRatioPattern = PriceRatioPattern(client, 'price_4y_sma')
        self.price_1w_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_1w_ema')
        self.price_8d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_8d_ema')
        self.price_12d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_12d_ema')
        self.price_13d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_13d_ema')
        self.price_21d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_21d_ema')
        self.price_26d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_26d_ema')
        self.price_1m_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_1m_ema')
        self.price_34d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_34d_ema')
        self.price_55d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_55d_ema')
        self.price_89d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_89d_ema')
        self.price_144d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_144d_ema')
        self.price_200d_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_200d_ema')
        self.price_1y_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_1y_ema')
        self.price_2y_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_2y_ema')
        self.price_200w_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_200w_ema')
        self.price_4y_ema: PriceRatioPattern = PriceRatioPattern(client, 'price_4y_ema')
        self.price_200d_sma_x2_4: SatsUsdPattern = SatsUsdPattern(client, 'price_200d_sma_x2_4')
        self.price_200d_sma_x0_8: SatsUsdPattern = SatsUsdPattern(client, 'price_200d_sma_x0_8')
        self.price_350d_sma_x2: SatsUsdPattern = SatsUsdPattern(client, 'price_350d_sma_x2')

class MetricsTree_Market_Dca_PeriodAveragePrice:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: SatsUsdPattern = SatsUsdPattern(client, '1w_dca_average_price')
        self._1m: SatsUsdPattern = SatsUsdPattern(client, '1m_dca_average_price')
        self._3m: SatsUsdPattern = SatsUsdPattern(client, '3m_dca_average_price')
        self._6m: SatsUsdPattern = SatsUsdPattern(client, '6m_dca_average_price')
        self._1y: SatsUsdPattern = SatsUsdPattern(client, '1y_dca_average_price')
        self._2y: SatsUsdPattern = SatsUsdPattern(client, '2y_dca_average_price')
        self._3y: SatsUsdPattern = SatsUsdPattern(client, '3y_dca_average_price')
        self._4y: SatsUsdPattern = SatsUsdPattern(client, '4y_dca_average_price')
        self._5y: SatsUsdPattern = SatsUsdPattern(client, '5y_dca_average_price')
        self._6y: SatsUsdPattern = SatsUsdPattern(client, '6y_dca_average_price')
        self._8y: SatsUsdPattern = SatsUsdPattern(client, '8y_dca_average_price')
        self._10y: SatsUsdPattern = SatsUsdPattern(client, '10y_dca_average_price')

class MetricsTree_Market_Dca_ClassStack:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2015_stack')
        self._2016: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2016_stack')
        self._2017: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2017_stack')
        self._2018: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2018_stack')
        self._2019: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2019_stack')
        self._2020: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2020_stack')
        self._2021: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2021_stack')
        self._2022: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2022_stack')
        self._2023: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2023_stack')
        self._2024: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2024_stack')
        self._2025: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2025_stack')
        self._2026: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'dca_class_2026_stack')

class MetricsTree_Market_Dca_ClassAveragePrice:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2015_average_price')
        self._2016: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2016_average_price')
        self._2017: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2017_average_price')
        self._2018: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2018_average_price')
        self._2019: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2019_average_price')
        self._2020: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2020_average_price')
        self._2021: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2021_average_price')
        self._2022: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2022_average_price')
        self._2023: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2023_average_price')
        self._2024: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2024_average_price')
        self._2025: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2025_average_price')
        self._2026: SatsUsdPattern = SatsUsdPattern(client, 'dca_class_2026_average_price')

class MetricsTree_Market_Dca_ClassDaysInProfit:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2015_days_in_profit')
        self._2016: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2016_days_in_profit')
        self._2017: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2017_days_in_profit')
        self._2018: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2018_days_in_profit')
        self._2019: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2019_days_in_profit')
        self._2020: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2020_days_in_profit')
        self._2021: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2021_days_in_profit')
        self._2022: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2022_days_in_profit')
        self._2023: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2023_days_in_profit')
        self._2024: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2024_days_in_profit')
        self._2025: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2025_days_in_profit')
        self._2026: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2026_days_in_profit')

class MetricsTree_Market_Dca_ClassDaysInLoss:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2015_days_in_loss')
        self._2016: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2016_days_in_loss')
        self._2017: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2017_days_in_loss')
        self._2018: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2018_days_in_loss')
        self._2019: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2019_days_in_loss')
        self._2020: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2020_days_in_loss')
        self._2021: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2021_days_in_loss')
        self._2022: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2022_days_in_loss')
        self._2023: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2023_days_in_loss')
        self._2024: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2024_days_in_loss')
        self._2025: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2025_days_in_loss')
        self._2026: MetricPattern1[StoredU32] = MetricPattern1(client, 'dca_class_2026_days_in_loss')

class MetricsTree_Market_Dca_ClassMinReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2015_min_return')
        self._2016: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2016_min_return')
        self._2017: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2017_min_return')
        self._2018: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2018_min_return')
        self._2019: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2019_min_return')
        self._2020: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2020_min_return')
        self._2021: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2021_min_return')
        self._2022: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2022_min_return')
        self._2023: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2023_min_return')
        self._2024: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2024_min_return')
        self._2025: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2025_min_return')
        self._2026: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2026_min_return')

class MetricsTree_Market_Dca_ClassMaxReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2015_max_return')
        self._2016: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2016_max_return')
        self._2017: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2017_max_return')
        self._2018: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2018_max_return')
        self._2019: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2019_max_return')
        self._2020: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2020_max_return')
        self._2021: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2021_max_return')
        self._2022: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2022_max_return')
        self._2023: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2023_max_return')
        self._2024: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2024_max_return')
        self._2025: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2025_max_return')
        self._2026: MetricPattern1[StoredF32] = MetricPattern1(client, 'dca_class_2026_max_return')

class MetricsTree_Market_Dca:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dca_sats_per_day: MetricPattern20[Sats] = MetricPattern20(client, 'dca_sats_per_day')
        self.period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'dca_stack')
        self.period_average_price: MetricsTree_Market_Dca_PeriodAveragePrice = MetricsTree_Market_Dca_PeriodAveragePrice(client)
        self.period_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_returns')
        self.period_cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'dca_cagr')
        self.period_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredU32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_days_in_profit')
        self.period_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredU32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_days_in_loss')
        self.period_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_min_return')
        self.period_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'dca_max_return')
        self.period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 = _10y1m1w1y2y3m3y4y5y6m6y8yPattern3(client, 'lump_sum_stack')
        self.period_lump_sum_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_returns')
        self.period_lump_sum_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredU32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_days_in_profit')
        self.period_lump_sum_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredU32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_days_in_loss')
        self.period_lump_sum_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_min_return')
        self.period_lump_sum_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2[StoredF32] = _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(client, 'lump_sum_max_return')
        self.class_stack: MetricsTree_Market_Dca_ClassStack = MetricsTree_Market_Dca_ClassStack(client)
        self.class_average_price: MetricsTree_Market_Dca_ClassAveragePrice = MetricsTree_Market_Dca_ClassAveragePrice(client)
        self.class_returns: _201520162017201820192020202120222023202420252026Pattern2[StoredF32] = _201520162017201820192020202120222023202420252026Pattern2(client, 'dca_class')
        self.class_days_in_profit: MetricsTree_Market_Dca_ClassDaysInProfit = MetricsTree_Market_Dca_ClassDaysInProfit(client)
        self.class_days_in_loss: MetricsTree_Market_Dca_ClassDaysInLoss = MetricsTree_Market_Dca_ClassDaysInLoss(client)
        self.class_min_return: MetricsTree_Market_Dca_ClassMinReturn = MetricsTree_Market_Dca_ClassMinReturn(client)
        self.class_max_return: MetricsTree_Market_Dca_ClassMaxReturn = MetricsTree_Market_Dca_ClassMaxReturn(client)

class MetricsTree_Market_Indicators_Rsi_1d:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1d')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1d')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_gain_1d')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_loss_1d')
        self.rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_1d')
        self.rsi_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_min_1d')
        self.rsi_max: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_max_1d')
        self.stoch_rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_1d')
        self.stoch_rsi_k: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_k_1d')
        self.stoch_rsi_d: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_d_1d')

class MetricsTree_Market_Indicators_Rsi_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1w')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1w')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_gain_1w')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_loss_1w')
        self.rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_1w')
        self.rsi_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_min_1w')
        self.rsi_max: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_max_1w')
        self.stoch_rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_1w')
        self.stoch_rsi_k: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_k_1w')
        self.stoch_rsi_d: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_d_1w')

class MetricsTree_Market_Indicators_Rsi_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gains: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_gains_1m')
        self.losses: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_losses_1m')
        self.average_gain: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_gain_1m')
        self.average_loss: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_avg_loss_1m')
        self.rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_1m')
        self.rsi_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_min_1m')
        self.rsi_max: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_rsi_max_1m')
        self.stoch_rsi: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_1m')
        self.stoch_rsi_k: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_k_1m')
        self.stoch_rsi_d: MetricPattern1[StoredF32] = MetricPattern1(client, 'rsi_stoch_rsi_d_1m')

class MetricsTree_Market_Indicators_Rsi:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: MetricsTree_Market_Indicators_Rsi_1d = MetricsTree_Market_Indicators_Rsi_1d(client)
        self._1w: MetricsTree_Market_Indicators_Rsi_1w = MetricsTree_Market_Indicators_Rsi_1w(client)
        self._1m: MetricsTree_Market_Indicators_Rsi_1m = MetricsTree_Market_Indicators_Rsi_1m(client)
        self._1y: AverageGainsLossesRsiStochPattern = AverageGainsLossesRsiStochPattern(client, 'rsi')

class MetricsTree_Market_Indicators_Macd_1d:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1d')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1d')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1d')

class MetricsTree_Market_Indicators_Macd_1w:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1w')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1w')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1w')

class MetricsTree_Market_Indicators_Macd_1m:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.line: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_line_1m')
        self.signal: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_signal_1m')
        self.histogram: MetricPattern1[StoredF32] = MetricPattern1(client, 'macd_histogram_1m')

class MetricsTree_Market_Indicators_Macd:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: MetricsTree_Market_Indicators_Macd_1d = MetricsTree_Market_Indicators_Macd_1d(client)
        self._1w: MetricsTree_Market_Indicators_Macd_1w = MetricsTree_Market_Indicators_Macd_1w(client)
        self._1m: MetricsTree_Market_Indicators_Macd_1m = MetricsTree_Market_Indicators_Macd_1m(client)
        self._1y: HistogramLineSignalPattern = HistogramLineSignalPattern(client, 'macd')

class MetricsTree_Market_Indicators:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.puell_multiple: MetricPattern1[StoredF32] = MetricPattern1(client, 'puell_multiple')
        self.nvt: MetricPattern1[StoredF32] = MetricPattern1(client, 'nvt')
        self.rsi: MetricsTree_Market_Indicators_Rsi = MetricsTree_Market_Indicators_Rsi(client)
        self.stoch_k: MetricPattern1[StoredF32] = MetricPattern1(client, 'stoch_k')
        self.stoch_d: MetricPattern1[StoredF32] = MetricPattern1(client, 'stoch_d')
        self.pi_cycle: MetricPattern1[StoredF32] = MetricPattern1(client, 'pi_cycle')
        self.macd: MetricsTree_Market_Indicators_Macd = MetricsTree_Market_Indicators_Macd(client)
        self.gini: MetricPattern1[StoredF32] = MetricPattern1(client, 'gini')

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

class MetricsTree_Pools_Vecs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.unknown: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'unknown')
        self.blockfills: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'blockfills')
        self.ultimuspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ultimuspool')
        self.terrapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'terrapool')
        self.luxor: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'luxor')
        self.onethash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onethash')
        self.btccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btccom')
        self.bitfarms: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfarms')
        self.huobipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'huobipool')
        self.wayicn: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'wayicn')
        self.canoepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'canoepool')
        self.btctop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btctop')
        self.bitcoincom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoincom')
        self.pool175btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pool175btc')
        self.gbminers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'gbminers')
        self.axbt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'axbt')
        self.asicminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'asicminer')
        self.bitminter: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitminter')
        self.bitcoinrussia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinrussia')
        self.btcserv: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcserv')
        self.simplecoinus: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'simplecoinus')
        self.btcguild: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcguild')
        self.eligius: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eligius')
        self.ozcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ozcoin')
        self.eclipsemc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eclipsemc')
        self.maxbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'maxbtc')
        self.triplemining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'triplemining')
        self.coinlab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'coinlab')
        self.pool50btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pool50btc')
        self.ghashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ghashio')
        self.stminingcorp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'stminingcorp')
        self.bitparking: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitparking')
        self.mmpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mmpool')
        self.polmine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'polmine')
        self.kncminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kncminer')
        self.bitalo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitalo')
        self.f2pool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'f2pool')
        self.hhtt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hhtt')
        self.megabigpower: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'megabigpower')
        self.mtred: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mtred')
        self.nmcbit: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nmcbit')
        self.yourbtcnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'yourbtcnet')
        self.givemecoins: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'givemecoins')
        self.braiinspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'braiinspool')
        self.antpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'antpool')
        self.multicoinco: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'multicoinco')
        self.bcpoolio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bcpoolio')
        self.cointerra: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'cointerra')
        self.kanopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kanopool')
        self.solock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'solock')
        self.ckpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ckpool')
        self.nicehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nicehash')
        self.bitclub: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitclub')
        self.bitcoinaffiliatenetwork: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinaffiliatenetwork')
        self.btcc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcc')
        self.bwpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bwpool')
        self.exxbw: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'exxbw')
        self.bitsolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitsolo')
        self.bitfury: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfury')
        self.twentyoneinc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'twentyoneinc')
        self.digitalbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'digitalbtc')
        self.eightbaochi: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eightbaochi')
        self.mybtccoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mybtccoinpool')
        self.tbdice: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tbdice')
        self.hashpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hashpool')
        self.nexious: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nexious')
        self.bravomining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bravomining')
        self.hotpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hotpool')
        self.okexpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okexpool')
        self.bcmonster: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bcmonster')
        self.onehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onehash')
        self.bixin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bixin')
        self.tatmaspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tatmaspool')
        self.viabtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'viabtc')
        self.connectbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'connectbtc')
        self.batpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'batpool')
        self.waterhole: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'waterhole')
        self.dcexploration: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dcexploration')
        self.dcex: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dcex')
        self.btpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btpool')
        self.fiftyeightcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'fiftyeightcoin')
        self.bitcoinindia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinindia')
        self.shawnp0wers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'shawnp0wers')
        self.phashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'phashio')
        self.rigpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'rigpool')
        self.haozhuzhu: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'haozhuzhu')
        self.sevenpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sevenpool')
        self.miningkings: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningkings')
        self.hashbx: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hashbx')
        self.dpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dpool')
        self.rawpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'rawpool')
        self.haominer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'haominer')
        self.helix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'helix')
        self.bitcoinukraine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinukraine')
        self.poolin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'poolin')
        self.secretsuperstar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'secretsuperstar')
        self.tigerpoolnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tigerpoolnet')
        self.sigmapoolcom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sigmapoolcom')
        self.okpooltop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okpooltop')
        self.hummerpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hummerpool')
        self.tangpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tangpool')
        self.bytepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bytepool')
        self.spiderpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'spiderpool')
        self.novablock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'novablock')
        self.miningcity: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningcity')
        self.binancepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'binancepool')
        self.minerium: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'minerium')
        self.lubiancom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'lubiancom')
        self.okkong: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okkong')
        self.aaopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'aaopool')
        self.emcdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'emcdpool')
        self.foundryusa: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'foundryusa')
        self.sbicrypto: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sbicrypto')
        self.arkpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'arkpool')
        self.purebtccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'purebtccom')
        self.marapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'marapool')
        self.kucoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kucoinpool')
        self.entrustcharitypool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'entrustcharitypool')
        self.okminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okminer')
        self.titan: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'titan')
        self.pegapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pegapool')
        self.btcnuggets: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcnuggets')
        self.cloudhashing: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'cloudhashing')
        self.digitalxmintsy: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'digitalxmintsy')
        self.telco214: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'telco214')
        self.btcpoolparty: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcpoolparty')
        self.multipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'multipool')
        self.transactioncoinmining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'transactioncoinmining')
        self.btcdig: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcdig')
        self.trickysbtcpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'trickysbtcpool')
        self.btcmp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcmp')
        self.eobot: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eobot')
        self.unomp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'unomp')
        self.patels: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'patels')
        self.gogreenlight: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'gogreenlight')
        self.bitcoinindiapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinindiapool')
        self.ekanembtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ekanembtc')
        self.canoe: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'canoe')
        self.tiger: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tiger')
        self.onem1x: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onem1x')
        self.zulupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'zulupool')
        self.secpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'secpool')
        self.ocean: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ocean')
        self.whitepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'whitepool')
        self.wiz: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'wiz')
        self.wk057: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'wk057')
        self.futurebitapollosolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'futurebitapollosolo')
        self.carbonnegative: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'carbonnegative')
        self.portlandhodl: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'portlandhodl')
        self.phoenix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'phoenix')
        self.neopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'neopool')
        self.maxipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'maxipool')
        self.bitfufupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfufupool')
        self.gdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'gdpool')
        self.miningdutch: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningdutch')
        self.publicpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'publicpool')
        self.miningsquared: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningsquared')
        self.innopolistech: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'innopolistech')
        self.btclab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btclab')
        self.parasite: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'parasite')
        self.redrockpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'redrockpool')
        self.est3lar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern = BlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'est3lar')

class MetricsTree_Pools:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.height_to_pool: MetricPattern20[PoolSlug] = MetricPattern20(client, 'pool')
        self.vecs: MetricsTree_Pools_Vecs = MetricsTree_Pools_Vecs(client)

class MetricsTree_Prices:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: CloseHighLowOpenPricePattern[Cents] = CloseHighLowOpenPricePattern(client, 'price_cents')
        self.usd: CloseHighLowOpenPricePattern[Dollars] = CloseHighLowOpenPricePattern(client, 'price_usd')
        self.sats: CloseHighLowOpenPricePattern[Sats] = CloseHighLowOpenPricePattern(client, 'price_sats')

class MetricsTree_Distribution_AnyAddressIndexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern26[AnyAddressIndex] = MetricPattern26(client, 'anyaddressindex')
        self.p2pk33: MetricPattern28[AnyAddressIndex] = MetricPattern28(client, 'anyaddressindex')
        self.p2pk65: MetricPattern29[AnyAddressIndex] = MetricPattern29(client, 'anyaddressindex')
        self.p2pkh: MetricPattern30[AnyAddressIndex] = MetricPattern30(client, 'anyaddressindex')
        self.p2sh: MetricPattern31[AnyAddressIndex] = MetricPattern31(client, 'anyaddressindex')
        self.p2tr: MetricPattern32[AnyAddressIndex] = MetricPattern32(client, 'anyaddressindex')
        self.p2wpkh: MetricPattern33[AnyAddressIndex] = MetricPattern33(client, 'anyaddressindex')
        self.p2wsh: MetricPattern34[AnyAddressIndex] = MetricPattern34(client, 'anyaddressindex')

class MetricsTree_Distribution_AddressesData:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.funded: MetricPattern36[FundedAddressData] = MetricPattern36(client, 'fundedaddressdata')
        self.empty: MetricPattern37[EmptyAddressData] = MetricPattern37(client, 'emptyaddressdata')

class MetricsTree_Distribution_UtxoCohorts_All_Relative:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, 'supply_in_profit_rel_to_own_supply')
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, 'supply_in_loss_rel_to_own_supply')
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_profit_rel_to_market_cap')
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_loss_rel_to_market_cap')
        self.neg_unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, 'neg_unrealized_loss_rel_to_market_cap')
        self.net_unrealized_pnl_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, 'net_unrealized_pnl_rel_to_market_cap')
        self.nupl: MetricPattern1[StoredF32] = MetricPattern1(client, 'nupl')
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, 'invested_capital_in_profit_pct')
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, 'invested_capital_in_loss_pct')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_profit_rel_to_own_total_unrealized_pnl')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_peak_regret_rel_to_market_cap')

class MetricsTree_Distribution_UtxoCohorts_All:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, '')
        self.outputs: UtxoPattern = UtxoPattern(client, 'utxo_count')
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, '')
        self.realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern = AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, '')
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, '')
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, '')
        self.relative: MetricsTree_Distribution_UtxoCohorts_All_Relative = MetricsTree_Distribution_UtxoCohorts_All_Relative(client)

class MetricsTree_Distribution_UtxoCohorts_Sth:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, 'sth')
        self.outputs: UtxoPattern = UtxoPattern(client, 'sth_utxo_count')
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, 'sth')
        self.realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern = AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern(client, 'sth')
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, 'sth')
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, 'sth')
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern2 = InvestedNegNetNuplSupplyUnrealizedPattern2(client, 'sth')

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
        self._1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1w_old')
        self._1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1m_old')
        self._2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_2m_old')
        self._3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_3m_old')
        self._4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_4m_old')
        self._5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_5m_old')
        self._6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_6m_old')
        self._1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1y_old')
        self._2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_2y_old')
        self._3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_3y_old')
        self._4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_4y_old')
        self._5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_5y_old')
        self._6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_6y_old')
        self._7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_7y_old')
        self._8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_8y_old')
        self._10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10y_old')
        self._12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_12y_old')
        self._15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_15y_old')

class MetricsTree_Distribution_UtxoCohorts_MinAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_1d_old')
        self._1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_1w_old')
        self._1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_1m_old')
        self._2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_2m_old')
        self._3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_3m_old')
        self._4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_4m_old')
        self._5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_5m_old')
        self._6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_6m_old')
        self._1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_1y_old')
        self._2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_2y_old')
        self._3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_3y_old')
        self._4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_4y_old')
        self._5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_5y_old')
        self._6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_6y_old')
        self._7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_7y_old')
        self._8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_8y_old')
        self._10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_10y_old')
        self._12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_over_12y_old')

class MetricsTree_Distribution_UtxoCohorts_GeAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_1sat')
        self._10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_10sats')
        self._100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_100sats')
        self._1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_1k_sats')
        self._10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_10k_sats')
        self._100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_100k_sats')
        self._1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_1m_sats')
        self._10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_10m_sats')
        self._1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_1btc')
        self._10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_10btc')
        self._100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_100btc')
        self._1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_1k_btc')
        self._10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_over_10k_btc')

class MetricsTree_Distribution_UtxoCohorts_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_with_0sats')
        self._1sat_to_10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_1sat_under_10sats')
        self._10sats_to_100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_10sats_under_100sats')
        self._100sats_to_1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_100sats_under_1k_sats')
        self._1k_sats_to_10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_1k_sats_under_10k_sats')
        self._10k_sats_to_100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_10k_sats_under_100k_sats')
        self._100k_sats_to_1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_100k_sats_under_1m_sats')
        self._1m_sats_to_10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_1m_sats_under_10m_sats')
        self._10m_sats_to_1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_10m_sats_under_1btc')
        self._1btc_to_10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_1btc_under_10btc')
        self._10btc_to_100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_10btc_under_100btc')
        self._100btc_to_1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_100btc_under_1k_btc')
        self._1k_btc_to_10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_1k_btc_under_10k_btc')
        self._10k_btc_to_100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_10k_btc_under_100k_btc')
        self._100k_btc_or_more: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_above_100k_btc')

class MetricsTree_Distribution_UtxoCohorts_LtAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_10sats')
        self._100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_100sats')
        self._1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_1k_sats')
        self._10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_10k_sats')
        self._100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_100k_sats')
        self._1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_1m_sats')
        self._10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_10m_sats')
        self._1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_1btc')
        self._10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_10btc')
        self._100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_100btc')
        self._1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_1k_btc')
        self._10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_10k_btc')
        self._100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'utxos_under_100k_btc')

class MetricsTree_Distribution_UtxoCohorts_Epoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'epoch_0')
        self._1: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'epoch_1')
        self._2: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'epoch_2')
        self._3: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'epoch_3')
        self._4: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'epoch_4')

class MetricsTree_Distribution_UtxoCohorts_Year:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2009: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2009')
        self._2010: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2010')
        self._2011: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2011')
        self._2012: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2012')
        self._2013: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2013')
        self._2014: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2014')
        self._2015: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2015')
        self._2016: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2016')
        self._2017: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2017')
        self._2018: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2018')
        self._2019: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2019')
        self._2020: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2020')
        self._2021: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2021')
        self._2022: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2022')
        self._2023: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2023')
        self._2024: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2024')
        self._2025: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2025')
        self._2026: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'year_2026')

class MetricsTree_Distribution_UtxoCohorts_Type:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pk65')
        self.p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pk33')
        self.p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pkh')
        self.p2ms: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2ms')
        self.p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2sh')
        self.p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2wpkh')
        self.p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2wsh')
        self.p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2tr')
        self.p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2a')
        self.unknown: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'unknown_outputs')
        self.empty: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'empty_outputs')

class MetricsTree_Distribution_UtxoCohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricsTree_Distribution_UtxoCohorts_All = MetricsTree_Distribution_UtxoCohorts_All(client)
        self.sth: MetricsTree_Distribution_UtxoCohorts_Sth = MetricsTree_Distribution_UtxoCohorts_Sth(client)
        self.lth: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'lth')
        self.age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange = MetricsTree_Distribution_UtxoCohorts_AgeRange(client)
        self.max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge = MetricsTree_Distribution_UtxoCohorts_MaxAge(client)
        self.min_age: MetricsTree_Distribution_UtxoCohorts_MinAge = MetricsTree_Distribution_UtxoCohorts_MinAge(client)
        self.ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount = MetricsTree_Distribution_UtxoCohorts_GeAmount(client)
        self.amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange = MetricsTree_Distribution_UtxoCohorts_AmountRange(client)
        self.lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount = MetricsTree_Distribution_UtxoCohorts_LtAmount(client)
        self.epoch: MetricsTree_Distribution_UtxoCohorts_Epoch = MetricsTree_Distribution_UtxoCohorts_Epoch(client)
        self.year: MetricsTree_Distribution_UtxoCohorts_Year = MetricsTree_Distribution_UtxoCohorts_Year(client)
        self.type_: MetricsTree_Distribution_UtxoCohorts_Type = MetricsTree_Distribution_UtxoCohorts_Type(client)

class MetricsTree_Distribution_AddressCohorts_GeAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_1sat')
        self._10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_10sats')
        self._100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_100sats')
        self._1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_1k_sats')
        self._10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_10k_sats')
        self._100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_100k_sats')
        self._1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_1m_sats')
        self._10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_10m_sats')
        self._1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_1btc')
        self._10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_10btc')
        self._100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_100btc')
        self._1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_1k_btc')
        self._10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_over_10k_btc')

class MetricsTree_Distribution_AddressCohorts_AmountRange:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_with_0sats')
        self._1sat_to_10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_1sat_under_10sats')
        self._10sats_to_100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_10sats_under_100sats')
        self._100sats_to_1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_100sats_under_1k_sats')
        self._1k_sats_to_10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_1k_sats_under_10k_sats')
        self._10k_sats_to_100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_10k_sats_under_100k_sats')
        self._100k_sats_to_1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_100k_sats_under_1m_sats')
        self._1m_sats_to_10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_1m_sats_under_10m_sats')
        self._10m_sats_to_1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_10m_sats_under_1btc')
        self._1btc_to_10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_1btc_under_10btc')
        self._10btc_to_100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_10btc_under_100btc')
        self._100btc_to_1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_100btc_under_1k_btc')
        self._1k_btc_to_10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_1k_btc_under_10k_btc')
        self._10k_btc_to_100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_10k_btc_under_100k_btc')
        self._100k_btc_or_more: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_above_100k_btc')

class MetricsTree_Distribution_AddressCohorts_LtAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_10sats')
        self._100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_100sats')
        self._1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_1k_sats')
        self._10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_10k_sats')
        self._100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_100k_sats')
        self._1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_1m_sats')
        self._10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_10m_sats')
        self._1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_1btc')
        self._10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_10btc')
        self._100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_100btc')
        self._1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_1k_btc')
        self._10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_10k_btc')
        self._100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern = ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern(client, 'addrs_under_100k_btc')

class MetricsTree_Distribution_AddressCohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount = MetricsTree_Distribution_AddressCohorts_GeAmount(client)
        self.amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange = MetricsTree_Distribution_AddressCohorts_AmountRange(client)
        self.lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount = MetricsTree_Distribution_AddressCohorts_LtAmount(client)

class MetricsTree_Distribution_AddressActivity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'address_activity')
        self.p2pk65: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2pk65_address_activity')
        self.p2pk33: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2pk33_address_activity')
        self.p2pkh: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2pkh_address_activity')
        self.p2sh: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2sh_address_activity')
        self.p2wpkh: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2wpkh_address_activity')
        self.p2wsh: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2wsh_address_activity')
        self.p2tr: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2tr_address_activity')
        self.p2a: BalanceBothReactivatedReceivingSendingPattern = BalanceBothReactivatedReceivingSendingPattern(client, 'p2a_address_activity')

class MetricsTree_Distribution_TotalAddrCount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, 'total_addr_count')
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk65_total_addr_count')
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk33_total_addr_count')
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pkh_total_addr_count')
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2sh_total_addr_count')
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wpkh_total_addr_count')
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wsh_total_addr_count')
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2tr_total_addr_count')
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2a_total_addr_count')

class MetricsTree_Distribution_NewAddrCount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: BaseRestPattern = BaseRestPattern(client, 'new_addr_count')
        self.p2pk65: BaseRestPattern = BaseRestPattern(client, 'p2pk65_new_addr_count')
        self.p2pk33: BaseRestPattern = BaseRestPattern(client, 'p2pk33_new_addr_count')
        self.p2pkh: BaseRestPattern = BaseRestPattern(client, 'p2pkh_new_addr_count')
        self.p2sh: BaseRestPattern = BaseRestPattern(client, 'p2sh_new_addr_count')
        self.p2wpkh: BaseRestPattern = BaseRestPattern(client, 'p2wpkh_new_addr_count')
        self.p2wsh: BaseRestPattern = BaseRestPattern(client, 'p2wsh_new_addr_count')
        self.p2tr: BaseRestPattern = BaseRestPattern(client, 'p2tr_new_addr_count')
        self.p2a: BaseRestPattern = BaseRestPattern(client, 'p2a_new_addr_count')

class MetricsTree_Distribution_GrowthRate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'growth_rate')
        self.p2pk65: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2pk65_growth_rate')
        self.p2pk33: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2pk33_growth_rate')
        self.p2pkh: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2pkh_growth_rate')
        self.p2sh: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2sh_growth_rate')
        self.p2wpkh: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2wpkh_growth_rate')
        self.p2wsh: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2wsh_growth_rate')
        self.p2tr: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2tr_growth_rate')
        self.p2a: AverageHeightMaxMedianMinP10P25P75P90Pattern[StoredF32] = AverageHeightMaxMedianMinP10P25P75P90Pattern(client, 'p2a_growth_rate')

class MetricsTree_Distribution:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_state: MetricPattern20[SupplyState] = MetricPattern20(client, 'supply_state')
        self.any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes = MetricsTree_Distribution_AnyAddressIndexes(client)
        self.addresses_data: MetricsTree_Distribution_AddressesData = MetricsTree_Distribution_AddressesData(client)
        self.utxo_cohorts: MetricsTree_Distribution_UtxoCohorts = MetricsTree_Distribution_UtxoCohorts(client)
        self.address_cohorts: MetricsTree_Distribution_AddressCohorts = MetricsTree_Distribution_AddressCohorts(client)
        self.addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'addr_count')
        self.empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'empty_addr_count')
        self.address_activity: MetricsTree_Distribution_AddressActivity = MetricsTree_Distribution_AddressActivity(client)
        self.total_addr_count: MetricsTree_Distribution_TotalAddrCount = MetricsTree_Distribution_TotalAddrCount(client)
        self.new_addr_count: MetricsTree_Distribution_NewAddrCount = MetricsTree_Distribution_NewAddrCount(client)
        self.growth_rate: MetricsTree_Distribution_GrowthRate = MetricsTree_Distribution_GrowthRate(client)
        self.fundedaddressindex: MetricPattern36[FundedAddressIndex] = MetricPattern36(client, 'fundedaddressindex')
        self.emptyaddressindex: MetricPattern37[EmptyAddressIndex] = MetricPattern37(client, 'emptyaddressindex')

class MetricsTree_Supply_Burned:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, 'opreturn_supply')
        self.unspendable: BtcSatsUsdPattern4 = BtcSatsUsdPattern4(client, 'unspendable_supply')

class MetricsTree_Supply_Velocity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern1[StoredF64] = MetricPattern1(client, 'btc_velocity')
        self.usd: MetricPattern1[StoredF64] = MetricPattern1(client, 'usd_velocity')

class MetricsTree_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.circulating: BtcSatsUsdPattern = BtcSatsUsdPattern(client, 'circulating_supply')
        self.burned: MetricsTree_Supply_Burned = MetricsTree_Supply_Burned(client)
        self.inflation: MetricPattern1[StoredF32] = MetricPattern1(client, 'inflation_rate')
        self.velocity: MetricsTree_Supply_Velocity = MetricsTree_Supply_Velocity(client)
        self.market_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'market_cap')
        self.market_cap_growth_rate: MetricPattern1[StoredF32] = MetricPattern1(client, 'market_cap_growth_rate')
        self.realized_cap_growth_rate: MetricPattern1[StoredF32] = MetricPattern1(client, 'realized_cap_growth_rate')
        self.cap_growth_rate_diff: MetricPattern1[StoredF32] = MetricPattern1(client, 'cap_growth_rate_diff')

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

    YEAR_NAMES = {
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

