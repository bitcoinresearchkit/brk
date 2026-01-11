# Auto-generated BRK Python client
# Do not edit manually

from __future__ import annotations
from typing import TypeVar, Generic, Any, Optional, List, Literal, TypedDict, Union, Protocol
import httpx

T = TypeVar('T')

# Type definitions

# Bitcoin address string
Address = str
# Satoshis
Sats = int
# Index within its type (e.g., 0 for first P2WPKH address)
TypeIndex = int
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

# Transaction ID (hash)
Txid = str
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
        address: The validated address
        isscript: Whether this is a script address (P2SH)
        isvalid: Whether the address is valid
        iswitness: Whether this is a witness address
        scriptPubKey: The scriptPubKey in hex
        witness_program: Witness program in hex
        witness_version: Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
    """
    address: Optional[str]
    isscript: Optional[bool]
    isvalid: bool
    iswitness: Optional[bool]
    scriptPubKey: Optional[str]
    witness_program: Optional[str]
    witness_version: Optional[int]

# Unified index for any address type (loaded or empty)
AnyAddressIndex = TypeIndex
# Bitcoin amount as floating point (1 BTC = 100,000,000 satoshis)
Bitcoin = float
# Position within a .blk file, encoding file index and byte offset
BlkPosition = int
class BlockCountParam(TypedDict):
    """
    Attributes:
        block_count: Number of recent blocks to include
    """
    block_count: int

# Block height
Height = int
# UNIX timestamp in seconds
Timestamp = int
class BlockFeesEntry(TypedDict):
    """
    A single block fees data point.
    """
    avgFees: Sats
    avgHeight: Height
    timestamp: Timestamp

# Block hash
BlockHash = str
class BlockHashParam(TypedDict):
    hash: BlockHash

TxIndex = int
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

# Transaction or block weight in weight units (WU)
Weight = int
class BlockInfo(TypedDict):
    """
    Block information returned by the API

    Attributes:
        difficulty: Block difficulty as a floating point number
        height: Block height
        id: Block hash
        size: Block size in bytes
        timestamp: Block timestamp (Unix time)
        tx_count: Number of transactions in the block
        weight: Block weight in weight units
    """
    difficulty: float
    height: Height
    id: BlockHash
    size: int
    timestamp: Timestamp
    tx_count: int
    weight: Weight

class BlockRewardsEntry(TypedDict):
    """
    A single block rewards data point.
    """
    avgHeight: int
    avgRewards: int
    timestamp: int

class BlockSizeEntry(TypedDict):
    """
    A single block size data point.
    """
    avgHeight: int
    avgSize: int
    timestamp: int

class BlockWeightEntry(TypedDict):
    """
    A single block weight data point.
    """
    avgHeight: int
    avgWeight: int
    timestamp: int

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
        height: Block height (only if in best chain)
        in_best_chain: Whether this block is in the best chain
        next_best: Hash of the next block in the best chain (only if in best chain and not tip)
    """
    height: Union[Height, None]
    in_best_chain: bool
    next_best: Union[BlockHash, None]

class BlockTimestamp(TypedDict):
    """
    Block information returned for timestamp queries

    Attributes:
        hash: Block hash
        height: Block height
        timestamp: Block timestamp in ISO 8601 format
    """
    hash: BlockHash
    height: Height
    timestamp: str

Cents = int
# Closing price value for a time period
Close = Cents
# Output format for API responses
Format = Literal["json", "csv"]
class DataRangeFormat(TypedDict):
    """
    Data range with output format for API query parameters

    Attributes:
        count: Number of values to return (ignored if `to` is set)
        format: Format of the output
        from_: Inclusive starting index, if negative counts from end
        to: Exclusive ending index, if negative counts from end
    """
    count: Optional[int]
    format: Format
    from_: Optional[int]
    to: Optional[int]

# Date in YYYYMMDD format stored as u32
Date = int
DateIndex = int
DecadeIndex = int
class DifficultyAdjustment(TypedDict):
    """
    Difficulty adjustment information.

    Attributes:
        adjustedTimeAvg: Time-adjusted average (accounting for timestamp manipulation)
        difficultyChange: Estimated difficulty change at next retarget (%)
        estimatedRetargetDate: Estimated Unix timestamp of next retarget
        nextRetargetHeight: Height of next retarget
        previousRetarget: Previous difficulty adjustment (%)
        progressPercent: Progress through current difficulty epoch (0-100%)
        remainingBlocks: Blocks remaining until retarget
        remainingTime: Estimated seconds until retarget
        timeAvg: Average block time in current epoch (seconds)
        timeOffset: Time offset from expected schedule (seconds)
    """
    adjustedTimeAvg: int
    difficultyChange: float
    estimatedRetargetDate: int
    nextRetargetHeight: Height
    previousRetarget: float
    progressPercent: float
    remainingBlocks: int
    remainingTime: int
    timeAvg: int
    timeOffset: int

class DifficultyAdjustmentEntry(TypedDict):
    """
    A single difficulty adjustment entry.
    Serializes as array: [timestamp, height, difficulty, change_percent]
    """
    change_percent: float
    difficulty: float
    height: Height
    timestamp: Timestamp

class DifficultyEntry(TypedDict):
    """
    A single difficulty data point.

    Attributes:
        difficulty: Difficulty value.
        height: Block height of the adjustment.
        timestamp: Unix timestamp of the difficulty adjustment.
    """
    difficulty: float
    height: Height
    timestamp: Timestamp

DifficultyEpoch = int
# US Dollar amount as floating point
Dollars = float
class EmptyAddressData(TypedDict):
    """
    Data of an empty address

    Attributes:
        funded_txo_count: Total funded/spent transaction output count (equal since address is empty)
        transfered: Total satoshis transferred
        tx_count: Total transaction count
    """
    funded_txo_count: int
    transfered: Sats
    tx_count: int

EmptyAddressIndex = TypeIndex
EmptyOutputIndex = TypeIndex
# Fee rate in sats/vB
FeeRate = float
HalvingEpoch = int
class HashrateEntry(TypedDict):
    """
    A single hashrate data point.

    Attributes:
        avgHashrate: Average hashrate (H/s).
        timestamp: Unix timestamp.
    """
    avgHashrate: int
    timestamp: Timestamp

class HashrateSummary(TypedDict):
    """
    Summary of network hashrate and difficulty data.

    Attributes:
        currentDifficulty: Current network difficulty.
        currentHashrate: Current network hashrate (H/s).
        difficulty: Historical difficulty adjustments.
        hashrates: Historical hashrate data points.
    """
    currentDifficulty: float
    currentHashrate: int
    difficulty: List[DifficultyEntry]
    hashrates: List[HashrateEntry]

class Health(TypedDict):
    """
    Server health status
    """
    service: str
    status: str
    timestamp: str

class HeightParam(TypedDict):
    height: Height

# Hex-encoded string
Hex = str
# Highest price value for a time period
High = Cents
class IndexInfo(TypedDict):
    """
    Information about an available index and its query aliases

    Attributes:
        aliases: All Accepted query aliases
        index: The canonical index name
    """
    aliases: List[str]
    index: Index

# Maximum number of results to return. Defaults to 100 if not specified.
Limit = int
class LimitParam(TypedDict):
    limit: Limit

class LoadedAddressData(TypedDict):
    """
    Data for a loaded (non-empty) address with current balance

    Attributes:
        funded_txo_count: Number of transaction outputs funded to this address
        realized_cap: The realized capitalization of this address
        received: Satoshis received by this address
        sent: Satoshis sent by this address
        spent_txo_count: Number of transaction outputs spent by this address
        tx_count: Total transaction count
    """
    funded_txo_count: int
    realized_cap: Dollars
    received: Sats
    sent: Sats
    spent_txo_count: int
    tx_count: int

LoadedAddressIndex = TypeIndex
# Lowest price value for a time period
Low = Cents
class MempoolBlock(TypedDict):
    """
    Block info in a mempool.space like format for fee estimation.

    Attributes:
        blockSize: Total block size in weight units
        blockVSize: Total block virtual size in vbytes
        feeRange: Fee rate range: [min, 10%, 25%, 50%, 75%, 90%, max]
        medianFee: Median fee rate in sat/vB
        nTx: Number of transactions in the projected block
        totalFees: Total fees in satoshis
    """
    blockSize: int
    blockVSize: float
    feeRange: List[FeeRate]
    medianFee: FeeRate
    nTx: int
    totalFees: Sats

# Virtual size in vbytes (weight / 4, rounded up)
VSize = int
class MempoolInfo(TypedDict):
    """
    Mempool statistics

    Attributes:
        count: Number of transactions in the mempool
        total_fee: Total fees of all transactions in the mempool (satoshis)
        vsize: Total virtual size of all transactions in the mempool (vbytes)
    """
    count: int
    total_fee: Sats
    vsize: VSize

# Metric name
Metric = str
class MetricCount(TypedDict):
    """
    Metric count statistics - distinct metrics and total metric-index combinations

    Attributes:
        distinct_metrics: Number of unique metrics available (e.g., realized_price, market_cap)
        lazy_endpoints: Number of lazy (computed on-the-fly) metric-index combinations
        stored_endpoints: Number of eager (stored on disk) metric-index combinations
        total_endpoints: Total number of metric-index combinations across all timeframes
    """
    distinct_metrics: int
    lazy_endpoints: int
    stored_endpoints: int
    total_endpoints: int

class MetricParam(TypedDict):
    metric: Metric

# Comma-separated list of metric names
Metrics = str
class MetricSelection(TypedDict):
    """
    Selection of metrics to query

    Attributes:
        count: Number of values to return (ignored if `to` is set)
        format: Format of the output
        from_: Inclusive starting index, if negative counts from end
        index: Index to query
        metrics: Requested metrics
        to: Exclusive ending index, if negative counts from end
    """
    count: Optional[int]
    format: Format
    from_: Optional[int]
    index: Index
    metrics: Metrics
    to: Optional[int]

class MetricSelectionLegacy(TypedDict):
    """
    Legacy metric selection parameters (deprecated)

    Attributes:
        count: Number of values to return (ignored if `to` is set)
        format: Format of the output
        from_: Inclusive starting index, if negative counts from end
        to: Exclusive ending index, if negative counts from end
    """
    count: Optional[int]
    format: Format
    from_: Optional[int]
    ids: Metrics
    index: Index
    to: Optional[int]

class MetricWithIndex(TypedDict):
    """
    Attributes:
        index: Aggregation index
        metric: Metric name
    """
    index: Index
    metric: Metric

