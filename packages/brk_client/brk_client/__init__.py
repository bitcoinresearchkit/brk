# Auto-generated BRK Python client
# Do not edit manually

from __future__ import annotations
from dataclasses import dataclass
from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Union, Protocol, overload, Iterator, Tuple, TYPE_CHECKING
from http.client import HTTPSConnection, HTTPConnection
from urllib.parse import urlparse
from datetime import date, timedelta
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
# Unified index for any address type (loaded or empty)
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
# Cents × Sats (u128) - price in cents multiplied by amount in sats.
# Uses u128 because large amounts at any price can overflow u64.
CentsSats = int
# Raw cents squared (u128) - stores cents² × sats without division.
# Used for precise accumulation of investor cap values: Σ(price² × sats).
# investor_price = investor_cap_raw / realized_cap_raw
CentsSquaredSats = int
# Unsigned cents (u64) - for values that should never be negative.
# Used for invested capital, realized cap, etc.
CentsUnsigned = int
# Closing price value for a time period
Close = CentsUnsigned
# Output format for API responses
Format = Literal["json", "csv"]
# Maximum number of results to return. Defaults to 100 if not specified.
Limit = int
# Date in YYYYMMDD format stored as u32
Date = int
DateIndex = int
DecadeIndex = int
DifficultyEpoch = int
# US Dollar amount as floating point
Dollars = float
EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
# Fee rate in sats/vB
FeeRate = float
HalvingEpoch = int
# Hex-encoded string
Hex = str
# Highest price value for a time period
High = CentsUnsigned
LoadedAddressIndex = TypeIndex
# Lowest price value for a time period
Low = CentsUnsigned
# Virtual size in vbytes (weight / 4, rounded up)
VSize = int
# Metric name
Metric = str
# Comma-separated list of metric names
Metrics = str
MonthIndex = int
# Opening price value for a time period
Open = CentsUnsigned
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
PoolSlug = Literal["unknown", "blockfills", "ultimuspool", "terrapool", "luxor", "onethash", "btccom", "bitfarms", "huobipool", "wayicn", "canoepool", "btctop", "bitcoincom", "pool175btc", "gbminers", "axbt", "asicminer", "bitminter", "bitcoinrussia", "btcserv", "simplecoinus", "btcguild", "eligius", "ozcoin", "eclipsemc", "maxbtc", "triplemining", "coinlab", "pool50btc", "ghashio", "stminingcorp", "bitparking", "mmpool", "polmine", "kncminer", "bitalo", "f2pool", "hhtt", "megabigpower", "mtred", "nmcbit", "yourbtcnet", "givemecoins", "braiinspool", "antpool", "multicoinco", "bcpoolio", "cointerra", "kanopool", "solock", "ckpool", "nicehash", "bitclub", "bitcoinaffiliatenetwork", "btcc", "bwpool", "exxbw", "bitsolo", "bitfury", "twentyoneinc", "digitalbtc", "eightbaochi", "mybtccoinpool", "tbdice", "hashpool", "nexious", "bravomining", "hotpool", "okexpool", "bcmonster", "onehash", "bixin", "tatmaspool", "viabtc", "connectbtc", "batpool", "waterhole", "dcexploration", "dcex", "btpool", "fiftyeightcoin", "bitcoinindia", "shawnp0wers", "phashio", "rigpool", "haozhuzhu", "sevenpool", "miningkings", "hashbx", "dpool", "rawpool", "haominer", "helix", "bitcoinukraine", "poolin", "secretsuperstar", "tigerpoolnet", "sigmapoolcom", "okpooltop", "hummerpool", "tangpool", "bytepool", "spiderpool", "novablock", "miningcity", "binancepool", "minerium", "lubiancom", "okkong", "aaopool", "emcdpool", "foundryusa", "sbicrypto", "arkpool", "purebtccom", "marapool", "kucoinpool", "entrustcharitypool", "okminer", "titan", "pegapool", "btcnuggets", "cloudhashing", "digitalxmintsy", "telco214", "btcpoolparty", "multipool", "transactioncoinmining", "btcdig", "trickysbtcpool", "btcmp", "eobot", "unomp", "patels", "gogreenlight", "ekanembtc", "canoe", "tiger", "onem1x", "zulupool", "secpool", "ocean", "whitepool", "wk057", "futurebitapollosolo", "carbonnegative", "portlandhodl", "phoenix", "neopool", "maxipool", "bitfufupool", "luckypool", "miningdutch", "publicpool", "miningsquared", "innopolistech", "btclab", "parasite"]
QuarterIndex = int
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
SemesterIndex = int
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
WeekIndex = int
YearIndex = int
# Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
# block-based (height, txindex), and address/output type indexes.
Index = Literal["dateindex", "decadeindex", "difficultyepoch", "emptyoutputindex", "halvingepoch", "height", "txinindex", "monthindex", "opreturnindex", "txoutindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "quarterindex", "semesterindex", "txindex", "unknownoutputindex", "weekindex", "yearindex", "loadedaddressindex", "emptyaddressindex", "pairoutputindex"]
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

class LoadedAddressData(TypedDict):
    """
    Data for a loaded (non-empty) address with current balance

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

class OHLCCentsUnsigned(TypedDict):
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

    def close(self):
        """Close the HTTP client."""
        if self._conn:
            self._conn.close()
            self._conn = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


def _m(acc: str, s: str) -> str:
    """Build metric name with suffix."""
    if not s: return acc
    return f"{acc}_{s}" if acc else s


def _p(prefix: str, acc: str) -> str:
    """Build metric name with prefix."""
    return f"{prefix}_{acc}" if acc else prefix


# Date conversion constants
_GENESIS = date(2009, 1, 3)  # dateindex 0, weekindex 0
_DAY_ONE = date(2009, 1, 9)  # dateindex 1 (6 day gap after genesis)
_DATE_INDEXES = frozenset(['dateindex', 'weekindex', 'monthindex', 'yearindex', 'quarterindex', 'semesterindex', 'decadeindex'])

def is_date_index(index: str) -> bool:
    """Check if an index type is date-based."""
    return index in _DATE_INDEXES

def index_to_date(index: str, i: int) -> date:
    """Convert an index value to a date for date-based indexes."""
    if index == 'dateindex':
        return _GENESIS if i == 0 else _DAY_ONE + timedelta(days=i - 1)
    elif index == 'weekindex':
        return _GENESIS + timedelta(weeks=i)
    elif index == 'monthindex':
        return date(2009 + i // 12, i % 12 + 1, 1)
    elif index == 'yearindex':
        return date(2009 + i, 1, 1)
    elif index == 'quarterindex':
        m = i * 3
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'semesterindex':
        m = i * 6
        return date(2009 + m // 12, m % 12 + 1, 1)
    elif index == 'decadeindex':
        return date(2009 + i * 10, 1, 1)
    else:
        raise ValueError(f"{index} is not a date-based index")


@dataclass
class MetricData(Generic[T]):
    """Metric data with range information."""
    version: int
    index: Index
    total: int
    start: int
    end: int
    stamp: str
    data: List[T]

    def dates(self) -> List[date]:
        """Convert index range to dates. Only works for date-based indexes."""
        return [index_to_date(self.index, i) for i in range(self.start, self.end)]

    def indexes(self) -> List[int]:
        """Get index range as list."""
        return list(range(self.start, self.end))

    def to_date_dict(self) -> dict[date, T]:
        """Return data as {date: value} dict. Only works for date-based indexes."""
        return dict(zip(self.dates(), self.data))

    def to_index_dict(self) -> dict[int, T]:
        """Return data as {index: value} dict."""
        return dict(zip(range(self.start, self.end), self.data))

    def date_items(self) -> List[Tuple[date, T]]:
        """Return data as [(date, value), ...] pairs. Only works for date-based indexes."""
        return list(zip(self.dates(), self.data))

    def index_items(self) -> List[Tuple[int, T]]:
        """Return data as [(index, value), ...] pairs."""
        return list(zip(range(self.start, self.end), self.data))

    def iter(self) -> Iterator[Tuple[int, T]]:
        """Iterate over (index, value) pairs."""
        return iter(zip(range(self.start, self.end), self.data))

    def iter_dates(self) -> Iterator[Tuple[date, T]]:
        """Iterate over (date, value) pairs. Date-based indexes only."""
        return iter(zip(self.dates(), self.data))

    def __iter__(self) -> Iterator[Tuple[int, T]]:
        """Default iteration over (index, value) pairs."""
        return self.iter()

    def to_polars(self, with_dates: bool = True) -> pl.DataFrame:
        """Convert to Polars DataFrame. Requires polars to be installed.

        Returns a DataFrame with columns:
        - 'date' (date) and 'value' (T) if with_dates=True and index is date-based
        - 'index' (int) and 'value' (T) otherwise
        """
        try:
            import polars as pl  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("polars is required: pip install polars")
        if with_dates and self.index in _DATE_INDEXES:
            return pl.DataFrame({"date": self.dates(), "value": self.data})
        return pl.DataFrame({"index": list(range(self.start, self.end)), "value": self.data})

    def to_pandas(self, with_dates: bool = True) -> pd.DataFrame:
        """Convert to Pandas DataFrame. Requires pandas to be installed.

        Returns a DataFrame with columns:
        - 'date' (date) and 'value' (T) if with_dates=True and index is date-based
        - 'index' (int) and 'value' (T) otherwise
        """
        try:
            import pandas as pd  # type: ignore[import-not-found]
        except ImportError:
            raise ImportError("pandas is required: pip install pandas")
        if with_dates and self.index in _DATE_INDEXES:
            return pd.DataFrame({"date": self.dates(), "value": self.data})
        return pd.DataFrame({"index": list(range(self.start, self.end)), "value": self.data})


# Type alias for non-generic usage
AnyMetricData = MetricData[Any]


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

    def get_metric(self) -> MetricData:
        return MetricData(**self.client.get_json(self._build_path()))

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
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            start, start + n
        ))

    def fetch(self) -> MetricData[T]:
        """Fetch from skipped position to end."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch as CSV."""
        return self._config.get_csv()


class MetricEndpointBuilder(Generic[T]):
    """Builder for metric endpoint queries.

    Use method chaining to specify the data range, then call fetch() or fetch_csv() to execute.

    Examples:
        # Fetch all data
        data = endpoint.fetch()

        # Single item access
        data = endpoint[5].fetch()

        # Slice syntax (Python-native)
        data = endpoint[:10].fetch()      # First 10
        data = endpoint[-5:].fetch()      # Last 5
        data = endpoint[100:110].fetch()  # Range

        # Convenience methods (pandas-style)
        data = endpoint.head().fetch()    # First 10 (default)
        data = endpoint.head(20).fetch()  # First 20
        data = endpoint.tail(5).fetch()   # Last 5

        # Iterator-style chaining
        data = endpoint.skip(100).take(10).fetch()
    """

    def __init__(self, client: BrkClientBase, name: str, index: Index):
        self._config = _EndpointConfig(client, name, index)

    @overload
    def __getitem__(self, key: int) -> SingleItemBuilder[T]: ...
    @overload
    def __getitem__(self, key: slice) -> RangeBuilder[T]: ...

    def __getitem__(self, key: Union[int, slice]) -> Union[SingleItemBuilder[T], RangeBuilder[T]]:
        """Access single item or slice.

        Examples:
            endpoint[5]        # Single item at index 5
            endpoint[:10]      # First 10
            endpoint[-5:]      # Last 5
            endpoint[100:110]  # Range 100-109
        """
        if isinstance(key, int):
            return SingleItemBuilder(_EndpointConfig(
                self._config.client, self._config.name, self._config.index,
                key, key + 1
            ))
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            key.start, key.stop
        ))

    def head(self, n: int = 10) -> RangeBuilder[T]:
        """Get the first n items (pandas-style)."""
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            None, n
        ))

    def tail(self, n: int = 10) -> RangeBuilder[T]:
        """Get the last n items (pandas-style)."""
        start, end = (None, 0) if n == 0 else (-n, None)
        return RangeBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            start, end
        ))

    def skip(self, n: int) -> SkippedBuilder[T]:
        """Skip the first n items. Chain with take() to get a range."""
        return SkippedBuilder(_EndpointConfig(
            self._config.client, self._config.name, self._config.index,
            n, None
        ))

    def fetch(self) -> MetricData[T]:
        """Fetch all data as parsed JSON."""
        return self._config.get_metric()

    def fetch_csv(self) -> str:
        """Fetch all data as CSV string."""
        return self._config.get_csv()

    def path(self) -> str:
        """Get the base endpoint path."""
        return self._config.path()