MonthIndex = int
# Opening price value for a time period
Open = Cents
class OHLCCents(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in cents
    """
    close: Close
    high: High
    low: Low
    open: Open

class OHLCDollars(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in dollars
    """
    close: Close
    high: High
    low: Low
    open: Open

class OHLCSats(TypedDict):
    """
    OHLC (Open, High, Low, Close) data in satoshis
    """
    close: Close
    high: High
    low: Low
    open: Open

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
        _1w: Blocks mined in last week
        _24h: Blocks mined in last 24 hours
        all: Total blocks mined (all time)
    """
    _1w: int
    _24h: int
    all: int

class PoolBlockShares(TypedDict):
    """
    Pool's share of total blocks for different time periods

    Attributes:
        _1w: Share of blocks in last week
        _24h: Share of blocks in last 24 hours
        all: Share of all blocks (0.0 - 1.0)
    """
    _1w: float
    _24h: float
    all: float

PoolSlug = Literal["unknown", "blockfills", "ultimuspool", "terrapool", "luxor", "onethash", "btccom", "bitfarms", "huobipool", "wayicn", "canoepool", "btctop", "bitcoincom", "pool175btc", "gbminers", "axbt", "asicminer", "bitminter", "bitcoinrussia", "btcserv", "simplecoinus", "btcguild", "eligius", "ozcoin", "eclipsemc", "maxbtc", "triplemining", "coinlab", "pool50btc", "ghashio", "stminingcorp", "bitparking", "mmpool", "polmine", "kncminer", "bitalo", "f2pool", "hhtt", "megabigpower", "mtred", "nmcbit", "yourbtcnet", "givemecoins", "braiinspool", "antpool", "multicoinco", "bcpoolio", "cointerra", "kanopool", "solock", "ckpool", "nicehash", "bitclub", "bitcoinaffiliatenetwork", "btcc", "bwpool", "exxbw", "bitsolo", "bitfury", "twentyoneinc", "digitalbtc", "eightbaochi", "mybtccoinpool", "tbdice", "hashpool", "nexious", "bravomining", "hotpool", "okexpool", "bcmonster", "onehash", "bixin", "tatmaspool", "viabtc", "connectbtc", "batpool", "waterhole", "dcexploration", "dcex", "btpool", "fiftyeightcoin", "bitcoinindia", "shawnp0wers", "phashio", "rigpool", "haozhuzhu", "sevenpool", "miningkings", "hashbx", "dpool", "rawpool", "haominer", "helix", "bitcoinukraine", "poolin", "secretsuperstar", "tigerpoolnet", "sigmapoolcom", "okpooltop", "hummerpool", "tangpool", "bytepool", "spiderpool", "novablock", "miningcity", "binancepool", "minerium", "lubiancom", "okkong", "aaopool", "emcdpool", "foundryusa", "sbicrypto", "arkpool", "purebtccom", "marapool", "kucoinpool", "entrustcharitypool", "okminer", "titan", "pegapool", "btcnuggets", "cloudhashing", "digitalxmintsy", "telco214", "btcpoolparty", "multipool", "transactioncoinmining", "btcdig", "trickysbtcpool", "btcmp", "eobot", "unomp", "patels", "gogreenlight", "ekanembtc", "canoe", "tiger", "onem1x", "zulupool", "secpool", "ocean", "whitepool", "wk057", "futurebitapollosolo", "carbonnegative", "portlandhodl", "phoenix", "neopool", "maxipool", "bitfufupool", "luckypool", "miningdutch", "publicpool", "miningsquared", "innopolistech", "btclab", "parasite"]
class PoolDetailInfo(TypedDict):
    """
    Pool information for detail view

    Attributes:
        addresses: Known payout addresses
        id: Unique pool identifier
        link: Pool website URL
        name: Pool name
        regexes: Coinbase tag patterns (regexes)
        slug: URL-friendly pool identifier
    """
    addresses: List[str]
    id: int
    link: str
    name: str
    regexes: List[str]
    slug: PoolSlug

class PoolDetail(TypedDict):
    """
    Detailed pool information with statistics across time periods

    Attributes:
        blockCount: Block counts for different time periods
        blockShare: Pool's share of total blocks for different time periods
        estimatedHashrate: Estimated hashrate based on blocks mined
        pool: Pool information
        reportedHashrate: Self-reported hashrate (if available)
    """
    blockCount: PoolBlockCounts
    blockShare: PoolBlockShares
    estimatedHashrate: int
    pool: PoolDetailInfo
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
        blockCount: Number of blocks mined in the time period
        emptyBlocks: Number of empty blocks mined
        link: Pool website URL
        name: Pool name
        poolId: Unique pool identifier
        rank: Pool ranking by block count (1 = most blocks)
        share: Pool's share of total blocks (0.0 - 1.0)
        slug: URL-friendly pool identifier
    """
    blockCount: int
    emptyBlocks: int
    link: str
    name: str
    poolId: int
    rank: int
    share: float
    slug: PoolSlug

class PoolsSummary(TypedDict):
    """
    Mining pools response for a time period

    Attributes:
        blockCount: Total blocks in the time period
        lastEstimatedHashrate: Estimated network hashrate (hashes per second)
        pools: List of pools sorted by block count descending
    """
    blockCount: int
    lastEstimatedHashrate: int
    pools: List[PoolStats]

QuarterIndex = int
# Transaction locktime
RawLockTime = int
class RecommendedFees(TypedDict):
    """
    Recommended fee rates in sat/vB

    Attributes:
        economyFee: Fee rate for economical confirmation
        fastestFee: Fee rate for fastest confirmation (next block)
        halfHourFee: Fee rate for confirmation within ~30 minutes (3 blocks)
        hourFee: Fee rate for confirmation within ~1 hour (6 blocks)
        minimumFee: Minimum relay fee rate
    """
    economyFee: FeeRate
    fastestFee: FeeRate
    halfHourFee: FeeRate
    hourFee: FeeRate
    minimumFee: FeeRate

class RewardStats(TypedDict):
    """
    Block reward statistics over a range of blocks

    Attributes:
        endBlock: Last block in the range
        startBlock: First block in the range
    """
    endBlock: Height
    startBlock: Height
    totalFee: Sats
    totalReward: Sats
    totalTx: int

SemesterIndex = int
# Fixed-size boolean value optimized for on-disk storage (stored as u16)
StoredBool = int
# Stored 32-bit floating point value
StoredF32 = float
# Fixed-size 64-bit floating point value optimized for on-disk storage
StoredF64 = float
StoredI16 = int
StoredU16 = int
# Fixed-size 32-bit unsigned integer optimized for on-disk storage
StoredU32 = int
# Fixed-size 64-bit unsigned integer optimized for on-disk storage
StoredU64 = int
class SupplyState(TypedDict):
    """
    Current supply state tracking UTXO count and total value

    Attributes:
        utxo_count: Number of unspent transaction outputs
        value: Total value in satoshis
    """
    utxo_count: int
    value: Sats

# Time period for mining statistics.
#
# Used to specify the lookback window for pool statistics, hashrate calculations,
# and other time-based mining metrics.
TimePeriod = Literal["24h", "3d", "1w", "1m", "3m", "6m", "1y", "2y", "3y"]
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

# Index of the output being spent in the previous transaction
Vout = int
class TxIn(TypedDict):
    """
    Transaction input

    Attributes:
        inner_redeemscript_asm: Inner redeemscript in assembly format (for P2SH-wrapped SegWit)
        is_coinbase: Whether this input is a coinbase (block reward) input
        prevout: Information about the previous output being spent
        scriptsig: Signature script (for non-SegWit inputs)
        scriptsig_asm: Signature script in assembly format
        sequence: Input sequence number
        txid: Transaction ID of the output being spent
    """
    inner_redeemscript_asm: Optional[str]
    is_coinbase: bool
    prevout: Union[TxOut, None]
    scriptsig: str
    scriptsig_asm: str
    sequence: int
    txid: Txid
    vout: Vout

class TxStatus(TypedDict):
    """
    Transaction confirmation status

    Attributes:
        block_hash: Block hash (only present if confirmed)
        block_height: Block height (only present if confirmed)
        block_time: Block timestamp (only present if confirmed)
        confirmed: Whether the transaction is confirmed
    """
    block_hash: Union[BlockHash, None]
    block_height: Union[Height, None]
    block_time: Union[Timestamp, None]
    confirmed: bool

# Transaction version number
TxVersion = int
class Transaction(TypedDict):
    """
    Transaction information compatible with mempool.space API format

    Attributes:
        fee: Transaction fee in satoshis
        sigops: Number of signature operations
        size: Transaction size in bytes
        vin: Transaction inputs
        vout: Transaction outputs
        weight: Transaction weight
    """
    fee: Sats
    index: Union[TxIndex, None]
    locktime: RawLockTime
    sigops: int
    size: int
    status: TxStatus
    txid: Txid
    version: TxVersion
    vin: List[TxIn]
    vout: List[TxOut]
    weight: Weight

TxInIndex = int
TxOutIndex = int
# Input index in the spending transaction
Vin = int
class TxOutspend(TypedDict):
    """
    Status of an output indicating whether it has been spent

    Attributes:
        spent: Whether the output has been spent
        status: Status of the spending transaction (only present if spent)
        txid: Transaction ID of the spending transaction (only present if spent)
        vin: Input index in the spending transaction (only present if spent)
    """
    spent: bool
    status: Union[TxStatus, None]
    txid: Union[Txid, None]
    vin: Union[Vin, None]

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

UnknownOutputIndex = TypeIndex
class Utxo(TypedDict):
    """
    Unspent transaction output
    """
    status: TxStatus
    txid: Txid
    value: Sats
    vout: Vout

class ValidateAddressParam(TypedDict):
    """
    Attributes:
        address: Bitcoin address to validate (can be any string)
    """
    address: str

WeekIndex = int
YearIndex = int
# Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
# block-based (height, txindex), and address/output type indexes.
Index = Literal["dateindex", "decadeindex", "difficultyepoch", "emptyoutputindex", "halvingepoch", "height", "txinindex", "monthindex", "opreturnindex", "txoutindex", "p2aaddressindex", "p2msoutputindex", "p2pk33addressindex", "p2pk65addressindex", "p2pkhaddressindex", "p2shaddressindex", "p2traddressindex", "p2wpkhaddressindex", "p2wshaddressindex", "quarterindex", "semesterindex", "txindex", "unknownoutputindex", "weekindex", "yearindex", "loadedaddressindex", "emptyaddressindex"]
class MetricLeafWithSchema(TypedDict):
    """
    MetricLeaf with JSON Schema for client generation

    Attributes:
        indexes: Available indexes for this metric
        kind: The Rust type (e.g., "Sats", "StoredF64")
        name: The metric name/identifier
        type: JSON Schema type (e.g., "integer", "number", "string", "boolean", "array", "object")
    """
    indexes: List[Index]
    kind: str
    name: str
    type: str

# Hierarchical tree node for organizing metrics into categories
TreeNode = Union[dict[str, "TreeNode"], MetricLeafWithSchema]

class BrkError(Exception):
    """Custom error class for BRK client errors."""

    def __init__(self, message: str, status: Optional[int] = None):
        super().__init__(message)
        self.status = status


class BrkClientBase:
    """Base HTTP client for making requests."""

    def __init__(self, base_url: str, timeout: float = 30.0):
        self.base_url = base_url
        self.timeout = timeout
        self._client = httpx.Client(timeout=timeout)

    def get(self, path: str) -> Any:
        """Make a GET request."""
        try:
            base = self.base_url.rstrip('/')
            response = self._client.get(f"{base}{path}")
            response.raise_for_status()
            return response.json()
        except httpx.HTTPStatusError as e:
            raise BrkError(f"HTTP error: {e.response.status_code}", e.response.status_code)
        except httpx.RequestError as e:
            raise BrkError(str(e))

    def close(self):
        """Close the HTTP client."""
        self._client.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


def _m(acc: str, s: str) -> str:
    """Build metric name with optional prefix."""
    return f"{acc}_{s}" if acc else s


class MetricData(TypedDict, Generic[T]):
    """Metric data with range information."""
    total: int
    from_: int  # 'from' is reserved in Python
    to: int
    data: List[T]


# Type alias for non-generic usage
AnyMetricData = MetricData[Any]


class MetricEndpoint(Generic[T]):
    """An endpoint for a specific metric + index combination."""

    def __init__(self, client: BrkClientBase, name: str, index: str):
        self._client = client
        self._name = name
        self._index = index

    def get(self) -> MetricData[T]:
        """Fetch all data points for this metric/index."""
        return self._client.get(self.path())

    def range(self, from_val: Optional[int] = None, to_val: Optional[int] = None) -> MetricData[T]:
        """Fetch data points within a range."""
        params = []
        if from_val is not None:
            params.append(f"from={from_val}")
        if to_val is not None:
            params.append(f"to={to_val}")
        query = "&".join(params)
        p = self.path()
        return self._client.get(f"{p}?{query}" if query else p)

    def path(self) -> str:
        """Get the endpoint path."""
        return f"/api/metric/{self._name}/{self._index}"


# Type alias for non-generic usage
AnyMetricEndpoint = MetricEndpoint[Any]


class MetricPattern(Protocol[T]):
    """Protocol for metric patterns with different index sets."""

    @property
    def name(self) -> str:
        """Get the metric name."""
        ...

    def indexes(self) -> List[str]:
        """Get the list of available indexes for this metric."""
        ...

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        ...


# Index accessor classes

class _MetricPattern1By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

    def height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

    def monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern1(Generic[T]):
    """Index accessor for metrics with 9 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern1By[T] = _MetricPattern1By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex', 'decadeindex', 'difficultyepoch', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        elif index == 'decadeindex': return self.by.decadeindex()
        elif index == 'difficultyepoch': return self.by.difficultyepoch()
        elif index == 'height': return self.by.height()
        elif index == 'monthindex': return self.by.monthindex()
        elif index == 'quarterindex': return self.by.quarterindex()
        elif index == 'semesterindex': return self.by.semesterindex()
        elif index == 'weekindex': return self.by.weekindex()
        elif index == 'yearindex': return self.by.yearindex()
        return None

class _MetricPattern2By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

    def monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern2(Generic[T]):
    """Index accessor for metrics with 8 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern2By[T] = _MetricPattern2By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex', 'decadeindex', 'difficultyepoch', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        elif index == 'decadeindex': return self.by.decadeindex()
        elif index == 'difficultyepoch': return self.by.difficultyepoch()
        elif index == 'monthindex': return self.by.monthindex()
        elif index == 'quarterindex': return self.by.quarterindex()
        elif index == 'semesterindex': return self.by.semesterindex()
        elif index == 'weekindex': return self.by.weekindex()
        elif index == 'yearindex': return self.by.yearindex()
        return None

class _MetricPattern3By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

    def monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern3(Generic[T]):
    """Index accessor for metrics with 8 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern3By[T] = _MetricPattern3By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex', 'decadeindex', 'height', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        elif index == 'decadeindex': return self.by.decadeindex()
        elif index == 'height': return self.by.height()
        elif index == 'monthindex': return self.by.monthindex()
        elif index == 'quarterindex': return self.by.quarterindex()
        elif index == 'semesterindex': return self.by.semesterindex()
        elif index == 'weekindex': return self.by.weekindex()
        elif index == 'yearindex': return self.by.yearindex()
        return None

class _MetricPattern4By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

    def monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

    def quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

    def semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

    def weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

    def yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern4(Generic[T]):
    """Index accessor for metrics with 7 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern4By[T] = _MetricPattern4By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex', 'decadeindex', 'monthindex', 'quarterindex', 'semesterindex', 'weekindex', 'yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        elif index == 'decadeindex': return self.by.decadeindex()
        elif index == 'monthindex': return self.by.monthindex()
        elif index == 'quarterindex': return self.by.quarterindex()
        elif index == 'semesterindex': return self.by.semesterindex()
        elif index == 'weekindex': return self.by.weekindex()
        elif index == 'yearindex': return self.by.yearindex()
        return None

class _MetricPattern5By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

    def height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

class MetricPattern5(Generic[T]):
    """Index accessor for metrics with 2 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern5By[T] = _MetricPattern5By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex', 'height']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        elif index == 'height': return self.by.height()
        return None

class _MetricPattern6By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def dateindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'dateindex')

class MetricPattern6(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern6By[T] = _MetricPattern6By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['dateindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'dateindex': return self.by.dateindex()
        return None

class _MetricPattern7By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def decadeindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'decadeindex')

class MetricPattern7(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern7By[T] = _MetricPattern7By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['decadeindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'decadeindex': return self.by.decadeindex()
        return None

class _MetricPattern8By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def difficultyepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'difficultyepoch')

class MetricPattern8(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern8By[T] = _MetricPattern8By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['difficultyepoch']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'difficultyepoch': return self.by.difficultyepoch()
        return None

class _MetricPattern9By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def emptyoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'emptyoutputindex')

class MetricPattern9(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern9By[T] = _MetricPattern9By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['emptyoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyoutputindex': return self.by.emptyoutputindex()
        return None

class _MetricPattern10By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def halvingepoch(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'halvingepoch')

class MetricPattern10(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern10By[T] = _MetricPattern10By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['halvingepoch']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'halvingepoch': return self.by.halvingepoch()
        return None

class _MetricPattern11By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def height(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'height')

class MetricPattern11(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern11By[T] = _MetricPattern11By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['height']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'height': return self.by.height()
        return None

class _MetricPattern12By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def txinindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txinindex')

class MetricPattern12(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern12By[T] = _MetricPattern12By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['txinindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txinindex': return self.by.txinindex()
        return None

class _MetricPattern13By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def monthindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'monthindex')

class MetricPattern13(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern13By[T] = _MetricPattern13By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['monthindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'monthindex': return self.by.monthindex()
        return None

class _MetricPattern14By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def opreturnindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'opreturnindex')

class MetricPattern14(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern14By[T] = _MetricPattern14By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['opreturnindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'opreturnindex': return self.by.opreturnindex()
        return None

class _MetricPattern15By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def txoutindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txoutindex')

class MetricPattern15(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern15By[T] = _MetricPattern15By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['txoutindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txoutindex': return self.by.txoutindex()
        return None

class _MetricPattern16By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2aaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2aaddressindex')

class MetricPattern16(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern16By[T] = _MetricPattern16By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2aaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2aaddressindex': return self.by.p2aaddressindex()
        return None

class _MetricPattern17By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2msoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2msoutputindex')

class MetricPattern17(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern17By[T] = _MetricPattern17By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2msoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2msoutputindex': return self.by.p2msoutputindex()
        return None

class _MetricPattern18By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2pk33addressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pk33addressindex')

class MetricPattern18(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern18By[T] = _MetricPattern18By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2pk33addressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk33addressindex': return self.by.p2pk33addressindex()
        return None

class _MetricPattern19By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2pk65addressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pk65addressindex')

class MetricPattern19(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern19By[T] = _MetricPattern19By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2pk65addressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pk65addressindex': return self.by.p2pk65addressindex()
        return None

class _MetricPattern20By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2pkhaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2pkhaddressindex')

class MetricPattern20(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern20By[T] = _MetricPattern20By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2pkhaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2pkhaddressindex': return self.by.p2pkhaddressindex()
        return None

class _MetricPattern21By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2shaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2shaddressindex')

class MetricPattern21(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern21By[T] = _MetricPattern21By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2shaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2shaddressindex': return self.by.p2shaddressindex()
        return None

class _MetricPattern22By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2traddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2traddressindex')

class MetricPattern22(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern22By[T] = _MetricPattern22By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2traddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2traddressindex': return self.by.p2traddressindex()
        return None

class _MetricPattern23By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2wpkhaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2wpkhaddressindex')

class MetricPattern23(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern23By[T] = _MetricPattern23By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2wpkhaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wpkhaddressindex': return self.by.p2wpkhaddressindex()
        return None

class _MetricPattern24By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def p2wshaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'p2wshaddressindex')

class MetricPattern24(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern24By[T] = _MetricPattern24By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['p2wshaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'p2wshaddressindex': return self.by.p2wshaddressindex()
        return None

class _MetricPattern25By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def quarterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'quarterindex')

class MetricPattern25(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern25By[T] = _MetricPattern25By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['quarterindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'quarterindex': return self.by.quarterindex()
        return None

class _MetricPattern26By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def semesterindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'semesterindex')

class MetricPattern26(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern26By[T] = _MetricPattern26By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['semesterindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'semesterindex': return self.by.semesterindex()
        return None

class _MetricPattern27By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def txindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'txindex')

class MetricPattern27(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern27By[T] = _MetricPattern27By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['txindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'txindex': return self.by.txindex()
        return None

class _MetricPattern28By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def unknownoutputindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'unknownoutputindex')

class MetricPattern28(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern28By[T] = _MetricPattern28By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['unknownoutputindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'unknownoutputindex': return self.by.unknownoutputindex()
        return None

class _MetricPattern29By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def weekindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'weekindex')

class MetricPattern29(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern29By[T] = _MetricPattern29By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['weekindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'weekindex': return self.by.weekindex()
        return None

class _MetricPattern30By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def yearindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'yearindex')

class MetricPattern30(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern30By[T] = _MetricPattern30By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['yearindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'yearindex': return self.by.yearindex()
        return None

class _MetricPattern31By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def loadedaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'loadedaddressindex')

class MetricPattern31(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern31By[T] = _MetricPattern31By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['loadedaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'loadedaddressindex': return self.by.loadedaddressindex()
        return None

class _MetricPattern32By(Generic[T]):
    """Index endpoint methods container."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name

    def emptyaddressindex(self) -> MetricEndpoint[T]:
        return MetricEndpoint(self._client, self._name, 'emptyaddressindex')

class MetricPattern32(Generic[T]):
    """Index accessor for metrics with 1 indexes."""

    def __init__(self, client: BrkClientBase, name: str):
        self._client = client
        self._name = name
        self.by: _MetricPattern32By[T] = _MetricPattern32By(client, name)

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    def indexes(self) -> List[str]:
        """Get the list of available indexes."""
        return ['emptyaddressindex']

    def get(self, index: str) -> Optional[MetricEndpoint[T]]:
        """Get an endpoint for a specific index, if supported."""
        if index == 'emptyaddressindex': return self.by.emptyaddressindex()
        return None

# Reusable structural pattern classes

class RealizedPattern3:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class RealizedPattern4:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.adjusted_sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr'))
        self.adjusted_sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_30d_ema'))
        self.adjusted_sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'adjusted_sopr_7d_ema'))
        self.adjusted_value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_created'))
        self.adjusted_value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'adjusted_value_destroyed'))
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RealizedPriceExtraPattern = RealizedPriceExtraPattern(client, _m(acc, 'realized_price'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class Ratio1ySdPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._0sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, '0sd_usd'))
        self.m0_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm0_5sd'))
        self.m0_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm0_5sd_usd'))
        self.m1_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm1_5sd'))
        self.m1_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm1_5sd_usd'))
        self.m1sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm1sd'))
        self.m1sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm1sd_usd'))
        self.m2_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm2_5sd'))
        self.m2_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm2_5sd_usd'))
        self.m2sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm2sd'))
        self.m2sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm2sd_usd'))
        self.m3sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'm3sd'))
        self.m3sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'm3sd_usd'))
        self.p0_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p0_5sd'))
        self.p0_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p0_5sd_usd'))
        self.p1_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p1_5sd'))
        self.p1_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p1_5sd_usd'))
        self.p1sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p1sd'))
        self.p1sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p1sd_usd'))
        self.p2_5sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p2_5sd'))
        self.p2_5sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p2_5sd_usd'))
        self.p2sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p2sd'))
        self.p2sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p2sd_usd'))
        self.p3sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'p3sd'))
        self.p3sd_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'p3sd_usd'))
        self.sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sd'))
        self.sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sma'))
        self.zscore: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'zscore'))

class RealizedPattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_cap_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'realized_cap_rel_to_own_market_cap'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: ActivePriceRatioPattern = ActivePriceRatioPattern(client, _m(acc, 'realized_price_ratio'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_profit_to_loss_ratio: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'realized_profit_to_loss_ratio'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class RealizedPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.mvrv: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'mvrv'))
        self.neg_realized_loss: BitcoinPattern[Dollars] = BitcoinPattern(client, _m(acc, 'neg_realized_loss'))
        self.net_realized_pnl: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'net_realized_pnl'))
        self.net_realized_pnl_cumulative_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_market_cap'))
        self.net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap'))
        self.net_realized_pnl_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'net_realized_pnl_rel_to_realized_cap'))
        self.realized_cap: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_cap'))
        self.realized_cap_30d_delta: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'realized_cap_30d_delta'))
        self.realized_loss: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_loss'))
        self.realized_loss_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_loss_rel_to_realized_cap'))
        self.realized_price: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_price'))
        self.realized_price_extra: RealizedPriceExtraPattern = RealizedPriceExtraPattern(client, _m(acc, 'realized_price'))
        self.realized_profit: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'realized_profit'))
        self.realized_profit_rel_to_realized_cap: BlockCountPattern[StoredF32] = BlockCountPattern(client, _m(acc, 'realized_profit_rel_to_realized_cap'))
        self.realized_value: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'realized_value'))
        self.sell_side_risk_ratio: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio'))
        self.sell_side_risk_ratio_30d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_30d_ema'))
        self.sell_side_risk_ratio_7d_ema: MetricPattern6[StoredF32] = MetricPattern6(client, _m(acc, 'sell_side_risk_ratio_7d_ema'))
        self.sopr: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr'))
        self.sopr_30d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_30d_ema'))
        self.sopr_7d_ema: MetricPattern6[StoredF64] = MetricPattern6(client, _m(acc, 'sopr_7d_ema'))
        self.total_realized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_realized_pnl'))
        self.value_created: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_created'))
        self.value_destroyed: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'value_destroyed'))

class Price111dSmaPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.price: MetricPattern4[Dollars] = MetricPattern4(client, acc)
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio'))
        self.ratio_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_1m_sma'))
        self.ratio_1w_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_1w_sma'))
        self.ratio_1y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, 'ratio_1y'))
        self.ratio_2y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, 'ratio_2y'))
        self.ratio_4y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, 'ratio_4y'))
        self.ratio_pct1: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct1'))
        self.ratio_pct1_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct1_usd'))
        self.ratio_pct2: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct2'))
        self.ratio_pct2_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct2_usd'))
        self.ratio_pct5: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct5'))
        self.ratio_pct5_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct5_usd'))
        self.ratio_pct95: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct95'))
        self.ratio_pct95_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct95_usd'))
        self.ratio_pct98: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct98'))
        self.ratio_pct98_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct98_usd'))
        self.ratio_pct99: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio_pct99'))
        self.ratio_pct99_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'ratio_pct99_usd'))
        self.ratio_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, 'ratio'))

class ActivePriceRatioPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, acc)
        self.ratio_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1m_sma'))
        self.ratio_1w_sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, '1w_sma'))
        self.ratio_1y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, '1y'))
        self.ratio_2y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, '2y'))
        self.ratio_4y_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, _m(acc, '4y'))
        self.ratio_pct1: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct1'))
        self.ratio_pct1_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct1_usd'))
        self.ratio_pct2: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct2'))
        self.ratio_pct2_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct2_usd'))
        self.ratio_pct5: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct5'))
        self.ratio_pct5_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct5_usd'))
        self.ratio_pct95: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct95'))
        self.ratio_pct95_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct95_usd'))
        self.ratio_pct98: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct98'))
        self.ratio_pct98_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct98_usd'))
        self.ratio_pct99: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'pct99'))
        self.ratio_pct99_usd: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct99_usd'))
        self.ratio_sd: Ratio1ySdPattern = Ratio1ySdPattern(client, acc)

class PercentilesPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cost_basis_pct05: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct05'))
        self.cost_basis_pct10: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct10'))
        self.cost_basis_pct15: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct15'))
        self.cost_basis_pct20: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct20'))
        self.cost_basis_pct25: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct25'))
        self.cost_basis_pct30: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct30'))
        self.cost_basis_pct35: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct35'))
        self.cost_basis_pct40: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct40'))
        self.cost_basis_pct45: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct45'))
        self.cost_basis_pct50: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct50'))
        self.cost_basis_pct55: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct55'))
        self.cost_basis_pct60: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct60'))
        self.cost_basis_pct65: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct65'))
        self.cost_basis_pct70: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct70'))
        self.cost_basis_pct75: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct75'))
        self.cost_basis_pct80: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct80'))
        self.cost_basis_pct85: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct85'))
        self.cost_basis_pct90: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct90'))
        self.cost_basis_pct95: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'pct95'))