# Type alias for non-generic usage
AnyMetricEndpointBuilder = MetricEndpointBuilder[Any]


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
_i1 = ('dateindex', 'decadeindex', 'difficultyepoch', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex')
_i2 = ('dateindex', 'decadeindex', 'difficultyepoch', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex')
_i3 = ('dateindex', 'decadeindex', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex')
_i4 = ('dateindex', 'decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex')
_i5 = ('dateindex', 'height')
_i6 = ('dateindex',)
_i7 = ('decadeindex',)
_i8 = ('difficultyepoch',)
_i9 = ('emptyoutputindex',)
_i10 = ('halvingepoch',)
_i11 = ('height',)
_i12 = ('txinindex',)
_i13 = ('monthindex',)
_i14 = ('opreturnindex',)
_i15 = ('txoutindex',)
_i16 = ('p2aaddressindex',)
_i17 = ('p2msoutputindex',)
_i18 = ('p2pk33addressindex',)
_i19 = ('p2pk65addressindex',)
_i20 = ('p2pkhaddressindex',)
_i21 = ('p2shaddressindex',)
_i22 = ('p2traddressindex',)
_i23 = ('p2wpkhaddressindex',)
_i24 = ('p2wshaddressindex',)
_i25 = ('quarterindex',)
_i26 = ('semesterindex',)
_i27 = ('txindex',)
_i28 = ('unknownoutputindex',)
_i29 = ('weekindex',)
_i30 = ('yearindex',)
_i31 = ('loadedaddressindex',)
_i32 = ('emptyaddressindex',)

def _ep(c: BrkClientBase, n: str, i: Index) -> MetricEndpointBuilder[Any]:
    return MetricEndpointBuilder(c, n, i)

# Index accessor classes

class _MetricPattern1By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')
    def decadeindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'decadeindex')
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')
    def monthindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'monthindex')
    def quarterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'quarterindex')
    def semesterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'semesterindex')
    def weekindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'weekindex')
    def yearindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'yearindex')

class MetricPattern1(Generic[T]):
    by: _MetricPattern1By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern1By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i1)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i1 else None

class _MetricPattern2By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')
    def decadeindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'decadeindex')
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')
    def monthindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'monthindex')
    def quarterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'quarterindex')
    def semesterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'semesterindex')
    def weekindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'weekindex')
    def yearindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'yearindex')

class MetricPattern2(Generic[T]):
    by: _MetricPattern2By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern2By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i2)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i2 else None

class _MetricPattern3By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')
    def decadeindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'decadeindex')
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')
    def monthindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'monthindex')
    def quarterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'quarterindex')
    def semesterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'semesterindex')
    def weekindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'weekindex')
    def yearindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'yearindex')

class MetricPattern3(Generic[T]):
    by: _MetricPattern3By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern3By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i3)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i3 else None

class _MetricPattern4By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')
    def decadeindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'decadeindex')
    def monthindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'monthindex')
    def quarterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'quarterindex')
    def semesterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'semesterindex')
    def weekindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'weekindex')
    def yearindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'yearindex')

class MetricPattern4(Generic[T]):
    by: _MetricPattern4By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern4By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i4)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i4 else None

class _MetricPattern5By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')

class MetricPattern5(Generic[T]):
    by: _MetricPattern5By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern5By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i5)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i5 else None

class _MetricPattern6By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def dateindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'dateindex')

class MetricPattern6(Generic[T]):
    by: _MetricPattern6By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern6By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i6)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i6 else None

class _MetricPattern7By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def decadeindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'decadeindex')

class MetricPattern7(Generic[T]):
    by: _MetricPattern7By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern7By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i7)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i7 else None

class _MetricPattern8By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def difficultyepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'difficultyepoch')

class MetricPattern8(Generic[T]):
    by: _MetricPattern8By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern8By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i8)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i8 else None

class _MetricPattern9By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyoutputindex')

class MetricPattern9(Generic[T]):
    by: _MetricPattern9By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern9By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i9)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i9 else None

class _MetricPattern10By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def halvingepoch(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'halvingepoch')

class MetricPattern10(Generic[T]):
    by: _MetricPattern10By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern10By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i10)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i10 else None

class _MetricPattern11By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def height(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'height')

class MetricPattern11(Generic[T]):
    by: _MetricPattern11By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern11By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i11)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i11 else None

class _MetricPattern12By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txinindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txinindex')

class MetricPattern12(Generic[T]):
    by: _MetricPattern12By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern12By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i12)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i12 else None

class _MetricPattern13By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def monthindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'monthindex')

class MetricPattern13(Generic[T]):
    by: _MetricPattern13By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern13By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i13)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i13 else None

class _MetricPattern14By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def opreturnindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'opreturnindex')

class MetricPattern14(Generic[T]):
    by: _MetricPattern14By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern14By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i14)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i14 else None

class _MetricPattern15By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txoutindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txoutindex')

class MetricPattern15(Generic[T]):
    by: _MetricPattern15By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern15By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i15)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i15 else None

class _MetricPattern16By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2aaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2aaddressindex')

class MetricPattern16(Generic[T]):
    by: _MetricPattern16By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern16By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i16)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i16 else None

class _MetricPattern17By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2msoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2msoutputindex')

class MetricPattern17(Generic[T]):
    by: _MetricPattern17By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern17By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i17)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i17 else None

class _MetricPattern18By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk33addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk33addressindex')

class MetricPattern18(Generic[T]):
    by: _MetricPattern18By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern18By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i18)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i18 else None

class _MetricPattern19By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pk65addressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pk65addressindex')

class MetricPattern19(Generic[T]):
    by: _MetricPattern19By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern19By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i19)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i19 else None

class _MetricPattern20By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2pkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2pkhaddressindex')

class MetricPattern20(Generic[T]):
    by: _MetricPattern20By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern20By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i20)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i20 else None

class _MetricPattern21By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2shaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2shaddressindex')

class MetricPattern21(Generic[T]):
    by: _MetricPattern21By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern21By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i21)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i21 else None

class _MetricPattern22By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2traddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2traddressindex')

class MetricPattern22(Generic[T]):
    by: _MetricPattern22By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern22By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i22)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i22 else None

class _MetricPattern23By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wpkhaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wpkhaddressindex')

class MetricPattern23(Generic[T]):
    by: _MetricPattern23By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern23By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i23)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i23 else None

class _MetricPattern24By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def p2wshaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'p2wshaddressindex')

class MetricPattern24(Generic[T]):
    by: _MetricPattern24By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern24By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i24)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i24 else None

class _MetricPattern25By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def quarterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'quarterindex')

class MetricPattern25(Generic[T]):
    by: _MetricPattern25By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern25By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i25)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i25 else None

class _MetricPattern26By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def semesterindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'semesterindex')

class MetricPattern26(Generic[T]):
    by: _MetricPattern26By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern26By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i26)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i26 else None

class _MetricPattern27By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def txindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'txindex')

class MetricPattern27(Generic[T]):
    by: _MetricPattern27By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern27By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i27)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i27 else None

class _MetricPattern28By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def unknownoutputindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'unknownoutputindex')

class MetricPattern28(Generic[T]):
    by: _MetricPattern28By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern28By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i28)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i28 else None

class _MetricPattern29By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def weekindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'weekindex')

class MetricPattern29(Generic[T]):
    by: _MetricPattern29By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern29By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i29)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i29 else None

class _MetricPattern30By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def yearindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'yearindex')

class MetricPattern30(Generic[T]):
    by: _MetricPattern30By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern30By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i30)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i30 else None

class _MetricPattern31By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def loadedaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'loadedaddressindex')

class MetricPattern31(Generic[T]):
    by: _MetricPattern31By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern31By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i31)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i31 else None

class _MetricPattern32By(Generic[T]):
    def __init__(self, c: BrkClientBase, n: str): self._c, self._n = c, n
    def emptyaddressindex(self) -> MetricEndpointBuilder[T]: return _ep(self._c, self._n, 'emptyaddressindex')

class MetricPattern32(Generic[T]):
    by: _MetricPattern32By[T]
    def __init__(self, c: BrkClientBase, n: str): self._n, self.by = n, _MetricPattern32By(c, n)
    @property
    def name(self) -> str: return self._n
    def indexes(self) -> List[str]: return list(_i32)
    def get(self, index: Index) -> Optional[MetricEndpointBuilder[T]]: return _ep(self.by._c, self._n, index) if index in _i32 else None

# Reusable structural pattern classes

class AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.cap_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern = RatioPattern(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: CumulativeSumPattern2[Dollars] = CumulativeSumPattern2(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern = RatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.cap_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: CumulativeSumPattern2[Dollars] = CumulativeSumPattern2(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_loss: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern = RatioPattern(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: CumulativeSumPattern2[Dollars] = CumulativeSumPattern2(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern = RatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cap_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'cap_raw'))
        self.capitulation_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'capitulation_flow'))
        self.investor_cap_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_raw'))
        self.investor_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'investor_price'))
        self.investor_price_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'investor_price_cents'))
        self.investor_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'investor_price_ratio'))
        self.loss_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_created'))
        self.loss_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'loss_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: CumulativeSumPattern2[Dollars] = CumulativeSumPattern2(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_7d_ema'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.peak_regret: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_peak_regret'))
        self.peak_regret_rel_to_realized_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'peak_regret_rel_to_realized_cap'))
        self.profit_flow: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_flow'))
        self.profit_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_created'))
        self.profit_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'profit_value_destroyed'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_cents: MetricPattern1[CentsUnsigned] = MetricPattern1(client, _m(acc, 'realized_cap_cents'))
        self.realized_loss: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_loss_7d_ema'))
        self.realized_loss_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RatioPattern2 = RatioPattern2(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_7d_ema: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_profit_7d_ema'))
        self.realized_profit_rel_to_realized_cap: CumulativeSumPattern[StoredF32] = CumulativeSumPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sent_in_loss: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_loss'))
        self.sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_loss_14d_ema'))
        self.sent_in_profit: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent_in_profit'))
        self.sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_in_profit_14d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._0sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, '0sd_usd'))
        self.m0_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm0_5sd'))
        self.m0_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm0_5sd_usd'))
        self.m1_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm1_5sd'))
        self.m1_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm1_5sd_usd'))
        self.m1sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm1sd'))
        self.m1sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm1sd_usd'))
        self.m2_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm2_5sd'))
        self.m2_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm2_5sd_usd'))
        self.m2sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm2sd'))
        self.m2sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm2sd_usd'))
        self.m3sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm3sd'))
        self.m3sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'm3sd_usd'))
        self.p0_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p0_5sd'))
        self.p0_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p0_5sd_usd'))
        self.p1_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p1_5sd'))
        self.p1_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p1_5sd_usd'))
        self.p1sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p1sd'))
        self.p1sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p1sd_usd'))
        self.p2_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p2_5sd'))
        self.p2_5sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p2_5sd_usd'))
        self.p2sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p2sd'))
        self.p2sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p2sd_usd'))
        self.p3sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p3sd'))
        self.p3sd_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'p3sd_usd'))
        self.sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sd'))
        self.sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sma'))
        self.zscore: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'zscore'))