class RelativePattern5:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
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
        self.unrealized_profit_rel_to_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_market_cap'))
        self.unrealized_profit_rel_to_own_market_cap: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_market_cap'))
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'unrealized_profit_rel_to_own_total_unrealized_pnl'))

class AaopoolPattern:
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
        self.blocks_mined: BlockCountPattern[StoredU32] = BlockCountPattern(client, _m(acc, 'blocks_mined'))
        self.coinbase: CoinbasePattern2 = CoinbasePattern2(client, _m(acc, 'coinbase'))
        self.days_since_block: MetricPattern4[StoredU16] = MetricPattern4(client, _m(acc, 'days_since_block'))
        self.dominance: MetricPattern1[StoredF32] = MetricPattern1(client, _m(acc, 'dominance'))
        self.fee: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'fee'))
        self.subsidy: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'subsidy'))

class PriceAgoPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, base_path: str):
        self._10y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_10y')
        self._1d: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_1d')
        self._1m: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_1m')
        self._1w: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_1w')
        self._1y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_1y')
        self._2y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2y')
        self._3m: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_3m')
        self._3y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_3y')
        self._4y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_4y')
        self._5y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_5y')
        self._6m: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_6m')
        self._6y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_6y')
        self._8y: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_8y')

class PeriodLumpSumStackPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: _2015Pattern = _2015Pattern(client, (f'10y_{acc}' if acc else '10y'))
        self._1m: _2015Pattern = _2015Pattern(client, (f'1m_{acc}' if acc else '1m'))
        self._1w: _2015Pattern = _2015Pattern(client, (f'1w_{acc}' if acc else '1w'))
        self._1y: _2015Pattern = _2015Pattern(client, (f'1y_{acc}' if acc else '1y'))
        self._2y: _2015Pattern = _2015Pattern(client, (f'2y_{acc}' if acc else '2y'))
        self._3m: _2015Pattern = _2015Pattern(client, (f'3m_{acc}' if acc else '3m'))
        self._3y: _2015Pattern = _2015Pattern(client, (f'3y_{acc}' if acc else '3y'))
        self._4y: _2015Pattern = _2015Pattern(client, (f'4y_{acc}' if acc else '4y'))
        self._5y: _2015Pattern = _2015Pattern(client, (f'5y_{acc}' if acc else '5y'))
        self._6m: _2015Pattern = _2015Pattern(client, (f'6m_{acc}' if acc else '6m'))
        self._6y: _2015Pattern = _2015Pattern(client, (f'6y_{acc}' if acc else '6y'))
        self._8y: _2015Pattern = _2015Pattern(client, (f'8y_{acc}' if acc else '8y'))

class PeriodAveragePricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[T] = MetricPattern4(client, (f'10y_{acc}' if acc else '10y'))
        self._1m: MetricPattern4[T] = MetricPattern4(client, (f'1m_{acc}' if acc else '1m'))
        self._1w: MetricPattern4[T] = MetricPattern4(client, (f'1w_{acc}' if acc else '1w'))
        self._1y: MetricPattern4[T] = MetricPattern4(client, (f'1y_{acc}' if acc else '1y'))
        self._2y: MetricPattern4[T] = MetricPattern4(client, (f'2y_{acc}' if acc else '2y'))
        self._3m: MetricPattern4[T] = MetricPattern4(client, (f'3m_{acc}' if acc else '3m'))
        self._3y: MetricPattern4[T] = MetricPattern4(client, (f'3y_{acc}' if acc else '3y'))
        self._4y: MetricPattern4[T] = MetricPattern4(client, (f'4y_{acc}' if acc else '4y'))
        self._5y: MetricPattern4[T] = MetricPattern4(client, (f'5y_{acc}' if acc else '5y'))
        self._6m: MetricPattern4[T] = MetricPattern4(client, (f'6m_{acc}' if acc else '6m'))
        self._6y: MetricPattern4[T] = MetricPattern4(client, (f'6y_{acc}' if acc else '6y'))
        self._8y: MetricPattern4[T] = MetricPattern4(client, (f'8y_{acc}' if acc else '8y'))

class DollarsPattern(Generic[T]):
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

class ClassAveragePricePattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, base_path: str):
        self._2015: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2015')
        self._2016: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2016')
        self._2017: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2017')
        self._2018: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2018')
        self._2019: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2019')
        self._2020: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2020')
        self._2021: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2021')
        self._2022: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2022')
        self._2023: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2023')
        self._2024: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2024')
        self._2025: MetricPattern4[T] = MetricPattern4(client, f'{base_path}_2025')

class FullnessPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.average: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'average'))
        self.base: MetricPattern11[T] = MetricPattern11(client, acc)
        self.cumulative: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.max: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'max'))
        self.median: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'median'))
        self.min: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'min'))
        self.pct10: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct10'))
        self.pct25: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct25'))
        self.pct75: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct75'))
        self.pct90: MetricPattern6[T] = MetricPattern6(client, _m(acc, 'pct90'))
        self.sum: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'sum'))

class RelativePattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
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

class RelativePattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
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

class CountPattern2(Generic[T]):
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

class AddrCountPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, base_path: str):
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_all')
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2a')
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2pk33')
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2pk65')
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2pkh')
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2sh')
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2tr')
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2wpkh')
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, f'{base_path}_p2wsh')

class FeeRatePattern(Generic[T]):
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

class _0satsPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.addr_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'addr_count'))
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.outputs: OutputsPattern = OutputsPattern(client, acc)
        self.realized: RealizedPattern = RealizedPattern(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class _10yTo12yPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern2 = CostBasisPattern2(client, acc)
        self.outputs: OutputsPattern = OutputsPattern(client, acc)
        self.realized: RealizedPattern2 = RealizedPattern2(client, acc)
        self.relative: RelativePattern2 = RelativePattern2(client, acc)
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class PeriodCagrPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self._10y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'10y_{acc}' if acc else '10y'))
        self._2y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'2y_{acc}' if acc else '2y'))
        self._3y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'3y_{acc}' if acc else '3y'))
        self._4y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'4y_{acc}' if acc else '4y'))
        self._5y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'5y_{acc}' if acc else '5y'))
        self._6y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'6y_{acc}' if acc else '6y'))
        self._8y: MetricPattern4[StoredF32] = MetricPattern4(client, (f'8y_{acc}' if acc else '8y'))

class _0satsPattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.outputs: OutputsPattern = OutputsPattern(client, acc)
        self.realized: RealizedPattern = RealizedPattern(client, acc)
        self.relative: RelativePattern4 = RelativePattern4(client, _m(acc, 'supply_in'))
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class UnrealizedPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.neg_unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'neg_unrealized_loss'))
        self.net_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'net_unrealized_pnl'))
        self.supply_in_loss: ActiveSupplyPattern = ActiveSupplyPattern(client, _m(acc, 'supply_in_loss'))
        self.supply_in_profit: ActiveSupplyPattern = ActiveSupplyPattern(client, _m(acc, 'supply_in_profit'))
        self.total_unrealized_pnl: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'total_unrealized_pnl'))
        self.unrealized_loss: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_loss'))
        self.unrealized_profit: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'unrealized_profit'))

class _100btcPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.outputs: OutputsPattern = OutputsPattern(client, acc)
        self.realized: RealizedPattern = RealizedPattern(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class _10yPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.activity: ActivityPattern2 = ActivityPattern2(client, acc)
        self.cost_basis: CostBasisPattern = CostBasisPattern(client, acc)
        self.outputs: OutputsPattern = OutputsPattern(client, acc)
        self.realized: RealizedPattern4 = RealizedPattern4(client, acc)
        self.relative: RelativePattern = RelativePattern(client, acc)
        self.supply: SupplyPattern2 = SupplyPattern2(client, _m(acc, 'supply'))
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, acc)

class ActivityPattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.coinblocks_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coinblocks_destroyed'))
        self.coindays_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, _m(acc, 'coindays_destroyed'))
        self.satblocks_destroyed: MetricPattern11[Sats] = MetricPattern11(client, _m(acc, 'satblocks_destroyed'))
        self.satdays_destroyed: MetricPattern11[Sats] = MetricPattern11(client, _m(acc, 'satdays_destroyed'))
        self.sent: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, _m(acc, 'sent'))

class SplitPattern2(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.close: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'close'))
        self.high: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'high'))
        self.low: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'low'))
        self.open: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'open'))

class CoinbasePattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: FullnessPattern[Bitcoin] = FullnessPattern(client, _m(acc, 'btc'))
        self.dollars: DollarsPattern[Dollars] = DollarsPattern(client, _m(acc, 'usd'))
        self.sats: DollarsPattern[Sats] = DollarsPattern(client, acc)

class ActiveSupplyPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern1[Bitcoin] = MetricPattern1(client, _m(acc, 'btc'))
        self.dollars: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'usd'))
        self.sats: MetricPattern1[Sats] = MetricPattern1(client, acc)

class UnclaimedRewardsPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BitcoinPattern[Bitcoin] = BitcoinPattern(client, _m(acc, 'btc'))
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'usd'))
        self.sats: BlockCountPattern[Sats] = BlockCountPattern(client, acc)

class SegwitAdoptionPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.base: MetricPattern11[StoredF32] = MetricPattern11(client, acc)
        self.cumulative: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern2[StoredF32] = MetricPattern2(client, _m(acc, 'sum'))

class CostBasisPattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, base_path: str):
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_max')
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, f'{base_path}_min')
        self.percentiles: PercentilesPattern = PercentilesPattern(client, f'{base_path}_percentiles')

class CoinbasePattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: BlockCountPattern[Bitcoin] = BlockCountPattern(client, _m(acc, 'btc'))
        self.dollars: BlockCountPattern[Dollars] = BlockCountPattern(client, _m(acc, 'usd'))
        self.sats: BlockCountPattern[Sats] = BlockCountPattern(client, acc)

class _2015Pattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.bitcoin: MetricPattern4[Bitcoin] = MetricPattern4(client, _m(acc, 'btc'))
        self.dollars: MetricPattern4[Dollars] = MetricPattern4(client, _m(acc, 'usd'))
        self.sats: MetricPattern4[Sats] = MetricPattern4(client, acc)

class SupplyPattern2:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.halved: ActiveSupplyPattern = ActiveSupplyPattern(client, _m(acc, 'half'))
        self.total: ActiveSupplyPattern = ActiveSupplyPattern(client, acc)

class _1dReturns1mSdPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.sd: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sd'))
        self.sma: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'sma'))

class CostBasisPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'max_cost_basis'))
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, _m(acc, 'min_cost_basis'))

class RelativePattern4:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'loss_rel_to_own_supply'))
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, _m(acc, 'profit_rel_to_own_supply'))

class BitcoinPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern2[T] = MetricPattern2(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, acc)

class SatsPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, base_path: str):
        self.ohlc: MetricPattern1[T] = MetricPattern1(client, f'{base_path}_ohlc')
        self.split: SplitPattern2[Any] = SplitPattern2(client, f'{base_path}_split')

class BlockCountPattern(Generic[T]):
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.cumulative: MetricPattern1[T] = MetricPattern1(client, _m(acc, 'cumulative'))
        self.sum: MetricPattern1[T] = MetricPattern1(client, acc)

class RealizedPriceExtraPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.ratio: MetricPattern4[StoredF32] = MetricPattern4(client, _m(acc, 'ratio'))

class OutputsPattern:
    """Pattern struct for repeated tree structure."""

    def __init__(self, client: BrkClientBase, acc: str):
        """Create pattern node with accumulated metric name."""
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, _m(acc, 'utxo_count'))

# Catalog tree classes

class CatalogTree:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addresses: CatalogTree_Addresses = CatalogTree_Addresses(client)
        self.blocks: CatalogTree_Blocks = CatalogTree_Blocks(client)
        self.cointime: CatalogTree_Cointime = CatalogTree_Cointime(client)
        self.constants: CatalogTree_Constants = CatalogTree_Constants(client)
        self.distribution: CatalogTree_Distribution = CatalogTree_Distribution(client)
        self.indexes: CatalogTree_Indexes = CatalogTree_Indexes(client)
        self.inputs: CatalogTree_Inputs = CatalogTree_Inputs(client)
        self.market: CatalogTree_Market = CatalogTree_Market(client)
        self.outputs: CatalogTree_Outputs = CatalogTree_Outputs(client)
        self.pools: CatalogTree_Pools = CatalogTree_Pools(client)
        self.positions: CatalogTree_Positions = CatalogTree_Positions(client)
        self.price: CatalogTree_Price = CatalogTree_Price(client)
        self.scripts: CatalogTree_Scripts = CatalogTree_Scripts(client)
        self.supply: CatalogTree_Supply = CatalogTree_Supply(client)
        self.transactions: CatalogTree_Transactions = CatalogTree_Transactions(client)

class CatalogTree_Addresses:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_p2aaddressindex: MetricPattern11[P2AAddressIndex] = MetricPattern11(client, 'first_p2aaddressindex')
        self.first_p2pk33addressindex: MetricPattern11[P2PK33AddressIndex] = MetricPattern11(client, 'first_p2pk33addressindex')
        self.first_p2pk65addressindex: MetricPattern11[P2PK65AddressIndex] = MetricPattern11(client, 'first_p2pk65addressindex')
        self.first_p2pkhaddressindex: MetricPattern11[P2PKHAddressIndex] = MetricPattern11(client, 'first_p2pkhaddressindex')
        self.first_p2shaddressindex: MetricPattern11[P2SHAddressIndex] = MetricPattern11(client, 'first_p2shaddressindex')
        self.first_p2traddressindex: MetricPattern11[P2TRAddressIndex] = MetricPattern11(client, 'first_p2traddressindex')
        self.first_p2wpkhaddressindex: MetricPattern11[P2WPKHAddressIndex] = MetricPattern11(client, 'first_p2wpkhaddressindex')
        self.first_p2wshaddressindex: MetricPattern11[P2WSHAddressIndex] = MetricPattern11(client, 'first_p2wshaddressindex')
        self.p2abytes: MetricPattern16[P2ABytes] = MetricPattern16(client, 'p2abytes')
        self.p2pk33bytes: MetricPattern18[P2PK33Bytes] = MetricPattern18(client, 'p2pk33bytes')
        self.p2pk65bytes: MetricPattern19[P2PK65Bytes] = MetricPattern19(client, 'p2pk65bytes')
        self.p2pkhbytes: MetricPattern20[P2PKHBytes] = MetricPattern20(client, 'p2pkhbytes')
        self.p2shbytes: MetricPattern21[P2SHBytes] = MetricPattern21(client, 'p2shbytes')
        self.p2trbytes: MetricPattern22[P2TRBytes] = MetricPattern22(client, 'p2trbytes')
        self.p2wpkhbytes: MetricPattern23[P2WPKHBytes] = MetricPattern23(client, 'p2wpkhbytes')
        self.p2wshbytes: MetricPattern24[P2WSHBytes] = MetricPattern24(client, 'p2wshbytes')

class CatalogTree_Blocks:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blockhash: MetricPattern11[BlockHash] = MetricPattern11(client, 'blockhash')
        self.count: CatalogTree_Blocks_Count = CatalogTree_Blocks_Count(client)
        self.difficulty: CatalogTree_Blocks_Difficulty = CatalogTree_Blocks_Difficulty(client)
        self.fullness: FullnessPattern[StoredF32] = FullnessPattern(client, 'block_fullness')
        self.halving: CatalogTree_Blocks_Halving = CatalogTree_Blocks_Halving(client)
        self.interval: CatalogTree_Blocks_Interval = CatalogTree_Blocks_Interval(client)
        self.mining: CatalogTree_Blocks_Mining = CatalogTree_Blocks_Mining(client)
        self.rewards: CatalogTree_Blocks_Rewards = CatalogTree_Blocks_Rewards(client)
        self.size: CatalogTree_Blocks_Size = CatalogTree_Blocks_Size(client)
        self.time: CatalogTree_Blocks_Time = CatalogTree_Blocks_Time(client)
        self.total_size: MetricPattern11[StoredU64] = MetricPattern11(client, 'total_size')
        self.vbytes: DollarsPattern[StoredU64] = DollarsPattern(client, 'block_vbytes')
        self.weight: DollarsPattern[Weight] = DollarsPattern(client, 'block_weight_average')

class CatalogTree_Blocks_Count:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1m_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1m_block_count')
        self._1m_start: MetricPattern11[Height] = MetricPattern11(client, '1m_start')
        self._1w_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1w_block_count')
        self._1w_start: MetricPattern11[Height] = MetricPattern11(client, '1w_start')
        self._1y_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '1y_block_count')
        self._1y_start: MetricPattern11[Height] = MetricPattern11(client, '1y_start')
        self._24h_block_count: MetricPattern1[StoredU32] = MetricPattern1(client, '24h_block_count')
        self._24h_start: MetricPattern11[Height] = MetricPattern11(client, '24h_start')
        self.block_count: BlockCountPattern[StoredU32] = BlockCountPattern(client, 'block_count')
        self.block_count_target: MetricPattern4[StoredU64] = MetricPattern4(client, 'block_count_target')

class CatalogTree_Blocks_Difficulty:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_adjustment')
        self.as_hash: MetricPattern1[StoredF32] = MetricPattern1(client, 'difficulty_as_hash')
        self.blocks_before_next_adjustment: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_difficulty_adjustment')
        self.days_before_next_adjustment: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_difficulty_adjustment')
        self.epoch: MetricPattern4[DifficultyEpoch] = MetricPattern4(client, 'difficultyepoch')
        self.raw: MetricPattern1[StoredF64] = MetricPattern1(client, 'difficulty')

class CatalogTree_Blocks_Halving:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.blocks_before_next_halving: MetricPattern1[StoredU32] = MetricPattern1(client, 'blocks_before_next_halving')
        self.days_before_next_halving: MetricPattern1[StoredF32] = MetricPattern1(client, 'days_before_next_halving')
        self.epoch: MetricPattern4[HalvingEpoch] = MetricPattern4(client, 'halvingepoch')

class CatalogTree_Blocks_Interval:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern2[Timestamp] = MetricPattern2(client, 'block_interval_average')
        self.base: MetricPattern11[Timestamp] = MetricPattern11(client, 'block_interval')
        self.max: MetricPattern2[Timestamp] = MetricPattern2(client, 'block_interval_max')
        self.median: MetricPattern6[Timestamp] = MetricPattern6(client, 'block_interval_median')
        self.min: MetricPattern2[Timestamp] = MetricPattern2(client, 'block_interval_min')
        self.pct10: MetricPattern6[Timestamp] = MetricPattern6(client, 'block_interval_pct10')
        self.pct25: MetricPattern6[Timestamp] = MetricPattern6(client, 'block_interval_pct25')
        self.pct75: MetricPattern6[Timestamp] = MetricPattern6(client, 'block_interval_pct75')
        self.pct90: MetricPattern6[Timestamp] = MetricPattern6(client, 'block_interval_pct90')