class InvestedNegNetNuplSupplyUnrealizedPattern4:
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
        self.supply_rel_to_circulating_supply: MetricPattern4[StoredF64] = MetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_loss_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap'))
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))
        self.unrealized_profit_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl'))

class PriceRatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.price: DollarsSatsPattern2 = DollarsSatsPattern2(client, acc)
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio'))
        self.ratio_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_1m_sma'))
        self.ratio_1w_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_1w_sma'))
        self.ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_1y'))
        self.ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_2y'))
        self.ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio_4y'))
        self.ratio_pct1: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct1'))
        self.ratio_pct1_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct1_usd'))
        self.ratio_pct2: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct2'))
        self.ratio_pct2_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct2_usd'))
        self.ratio_pct5: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct5'))
        self.ratio_pct5_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct5_usd'))
        self.ratio_pct95: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct95'))
        self.ratio_pct95_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct95_usd'))
        self.ratio_pct98: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct98'))
        self.ratio_pct98_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct98_usd'))
        self.ratio_pct99: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct99'))
        self.ratio_pct99_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'ratio_pct99_usd'))
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, 'ratio'))

class Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.pct05: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct05'))
        self.pct10: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct10'))
        self.pct15: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct15'))
        self.pct20: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct20'))
        self.pct25: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct25'))
        self.pct30: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct30'))
        self.pct35: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct35'))
        self.pct40: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct40'))
        self.pct45: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct45'))
        self.pct50: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct50'))
        self.pct55: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct55'))
        self.pct60: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct60'))
        self.pct65: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct65'))
        self.pct70: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct70'))
        self.pct75: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct75'))
        self.pct80: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct80'))
        self.pct85: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct85'))
        self.pct90: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct90'))
        self.pct95: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct95'))

class RatioPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, acc)
        self.ratio_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1m_sma'))
        self.ratio_1w_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1w_sma'))
        self.ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '1y'))
        self.ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '2y'))
        self.ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, _m(acc, '4y'))
        self.ratio_pct1: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct1'))
        self.ratio_pct1_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct1_usd'))
        self.ratio_pct2: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct2'))
        self.ratio_pct2_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct2_usd'))
        self.ratio_pct5: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct5'))
        self.ratio_pct5_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct5_usd'))
        self.ratio_pct95: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct95'))
        self.ratio_pct95_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct95_usd'))
        self.ratio_pct98: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct98'))
        self.ratio_pct98_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct98_usd'))
        self.ratio_pct99: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct99'))
        self.ratio_pct99_usd: DollarsSatsPattern2 = DollarsSatsPattern2(client, _m(acc, 'pct99_usd'))
        self.ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern = _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern(client, acc)

class GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'greed_index'))
        self.invested_capital_in_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss'))
        self.invested_capital_in_loss_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit'))
        self.invested_capital_in_profit_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_sentiment: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_sentiment'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.pain_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'pain_index'))
        self.peak_regret: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'unrealized_peak_regret'))
        self.supply_in_loss: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_profit'))

class GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.greed_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'greed_index'))
        self.invested_capital_in_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss'))
        self.invested_capital_in_loss_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'invested_capital_in_loss_raw'))
        self.invested_capital_in_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit'))
        self.invested_capital_in_profit_raw: MetricPattern11[CentsSats] = MetricPattern11(client, _m(acc, 'invested_capital_in_profit_raw'))
        self.investor_cap_in_loss_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_in_loss_raw'))
        self.investor_cap_in_profit_raw: MetricPattern11[CentsSquaredSats] = MetricPattern11(client, _m(acc, 'investor_cap_in_profit_raw'))
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_sentiment: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_sentiment'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.pain_index: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'pain_index'))
        self.supply_in_loss: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_profit'))

class _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._1m_blocks_mined: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, '1m_blocks_mined'))
        self._1m_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1m_dominance'))
        self._1w_blocks_mined: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, '1w_blocks_mined'))
        self._1w_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1w_dominance'))
        self._1y_blocks_mined: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, '1y_blocks_mined'))
        self._1y_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '1y_dominance'))
        self._24h_blocks_mined: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, '24h_blocks_mined'))
        self._24h_dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, '24h_dominance'))
        self.blocks_mined: CumulativeSumPattern[StoredU32] = CumulativeSumPattern(client, _m(acc, 'blocks_mined'))
        self.blocks_since_block: MetricPattern1[StoredU32] = MetricPattern1(client, _m(acc, 'blocks_since_block'))
        self.coinbase: BitcoinDollarsSatsPattern6 = BitcoinDollarsSatsPattern6(client, _m(acc, 'coinbase'))
        self.days_since_block: MetricPattern4[StoredU16] = MetricPattern4(client, _m(acc, 'days_since_block'))
        self.dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance'))
        self.fee: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'fee'))
        self.subsidy: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'subsidy'))

class InvestedNegNetNuplSupplyUnrealizedPattern3:
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
        self.supply_rel_to_circulating_supply: MetricPattern4[StoredF64] = MetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'unrealized_peak_regret_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('10y', acc))
        self._1m: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('1m', acc))
        self._1w: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('1w', acc))
        self._1y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('1y', acc))
        self._2y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('2y', acc))
        self._3m: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('3m', acc))
        self._3y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('3y', acc))
        self._4y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('4y', acc))
        self._5y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('5y', acc))
        self._6m: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('6m', acc))
        self._6y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('6y', acc))
        self._8y: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _p('8y', acc))

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
        self.supply_rel_to_circulating_supply: MetricPattern4[StoredF64] = MetricPattern4(client, _m(acc, 'supply_rel_to_circulating_supply'))
        self.unrealized_loss_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_market_cap'))
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))

class InvestedNegNetSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct'))
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct'))
        self.neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_market_cap'))
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_market_cap'))
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))
        self.unrealized_loss_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_market_cap'))
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_loss_rel_to_own_total_unrealized_pnl'))
        self.unrealized_profit_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl'))

class _10y1m1w1y2y3m3y4y5y6m6y8yPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[T] = MetricPattern4(client, _p('10y', acc))
        self._1m: MetricPattern4[T] = MetricPattern4(client, _p('1m', acc))
        self._1w: MetricPattern4[T] = MetricPattern4(client, _p('1w', acc))
        self._1y: MetricPattern4[T] = MetricPattern4(client, _p('1y', acc))
        self._2y: MetricPattern4[T] = MetricPattern4(client, _p('2y', acc))
        self._3m: MetricPattern4[T] = MetricPattern4(client, _p('3m', acc))
        self._3y: MetricPattern4[T] = MetricPattern4(client, _p('3y', acc))
        self._4y: MetricPattern4[T] = MetricPattern4(client, _p('4y', acc))
        self._5y: MetricPattern4[T] = MetricPattern4(client, _p('5y', acc))
        self._6m: MetricPattern4[T] = MetricPattern4(client, _p('6m', acc))
        self._6y: MetricPattern4[T] = MetricPattern4(client, _p('6y', acc))
        self._8y: MetricPattern4[T] = MetricPattern4(client, _p('8y', acc))

class _201520162017201820192020202120222023202420252026Pattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._2015: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2015_returns'))
        self._2016: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2016_returns'))
        self._2017: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2017_returns'))
        self._2018: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2018_returns'))
        self._2019: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2019_returns'))
        self._2020: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2020_returns'))
        self._2021: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2021_returns'))
        self._2022: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2022_returns'))
        self._2023: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2023_returns'))
        self._2024: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2024_returns'))
        self._2025: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2025_returns'))
        self._2026: MetricPattern4[T] = MetricPattern4(client, _m(acc, '2026_returns'))

class AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[Bitcoin] = MetricPattern2(client, _m(acc, 'average'))
        self.base: MetricPattern11[Bitcoin] = MetricPattern11(client, acc)
        self.cumulative: MetricPattern2[Bitcoin] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[Bitcoin] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern6[Bitcoin] = MetricPattern6(client, _m(acc, 'median'))
        self.min: MetricPattern2[Bitcoin] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern6[Bitcoin] = MetricPattern6(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern6[Bitcoin] = MetricPattern6(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern6[Bitcoin] = MetricPattern6(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern6[Bitcoin] = MetricPattern6(client, _m(acc, 'pct90'))
        self.sum: MetricPattern2[Bitcoin] = MetricPattern2(client, _m(acc, 'sum'))

class AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'average'))
        self.base: MetricPattern11[T] = MetricPattern11(client, acc)
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'median'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct90'))
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average'))
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.max: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'max'))
        self.median: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'median'))
        self.min: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'min'))
        self.pct10: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct90'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'sum'))

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

class AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'average'))
        self.max: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'max'))
        self.median: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'median'))
        self.min: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'min'))
        self.pct10: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern11[T] = MetricPattern11(client, _m(acc, 'pct90'))
        self.txindex: MetricPattern27[T] = MetricPattern27(client, acc)

class AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'average'))
        self.base: MetricPattern11[T] = MetricPattern11(client, acc)
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'median'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct90'))

class ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'addr_count'))
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern = InvestedNegNetNuplSupplyUnrealizedPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class _10y2y3y4y5y6y8yPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('10y', acc))
        self._2y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('2y', acc))
        self._3y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('3y', acc))
        self._4y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('4y', acc))
        self._5y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('5y', acc))
        self._6y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('6y', acc))
        self._8y: MetricPattern4[StoredF32] = MetricPattern4(client, _p('8y', acc))

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2 = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc)
        self.relative: InvestedNegNetSupplyUnrealizedPattern = InvestedNegNetSupplyUnrealizedPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2 = AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern3 = InvestedNegNetNuplSupplyUnrealizedPattern3(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern = InvestedNegNetNuplSupplyUnrealizedPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc)
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern3 = InvestedNegNetNuplSupplyUnrealizedPattern3(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc)
        self.relative: InvestedSupplyPattern = InvestedSupplyPattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class ActivityCostOutputsRealizedSupplyUnrealizedPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, acc)
        self.cost_basis: MaxMinPattern = MaxMinPattern(client, acc)
        self.outputs: UtxoPattern = UtxoPattern(client, _m(acc, 'utxo_count'))
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, acc)
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, acc)
        self.unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern(client, acc)

class BalanceBothReactivatedReceivingSendingPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.balance_decreased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_decreased'))
        self.balance_increased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'balance_increased'))
        self.both: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'both'))
        self.reactivated: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'reactivated'))
        self.receiving: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'receiving'))
        self.sending: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredU32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, _m(acc, 'sending'))

class CoinblocksCoindaysSatblocksSatdaysSentPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, _m(acc, 'coindays_destroyed'))
        self.satblocks_destroyed: MetricPattern11[Sats] = MetricPattern11(client, _m(acc, 'satblocks_destroyed'))
        self.satdays_destroyed: MetricPattern11[Sats] = MetricPattern11(client, _m(acc, 'satdays_destroyed'))
        self.sent: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, _m(acc, 'sent'))
        self.sent_14d_ema: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, 'sent_14d_ema'))

class InvestedMaxMinPercentilesSpotPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'invested_capital'))
        self.max: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'max_cost_basis'))
        self.min: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'min_cost_basis'))
        self.percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern = Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern(client, _m(acc, 'cost_basis'))
        self.spot_cost_basis_percentile: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'spot_cost_basis_percentile'))
        self.spot_invested_capital_percentile: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'spot_invested_capital_percentile'))

class InvestedSupplyPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_loss_pct'))
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'invested_capital_in_profit_pct'))
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'supply_in_profit_rel_to_own_supply'))

class CloseHighLowOpenPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.close: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'close'))
        self.high: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'high'))
        self.low: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'low'))
        self.open: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'open'))