class CatalogTree_Blocks_Mining:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.hash_price_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs')
        self.hash_price_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_phs_min')
        self.hash_price_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_rebound')
        self.hash_price_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths')
        self.hash_price_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_price_ths_min')
        self.hash_rate: MetricPattern1[StoredF64] = MetricPattern1(client, 'hash_rate')
        self.hash_rate_1m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_1m_sma')
        self.hash_rate_1w_sma: MetricPattern4[StoredF64] = MetricPattern4(client, 'hash_rate_1w_sma')
        self.hash_rate_1y_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_1y_sma')
        self.hash_rate_2m_sma: MetricPattern4[StoredF32] = MetricPattern4(client, 'hash_rate_2m_sma')
        self.hash_value_phs: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs')
        self.hash_value_phs_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_phs_min')
        self.hash_value_rebound: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_rebound')
        self.hash_value_ths: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths')
        self.hash_value_ths_min: MetricPattern1[StoredF32] = MetricPattern1(client, 'hash_value_ths_min')

class CatalogTree_Blocks_Rewards:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._24h_coinbase_sum: CatalogTree_Blocks_Rewards_24hCoinbaseSum = CatalogTree_Blocks_Rewards_24hCoinbaseSum(client)
        self.coinbase: CoinbasePattern = CoinbasePattern(client, 'coinbase')
        self.fee_dominance: MetricPattern6[StoredF32] = MetricPattern6(client, 'fee_dominance')
        self.subsidy: CoinbasePattern = CoinbasePattern(client, 'subsidy')
        self.subsidy_dominance: MetricPattern6[StoredF32] = MetricPattern6(client, 'subsidy_dominance')
        self.subsidy_usd_1y_sma: MetricPattern4[Dollars] = MetricPattern4(client, 'subsidy_usd_1y_sma')
        self.unclaimed_rewards: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'unclaimed_rewards')

class CatalogTree_Blocks_Rewards_24hCoinbaseSum:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: MetricPattern11[Bitcoin] = MetricPattern11(client, '24h_coinbase_sum_btc')
        self.dollars: MetricPattern11[Dollars] = MetricPattern11(client, '24h_coinbase_sum_usd')
        self.sats: MetricPattern11[Sats] = MetricPattern11(client, '24h_coinbase_sum')

class CatalogTree_Blocks_Size:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_average')
        self.cumulative: MetricPattern1[StoredU64] = MetricPattern1(client, 'block_size_cumulative')
        self.max: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_max')
        self.median: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_median')
        self.min: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_min')
        self.pct10: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct10')
        self.pct25: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct25')
        self.pct75: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct75')
        self.pct90: MetricPattern6[StoredU64] = MetricPattern6(client, 'block_size_pct90')
        self.sum: MetricPattern2[StoredU64] = MetricPattern2(client, 'block_size_sum')

class CatalogTree_Blocks_Time:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern11[Date] = MetricPattern11(client, 'date')
        self.date_fixed: MetricPattern11[Date] = MetricPattern11(client, 'date_fixed')
        self.timestamp: MetricPattern1[Timestamp] = MetricPattern1(client, 'timestamp')
        self.timestamp_fixed: MetricPattern11[Timestamp] = MetricPattern11(client, 'timestamp_fixed')

class CatalogTree_Cointime:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: CatalogTree_Cointime_Activity = CatalogTree_Cointime_Activity(client)
        self.adjusted: CatalogTree_Cointime_Adjusted = CatalogTree_Cointime_Adjusted(client)
        self.cap: CatalogTree_Cointime_Cap = CatalogTree_Cointime_Cap(client)
        self.pricing: CatalogTree_Cointime_Pricing = CatalogTree_Cointime_Pricing(client)
        self.supply: CatalogTree_Cointime_Supply = CatalogTree_Cointime_Supply(client)
        self.value: CatalogTree_Cointime_Value = CatalogTree_Cointime_Value(client)

class CatalogTree_Cointime_Activity:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity_to_vaultedness_ratio: MetricPattern1[StoredF64] = MetricPattern1(client, 'activity_to_vaultedness_ratio')
        self.coinblocks_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_created')
        self.coinblocks_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'coinblocks_stored')
        self.liveliness: MetricPattern1[StoredF64] = MetricPattern1(client, 'liveliness')
        self.vaultedness: MetricPattern1[StoredF64] = MetricPattern1(client, 'vaultedness')

class CatalogTree_Cointime_Adjusted:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_adj_inflation_rate: MetricPattern4[StoredF32] = MetricPattern4(client, 'cointime_adj_inflation_rate')
        self.cointime_adj_tx_btc_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, 'cointime_adj_tx_btc_velocity')
        self.cointime_adj_tx_usd_velocity: MetricPattern4[StoredF64] = MetricPattern4(client, 'cointime_adj_tx_usd_velocity')

class CatalogTree_Cointime_Cap:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'active_cap')
        self.cointime_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'cointime_cap')
        self.investor_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'investor_cap')
        self.thermo_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'thermo_cap')
        self.vaulted_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'vaulted_cap')

class CatalogTree_Cointime_Pricing:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_price: MetricPattern1[Dollars] = MetricPattern1(client, 'active_price')
        self.active_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'active_price_ratio')
        self.cointime_price: MetricPattern1[Dollars] = MetricPattern1(client, 'cointime_price')
        self.cointime_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'cointime_price_ratio')
        self.true_market_mean: MetricPattern1[Dollars] = MetricPattern1(client, 'true_market_mean')
        self.true_market_mean_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'true_market_mean_ratio')
        self.vaulted_price: MetricPattern1[Dollars] = MetricPattern1(client, 'vaulted_price')
        self.vaulted_price_ratio: ActivePriceRatioPattern = ActivePriceRatioPattern(client, 'vaulted_price_ratio')

class CatalogTree_Cointime_Supply:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.active_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, 'active_supply')
        self.vaulted_supply: ActiveSupplyPattern = ActiveSupplyPattern(client, 'vaulted_supply')

class CatalogTree_Cointime_Value:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cointime_value_created: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_created')
        self.cointime_value_destroyed: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_destroyed')
        self.cointime_value_stored: BlockCountPattern[StoredF64] = BlockCountPattern(client, 'cointime_value_stored')

class CatalogTree_Constants:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.constant_0: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_0')
        self.constant_1: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_1')
        self.constant_100: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_100')
        self.constant_2: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_2')
        self.constant_20: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_20')
        self.constant_3: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_3')
        self.constant_30: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_30')
        self.constant_38_2: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_38_2')
        self.constant_4: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_4')
        self.constant_50: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_50')
        self.constant_600: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_600')
        self.constant_61_8: MetricPattern1[StoredF32] = MetricPattern1(client, 'constant_61_8')
        self.constant_70: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_70')
        self.constant_80: MetricPattern1[StoredU16] = MetricPattern1(client, 'constant_80')
        self.constant_minus_1: MetricPattern1[StoredI16] = MetricPattern1(client, 'constant_minus_1')
        self.constant_minus_2: MetricPattern1[StoredI16] = MetricPattern1(client, 'constant_minus_2')
        self.constant_minus_3: MetricPattern1[StoredI16] = MetricPattern1(client, 'constant_minus_3')
        self.constant_minus_4: MetricPattern1[StoredI16] = MetricPattern1(client, 'constant_minus_4')

class CatalogTree_Distribution:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.addr_count: CatalogTree_Distribution_AddrCount = CatalogTree_Distribution_AddrCount(client)
        self.address_cohorts: CatalogTree_Distribution_AddressCohorts = CatalogTree_Distribution_AddressCohorts(client)
        self.addresses_data: CatalogTree_Distribution_AddressesData = CatalogTree_Distribution_AddressesData(client)
        self.any_address_indexes: CatalogTree_Distribution_AnyAddressIndexes = CatalogTree_Distribution_AnyAddressIndexes(client)
        self.chain_state: MetricPattern11[SupplyState] = MetricPattern11(client, 'chain')
        self.empty_addr_count: CatalogTree_Distribution_EmptyAddrCount = CatalogTree_Distribution_EmptyAddrCount(client)
        self.emptyaddressindex: MetricPattern32[EmptyAddressIndex] = MetricPattern32(client, 'emptyaddressindex')
        self.loadedaddressindex: MetricPattern31[LoadedAddressIndex] = MetricPattern31(client, 'loadedaddressindex')
        self.utxo_cohorts: CatalogTree_Distribution_UtxoCohorts = CatalogTree_Distribution_UtxoCohorts(client)

class CatalogTree_Distribution_AddrCount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, 'addr_count')
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2a_addr_count')
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk33_addr_count')
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk65_addr_count')
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pkh_addr_count')
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2sh_addr_count')
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2tr_addr_count')
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wpkh_addr_count')
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wsh_addr_count')

class CatalogTree_Distribution_AddressCohorts:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.amount_range: CatalogTree_Distribution_AddressCohorts_AmountRange = CatalogTree_Distribution_AddressCohorts_AmountRange(client)
        self.ge_amount: CatalogTree_Distribution_AddressCohorts_GeAmount = CatalogTree_Distribution_AddressCohorts_GeAmount(client)
        self.lt_amount: CatalogTree_Distribution_AddressCohorts_LtAmount = CatalogTree_Distribution_AddressCohorts_LtAmount(client)

class CatalogTree_Distribution_AddressCohorts_AmountRange:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: _0satsPattern = _0satsPattern(client, 'addrs_with_0sats')
        self._100btc_to_1k_btc: _0satsPattern = _0satsPattern(client, 'addrs_above_100btc_under_1k_btc')
        self._100k_btc_or_more: _0satsPattern = _0satsPattern(client, 'addrs_above_100k_btc')
        self._100k_sats_to_1m_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_100k_sats_under_1m_sats')
        self._100sats_to_1k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_100sats_under_1k_sats')
        self._10btc_to_100btc: _0satsPattern = _0satsPattern(client, 'addrs_above_10btc_under_100btc')
        self._10k_btc_to_100k_btc: _0satsPattern = _0satsPattern(client, 'addrs_above_10k_btc_under_100k_btc')
        self._10k_sats_to_100k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_10k_sats_under_100k_sats')
        self._10m_sats_to_1btc: _0satsPattern = _0satsPattern(client, 'addrs_above_10m_sats_under_1btc')
        self._10sats_to_100sats: _0satsPattern = _0satsPattern(client, 'addrs_above_10sats_under_100sats')
        self._1btc_to_10btc: _0satsPattern = _0satsPattern(client, 'addrs_above_1btc_under_10btc')
        self._1k_btc_to_10k_btc: _0satsPattern = _0satsPattern(client, 'addrs_above_1k_btc_under_10k_btc')
        self._1k_sats_to_10k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_1k_sats_under_10k_sats')
        self._1m_sats_to_10m_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_1m_sats_under_10m_sats')
        self._1sat_to_10sats: _0satsPattern = _0satsPattern(client, 'addrs_above_1sat_under_10sats')

class CatalogTree_Distribution_AddressCohorts_GeAmount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern = _0satsPattern(client, 'addrs_above_100btc')
        self._100k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_100k_sats')
        self._100sats: _0satsPattern = _0satsPattern(client, 'addrs_above_100sats')
        self._10btc: _0satsPattern = _0satsPattern(client, 'addrs_above_10btc')
        self._10k_btc: _0satsPattern = _0satsPattern(client, 'addrs_above_10k_btc')
        self._10k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_10k_sats')
        self._10m_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_10m_sats')
        self._10sats: _0satsPattern = _0satsPattern(client, 'addrs_above_10sats')
        self._1btc: _0satsPattern = _0satsPattern(client, 'addrs_above_1btc')
        self._1k_btc: _0satsPattern = _0satsPattern(client, 'addrs_above_1k_btc')
        self._1k_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_1k_sats')
        self._1m_sats: _0satsPattern = _0satsPattern(client, 'addrs_above_1m_sats')
        self._1sat: _0satsPattern = _0satsPattern(client, 'addrs_above_1sat')

class CatalogTree_Distribution_AddressCohorts_LtAmount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _0satsPattern = _0satsPattern(client, 'addrs_under_100btc')
        self._100k_btc: _0satsPattern = _0satsPattern(client, 'addrs_under_100k_btc')
        self._100k_sats: _0satsPattern = _0satsPattern(client, 'addrs_under_100k_sats')
        self._100sats: _0satsPattern = _0satsPattern(client, 'addrs_under_100sats')
        self._10btc: _0satsPattern = _0satsPattern(client, 'addrs_under_10btc')
        self._10k_btc: _0satsPattern = _0satsPattern(client, 'addrs_under_10k_btc')
        self._10k_sats: _0satsPattern = _0satsPattern(client, 'addrs_under_10k_sats')
        self._10m_sats: _0satsPattern = _0satsPattern(client, 'addrs_under_10m_sats')
        self._10sats: _0satsPattern = _0satsPattern(client, 'addrs_under_10sats')
        self._1btc: _0satsPattern = _0satsPattern(client, 'addrs_under_1btc')
        self._1k_btc: _0satsPattern = _0satsPattern(client, 'addrs_under_1k_btc')
        self._1k_sats: _0satsPattern = _0satsPattern(client, 'addrs_under_1k_sats')
        self._1m_sats: _0satsPattern = _0satsPattern(client, 'addrs_under_1m_sats')

class CatalogTree_Distribution_AddressesData:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: MetricPattern32[EmptyAddressData] = MetricPattern32(client, 'emptyaddressdata')
        self.loaded: MetricPattern31[LoadedAddressData] = MetricPattern31(client, 'loadedaddressdata')

class CatalogTree_Distribution_AnyAddressIndexes:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.p2a: MetricPattern16[AnyAddressIndex] = MetricPattern16(client, 'anyaddressindex')
        self.p2pk33: MetricPattern18[AnyAddressIndex] = MetricPattern18(client, 'anyaddressindex')
        self.p2pk65: MetricPattern19[AnyAddressIndex] = MetricPattern19(client, 'anyaddressindex')
        self.p2pkh: MetricPattern20[AnyAddressIndex] = MetricPattern20(client, 'anyaddressindex')
        self.p2sh: MetricPattern21[AnyAddressIndex] = MetricPattern21(client, 'anyaddressindex')
        self.p2tr: MetricPattern22[AnyAddressIndex] = MetricPattern22(client, 'anyaddressindex')
        self.p2wpkh: MetricPattern23[AnyAddressIndex] = MetricPattern23(client, 'anyaddressindex')
        self.p2wsh: MetricPattern24[AnyAddressIndex] = MetricPattern24(client, 'anyaddressindex')

class CatalogTree_Distribution_EmptyAddrCount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.all: MetricPattern1[StoredU64] = MetricPattern1(client, 'empty_addr_count')
        self.p2a: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2a_empty_addr_count')
        self.p2pk33: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk33_empty_addr_count')
        self.p2pk65: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pk65_empty_addr_count')
        self.p2pkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2pkh_empty_addr_count')
        self.p2sh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2sh_empty_addr_count')
        self.p2tr: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2tr_empty_addr_count')
        self.p2wpkh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wpkh_empty_addr_count')
        self.p2wsh: MetricPattern1[StoredU64] = MetricPattern1(client, 'p2wsh_empty_addr_count')

class CatalogTree_Distribution_UtxoCohorts:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.age_range: CatalogTree_Distribution_UtxoCohorts_AgeRange = CatalogTree_Distribution_UtxoCohorts_AgeRange(client)
        self.all: CatalogTree_Distribution_UtxoCohorts_All = CatalogTree_Distribution_UtxoCohorts_All(client)
        self.amount_range: CatalogTree_Distribution_UtxoCohorts_AmountRange = CatalogTree_Distribution_UtxoCohorts_AmountRange(client)
        self.epoch: CatalogTree_Distribution_UtxoCohorts_Epoch = CatalogTree_Distribution_UtxoCohorts_Epoch(client)
        self.ge_amount: CatalogTree_Distribution_UtxoCohorts_GeAmount = CatalogTree_Distribution_UtxoCohorts_GeAmount(client)
        self.lt_amount: CatalogTree_Distribution_UtxoCohorts_LtAmount = CatalogTree_Distribution_UtxoCohorts_LtAmount(client)
        self.max_age: CatalogTree_Distribution_UtxoCohorts_MaxAge = CatalogTree_Distribution_UtxoCohorts_MaxAge(client)
        self.min_age: CatalogTree_Distribution_UtxoCohorts_MinAge = CatalogTree_Distribution_UtxoCohorts_MinAge(client)
        self.term: CatalogTree_Distribution_UtxoCohorts_Term = CatalogTree_Distribution_UtxoCohorts_Term(client)
        self.type_: CatalogTree_Distribution_UtxoCohorts_Type = CatalogTree_Distribution_UtxoCohorts_Type(client)
        self.year: CatalogTree_Distribution_UtxoCohorts_Year = CatalogTree_Distribution_UtxoCohorts_Year(client)

class CatalogTree_Distribution_UtxoCohorts_AgeRange:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y_to_12y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_10y_up_to_12y_old')
        self._12y_to_15y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_12y_up_to_15y_old')
        self._1d_to_1w: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1d_up_to_1w_old')
        self._1h_to_1d: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1h_up_to_1d_old')
        self._1m_to_2m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1m_up_to_2m_old')
        self._1w_to_1m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1w_up_to_1m_old')
        self._1y_to_2y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_1y_up_to_2y_old')
        self._2m_to_3m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_2m_up_to_3m_old')
        self._2y_to_3y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_2y_up_to_3y_old')
        self._3m_to_4m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_3m_up_to_4m_old')
        self._3y_to_4y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_3y_up_to_4y_old')
        self._4m_to_5m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_4m_up_to_5m_old')
        self._4y_to_5y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_4y_up_to_5y_old')
        self._5m_to_6m: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_5m_up_to_6m_old')
        self._5y_to_6y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_5y_up_to_6y_old')
        self._6m_to_1y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_6m_up_to_1y_old')
        self._6y_to_7y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_6y_up_to_7y_old')
        self._7y_to_8y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_7y_up_to_8y_old')
        self._8y_to_10y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_8y_up_to_10y_old')
        self.from_15y: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_at_least_15y_old')
        self.up_to_1h: _10yTo12yPattern = _10yTo12yPattern(client, 'utxos_up_to_1h_old')

class CatalogTree_Distribution_UtxoCohorts_All:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, 'coinblocks_destroyed_cumulative')
        self.cost_basis: CatalogTree_Distribution_UtxoCohorts_All_CostBasis = CatalogTree_Distribution_UtxoCohorts_All_CostBasis(client)
        self.outputs: OutputsPattern = OutputsPattern(client, 'utxo_count')
        self.realized: RealizedPattern3 = RealizedPattern3(client, 'adjusted_sopr')
        self.relative: CatalogTree_Distribution_UtxoCohorts_All_Relative = CatalogTree_Distribution_UtxoCohorts_All_Relative(client)
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, 'neg_unrealized_loss')

class CatalogTree_Distribution_UtxoCohorts_All_CostBasis:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, 'max_cost_basis')
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, 'min_cost_basis')
        self.percentiles: PercentilesPattern = PercentilesPattern(client, 'cost_basis')

class CatalogTree_Distribution_UtxoCohorts_All_Relative:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'neg_unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'net_unrealized_pnl_rel_to_own_total_unrealized_pnl')
        self.supply_in_loss_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, 'supply_in_loss_rel_to_own_supply')
        self.supply_in_profit_rel_to_own_supply: MetricPattern1[StoredF64] = MetricPattern1(client, 'supply_in_profit_rel_to_own_supply')
        self.unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_loss_rel_to_own_total_unrealized_pnl')
        self.unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1[StoredF32] = MetricPattern1(client, 'unrealized_profit_rel_to_own_total_unrealized_pnl')