class _30dHalvedTotalPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._30d_change: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, _m(acc, '_30d_change'))
        self.halved: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply_halved'))
        self.total: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, _m(acc, 'supply'))

class BaseCumulativeSumPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern11[StoredF32] = MetricPattern11(client, acc)
        self.cumulative: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'sum'))

class BitcoinDollarsSatsPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern(client, _m(acc, 'btc'))
        self.dollars: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Dollars] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, _m(acc, 'usd'))
        self.sats: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Sats] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, acc)

class BitcoinDollarsSatsPattern4:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.dollars: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, acc)

class BitcoinDollarsSatsPattern5:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern4[Bitcoin] = MetricPattern4(client, _m(acc, 'btc'))
        self.dollars: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'usd'))
        self.sats: MetricPattern4[Sats] = MetricPattern4(client, acc)

class BitcoinDollarsSatsPattern6:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: CumulativeSumPattern[Bitcoin] = CumulativeSumPattern(client, _m(acc, 'btc'))
        self.dollars: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'usd'))
        self.sats: CumulativeSumPattern[Sats] = CumulativeSumPattern(client, acc)

class BitcoinDollarsSatsPattern3:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: CumulativeSumPattern2[Bitcoin] = CumulativeSumPattern2(client, _m(acc, 'btc'))
        self.dollars: CumulativeSumPattern[Dollars] = CumulativeSumPattern(client, _m(acc, 'usd'))
        self.sats: CumulativeSumPattern[Sats] = CumulativeSumPattern(client, acc)

class DollarsSatsPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.dollars: MetricPattern1[Dollars] = MetricPattern1(client, acc)
        self.sats: MetricPattern1[SatsFract] = MetricPattern1(client, _m(acc, 'sats'))

class DollarsSatsPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.dollars: MetricPattern4[Dollars] = MetricPattern4(client, acc)
        self.sats: MetricPattern4[SatsFract] = MetricPattern4(client, _m(acc, 'sats'))

class MaxMinPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'max_cost_basis'))
        self.min: DollarsSatsPattern = DollarsSatsPattern(client, _m(acc, 'min_cost_basis'))

class SdSmaPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sd'))
        self.sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sma'))

class CumulativeSumPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, acc)

class CumulativeSumPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, acc)

class OhlcSplitPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ohlc: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'ohlc_sats'))
        self.split: CloseHighLowOpenPattern2[T] = CloseHighLowOpenPattern2(client, _m(acc, 'sats'))

class RatioPattern2:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, acc)

class UtxoPattern:
    """Pattern struct for repeated tree structure."""
    
    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, acc)

# Metrics tree classes

class MetricsTree_Blocks_Difficulty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.raw: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty')
        self.as_hash: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_as_hash')
        self.adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_adjustment')
        self.epoch: MetricPattern4[DifficultyEpoch] = MetricPattern4(client, 'difficultyepoch')
        self.blocks_before_next_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_difficulty_adjustment')
        self.days_before_next_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_difficulty_adjustment')

class MetricsTree_Blocks_Time:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.timestamp: MetricPattern1[Timestamp] = MetricPattern1(client, 'timestamp')
        self.date: MetricPattern11[Date] = MetricPattern11(client, 'date')
        self.timestamp_monotonic: MetricPattern11[Timestamp] = MetricPattern11(client, 'timestamp_monotonic')

class MetricsTree_Blocks_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_count_target: MetricPattern4[StoredU64] = MetricPattern4(client, 'block_count_target')
        self.block_count: CumulativeSumPattern[StoredU32] = CumulativeSumPattern(client, 'block_count')
        self._24h_start: MetricPattern11[Height] = MetricPattern11(client, '24h_start')
        self._1w_start: MetricPattern11[Height] = MetricPattern11(client, '1w_start')
        self._1m_start: MetricPattern11[Height] = MetricPattern11(client, '1m_start')
        self._1y_start: MetricPattern11[Height] = MetricPattern11(client, '1y_start')
        self._24h_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '24h_block_count')
        self._1w_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1w_block_count')
        self._1m_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1m_block_count')
        self._1y_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1y_block_count')

class MetricsTree_Blocks_Mining:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.hash_rate: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate')
        self.hash_rate_1w_sma: MetricPattern4[StoredF64] = MetricPattern4(client, 'hash_rate_1w_sma')
        self.hash_rate_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_1m_sma')
        self.hash_rate_2m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_2m_sma')
        self.hash_rate_1y_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_1y_sma')
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

class MetricsTree_Blocks_Rewards_24hCoinbaseSum:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sats: MetricPattern11[Sats] = MetricPattern11(client, '24h_coinbase_sum')
        self.bitcoin: MetricPattern11[Bitcoin] = MetricPattern11(client, '24h_coinbase_sum_btc')
        self.dollars: MetricPattern11[Dollars] = MetricPattern11(client, '24h_coinbase_sum_usd')

class MetricsTree_Blocks_Rewards:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h_coinbase_sum: MetricsTree_Blocks_Rewards_24hCoinbaseSum = MetricsTree_Blocks_Rewards_24hCoinbaseSum(client)
        self.coinbase: BitcoinDollarsSatsPattern2 = BitcoinDollarsSatsPattern2(client, 'coinbase')
        self.subsidy: BitcoinDollarsSatsPattern2 = BitcoinDollarsSatsPattern2(client, 'subsidy')
        self.unclaimed_rewards: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, 'unclaimed_rewards')
        self.fee_dominance: MetricPattern6[StoredF32] = MetricPattern6(client, 'fee_dominance')
        self.subsidy_dominance: MetricPattern6[StoredF32] = MetricPattern6(client, 'subsidy_dominance')
        self.subsidy_usd_1y_sma: MetricPattern4[Dollars] = MetricPattern4(client, 'subsidy_usd_1y_sma')

class MetricsTree_Blocks_Halving:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.epoch: MetricPattern4[HalvingEpoch] = MetricPattern4(client, 'halvingepoch')
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_halving')

class MetricsTree_Blocks_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_size_cumulative')
        self.average: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_average')
        self.min: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_min')
        self.max: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_max')
        self.pct10: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct10')
        self.pct25: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct25')
        self.median: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_median')
        self.pct75: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct75')
        self.pct90: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct90')
        self.sum: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_sum')

class MetricsTree_Blocks:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern11[BlockHash] = MetricPattern11(client, 'blockhash')
        self.difficulty: MetricsTree_Blocks_Difficulty = MetricsTree_Blocks_Difficulty(client)
        self.time: MetricsTree_Blocks_Time = MetricsTree_Blocks_Time(client)
        self.total_size: MetricPattern11[StoredU64] = MetricPattern11(client, 'total_size')
        self.weight: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Weight] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'block_weight')
        self.count: MetricsTree_Blocks_Count = MetricsTree_Blocks_Count(client)
        self.interval: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[Timestamp] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'block_interval')
        self.mining: MetricsTree_Blocks_Mining = MetricsTree_Blocks_Mining(client)
        self.rewards: MetricsTree_Blocks_Rewards = MetricsTree_Blocks_Rewards(client)
        self.halving: MetricsTree_Blocks_Halving = MetricsTree_Blocks_Halving(client)
        self.vbytes: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'block_vbytes')
        self.size: MetricsTree_Blocks_Size = MetricsTree_Blocks_Size(client)
        self.fullness: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'block_fullness')

class MetricsTree_Transactions_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.tx_count: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'tx_count')
        self.is_coinbase: MetricPattern27[StoredBool] = MetricPattern27(client, 'is_coinbase')

class MetricsTree_Transactions_Size:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern[VSize] = AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(client, 'tx_vsize')
        self.weight: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern[Weight] = AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(client, 'tx_weight')

class MetricsTree_Transactions_Fees_Fee:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txindex: MetricPattern27[Sats] = MetricPattern27(client, 'fee')
        self.sats: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Sats] = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fee')
        self.bitcoin: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Bitcoin] = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fee_btc')
        self.dollars: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[Dollars] = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'fee_usd')

class MetricsTree_Transactions_Fees:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.input_value: MetricPattern27[Sats] = MetricPattern27(client, 'input_value')
        self.output_value: MetricPattern27[Sats] = MetricPattern27(client, 'output_value')
        self.fee: MetricsTree_Transactions_Fees_Fee = MetricsTree_Transactions_Fees_Fee(client)
        self.fee_rate: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern[FeeRate] = AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern(client, 'fee_rate')

class MetricsTree_Transactions_Versions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: CumulativeSumPattern[StoredU64] = CumulativeSumPattern(client, 'tx_v1')
        self.v2: CumulativeSumPattern[StoredU64] = CumulativeSumPattern(client, 'tx_v2')
        self.v3: CumulativeSumPattern[StoredU64] = CumulativeSumPattern(client, 'tx_v3')

class MetricsTree_Transactions_Volume:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sent_sum: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, 'sent_sum')
        self.received_sum: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, 'received_sum')
        self.annualized_volume: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'annualized_volume')
        self.tx_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'tx_per_sec')
        self.outputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'outputs_per_sec')
        self.inputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'inputs_per_sec')

class MetricsTree_Transactions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txindex: MetricPattern11[TxIndex] = MetricPattern11(client, 'first_txindex')
        self.height: MetricPattern27[Height] = MetricPattern27(client, 'height')
        self.txid: MetricPattern27[Txid] = MetricPattern27(client, 'txid')
        self.txversion: MetricPattern27[TxVersion] = MetricPattern27(client, 'txversion')
        self.rawlocktime: MetricPattern27[RawLockTime] = MetricPattern27(client, 'rawlocktime')
        self.base_size: MetricPattern27[StoredU32] = MetricPattern27(client, 'base_size')
        self.total_size: MetricPattern27[StoredU32] = MetricPattern27(client, 'total_size')
        self.is_explicitly_rbf: MetricPattern27[StoredBool] = MetricPattern27(client, 'is_explicitly_rbf')
        self.first_txinindex: MetricPattern27[TxInIndex] = MetricPattern27(client, 'first_txinindex')
        self.first_txoutindex: MetricPattern27[TxOutIndex] = MetricPattern27(client, 'first_txoutindex')
        self.count: MetricsTree_Transactions_Count = MetricsTree_Transactions_Count(client)
        self.size: MetricsTree_Transactions_Size = MetricsTree_Transactions_Size(client)
        self.fees: MetricsTree_Transactions_Fees = MetricsTree_Transactions_Fees(client)
        self.versions: MetricsTree_Transactions_Versions = MetricsTree_Transactions_Versions(client)
        self.volume: MetricsTree_Transactions_Volume = MetricsTree_Transactions_Volume(client)

class MetricsTree_Inputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern12[TxOutIndex] = MetricPattern12(client, 'txoutindex')
        self.value: MetricPattern12[Sats] = MetricPattern12(client, 'value')

class MetricsTree_Inputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txinindex: MetricPattern11[TxInIndex] = MetricPattern11(client, 'first_txinindex')
        self.outpoint: MetricPattern12[OutPoint] = MetricPattern12(client, 'outpoint')
        self.txindex: MetricPattern12[TxIndex] = MetricPattern12(client, 'txindex')
        self.outputtype: MetricPattern12[OutputType] = MetricPattern12(client, 'outputtype')
        self.typeindex: MetricPattern12[TypeIndex] = MetricPattern12(client, 'typeindex')
        self.spent: MetricsTree_Inputs_Spent = MetricsTree_Inputs_Spent(client)
        self.count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'input_count')

class MetricsTree_Outputs_Spent:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern15[TxInIndex] = MetricPattern15(client, 'txinindex')

class MetricsTree_Outputs_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'output_count')
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, 'exact_utxo_count')

class MetricsTree_Outputs:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_txoutindex: MetricPattern11[TxOutIndex] = MetricPattern11(client, 'first_txoutindex')
        self.value: MetricPattern15[Sats] = MetricPattern15(client, 'value')
        self.outputtype: MetricPattern15[OutputType] = MetricPattern15(client, 'outputtype')
        self.typeindex: MetricPattern15[TypeIndex] = MetricPattern15(client, 'typeindex')
        self.txindex: MetricPattern15[TxIndex] = MetricPattern15(client, 'txindex')
        self.spent: MetricsTree_Outputs_Spent = MetricsTree_Outputs_Spent(client)
        self.count: MetricsTree_Outputs_Count = MetricsTree_Outputs_Count(client)

class MetricsTree_Addresses:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2pk65addressindex: MetricPattern11[P2PK65AddressIndex] = MetricPattern11(client, 'first_p2pk65addressindex')
        self.first_p2pk33addressindex: MetricPattern11[P2PK33AddressIndex] = MetricPattern11(client, 'first_p2pk33addressindex')
        self.first_p2pkhaddressindex: MetricPattern11[P2PKHAddressIndex] = MetricPattern11(client, 'first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern11[P2SHAddressIndex] = MetricPattern11(client, 'first_p2shaddressindex')
        self.first_p2wpkhaddressindex: MetricPattern11[P2WPKHAddressIndex] = MetricPattern11(client, 'first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern11[P2WSHAddressIndex] = MetricPattern11(client, 'first_p2wshaddressindex')
        self.first_p2traddressindex: MetricPattern11[P2TRAddressIndex] = MetricPattern11(client, 'first_p2traddressindex')
        self.first_p2aaddressindex: MetricPattern11[P2AAddressIndex] = MetricPattern11(client, 'first_p2aaddressindex')
        self.p2pk65bytes: MetricPattern19[P2PK65Bytes] = MetricPattern19(client, 'p2pk65bytes')
        self.p2pk33bytes: MetricPattern18[P2PK33Bytes] = MetricPattern18(client, 'p2pk33bytes')
        self.p2pkhbytes: MetricPattern20[P2PKHBytes] = MetricPattern20(client, 'p2pkhbytes')
        self.p2shbytes: MetricPattern21[P2SHBytes] = MetricPattern21(client, 'p2shbytes')
        self.p2wpkhbytes: MetricPattern23[P2WPKHBytes] = MetricPattern23(client, 'p2wpkhbytes')
        self.p2wshbytes: MetricPattern24[P2WSHBytes] = MetricPattern24(client, 'p2wshbytes')
        self.p2trbytes: MetricPattern22[P2TRBytes] = MetricPattern22(client, 'p2trbytes')
        self.p2abytes: MetricPattern16[P2ABytes] = MetricPattern16(client, 'p2abytes')

class MetricsTree_Scripts_Count:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2a_count')
        self.p2ms: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2ms_count')
        self.p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pk33_count')
        self.p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pk65_count')
        self.p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pkh_count')
        self.p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2sh_count')
        self.p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2tr_count')
        self.p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2wpkh_count')
        self.p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2wsh_count')
        self.opreturn: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'opreturn_count')
        self.emptyoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'emptyoutput_count')
        self.unknownoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'unknownoutput_count')
        self.segwit: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'segwit_count')
        self.taproot_adoption: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'taproot_adoption')
        self.segwit_adoption: BaseCumulativeSumPattern = BaseCumulativeSumPattern(client, 'segwit_adoption')

class MetricsTree_Scripts_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BitcoinDollarsSatsPattern2 = BitcoinDollarsSatsPattern2(client, 'opreturn_value')

class MetricsTree_Scripts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_emptyoutputindex: MetricPattern11[EmptyOutputIndex] = MetricPattern11(client, 'first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern11[OpReturnIndex] = MetricPattern11(client, 'first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern11[P2MSOutputIndex] = MetricPattern11(client, 'first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern11[UnknownOutputIndex] = MetricPattern11(client, 'first_unknownoutputindex')
        self.empty_to_txindex: MetricPattern9[TxIndex] = MetricPattern9(client, 'txindex')
        self.opreturn_to_txindex: MetricPattern14[TxIndex] = MetricPattern14(client, 'txindex')
        self.p2ms_to_txindex: MetricPattern17[TxIndex] = MetricPattern17(client, 'txindex')
        self.unknown_to_txindex: MetricPattern28[TxIndex] = MetricPattern28(client, 'txindex')
        self.count: MetricsTree_Scripts_Count = MetricsTree_Scripts_Count(client)
        self.value: MetricsTree_Scripts_Value = MetricsTree_Scripts_Value(client)

class MetricsTree_Positions:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_position: MetricPattern11[BlkPosition] = MetricPattern11(client, 'position')
        self.tx_position: MetricPattern27[BlkPosition] = MetricPattern27(client, 'position')

class MetricsTree_Cointime_Activity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.coinblocks_created: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'coinblocks_created')
        self.coinblocks_stored: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, 'liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, 'vaultedness')
        self.activity_to_vaultedness_ratio: MetricPattern1[StoredF64] = MetricPattern1(client, 'activity_to_vaultedness_ratio')

class MetricsTree_Cointime_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vaulted_supply: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, 'vaulted_supply')
        self.active_supply: BitcoinDollarsSatsPattern4 = BitcoinDollarsSatsPattern4(client, 'active_supply')

class MetricsTree_Cointime_Value:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_value_destroyed: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'cointime_value_destroyed')
        self.cointime_value_created: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'cointime_value_created')
        self.cointime_value_stored: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'cointime_value_stored')
        self.vocdd: CumulativeSumPattern[StoredF64] = CumulativeSumPattern(client, 'vocdd')

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
        self.vaulted_price: DollarsSatsPattern = DollarsSatsPattern(client, 'vaulted_price')
        self.vaulted_price_ratio: RatioPattern = RatioPattern(client, 'vaulted_price_ratio')
        self.active_price: DollarsSatsPattern = DollarsSatsPattern(client, 'active_price')
        self.active_price_ratio: RatioPattern = RatioPattern(client, 'active_price_ratio')
        self.true_market_mean: DollarsSatsPattern = DollarsSatsPattern(client, 'true_market_mean')
        self.true_market_mean_ratio: RatioPattern = RatioPattern(client, 'true_market_mean_ratio')
        self.cointime_price: DollarsSatsPattern = DollarsSatsPattern(client, 'cointime_price')
        self.cointime_price_ratio: RatioPattern = RatioPattern(client, 'cointime_price_ratio')

class MetricsTree_Cointime_Adjusted:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: MetricPattern4[StoredF32] = MetricPattern4(client, 'cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, 'cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, 'cointime_adj_tx_usd_velocity')

class MetricsTree_Cointime_ReserveRisk:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vocdd_365d_sma: MetricPattern6[StoredF64] = MetricPattern6(client, 'vocdd_365d_sma')
        self.hodl_bank: MetricPattern6[StoredF64] = MetricPattern6(client, 'hodl_bank')
        self.reserve_risk: MetricPattern4[StoredF64] = MetricPattern4(client, 'reserve_risk')

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
        self.identity: MetricPattern18[P2PK33AddressIndex] = MetricPattern18(client, 'p2pk33addressindex')

class MetricsTree_Indexes_Address_P2pk65:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern19[P2PK65AddressIndex] = MetricPattern19(client, 'p2pk65addressindex')

class MetricsTree_Indexes_Address_P2pkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern20[P2PKHAddressIndex] = MetricPattern20(client, 'p2pkhaddressindex')

class MetricsTree_Indexes_Address_P2sh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern21[P2SHAddressIndex] = MetricPattern21(client, 'p2shaddressindex')

class MetricsTree_Indexes_Address_P2tr:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern22[P2TRAddressIndex] = MetricPattern22(client, 'p2traddressindex')

class MetricsTree_Indexes_Address_P2wpkh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern23[P2WPKHAddressIndex] = MetricPattern23(client, 'p2wpkhaddressindex')

class MetricsTree_Indexes_Address_P2wsh:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern24[P2WSHAddressIndex] = MetricPattern24(client, 'p2wshaddressindex')

class MetricsTree_Indexes_Address_P2a:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern16[P2AAddressIndex] = MetricPattern16(client, 'p2aaddressindex')

class MetricsTree_Indexes_Address_P2ms:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern17[P2MSOutputIndex] = MetricPattern17(client, 'p2msoutputindex')

class MetricsTree_Indexes_Address_Empty:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern9[EmptyOutputIndex] = MetricPattern9(client, 'emptyoutputindex')

class MetricsTree_Indexes_Address_Unknown:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern28[UnknownOutputIndex] = MetricPattern28(client, 'unknownoutputindex')

class MetricsTree_Indexes_Address_Opreturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern14[OpReturnIndex] = MetricPattern14(client, 'opreturnindex')

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
        self.identity: MetricPattern11[Height] = MetricPattern11(client, 'height')
        self.dateindex: MetricPattern11[DateIndex] = MetricPattern11(client, 'dateindex')
        self.difficultyepoch: MetricPattern11[DifficultyEpoch] = MetricPattern11(client, 'difficultyepoch')
        self.halvingepoch: MetricPattern11[HalvingEpoch] = MetricPattern11(client, 'halvingepoch')
        self.txindex_count: MetricPattern11[StoredU64] = MetricPattern11(client, 'txindex_count')

class MetricsTree_Indexes_Difficultyepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern8[DifficultyEpoch] = MetricPattern8(client, 'difficultyepoch')
        self.first_height: MetricPattern8[Height] = MetricPattern8(client, 'first_height')
        self.height_count: MetricPattern8[StoredU64] = MetricPattern8(client, 'height_count')

class MetricsTree_Indexes_Halvingepoch:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern10[HalvingEpoch] = MetricPattern10(client, 'halvingepoch')
        self.first_height: MetricPattern10[Height] = MetricPattern10(client, 'first_height')

class MetricsTree_Indexes_Dateindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern6[DateIndex] = MetricPattern6(client, 'dateindex')
        self.date: MetricPattern6[Date] = MetricPattern6(client, 'date')
        self.first_height: MetricPattern6[Height] = MetricPattern6(client, 'first_height')
        self.height_count: MetricPattern6[StoredU64] = MetricPattern6(client, 'height_count')
        self.weekindex: MetricPattern6[WeekIndex] = MetricPattern6(client, 'weekindex')
        self.monthindex: MetricPattern6[MonthIndex] = MetricPattern6(client, 'monthindex')

class MetricsTree_Indexes_Weekindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern29[WeekIndex] = MetricPattern29(client, 'weekindex')
        self.date: MetricPattern29[Date] = MetricPattern29(client, 'date')
        self.first_dateindex: MetricPattern29[DateIndex] = MetricPattern29(client, 'first_dateindex')
        self.dateindex_count: MetricPattern29[StoredU64] = MetricPattern29(client, 'dateindex_count')

class MetricsTree_Indexes_Monthindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern13[MonthIndex] = MetricPattern13(client, 'monthindex')
        self.date: MetricPattern13[Date] = MetricPattern13(client, 'date')
        self.first_dateindex: MetricPattern13[DateIndex] = MetricPattern13(client, 'first_dateindex')
        self.dateindex_count: MetricPattern13[StoredU64] = MetricPattern13(client, 'dateindex_count')
        self.quarterindex: MetricPattern13[QuarterIndex] = MetricPattern13(client, 'quarterindex')
        self.semesterindex: MetricPattern13[SemesterIndex] = MetricPattern13(client, 'semesterindex')
        self.yearindex: MetricPattern13[YearIndex] = MetricPattern13(client, 'yearindex')

class MetricsTree_Indexes_Quarterindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern25[QuarterIndex] = MetricPattern25(client, 'quarterindex')
        self.date: MetricPattern25[Date] = MetricPattern25(client, 'date')
        self.first_monthindex: MetricPattern25[MonthIndex] = MetricPattern25(client, 'first_monthindex')
        self.monthindex_count: MetricPattern25[StoredU64] = MetricPattern25(client, 'monthindex_count')