class CatalogTree_Distribution_UtxoCohorts_AmountRange:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_with_0sats')
        self._100btc_to_1k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100btc_under_1k_btc')
        self._100k_btc_or_more: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100k_btc')
        self._100k_sats_to_1m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100k_sats_under_1m_sats')
        self._100sats_to_1k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_100sats_under_1k_sats')
        self._10btc_to_100btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10btc_under_100btc')
        self._10k_btc_to_100k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10k_btc_under_100k_btc')
        self._10k_sats_to_100k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10k_sats_under_100k_sats')
        self._10m_sats_to_1btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10m_sats_under_1btc')
        self._10sats_to_100sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_10sats_under_100sats')
        self._1btc_to_10btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1btc_under_10btc')
        self._1k_btc_to_10k_btc: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1k_btc_under_10k_btc')
        self._1k_sats_to_10k_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1k_sats_under_10k_sats')
        self._1m_sats_to_10m_sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1m_sats_under_10m_sats')
        self._1sat_to_10sats: _0satsPattern2 = _0satsPattern2(client, 'utxos_above_1sat_under_10sats')

class CatalogTree_Distribution_UtxoCohorts_Epoch:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._0: _0satsPattern2 = _0satsPattern2(client, 'epoch_0')
        self._1: _0satsPattern2 = _0satsPattern2(client, 'epoch_1')
        self._2: _0satsPattern2 = _0satsPattern2(client, 'epoch_2')
        self._3: _0satsPattern2 = _0satsPattern2(client, 'epoch_3')
        self._4: _0satsPattern2 = _0satsPattern2(client, 'epoch_4')

class CatalogTree_Distribution_UtxoCohorts_GeAmount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _100btcPattern = _100btcPattern(client, 'utxos_above_100btc')
        self._100k_sats: _100btcPattern = _100btcPattern(client, 'utxos_above_100k_sats')
        self._100sats: _100btcPattern = _100btcPattern(client, 'utxos_above_100sats')
        self._10btc: _100btcPattern = _100btcPattern(client, 'utxos_above_10btc')
        self._10k_btc: _100btcPattern = _100btcPattern(client, 'utxos_above_10k_btc')
        self._10k_sats: _100btcPattern = _100btcPattern(client, 'utxos_above_10k_sats')
        self._10m_sats: _100btcPattern = _100btcPattern(client, 'utxos_above_10m_sats')
        self._10sats: _100btcPattern = _100btcPattern(client, 'utxos_above_10sats')
        self._1btc: _100btcPattern = _100btcPattern(client, 'utxos_above_1btc')
        self._1k_btc: _100btcPattern = _100btcPattern(client, 'utxos_above_1k_btc')
        self._1k_sats: _100btcPattern = _100btcPattern(client, 'utxos_above_1k_sats')
        self._1m_sats: _100btcPattern = _100btcPattern(client, 'utxos_above_1m_sats')
        self._1sat: _100btcPattern = _100btcPattern(client, 'utxos_above_1sat')

class CatalogTree_Distribution_UtxoCohorts_LtAmount:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._100btc: _100btcPattern = _100btcPattern(client, 'utxos_under_100btc')
        self._100k_btc: _100btcPattern = _100btcPattern(client, 'utxos_under_100k_btc')
        self._100k_sats: _100btcPattern = _100btcPattern(client, 'utxos_under_100k_sats')
        self._100sats: _100btcPattern = _100btcPattern(client, 'utxos_under_100sats')
        self._10btc: _100btcPattern = _100btcPattern(client, 'utxos_under_10btc')
        self._10k_btc: _100btcPattern = _100btcPattern(client, 'utxos_under_10k_btc')
        self._10k_sats: _100btcPattern = _100btcPattern(client, 'utxos_under_10k_sats')
        self._10m_sats: _100btcPattern = _100btcPattern(client, 'utxos_under_10m_sats')
        self._10sats: _100btcPattern = _100btcPattern(client, 'utxos_under_10sats')
        self._1btc: _100btcPattern = _100btcPattern(client, 'utxos_under_1btc')
        self._1k_btc: _100btcPattern = _100btcPattern(client, 'utxos_under_1k_btc')
        self._1k_sats: _100btcPattern = _100btcPattern(client, 'utxos_under_1k_sats')
        self._1m_sats: _100btcPattern = _100btcPattern(client, 'utxos_under_1m_sats')

class CatalogTree_Distribution_UtxoCohorts_MaxAge:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: _10yPattern = _10yPattern(client, 'utxos_up_to_10y_old')
        self._12y: _10yPattern = _10yPattern(client, 'utxos_up_to_12y_old')
        self._15y: _10yPattern = _10yPattern(client, 'utxos_up_to_15y_old')
        self._1m: _10yPattern = _10yPattern(client, 'utxos_up_to_1m_old')
        self._1w: _10yPattern = _10yPattern(client, 'utxos_up_to_1w_old')
        self._1y: _10yPattern = _10yPattern(client, 'utxos_up_to_1y_old')
        self._2m: _10yPattern = _10yPattern(client, 'utxos_up_to_2m_old')
        self._2y: _10yPattern = _10yPattern(client, 'utxos_up_to_2y_old')
        self._3m: _10yPattern = _10yPattern(client, 'utxos_up_to_3m_old')
        self._3y: _10yPattern = _10yPattern(client, 'utxos_up_to_3y_old')
        self._4m: _10yPattern = _10yPattern(client, 'utxos_up_to_4m_old')
        self._4y: _10yPattern = _10yPattern(client, 'utxos_up_to_4y_old')
        self._5m: _10yPattern = _10yPattern(client, 'utxos_up_to_5m_old')
        self._5y: _10yPattern = _10yPattern(client, 'utxos_up_to_5y_old')
        self._6m: _10yPattern = _10yPattern(client, 'utxos_up_to_6m_old')
        self._6y: _10yPattern = _10yPattern(client, 'utxos_up_to_6y_old')
        self._7y: _10yPattern = _10yPattern(client, 'utxos_up_to_7y_old')
        self._8y: _10yPattern = _10yPattern(client, 'utxos_up_to_8y_old')

class CatalogTree_Distribution_UtxoCohorts_MinAge:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_10y_old')
        self._12y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_12y_old')
        self._1d: _100btcPattern = _100btcPattern(client, 'utxos_at_least_1d_old')
        self._1m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_1m_old')
        self._1w: _100btcPattern = _100btcPattern(client, 'utxos_at_least_1w_old')
        self._1y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_1y_old')
        self._2m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_2m_old')
        self._2y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_2y_old')
        self._3m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_3m_old')
        self._3y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_3y_old')
        self._4m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_4m_old')
        self._4y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_4y_old')
        self._5m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_5m_old')
        self._5y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_5y_old')
        self._6m: _100btcPattern = _100btcPattern(client, 'utxos_at_least_6m_old')
        self._6y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_6y_old')
        self._7y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_7y_old')
        self._8y: _100btcPattern = _100btcPattern(client, 'utxos_at_least_8y_old')

class CatalogTree_Distribution_UtxoCohorts_Term:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.long: CatalogTree_Distribution_UtxoCohorts_Term_Long = CatalogTree_Distribution_UtxoCohorts_Term_Long(client)
        self.short: CatalogTree_Distribution_UtxoCohorts_Term_Short = CatalogTree_Distribution_UtxoCohorts_Term_Short(client)

class CatalogTree_Distribution_UtxoCohorts_Term_Long:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, 'lth')
        self.cost_basis: CatalogTree_Distribution_UtxoCohorts_Term_Long_CostBasis = CatalogTree_Distribution_UtxoCohorts_Term_Long_CostBasis(client)
        self.outputs: OutputsPattern = OutputsPattern(client, 'lth')
        self.realized: RealizedPattern2 = RealizedPattern2(client, 'lth')
        self.relative: RelativePattern5 = RelativePattern5(client, 'lth')
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'lth_supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, 'lth')

class CatalogTree_Distribution_UtxoCohorts_Term_Long_CostBasis:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, 'lth_max_cost_basis')
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, 'lth_min_cost_basis')
        self.percentiles: PercentilesPattern = PercentilesPattern(client, 'lth_cost_basis')

class CatalogTree_Distribution_UtxoCohorts_Term_Short:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.activity: ActivityPattern2 = ActivityPattern2(client, 'sth')
        self.cost_basis: CatalogTree_Distribution_UtxoCohorts_Term_Short_CostBasis = CatalogTree_Distribution_UtxoCohorts_Term_Short_CostBasis(client)
        self.outputs: OutputsPattern = OutputsPattern(client, 'sth')
        self.realized: RealizedPattern3 = RealizedPattern3(client, 'sth')
        self.relative: RelativePattern5 = RelativePattern5(client, 'sth')
        self.supply: SupplyPattern2 = SupplyPattern2(client, 'sth_supply')
        self.unrealized: UnrealizedPattern = UnrealizedPattern(client, 'sth')

class CatalogTree_Distribution_UtxoCohorts_Term_Short_CostBasis:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, 'sth_max_cost_basis')
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, 'sth_min_cost_basis')
        self.percentiles: PercentilesPattern = PercentilesPattern(client, 'sth_cost_basis')

class CatalogTree_Distribution_UtxoCohorts_Type:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: _0satsPattern2 = _0satsPattern2(client, 'empty_outputs')
        self.p2a: _0satsPattern2 = _0satsPattern2(client, 'p2a')
        self.p2ms: _0satsPattern2 = _0satsPattern2(client, 'p2ms')
        self.p2pk33: _0satsPattern2 = _0satsPattern2(client, 'p2pk33')
        self.p2pk65: _0satsPattern2 = _0satsPattern2(client, 'p2pk65')
        self.p2pkh: _0satsPattern2 = _0satsPattern2(client, 'p2pkh')
        self.p2sh: _0satsPattern2 = _0satsPattern2(client, 'p2sh')
        self.p2tr: _0satsPattern2 = _0satsPattern2(client, 'p2tr')
        self.p2wpkh: _0satsPattern2 = _0satsPattern2(client, 'p2wpkh')
        self.p2wsh: _0satsPattern2 = _0satsPattern2(client, 'p2wsh')
        self.unknown: _0satsPattern2 = _0satsPattern2(client, 'unknown_outputs')

class CatalogTree_Distribution_UtxoCohorts_Year:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2009: _0satsPattern2 = _0satsPattern2(client, 'year_2009')
        self._2010: _0satsPattern2 = _0satsPattern2(client, 'year_2010')
        self._2011: _0satsPattern2 = _0satsPattern2(client, 'year_2011')
        self._2012: _0satsPattern2 = _0satsPattern2(client, 'year_2012')
        self._2013: _0satsPattern2 = _0satsPattern2(client, 'year_2013')
        self._2014: _0satsPattern2 = _0satsPattern2(client, 'year_2014')
        self._2015: _0satsPattern2 = _0satsPattern2(client, 'year_2015')
        self._2016: _0satsPattern2 = _0satsPattern2(client, 'year_2016')
        self._2017: _0satsPattern2 = _0satsPattern2(client, 'year_2017')
        self._2018: _0satsPattern2 = _0satsPattern2(client, 'year_2018')
        self._2019: _0satsPattern2 = _0satsPattern2(client, 'year_2019')
        self._2020: _0satsPattern2 = _0satsPattern2(client, 'year_2020')
        self._2021: _0satsPattern2 = _0satsPattern2(client, 'year_2021')
        self._2022: _0satsPattern2 = _0satsPattern2(client, 'year_2022')
        self._2023: _0satsPattern2 = _0satsPattern2(client, 'year_2023')
        self._2024: _0satsPattern2 = _0satsPattern2(client, 'year_2024')
        self._2025: _0satsPattern2 = _0satsPattern2(client, 'year_2025')
        self._2026: _0satsPattern2 = _0satsPattern2(client, 'year_2026')

class CatalogTree_Indexes:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.address: CatalogTree_Indexes_Address = CatalogTree_Indexes_Address(client)
        self.dateindex: CatalogTree_Indexes_Dateindex = CatalogTree_Indexes_Dateindex(client)
        self.decadeindex: CatalogTree_Indexes_Decadeindex = CatalogTree_Indexes_Decadeindex(client)
        self.difficultyepoch: CatalogTree_Indexes_Difficultyepoch = CatalogTree_Indexes_Difficultyepoch(client)
        self.halvingepoch: CatalogTree_Indexes_Halvingepoch = CatalogTree_Indexes_Halvingepoch(client)
        self.height: CatalogTree_Indexes_Height = CatalogTree_Indexes_Height(client)
        self.monthindex: CatalogTree_Indexes_Monthindex = CatalogTree_Indexes_Monthindex(client)
        self.quarterindex: CatalogTree_Indexes_Quarterindex = CatalogTree_Indexes_Quarterindex(client)
        self.semesterindex: CatalogTree_Indexes_Semesterindex = CatalogTree_Indexes_Semesterindex(client)
        self.txindex: CatalogTree_Indexes_Txindex = CatalogTree_Indexes_Txindex(client)
        self.txinindex: CatalogTree_Indexes_Txinindex = CatalogTree_Indexes_Txinindex(client)
        self.txoutindex: CatalogTree_Indexes_Txoutindex = CatalogTree_Indexes_Txoutindex(client)
        self.weekindex: CatalogTree_Indexes_Weekindex = CatalogTree_Indexes_Weekindex(client)
        self.yearindex: CatalogTree_Indexes_Yearindex = CatalogTree_Indexes_Yearindex(client)

class CatalogTree_Indexes_Address:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.empty: CatalogTree_Indexes_Address_Empty = CatalogTree_Indexes_Address_Empty(client)
        self.opreturn: CatalogTree_Indexes_Address_Opreturn = CatalogTree_Indexes_Address_Opreturn(client)
        self.p2a: CatalogTree_Indexes_Address_P2a = CatalogTree_Indexes_Address_P2a(client)
        self.p2ms: CatalogTree_Indexes_Address_P2ms = CatalogTree_Indexes_Address_P2ms(client)
        self.p2pk33: CatalogTree_Indexes_Address_P2pk33 = CatalogTree_Indexes_Address_P2pk33(client)
        self.p2pk65: CatalogTree_Indexes_Address_P2pk65 = CatalogTree_Indexes_Address_P2pk65(client)
        self.p2pkh: CatalogTree_Indexes_Address_P2pkh = CatalogTree_Indexes_Address_P2pkh(client)
        self.p2sh: CatalogTree_Indexes_Address_P2sh = CatalogTree_Indexes_Address_P2sh(client)
        self.p2tr: CatalogTree_Indexes_Address_P2tr = CatalogTree_Indexes_Address_P2tr(client)
        self.p2wpkh: CatalogTree_Indexes_Address_P2wpkh = CatalogTree_Indexes_Address_P2wpkh(client)
        self.p2wsh: CatalogTree_Indexes_Address_P2wsh = CatalogTree_Indexes_Address_P2wsh(client)
        self.unknown: CatalogTree_Indexes_Address_Unknown = CatalogTree_Indexes_Address_Unknown(client)

class CatalogTree_Indexes_Address_Empty:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern9[EmptyOutputIndex] = MetricPattern9(client, 'emptyoutputindex')

class CatalogTree_Indexes_Address_Opreturn:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern14[OpReturnIndex] = MetricPattern14(client, 'opreturnindex')

class CatalogTree_Indexes_Address_P2a:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern16[P2AAddressIndex] = MetricPattern16(client, 'p2aaddressindex')

class CatalogTree_Indexes_Address_P2ms:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern17[P2MSOutputIndex] = MetricPattern17(client, 'p2msoutputindex')

class CatalogTree_Indexes_Address_P2pk33:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern18[P2PK33AddressIndex] = MetricPattern18(client, 'p2pk33addressindex')

class CatalogTree_Indexes_Address_P2pk65:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern19[P2PK65AddressIndex] = MetricPattern19(client, 'p2pk65addressindex')

class CatalogTree_Indexes_Address_P2pkh:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern20[P2PKHAddressIndex] = MetricPattern20(client, 'p2pkhaddressindex')

class CatalogTree_Indexes_Address_P2sh:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern21[P2SHAddressIndex] = MetricPattern21(client, 'p2shaddressindex')

class CatalogTree_Indexes_Address_P2tr:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern22[P2TRAddressIndex] = MetricPattern22(client, 'p2traddressindex')

class CatalogTree_Indexes_Address_P2wpkh:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern23[P2WPKHAddressIndex] = MetricPattern23(client, 'p2wpkhaddressindex')

class CatalogTree_Indexes_Address_P2wsh:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern24[P2WSHAddressIndex] = MetricPattern24(client, 'p2wshaddressindex')

class CatalogTree_Indexes_Address_Unknown:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern28[UnknownOutputIndex] = MetricPattern28(client, 'unknownoutputindex')

class CatalogTree_Indexes_Dateindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.date: MetricPattern6[Date] = MetricPattern6(client, 'dateindex_date')
        self.first_height: MetricPattern6[Height] = MetricPattern6(client, 'dateindex_first_height')
        self.height_count: MetricPattern6[StoredU64] = MetricPattern6(client, 'dateindex_height_count')
        self.identity: MetricPattern6[DateIndex] = MetricPattern6(client, 'dateindex')
        self.monthindex: MetricPattern6[MonthIndex] = MetricPattern6(client, 'dateindex_monthindex')
        self.weekindex: MetricPattern6[WeekIndex] = MetricPattern6(client, 'dateindex_weekindex')

class CatalogTree_Indexes_Decadeindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_yearindex: MetricPattern7[YearIndex] = MetricPattern7(client, 'decadeindex_first_yearindex')
        self.identity: MetricPattern7[DecadeIndex] = MetricPattern7(client, 'decadeindex')
        self.yearindex_count: MetricPattern7[StoredU64] = MetricPattern7(client, 'decadeindex_yearindex_count')

class CatalogTree_Indexes_Difficultyepoch:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_height: MetricPattern8[Height] = MetricPattern8(client, 'difficultyepoch_first_height')
        self.height_count: MetricPattern8[StoredU64] = MetricPattern8(client, 'difficultyepoch_height_count')
        self.identity: MetricPattern8[DifficultyEpoch] = MetricPattern8(client, 'difficultyepoch')

class CatalogTree_Indexes_Halvingepoch:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_height: MetricPattern10[Height] = MetricPattern10(client, 'halvingepoch_first_height')
        self.identity: MetricPattern10[HalvingEpoch] = MetricPattern10(client, 'halvingepoch')

class CatalogTree_Indexes_Height:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex: MetricPattern11[DateIndex] = MetricPattern11(client, 'height_dateindex')
        self.difficultyepoch: MetricPattern11[DifficultyEpoch] = MetricPattern11(client, 'height_difficultyepoch')
        self.halvingepoch: MetricPattern11[HalvingEpoch] = MetricPattern11(client, 'height_halvingepoch')
        self.identity: MetricPattern11[Height] = MetricPattern11(client, 'height')
        self.txindex_count: MetricPattern11[StoredU64] = MetricPattern11(client, 'height_txindex_count')

class CatalogTree_Indexes_Monthindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex_count: MetricPattern13[StoredU64] = MetricPattern13(client, 'monthindex_dateindex_count')
        self.first_dateindex: MetricPattern13[DateIndex] = MetricPattern13(client, 'monthindex_first_dateindex')
        self.identity: MetricPattern13[MonthIndex] = MetricPattern13(client, 'monthindex')
        self.quarterindex: MetricPattern13[QuarterIndex] = MetricPattern13(client, 'monthindex_quarterindex')
        self.semesterindex: MetricPattern13[SemesterIndex] = MetricPattern13(client, 'monthindex_semesterindex')
        self.yearindex: MetricPattern13[YearIndex] = MetricPattern13(client, 'monthindex_yearindex')