class MetricsTree_Indexes_Semesterindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern26[SemesterIndex] = MetricPattern26(client, 'semesterindex')
        self.date: MetricPattern26[Date] = MetricPattern26(client, 'date')
        self.first_monthindex: MetricPattern26[MonthIndex] = MetricPattern26(client, 'first_monthindex')
        self.monthindex_count: MetricPattern26[StoredU64] = MetricPattern26(client, 'monthindex_count')

class MetricsTree_Indexes_Yearindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern30[YearIndex] = MetricPattern30(client, 'yearindex')
        self.date: MetricPattern30[Date] = MetricPattern30(client, 'date')
        self.first_monthindex: MetricPattern30[MonthIndex] = MetricPattern30(client, 'first_monthindex')
        self.monthindex_count: MetricPattern30[StoredU64] = MetricPattern30(client, 'monthindex_count')
        self.decadeindex: MetricPattern30[DecadeIndex] = MetricPattern30(client, 'decadeindex')

class MetricsTree_Indexes_Decadeindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern7[DecadeIndex] = MetricPattern7(client, 'decadeindex')
        self.date: MetricPattern7[Date] = MetricPattern7(client, 'date')
        self.first_yearindex: MetricPattern7[YearIndex] = MetricPattern7(client, 'first_yearindex')
        self.yearindex_count: MetricPattern7[StoredU64] = MetricPattern7(client, 'yearindex_count')

class MetricsTree_Indexes_Txindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern27[TxIndex] = MetricPattern27(client, 'txindex')
        self.input_count: MetricPattern27[StoredU64] = MetricPattern27(client, 'input_count')
        self.output_count: MetricPattern27[StoredU64] = MetricPattern27(client, 'output_count')

class MetricsTree_Indexes_Txinindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern12[TxInIndex] = MetricPattern12(client, 'txinindex')

class MetricsTree_Indexes_Txoutindex:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern15[TxOutIndex] = MetricPattern15(client, 'txoutindex')

class MetricsTree_Indexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: MetricsTree_Indexes_Address = MetricsTree_Indexes_Address(client)
        self.height: MetricsTree_Indexes_Height = MetricsTree_Indexes_Height(client)
        self.difficultyepoch: MetricsTree_Indexes_Difficultyepoch = MetricsTree_Indexes_Difficultyepoch(client)
        self.halvingepoch: MetricsTree_Indexes_Halvingepoch = MetricsTree_Indexes_Halvingepoch(client)
        self.dateindex: MetricsTree_Indexes_Dateindex = MetricsTree_Indexes_Dateindex(client)
        self.weekindex: MetricsTree_Indexes_Weekindex = MetricsTree_Indexes_Weekindex(client)
        self.monthindex: MetricsTree_Indexes_Monthindex = MetricsTree_Indexes_Monthindex(client)
        self.quarterindex: MetricsTree_Indexes_Quarterindex = MetricsTree_Indexes_Quarterindex(client)
        self.semesterindex: MetricsTree_Indexes_Semesterindex = MetricsTree_Indexes_Semesterindex(client)
        self.yearindex: MetricsTree_Indexes_Yearindex = MetricsTree_Indexes_Yearindex(client)
        self.decadeindex: MetricsTree_Indexes_Decadeindex = MetricsTree_Indexes_Decadeindex(client)
        self.txindex: MetricsTree_Indexes_Txindex = MetricsTree_Indexes_Txindex(client)
        self.txinindex: MetricsTree_Indexes_Txinindex = MetricsTree_Indexes_Txinindex(client)
        self.txoutindex: MetricsTree_Indexes_Txoutindex = MetricsTree_Indexes_Txoutindex(client)

class MetricsTree_Market_Ath:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ath: DollarsSatsPattern = DollarsSatsPattern(client, 'price_ath')
        self.price_drawdown: MetricPattern3[StoredF32] = MetricPattern3(client, 'price_drawdown')
        self.days_since_price_ath: MetricPattern4[StoredU16] = MetricPattern4(client, 'days_since_price_ath')
        self.years_since_price_ath: MetricPattern4[StoredF32] = MetricPattern4(client, 'years_since_price_ath')
        self.max_days_between_price_aths: MetricPattern4[StoredU16] = MetricPattern4(client, 'max_days_between_price_aths')
        self.max_years_between_price_aths: MetricPattern4[StoredF32] = MetricPattern4(client, 'max_years_between_price_aths')

class MetricsTree_Market_Lookback:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1d_ago')
        self._1w: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1w_ago')
        self._1m: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1m_ago')
        self._3m: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_3m_ago')
        self._6m: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_6m_ago')
        self._1y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1y_ago')
        self._2y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_2y_ago')
        self._3y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_3y_ago')
        self._4y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_4y_ago')
        self._5y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_5y_ago')
        self._6y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_6y_ago')
        self._8y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_8y_ago')
        self._10y: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_10y_ago')

class MetricsTree_Market_Returns_PriceReturns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: MetricPattern4[StoredF32] = MetricPattern4(client, '1d_price_returns')
        self._1w: MetricPattern4[StoredF32] = MetricPattern4(client, '1w_price_returns')
        self._1m: MetricPattern4[StoredF32] = MetricPattern4(client, '1m_price_returns')
        self._3m: MetricPattern4[StoredF32] = MetricPattern4(client, '3m_price_returns')
        self._6m: MetricPattern4[StoredF32] = MetricPattern4(client, '6m_price_returns')
        self._1y: MetricPattern4[StoredF32] = MetricPattern4(client, '1y_price_returns')
        self._2y: MetricPattern4[StoredF32] = MetricPattern4(client, '2y_price_returns')
        self._3y: MetricPattern4[StoredF32] = MetricPattern4(client, '3y_price_returns')
        self._4y: MetricPattern4[StoredF32] = MetricPattern4(client, '4y_price_returns')
        self._5y: MetricPattern4[StoredF32] = MetricPattern4(client, '5y_price_returns')
        self._6y: MetricPattern4[StoredF32] = MetricPattern4(client, '6y_price_returns')
        self._8y: MetricPattern4[StoredF32] = MetricPattern4(client, '8y_price_returns')
        self._10y: MetricPattern4[StoredF32] = MetricPattern4(client, '10y_price_returns')

class MetricsTree_Market_Returns:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_returns: MetricsTree_Market_Returns_PriceReturns = MetricsTree_Market_Returns_PriceReturns(client)
        self.cagr: _10y2y3y4y5y6y8yPattern = _10y2y3y4y5y6y8yPattern(client, 'cagr')
        self._1d_returns_1w_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1w_sd')
        self._1d_returns_1m_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1m_sd')
        self._1d_returns_1y_sd: SdSmaPattern = SdSmaPattern(client, '1d_returns_1y_sd')
        self.downside_returns: MetricPattern6[StoredF32] = MetricPattern6(client, 'downside_returns')
        self.downside_1w_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1w_sd')
        self.downside_1m_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1m_sd')
        self.downside_1y_sd: SdSmaPattern = SdSmaPattern(client, 'downside_1y_sd')

class MetricsTree_Market_Volatility:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1w_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1w_volatility')
        self.price_1m_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1m_volatility')
        self.price_1y_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1y_volatility')
        self.sharpe_1w: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1w')
        self.sharpe_1m: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1m')
        self.sharpe_1y: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1y')
        self.sortino_1w: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1w')
        self.sortino_1m: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1m')
        self.sortino_1y: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1y')

class MetricsTree_Market_Range:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1w_min: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1w_min')
        self.price_1w_max: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1w_max')
        self.price_2w_min: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_2w_min')
        self.price_2w_max: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_2w_max')
        self.price_1m_min: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1m_min')
        self.price_1m_max: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1m_max')
        self.price_1y_min: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1y_min')
        self.price_1y_max: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_1y_max')
        self.price_true_range: MetricPattern6[StoredF32] = MetricPattern6(client, 'price_true_range')
        self.price_true_range_2w_sum: MetricPattern6[StoredF32] = MetricPattern6(client, 'price_true_range_2w_sum')
        self.price_2w_choppiness_index: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_2w_choppiness_index')

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
        self.price_200d_sma_x2_4: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_200d_sma_x2_4')
        self.price_200d_sma_x0_8: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_200d_sma_x0_8')
        self.price_350d_sma_x2: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'price_350d_sma_x2')

class MetricsTree_Market_Dca_PeriodAveragePrice:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: DollarsSatsPattern2 = DollarsSatsPattern2(client, '1w_dca_average_price')
        self._1m: DollarsSatsPattern2 = DollarsSatsPattern2(client, '1m_dca_average_price')
        self._3m: DollarsSatsPattern2 = DollarsSatsPattern2(client, '3m_dca_average_price')
        self._6m: DollarsSatsPattern2 = DollarsSatsPattern2(client, '6m_dca_average_price')
        self._1y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '1y_dca_average_price')
        self._2y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '2y_dca_average_price')
        self._3y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '3y_dca_average_price')
        self._4y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '4y_dca_average_price')
        self._5y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '5y_dca_average_price')
        self._6y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '6y_dca_average_price')
        self._8y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '8y_dca_average_price')
        self._10y: DollarsSatsPattern2 = DollarsSatsPattern2(client, '10y_dca_average_price')

class MetricsTree_Market_Dca_ClassStack:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2015_stack')
        self._2016: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2016_stack')
        self._2017: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2017_stack')
        self._2018: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2018_stack')
        self._2019: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2019_stack')
        self._2020: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2020_stack')
        self._2021: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2021_stack')
        self._2022: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2022_stack')
        self._2023: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2023_stack')
        self._2024: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2024_stack')
        self._2025: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2025_stack')
        self._2026: BitcoinDollarsSatsPattern5 = BitcoinDollarsSatsPattern5(client, 'dca_class_2026_stack')

class MetricsTree_Market_Dca_ClassAveragePrice:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2015_average_price')
        self._2016: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2016_average_price')
        self._2017: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2017_average_price')
        self._2018: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2018_average_price')
        self._2019: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2019_average_price')
        self._2020: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2020_average_price')
        self._2021: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2021_average_price')
        self._2022: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2022_average_price')
        self._2023: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2023_average_price')
        self._2024: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2024_average_price')
        self._2025: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2025_average_price')
        self._2026: DollarsSatsPattern2 = DollarsSatsPattern2(client, 'dca_class_2026_average_price')

class MetricsTree_Market_Dca_ClassDaysInProfit:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2015_days_in_profit')
        self._2016: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2016_days_in_profit')
        self._2017: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2017_days_in_profit')
        self._2018: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2018_days_in_profit')
        self._2019: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2019_days_in_profit')
        self._2020: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2020_days_in_profit')
        self._2021: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2021_days_in_profit')
        self._2022: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2022_days_in_profit')
        self._2023: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2023_days_in_profit')
        self._2024: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2024_days_in_profit')
        self._2025: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2025_days_in_profit')
        self._2026: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2026_days_in_profit')

class MetricsTree_Market_Dca_ClassDaysInLoss:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2015_days_in_loss')
        self._2016: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2016_days_in_loss')
        self._2017: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2017_days_in_loss')
        self._2018: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2018_days_in_loss')
        self._2019: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2019_days_in_loss')
        self._2020: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2020_days_in_loss')
        self._2021: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2021_days_in_loss')
        self._2022: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2022_days_in_loss')
        self._2023: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2023_days_in_loss')
        self._2024: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2024_days_in_loss')
        self._2025: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2025_days_in_loss')
        self._2026: MetricPattern4[StoredU32] = MetricPattern4(client, 'dca_class_2026_days_in_loss')