class CatalogTree_Indexes_Quarterindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_monthindex: MetricPattern25[MonthIndex] = MetricPattern25(client, 'quarterindex_first_monthindex')
        self.identity: MetricPattern25[QuarterIndex] = MetricPattern25(client, 'quarterindex')
        self.monthindex_count: MetricPattern25[StoredU64] = MetricPattern25(client, 'quarterindex_monthindex_count')

class CatalogTree_Indexes_Semesterindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.first_monthindex: MetricPattern26[MonthIndex] = MetricPattern26(client, 'semesterindex_first_monthindex')
        self.identity: MetricPattern26[SemesterIndex] = MetricPattern26(client, 'semesterindex')
        self.monthindex_count: MetricPattern26[StoredU64] = MetricPattern26(client, 'semesterindex_monthindex_count')

class CatalogTree_Indexes_Txindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern27[TxIndex] = MetricPattern27(client, 'txindex')
        self.input_count: MetricPattern27[StoredU64] = MetricPattern27(client, 'txindex_input_count')
        self.output_count: MetricPattern27[StoredU64] = MetricPattern27(client, 'txindex_output_count')

class CatalogTree_Indexes_Txinindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern12[TxInIndex] = MetricPattern12(client, 'txinindex')

class CatalogTree_Indexes_Txoutindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.identity: MetricPattern15[TxOutIndex] = MetricPattern15(client, 'txoutindex')

class CatalogTree_Indexes_Weekindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.dateindex_count: MetricPattern29[StoredU64] = MetricPattern29(client, 'weekindex_dateindex_count')
        self.first_dateindex: MetricPattern29[DateIndex] = MetricPattern29(client, 'weekindex_first_dateindex')
        self.identity: MetricPattern29[WeekIndex] = MetricPattern29(client, 'weekindex')

class CatalogTree_Indexes_Yearindex:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.decadeindex: MetricPattern30[DecadeIndex] = MetricPattern30(client, 'yearindex_decadeindex')
        self.first_monthindex: MetricPattern30[MonthIndex] = MetricPattern30(client, 'yearindex_first_monthindex')
        self.identity: MetricPattern30[YearIndex] = MetricPattern30(client, 'yearindex')
        self.monthindex_count: MetricPattern30[StoredU64] = MetricPattern30(client, 'yearindex_monthindex_count')

class CatalogTree_Inputs:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CountPattern2[StoredU64] = CountPattern2(client, 'input_count')
        self.first_txinindex: MetricPattern11[TxInIndex] = MetricPattern11(client, 'first_txinindex')
        self.outpoint: MetricPattern12[OutPoint] = MetricPattern12(client, 'outpoint')
        self.outputtype: MetricPattern12[OutputType] = MetricPattern12(client, 'outputtype')
        self.spent: CatalogTree_Inputs_Spent = CatalogTree_Inputs_Spent(client)
        self.txindex: MetricPattern12[TxIndex] = MetricPattern12(client, 'txindex')
        self.typeindex: MetricPattern12[TypeIndex] = MetricPattern12(client, 'typeindex')
        self.witness_size: MetricPattern12[StoredU32] = MetricPattern12(client, 'witness_size')

class CatalogTree_Inputs_Spent:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txoutindex: MetricPattern12[TxOutIndex] = MetricPattern12(client, 'txoutindex')
        self.value: MetricPattern12[Sats] = MetricPattern12(client, 'value')

class CatalogTree_Market:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ath: CatalogTree_Market_Ath = CatalogTree_Market_Ath(client)
        self.dca: CatalogTree_Market_Dca = CatalogTree_Market_Dca(client)
        self.indicators: CatalogTree_Market_Indicators = CatalogTree_Market_Indicators(client)
        self.lookback: CatalogTree_Market_Lookback = CatalogTree_Market_Lookback(client)
        self.moving_average: CatalogTree_Market_MovingAverage = CatalogTree_Market_MovingAverage(client)
        self.range: CatalogTree_Market_Range = CatalogTree_Market_Range(client)
        self.returns: CatalogTree_Market_Returns = CatalogTree_Market_Returns(client)
        self.volatility: CatalogTree_Market_Volatility = CatalogTree_Market_Volatility(client)

class CatalogTree_Market_Ath:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.days_since_price_ath: MetricPattern4[StoredU16] = MetricPattern4(client, 'days_since_price_ath')
        self.max_days_between_price_aths: MetricPattern4[StoredU16] = MetricPattern4(client, 'max_days_between_price_aths')
        self.max_years_between_price_aths: MetricPattern4[StoredF32] = MetricPattern4(client, 'max_years_between_price_aths')
        self.price_ath: MetricPattern1[Dollars] = MetricPattern1(client, 'price_ath')
        self.price_drawdown: MetricPattern3[StoredF32] = MetricPattern3(client, 'price_drawdown')
        self.years_since_price_ath: MetricPattern4[StoredF32] = MetricPattern4(client, 'years_since_price_ath')

class CatalogTree_Market_Dca:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.class_average_price: CatalogTree_Market_Dca_ClassAveragePrice = CatalogTree_Market_Dca_ClassAveragePrice(client)
        self.class_returns: CatalogTree_Market_Dca_ClassReturns = CatalogTree_Market_Dca_ClassReturns(client)
        self.class_stack: CatalogTree_Market_Dca_ClassStack = CatalogTree_Market_Dca_ClassStack(client)
        self.period_average_price: PeriodAveragePricePattern[Dollars] = PeriodAveragePricePattern(client, 'dca_average_price')
        self.period_cagr: PeriodCagrPattern = PeriodCagrPattern(client, 'dca_cagr')
        self.period_lump_sum_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, 'lump_sum_stack_btc')
        self.period_returns: PeriodAveragePricePattern[StoredF32] = PeriodAveragePricePattern(client, 'dca_returns')
        self.period_stack: PeriodLumpSumStackPattern = PeriodLumpSumStackPattern(client, 'dca_stack_btc')

class CatalogTree_Market_Dca_ClassAveragePrice:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2015_average_price')
        self._2016: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2016_average_price')
        self._2017: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2017_average_price')
        self._2018: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2018_average_price')
        self._2019: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2019_average_price')
        self._2020: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2020_average_price')
        self._2021: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2021_average_price')
        self._2022: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2022_average_price')
        self._2023: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2023_average_price')
        self._2024: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2024_average_price')
        self._2025: MetricPattern4[Dollars] = MetricPattern4(client, 'dca_class_2025_average_price')

class CatalogTree_Market_Dca_ClassReturns:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2015_returns')
        self._2016: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2016_returns')
        self._2017: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2017_returns')
        self._2018: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2018_returns')
        self._2019: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2019_returns')
        self._2020: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2020_returns')
        self._2021: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2021_returns')
        self._2022: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2022_returns')
        self._2023: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2023_returns')
        self._2024: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2024_returns')
        self._2025: MetricPattern4[StoredF32] = MetricPattern4(client, 'dca_class_2025_returns')

class CatalogTree_Market_Dca_ClassStack:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._2015: _2015Pattern = _2015Pattern(client, 'dca_class_2015_stack')
        self._2016: _2015Pattern = _2015Pattern(client, 'dca_class_2016_stack')
        self._2017: _2015Pattern = _2015Pattern(client, 'dca_class_2017_stack')
        self._2018: _2015Pattern = _2015Pattern(client, 'dca_class_2018_stack')
        self._2019: _2015Pattern = _2015Pattern(client, 'dca_class_2019_stack')
        self._2020: _2015Pattern = _2015Pattern(client, 'dca_class_2020_stack')
        self._2021: _2015Pattern = _2015Pattern(client, 'dca_class_2021_stack')
        self._2022: _2015Pattern = _2015Pattern(client, 'dca_class_2022_stack')
        self._2023: _2015Pattern = _2015Pattern(client, 'dca_class_2023_stack')
        self._2024: _2015Pattern = _2015Pattern(client, 'dca_class_2024_stack')
        self._2025: _2015Pattern = _2015Pattern(client, 'dca_class_2025_stack')

class CatalogTree_Market_Indicators:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.gini: MetricPattern6[StoredF32] = MetricPattern6(client, 'gini')
        self.macd_histogram: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_histogram')
        self.macd_line: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_line')
        self.macd_signal: MetricPattern6[StoredF32] = MetricPattern6(client, 'macd_signal')
        self.nvt: MetricPattern4[StoredF32] = MetricPattern4(client, 'nvt')
        self.pi_cycle: MetricPattern6[StoredF32] = MetricPattern6(client, 'pi_cycle')
        self.puell_multiple: MetricPattern4[StoredF32] = MetricPattern4(client, 'puell_multiple')
        self.rsi_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d')
        self.rsi_14d_max: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d_max')
        self.rsi_14d_min: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_14d_min')
        self.rsi_average_gain_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_average_gain_14d')
        self.rsi_average_loss_14d: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_average_loss_14d')
        self.rsi_gains: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_gains')
        self.rsi_losses: MetricPattern6[StoredF32] = MetricPattern6(client, 'rsi_losses')
        self.stoch_d: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_d')
        self.stoch_k: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_k')
        self.stoch_rsi: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi')
        self.stoch_rsi_d: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi_d')
        self.stoch_rsi_k: MetricPattern6[StoredF32] = MetricPattern6(client, 'stoch_rsi_k')

class CatalogTree_Market_Lookback:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_ago: CatalogTree_Market_Lookback_PriceAgo = CatalogTree_Market_Lookback_PriceAgo(client)

class CatalogTree_Market_Lookback_PriceAgo:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_10y_ago')
        self._1d: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1d_ago')
        self._1m: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1m_ago')
        self._1w: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1w_ago')
        self._1y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1y_ago')
        self._2y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_2y_ago')
        self._3m: MetricPattern4[Dollars] = MetricPattern4(client, 'price_3m_ago')
        self._3y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_3y_ago')
        self._4y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_4y_ago')
        self._5y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_5y_ago')
        self._6m: MetricPattern4[Dollars] = MetricPattern4(client, 'price_6m_ago')
        self._6y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_6y_ago')
        self._8y: MetricPattern4[Dollars] = MetricPattern4(client, 'price_8y_ago')

class CatalogTree_Market_MovingAverage:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_111d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_111d_sma')
        self.price_12d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_12d_ema')
        self.price_13d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_13d_ema')
        self.price_13d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_13d_sma')
        self.price_144d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_144d_ema')
        self.price_144d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_144d_sma')
        self.price_1m_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1m_ema')
        self.price_1m_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1m_sma')
        self.price_1w_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1w_ema')
        self.price_1w_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1w_sma')
        self.price_1y_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1y_ema')
        self.price_1y_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_1y_sma')
        self.price_200d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_200d_ema')
        self.price_200d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_200d_sma')
        self.price_200d_sma_x0_8: MetricPattern4[Dollars] = MetricPattern4(client, 'price_200d_sma_x0_8')
        self.price_200d_sma_x2_4: MetricPattern4[Dollars] = MetricPattern4(client, 'price_200d_sma_x2_4')
        self.price_200w_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_200w_ema')
        self.price_200w_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_200w_sma')
        self.price_21d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_21d_ema')
        self.price_21d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_21d_sma')
        self.price_26d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_26d_ema')
        self.price_2y_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_2y_ema')
        self.price_2y_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_2y_sma')
        self.price_34d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_34d_ema')
        self.price_34d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_34d_sma')
        self.price_350d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_350d_sma')
        self.price_350d_sma_x2: MetricPattern4[Dollars] = MetricPattern4(client, 'price_350d_sma_x2')
        self.price_4y_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_4y_ema')
        self.price_4y_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_4y_sma')
        self.price_55d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_55d_ema')
        self.price_55d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_55d_sma')
        self.price_89d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_89d_ema')
        self.price_89d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_89d_sma')
        self.price_8d_ema: Price111dSmaPattern = Price111dSmaPattern(client, 'price_8d_ema')
        self.price_8d_sma: Price111dSmaPattern = Price111dSmaPattern(client, 'price_8d_sma')

class CatalogTree_Market_Range:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1m_max: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1m_max')
        self.price_1m_min: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1m_min')
        self.price_1w_max: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1w_max')
        self.price_1w_min: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1w_min')
        self.price_1y_max: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1y_max')
        self.price_1y_min: MetricPattern4[Dollars] = MetricPattern4(client, 'price_1y_min')
        self.price_2w_choppiness_index: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_2w_choppiness_index')
        self.price_2w_max: MetricPattern4[Dollars] = MetricPattern4(client, 'price_2w_max')
        self.price_2w_min: MetricPattern4[Dollars] = MetricPattern4(client, 'price_2w_min')
        self.price_true_range: MetricPattern6[StoredF32] = MetricPattern6(client, 'price_true_range')
        self.price_true_range_2w_sum: MetricPattern6[StoredF32] = MetricPattern6(client, 'price_true_range_2w_sum')

class CatalogTree_Market_Returns:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._1d_returns_1m_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1m_sd')
        self._1d_returns_1w_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1w_sd')
        self._1d_returns_1y_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, '1d_returns_1y_sd')
        self.cagr: PeriodCagrPattern = PeriodCagrPattern(client, 'cagr')
        self.downside_1m_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1m_sd')
        self.downside_1w_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1w_sd')
        self.downside_1y_sd: _1dReturns1mSdPattern = _1dReturns1mSdPattern(client, 'downside_1y_sd')
        self.downside_returns: MetricPattern6[StoredF32] = MetricPattern6(client, 'downside_returns')
        self.price_returns: CatalogTree_Market_Returns_PriceReturns = CatalogTree_Market_Returns_PriceReturns(client)

class CatalogTree_Market_Returns_PriceReturns:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self._10y: MetricPattern4[StoredF32] = MetricPattern4(client, '10y_price_returns')
        self._1d: MetricPattern4[StoredF32] = MetricPattern4(client, '1d_price_returns')
        self._1m: MetricPattern4[StoredF32] = MetricPattern4(client, '1m_price_returns')
        self._1w: MetricPattern4[StoredF32] = MetricPattern4(client, '1w_price_returns')
        self._1y: MetricPattern4[StoredF32] = MetricPattern4(client, '1y_price_returns')
        self._2y: MetricPattern4[StoredF32] = MetricPattern4(client, '2y_price_returns')
        self._3m: MetricPattern4[StoredF32] = MetricPattern4(client, '3m_price_returns')
        self._3y: MetricPattern4[StoredF32] = MetricPattern4(client, '3y_price_returns')
        self._4y: MetricPattern4[StoredF32] = MetricPattern4(client, '4y_price_returns')
        self._5y: MetricPattern4[StoredF32] = MetricPattern4(client, '5y_price_returns')
        self._6m: MetricPattern4[StoredF32] = MetricPattern4(client, '6m_price_returns')
        self._6y: MetricPattern4[StoredF32] = MetricPattern4(client, '6y_price_returns')
        self._8y: MetricPattern4[StoredF32] = MetricPattern4(client, '8y_price_returns')

class CatalogTree_Market_Volatility:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.price_1m_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1m_volatility')
        self.price_1w_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1w_volatility')
        self.price_1y_volatility: MetricPattern4[StoredF32] = MetricPattern4(client, 'price_1y_volatility')
        self.sharpe_1m: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1m')
        self.sharpe_1w: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1w')
        self.sharpe_1y: MetricPattern6[StoredF32] = MetricPattern6(client, 'sharpe_1y')
        self.sortino_1m: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1m')
        self.sortino_1w: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1w')
        self.sortino_1y: MetricPattern6[StoredF32] = MetricPattern6(client, 'sortino_1y')

class CatalogTree_Outputs:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Outputs_Count = CatalogTree_Outputs_Count(client)
        self.first_txoutindex: MetricPattern11[TxOutIndex] = MetricPattern11(client, 'first_txoutindex')
        self.outputtype: MetricPattern15[OutputType] = MetricPattern15(client, 'outputtype')
        self.spent: CatalogTree_Outputs_Spent = CatalogTree_Outputs_Spent(client)
        self.txindex: MetricPattern15[TxIndex] = MetricPattern15(client, 'txindex')
        self.typeindex: MetricPattern15[TypeIndex] = MetricPattern15(client, 'typeindex')
        self.value: MetricPattern15[Sats] = MetricPattern15(client, 'value')

class CatalogTree_Outputs_Count:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.total_count: CountPattern2[StoredU64] = CountPattern2(client, 'output_count')
        self.utxo_count: MetricPattern1[StoredU64] = MetricPattern1(client, 'exact_utxo_count')

class CatalogTree_Outputs_Spent:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.txinindex: MetricPattern15[TxInIndex] = MetricPattern15(client, 'txinindex')

class CatalogTree_Pools:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.height_to_pool: MetricPattern11[PoolSlug] = MetricPattern11(client, 'pool')
        self.vecs: CatalogTree_Pools_Vecs = CatalogTree_Pools_Vecs(client)

class CatalogTree_Pools_Vecs:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.aaopool: AaopoolPattern = AaopoolPattern(client, 'aaopool')
        self.antpool: AaopoolPattern = AaopoolPattern(client, 'antpool')
        self.arkpool: AaopoolPattern = AaopoolPattern(client, 'arkpool')
        self.asicminer: AaopoolPattern = AaopoolPattern(client, 'asicminer')
        self.axbt: AaopoolPattern = AaopoolPattern(client, 'axbt')
        self.batpool: AaopoolPattern = AaopoolPattern(client, 'batpool')
        self.bcmonster: AaopoolPattern = AaopoolPattern(client, 'bcmonster')
        self.bcpoolio: AaopoolPattern = AaopoolPattern(client, 'bcpoolio')
        self.binancepool: AaopoolPattern = AaopoolPattern(client, 'binancepool')
        self.bitalo: AaopoolPattern = AaopoolPattern(client, 'bitalo')
        self.bitclub: AaopoolPattern = AaopoolPattern(client, 'bitclub')
        self.bitcoinaffiliatenetwork: AaopoolPattern = AaopoolPattern(client, 'bitcoinaffiliatenetwork')
        self.bitcoincom: AaopoolPattern = AaopoolPattern(client, 'bitcoincom')
        self.bitcoinindia: AaopoolPattern = AaopoolPattern(client, 'bitcoinindia')
        self.bitcoinrussia: AaopoolPattern = AaopoolPattern(client, 'bitcoinrussia')
        self.bitcoinukraine: AaopoolPattern = AaopoolPattern(client, 'bitcoinukraine')
        self.bitfarms: AaopoolPattern = AaopoolPattern(client, 'bitfarms')
        self.bitfufupool: AaopoolPattern = AaopoolPattern(client, 'bitfufupool')
        self.bitfury: AaopoolPattern = AaopoolPattern(client, 'bitfury')
        self.bitminter: AaopoolPattern = AaopoolPattern(client, 'bitminter')
        self.bitparking: AaopoolPattern = AaopoolPattern(client, 'bitparking')
        self.bitsolo: AaopoolPattern = AaopoolPattern(client, 'bitsolo')
        self.bixin: AaopoolPattern = AaopoolPattern(client, 'bixin')
        self.blockfills: AaopoolPattern = AaopoolPattern(client, 'blockfills')
        self.braiinspool: AaopoolPattern = AaopoolPattern(client, 'braiinspool')
        self.bravomining: AaopoolPattern = AaopoolPattern(client, 'bravomining')
        self.btcc: AaopoolPattern = AaopoolPattern(client, 'btcc')
        self.btccom: AaopoolPattern = AaopoolPattern(client, 'btccom')
        self.btcdig: AaopoolPattern = AaopoolPattern(client, 'btcdig')
        self.btcguild: AaopoolPattern = AaopoolPattern(client, 'btcguild')
        self.btclab: AaopoolPattern = AaopoolPattern(client, 'btclab')
        self.btcmp: AaopoolPattern = AaopoolPattern(client, 'btcmp')
        self.btcnuggets: AaopoolPattern = AaopoolPattern(client, 'btcnuggets')
        self.btcpoolparty: AaopoolPattern = AaopoolPattern(client, 'btcpoolparty')
        self.btcserv: AaopoolPattern = AaopoolPattern(client, 'btcserv')
        self.btctop: AaopoolPattern = AaopoolPattern(client, 'btctop')
        self.btpool: AaopoolPattern = AaopoolPattern(client, 'btpool')
        self.bwpool: AaopoolPattern = AaopoolPattern(client, 'bwpool')
        self.bytepool: AaopoolPattern = AaopoolPattern(client, 'bytepool')
        self.canoe: AaopoolPattern = AaopoolPattern(client, 'canoe')
        self.canoepool: AaopoolPattern = AaopoolPattern(client, 'canoepool')
        self.carbonnegative: AaopoolPattern = AaopoolPattern(client, 'carbonnegative')
        self.ckpool: AaopoolPattern = AaopoolPattern(client, 'ckpool')
        self.cloudhashing: AaopoolPattern = AaopoolPattern(client, 'cloudhashing')
        self.coinlab: AaopoolPattern = AaopoolPattern(client, 'coinlab')
        self.cointerra: AaopoolPattern = AaopoolPattern(client, 'cointerra')
        self.connectbtc: AaopoolPattern = AaopoolPattern(client, 'connectbtc')
        self.dcex: AaopoolPattern = AaopoolPattern(client, 'dcex')
        self.dcexploration: AaopoolPattern = AaopoolPattern(client, 'dcexploration')
        self.digitalbtc: AaopoolPattern = AaopoolPattern(client, 'digitalbtc')
        self.digitalxmintsy: AaopoolPattern = AaopoolPattern(client, 'digitalxmintsy')
        self.dpool: AaopoolPattern = AaopoolPattern(client, 'dpool')
        self.eclipsemc: AaopoolPattern = AaopoolPattern(client, 'eclipsemc')
        self.eightbaochi: AaopoolPattern = AaopoolPattern(client, 'eightbaochi')
        self.ekanembtc: AaopoolPattern = AaopoolPattern(client, 'ekanembtc')
        self.eligius: AaopoolPattern = AaopoolPattern(client, 'eligius')
        self.emcdpool: AaopoolPattern = AaopoolPattern(client, 'emcdpool')
        self.entrustcharitypool: AaopoolPattern = AaopoolPattern(client, 'entrustcharitypool')
        self.eobot: AaopoolPattern = AaopoolPattern(client, 'eobot')
        self.exxbw: AaopoolPattern = AaopoolPattern(client, 'exxbw')
        self.f2pool: AaopoolPattern = AaopoolPattern(client, 'f2pool')
        self.fiftyeightcoin: AaopoolPattern = AaopoolPattern(client, 'fiftyeightcoin')
        self.foundryusa: AaopoolPattern = AaopoolPattern(client, 'foundryusa')
        self.futurebitapollosolo: AaopoolPattern = AaopoolPattern(client, 'futurebitapollosolo')
        self.gbminers: AaopoolPattern = AaopoolPattern(client, 'gbminers')
        self.ghashio: AaopoolPattern = AaopoolPattern(client, 'ghashio')
        self.givemecoins: AaopoolPattern = AaopoolPattern(client, 'givemecoins')
        self.gogreenlight: AaopoolPattern = AaopoolPattern(client, 'gogreenlight')
        self.haominer: AaopoolPattern = AaopoolPattern(client, 'haominer')
        self.haozhuzhu: AaopoolPattern = AaopoolPattern(client, 'haozhuzhu')
        self.hashbx: AaopoolPattern = AaopoolPattern(client, 'hashbx')
        self.hashpool: AaopoolPattern = AaopoolPattern(client, 'hashpool')
        self.helix: AaopoolPattern = AaopoolPattern(client, 'helix')
        self.hhtt: AaopoolPattern = AaopoolPattern(client, 'hhtt')
        self.hotpool: AaopoolPattern = AaopoolPattern(client, 'hotpool')
        self.hummerpool: AaopoolPattern = AaopoolPattern(client, 'hummerpool')
        self.huobipool: AaopoolPattern = AaopoolPattern(client, 'huobipool')
        self.innopolistech: AaopoolPattern = AaopoolPattern(client, 'innopolistech')
        self.kanopool: AaopoolPattern = AaopoolPattern(client, 'kanopool')
        self.kncminer: AaopoolPattern = AaopoolPattern(client, 'kncminer')
        self.kucoinpool: AaopoolPattern = AaopoolPattern(client, 'kucoinpool')
        self.lubiancom: AaopoolPattern = AaopoolPattern(client, 'lubiancom')
        self.luckypool: AaopoolPattern = AaopoolPattern(client, 'luckypool')
        self.luxor: AaopoolPattern = AaopoolPattern(client, 'luxor')
        self.marapool: AaopoolPattern = AaopoolPattern(client, 'marapool')
        self.maxbtc: AaopoolPattern = AaopoolPattern(client, 'maxbtc')
        self.maxipool: AaopoolPattern = AaopoolPattern(client, 'maxipool')
        self.megabigpower: AaopoolPattern = AaopoolPattern(client, 'megabigpower')
        self.minerium: AaopoolPattern = AaopoolPattern(client, 'minerium')
        self.miningcity: AaopoolPattern = AaopoolPattern(client, 'miningcity')
        self.miningdutch: AaopoolPattern = AaopoolPattern(client, 'miningdutch')
        self.miningkings: AaopoolPattern = AaopoolPattern(client, 'miningkings')
        self.miningsquared: AaopoolPattern = AaopoolPattern(client, 'miningsquared')
        self.mmpool: AaopoolPattern = AaopoolPattern(client, 'mmpool')
        self.mtred: AaopoolPattern = AaopoolPattern(client, 'mtred')
        self.multicoinco: AaopoolPattern = AaopoolPattern(client, 'multicoinco')
        self.multipool: AaopoolPattern = AaopoolPattern(client, 'multipool')
        self.mybtccoinpool: AaopoolPattern = AaopoolPattern(client, 'mybtccoinpool')
        self.neopool: AaopoolPattern = AaopoolPattern(client, 'neopool')
        self.nexious: AaopoolPattern = AaopoolPattern(client, 'nexious')
        self.nicehash: AaopoolPattern = AaopoolPattern(client, 'nicehash')
        self.nmcbit: AaopoolPattern = AaopoolPattern(client, 'nmcbit')
        self.novablock: AaopoolPattern = AaopoolPattern(client, 'novablock')
        self.ocean: AaopoolPattern = AaopoolPattern(client, 'ocean')
        self.okexpool: AaopoolPattern = AaopoolPattern(client, 'okexpool')
        self.okkong: AaopoolPattern = AaopoolPattern(client, 'okkong')
        self.okminer: AaopoolPattern = AaopoolPattern(client, 'okminer')
        self.okpooltop: AaopoolPattern = AaopoolPattern(client, 'okpooltop')
        self.onehash: AaopoolPattern = AaopoolPattern(client, 'onehash')
        self.onem1x: AaopoolPattern = AaopoolPattern(client, 'onem1x')
        self.onethash: AaopoolPattern = AaopoolPattern(client, 'onethash')
        self.ozcoin: AaopoolPattern = AaopoolPattern(client, 'ozcoin')
        self.parasite: AaopoolPattern = AaopoolPattern(client, 'parasite')
        self.patels: AaopoolPattern = AaopoolPattern(client, 'patels')
        self.pegapool: AaopoolPattern = AaopoolPattern(client, 'pegapool')
        self.phashio: AaopoolPattern = AaopoolPattern(client, 'phashio')
        self.phoenix: AaopoolPattern = AaopoolPattern(client, 'phoenix')
        self.polmine: AaopoolPattern = AaopoolPattern(client, 'polmine')
        self.pool175btc: AaopoolPattern = AaopoolPattern(client, 'pool175btc')
        self.pool50btc: AaopoolPattern = AaopoolPattern(client, 'pool50btc')
        self.poolin: AaopoolPattern = AaopoolPattern(client, 'poolin')
        self.portlandhodl: AaopoolPattern = AaopoolPattern(client, 'portlandhodl')
        self.publicpool: AaopoolPattern = AaopoolPattern(client, 'publicpool')
        self.purebtccom: AaopoolPattern = AaopoolPattern(client, 'purebtccom')
        self.rawpool: AaopoolPattern = AaopoolPattern(client, 'rawpool')
        self.rigpool: AaopoolPattern = AaopoolPattern(client, 'rigpool')
        self.sbicrypto: AaopoolPattern = AaopoolPattern(client, 'sbicrypto')
        self.secpool: AaopoolPattern = AaopoolPattern(client, 'secpool')
        self.secretsuperstar: AaopoolPattern = AaopoolPattern(client, 'secretsuperstar')
        self.sevenpool: AaopoolPattern = AaopoolPattern(client, 'sevenpool')
        self.shawnp0wers: AaopoolPattern = AaopoolPattern(client, 'shawnp0wers')
        self.sigmapoolcom: AaopoolPattern = AaopoolPattern(client, 'sigmapoolcom')
        self.simplecoinus: AaopoolPattern = AaopoolPattern(client, 'simplecoinus')
        self.solock: AaopoolPattern = AaopoolPattern(client, 'solock')
        self.spiderpool: AaopoolPattern = AaopoolPattern(client, 'spiderpool')
        self.stminingcorp: AaopoolPattern = AaopoolPattern(client, 'stminingcorp')
        self.tangpool: AaopoolPattern = AaopoolPattern(client, 'tangpool')
        self.tatmaspool: AaopoolPattern = AaopoolPattern(client, 'tatmaspool')
        self.tbdice: AaopoolPattern = AaopoolPattern(client, 'tbdice')
        self.telco214: AaopoolPattern = AaopoolPattern(client, 'telco214')
        self.terrapool: AaopoolPattern = AaopoolPattern(client, 'terrapool')
        self.tiger: AaopoolPattern = AaopoolPattern(client, 'tiger')
        self.tigerpoolnet: AaopoolPattern = AaopoolPattern(client, 'tigerpoolnet')
        self.titan: AaopoolPattern = AaopoolPattern(client, 'titan')
        self.transactioncoinmining: AaopoolPattern = AaopoolPattern(client, 'transactioncoinmining')
        self.trickysbtcpool: AaopoolPattern = AaopoolPattern(client, 'trickysbtcpool')
        self.triplemining: AaopoolPattern = AaopoolPattern(client, 'triplemining')
        self.twentyoneinc: AaopoolPattern = AaopoolPattern(client, 'twentyoneinc')
        self.ultimuspool: AaopoolPattern = AaopoolPattern(client, 'ultimuspool')
        self.unknown: AaopoolPattern = AaopoolPattern(client, 'unknown')
        self.unomp: AaopoolPattern = AaopoolPattern(client, 'unomp')
        self.viabtc: AaopoolPattern = AaopoolPattern(client, 'viabtc')
        self.waterhole: AaopoolPattern = AaopoolPattern(client, 'waterhole')
        self.wayicn: AaopoolPattern = AaopoolPattern(client, 'wayicn')
        self.whitepool: AaopoolPattern = AaopoolPattern(client, 'whitepool')
        self.wk057: AaopoolPattern = AaopoolPattern(client, 'wk057')
        self.yourbtcnet: AaopoolPattern = AaopoolPattern(client, 'yourbtcnet')
        self.zulupool: AaopoolPattern = AaopoolPattern(client, 'zulupool')

class CatalogTree_Positions:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.block_position: MetricPattern11[BlkPosition] = MetricPattern11(client, 'position')
        self.tx_position: MetricPattern27[BlkPosition] = MetricPattern27(client, 'position')

class CatalogTree_Price:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.cents: CatalogTree_Price_Cents = CatalogTree_Price_Cents(client)
        self.sats: CatalogTree_Price_Sats = CatalogTree_Price_Sats(client)
        self.usd: CatalogTree_Price_Usd = CatalogTree_Price_Usd(client)

class CatalogTree_Price_Cents:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc: MetricPattern5[OHLCCents] = MetricPattern5(client, 'ohlc_cents')
        self.split: CatalogTree_Price_Cents_Split = CatalogTree_Price_Cents_Split(client)

class CatalogTree_Price_Cents_Split:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.close: MetricPattern5[Close<Cents>] = MetricPattern5(client, 'price_close_cents')
        self.high: MetricPattern5[High<Cents>] = MetricPattern5(client, 'price_high_cents')
        self.low: MetricPattern5[Low<Cents>] = MetricPattern5(client, 'price_low_cents')
        self.open: MetricPattern5[Open<Cents>] = MetricPattern5(client, 'price_open_cents')

class CatalogTree_Price_Sats:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc: MetricPattern1[OHLCSats] = MetricPattern1(client, 'price_ohlc_sats')
        self.split: SplitPattern2[Sats] = SplitPattern2(client, 'price_sats')

class CatalogTree_Price_Usd:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.ohlc: MetricPattern1[OHLCDollars] = MetricPattern1(client, 'price_ohlc')
        self.split: SplitPattern2[Dollars] = SplitPattern2(client, 'price')

class CatalogTree_Scripts:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.count: CatalogTree_Scripts_Count = CatalogTree_Scripts_Count(client)
        self.empty_to_txindex: MetricPattern9[TxIndex] = MetricPattern9(client, 'txindex')
        self.first_emptyoutputindex: MetricPattern11[EmptyOutputIndex] = MetricPattern11(client, 'first_emptyoutputindex')
        self.first_opreturnindex: MetricPattern11[OpReturnIndex] = MetricPattern11(client, 'first_opreturnindex')
        self.first_p2msoutputindex: MetricPattern11[P2MSOutputIndex] = MetricPattern11(client, 'first_p2msoutputindex')
        self.first_unknownoutputindex: MetricPattern11[UnknownOutputIndex] = MetricPattern11(client, 'first_unknownoutputindex')
        self.opreturn_to_txindex: MetricPattern14[TxIndex] = MetricPattern14(client, 'txindex')
        self.p2ms_to_txindex: MetricPattern17[TxIndex] = MetricPattern17(client, 'txindex')
        self.unknown_to_txindex: MetricPattern28[TxIndex] = MetricPattern28(client, 'txindex')
        self.value: CatalogTree_Scripts_Value = CatalogTree_Scripts_Value(client)

class CatalogTree_Scripts_Count:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.emptyoutput: DollarsPattern[StoredU64] = DollarsPattern(client, 'emptyoutput_count')
        self.opreturn: DollarsPattern[StoredU64] = DollarsPattern(client, 'opreturn_count')
        self.p2a: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2a_count')
        self.p2ms: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2ms_count')
        self.p2pk33: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2pk33_count')
        self.p2pk65: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2pk65_count')
        self.p2pkh: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2pkh_count')
        self.p2sh: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2sh_count')
        self.p2tr: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2tr_count')
        self.p2wpkh: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2wpkh_count')
        self.p2wsh: DollarsPattern[StoredU64] = DollarsPattern(client, 'p2wsh_count')
        self.segwit: DollarsPattern[StoredU64] = DollarsPattern(client, 'segwit_count')
        self.segwit_adoption: SegwitAdoptionPattern = SegwitAdoptionPattern(client, 'segwit_adoption')
        self.taproot_adoption: SegwitAdoptionPattern = SegwitAdoptionPattern(client, 'taproot_adoption')
        self.unknownoutput: DollarsPattern[StoredU64] = DollarsPattern(client, 'unknownoutput_count')

class CatalogTree_Scripts_Value:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: CoinbasePattern = CoinbasePattern(client, 'opreturn_value')

class CatalogTree_Supply:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.burned: CatalogTree_Supply_Burned = CatalogTree_Supply_Burned(client)
        self.circulating: CatalogTree_Supply_Circulating = CatalogTree_Supply_Circulating(client)
        self.inflation: MetricPattern4[StoredF32] = MetricPattern4(client, 'inflation_rate')
        self.market_cap: MetricPattern1[Dollars] = MetricPattern1(client, 'market_cap')
        self.velocity: CatalogTree_Supply_Velocity = CatalogTree_Supply_Velocity(client)

class CatalogTree_Supply_Burned:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.opreturn: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'opreturn_supply')
        self.unspendable: UnclaimedRewardsPattern = UnclaimedRewardsPattern(client, 'unspendable_supply')

class CatalogTree_Supply_Circulating:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: MetricPattern3[Bitcoin] = MetricPattern3(client, 'circulating_supply_btc')
        self.dollars: MetricPattern3[Dollars] = MetricPattern3(client, 'circulating_supply_usd')
        self.sats: MetricPattern3[Sats] = MetricPattern3(client, 'circulating_supply')

class CatalogTree_Supply_Velocity:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.btc: MetricPattern4[StoredF64] = MetricPattern4(client, 'btc_velocity')
        self.usd: MetricPattern4[StoredF64] = MetricPattern4(client, 'usd_velocity')

class CatalogTree_Transactions:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.base_size: MetricPattern27[StoredU32] = MetricPattern27(client, 'base_size')
        self.count: CatalogTree_Transactions_Count = CatalogTree_Transactions_Count(client)
        self.fees: CatalogTree_Transactions_Fees = CatalogTree_Transactions_Fees(client)
        self.first_txindex: MetricPattern11[TxIndex] = MetricPattern11(client, 'first_txindex')
        self.first_txinindex: MetricPattern27[TxInIndex] = MetricPattern27(client, 'first_txinindex')
        self.first_txoutindex: MetricPattern27[TxOutIndex] = MetricPattern27(client, 'first_txoutindex')
        self.height: MetricPattern27[Height] = MetricPattern27(client, 'height')
        self.is_explicitly_rbf: MetricPattern27[StoredBool] = MetricPattern27(client, 'is_explicitly_rbf')
        self.rawlocktime: MetricPattern27[RawLockTime] = MetricPattern27(client, 'rawlocktime')
        self.size: CatalogTree_Transactions_Size = CatalogTree_Transactions_Size(client)
        self.total_size: MetricPattern27[StoredU32] = MetricPattern27(client, 'total_size')
        self.txid: MetricPattern27[Txid] = MetricPattern27(client, 'txid')
        self.txversion: MetricPattern27[TxVersion] = MetricPattern27(client, 'txversion')
        self.versions: CatalogTree_Transactions_Versions = CatalogTree_Transactions_Versions(client)
        self.volume: CatalogTree_Transactions_Volume = CatalogTree_Transactions_Volume(client)

class CatalogTree_Transactions_Count:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.is_coinbase: MetricPattern27[StoredBool] = MetricPattern27(client, 'is_coinbase')
        self.tx_count: DollarsPattern[StoredU64] = DollarsPattern(client, 'tx_count')

class CatalogTree_Transactions_Fees:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.fee: CatalogTree_Transactions_Fees_Fee = CatalogTree_Transactions_Fees_Fee(client)
        self.fee_rate: FeeRatePattern[FeeRate] = FeeRatePattern(client, 'fee_rate')
        self.input_value: MetricPattern27[Sats] = MetricPattern27(client, 'input_value')
        self.output_value: MetricPattern27[Sats] = MetricPattern27(client, 'output_value')