class MetricsTree_Market_Dca_ClassMinReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2015_min_return')
        self._2016: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2016_min_return')
        self._2017: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2017_min_return')
        self._2018: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2018_min_return')
        self._2019: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2019_min_return')
        self._2020: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2020_min_return')
        self._2021: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2021_min_return')
        self._2022: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2022_min_return')
        self._2023: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2023_min_return')
        self._2024: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2024_min_return')
        self._2025: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2025_min_return')
        self._2026: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2026_min_return')

class MetricsTree_Market_Dca_ClassMaxReturn:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2015_max_return')
        self._2016: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2016_max_return')
        self._2017: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2017_max_return')
        self._2018: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2018_max_return')
        self._2019: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2019_max_return')
        self._2020: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2020_max_return')
        self._2021: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2021_max_return')
        self._2022: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2022_max_return')
        self._2023: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2023_max_return')
        self._2024: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2024_max_return')
        self._2025: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2025_max_return')
        self._2026: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2026_max_return')

class MetricsTree_Market_Dca:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
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

class MetricsTree_Market_Indicators:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.puell_multiple: MetricPattern4[StoredF32] = MetricPattern4(client, 'puell_multiple')
        self.nvt: MetricPattern4[StoredF32] = MetricPattern4(client, 'nvt')
        self.rsi_gains: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_gains')
        self.rsi_losses: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_losses')
        self.rsi_average_gain_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_average_gain_14d')
        self.rsi_average_loss_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_average_loss_14d')
        self.rsi_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d')
        self.rsi_14d_min: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d_min')
        self.rsi_14d_max: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d_max')
        self.stoch_rsi: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi')
        self.stoch_rsi_k: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi_k')
        self.stoch_rsi_d: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi_d')
        self.stoch_k: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_k')
        self.stoch_d: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_d')
        self.pi_cycle: MetricPattern6[StoredF32] = MetricPattern6(client, 'pi_cycle')
        self.macd_line: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_line')
        self.macd_signal: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_signal')
        self.macd_histogram: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_histogram')
        self.gini: MetricPattern6[StoredF32] = MetricPattern6(client, 'gini')

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
        self.unknown: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'unknown')
        self.blockfills: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'blockfills')
        self.ultimuspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ultimuspool')
        self.terrapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'terrapool')
        self.luxor: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'luxor')
        self.onethash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onethash')
        self.btccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btccom')
        self.bitfarms: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfarms')
        self.huobipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'huobipool')
        self.wayicn: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'wayicn')
        self.canoepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'canoepool')
        self.btctop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btctop')
        self.bitcoincom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoincom')
        self.pool175btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pool175btc')
        self.gbminers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'gbminers')
        self.axbt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'axbt')
        self.asicminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'asicminer')
        self.bitminter: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitminter')
        self.bitcoinrussia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinrussia')
        self.btcserv: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcserv')
        self.simplecoinus: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'simplecoinus')
        self.btcguild: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcguild')
        self.eligius: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eligius')
        self.ozcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ozcoin')
        self.eclipsemc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eclipsemc')
        self.maxbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'maxbtc')
        self.triplemining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'triplemining')
        self.coinlab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'coinlab')
        self.pool50btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pool50btc')
        self.ghashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ghashio')
        self.stminingcorp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'stminingcorp')
        self.bitparking: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitparking')
        self.mmpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mmpool')
        self.polmine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'polmine')
        self.kncminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kncminer')
        self.bitalo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitalo')
        self.f2pool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'f2pool')
        self.hhtt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hhtt')
        self.megabigpower: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'megabigpower')
        self.mtred: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mtred')
        self.nmcbit: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nmcbit')
        self.yourbtcnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'yourbtcnet')
        self.givemecoins: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'givemecoins')
        self.braiinspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'braiinspool')
        self.antpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'antpool')
        self.multicoinco: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'multicoinco')
        self.bcpoolio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bcpoolio')
        self.cointerra: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'cointerra')
        self.kanopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kanopool')
        self.solock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'solock')
        self.ckpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ckpool')
        self.nicehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nicehash')
        self.bitclub: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitclub')
        self.bitcoinaffiliatenetwork: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinaffiliatenetwork')
        self.btcc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcc')
        self.bwpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bwpool')
        self.exxbw: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'exxbw')
        self.bitsolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitsolo')
        self.bitfury: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfury')
        self.twentyoneinc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'twentyoneinc')
        self.digitalbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'digitalbtc')
        self.eightbaochi: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eightbaochi')
        self.mybtccoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'mybtccoinpool')
        self.tbdice: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tbdice')
        self.hashpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hashpool')
        self.nexious: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'nexious')
        self.bravomining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bravomining')
        self.hotpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hotpool')
        self.okexpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okexpool')
        self.bcmonster: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bcmonster')
        self.onehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onehash')
        self.bixin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bixin')
        self.tatmaspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tatmaspool')
        self.viabtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'viabtc')
        self.connectbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'connectbtc')
        self.batpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'batpool')
        self.waterhole: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'waterhole')
        self.dcexploration: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dcexploration')
        self.dcex: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dcex')
        self.btpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btpool')
        self.fiftyeightcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'fiftyeightcoin')
        self.bitcoinindia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinindia')
        self.shawnp0wers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'shawnp0wers')
        self.phashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'phashio')
        self.rigpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'rigpool')
        self.haozhuzhu: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'haozhuzhu')
        self.sevenpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sevenpool')
        self.miningkings: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningkings')
        self.hashbx: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hashbx')
        self.dpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'dpool')
        self.rawpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'rawpool')
        self.haominer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'haominer')
        self.helix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'helix')
        self.bitcoinukraine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitcoinukraine')
        self.poolin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'poolin')
        self.secretsuperstar: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'secretsuperstar')
        self.tigerpoolnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tigerpoolnet')
        self.sigmapoolcom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sigmapoolcom')
        self.okpooltop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okpooltop')
        self.hummerpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'hummerpool')
        self.tangpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tangpool')
        self.bytepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bytepool')
        self.spiderpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'spiderpool')
        self.novablock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'novablock')
        self.miningcity: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningcity')
        self.binancepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'binancepool')
        self.minerium: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'minerium')
        self.lubiancom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'lubiancom')
        self.okkong: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okkong')
        self.aaopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'aaopool')
        self.emcdpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'emcdpool')
        self.foundryusa: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'foundryusa')
        self.sbicrypto: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'sbicrypto')
        self.arkpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'arkpool')
        self.purebtccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'purebtccom')
        self.marapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'marapool')
        self.kucoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'kucoinpool')
        self.entrustcharitypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'entrustcharitypool')
        self.okminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'okminer')
        self.titan: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'titan')
        self.pegapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'pegapool')
        self.btcnuggets: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcnuggets')
        self.cloudhashing: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'cloudhashing')
        self.digitalxmintsy: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'digitalxmintsy')
        self.telco214: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'telco214')
        self.btcpoolparty: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcpoolparty')
        self.multipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'multipool')
        self.transactioncoinmining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'transactioncoinmining')
        self.btcdig: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcdig')
        self.trickysbtcpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'trickysbtcpool')
        self.btcmp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btcmp')
        self.eobot: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'eobot')
        self.unomp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'unomp')
        self.patels: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'patels')
        self.gogreenlight: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'gogreenlight')
        self.ekanembtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ekanembtc')
        self.canoe: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'canoe')
        self.tiger: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'tiger')
        self.onem1x: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'onem1x')
        self.zulupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'zulupool')
        self.secpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'secpool')
        self.ocean: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'ocean')
        self.whitepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'whitepool')
        self.wk057: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'wk057')
        self.futurebitapollosolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'futurebitapollosolo')
        self.carbonnegative: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'carbonnegative')
        self.portlandhodl: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'portlandhodl')
        self.phoenix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'phoenix')
        self.neopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'neopool')
        self.maxipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'maxipool')
        self.bitfufupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'bitfufupool')
        self.luckypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'luckypool')
        self.miningdutch: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningdutch')
        self.publicpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'publicpool')
        self.miningsquared: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'miningsquared')
        self.innopolistech: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'innopolistech')
        self.btclab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'btclab')
        self.parasite: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern = _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern(client, 'parasite')

class MetricsTree_Pools:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.height_to_pool: MetricPattern11[PoolSlug] = MetricPattern11(client, 'pool')
        self.vecs: MetricsTree_Pools_Vecs = MetricsTree_Pools_Vecs(client)

class MetricsTree_Price_Cents_Split:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.open: MetricPattern5[CentsUnsigned] = MetricPattern5(client, 'price_open_cents')
        self.high: MetricPattern5[CentsUnsigned] = MetricPattern5(client, 'price_high_cents')
        self.low: MetricPattern5[CentsUnsigned] = MetricPattern5(client, 'price_low_cents')
        self.close: MetricPattern5[CentsUnsigned] = MetricPattern5(client, 'price_close_cents')

class MetricsTree_Price_Cents:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.split: MetricsTree_Price_Cents_Split = MetricsTree_Price_Cents_Split(client)
        self.ohlc: MetricPattern5[OHLCCentsUnsigned] = MetricPattern5(client, 'ohlc_cents')

class MetricsTree_Price_Usd:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.split: CloseHighLowOpenPattern2[Dollars] = CloseHighLowOpenPattern2(client, 'price')
        self.ohlc: MetricPattern1[OHLCDollars] = MetricPattern1(client, 'price_ohlc')

class MetricsTree_Price:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: MetricsTree_Price_Cents = MetricsTree_Price_Cents(client)
        self.usd: MetricsTree_Price_Usd = MetricsTree_Price_Usd(client)
        self.sats: OhlcSplitPattern2[OHLCSats] = OhlcSplitPattern2(client, 'price')

class MetricsTree_Distribution_AnyAddressIndexes:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern16[AnyAddressIndex] = MetricPattern16(client, 'anyaddressindex')
        self.p2pk33: MetricPattern18[AnyAddressIndex] = MetricPattern18(client, 'anyaddressindex')
        self.p2pk65: MetricPattern19[AnyAddressIndex] = MetricPattern19(client, 'anyaddressindex')
        self.p2pkh: MetricPattern20[AnyAddressIndex] = MetricPattern20(client, 'anyaddressindex')
        self.p2sh: MetricPattern21[AnyAddressIndex] = MetricPattern21(client, 'anyaddressindex')
        self.p2tr: MetricPattern22[AnyAddressIndex] = MetricPattern22(client, 'anyaddressindex')
        self.p2wpkh: MetricPattern23[AnyAddressIndex] = MetricPattern23(client, 'anyaddressindex')
        self.p2wsh: MetricPattern24[AnyAddressIndex] = MetricPattern24(client, 'anyaddressindex')

class MetricsTree_Distribution_AddressesData:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.loaded: MetricPattern31[LoadedAddressData] = MetricPattern31(client, 'loadedaddressdata')
        self.empty: MetricPattern32[EmptyAddressData] = MetricPattern32(client, 'emptyaddressdata')

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
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_profit_rel_to_own_total_unrealized_pnl')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.invested_capital_in_profit_pct: MetricPattern1[StoredF32] = MetricPattern1(client, 'invested_capital_in_profit_pct')
        self.invested_capital_in_loss_pct: MetricPattern1[StoredF32] = MetricPattern1(client, 'invested_capital_in_loss_pct')
        self.unrealized_peak_regret_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, 'unrealized_peak_regret_rel_to_market_cap')

class MetricsTree_Distribution_UtxoCohorts_All:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, '')
        self.outputs: UtxoPattern = UtxoPattern(client, 'utxo_count')
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, '')
        self.realized: AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, '')
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, '')
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, '')
        self.relative: MetricsTree_Distribution_UtxoCohorts_All_Relative = MetricsTree_Distribution_UtxoCohorts_All_Relative(client)

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