class CatalogTree_Transactions_Fees_Fee:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.bitcoin: CountPattern2[Bitcoin] = CountPattern2(client, 'fee_btc')
        self.dollars: CatalogTree_Transactions_Fees_Fee_Dollars = CatalogTree_Transactions_Fees_Fee_Dollars(client)
        self.sats: CountPattern2[Sats] = CountPattern2(client, 'fee')
        self.txindex: MetricPattern27[Sats] = MetricPattern27(client, 'fee')

class CatalogTree_Transactions_Fees_Fee_Dollars:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.average: MetricPattern1[Dollars] = MetricPattern1(client, 'fee_usd_average')
        self.cumulative: MetricPattern2[Dollars] = MetricPattern2(client, 'fee_usd_cumulative')
        self.height_cumulative: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_cumulative')
        self.max: MetricPattern1[Dollars] = MetricPattern1(client, 'fee_usd_max')
        self.median: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_median')
        self.min: MetricPattern1[Dollars] = MetricPattern1(client, 'fee_usd_min')
        self.pct10: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_pct10')
        self.pct25: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_pct25')
        self.pct75: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_pct75')
        self.pct90: MetricPattern11[Dollars] = MetricPattern11(client, 'fee_usd_pct90')
        self.sum: MetricPattern1[Dollars] = MetricPattern1(client, 'fee_usd_sum')

class CatalogTree_Transactions_Size:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.vsize: FeeRatePattern[VSize] = FeeRatePattern(client, 'tx_vsize_average')
        self.weight: FeeRatePattern[Weight] = FeeRatePattern(client, 'tx_weight_average')

class CatalogTree_Transactions_Versions:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.v1: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v1')
        self.v2: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v2')
        self.v3: BlockCountPattern[StoredU64] = BlockCountPattern(client, 'tx_v3')

class CatalogTree_Transactions_Volume:
    """Catalog tree node."""

    def __init__(self, client: BrkClientBase, base_path: str = ''):
        self.annualized_volume: _2015Pattern = _2015Pattern(client, 'annualized_volume')
        self.inputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'inputs_per_sec')
        self.outputs_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'outputs_per_sec')
        self.sent_sum: ActiveSupplyPattern = ActiveSupplyPattern(client, 'sent_sum')
        self.tx_per_sec: MetricPattern4[StoredF32] = MetricPattern4(client, 'tx_per_sec')

class BrkClient(BrkClientBase):
    """Main BRK client with catalog tree and API methods."""

    VERSION = "v0.1.0-alpha.2"

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
      "emptyaddressindex"
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
        "id": "up_to_1h_old",
        "short": "<1h",
        "long": "Up to 1 Hour Old"
      },
      "_1h_to_1d": {
        "id": "at_least_1h_up_to_1d_old",
        "short": "1h-1d",
        "long": "1 Hour to 1 Day Old"
      },
      "_1d_to_1w": {
        "id": "at_least_1d_up_to_1w_old",
        "short": "1d-1w",
        "long": "1 Day to 1 Week Old"
      },
      "_1w_to_1m": {
        "id": "at_least_1w_up_to_1m_old",
        "short": "1w-1m",
        "long": "1 Week to 1 Month Old"
      },
      "_1m_to_2m": {
        "id": "at_least_1m_up_to_2m_old",
        "short": "1m-2m",
        "long": "1 to 2 Months Old"
      },
      "_2m_to_3m": {
        "id": "at_least_2m_up_to_3m_old",
        "short": "2m-3m",
        "long": "2 to 3 Months Old"
      },
      "_3m_to_4m": {
        "id": "at_least_3m_up_to_4m_old",
        "short": "3m-4m",
        "long": "3 to 4 Months Old"
      },
      "_4m_to_5m": {
        "id": "at_least_4m_up_to_5m_old",
        "short": "4m-5m",
        "long": "4 to 5 Months Old"
      },
      "_5m_to_6m": {
        "id": "at_least_5m_up_to_6m_old",
        "short": "5m-6m",
        "long": "5 to 6 Months Old"
      },
      "_6m_to_1y": {
        "id": "at_least_6m_up_to_1y_old",
        "short": "6m-1y",
        "long": "6 Months to 1 Year Old"
      },
      "_1y_to_2y": {
        "id": "at_least_1y_up_to_2y_old",
        "short": "1y-2y",
        "long": "1 to 2 Years Old"
      },
      "_2y_to_3y": {
        "id": "at_least_2y_up_to_3y_old",
        "short": "2y-3y",
        "long": "2 to 3 Years Old"
      },
      "_3y_to_4y": {
        "id": "at_least_3y_up_to_4y_old",
        "short": "3y-4y",
        "long": "3 to 4 Years Old"
      },
      "_4y_to_5y": {
        "id": "at_least_4y_up_to_5y_old",
        "short": "4y-5y",
        "long": "4 to 5 Years Old"
      },
      "_5y_to_6y": {
        "id": "at_least_5y_up_to_6y_old",
        "short": "5y-6y",
        "long": "5 to 6 Years Old"
      },
      "_6y_to_7y": {
        "id": "at_least_6y_up_to_7y_old",
        "short": "6y-7y",
        "long": "6 to 7 Years Old"
      },
      "_7y_to_8y": {
        "id": "at_least_7y_up_to_8y_old",
        "short": "7y-8y",
        "long": "7 to 8 Years Old"
      },
      "_8y_to_10y": {
        "id": "at_least_8y_up_to_10y_old",
        "short": "8y-10y",
        "long": "8 to 10 Years Old"
      },
      "_10y_to_12y": {
        "id": "at_least_10y_up_to_12y_old",
        "short": "10y-12y",
        "long": "10 to 12 Years Old"
      },
      "_12y_to_15y": {
        "id": "at_least_12y_up_to_15y_old",
        "short": "12y-15y",
        "long": "12 to 15 Years Old"
      },
      "from_15y": {
        "id": "at_least_15y_old",
        "short": "15y+",
        "long": "15+ Years Old"
      }
    }

    MAX_AGE_NAMES = {
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
    }

    MIN_AGE_NAMES = {
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
        "long": "1 to 10 Sats"
      },
      "_10sats_to_100sats": {
        "id": "above_10sats_under_100sats",
        "short": "10-100 sats",
        "long": "10 to 100 Sats"
      },
      "_100sats_to_1k_sats": {
        "id": "above_100sats_under_1k_sats",
        "short": "100-1k sats",
        "long": "100 to 1K Sats"
      },
      "_1k_sats_to_10k_sats": {
        "id": "above_1k_sats_under_10k_sats",
        "short": "1k-10k sats",
        "long": "1K to 10K Sats"
      },
      "_10k_sats_to_100k_sats": {
        "id": "above_10k_sats_under_100k_sats",
        "short": "10k-100k sats",
        "long": "10K to 100K Sats"
      },
      "_100k_sats_to_1m_sats": {
        "id": "above_100k_sats_under_1m_sats",
        "short": "100k-1M sats",
        "long": "100K to 1M Sats"
      },
      "_1m_sats_to_10m_sats": {
        "id": "above_1m_sats_under_10m_sats",
        "short": "1M-10M sats",
        "long": "1M to 10M Sats"
      },
      "_10m_sats_to_1btc": {
        "id": "above_10m_sats_under_1btc",
        "short": "0.1-1 BTC",
        "long": "0.1 to 1 BTC"
      },
      "_1btc_to_10btc": {
        "id": "above_1btc_under_10btc",
        "short": "1-10 BTC",
        "long": "1 to 10 BTC"
      },
      "_10btc_to_100btc": {
        "id": "above_10btc_under_100btc",
        "short": "10-100 BTC",
        "long": "10 to 100 BTC"
      },
      "_100btc_to_1k_btc": {
        "id": "above_100btc_under_1k_btc",
        "short": "100-1k BTC",
        "long": "100 to 1K BTC"
      },
      "_1k_btc_to_10k_btc": {
        "id": "above_1k_btc_under_10k_btc",
        "short": "1k-10k BTC",
        "long": "1K to 10K BTC"
      },
      "_10k_btc_to_100k_btc": {
        "id": "above_10k_btc_under_100k_btc",
        "short": "10k-100k BTC",
        "long": "10K to 100K BTC"
      },
      "_100k_btc_or_more": {
        "id": "above_100k_btc",
        "short": "100k+ BTC",
        "long": "100K+ BTC"
      }
    }

    GE_AMOUNT_NAMES = {
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
      "_1k_sats": {
        "id": "above_1k_sats",
        "short": "1k+ sats",
        "long": "Above 1K Sats"
      },
      "_10k_sats": {
        "id": "above_10k_sats",
        "short": "10k+ sats",
        "long": "Above 10K Sats"
      },
      "_100k_sats": {
        "id": "above_100k_sats",
        "short": "100k+ sats",
        "long": "Above 100K Sats"
      },
      "_1m_sats": {
        "id": "above_1m_sats",
        "short": "1M+ sats",
        "long": "Above 1M Sats"
      },
      "_10m_sats": {
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
      "_1k_btc": {
        "id": "above_1k_btc",
        "short": "1k+ BTC",
        "long": "Above 1K BTC"
      },
      "_10k_btc": {
        "id": "above_10k_btc",
        "short": "10k+ BTC",
        "long": "Above 10K BTC"
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
        self.tree = CatalogTree(self)

    def get_address(self, address: Address) -> AddressStats:
        """Address information.

        Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.)."""
        return self.get(f'/api/address/{address}')

    def get_address_txs(self, address: Address, after_txid: Optional[str] = None, limit: Optional[float] = None) -> List[Txid]:
        """Address transaction IDs.

        Get transaction IDs for an address, newest first. Use after_txid for pagination."""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/address/{address}/txs{"?" + query if query else ""}')

    def get_address_txs_chain(self, address: Address, after_txid: Optional[str] = None, limit: Optional[float] = None) -> List[Txid]:
        """Address confirmed transactions.

        Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination."""
        params = []
        if after_txid is not None: params.append(f'after_txid={after_txid}')
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/address/{address}/txs/chain{"?" + query if query else ""}')

    def get_address_txs_mempool(self, address: Address) -> List[Txid]:
        """Address mempool transactions.

        Get unconfirmed transaction IDs for an address from the mempool (up to 50)."""
        return self.get(f'/api/address/{address}/txs/mempool')

    def get_address_utxo(self, address: Address) -> List[Utxo]:
        """Address UTXOs.

        Get unspent transaction outputs for an address."""
        return self.get(f'/api/address/{address}/utxo')

    def get_block_height(self, height: Height) -> BlockInfo:
        """Block by height.

        Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count."""
        return self.get(f'/api/block-height/{height}')

    def get_block_by_hash(self, hash: BlockHash) -> BlockInfo:
        """Block information.

        Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count."""
        return self.get(f'/api/block/{hash}')

    def get_block_by_hash_raw(self, hash: BlockHash) -> List[float]:
        """Raw block.

        Returns the raw block data in binary format."""
        return self.get(f'/api/block/{hash}/raw')

    def get_block_by_hash_status(self, hash: BlockHash) -> BlockStatus:
        """Block status.

        Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block."""
        return self.get(f'/api/block/{hash}/status')

    def get_block_by_hash_txid_by_index(self, hash: BlockHash, index: TxIndex) -> Txid:
        """Transaction ID at index.

        Retrieve a single transaction ID at a specific index within a block. Returns plain text txid."""
        return self.get(f'/api/block/{hash}/txid/{index}')

    def get_block_by_hash_txids(self, hash: BlockHash) -> List[Txid]:
        """Block transaction IDs.

        Retrieve all transaction IDs in a block by block hash."""
        return self.get(f'/api/block/{hash}/txids')

    def get_block_by_hash_txs_by_start_index(self, hash: BlockHash, start_index: TxIndex) -> List[Transaction]:
        """Block transactions (paginated).

        Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time."""
        return self.get(f'/api/block/{hash}/txs/{start_index}')

    def get_blocks(self) -> List[BlockInfo]:
        """Recent blocks.

        Retrieve the last 10 blocks. Returns block metadata for each block."""
        return self.get('/api/blocks')

    def get_blocks_by_height(self, height: Height) -> List[BlockInfo]:
        """Blocks from height.

        Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0."""
        return self.get(f'/api/blocks/{height}')

    def get_mempool_info(self) -> MempoolInfo:
        """Mempool statistics.

        Get current mempool statistics including transaction count, total vsize, and total fees."""
        return self.get('/api/mempool/info')

    def get_mempool_txids(self) -> List[Txid]:
        """Mempool transaction IDs.

        Get all transaction IDs currently in the mempool."""
        return self.get('/api/mempool/txids')

    def get_metric(self, metric: Metric) -> List[Index]:
        """Get supported indexes for a metric.

        Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex."""
        return self.get(f'/api/metric/{metric}')

    def get_metric_by_index(self, index: Index, metric: Metric, count: Optional[Any] = None, format: Optional[Format] = None, from_: Optional[Any] = None, to: Optional[Any] = None) -> AnyMetricData:
        """Get metric data.

        Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv)."""
        params = []
        if count is not None: params.append(f'count={count}')
        if format is not None: params.append(f'format={format}')
        if from_ is not None: params.append(f'from={from_}')
        if to is not None: params.append(f'to={to}')
        query = '&'.join(params)
        return self.get(f'/api/metric/{metric}/{index}{"?" + query if query else ""}')

    def get_metrics_bulk(self, index: Index, metrics: Metrics, count: Optional[Any] = None, format: Optional[Format] = None, from_: Optional[Any] = None, to: Optional[Any] = None) -> List[AnyMetricData]:
        """Bulk metric data.

        Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects."""
        params = []
        if count is not None: params.append(f'count={count}')
        if format is not None: params.append(f'format={format}')
        if from_ is not None: params.append(f'from={from_}')
        params.append(f'index={index}')
        params.append(f'metrics={metrics}')
        if to is not None: params.append(f'to={to}')
        query = '&'.join(params)
        return self.get(f'/api/metrics/bulk{"?" + query if query else ""}')

    def get_metrics_catalog(self) -> TreeNode:
        """Metrics catalog.

        Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure."""
        return self.get('/api/metrics/catalog')

    def get_metrics_count(self) -> List[MetricCount]:
        """Metric count.

        Current metric count"""
        return self.get('/api/metrics/count')

    def get_metrics_indexes(self) -> List[IndexInfo]:
        """List available indexes.

        Returns all available indexes with their accepted query aliases. Use any alias when querying metrics."""
        return self.get('/api/metrics/indexes')

    def get_metrics_list(self, page: Optional[Any] = None) -> PaginatedMetrics:
        """Metrics list.

        Paginated list of available metrics"""
        params = []
        if page is not None: params.append(f'page={page}')
        query = '&'.join(params)
        return self.get(f'/api/metrics/list{"?" + query if query else ""}')

    def get_metrics_search_by_metric(self, metric: Metric, limit: Optional[Limit] = None) -> List[Metric]:
        """Search metrics.

        Fuzzy search for metrics by name. Supports partial matches and typos."""
        params = []
        if limit is not None: params.append(f'limit={limit}')
        query = '&'.join(params)
        return self.get(f'/api/metrics/search/{metric}{"?" + query if query else ""}')

    def get_tx_by_txid(self, txid: Txid) -> Transaction:
        """Transaction information.

        Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files."""
        return self.get(f'/api/tx/{txid}')

    def get_tx_by_txid_hex(self, txid: Txid) -> Hex:
        """Transaction hex.

        Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format."""
        return self.get(f'/api/tx/{txid}/hex')

    def get_tx_by_txid_outspend_by_vout(self, txid: Txid, vout: Vout) -> TxOutspend:
        """Output spend status.

        Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details."""
        return self.get(f'/api/tx/{txid}/outspend/{vout}')

    def get_tx_by_txid_outspends(self, txid: Txid) -> List[TxOutspend]:
        """All output spend statuses.

        Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output."""
        return self.get(f'/api/tx/{txid}/outspends')

    def get_tx_by_txid_status(self, txid: Txid) -> TxStatus:
        """Transaction status.

        Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp."""
        return self.get(f'/api/tx/{txid}/status')

    def get_v1_difficulty_adjustment(self) -> DifficultyAdjustment:
        """Difficulty adjustment.

        Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction."""
        return self.get('/api/v1/difficulty-adjustment')

    def get_v1_fees_mempool_blocks(self) -> List[MempoolBlock]:
        """Projected mempool blocks.

        Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now."""
        return self.get('/api/v1/fees/mempool-blocks')

    def get_v1_fees_recommended(self) -> RecommendedFees:
        """Recommended fees.

        Get recommended fee rates for different confirmation targets based on current mempool state."""
        return self.get('/api/v1/fees/recommended')

    def get_v1_mining_blocks_fees_by_time_period(self, time_period: TimePeriod) -> List[BlockFeesEntry]:
        """Block fees.

        Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/fees/{time_period}')

    def get_v1_mining_blocks_rewards_by_time_period(self, time_period: TimePeriod) -> List[BlockRewardsEntry]:
        """Block rewards.

        Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/rewards/{time_period}')

    def get_v1_mining_blocks_sizes_weights_by_time_period(self, time_period: TimePeriod) -> BlockSizesWeights:
        """Block sizes and weights.

        Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/blocks/sizes-weights/{time_period}')

    def get_v1_mining_blocks_timestamp(self, timestamp: Timestamp) -> BlockTimestamp:
        """Block by timestamp.

        Find the block closest to a given UNIX timestamp."""
        return self.get(f'/api/v1/mining/blocks/timestamp/{timestamp}')

    def get_v1_mining_difficulty_adjustments(self) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments (all time).

        Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent]."""
        return self.get('/api/v1/mining/difficulty-adjustments')

    def get_v1_mining_difficulty_adjustments_by_time_period(self, time_period: TimePeriod) -> List[DifficultyAdjustmentEntry]:
        """Difficulty adjustments.

        Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent]."""
        return self.get(f'/api/v1/mining/difficulty-adjustments/{time_period}')

    def get_v1_mining_hashrate(self) -> HashrateSummary:
        """Network hashrate (all time).

        Get network hashrate and difficulty data for all time."""
        return self.get('/api/v1/mining/hashrate')

    def get_v1_mining_hashrate_by_time_period(self, time_period: TimePeriod) -> HashrateSummary:
        """Network hashrate.

        Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/hashrate/{time_period}')

    def get_v1_mining_pool_by_slug(self, slug: PoolSlug) -> PoolDetail:
        """Mining pool details.

        Get detailed information about a specific mining pool including block counts and shares for different time periods."""
        return self.get(f'/api/v1/mining/pool/{slug}')

    def get_v1_mining_pools(self) -> List[PoolInfo]:
        """List all mining pools.

        Get list of all known mining pools with their identifiers."""
        return self.get('/api/v1/mining/pools')

    def get_v1_mining_pools_by_time_period(self, time_period: TimePeriod) -> PoolsSummary:
        """Mining pool statistics.

        Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y"""
        return self.get(f'/api/v1/mining/pools/{time_period}')

    def get_v1_mining_reward_stats_by_block_count(self, block_count: float) -> RewardStats:
        """Mining reward statistics.

        Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count."""
        return self.get(f'/api/v1/mining/reward-stats/{block_count}')

    def get_v1_validate_address(self, address: str) -> AddressValidation:
        """Validate address.

        Validate a Bitcoin address and get information about its type and scriptPubKey."""
        return self.get(f'/api/v1/validate-address/{address}')

    def get_health(self) -> Health:
        """Health check.

        Returns the health status of the API server"""
        return self.get('/health')

    def get_version(self) -> str:
        """API version.

        Returns the current version of the API server"""
        return self.get('/version')