class MetricsTree_Distribution_UtxoCohorts_MinAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_1d_old')
        self._1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_1w_old')
        self._1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_1m_old')
        self._2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_2m_old')
        self._3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_3m_old')
        self._4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_4m_old')
        self._5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_5m_old')
        self._6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_6m_old')
        self._1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_1y_old')
        self._2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_2y_old')
        self._3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_3y_old')
        self._4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_4y_old')
        self._5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_5y_old')
        self._6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_6y_old')
        self._7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_7y_old')
        self._8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_8y_old')
        self._10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_10y_old')
        self._12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6(client, 'utxos_over_12y_old')

class MetricsTree_Distribution_UtxoCohorts_GeAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_1sat')
        self._10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_10sats')
        self._100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_100sats')
        self._1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_1k_sats')
        self._10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_10k_sats')
        self._100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_100k_sats')
        self._1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_1m_sats')
        self._10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_10m_sats')
        self._1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_1btc')
        self._10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_10btc')
        self._100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_100btc')
        self._1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_1k_btc')
        self._10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_over_10k_btc')

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

class MetricsTree_Distribution_UtxoCohorts_Term_Short:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, 'sth')
        self.outputs: UtxoPattern = UtxoPattern(client, 'sth_utxo_count')
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, 'sth')
        self.realized: AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern = AdjustedCapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern(client, 'sth')
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, 'sth')
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, 'sth')
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern4 = InvestedNegNetNuplSupplyUnrealizedPattern4(client, 'sth')

class MetricsTree_Distribution_UtxoCohorts_Term_Long:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply: _30dHalvedTotalPattern = _30dHalvedTotalPattern(client, 'lth')
        self.outputs: UtxoPattern = UtxoPattern(client, 'lth_utxo_count')
        self.activity: CoinblocksCoindaysSatblocksSatdaysSentPattern = CoinblocksCoindaysSatblocksSatdaysSentPattern(client, 'lth')
        self.realized: CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2 = CapCapitulationInvestorLossMvrvNegNetPeakProfitRealizedSellSentSoprTotalValuePattern2(client, 'lth')
        self.unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern = GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern(client, 'lth')
        self.cost_basis: InvestedMaxMinPercentilesSpotPattern = InvestedMaxMinPercentilesSpotPattern(client, 'lth')
        self.relative: InvestedNegNetNuplSupplyUnrealizedPattern4 = InvestedNegNetNuplSupplyUnrealizedPattern4(client, 'lth')

class MetricsTree_Distribution_UtxoCohorts_Term:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.short: MetricsTree_Distribution_UtxoCohorts_Term_Short = MetricsTree_Distribution_UtxoCohorts_Term_Short(client)
        self.long: MetricsTree_Distribution_UtxoCohorts_Term_Long = MetricsTree_Distribution_UtxoCohorts_Term_Long(client)

class MetricsTree_Distribution_UtxoCohorts_Type:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pk65')
        self.p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pk33')
        self.p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2pkh')
        self.p2ms: ActivityCostOutputsRealizedSupplyUnrealizedPattern = ActivityCostOutputsRealizedSupplyUnrealizedPattern(client, 'p2ms')
        self.p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2sh')
        self.p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2wpkh')
        self.p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2wsh')
        self.p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2tr')
        self.p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3(client, 'p2a')
        self.unknown: ActivityCostOutputsRealizedSupplyUnrealizedPattern = ActivityCostOutputsRealizedSupplyUnrealizedPattern(client, 'unknown_outputs')
        self.empty: ActivityCostOutputsRealizedSupplyUnrealizedPattern = ActivityCostOutputsRealizedSupplyUnrealizedPattern(client, 'empty_outputs')

class MetricsTree_Distribution_UtxoCohorts_MaxAge:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_1w_old')
        self._1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_1m_old')
        self._2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_2m_old')
        self._3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_3m_old')
        self._4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_4m_old')
        self._5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_5m_old')
        self._6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_6m_old')
        self._1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_1y_old')
        self._2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_2y_old')
        self._3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_3y_old')
        self._4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_4y_old')
        self._5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_5y_old')
        self._6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_6y_old')
        self._7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_7y_old')
        self._8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_8y_old')
        self._10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_10y_old')
        self._12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_12y_old')
        self._15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5(client, 'utxos_under_15y_old')

class MetricsTree_Distribution_UtxoCohorts_LtAmount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10sats')
        self._100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_100sats')
        self._1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1k_sats')
        self._10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10k_sats')
        self._100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_100k_sats')
        self._1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1m_sats')
        self._10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10m_sats')
        self._1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1btc')
        self._10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10btc')
        self._100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_100btc')
        self._1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_1k_btc')
        self._10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_10k_btc')
        self._100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 = ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4(client, 'utxos_under_100k_btc')

class MetricsTree_Distribution_UtxoCohorts:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricsTree_Distribution_UtxoCohorts_All = MetricsTree_Distribution_UtxoCohorts_All(client)
        self.age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange = MetricsTree_Distribution_UtxoCohorts_AgeRange(client)
        self.epoch: MetricsTree_Distribution_UtxoCohorts_Epoch = MetricsTree_Distribution_UtxoCohorts_Epoch(client)
        self.year: MetricsTree_Distribution_UtxoCohorts_Year = MetricsTree_Distribution_UtxoCohorts_Year(client)
        self.min_age: MetricsTree_Distribution_UtxoCohorts_MinAge = MetricsTree_Distribution_UtxoCohorts_MinAge(client)
        self.ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount = MetricsTree_Distribution_UtxoCohorts_GeAmount(client)
        self.amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange = MetricsTree_Distribution_UtxoCohorts_AmountRange(client)
        self.term: MetricsTree_Distribution_UtxoCohorts_Term = MetricsTree_Distribution_UtxoCohorts_Term(client)
        self.type_: MetricsTree_Distribution_UtxoCohorts_Type = MetricsTree_Distribution_UtxoCohorts_Type(client)
        self.max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge = MetricsTree_Distribution_UtxoCohorts_MaxAge(client)
        self.lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount = MetricsTree_Distribution_UtxoCohorts_LtAmount(client)

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

class MetricsTree_Distribution_NewAddrCount:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'new_addr_count')
        self.p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pk65_new_addr_count')
        self.p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pk33_new_addr_count')
        self.p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2pkh_new_addr_count')
        self.p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2sh_new_addr_count')
        self.p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2wpkh_new_addr_count')
        self.p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2wsh_new_addr_count')
        self.p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2tr_new_addr_count')
        self.p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2[StoredU64] = AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2(client, 'p2a_new_addr_count')

class MetricsTree_Distribution_GrowthRate:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'growth_rate')
        self.p2pk65: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2pk65_growth_rate')
        self.p2pk33: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2pk33_growth_rate')
        self.p2pkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2pkh_growth_rate')
        self.p2sh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2sh_growth_rate')
        self.p2wpkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2wpkh_growth_rate')
        self.p2wsh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2wsh_growth_rate')
        self.p2tr: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2tr_growth_rate')
        self.p2a: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern[StoredF32] = AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern(client, 'p2a_growth_rate')

class MetricsTree_Distribution:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.supply_state: MetricPattern11[SupplyState] = MetricPattern11(client, 'supply_state')
        self.any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes = MetricsTree_Distribution_AnyAddressIndexes(client)
        self.addresses_data: MetricsTree_Distribution_AddressesData = MetricsTree_Distribution_AddressesData(client)
        self.utxo_cohorts: MetricsTree_Distribution_UtxoCohorts = MetricsTree_Distribution_UtxoCohorts(client)
        self.address_cohorts: MetricsTree_Distribution_AddressCohorts = MetricsTree_Distribution_AddressCohorts(client)
        self.addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'addr_count')
        self.empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'empty_addr_count')
        self.address_activity: MetricsTree_Distribution_AddressActivity = MetricsTree_Distribution_AddressActivity(client)
        self.total_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern = AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern(client, 'total_addr_count')
        self.new_addr_count: MetricsTree_Distribution_NewAddrCount = MetricsTree_Distribution_NewAddrCount(client)
        self.growth_rate: MetricsTree_Distribution_GrowthRate = MetricsTree_Distribution_GrowthRate(client)
        self.loadedaddressindex: MetricPattern31[LoadedAddressIndex] = MetricPattern31(client, 'loadedaddressindex')
        self.emptyaddressindex: MetricPattern32[EmptyAddressIndex] = MetricPattern32(client, 'emptyaddressindex')

class MetricsTree_Supply_Circulating:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.sats: MetricPattern3[Sats] = MetricPattern3(client, 'circulating_supply')
        self.bitcoin: MetricPattern3[Bitcoin] = MetricPattern3(client, 'circulating_supply_btc')
        self.dollars: MetricPattern3[Dollars] = MetricPattern3(client, 'circulating_supply_usd')

class MetricsTree_Supply_Burned:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, 'opreturn_supply')
        self.unspendable: BitcoinDollarsSatsPattern3 = BitcoinDollarsSatsPattern3(client, 'unspendable_supply')

class MetricsTree_Supply_Velocity:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern4[StoredF64] = MetricPattern4(client, 'btc_velocity')
        self.usd: MetricPattern4[StoredF64] = MetricPattern4(client, 'usd_velocity')

class MetricsTree_Supply:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.circulating: MetricsTree_Supply_Circulating = MetricsTree_Supply_Circulating(client)
        self.burned: MetricsTree_Supply_Burned = MetricsTree_Supply_Burned(client)
        self.inflation: MetricPattern4[StoredF32] = MetricPattern4(client, 'inflation_rate')
        self.velocity: MetricsTree_Supply_Velocity = MetricsTree_Supply_Velocity(client)
        self.market_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'market_cap')

class MetricsTree:
    """Metrics tree node."""
    
    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks: MetricsTree_Blocks = MetricsTree_Blocks(client)
        self.transactions: MetricsTree_Transactions = MetricsTree_Transactions(client)
        self.inputs: MetricsTree_Inputs = MetricsTree_Inputs(client)
        self.outputs: MetricsTree_Outputs = MetricsTree_Outputs(client)
        self.addresses: MetricsTree_Addresses = MetricsTree_Addresses(client)
        self.scripts: MetricsTree_Scripts = MetricsTree_Scripts(client)
        self.positions: MetricsTree_Positions = MetricsTree_Positions(client)
        self.cointime: MetricsTree_Cointime = MetricsTree_Cointime(client)
        self.constants: MetricsTree_Constants = MetricsTree_Constants(client)
        self.indexes: MetricsTree_Indexes = MetricsTree_Indexes(client)
        self.market: MetricsTree_Market = MetricsTree_Market(client)
        self.pools: MetricsTree_Pools = MetricsTree_Pools(client)
        self.price: MetricsTree_Price = MetricsTree_Price(client)
        self.distribution: MetricsTree_Distribution = MetricsTree_Distribution(client)
        self.supply: MetricsTree_Supply = MetricsTree_Supply(client)

class BrkClient(BrkClientBase):
    """Main BRK client with metrics tree and API methods."""

    VERSION = "v0.1.2"

    INDEXES = [
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
      "exxbw": "EXX&BW",
      "f2pool": "F2Pool",
      "fiftyeightcoin": "58COIN",
      "foundryusa": "Foundry USA",
      "futurebitapollosolo": "FutureBit Apollo Solo",
      "gbminers": "GBMiners",
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
      "luckypool": "luckyPool",
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

    def index_to_date(self, index: Index, i: int) -> date:
        """Convert an index value to a date for date-based indexes."""
        return index_to_date(index, i)

    def is_date_index(self, index: Index) -> bool:
        """Check if an index type is date-based."""
        return is_date_index(index)

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

    def get_mempool_txids(self) -> List[Txid]:
        """Mempool transaction IDs.

        Get all transaction IDs currently in the mempool.

        *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*

        Endpoint: `GET /api/mempool/txids`"""
        return self.get_json('/api/mempool/txids')

    def get_metric_info(self, metric: Metric) -> List[Index]:
        """Get supported indexes for a metric.

        Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.

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

